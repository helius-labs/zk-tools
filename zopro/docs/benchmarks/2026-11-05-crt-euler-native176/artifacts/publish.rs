use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{fs,path::{Path,PathBuf},process::{Command,Stdio}};
const R:&str="/home/ubuntu/src/zopro-perf-work/crt-euler-native176";
const P:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-05-crt-euler-native176";
fn sha(x:&[u8])->String{format!("{:x}",Sha256::digest(x))}
fn files(root:&Path)->Vec<PathBuf>{let mut out=Vec::new();for e in fs::read_dir(root).unwrap(){let e=e.unwrap();let ty=e.file_type().unwrap();if ty.is_dir(){out.extend(files(&e.path()));}else{assert!(ty.is_file());out.push(e.path());}}out.sort();out}
fn jsonfile(path:impl AsRef<Path>,v:&Value){fs::write(path,serde_json::to_vec_pretty(v).unwrap()).unwrap();}
fn main(){
 let a=Path::new(P).join("artifacts");fs::create_dir_all(&a).unwrap();assert!(!a.join("receipts.json.gz").exists());
 let local=["base.rs","direct.rs","native.rs","qualify.rs","bench.rs","perf.rs","prepare.rs","prepare-bench.rs","campaign.rs","sampling.rs","metadata.rs","counter.rs","review-initial.rs","review.rs","publish.rs","design.md","capture-design.md","capture-design-final.md","reproduce.md","libraries.json","identities.json","qualification.json","pins.json","build-pins.json","capture-pins.json","plan.json","build-plan.json","capture-plan.json","cases.json","jobs.json","analysis.json","review.json","projection.s","crt-dot.s"];
 for name in local{fs::copy(format!("{R}/{name}"),a.join(name)).unwrap();}
 let old=Path::new("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-05-crt-euler176/artifacts");for name in["Cargo.toml","Cargo.lock","TFHE-NTT-LICENSE","upstream.json"]{fs::copy(old.join(name),a.join(name)).unwrap();}
 let manifest:Value=serde_json::from_slice(&fs::read("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json").unwrap()).unwrap();
 jsonfile(a.join("accepted-baseline.json"),&json!({"inherited_not_remeasured":true,"native_binary_sha256":manifest["native_binary_sha256"],"ms":manifest["ms"],"m5":manifest["m5"],"CPU10x_rows":0,"proof_ledger_count":29953}));
 let mut member_paths=Vec::new();for stage in["runs","build","capture"]{for path in files(&Path::new(R).join(stage)){member_paths.push(path);}}
 for name in["review-initial.log","review.log","prepare.log","prepare-bench.log","workflow.log","build.log","capture-prepare.log","capture.log","analysis.log"]{member_paths.push(Path::new(R).join(name));}
 for dir in["stat"]{member_paths.extend(files(&Path::new(R).join(dir)));}member_paths.sort();
 let mut members=Vec::new();for p in&member_paths{let relative=p.strip_prefix(R).unwrap().to_str().unwrap();assert!(!relative.contains(".."));assert!(relative.ends_with(".json")||relative.ends_with(".log")||relative.ends_with(".csv"));let data=fs::read(p).unwrap();let utf8=String::from_utf8(data.clone()).unwrap();members.push(json!({"path":relative,"bytes":data.len(),"sha256":sha(&data),"utf8":utf8}));}
 let archive=json!({"format":"exact-utf8-receipt-members-v1","members":members});let raw=serde_json::to_vec(&archive).unwrap();let temporary=Path::new(R).join("publication-receipts.json");fs::write(&temporary,&raw).unwrap();
 let gzip=a.join("receipts.json.gz");let status=Command::new("gzip").args(["-n","-9","-c"]).arg(&temporary).stdout(Stdio::from(fs::File::create(&gzip).unwrap())).status().unwrap();assert!(status.success());
 let decoded=Command::new("gzip").arg("-dc").arg(&gzip).output().unwrap();assert!(decoded.status.success());assert_eq!(decoded.stdout,raw);
 let unpacked:Value=serde_json::from_slice(&decoded.stdout).unwrap();for m in unpacked["members"].as_array().unwrap(){let s=m["utf8"].as_str().unwrap().as_bytes();assert_eq!(s.len()as u64,m["bytes"].as_u64().unwrap());assert_eq!(sha(s),m["sha256"].as_str().unwrap());assert_eq!(fs::read(Path::new(R).join(m["path"].as_str().unwrap())).unwrap(),s);}
 let readme=fs::read_to_string(Path::new(P).join("README.md")).unwrap();let mut links=Vec::new();let mut tail=readme.as_str();while let Some(start)=tail.find("]("){tail=&tail[start+2..];let end=tail.find(')').unwrap();let link=&tail[..end];assert!(!link.starts_with("http"));assert!(Path::new(P).join(link).is_file()||["artifact-index.json","SHA256SUMS"].contains(&link),"missing link {link}");links.push(link.to_string());tail=&tail[end+1..];}
 let review=json!({"publication_passed":true,"receipt_members":member_paths.len(),"archive_sha256":sha(&fs::read(&gzip).unwrap()),"uncompressed_sha256":sha(&raw),"archive_exact_roundtrip":true,"all_extracted_bytes_and_hashes_match":true,"README_local_links_checked":links,"binary_library_raw_perf_private_input_payloads_excluded":true,"initial_reviewer_schema_failure_preserved":true});jsonfile(Path::new(P).join("publication-review.json"),&review);
 let artifact_rows=files(Path::new(P)).into_iter().map(|p|{let b=fs::read(&p).unwrap();json!({"path":p.strip_prefix(P).unwrap().to_str().unwrap(),"bytes":b.len(),"sha256":sha(&b)})}).collect::<Vec<_>>();
 jsonfile(Path::new(P).join("artifact-index.json"),&json!({"files":artifact_rows,"receipt_archive":"artifacts/receipts.json.gz","excluded":["executables","rlibs","perf.data","private_keys_images_witnesses","input_arrays"]}));
 let mut sums=String::new();for p in files(Path::new(P)){assert_ne!(p.file_name().unwrap(),"SHA256SUMS");sums.push_str(&format!("{}  {}\n",sha(&fs::read(&p).unwrap()),p.strip_prefix(P).unwrap().to_str().unwrap()));}fs::write(Path::new(P).join("SHA256SUMS"),sums).unwrap();
 for link in&links{assert!(Path::new(P).join(link).is_file());}
 println!("Publication PASS: {} exact UTF8 receipt members, {} indexed files, {} README links; archive roundtrip verified",member_paths.len(),files(Path::new(P)).len()-1,links.len());
}
