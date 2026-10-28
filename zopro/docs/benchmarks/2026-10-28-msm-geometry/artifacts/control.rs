use serde_json::{json,Value};use sha2::{Digest,Sha256};
use std::{collections::{BTreeMap,BTreeSet},fs,io::Read,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/msm-geometry";const Z:&str="/home/ubuntu/src/zk-tools/zopro";
const X:&str="/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask";
const D:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-msm-geometry";
const IMAGE:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-25-worker-groups-request-entry/qualification/warm-default-w32-single-request/input/transfer_confidential_1_2.image";
const IH:&str="16a3e2a686372d531879d4ff99e63eab0ca374d686f05a9d7343d2bf534dd041";
const PH:&str="3f0d846c3905d11595d77dd988df829ddc51325eda2b93e4db8f0e17a5928e5b";
const VERIFIER:&str="/home/ubuntu/src/zopro-perf-work/batch-local-steal/gnark/verify";
fn read(p:impl AsRef<Path>)->Value {serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:impl AsRef<Path>,v:&Value) {fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap();}
fn hash(p:impl AsRef<Path>)->String {let mut f=fs::File::open(p).unwrap();let mut h=Sha256::new();let mut b=[0;65536];loop{let n=f.read(&mut b).unwrap();if n==0{break;}h.update(&b[..n]);}format!("{:x}",h.finalize())}
fn pin(m:&mut BTreeMap<String,String>,p:impl AsRef<Path>) {let p=p.as_ref();m.insert(p.display().to_string(),hash(p));}
fn check_source()->BTreeMap<String,String> {
    let r=Path::new(R);let mut pins=BTreeMap::new();let edits=read(r.join("inverse-edits-extended.json"));
    let m=read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json"));
    for (file,h) in m["source_files_sha256"].as_object().unwrap() {
        let p=Path::new(Z).join(file);assert_eq!(hash(&p),h.as_str().unwrap());pin(&mut pins,&p);let local=r.join("source").join(file);pin(&mut pins,&local);
        if let Some(list)=edits.get(file) {
            let mut recovered=fs::read_to_string(&local).unwrap();for edit in list.as_array().unwrap().iter().rev() {let a=edit[0].as_str().unwrap();let b=edit[1].as_str().unwrap();assert_eq!(recovered.matches(b).count(),1);recovered=recovered.replacen(b,a,1);}
            assert_eq!(recovered,fs::read_to_string(&p).unwrap());
        } else {assert_eq!(hash(&local),h.as_str().unwrap());}
    }
    assert_eq!(edits.as_object().unwrap().len(),6);
    let p=r.join("source/crates/zopro-core/src/msm_geometry.rs");assert_eq!(hash(&p),hash(r.join("observer.rs")));pin(&mut pins,p);
    assert!(fs::read_to_string(r.join("observer-tests.log")).unwrap().contains("test result: ok. 3 passed; 0 failed"));
    assert_eq!(read(r.join("source-audit-extended.json"))["observer_module_sha256_after_passed_tests"],hash(r.join("observer.rs")));
    for name in ["design.md","observer.rs","driver.rs","setup.rs","setup","extend-feeds.rs","extend-feeds","control.rs","control","native-flags.txt","source-audit.json","source-audit-extended.json","inverse-edits.json","inverse-edits-extended.json","observer-tests.log"] {pin(&mut pins,r.join(name));}
    pin(&mut pins,X);pins
}
fn build_plan() {
    let r=Path::new(R);assert!(!r.join("build-plan.json").exists());let pins=check_source();save(r.join("build-pins.json"),&json!(pins));
    let flags=fs::read_to_string(r.join("native-flags.txt")).unwrap();
    save(r.join("build-plan.json"),&json!({"version":1,"source_pins":r.join("build-pins.json"),"environment":{"RUSTFLAGS":flags.trim(),"CARGO_TARGET_DIR":r.join("target"),"CARGO_BUILD_JOBS":"16"},"steps":[
        {"kind":"command","name":"libraries","argv":["cargo","build","--manifest-path",r.join("source/Cargo.toml"),"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"timeout_seconds":1200},
        {"kind":"link-sdk","name":"observer","artifacts":r.join("native/libraries/stdout.log"),"driver":r.join("driver.rs"),"timeout_seconds":600}
    ]}));
}
fn prior()->BTreeSet<String> {
    let mut seen:BTreeSet<String>=serde_json::from_value(read("/home/ubuntu/src/zopro-perf-work/off-ibs/prior-proof-sha256.json")).unwrap();assert_eq!(seen.len(),12518);
    for (p,h) in read("/home/ubuntu/src/zopro-perf-work/off-ibs/proof-inventory.json").as_object().unwrap() {if p.ends_with(".proof") {assert_eq!(hash(p),h.as_str().unwrap());assert!(seen.insert(h.as_str().unwrap().into()));}}
    assert_eq!(seen.len(),12524);seen
}
fn prepare() {
    let r=Path::new(R);assert!(!r.join("run-plan.json").exists());assert_eq!(read(r.join("native/result.json"))["status"],"complete");
    let mut pins=check_source();for (p,h) in read(r.join("native/observer/artifacts.json"))["input_pins"].as_object().unwrap() {assert_eq!(hash(p),h.as_str().unwrap());pin(&mut pins,p);}
    let bin=r.join("native/observer-sdk");let receipt=read(r.join("native/observer/artifacts.json"));assert_eq!(hash(&bin),receipt["binary_sha256"].as_str().unwrap());pin(&mut pins,&bin);
    assert_eq!(hash(IMAGE),IH);let request=format!("{Z}/tools/zolana/fixtures/transfer-1x2.json");for p in [IMAGE,VERIFIER,&request] {pin(&mut pins,p);}
    let seen=prior();save(r.join("prior-proof-sha256.json"),&json!(seen));pin(&mut pins,r.join("prior-proof-sha256.json"));
    let m=read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json"));let mut env=m["environment"].clone();
    env["ZOPRO_BASIS_KEY_DIR"]=json!("/home/ubuntu/src/zopro-perf-work/batch-local-steal/inputs");env["ZOPRO_CPU_LAGRANGE_DIR"]=json!("/home/ubuntu/src/zopro-perf-work/batch-local-steal/lagrange-cache");
    let mut steps=Vec::new();for i in 0..3 {steps.push(json!({"kind":"command","name":format!("observe-{i}"),"argv":["taskset","-c","0-31",bin,IMAGE,request,r.join(format!("proofs/{i}"))],"timeout_seconds":180}));}
    for i in 0..3 {steps.push(json!({"kind":"command","name":format!("verify-{i}"),"argv":[VERIFIER,IMAGE,IH,r.join(format!("proofs/{i}/0.proof")),r.join(format!("proofs/{i}/0.public")),PH],"timeout_seconds":120}));}
    save(r.join("pins.json"),&json!(pins));save(r.join("run-plan.json"),&json!({"version":1,"source_pins":r.join("pins.json"),"environment":env,"steps":steps}));
    println!("Prepared3observedSDKprocesses,12SDKrequests,3exports;ledger12524->12527 ifqualified");
}
fn parts(key:&str)->BTreeMap<&str,u64> {key.split('/').filter_map(|s|s.split_once('=').and_then(|(k,v)|v.parse::<u64>().ok().map(|v|(k,v)))).collect()}
fn archive(root:&Path,dir:&Path,dest:&Path,index:&mut BTreeMap<String,String>) {
    for e in fs::read_dir(dir).unwrap() {let e=e.unwrap();let p=e.path();let name=e.file_name().to_string_lossy().to_string();if e.file_type().unwrap().is_dir() {if ["source","target"].contains(&name.as_str()) {continue;}archive(root,&p,dest,index);}else if ["rs","md","json","log","txt","proof","public"].contains(&p.extension().and_then(|s|s.to_str()).unwrap_or(""))&&! ["workflow.log","build-workflow.log","close.log","archive-verification.log"].contains(&name.as_str()) {let rel=p.strip_prefix(root).unwrap();let out=dest.join(rel);fs::create_dir_all(out.parent().unwrap()).unwrap();fs::copy(&p,&out).unwrap();index.insert(format!("artifacts/{}",rel.display()),hash(out));}}
}
fn finish() {
    let r=Path::new(R);assert_eq!(read(r.join("runs/result.json"))["status"],"complete");for (p,h) in read(r.join("pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
    let mut seen:BTreeSet<String>=serde_json::from_value(read(r.join("prior-proof-sha256.json"))).unwrap();assert_eq!(seen.len(),12524);
    let mut inventory=BTreeMap::new();let mut processes=Vec::new();
    for i in 0..3 {
        let text=fs::read_to_string(r.join(format!("runs/observe-{i}/stdout.log"))).unwrap();
        let counts: BTreeMap<String,u64>=serde_json::from_str(text.lines().find_map(|s|s.strip_prefix("GEOMETRY ")).unwrap()).unwrap();
        let summary:Value=serde_json::from_str(text.lines().find_map(|s|s.strip_prefix("SUMMARY ")).unwrap()).unwrap();assert_eq!(summary["SDK_requests"],4);assert_eq!(summary["internally_verified"],true);
        let mut streams=BTreeMap::<String,BTreeMap<String,u64>>::new();let mut trees=BTreeMap::<String,BTreeMap<String,u64>>::new();
        for (key,n) in &counts {
            if key.starts_with("stream/") {
                let words:Vec<_>=key.split('/').collect();let prefix=words[..5].join("/");let metric=words[5..].join("/");
                let s=streams.entry(prefix).or_default();
                if metric.starts_with("flush/") {
                    let p=parts(&metric);assert_eq!(p["queued"],p["entries"]+p["free"]);assert_eq!(p["entries"],p["matched"]+p["extra"]);assert!(p["matched"]<=p["len"]);
                    *s.entry("flush_histogram_calls".into()).or_default()+=n;
                    *s.entry("flush_histogram_entries".into()).or_default()+=n*p["len"];
                    *s.entry("flush_histogram_matches".into()).or_default()+=n*p["matched"];
                    if p["matched"]>0 {*s.entry("flushes_with_matched_queue".into()).or_default()+=n;}
                    let band=if p["matched"]==0 {"0"} else if p["matched"]<8 {"1-7"} else if p["matched"]<=16 {"8-16"} else if p["matched"]<=64 {"17-64"} else {"65+"};
                    *s.entry(format!("matched_destinations_band/{band}")).or_default()+=n;
                    if p["len"]<64 {*s.entry("flush_length_below64".into()).or_default()+=n;}
                } else if metric.starts_with("projective_flush/") {
                    let p=parts(&metric);assert_eq!(p["queued"],p["entries"]+p["free"]);
                    *s.entry("projective_flush_histogram_entries".into()).or_default()+=n*p["queued"];
                } else {s.insert(metric,*n);}
            } else if key.starts_with("tree/") {
                let kind=key.split('/').nth(1).unwrap().to_owned();let p=parts(key);let t=trees.entry(kind).or_default();
                *t.entry("calls".into()).or_default()+=n;*t.entry("ordinary_pairs".into()).or_default()+=n*p["ordinary_pairs"];*t.entry("triple_slots_over_snapshots".into()).or_default()+=n*p["triple_slots"];
                let m=p["triple_slots"];let band=if m==0 {"0"}else if m<8 {"1-7"}else if m<=16 {"8-16"}else if m<=64 {"17-64"}else {"65+"};
                *t.entry(format!("triple_slots_band/{band}")).or_default()+=n;
            } else {panic!("unknown metric {key}");}
        }
        for s in streams.values() {
            assert_eq!(s.get("flush_histogram_calls").copied().unwrap_or(0),s.get("execute_calls").copied().unwrap_or(0));
            assert_eq!(s.get("flush_histogram_entries").copied().unwrap_or(0),s.get("executed_entries").copied().unwrap_or(0));
            assert_eq!(s.get("flush_histogram_matches").copied().unwrap_or(0),s.get("matched_destinations_over_flushes").copied().unwrap_or(0));
            assert_eq!(s.get("projective_flush_histogram_entries").copied().unwrap_or(0),s.get("projective_queue_entries").copied().unwrap_or(0));
        }
        for ext in ["proof","public"] {let p=r.join(format!("proofs/{i}/0.{ext}"));let h=hash(&p);if ext=="proof" {assert_eq!(fs::metadata(&p).unwrap().len(),164);assert!(seen.insert(h.clone()));}else{assert_eq!(h,PH);}inventory.insert(p.display().to_string(),h);}
        processes.push(json!({"process":i,"summary":summary,"streams":streams,"trees":trees,"counts":counts}));
    }
    assert_eq!(seen.len(),12527);save(r.join("proof-inventory.json"),&json!(inventory));
    save(r.join("analysis.json"),&json!({"status":"complete real 1x2 MSM geometry diagnostic","processes":processes,"SDK_requests":12,"observed_proofs":3,"fresh_independent_exports":3,"prior_proof_count":12524,"next_prior_proof_count":12527,"latency_measured":false,"production_changed":false,"adopted":false,"goal_achieved":false,"scope":"1x2 only; diagnostic counters; queue snapshots may revisit entries; ideal tree slots are not validated triple pairs or avoided inversions"}));
    let dest=Path::new(D);fs::create_dir_all(dest.join("artifacts")).unwrap();
    let audit=read(r.join("source-audit-extended.json"));for f in audit["changed_original_files"].as_array().unwrap().iter().map(|v|v.as_str().unwrap()).chain(["crates/zopro-core/src/msm_geometry.rs"]) {let out=r.join("candidate-source").join(f);fs::create_dir_all(out.parent().unwrap()).unwrap();fs::copy(r.join("source").join(f),out).unwrap();}
    let mut index=BTreeMap::new();archive(r,r,&dest.join("artifacts"),&mut index);save(dest.join("artifact-index.json"),&json!(index));fs::write(dest.join("SHA256SUMS"),index.iter().map(|(p,h)|format!("{h}  {p}\n")).collect::<String>()).unwrap();
    println!("Closed diagnostic:12SDK3newGnarkexports;ledger12527;{}archivehashes",index.len());
}
fn main() {match std::env::args().nth(1).as_deref(){Some("build-plan")=>build_plan(),Some("prepare")=>prepare(),Some("finish")=>finish(),_=>panic!("build-plan/prepare/finish")}}
