use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/unweighted-triple-census174";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const MANIFEST:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-g2-borrowed-adoption/artifacts/production-source-manifest.json";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut b = [0; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    format!("{:x}", h.finalize())
}
fn pin(p: &mut BTreeMap<String, String>, f: impl AsRef<Path>) {
    let f = f.as_ref();
    p.insert(f.display().to_string(), hash(f));
}
use std::process::Command;
const OLD:&str="/home/ubuntu/src/zopro-perf-work/g2-phased-inverse174";
const VERIFY:&str="/home/ubuntu/src/zopro-perf-work/batch-local-steal/gnark/verify";
const X:&str="/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";
const METRICS:[&str;16]=["calls","groups","points","binary_slots","triple_slots","ordinary_triples","identity_triples","equal_X_triples","H_zero_triples","remainder_points","jobs_8plus_ordinary","jobs_all_triples_ordinary","groups_3plus","max_group","max_triples_in_job","jobs_with_guard"];
const HIST:[&str;14]=["0","1","2","3","4-7","8-15","16-31","32-63","64-127","128-255","256-511","512-1023","1024-2047","2048+"];
const CELLS:[&str;4]=["affine_group/G1","affine_group/G2","indexed_tile/G1","indexed_tile/G2"];
fn prepare(){
 assert!(!r("run-plan.json").exists());assert_eq!(read(r("native/result.json"))["status"],"complete");
 for(name,n)in[("core",124),("prover",8)]{assert_eq!(read(r(format!("native/{name}/tests.json")))["passed_tests"].as_array().unwrap().len(),n);}
 let mut pins:BTreeMap<String,String>=serde_json::from_value(read(r("build-pins.json"))).unwrap();for(f,h)in &pins{assert_eq!(hash(f),*h);}
 let receipt=read(r("native/candidate/artifacts.json"));assert_eq!(receipt["binary_sha256"],hash(receipt["binary"].as_str().unwrap()));
 for(f,h)in receipt["input_pins"].as_object().unwrap(){assert_eq!(hash(f),h.as_str().unwrap());pin(&mut pins,f);}
 for(f,h)in read(r("frozen-candidate.json"))["input_pins"].as_object().unwrap(){assert_eq!(hash(f),h.as_str().unwrap());pin(&mut pins,f);}
 assert_eq!(read(r("frozen-candidate.json"))["rustflags"],format!("{} --cfg zopro_reduction_census",read(MANIFEST)["rustflags"].as_str().unwrap()));
 let mut prior:BTreeSet<String>=serde_json::from_value(read(format!("{OLD}/prior-proof-sha256.json"))).unwrap();assert_eq!(prior.len(),15433);
 for(f,h)in read(format!("{OLD}/proof-inventory.json")).as_object().unwrap(){assert_eq!(hash(f),h.as_str().unwrap());if f.ends_with(".proof"){assert!(prior.insert(h.as_str().unwrap().into()));}}
 assert_eq!(prior.len(),15541);save(r("prior-proof-sha256.json"),&json!(prior));
 let all=read("/home/ubuntu/src/zopro-perf-work/sdk-control-copies/cases.json");let cases:Vec<_>=["1x2","2x2","16x2","merge-54x1"].into_iter().map(|s|all.as_array().unwrap().iter().find(|c|c["shape"]==s).unwrap().clone()).collect();save(r("cases.json"),&json!(cases));
 let base=read("/home/ubuntu/src/zopro-perf-work/g2-borrowed-production/run-plan.json")["steps"][0]["environment"].clone();let mut jobs=vec![];let mut steps=vec![];
 for b in 0..3 {let mut order=vec![0,1,2,3];order.rotate_left(b%4);if b%2==1{order.reverse();}for i in order{let c=&cases[i];let name=format!("b{b}-{}",c["name"].as_str().unwrap());let directory=r("proofs").join(&name);let mut env=base.clone();env["RAYON_NUM_THREADS"]=json!("32");env["ZOPRO_CPU_DEDUP"]=json!("1");env["ZOPRO_CPU_ZERO_AWARE_GROUPS"]=json!("1");steps.push(json!({"kind":"command","name":name,"argv":["taskset","-c",c["cpu_affinity"],receipt["binary"],c["image"],c["request"],"1",directory],"environment":env,"timeout_seconds":900}));jobs.push(json!({"name":name,"case":c,"block":b,"directory":directory}));}}
 assert_eq!(jobs.len(),12);
 for j in &jobs{let c=&j["case"];let dir=Path::new(j["directory"].as_str().unwrap());steps.push(json!({"kind":"command","name":format!("verify-{}",j["name"].as_str().unwrap()),"argv":[VERIFY,c["image"],c["image_sha256"],dir.join("0.proof"),dir.join("0.public"),c["public_sha256"]],"timeout_seconds":120}));}
 assert_eq!(steps.len(),24);save(r("jobs.json"),&json!(jobs));save(r("metrics.json"),&json!({"metrics":METRICS,"histogram_bins":HIST,"cells":CELLS,"max_metric_indices":[13,14],"reserved_zero_indices":[30,31]}));
 for c in &cases {assert_eq!(hash(c["image"].as_str().unwrap()),c["image_sha256"].as_str().unwrap());pin(&mut pins,c["image"].as_str().unwrap());pin(&mut pins,c["request"].as_str().unwrap());}
 for e in fs::read_dir("/home/ubuntu/src/zopro-perf-work/sdk-control-copies/lagrange-cache").unwrap(){pin(&mut pins,e.unwrap().path());}
 for f in [VERIFY,X,MANIFEST]{pin(&mut pins,f);}
 for f in ["capture.rs","capture-helper","close.rs","close-helper","research-sha256.txt","metrics.json","cases.json","jobs.json","prior-proof-sha256.json","native/result.json","native/candidate/artifacts.json","frozen-candidate.json"]{pin(&mut pins,r(f));}
 save(r("pins.json"),&json!(pins));save(r("run-plan.json"),&json!({"version":1,"source_pins":r("pins.json"),"steps":steps}));
 let ps=Command::new("ps").args(["-eo","pid,ppid,comm,pcpu,pmem,etimes","--sort=-pcpu"]).output().unwrap();assert!(ps.status.success());fs::write(r("host-before.txt"),ps.stdout).unwrap();
 println!("Prepared12captures then12Gnark;48SDK/12exports ledger15541->15553;no timing claim");
}
fn finish(){
 assert_eq!(read(r("runs/result.json"))["status"],"complete");for(f,h)in read(r("pins.json")).as_object().unwrap(){assert_eq!(hash(f),h.as_str().unwrap());}
 let mut prior:BTreeSet<String>=serde_json::from_value(read(r("prior-proof-sha256.json"))).unwrap();assert_eq!(prior.len(),15541);
 let mut inventory=BTreeMap::new();let mut observations=BTreeMap::<String,Vec<Value>>::new();
 for j in read(r("jobs.json")).as_array().unwrap(){let name=j["name"].as_str().unwrap();let log=fs::read_to_string(r(format!("runs/{name}/stdout.log"))).unwrap();let frames:Vec<_>=log.lines().filter_map(|l|l.strip_prefix("CENSUS ")).collect();assert_eq!(frames.len(),1);let frame:Value=serde_json::from_str(frames[0]).unwrap();assert_eq!(frame["priming"],3);assert_eq!(frame["observed"],1);assert_eq!(frame["workers"],32);assert_eq!(frame["timing_claim"],false);
  let cells=frame["cells"].as_array().unwrap();assert_eq!(cells.len(),4);let mut decoded=BTreeMap::new();
  for(k,row)in cells.iter().enumerate(){let a:Vec<u64>=serde_json::from_value(row.clone()).unwrap();assert_eq!(a.len(),32);assert_eq!(a[4],a[5]+a[6]+a[7]+a[8]);assert_eq!(a[2],3*a[4]+a[9]);assert_eq!(a[1],a[16..30].iter().sum::<u64>());assert_eq!(a[30],0);assert_eq!(a[31],0);assert!(a[10]<=a[0]&&a[11]<=a[0]&&a[12]<=a[1]&&a[15]<=a[0]);let mut x=BTreeMap::new();for(i,m)in METRICS.iter().enumerate(){x.insert(m.to_string(),json!(a[i]));}x.insert("histogram".into(),json!(HIST.iter().enumerate().map(|(i,s)|(*s,a[16+i])).collect::<BTreeMap<_,_>>()));decoded.insert(CELLS[k],json!(x));}
  let verified=fs::read_to_string(r(format!("runs/verify-{name}/stdout.log"))).unwrap();assert!(verified.contains("VERIFIED proof_sha256="));assert!(verified.contains(&format!("image_sha256={}",j["case"]["image_sha256"].as_str().unwrap())));
  for ext in ["proof","public"]{let p=Path::new(j["directory"].as_str().unwrap()).join(format!("0.{ext}"));let h=hash(&p);if ext=="proof"{assert_eq!(fs::metadata(&p).unwrap().len(),164);assert!(prior.insert(h.clone()));}else{assert_eq!(h,j["case"]["public_sha256"].as_str().unwrap());}assert!(verified.contains(&format!("{ext}_sha256={h}")));inventory.insert(p.display().to_string(),h);}
  observations.entry(j["case"]["name"].as_str().unwrap().into()).or_default().push(json!({"name":name,"block":j["block"],"raw":frame,"contexts":decoded}));
 }
 assert_eq!(prior.len(),15553);assert_eq!(inventory.len(),24);save(r("proof-inventory.json"),&json!(inventory));let mut cases=BTreeMap::new();let mut consistency=true;
 for(name,mut rows)in observations{rows.sort_by_key(|r|r["block"].as_u64().unwrap());assert_eq!(rows.len(),3);let same=rows[1]["raw"]["cells"]==rows[0]["raw"]["cells"]&&rows[2]["raw"]["cells"]==rows[0]["raw"]["cells"];consistency&=same;cases.insert(name,json!({"three_maps_identical":same,"observations":rows}));}
 let a=json!({"status":"completed first-layer unweighted reduction census; no optimization/adoption","cases":cases,"consistency_condition_passed":consistency,"all_conservation_checks_passed":true,"SDK_requests":48,"new_independent_exports":12,"independent_proof_count":15553,"native_core_tests":124,"native_prover_tests":8,"field_source_unchanged_qualification_inherited":true,"production_changed":false,"adopted":false,"timing_CPU_RSS_minRAM_claim":false,"first_layer_only":true,"weighted_and_streaming_excluded":true,"inherited_accepted_M5":read(MANIFEST)["m5"],"goal_status":"active","goal_achieved":false,"accepted_original8_CPU10x_rows":0});save(r("analysis.json"),&a);
 println!("Countsconsistent{consistency};48SDK/12freshGnark/ledger15553;no speed claim");
 for(name,case)in a["cases"].as_object().unwrap(){println!("{name} identical{}",case["three_maps_identical"]);for(k,v)in case["observations"][0]["contexts"].as_object().unwrap(){println!("{k} points{} triples{} ordinary{} vectorjobs{} maxtriples{}",v["points"],v["triple_slots"],v["ordinary_triples"],v["jobs_8plus_ordinary"],v["max_triples_in_job"]);}}
}
fn main(){match std::env::args().nth(1).as_deref(){Some("prepare")=>prepare(),Some("finish")=>finish(),_=>panic!("prepare|finish")}}
