use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/g1-offset177";
const D:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-11-g1-offset177";
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){let review:Value=serde_json::from_slice(&fs::read(format!("{R}/build-review.json")).unwrap()).unwrap();assert_eq!(review["passed"],true);
 assert!(!Path::new(&format!("{D}/artifacts")).exists());fs::create_dir(format!("{D}/artifacts")).unwrap();
 for name in["prepare.rs","review.rs","review-build.rs","review-build-initial.rs","save-stage.rs","design.md","source-audit.json","source-review.json","build-review.json","candidate-plan.json","control-plan.json","portable-plan.json","candidate-flags.txt","control-flags.txt"]{fs::copy(format!("{R}/{name}"),format!("{D}/artifacts/{name}")).unwrap();}
 for(rel,name)in[("crates/zopro-bn254/src/ifma8/affine_offset.rs","affine_offset.rs"),("crates/zopro-core/src/msm/affine_offset_caller.rs","affine_offset_caller.rs")]{fs::copy(format!("{R}/source/{rel}"),format!("{D}/artifacts/{name}")).unwrap();}
 let checkpoint=json!({"status":"native and portable preparation complete; full-proof protocol not yet frozen","accepted_commit":"e387296070ea96090de8c50a521d494c1aed0158","consumers":review["consumers"],"correctness":{"native_core":130,"native_prover_unit":2,"native_cubic":6,"generic_core":97},"performance":"untested","adopted":false,"live_handles":[],"terminal_sessions":[59026,96429,69344],"new_SDK_calls":0,"new_Gnark_exports":0,"new_AWS_resources":0,"canonical_count":52000,"canonical_path":"/home/ubuntu/src/zopro-perf-work/gather-backing177/proof-sha256-after-capture.json","canonical_sha256":"7069c3c56ce4619e024753ef25eccd24b8d36ac1211929b923fdd2767e1e76ca","push_approved":false,"next":"Freeze full SDK protocol, exact independent backings and readers before any preflight. Reuse these successful builds/tests. Old combined candidate remains failed; no adoption from its G1 subset."});
 fs::write(format!("{R}/checkpoint.json"),serde_json::to_vec_pretty(&checkpoint).unwrap()).unwrap();
 fs::copy(format!("{R}/checkpoint.json"),format!("{D}/artifacts/checkpoint.json")).unwrap();
 let mut entries=fs::read_dir(format!("{D}/artifacts")).unwrap().map(|e|e.unwrap().path()).collect::<Vec<_>>();entries.push(Path::new(D).join("README.md"));entries.sort();let mut sums=String::new();for p in entries{sums.push_str(&format!("{}  {}\n",hash(&p),p.strip_prefix(D).unwrap().display()));}fs::write(format!("{D}/SHA256SUMS"),sums).unwrap();
 println!("Saved preparation artifacts and checkpoint; no publication or proof execution.");
}
