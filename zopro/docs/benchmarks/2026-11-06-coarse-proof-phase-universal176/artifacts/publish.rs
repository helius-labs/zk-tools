use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, path::{Path, PathBuf}, process::{Command, Stdio}};
const ROOT: &str = "/home/ubuntu/src/zopro-perf-work/coarse-proof-phase-universal176";
const OUT: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-coarse-proof-phase-universal176";
fn root(p: impl AsRef<Path>) -> PathBuf { Path::new(ROOT).join(p) }
fn read(p: impl AsRef<Path>) -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() }
fn hash(b: &[u8]) -> String { format!("{:x}", Sha256::digest(b)) }
fn walk(p: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in fs::read_dir(p).unwrap() { let p = entry.unwrap().path(); if p.is_dir() { files.extend(walk(&p)); } else { files.push(p); } }
    files.sort(); files
}
fn gzip(p: &Path, bytes: &[u8]) {
    let temp = p.with_extension("tmp"); fs::write(&temp, bytes).unwrap();
    assert!(Command::new("gzip").args(["-n", "-c"]).arg(&temp).stdout(Stdio::from(fs::File::create(p).unwrap())).status().unwrap().success());
    let check = Command::new("gzip").args(["-d", "-c"]).arg(p).output().unwrap();
    assert!(check.status.success()); assert_eq!(check.stdout, bytes); fs::remove_file(temp).unwrap();
}
fn hex(bytes: &[u8]) -> Value {
    let text = bytes.iter().map(|b| format!("{b:02x}")).collect::<String>();
    let back = text.as_bytes().chunks_exact(2).map(|b| u8::from_str_radix(std::str::from_utf8(b).unwrap(),16).unwrap()).collect::<Vec<_>>();
    assert_eq!(back, bytes); json!({"hex": text, "length": bytes.len(), "sha256": hash(bytes)})
}
fn add(receipts: &mut BTreeMap<String, Value>, p: &Path) {
    let bytes = fs::read(p).unwrap(); let text = String::from_utf8(bytes.clone()).unwrap();
    assert_eq!(text.as_bytes(), bytes); receipts.insert(p.display().to_string(), json!({"utf8": text, "length": bytes.len(), "sha256": hash(&bytes)}));
}
fn main() {
    let review = read(root("review.json")); assert_eq!(review["passed"], true); assert_eq!(review["diagnostic_qualified"], false);
    let out = Path::new(OUT); assert!(!out.exists()); fs::create_dir_all(out.join("artifacts")).unwrap();
    let mut source = BTreeMap::new();
    for p in walk(&root("source")) { source.insert(p.strip_prefix(root("source")).unwrap().display().to_string(), hex(&fs::read(p).unwrap())); }
    assert_eq!(source.len(),177); gzip(&out.join("artifacts/source-snapshot.hex.json.gz"), &serde_json::to_vec(&source).unwrap());
    let mut receipts = BTreeMap::new();
    let allowed = |p: &Path| matches!(p.extension().and_then(|s|s.to_str()),Some("rs"|"json"|"jsonl"|"log"|"md"|"txt"|"csv"));
    for entry in fs::read_dir(ROOT).unwrap() { let p = entry.unwrap().path(); if p.is_file() && allowed(&p) { add(&mut receipts,&p); } }
    for dir in ["native","cubic","links","runs","support","initial-bootstrap-indent","initial-native-basis-argument","stat"] {
        for p in walk(&root(dir)) { if allowed(&p) { add(&mut receipts,&p); } }
    }
    let jobs = read(root("jobs.json"));
    for job in jobs.as_array().unwrap().iter().filter(|j|j["reused_from"].is_string()) {
        for key in ["reused_from","reused_verify_from"] { for p in walk(Path::new(job[key].as_str().unwrap())) { assert!(allowed(&p)); add(&mut receipts,&p); } }
    }
    let receipt_count=receipts.len(); gzip(&out.join("artifacts/receipts.json.gz"),&serde_json::to_vec(&receipts).unwrap());
    let inventory=read(root("proof-inventory.json")); let mut proofs=BTreeMap::new();
    for entry in inventory.as_array().unwrap() {
        let name=entry["name"].as_str().unwrap(); let job=jobs.as_array().unwrap().iter().find(|j|j["name"]==name).unwrap();
        for file in ["0.proof","0.public"] { let bytes=fs::read(Path::new(job["directory"].as_str().unwrap()).join(file)).unwrap(); let key=if file=="0.proof" {"proof_sha256"}else{"public_sha256"}; assert_eq!(hash(&bytes),entry[key].as_str().unwrap()); proofs.insert(format!("{name}/{file}"),hex(&bytes)); }
    }
    assert_eq!(proofs.len(),264); gzip(&out.join("artifacts/public-proofs.hex.json.gz"),&serde_json::to_vec(&proofs).unwrap());
    let mut raw=BTreeMap::new();
    for p in walk(&root("perf")) { assert_eq!(p.extension().unwrap(),"data"); let bytes=fs::read(&p).unwrap(); let name=p.file_name().unwrap().to_str().unwrap(); gzip(&out.join("artifacts").join(format!("{name}.gz")),&bytes); raw.insert(name.to_string(),json!({"length":bytes.len(),"sha256":hash(&bytes)})); }
    assert_eq!(raw.len(),6); fs::write(out.join("artifacts/raw-profile-inventory.json"),serde_json::to_vec_pretty(&raw).unwrap()).unwrap();
    for name in ["analysis.json","review.json","source-audit.json","pre-native-review.json","proof-inventory.json","proof-sha256-after.json","rows.json","graphs.json","profile-symbols.json","identities.json","design.md","coarse_phase.rs","publish.rs"] { fs::copy(root(name),out.join("artifacts").join(name)).unwrap(); }
    fs::copy(root("report-draft.md"),out.join("README.md")).unwrap();
    let mut sums=String::new(); let files=walk(out);
    for p in &files { writeln!(sums,"{}  {}",hash(&fs::read(p).unwrap()),p.strip_prefix(out).unwrap().display()).unwrap(); }
    fs::write(out.join("SHA256SUMS"),sums).unwrap();
    println!("Published {} files: 177 source members, {receipt_count} UTF-8 receipts, 264 proof/public members, six losslessly compressed profiles. All archives byte-checked.",files.len()+1);
}
