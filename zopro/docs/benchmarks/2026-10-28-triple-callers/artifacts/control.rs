use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::{BTreeMap,BTreeSet},fs,io::Read,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/triple-callers";
const Z:&str="/home/ubuntu/src/zk-tools/zopro";
const D:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-triple-callers";
const X:&str="/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";
fn read(p:impl AsRef<Path>)->Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() }
fn save(p:impl AsRef<Path>,v:&Value) { fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap(); }
fn hash(p:impl AsRef<Path>)->String {
    let mut f=fs::File::open(p).unwrap(); let mut h=Sha256::new(); let mut b=[0;65536];
    loop { let n=f.read(&mut b).unwrap(); if n==0 {break;} h.update(&b[..n]); }
    format!("{:x}",h.finalize())
}
fn pin(m:&mut BTreeMap<String,String>,p:impl AsRef<Path>) { let p=p.as_ref();m.insert(p.display().to_string(),hash(p)); }
fn cases()->Vec<(String,String,usize,usize)> {
    ["g1","g2"].into_iter().flat_map(|kind|[(8,8192),(16,4096),(32,2048),(64,1024)].map(move |(m,n)|(format!("{kind}-{m}"),kind.into(),m,n))).collect()
}
fn env()->BTreeMap<String,String> {
    [("ZOPRO_CPU_G2_PREFIX_RECORDS","1"),("ZOPRO_CPU_DIRECT","1"),("ZOPRO_CPU_CANONICAL_BATCH","1"),("ZOPRO_CPU_VECTOR_INVERSE","1"),("ZOPRO_CPU_G1_SCRATCH_STORE","2"),("RAYON_NUM_THREADS","1"),("MALLOC_ARENA_MAX","32"),("MALLOC_MMAP_THRESHOLD_","1073741824"),("MALLOC_TRIM_THRESHOLD_","-1")].into_iter().map(|(k,v)|(k.into(),v.into())).collect()
}
fn step(name:String,argv:Vec<String>)->Value { json!({"kind":"command","name":name,"argv":argv,"timeout_seconds":240}) }
fn prepare() {
    let r=Path::new(R);assert!(!r.join("run-plan.json").exists());
    let tests=fs::read_to_string(r.join("tests.log")).unwrap();
    assert!(tests.contains("test result: ok. 2 passed; 0 failed"));
    for name in ["g1_three_point_native_and_complete_fallback","g2_three_point_native_and_complete_fallback"] { assert!(tests.contains(name)); }
    let mut pins=BTreeMap::new(); let mut changed=Vec::new();
    let manifest=read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json"));
    for (name,h) in manifest["source_files_sha256"].as_object().unwrap() {
        let p=Path::new(Z).join(name); assert_eq!(hash(&p),h.as_str().unwrap());pin(&mut pins,&p);
        let local=r.join("source").join(name);pin(&mut pins,&local);
        if hash(&local)!=h.as_str().unwrap() { changed.push(name.clone()); }
    }
    assert_eq!(changed,["crates/zopro-core/src/lib.rs","crates/zopro-core/src/msm.rs"]);
    for (name,suffix) in [
        ("crates/zopro-core/src/msm.rs","\n#[cfg(zopro_triple_probe)]\nmod triple_probe;\n#[cfg(zopro_triple_probe)]\npub use triple_probe::{TripleG1,TripleG2};\n"),
        ("crates/zopro-core/src/lib.rs","\n#[cfg(zopro_triple_probe)]\npub use msm::{TripleG1,TripleG2};\n")
    ] {
        let local=fs::read_to_string(r.join("source").join(name)).unwrap();
        assert_eq!(local.strip_suffix(suffix).unwrap(),fs::read_to_string(Path::new(Z).join(name)).unwrap());
    }
    let module=r.join("source/crates/zopro-core/src/msm/triple_probe.rs");
    assert_eq!(hash(&module),hash(r.join("triple_probe.rs")));pin(&mut pins,module);
    assert_eq!(hash(r.join("src/bin/probe.rs")),hash(r.join("probe.rs")));
    for name in ["design.md","control.rs","control","probe.rs","src/bin/probe.rs","triple_probe.rs","native-flags.txt","Cargo.toml","Cargo.lock","test-build.log","tests.log","probe-build.log","probe","target/release/deps/probe-cdbc48cffef0052f","host.txt","rustc.txt"] {pin(&mut pins,r.join(name));}
    for e in fs::read_dir(r.join("target/release/deps")).unwrap() {
        let e=e.unwrap(); if e.file_name().to_string_lossy().ends_with(".rlib") {pin(&mut pins,e.path());}
    }
    pin(&mut pins,X);
    let mut steps=Vec::new(); let cs=cases();
    for (case,kind,size,_) in &cs { for variant in ["binary","direct"] {
        steps.push(step(format!("preflight-{case}-{variant}"),vec!["taskset".into(),"-c".into(),"2".into(),format!("{R}/probe"),kind.clone(),size.to_string(),"17".into(),variant.into()]));
    }}
    for b in 0..31 {
        let mut order:Vec<_>=cs.iter().flat_map(|c|["binary","direct"].map(move |v|(c,v))).collect();
        let len=order.len();order.rotate_left(b*7%len);if b%2!=0 {order.reverse();}
        for ((case,kind,size,n),variant) in order {
            steps.push(step(format!("lat-b{b:02}-{case}-{variant}"),vec!["taskset".into(),"-c".into(),"2".into(),format!("{R}/probe"),kind.clone(),size.to_string(),n.to_string(),variant.into()]));
        }
    }
    let environment=env();
    for kind in ["g1","g2"] {for variant in ["binary","direct"] {
        let key=format!("{kind}-{variant}");let p=r.join("profiles").join(&key);fs::create_dir_all(&p).unwrap();
        let mut argv=vec!["sudo".into(),"-n".into(),"env".into()];
        argv.extend(environment.iter().map(|(k,v)|format!("{k}={v}")));
        argv.extend(["perf".into(),"record".into(),"--no-buildid-cache".into(),"-e".into(),"cycles:u".into(),"-F".into(),"199".into(),"-o".into(),p.join("perf.data").display().to_string(),"--".into(),"taskset".into(),"-c".into(),"2".into(),format!("{R}/probe"),kind.into(),"8".into(),"1048576".into(),variant.into()]);
        steps.push(step(format!("profile-{key}"),argv));
    }}
    for kind in ["g1","g2"] {for variant in ["binary","direct"] {
        let key=format!("{kind}-{variant}");let file=format!("{R}/profiles/{key}/perf.data");
        steps.push(step(format!("owner-{key}"),vec!["sudo".into(),"-n".into(),"chown".into(),"ubuntu:ubuntu".into(),file.clone()]));
        steps.push(step(format!("decode-{key}"),vec!["perf".into(),"script".into(),"--header".into(),"-i".into(),file.clone(),"-F".into(),"pid,tid,period,ip,sym,dso".into()]));
        steps.push(step(format!("loss-{key}"),vec!["perf".into(),"script".into(),"-D".into(),"-i".into(),file]));
    }}
    save(r.join("pins.json"),&json!(pins));
    save(r.join("source-audit.json"),&json!({"production_files_unchanged":172,"private_original_files_unchanged":170,"private_original_files_changed":changed,"inverse_edits_recover_originals":true,"new_module":"msm/triple_probe.rs","field_backend_changed":false,"binary_stream_functions_unchanged":true}));
    save(r.join("run-plan.json"),&json!({"version":1,"source_pins":r.join("pins.json"),"environment":environment,"steps":steps}));
    save(r.join("declaration.json"),&json!({"cases":cs,"blocks":31,"latency_processes":496,"preflights":16,"profiles":4,"prime_calls":3,"prime_rounds":8,"same_binary_for_both_algorithms":true,"pin_count":pins.len(),"gate":"percase upper ratio<1 and>=21wins; correctness and zero timed allocations; no familywise guarantee","SDK_requests":0,"proof_ledger":12524,"goal_achieved":false}));
    println!("Prepared496latency+16preflight+4profiles,{}pins",pins.len());
}
fn result(name:&str)->Value {
    let text=fs::read_to_string(format!("{R}/runs/{name}/stdout.log")).unwrap();
    let lines:Vec<_>=text.lines().filter_map(|l|l.strip_prefix("RESULT ")).collect();assert_eq!(lines.len(),1);
    let v:Value=serde_json::from_str(lines[0]).unwrap();assert_eq!(v["internally_verified"],true);assert_eq!(v["timed_allocations"],0);v
}
fn stats(mut values:Vec<f64>)->Value {
    assert_eq!(values.len(),31);let all=values.clone();values.sort_by(f64::total_cmp);
    json!({"median":values[15],"low":values[9],"high":values[21],"wins":values.iter().filter(|&&x|x<1.).count(),"all_ratios":all})
}
fn median(mut values:Vec<f64>)->f64 {values.sort_by(f64::total_cmp);values[values.len()/2]}
fn archive(root:&Path,dir:&Path,dest:&Path,index:&mut BTreeMap<String,String>) {
    for e in fs::read_dir(dir).unwrap() {
        let e=e.unwrap();let p=e.path();let name=e.file_name().to_string_lossy().to_string();
        if e.file_type().unwrap().is_dir() {
            if ["source","target","profiles"].contains(&name.as_str()) {continue;}
            archive(root,&p,dest,index);
        } else if ["rs","json","md","log","toml","lock","txt","s"].contains(&p.extension().and_then(|s|s.to_str()).unwrap_or(""))
            && !["workflow.log","close.log","archive-verification.log"].contains(&name.as_str()) {
            let rel=p.strip_prefix(root).unwrap();let target=dest.join(rel);fs::create_dir_all(target.parent().unwrap()).unwrap();fs::copy(&p,&target).unwrap();index.insert(format!("artifacts/{}",rel.display()),hash(target));
        }
    }
}
fn finish() {
    let r=Path::new(R);for (p,h) in read(r.join("pins.json")).as_object().unwrap() {assert_eq!(hash(p),h.as_str().unwrap(),"{p}");}
    assert_eq!(read(r.join("runs/result.json"))["status"],"complete");
    let mut rows=Vec::new();let mut eligible=Vec::new();
    for (case,kind,size,n) in cases() {
        for v in ["binary","direct"] {assert_eq!(result(&format!("preflight-{case}-{v}"))["iterations"],17);}
        let mut times=BTreeMap::new();let mut memory=BTreeMap::new();let mut output=None;
        for variant in ["binary","direct"] {
            let mut t=Vec::new();
            for b in 0..31 {
                let v=result(&format!("lat-b{b:02}-{case}-{variant}"));assert_eq!(v["iterations"],n);
                if let Some(h)=&output {assert_eq!(h,&v["output_sha256"]);}else{output=Some(v["output_sha256"].clone());}
                let mem=json!({"direct_capacity_bytes":v["direct_scratch_capacity_bytes"],"binary_capacity_bytes":v["binary_scratch_capacity_bytes"],"direct_record_bytes":v["direct_record_bytes"]});
                if let Some(old)=memory.get(variant) {assert_eq!(old,&mem);}else{memory.insert(variant,mem);}
                t.push(v["ns_per_round"].as_f64().unwrap());
            }
            times.insert(variant,t);
        }
        let ratio=stats(times["direct"].iter().zip(&times["binary"]).map(|(a,b)|a/b).collect());
        let passed=ratio["high"].as_f64().unwrap()<1.&&ratio["wins"].as_u64().unwrap()>=21;
        if passed {eligible.push(case.clone());}
        rows.push(json!({"case":case,"kind":kind,"triples":size,"iterations":n,"median_binary_ns":median(times["binary"].clone()),"median_direct_ns":median(times["direct"].clone()),"direct_vs_binary":ratio,"caller_gain_gate":passed,"scratch_capacity":memory,"output_sha256":output}));
    }
    let mut profiles=BTreeMap::new();let mut raw=BTreeMap::new();
    for kind in ["g1","g2"] {for variant in ["binary","direct"] {
        let key=format!("{kind}-{variant}");let v=result(&format!("profile-{key}"));assert_eq!(v["iterations"],1048576);
        let decode=fs::read_to_string(r.join(format!("runs/decode-{key}/stdout.log"))).unwrap();
        let loss=r.join(format!("runs/loss-{key}/stdout.log"));assert!(!fs::read_to_string(&loss).unwrap().contains("PERF_RECORD_LOST"));
        let mut pids=BTreeSet::new();let mut samples=0;let mut symbols=BTreeMap::<String,u64>::new();let mut period=0;
        for line in decode.lines().filter(|s|!s.trim_start().starts_with('#')) {
            let w:Vec<_>=line.split_whitespace().collect();if w.len()<5 {continue;}
            let parsed=if let Some((pid,_))=w[0].split_once('/') {pid.parse::<u64>().ok().zip(w[1].parse::<u64>().ok()).map(|(p,n)|(p,n,3))}
            else if w.len()>=6 {w[0].parse::<u64>().ok().zip(w[2].parse::<u64>().ok()).map(|(p,n)|(p,n,4))}else{None};
            if let Some((p,n,start))=parsed {pids.insert(p);samples+=1;period+=n;*symbols.entry(w[start..].join(" ")).or_default()+=n;}
        }
        assert!(samples>0);assert_eq!(pids.len(),1);
        pin(&mut raw,r.join(format!("profiles/{key}/perf.data")));pin(&mut raw,loss);
        profiles.insert(key,json!({"samples":samples,"period":period,"pids":pids,"lost":0,"symbols":symbols,"output":v}));
    }}
    let asm=fs::read_to_string(r.join("assembly.local")).unwrap();let mut name=String::new();let mut lines=Vec::new();let mut selected=String::new();let mut functions=Vec::new();
    let mut emit=|name:&str,lines:&Vec<String>| {
        if ["triple_probe::direct","triple_probe::binary","triple_probe::Fixture","msm::stream_g1_impl","msm::g2_prefix::apply"].iter().any(|s|name.contains(s)) {
            selected.push_str(name);selected.push('\n');for l in lines {selected.push_str(l);selected.push('\n');}
            functions.push(json!({"symbol":name,"instruction_sites":lines.len(),"IFMA_sites":lines.iter().filter(|l|l.contains("vpmadd52")).count(),"RSP_operand_lines":lines.iter().filter(|l|l.contains("%rsp")).count(),"frame_subtractions":lines.iter().filter(|l| {let w:Vec<_>=l.split_whitespace().collect();w.len()==3&&w[1]=="sub"&&w[2].starts_with('$')&&w[2].ends_with(",%rsp")}).collect::<Vec<_>>()}));
        }
    };
    for line in asm.lines() {if line.ends_with(">:"){emit(&name,&lines);name=line.into();lines.clear();}else if line.contains(':')&&!line.trim().is_empty(){lines.push(line.into());}}
    emit(&name,&lines);fs::write(r.join("hot.s"),selected).unwrap();
    save(r.join("analysis.json"),&json!({"status":"complete native triple caller experiment","rows":rows,"eligible_small_batch_cases":eligible,"profiles":profiles,"raw_profile_pins":raw,"static_code":{"full_assembly_sha256":hash(r.join("assembly.local")),"functions":functions},"new_tests":2,"latency_processes":496,"preflights":16,"profile_processes":4,"all_outputs_match":true,"timed_allocation_calls":0,"production_changed":false,"adopted":false,"SDK_requests":0,"new_exports":0,"proof_ledger":12524,"goal_achieved":false,"limits":"single active native caller; fixture contract distinct destinations; no production MSM geometry, SDK, proving latency, peakRSS, pressure, TEE, phones, AWS or Aeglos qualification"}));
    let d=Path::new(D);fs::create_dir_all(d.join("artifacts")).unwrap();
    for name in ["crates/zopro-core/src/lib.rs","crates/zopro-core/src/msm.rs","crates/zopro-core/src/msm/triple_probe.rs"] {
        let dest=r.join("candidate-source").join(name);fs::create_dir_all(dest.parent().unwrap()).unwrap();fs::copy(r.join("source").join(name),dest).unwrap();
    }
    let mut index=BTreeMap::new();archive(r,r,&d.join("artifacts"),&mut index);
    save(d.join("artifact-index.json"),&json!(index));
    let sums=index.iter().map(|(p,h)|format!("{h}  {p}\n")).collect::<String>();fs::write(d.join("SHA256SUMS"),sums).unwrap();
    println!("Closed native callers: {} archived hashes; proofledger12524 unchanged",index.len());
}
fn main() {match std::env::args().nth(1).as_deref(){Some("prepare")=>prepare(),Some("finish")=>finish(),_=>panic!("prepare/finish")}}
