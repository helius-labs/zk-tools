use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, path::{Path, PathBuf}, process::{Command, Stdio}};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-contained-index176";
const P: &str = "/home/ubuntu/src/zopro-perf-work/g2-bucket-preserving-pack176";
const O: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-g2-contained-index176";
fn r(p: impl AsRef<Path>) -> PathBuf { Path::new(R).join(p) }
fn hash(b: &[u8]) -> String { format!("{:x}", Sha256::digest(b)) }
fn read(p: impl AsRef<Path>) -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() }
fn walk(p: &Path) -> Vec<PathBuf> {
    let mut v = vec![];
    for e in fs::read_dir(p).unwrap() { let p=e.unwrap().path(); if p.is_dir(){v.extend(walk(&p))}else if p.is_file(){v.push(p)} }
    v.sort(); v
}
fn allowed(p: &Path) -> bool {
    matches!(p.extension().and_then(|s|s.to_str()), Some("rs"|"md"|"json"|"log"|"toml"|"lock"|"csv"))
}
fn gzip(p: &Path, b: &[u8]) {
    let tmp=p.with_extension("temporary"); fs::write(&tmp,b).unwrap();
    assert!(Command::new("gzip").args(["-n","-c"]).arg(&tmp).stdout(Stdio::from(fs::File::create(p).unwrap())).status().unwrap().success());
    let out=Command::new("gzip").args(["-d","-c"]).arg(p).output().unwrap();
    assert!(out.status.success()); assert_eq!(out.stdout,b); fs::remove_file(tmp).unwrap();
}
fn archive_utf8(out: &Path, paths: Vec<PathBuf>) -> usize {
    let mut records=BTreeMap::new();
    for p in paths { let b=fs::read(&p).unwrap(); let text=String::from_utf8(b.clone()).unwrap(); assert_eq!(text.as_bytes(),b); records.insert(p.display().to_string(),json!({"utf8":text,"length":b.len(),"sha256":hash(&b)})); }
    let count=records.len();
    let encoded=serde_json::to_vec(&records).unwrap();
    let decoded: Value=serde_json::from_slice(&encoded).unwrap();
    for (_,v) in decoded.as_object().unwrap(){let b=v["utf8"].as_str().unwrap().as_bytes();assert_eq!(b.len(),v["length"].as_u64().unwrap() as usize);assert_eq!(hash(b),v["sha256"]);}
    gzip(out,&encoded); count
}
fn main() {
    let review=read(r("review.json")); assert_eq!(review["passed"],true); assert_eq!(review["component_gate_passed"],false);
    // Independently bind the actual driver and inverse edit sequence.
    let d=read(r("driver-audit.json")); let mut recovered=fs::read_to_string(r("driver/src/main.rs")).unwrap();
    assert_eq!(recovered,fs::read_to_string(r("driver.rs")).unwrap());
    for edit in d["edits"].as_array().unwrap().iter().rev(){let from=edit[1].as_str().unwrap();assert_eq!(recovered.matches(from).count(),1);recovered=recovered.replacen(from,edit[0].as_str().unwrap(),1);}
    assert_eq!(recovered,fs::read_to_string(d["original_driver"].as_str().unwrap()).unwrap());
    let out=Path::new(O); assert!(!out.exists()); fs::create_dir_all(out.join("artifacts")).unwrap();
    let mut source=BTreeMap::new();
    for p in walk(&r("source")){
        let b=fs::read(&p).unwrap(); let hex=b.iter().map(|v|format!("{v:02x}")).collect::<String>();
        let restored=hex.as_bytes().chunks_exact(2).map(|p|u8::from_str_radix(std::str::from_utf8(p).unwrap(),16).unwrap()).collect::<Vec<_>>(); assert_eq!(restored,b);
        source.insert(p.strip_prefix(r("source")).unwrap().display().to_string(),json!({"hex":hex,"length":b.len(),"sha256":hash(&b)}));
    }
    assert_eq!(source.len(),178); gzip(&out.join("artifacts/source178.json.gz"),&serde_json::to_vec(&source).unwrap());
    let mut files=vec![];
    for n in ["setup.rs","native-tests.rs","design.md","protocol.md","source-audit.json","driver-audit.json","environment.json","fixtures.json","cases.json","jobs.json","identities.json","build-plan.json","build-pins.json","capture-plan.json","capture-pins.json","campaign.rs","analyze.rs","analysis.json","callers.json","symbols.json","review.rs","review.json","report-draft.md","publish.rs"] { files.push(r(n)); }
    for d in ["driver","native","runs","stat"] {files.extend(walk(&r(d)).into_iter().filter(|p|allowed(p)));}
    for j in read(r("jobs.json")).as_array().unwrap(){if let Some(p)=j["reused_from"].as_str(){files.extend(walk(Path::new(p)).into_iter().filter(|p|allowed(p)));}}
    let receipts=archive_utf8(&out.join("artifacts/receipts-and-helpers.json.gz"),files);
    let preliminary=archive_utf8(&out.join("artifacts/preliminary-planner-census.json.gz"),walk(Path::new(P)).into_iter().filter(|p|allowed(p)).collect());
    for shape in ["1x2","2x2"] {for arm in ["FROZEN","OFF","ON"] {let name=format!("cycles-{shape}-B-w32-{arm}.data");gzip(&out.join("artifacts").join(format!("{name}.gz")),&fs::read(r("perf").join(name)).unwrap());}}
    for f in ["analysis.json","review.json","source-audit.json","driver-audit.json","identities.json","design.md","protocol.md","native-tests.rs","setup.rs","campaign.rs","analyze.rs","review.rs","publish.rs"]{fs::copy(r(f),out.join("artifacts").join(f)).unwrap();}
    fs::copy(Path::new(P).join("census.json"),out.join("artifacts/preliminary-census.json")).unwrap();
    fs::copy(r("report-draft.md"),out.join("README.md")).unwrap();
    let record=json!({"source_members":178,"receipt_members":receipts,"preliminary_members":preliminary,"raw_profiles":6,"gzip_roundtrip_verified":true,"source_hex_roundtrip_verified":true,"UTF8_member_hashes_verified":true,"actual_driver_inverse_recovered":true,"private_arrays_or_executables_published":false});
    fs::write(out.join("artifacts/publication.json"),serde_json::to_vec_pretty(&record).unwrap()).unwrap();
    let mut sums=String::new(); let files=walk(out);
    for p in &files{writeln!(sums,"{}  {}",hash(&fs::read(p).unwrap()),p.strip_prefix(out).unwrap().display()).unwrap();}
    fs::write(out.join("SHA256SUMS"),sums).unwrap();
    println!("{}",json!({"published_files":files.len()+1,"publication":record}));
}
