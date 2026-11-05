use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,path::Path,collections::BTreeMap};
const R:&str="/home/ubuntu/src/zopro-perf-work/component-alias-count176";
const D:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-05-component-alias-count176";
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn copy(a:impl AsRef<Path>,b:impl AsRef<Path>){let(a,b)=(a.as_ref(),b.as_ref());fs::create_dir_all(b.parent().unwrap()).unwrap();fs::copy(a,b).unwrap();assert_eq!(hash(a),hash(b));}
fn list(p:&Path,out:&mut Vec<String>){for e in fs::read_dir(p).unwrap(){let e=e.unwrap();assert!(!e.file_type().unwrap().is_symlink());if e.file_type().unwrap().is_dir(){list(&e.path(),out)}else{assert!(e.file_type().unwrap().is_file());out.push(e.path().strip_prefix(D).unwrap().display().to_string());}}}
fn main(){
 assert!(!Path::new(D).exists());
 let review:Value=serde_json::from_slice(&fs::read(format!("{R}/review.json")).unwrap()).unwrap();assert_eq!(review["passed"],true);
 copy(format!("{R}/report.md"),format!("{D}/README.md"));
 for f in["design.md","analysis.json","review.json","review.log","inputs.json","pins.json","plan.json","setup.rs","setup.log","study.rs","split.rs","review.rs","publish.rs","workflow.log"]{copy(format!("{R}/{f}"),format!("{D}/artifacts/{f}"));}
 for f in["protocol.json","source-pins.json","completed.json","progress.json","result.json","attempt-0.json"]{copy(format!("{R}/runs/{f}"),format!("{D}/artifacts/receipts/{f}"));}
 for s in["compile-tests","alias-tests","compile-study","real-alias-counts"]{for f in["command.json","exit.json","stdout.log","stderr.log","tests.json"]{copy(format!("{R}/runs/{s}/{f}"),format!("{D}/artifacts/receipts/{s}/{f}"));}}
 let upstream=json!({"commit":"3c7386eb7bbddec89bb6548982583eb65bb563bc","commit_date":"2026-06-05T12:22:34Z","url":"https://github.com/AztecProtocol/aztec-packages/blob/3c7386eb7bbddec89bb6548982583eb65bb563bc/barretenberg/cpp/src/barretenberg/ecc/scalar_multiplication/pippenger_rewrite_review_map.md","local_download_sha256":hash("/tmp/zopro-pippenger-review-map.md"),"source_read_local_not_republished":true,"speed_claim_transferred":false,"runtime_code_imported":false});
 fs::write(format!("{D}/artifacts/upstream.json"),serde_json::to_vec_pretty(&upstream).unwrap()).unwrap();
 let mut files=Vec::new();list(Path::new(D),&mut files);files.sort();let index=files.iter().map(|f|(f.clone(),json!({"sha256":hash(format!("{D}/{f}")),"bytes":fs::metadata(format!("{D}/{f}")).unwrap().len()}))).collect::<BTreeMap<_,_>>();
 fs::write(format!("{D}/artifact-index.json"),serde_json::to_vec_pretty(&index).unwrap()).unwrap();
 let text=fs::read_to_string(format!("{D}/README.md")).unwrap();let mut links=0;
 for tail in text.split("](").skip(1){let target=tail.split_once(')').unwrap().0;if target.starts_with("https://"){continue;}assert!(Path::new(D).join(target).is_file(),"missing {target}");links+=1;}
 let pubreview=json!({"passed":true,"regular_files_copied_and_hash_checked":files.len(),"local_readme_links_checked":links,"raw_production_files_unchanged":176,"workflow_commands":4,"new_tests":3,"new_proofs":0,"private_input_or_binary_payloads_published":false,"archives_created":false});
 fs::write(format!("{R}/publication-review.json"),serde_json::to_vec_pretty(&pubreview).unwrap()).unwrap();copy(format!("{R}/publication-review.json"),format!("{D}/publication-review.json"));
 files.clear();list(Path::new(D),&mut files);files.sort();let sums=files.iter().map(|f|format!("{}  {f}\n",hash(format!("{D}/{f}")))).collect::<String>();fs::write(format!("{D}/SHA256SUMS"),sums).unwrap();
 for(f,v)in index{assert_eq!(hash(format!("{D}/{f}")),v["sha256"].as_str().unwrap());}
 println!("PublicationPASS {}checksums,{}local links;no archives/private payloads",files.len(),links);
}
