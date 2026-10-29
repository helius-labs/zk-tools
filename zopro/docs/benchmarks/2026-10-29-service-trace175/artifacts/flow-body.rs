const D:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-service-trace175";
const X:&str="/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";
const VERIFY:&str="/home/ubuntu/src/zopro-perf-work/batch-local-steal/gnark/verify";
const ACCEPTED:&str="/home/ubuntu/src/zopro-perf-work/balanced-kh-production/native/candidate-sdk";
const MANIFEST:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json";
type Event=[u64;10];
fn med(mut x:Vec<f64>)->f64{x.sort_by(f64::total_cmp);assert!(!x.is_empty());if x.len()%2==1{x[x.len()/2]}else{(x[x.len()/2-1]+x[x.len()/2])/2.}}
fn stdout(job:&Value)->String{fs::read_to_string(r(format!("runs/{}/stdout.log",job["name"].as_str().unwrap()))).unwrap()}
fn jsonlines(s:&str,prefix:&str)->Vec<Value>{s.lines().filter_map(|l|l.strip_prefix(prefix)).map(|s|serde_json::from_str(s).unwrap()).collect()}
fn graph(g:&Value,sample:f64)->Value{
    let bytes=fs::read(g["file"].as_str().unwrap()).unwrap();assert_eq!(g["record_bytes"],80);assert_eq!(g["limit_per_thread"],1024);
    let lengths=g["thread_lengths"].as_array().unwrap();let drops:u64=g["thread_dropped"].as_array().unwrap().iter().map(|x|x.as_u64().unwrap()).sum();
    assert_eq!(lengths.len(),33);assert_eq!(g["thread_dropped"].as_array().unwrap().len(),33);
    let count:usize=lengths.iter().map(|x|x.as_u64().unwrap()as usize).sum();assert_eq!(bytes.len(),count*80);
    let mut events=BTreeMap::<u64,Event>::new();let mut at=0;
    for(i,n)in lengths.iter().enumerate(){let worker=if i+1==lengths.len(){0}else{i+1}as u64;
        assert!(n.as_u64().unwrap()<=1024);
        for slot in 0..n.as_u64().unwrap(){let e:Event=core::array::from_fn(|j|u64::from_le_bytes(bytes[at+j*8..at+j*8+8].try_into().unwrap()));at+=80;
            assert_eq!(e[0]>>32,worker);assert_eq!(e[7],worker);assert_eq!(e[0]&0xffffffff,slot+1);assert!(e[6]>=e[5]&&e[6]>0);assert!(e[9]>=e[8]);assert!(events.insert(e[0],e).is_none());
        }
    }
    if count==0{assert_eq!(drops,0);return json!({"round":g["round"],"events":0,"dropped":0,"complete":false,"disabled":true});}
    if drops>0{return json!({"round":g["round"],"events":count,"dropped":drops,"complete":false,"disabled":false});}
    for e in events.values(){if e[1]!=0{let p=events.get(&e[1]).expect("missing parent");assert!(p[5]<=e[5]&&e[6]<=p[6],"parent containment kind{}",e[2]);}}
    let one=|kind:u64|->Event{let es:Vec<_>=events.values().filter(|e|e[2]==kind).collect();assert_eq!(es.len(),1,"unique boundary kind{kind}");*es[0]};
    let api=one(0);assert_eq!(api[1],0);assert!((api[6]-api[5])as f64/1e6<=sample+0.05);
    let a=one(6);let kh=one(2);let g2=one(5);for e in [a,kh,g2]{assert_eq!(e[1],api[0]);}
    let ak=one(1);let ab=one(7);let kb=one(8);let gk=one(3);let ready=one(4);
    assert_eq!(ak[1],a[0]);assert_eq!(ab[1],ak[0]);assert_eq!(kb[1],kh[0]);assert_eq!(gk[1],g2[0]);assert_eq!(ready[1],gk[0]);
    let ancestry=|id:u64|->Vec<u64>{let mut ids=vec![];let mut next=id;while next!=0{assert!(ids.len()<128);let e=events.get(&next).unwrap();ids.push(next);next=e[1];}ids};
    let serial:Vec<_>=events.values().filter(|e|e[8]!=0).collect();
    for e in &serial{assert!(ancestry(e[1]).iter().all(|id|events[id][8]==0),"nested serial span");}
    for worker in 0..33{let mut es:Vec<_>=serial.iter().filter(|e|e[7]==worker).collect();es.sort_by_key(|e|e[5]);for w in es.windows(2){assert!(w[0][6]<=w[1][5],"overlapping serial spans");}}
    let mut queries=BTreeMap::new();let mut last=(0u64,"");
    for(name,q,kernel,windowkind,available)in[("a",a,ab,10,ab[5]),("kh",kh,kb,11,kb[5]),("g2",g2,gk,12,ready[6])]{
        let mut windows:Vec<_>=events.values().filter(|e|e[2]==windowkind&&e[1]==kernel[0]).copied().collect();windows.sort_by_key(|e|e[3]);
        let expected=if name=="a"{255usize.div_ceil(kernel[4]as usize)}else if name=="kh"{20}else{11};assert_eq!(windows.len(),expected,"window count {name}");
        if name=="kh"{let mut bit=0;let mut widths=vec![];for w in &windows{assert_eq!(w[3],bit);assert!([12,13].contains(&w[4]));bit+=w[4];widths.push(w[4]);}assert_eq!(bit,255);assert_eq!(widths.iter().filter(|&&x|x==12).count(),5);}
        else{for(i,w)in windows.iter().enumerate(){assert_eq!(w[3],i as u64);}}
        let mut serial_cpu=0u64;let mut by_kind=BTreeMap::<u64,u64>::new();let mut leaves=0;
        for e in &serial{if ancestry(e[0]).contains(&q[0]){serial_cpu+=e[9]-e[8];*by_kind.entry(e[2]).or_default()+=e[9]-e[8];leaves+=1;}}
        let mut ws=vec![];
        for w in windows{
            assert!(w[5]>=available,"window begins before input marker");
            let parts:Vec<_>=events.values().filter(|e|e[1]==w[0]&&[13,14].contains(&e[2])).collect();
            if name=="g2"{assert_eq!(parts.len(),2);}
            if name=="kh"{assert!(parts.is_empty());let feeds=serial.iter().filter(|e|e[1]==w[0]&&e[2]==21).count();assert_eq!(feeds,2);}
            if name=="a"&&w[3]+1<expected as u64{assert!((1..=2).contains(&parts.len()));}
            let cpu:u64=serial.iter().filter(|e|ancestry(e[0]).contains(&w[0])).map(|e|e[9]-e[8]).sum();
            ws.push(json!({"id":w[0],"index_or_bit":w[3],"width":w[4],"start_ns":w[5],"end_ns":w[6],"input_available_to_start_ms":(w[5]-available)as f64/1e6,"elapsed_ms":(w[6]-w[5])as f64/1e6,"covered_serial_CPU_ms":cpu as f64/1e6,"parts":parts.len()}));
        }
        if q[6]>last.0{last=(q[6],name);}
        queries.insert(name,json!({"id":q[0],"start_ns":q[5],"end_ns":q[6],"input_available_ns":available,"query_elapsed_ms":(q[6]-q[5])as f64/1e6,"pre_input_elapsed_ms":(available-q[5])as f64/1e6,"covered_serial_CPU_ms":serial_cpu as f64/1e6,"serial_leaves":leaves,"serial_CPU_ns_by_kind":by_kind,"windows":ws}));
    }
    assert!(serial.iter().any(|e|e[2]==20));assert!(serial.iter().any(|e|e[2]==21));assert!(serial.iter().any(|e|e[2]==22));assert!(serial.iter().any(|e|e[2]==28));
    json!({"round":g["round"],"events":count,"dropped":0,"complete":true,"disabled":false,"file":g["file"],"sha256":hash(g["file"].as_str().unwrap()),"api_start_ns":api[5],"api_end_ns":api[6],"last_query":last.1,"after_last_query_ms":(api[6]-last.0)as f64/1e6,"queries":queries})
}
fn semantic(){let j=read(r("jobs.json"));let job=j.as_array().unwrap().iter().find(|j|j["phase"]=="preflight"&&j["mode"]=="on").unwrap();let s=stdout(job);let samples=jsonlines(&s,"SAMPLE ");let gs=jsonlines(&s,"GRAPH ");assert_eq!(gs.len(),1);let g=graph(&gs[0],samples[0]["ms"].as_f64().unwrap());assert_eq!(g["complete"],true);save(r("semantic-preflight.json"),&g);println!("Semantic preflight all current query/kernel/window/serial-leaf parents valid; zero drops");}
fn prepare(){
    assert!(!r("run-plan.json").exists());assert_eq!(read(r("native/result.json"))["status"],"complete");
    let mut pins:BTreeMap<String,String>=serde_json::from_value(read(r("build-pins.json"))).unwrap();for(p,h)in &pins{assert_eq!(hash(p),*h);}
    let audit=read(r("source-audit.json"));assert_eq!(audit["source_files"],176);assert_eq!(audit["originals_exact"],167);assert_eq!(audit["reverse175_exact"],true);assert_eq!(audit["field_source_files_exact"],17);
    assert_eq!(read(r("native/candidate-core/tests.json"))["passed_tests"].as_array().unwrap().len(),126);assert_eq!(read(r("native/candidate-core/tests.json"))["unfiltered"],true);assert_eq!(read(r("native/candidate-prover/tests.json"))["passed_tests"].as_array().unwrap().len(),8);
    for f in ["frozen-candidate.json","native/candidate/artifacts.json"]{let v=read(r(f));for(p,h)in v["input_pins"].as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());pin(&mut pins,p);}pin(&mut pins,r(f));}
    assert_eq!(hash(ACCEPTED),"20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f");
    let prior_root=Path::new("/home/ubuntu/src/zopro-perf-work/g2-packed-index175");let mut prior:BTreeSet<String>=serde_json::from_value(read(prior_root.join("prior-proof-sha256.json"))).unwrap();assert_eq!(prior.len(),17708);
    for(p,h)in read(prior_root.join("proof-inventory.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());pin(&mut pins,p);if p.ends_with(".proof"){assert!(prior.insert(h.as_str().unwrap().into()));}}
    assert_eq!(prior.len(),17786);save(r("prior-proof-sha256.json"),&json!(prior));
    let cases=read("/home/ubuntu/src/zopro-perf-work/sdk-control-copies/cases.json");assert_eq!(cases.as_array().unwrap().len(),9);save(r("cases.json"),&cases);
    let mut env=read(MANIFEST)["environment"].clone();env["RAYON_NUM_THREADS"]=json!("32");let mut steps=vec![];let mut jobs=vec![];
    let add=|name:String,phase:&str,mode:&str,block:usize,c:&Value,rounds:usize,steps:&mut Vec<Value>,jobs:&mut Vec<Value>|{
        let dir=r("proofs").join(&name);let binary=if mode=="accepted"{PathBuf::from(ACCEPTED)}else{r("native/candidate-sdk")};
        let mut args=vec![json!("taskset"),json!("-c"),json!("0-31"),json!(binary),c["image"].clone(),c["request"].clone(),json!(rounds.to_string()),json!(dir)];if mode!="accepted"{args.push(json!(mode));}
        steps.push(json!({"kind":"command","name":name,"argv":args,"environment":env,"timeout_seconds":600}));jobs.push(json!({"name":name,"phase":phase,"mode":mode,"block":block,"rounds":rounds,"case":c,"directory":dir}));
    };
    let verify=|j:&Value|{let c=&j["case"];let dir=Path::new(j["directory"].as_str().unwrap());json!({"kind":"command","name":format!("verify-{}",j["name"].as_str().unwrap()),"argv":[VERIFY,c["image"],c["image_sha256"],dir.join("0.proof"),dir.join("0.public"),c["public_sha256"]],"timeout_seconds":120})};
    for c in cases.as_array().unwrap(){add(format!("pre-{}-off",c["name"].as_str().unwrap()),"preflight","off",0,c,1,&mut steps,&mut jobs);steps.push(verify(jobs.last().unwrap()));}
    add("pre-primary-on".into(),"preflight","on",0,&cases[0],1,&mut steps,&mut jobs);steps.push(verify(jobs.last().unwrap()));
    steps.push(json!({"kind":"command","name":"semantic-preflight","argv":[r("flow-helper"),"semantic"],"timeout_seconds":120}));
    for block in 0..3{let mut modes=["accepted","off","on"];modes.rotate_left(block%3);if block%2!=0{modes.reverse();}for mode in modes{add(format!("b{block}-{mode}"),"main",mode,block,&cases[0],31,&mut steps,&mut jobs);}}
    for j in jobs.iter().filter(|j|j["phase"]=="main"){steps.push(verify(j));}assert_eq!(jobs.len(),19);assert_eq!(steps.len(),39);save(r("jobs.json"),&json!(jobs));
    for c in cases.as_array().unwrap(){assert_eq!(hash(c["image"].as_str().unwrap()),c["image_sha256"].as_str().unwrap());pin(&mut pins,c["image"].as_str().unwrap());pin(&mut pins,c["request"].as_str().unwrap());}
    for e in fs::read_dir("/home/ubuntu/src/zopro-perf-work/sdk-control-copies/lagrange-cache").unwrap(){pin(&mut pins,e.unwrap().path());}
    for f in [VERIFY,X,MANIFEST,ACCEPTED]{pin(&mut pins,f);}for f in ["native/candidate-sdk","code-audit.json","native/result.json","native/candidate-core/tests.json","native/candidate-prover/tests.json","flow-body.rs","bootstrap-flow.rs","flow.rs","flow-helper","jobs.json","cases.json","prior-proof-sha256.json"]{pin(&mut pins,r(f));}
    for e in fs::read_dir(r("codegen")).unwrap(){pin(&mut pins,e.unwrap().path());}
    save(r("pins.json"),&json!(pins));save(r("run-plan.json"),&json!({"version":1,"source_pins":r("pins.json"),"steps":steps}));println!("Prepared39steps346SDK19freshGnark17786->17805; semantic preflight before main");
}
fn finish(){
    assert_eq!(read(r("runs/result.json"))["status"],"complete");for(p,h)in read(r("pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
    let mut prior:BTreeSet<String>=serde_json::from_value(read(r("prior-proof-sha256.json"))).unwrap();assert_eq!(prior.len(),17786);let mut inventory=BTreeMap::new();let mut requests=0;let mut rows=vec![];let mut graphs=vec![];
    for j in read(r("jobs.json")).as_array().unwrap(){let s=stdout(j);let samples=jsonlines(&s,"SAMPLE ");assert_eq!(samples.len(),j["rounds"].as_u64().unwrap()as usize);let summary=jsonlines(&s,"SUMMARY ");assert_eq!(summary.len(),1);let summary=&summary[0];assert_eq!(summary["workers"],32);assert_eq!(summary["priming"],3);requests+=samples.len()+3;
        let dir=Path::new(j["directory"].as_str().unwrap());let proof=dir.join("0.proof");let public=dir.join("0.public");let ph=hash(&proof);let uh=hash(&public);assert_eq!(fs::metadata(&proof).unwrap().len(),164);assert!(prior.insert(ph.clone()));assert_eq!(uh,j["case"]["public_sha256"].as_str().unwrap());
        let verified=fs::read_to_string(r(format!("runs/verify-{}/stdout.log",j["name"].as_str().unwrap()))).unwrap();for marker in [format!("proof_sha256={ph}"),format!("public_sha256={uh}"),format!("image_sha256={}",j["case"]["image_sha256"].as_str().unwrap())]{assert!(verified.contains(&marker));}inventory.insert(proof.display().to_string(),ph);inventory.insert(public.display().to_string(),uh);
        if j["mode"]!="accepted"{for e in jsonlines(&s,"EVENTS "){assert_eq!(e["dropped"],0);if j["mode"]=="off"{assert_eq!(e["events"],0);}}
            assert!(summary["prime_dropped"].as_array().unwrap().iter().all(|x|x==0));
            let gs=jsonlines(&s,"GRAPH ");assert_eq!(gs.len(),if samples.len()==1{1}else{3});
            for g in gs{let round=g["round"].as_u64().unwrap()as usize;assert!([0,samples.len()/2,samples.len()-1].contains(&round));let v=graph(&g,samples[round]["ms"].as_f64().unwrap());if j["mode"]=="on"{assert_eq!(v["complete"],true);}graphs.push(json!({"job":j,"graph":v}));}
        }
        rows.push(json!({"job":j,"median_ms":med(samples.iter().map(|x|x["ms"].as_f64().unwrap()).collect()),"samples":samples,"summary":summary}));
    }
    assert_eq!(requests,346);assert_eq!(prior.len(),17805);assert_eq!(inventory.len(),38);
    let value=|mode:&str,block:usize|->f64{rows.iter().find(|r|r["job"]["phase"]=="main"&&r["job"]["mode"]==mode&&r["job"]["block"]==block).unwrap()["median_ms"].as_f64().unwrap()};
    let cal:Vec<_>=(0..3).map(|b|value("off",b)/value("accepted",b)).collect();let over:Vec<_>=(0..3).map(|b|value("on",b)/value("off",b)).collect();let calibration=cal.iter().all(|x|(1./1.05..=1.05).contains(x))&&(1./1.03..=1.03).contains(&med(cal.clone()));let overhead=med(over.clone())<=1.05;
    let main:Vec<_>=graphs.iter().filter(|g|g["job"]["phase"]=="main"&&g["job"]["mode"]=="on").collect();assert_eq!(main.len(),9);
    let mut last=BTreeMap::<String,usize>::new();for g in &main{*last.entry(g["graph"]["last_query"].as_str().unwrap().into()).or_default()+=1;}
    let mut qs=BTreeMap::new();for name in ["a","kh","g2"]{let qval=|key:&str|main.iter().map(|g|g["graph"]["queries"][name][key].as_f64().unwrap()).collect();let waits:Vec<f64>=main.iter().flat_map(|g|g["graph"]["queries"][name]["windows"].as_array().unwrap().iter().map(|w|w["input_available_to_start_ms"].as_f64().unwrap())).collect();qs.insert(name,json!({"median_query_elapsed_ms":med(qval("query_elapsed_ms")),"median_covered_serial_CPU_ms":med(qval("covered_serial_CPU_ms")),"median_pre_input_elapsed_ms":med(qval("pre_input_elapsed_ms")),"median_available_to_window_start_ms":med(waits.clone()),"max_available_to_window_start_ms":waits.into_iter().fold(0.,f64::max)}));}
    let mut modes=BTreeMap::new();for mode in ["accepted","off","on"]{let rr:Vec<_>=rows.iter().filter(|r|r["job"]["phase"]=="main"&&r["job"]["mode"]==mode).collect();modes.insert(mode,json!({"median_ms":med(rr.iter().map(|r|r["median_ms"].as_f64().unwrap()).collect()),"max_RSS_MiB":rr.iter().map(|r|r["summary"]["whole_process_maxrss_kib"].as_f64().unwrap()/1024.).fold(0.,f64::max)}));}
    let a=json!({"status":"completed sparse service diagnostic","SDK_requests":346,"new_independent_exports":19,"independent_proof_count":17805,"main_enabled_graphs":9,"preflight_enabled_graphs":1,"all_enabled_graphs_complete_zero_drops":true,"calibration_passed":calibration,"overhead_passed":overhead,"low_overhead_qualified":calibration&&overhead,"OFF_accepted_ratios":cal,"ON_OFF_ratios":over,"ON_OFF_median":med(over),"modes":modes,"last_query_counts":last,"queries":qs,"inherited_accepted_M5":read(MANIFEST)["m5"],"production_changed":false,"adopted":false,"goal_achieved":false,"goal_status":"active/unmet0of8","native_core_tests":126,"new_recorder_tests":2,"native_prover_tests":8,"field33_inherited":true,"PMU_captured":false});
    save(r("proof-inventory.json"),&json!(inventory));save(r("rows.json"),&json!(rows));save(r("graphs.json"),&json!(graphs));save(r("analysis.json"),&a);println!("Complete346SDK19Gnark17805; calibration{}overhead{} ratio{}; lastquery{}",calibration,overhead,a["ON_OFF_median"],a["last_query_counts"]);for(name,q)in qs{println!("{name} {q}");}
}
fn walk(p:&Path,out:&mut Vec<PathBuf>){for e in fs::read_dir(p).unwrap(){let e=e.unwrap();if e.file_type().unwrap().is_dir(){let n=e.file_name().to_string_lossy().to_string();if !n.starts_with("target-")&&n!="frozen"{walk(&e.path(),out);}}else if e.file_type().unwrap().is_file(){out.push(e.path());}}}
fn seal(){let d=Path::new(D);let mut ps=vec![];walk(d,&mut ps);let mut sums=BTreeMap::new();for p in ps{let rel=p.strip_prefix(d).unwrap().display().to_string();if rel!="SHA256SUMS"&&rel!="artifact-index.json"{sums.insert(rel,hash(&p));}}save(d.join("artifact-index.json"),&json!(sums));sums.insert("artifact-index.json".into(),hash(d.join("artifact-index.json")));fs::write(d.join("SHA256SUMS"),sums.iter().map(|(p,h)|format!("{h}  {p}\n")).collect::<String>()).unwrap();println!("Sealed{}hashes",sums.len());}
fn archive(){let d=Path::new(D);assert!(!d.exists());assert_eq!(read(r("analysis.json"))["independent_proof_count"],17805);for(p,h)in read(r("pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}fs::create_dir_all(d.join("artifacts")).unwrap();let mut ps=vec![];walk(Path::new(R),&mut ps);let mut binaries=BTreeMap::new();for p in ps{let name=p.file_name().unwrap().to_string_lossy();if ["commit.log","push.log","remote.log","strict-hashes.log","archive.log","diff-check.log"].contains(&name.as_ref()){continue;}let b=fs::read(&p).unwrap();if b.starts_with(b"\x7fELF")||b.starts_with(b"!<arch>"){binaries.insert(p.display().to_string(),hash(&p));continue;}let to=d.join("artifacts").join(p.strip_prefix(R).unwrap());fs::create_dir_all(to.parent().unwrap()).unwrap();fs::copy(p,to).unwrap();}save(d.join("artifacts/local-binary-pins.json"),&json!(binaries));let mut source=vec![];walk(&d.join("artifacts/source"),&mut source);assert_eq!(source.len(),176);report();seal();}
fn report(){use std::fmt::Write;let a=read(r("analysis.json"));let mut s=String::from("# Sparse service trace on accepted175\n\n");writeln!(s,"Low-overhead 32-worker qualification: **{}** (calibration {}, overhead {}; ON/OFF paired median {}). This diagnostic adopts no optimization. All nine saved main graphs and one preflight graph are complete with zero drops.\n",a["low_overhead_qualified"],a["calibration_passed"],a["overhead_passed"],a["ON_OFF_median"]).unwrap();
    s.push_str("## Observed requests\n\n| Mode | Median ms | Whole-process maximum RSS MiB |\n|---|---:|---:|\n");for mode in ["accepted","off","on"]{let m=&a["modes"][mode];writeln!(s,"| {mode} | {:.6} | {:.2} |",m["median_ms"].as_f64().unwrap(),m["max_RSS_MiB"].as_f64().unwrap()).unwrap();}
    writeln!(s,"\nLast query counts under instrumentation: `{}`. This names the latest query boundary; it is not a removable critical-path duration or a causal estimate of uninstrumented latency.\n",a["last_query_counts"]).unwrap();
    s.push_str("| Query | Median query elapsed ms | Covered serial CPU ms | Pre-input elapsed ms | Median/max available-to-window start ms |\n|---|---:|---:|---:|---:|\n");for(name,q)in a["queries"].as_object().unwrap(){writeln!(s,"| {name} | {:.6} | {:.6} | {:.6} | {:.6}/{:.6} |",q["median_query_elapsed_ms"].as_f64().unwrap(),q["median_covered_serial_CPU_ms"].as_f64().unwrap(),q["median_pre_input_elapsed_ms"].as_f64().unwrap(),q["median_available_to_window_start_ms"].as_f64().unwrap(),q["max_available_to_window_start_ms"].as_f64().unwrap()).unwrap();}
    writeln!(s,"\nOFF/accepted block ratios: `{}`; ON/OFF: `{}`. Calibration requires every block within factor 1.05 and paired median within factor 1.03; ON/OFF median overhead must be at most 1.05. Thresholds and all observations are retained. Failure leaves graphs descriptive of the instrumented schedule.\n",a["OFF_accepted_ratios"],a["ON_OFF_ratios"]).unwrap();
    s.push_str("## Scope and coverage\n\nCurrent source175 includes balanced K/H and borrowed A/G2 paths. Query roots cover canonical A, fused K/H and exclusive G2, including grouping. Explicit parent IDs cross new window/part and merge/group/indexed/weighted callbacks. Serial CPU is recorded only at nonforking feeds, pre-normalization tails, phi maps and leaf callbacks. It includes allocation, data movement and recorder work; it is not a field-cycle count. Input markers are mathematical entry/readiness boundaries, not enqueue timestamps. Solving, FFT, scattering, assembly, verification and uncovered normalization remain outside covered serial service.\n\nFixed1024 events of80 bytes per worker/caller bound recorder payload to2.579MiB at32 workers plus metadata and snapshots. Driver resets/snapshots are outside request samples but can affect context; loop CPU includes that observer work. All31 observations and drop counts remain; graphs only at0/15/30 by prior declaration. No buffer growth, exclusions, sample extension or successful rerun. These allocations belong only to the diagnostic.\n\nFresh126 core and8prover tests pass, including two recorder fidelity/drop tests.17 field files are exact;33 field tests are inherited. Eight cfg-only originals plus one new child recover all175 original bytes;167 originals stay exact. All actual arithmetic, field, scheduler, scalar split, key and plan logic is unchanged. Semantic preflight requires actual canonical/borrowed A, balanced20-window K/H,11-window phi2 G2 and consistent parent/containment/serial clocks before main capture. No nested serial spans or serial overlaps per worker are accepted.\n\n## CPU and Metal references\n\nThe all-eight comparison below is inherited from accepted175, not newly measured here. No instrumented speedup or candidate all-shape performance qualification is claimed.\n\n| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup | CPU10× target ms |\n|---|---:|---:|---:|---:|---:|---:|\n");
    for name in ["1x2","2x2","4x2","8x2","12x2","16x2","24x2","merge-24x1"]{let p=&a["inherited_accepted_M5"][name];writeln!(s,"| {name} | {:.6} | {} | {:.3}× | {} | {:.3}× | {} |",p["accepted_ms"].as_f64().unwrap(),p["cpu_reference_ms"],p["cpu_speedup"].as_f64().unwrap(),p["metal_reference_ms"],p["metal_speedup"].as_f64().unwrap(),p["required_ms_for_CPU10x"]).unwrap();}
    s.push_str("\nAccepted source175/SDK20fde remains qualified1x2 8.022165ms/max134MiB and54→1 162.250464ms (no supplied M5 reference). Physical16 baseline-calibration engineering exception remains. All8CPU10x goal remains active/unmet0/8; primary needs3.7ms. No new AWS/Aeglos,16-worker, mobile/ARM/TEE, huge250/4999, pressure/swap/minRAM/concurrency/throughput result.\n\n## Provenance\n\nAll39 workflow steps complete:346SDK requests verify internally;19fresh164-byte exports independently pass Gnark and exact image/public binding and freshness against17786 prior identities. Ledger17805;38 proof/public members represent19proofs. No owned build/helper/hash/archive/other workload overlapped capture. Source/image/library/driver/helper pins close. [Design](artifacts/design.md), [source audit](artifacts/source-audit.json), [semantic preflight](artifacts/semantic-preflight.json), [graphs](artifacts/graphs.json), [observations](artifacts/rows.json), [analysis](artifacts/analysis.json), [proof inventory](artifacts/proof-inventory.json).\n");fs::write(Path::new(D).join("README.md"),s).unwrap();}
fn main(){match std::env::args().nth(1).as_deref(){Some("prepare")=>prepare(),Some("semantic")=>semantic(),Some("finish")=>finish(),Some("archive")=>archive(),Some("seal")=>seal(),_=>panic!("prepare|semantic|finish|archive|seal")}}
