use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs,io::Read,path::{Path,PathBuf}};
const R:&str="/home/ubuntu/src/zopro-perf-work/unweighted-triple-caller174";
const D:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-unweighted-triple-caller174";
fn read(p:impl AsRef<Path>)->Value {serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:impl AsRef<Path>,v:&Value) {fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String {
    let mut f=fs::File::open(p).unwrap();let mut h=Sha256::new();let mut b=[0;65536];
    loop {let n=f.read(&mut b).unwrap();if n==0 {break;}h.update(&b[..n]);}format!("{:x}",h.finalize())
}
fn main() {
    let root=Path::new(R);let dest=Path::new(D);
    let analysis=read(root.join("analysis.json"));assert_eq!(analysis["main_component_gate"],false);
    assert_eq!(read(root.join("runs/result.json"))["status"],"complete");
    for (p,h) in read(root.join("pins.json")).as_object().unwrap() {assert_eq!(hash(p),h.as_str().unwrap(),"{p}");}
    let mut index:BTreeMap<String,String>=serde_json::from_value(read(dest.join("artifact-index.json"))).unwrap();
    let mut omitted=BTreeMap::new();
    for case in ["tile384","tile2048"] {for variant in ["binary","ternary"] {
        let key=format!("artifacts/runs/loss-{case}-{variant}/stdout.log");
        let old=index.remove(&key).unwrap();let local=root.join(key.strip_prefix("artifacts/").unwrap());
        assert_eq!(hash(&local),old);assert!(!dest.join(&key).exists());
        omitted.insert(local.display().to_string(),old);
    }}
    for (p,h) in &index {assert_eq!(&hash(dest.join(p)),h,"{p}");}
    assert_eq!(index.keys().filter(|p|p.starts_with("artifacts/source/")).count(),175);
    let whitespace=fs::read_to_string(root.join("staged-whitespace.log")).unwrap();
    let diagnostics:Vec<_>=whitespace.lines().filter(|l|l.starts_with("zopro/")).collect();
    assert_eq!(diagnostics.len(),11);
    assert!(diagnostics.iter().all(|l|l.contains("/native/core/stdout.log:")||l.contains("/native/oracle/stdout.log:")||l.contains("/runs/decode-")||l.contains("/source/crates/zopro-metal/Cargo.toml:")));
    let publication=json!({"status":"complete; not adopted","accepted_source_files":174,"private_source_files":175,
        "production_unchanged":true,"all_capture_closing_pins_match":true,"native_core_tests":124,"independent_arkworks_tests":1,
        "native_steps":5,"capture_steps":464,"latency_processes":434,"preflight_processes":14,"profile_processes":4,
        "profile_samples":3157,"lost_records":0,"main_component_gate":false,"SDK_requests":0,"new_proof_exports":0,
        "independent_proof_count":15553,"goal_achieved":false,"goal_progress":"completed complete-caller experiment; closes realistic-size ternary candidate",
        "captured_whitespace_diagnostics":11,"authored_whitespace_diagnostics":0,"captured_bytes_preserved":true,
        "archive_omissions_with_local_sha256":omitted,"omission_reason":"perf hexadecimal dumps retained locally; decoded profiles, analysis and zero-loss checks archived",
        "archival_policy_changed_after_capture_only":true,"raw_perf_local_sha256":analysis["raw_profile_sha256"],
        "accepted_primary_ms":8.366931,"accepted_primary_peak_RSS_max_MiB":135.0,"all_eight_CPU_Metal_table":"inherited unchanged in README",
        "no_owned_jobs_or_cloud_resources":true,"no_unchanged_reroll":true});
    save(dest.join("publication.json"),&publication);
    fs::copy(root.join("publication.rs"),dest.join("artifacts/publication.rs")).unwrap();
    for p in ["publication.json","README.md","artifacts/publication.rs"] {index.insert(p.into(),hash(dest.join(p)));}
    save(dest.join("artifact-index.json"),&json!(index));
    let mut sums=index.clone();sums.insert("artifact-index.json".into(),hash(dest.join("artifact-index.json")));
    fs::write(dest.join("SHA256SUMS"),sums.iter().map(|(p,h)|format!("{h}  {p}\n")).collect::<String>()).unwrap();
    for (p,h) in &sums {assert_eq!(&hash(dest.join(p)),h);}
    println!("Verified{}strict hashes;175complete source files;11captured/0authored whitespace findings;464capture steps;no adoption",sums.len());
    let _:PathBuf=root.into();
}
