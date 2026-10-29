const D:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-g2-gls4-census177";
const VERIFY:&str="/home/ubuntu/src/zopro-perf-work/batch-local-steal/gnark/verify";
const X:&str="/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";
const MANIFEST:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json";
const SUMS:[&str;18]=["input","skip_zero","skip_identity","nonzero","direct","push","pop","flushed","assigned","doubled","cancelled","ordinary","executed","batches","proj_init","proj_update","distinct","sum_squared_arrivals"];
fn n(v:&Value)->u64 {v.as_u64().unwrap()}
fn prepare(){
    assert!(!r("run-plan.json").exists());assert_eq!(read(r("native/result.json"))["status"],"complete");
    let mut pins:BTreeMap<String,String>=serde_json::from_value(read(r("build-pins.json"))).unwrap();for(p,h)in &pins{assert_eq!(hash(p),*h);}
    let tests=read(r("native/census-tests/tests.json"));assert_eq!(tests["passed_tests"].as_array().unwrap().len(),2);assert_eq!(tests["unfiltered"],false);
    let a=read(r("source-audit.json"));assert_eq!(a["source_files"],178);assert_eq!(a["parent_files_exact"],176);assert_eq!(a["parent_recovered_by_inverse_edits"],true);assert_eq!(a["field_source_files_exact"],17);
    assert_eq!(read(r("code-audit.json"))["linked_observer_present"],true);
    let artifact=read(r("native/candidate/artifacts.json"));assert_eq!(hash(artifact["binary"].as_str().unwrap()),artifact["binary_sha256"].as_str().unwrap());assert_eq!(artifact["libraries"].as_object().unwrap().len(),8);
    for (p,h)in artifact["input_pins"].as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());pin(&mut pins,p);}
    for (p,h)in read(r("frozen-candidate.json"))["input_pins"].as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());pin(&mut pins,p);}
    let case=read("/home/ubuntu/src/zopro-perf-work/sdk-control-copies/cases.json")[0].clone();assert_eq!(case["name"],"1x2-w32");
    assert_eq!(hash(case["image"].as_str().unwrap()),case["image_sha256"].as_str().unwrap());pin(&mut pins,case["image"].as_str().unwrap());pin(&mut pins,case["request"].as_str().unwrap());save(r("case.json"),&case);
    let mut prior:BTreeSet<String>=serde_json::from_value(read(format!("{PARENT}/prior-proof-sha256.json"))).unwrap();assert_eq!(prior.len(),17473);
    for (p,h)in read(format!("{PARENT}/proof-inventory.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());pin(&mut pins,p);if p.ends_with(".proof"){assert!(prior.insert(h.as_str().unwrap().into()));}}
    assert_eq!(prior.len(),17551);save(r("prior-proof-sha256.json"),&json!(prior));
    let mut env=read(MANIFEST)["environment"].clone();env["RAYON_NUM_THREADS"]=json!("32");env["ZOPRO_GLS4_CENSUS"]=json!("1");
    let steps=json!([
        {"kind":"command","name":"primary-census","argv":["taskset","-c","0-31",r("native/candidate-sdk"),case["image"],case["request"],"1",r("proofs")],"environment":env,"timeout_seconds":900},
        {"kind":"command","name":"verify-primary","argv":[VERIFY,case["image"],case["image_sha256"],r("proofs/0.proof"),r("proofs/0.public"),case["public_sha256"]],"timeout_seconds":120}
    ]);
    for p in [VERIFY,X,MANIFEST]{pin(&mut pins,p);}
    for p in ["design.md","census.rs","setup.rs","native.rs","native-helper","bootstrap.rs","bootstrap-flow.rs","flow-body.rs","flow.rs","flow-helper","code-audit.json","code-audit.rs","native/result.json","native/census-tests/tests.json","native/candidate/artifacts.json","native/candidate-sdk","frozen-candidate.json","case.json","prior-proof-sha256.json"]{pin(&mut pins,r(p));}
    for f in fs::read_dir(r("codegen")).unwrap(){pin(&mut pins,f.unwrap().path());}
    save(r("pins.json"),&json!(pins));save(r("run-plan.json"),&json!({"version":1,"source_pins":r("pins.json"),"steps":steps}));
    println!("Prepared2steps/4SDK/1newGnark/384expectedrecords; ledger17551->17552; all timings noncomparative");
}
fn validate(row:&Value){
    let x=|k:&str|n(&row[k]);assert_eq!(row["flow_checked"],true);assert_eq!(x("input"),x("skip_zero")+x("skip_identity")+x("nonzero"));
    assert_eq!(x("nonzero"),x("direct")+x("push"));assert_eq!(x("push"),x("pop")+x("flushed"));
    assert_eq!(x("direct")+x("pop"),x("assigned")+x("doubled")+x("cancelled")+x("ordinary"));assert_eq!(x("ordinary"),x("executed"));assert_eq!(x("doubled")+x("flushed"),x("proj_init")+x("proj_update"));
    let h=row["batch_hist"].as_array().unwrap();assert_eq!(h.len(),153);assert_eq!(x("batches"),h.iter().map(n).sum::<u64>());assert_eq!(x("executed"),h.iter().enumerate().map(|(i,v)|i as u64*n(v)).sum::<u64>());
    assert!(x("distinct")<=1024&&x("max_digit")<=1024);assert!(x("distinct")<=x("nonzero"));
    let p=x("part")as usize;let start=(p*7523).min(30089);let end=(start+7523).min(30089);assert_eq!(x("input")as usize,end-start);
}
fn aggregate(rows:&[Value])->Value {
    let mut out=serde_json::Map::new();for k in SUMS{out.insert(k.into(),json!(rows.iter().map(|r|n(&r[k])).sum::<u64>()));}
    let hist:Vec<_>=(0..153).map(|i|rows.iter().map(|r|n(&r["batch_hist"][i])).sum::<u64>()).collect();
    out.insert("batch_hist".into(),json!(hist));out.insert("scalar_tail_entries".into(),json!(hist.iter().enumerate().map(|(i,k)|(i%8)as u64*k).sum::<u64>()));
    out.insert("max_digit".into(),json!(rows.iter().map(|r|n(&r["max_digit"])).max().unwrap_or(0)));
    out.insert("distinct_min".into(),json!(rows.iter().map(|r|n(&r["distinct"])).min().unwrap_or(0)));out.insert("distinct_max".into(),json!(rows.iter().map(|r|n(&r["distinct"])).max().unwrap_or(0)));
    out.insert("max_arrivals".into(),json!(rows.iter().map(|r|n(&r["max_arrivals"])).max().unwrap_or(0)));Value::Object(out)
}
fn finish(){
    assert_eq!(read(r("runs/result.json"))["status"],"complete");for(p,h)in read(r("pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
    let case=read(r("case.json"));let stdout=fs::read_to_string(r("runs/primary-census/stdout.log")).unwrap();let summary:Value=serde_json::from_str(stdout.lines().find_map(|l|l.strip_prefix("SUMMARY ")).unwrap()).unwrap();assert_eq!(summary["rounds"],1);assert_eq!(summary["priming"],3);assert_eq!(summary["workers"],32);
    let mut records=BTreeMap::<(u64,u64,u64,u64),Value>::new();
    for line in fs::read_to_string(r("runs/primary-census/stderr.log")).unwrap().lines(){if let Some(s)=line.strip_prefix("GLS4_CENSUS "){let row:Value=serde_json::from_str(s).unwrap();validate(&row);let key=(n(&row["query"]),n(&row["window"]),n(&row["part"]),n(&row["component"]));assert!(records.insert(key,row).is_none());}}
    assert_eq!(records.len(),384);for q in 1..=4 {for w in 0..6 {for p in 0..4 {for c in 0..4 {assert!(records.contains_key(&(q,w,p,c)));}}}}
    for q in 1..=4 {for w in 0..6 {for p in 0..4 {assert!((0..4).map(|c|n(&records[&(q,w,p,c)]["proj_init"])).sum::<u64>()<=1024);}}}
    let all:Vec<_>=records.values().cloned().collect();save(r("records.json"),&json!(all));
    let mut queries=serde_json::Map::new();for q in 1..=4 {
        let rows:Vec<_>=records.iter().filter(|(k,_)|k.0==q).map(|(_,r)|r.clone()).collect();let total=aggregate(&rows);assert_eq!(n(&total["input"]),30089*24);
        let mut windows=serde_json::Map::new();let mut components=serde_json::Map::new();
        for w in 0..6 {let selected:Vec<_>=rows.iter().filter(|r|n(&r["window"])==w).cloned().collect();windows.insert(w.to_string(),aggregate(&selected));
            for c in 0..4 {let selected:Vec<_>=rows.iter().filter(|r|n(&r["window"])==w&&n(&r["component"])==c).cloned().collect();components.insert(format!("{w}/{c}"),aggregate(&selected));}}
        queries.insert(q.to_string(),json!({"totals":total,"windows":windows,"window_components":components}));
    }
    let mut equivalent=true;for q in 2..=4 {for w in 0..6 {for p in 0..4 {for c in 0..4 {let mut a=records[&(1,w,p,c)].clone();let mut b=records[&(q,w,p,c)].clone();a.as_object_mut().unwrap().remove("query");b.as_object_mut().unwrap().remove("query");equivalent&=a==b;}}}}
    let proof=r("proofs/0.proof");let public=r("proofs/0.public");assert_eq!(fs::metadata(&proof).unwrap().len(),164);let ph=hash(&proof);let uh=hash(&public);assert_eq!(uh,case["public_sha256"].as_str().unwrap());
    let verify=fs::read_to_string(r("runs/verify-primary/stdout.log")).unwrap();for item in [format!("proof_sha256={ph}"),format!("public_sha256={uh}"),format!("image_sha256={}",case["image_sha256"].as_str().unwrap())]{assert!(verify.contains(&item));}assert!(verify.contains("VERIFIED "));
    let mut prior:BTreeSet<String>=serde_json::from_value(read(r("prior-proof-sha256.json"))).unwrap();assert_eq!(prior.len(),17551);assert!(prior.insert(ph.clone()));assert_eq!(prior.len(),17552);
    save(r("proof-inventory.json"),&json!({proof.display().to_string():ph,public.display().to_string():uh}));
    let a=json!({"status":"diagnostic complete; no timing/gain claim","SDK_requests":4,"new_independent_exports":1,"independent_proof_count":17552,"records":384,"queries":queries,"counts_match_across_all_requests":equivalent,"displayed_query":4,"flow_checks_passed":true,"new_observer_tests_passed":2,"existing_tests_filtered":128,"native128core8prover33field_inherited":true,"field17_source_exact":true,"production_changed":false,"candidate_parent_remains_rejected":true,"goal_status":"active/unmet0of8","goal_achieved":false,"inherited_accepted_M5":read(MANIFEST)["m5"],"instrumented_timings_used":false});
    save(r("analysis.json"),&a);println!("Diagnosticcomplete384records/4SDK/1freshGnark; ledger17552; counts_equal={equivalent}");
    for(w,v)in a["queries"]["4"]["windows"].as_object().unwrap(){println!("window{w} nonzero{} batch{} projective_updates{} flushed{} destinations{}..{} maxdigit{}",v["nonzero"],v["executed"],v["proj_update"],v["flushed"],v["distinct_min"],v["distinct_max"],v["max_digit"]);}
}
fn walk(p:&Path,out:&mut Vec<PathBuf>){for e in fs::read_dir(p).unwrap(){let e=e.unwrap();let name=e.file_name().to_string_lossy().to_string();if e.file_type().unwrap().is_dir(){if !name.starts_with("target-")&&name!="frozen"{walk(&e.path(),out);}}else if e.file_type().unwrap().is_file(){out.push(e.path());}}}
fn report(){
    use std::fmt::Write;
    let a=read(r("analysis.json"));let q=&a["queries"]["4"];let mut s=String::from("# GLS4 deferred-point census\n\n");
    writeln!(s,"All **384 feed records** pass independent flow checks. The diagnostic covers four complete primary SDK requests (three primes and one observed request); the final fresh export independently verifies with Gnark. Counts match across all four requests: **{}**. This instrumented run establishes no latency or speedup. The [parent GLS4 implementation](../2026-10-29-g2-gls4-horner175/README.md) remains rejected.\n",a["counts_match_across_all_requests"]).unwrap();
    s.push_str("## Observed request\n\nThe displayed fourth request follows the declared SDK boundary. All four requests are retained in [analysis](artifacts/analysis.json) and [raw structured records](artifacts/records.json). Each window row sums four partitions and four component passes. Distinct destinations are the minimum/maximum per partition/component; they are not one combined set.\n\n| Window (low to high) | Nonzero arrivals | Affine batch entries | Queue-flush entries | Projective initializations | Existing projective updates | Distinct destinations | Max absolute digit |\n|---|---:|---:|---:|---:|---:|---|---:|\n");
    for(w,v)in q["windows"].as_object().unwrap(){writeln!(s,"| {w} | {} | {} | {} | {} | {} | {}–{} | {} |",v["nonzero"],v["executed"],v["flushed"],v["proj_init"],v["proj_update"],v["distinct_min"],v["distinct_max"],v["max_digit"]).unwrap();}
    let total=n(&q["totals"]["proj_update"]);let top=n(&q["windows"]["5"]["proj_update"]);
    writeln!(s,"\nThe highest window contributes **{top}/{total} ({:.3}%)** of existing projective updates in these feeds. The census also records equal-point deferrals, opposite cancellations, initializer counts and batch sizes separately. An initializer stores a projective point; an existing update uses the original mixed-add operation. Final normalization/combining, weighted reductions and transforms are outside these feed counters. Counts cannot isolate cycle cost or explain the entire parent latency regression.\n",100.*top as f64/total.max(1)as f64).unwrap();
    s.push_str("## Observer fidelity and scope\n\nThe observer wraps original add/execute/flush methods and follows the original LIFO drain order. It reads a fixed seen bitmap to count repeated flush destinations before calling the original flush once. Scoped thread-local context identifies nested worker jobs. Each record checks input/skip flow, direct/queued flow, queue pop/flush conservation, insertion cases, executed affine entries and projective initialization/update totals. The analyzer independently repeats the checks, verifies all query/window/partition/component tuples and bound checks, and checks batch histograms.\n\nTwo fresh observer tests pass; 128 existing tests are filtered out. The tests compare exact affine/projective state with original feeds through all four map passes and check an independently specified doubling/cancellation/queue example and its final generator sum. Parent 128 core, eight prover and 33 field tests are inherited, with inverse edits recovering all 177 parent files and all 17 field files exact. The new snapshot contains 178 files; 176 parent files remain byte-identical. Production source 175 remains untouched. No new field, queue, decomposition, width/partition or input algorithm is selected here.\n\nThe standard SDK driver retains raw elapsed/usage output. Those numbers include counters and synchronized output and are excluded from performance conclusions. There is no new PMU/profile capture, unchanged latency rerun, all-shape candidate performance, AWS/Aeglos, phone, pressure, giant-circuit or minimum-RAM claim.\n\n## Inherited CPU and Metal comparison\n\nThese are the accepted source-175 timings, not new measurements. The all-eight CPU 10× objective remains active and unmet, 0/8.\n\n| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup | CPU 10× target ms |\n|---|---:|---:|---:|---:|---:|---:|\n");
    for shape in ["1x2","2x2","4x2","8x2","12x2","16x2","24x2","merge-24x1"]{let v=&a["inherited_accepted_M5"][shape];writeln!(s,"| {shape} | {:.6} | {:.0} | {:.3}× | {:.0} | {:.3}× | {:.1} |",v["accepted_ms"].as_f64().unwrap(),v["cpu_reference_ms"].as_f64().unwrap(),v["cpu_speedup"].as_f64().unwrap(),v["metal_reference_ms"].as_f64().unwrap(),v["metal_speedup"].as_f64().unwrap(),v["required_ms_for_CPU10x"].as_f64().unwrap()).unwrap();}
    s.push_str("\nAccepted primary remains 8.022165 ms / 134 MiB peak RSS; merge54 remains 162.250464 ms with no supplied M5 reference. The accepted physical-16 baseline-calibration engineering exception is retained. This census does not qualify a product change.\n\n## Provenance\n\nFour SDK requests verify internally; one fresh 164-byte export independently passes Gnark with exact image/public binding and freshness against 17,551 prior identities. The ledger is 17,552; two archived proof/public members represent one proof. Source, image, library/driver/helper and observation pins close. Native binaries/rlibs remain local with hashes. [Design](artifacts/design.md), [observer](artifacts/source/crates/zopro-core/src/msm_affine/g2_borrowed/gls4/census.rs), [source audit](artifacts/source-audit.json), [new tests](artifacts/native/census-tests/tests.json), [workflow](artifacts/runs/result.json), [proof inventory](artifacts/proof-inventory.json).\n");
    fs::write(Path::new(D).join("README.md"),s).unwrap();
}
fn seal(){let d=Path::new(D);let mut paths=vec![];walk(d,&mut paths);let mut sums=BTreeMap::new();for p in paths{let rel=p.strip_prefix(d).unwrap().display().to_string();if rel!="SHA256SUMS"&&rel!="artifact-index.json"{sums.insert(rel,hash(p));}}
    save(d.join("artifact-index.json"),&json!(sums));sums.insert("artifact-index.json".into(),hash(d.join("artifact-index.json")));fs::write(d.join("SHA256SUMS"),sums.iter().map(|(p,h)|format!("{h}  {p}\n")).collect::<String>()).unwrap();println!("Sealed{}hashes",sums.len());}
fn archive(){let d=Path::new(D);assert!(!d.exists());let a=read(r("analysis.json"));assert_eq!(a["records"],384);assert_eq!(a["new_independent_exports"],1);assert_eq!(a["production_changed"],false);for(p,h)in read(r("pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
    fs::create_dir_all(d.join("artifacts")).unwrap();let mut paths=vec![];walk(Path::new(R),&mut paths);let mut binaries=BTreeMap::new();for p in paths{let name=p.file_name().unwrap().to_str().unwrap();if name.starts_with("git-")||["archive.log","archive.stderr.log","strict-hashes.log","commit.log","push.log","remote.log","seal.log"].contains(&name){continue;}
        let mut f=fs::File::open(&p).unwrap();let mut header=[0;8];let len=f.read(&mut header).unwrap();if header[..len].starts_with(b"\x7fELF")||header[..len].starts_with(b"!<arch>"){binaries.insert(p.display().to_string(),hash(&p));continue;}
        let to=d.join("artifacts").join(p.strip_prefix(R).unwrap());fs::create_dir_all(to.parent().unwrap()).unwrap();fs::copy(&p,&to).unwrap();assert_eq!(hash(&p),hash(&to));}
    save(d.join("artifacts/local-binary-pins.json"),&json!(binaries));let mut source=vec![];walk(&d.join("artifacts/source"),&mut source);assert_eq!(source.len(),178);report();seal();}
fn main(){match std::env::args().nth(1).as_deref(){Some("prepare")=>prepare(),Some("finish")=>finish(),Some("archive")=>archive(),Some("seal")=>seal(),_=>panic!("prepare|finish|archive|seal")}}
