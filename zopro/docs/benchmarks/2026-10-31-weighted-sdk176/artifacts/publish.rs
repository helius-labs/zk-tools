use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fmt::Write,fs,path::{Path,PathBuf},process::Command};
const R:&str="/home/ubuntu/src/zopro-perf-work/weighted-sdk176";
const D:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-31-weighted-sdk176";
fn r(p:impl AsRef<Path>)->PathBuf{Path::new(R).join(p)}
fn d(p:impl AsRef<Path>)->PathBuf{Path::new(D).join(p)}
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn walk(p:&Path,out:&mut Vec<PathBuf>){for e in fs::read_dir(p).unwrap(){let e=e.unwrap();let t=e.file_type().unwrap();if t.is_dir(){walk(&e.path(),out)}else if t.is_file(){out.push(e.path())}}}
fn interval(s:&Value)->String{format!("{:.6} [{:.6}, {:.6}], {}/15",s["median"].as_f64().unwrap(),s["low"].as_f64().unwrap(),s["high"].as_f64().unwrap(),s["wins"].as_u64().unwrap())}
fn archive(name:&str,mut files:Vec<PathBuf>)->Value{
 files.sort();files.dedup();assert!(!files.is_empty());
 let relative=files.iter().map(|p|p.strip_prefix(R).unwrap().to_str().unwrap().to_owned()).collect::<Vec<_>>();
 for p in &files{assert!(p.symlink_metadata().unwrap().file_type().is_file());}
 let to=r(format!("publication-roundtrip-{name}"));assert!(!to.exists());fs::create_dir(&to).unwrap();let file=d(format!("artifacts/{name}.tar.gz"));
 assert!(Command::new("tar").args(["--sort=name","--mtime=@0","--owner=0","--group=0","--numeric-owner","-czf"]).arg(&file).arg("-C").arg(R).args(&relative).status().unwrap().success());
 assert!(Command::new("tar").arg("-xzf").arg(&file).arg("-C").arg(&to).status().unwrap().success());
 let mut got=vec![];walk(&to,&mut got);got.sort();let actual=got.iter().map(|p|p.strip_prefix(&to).unwrap().to_str().unwrap().to_owned()).collect::<Vec<_>>();assert_eq!(actual,relative);
 for (original,extracted)in files.iter().zip(&got){assert_eq!(hash(original),hash(extracted));}
 json!({"regular_files":files.len(),"sha256":hash(file),"exact_member_set_and_bytes_verified":true})
}
fn main(){
 assert!(!d("README.md").exists());
 let a=read(r("analysis.json"));let v=read(r("review.json"));let cases=read(r("cases.json"));let mem=read(r("memory-audit/result.json"));let i=read(r("interpretation.json"));
 assert_eq!(v["passed"],true);assert_eq!(i["passed"],true);assert_eq!(v["gate_passed"],false);assert_eq!(v["gate_passed"],a["gate_passed"]);assert_eq!(v["ledger"],28847);assert_eq!(v["workflow_command_receipts"],1049);assert_eq!(a["total_SDK_requests"],36762);assert_eq!(v["direct_core_tests"],130);
 // Verify the complete immutable source/binary/protocol pin set again before publication.
 let pins=read(r("continuation-pins.json"));for(p,h)in pins.as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap(),"{p}");}
 let manifest=read("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json");let originals=manifest["source_files_sha256"].as_object().unwrap();assert_eq!(originals.len(),176);
 for(p,h)in originals{assert_eq!(hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)),h.as_str().unwrap());}
 assert_eq!(hash("/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175/native/candidate-sdk"),"6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661");
 assert_eq!(hash(r("links/sdk-sdk")),"74bbd21bbd95013419798c9e2e85468a1a082caeab5e9c921873a2304f524443");
 let mut doc=fs::read_to_string(r("report.md.in")).unwrap();
 let mut t=String::new();for control in["accepted","OFF"]{writeln!(t,"Original-eight ON/{control}: **{}**.\n",interval(&a["original8_aggregates"][control]["statistics"])).unwrap();}
 t.push_str("|Context|Accepted / OFF / ON ms|ON/accepted [bounds], wins|ON/OFF [bounds], wins|OFF/accepted [bounds], wins|\n|---|---:|---|---|---|\n");
 for c in cases.as_array().unwrap(){let n=c["name"].as_str().unwrap();let f=|arm:&str|a["cells"][format!("{n}/{arm}")]["median_ms"].as_f64().unwrap();writeln!(t,"|{n}|{:.6} / {:.6} / {:.6}|{}|{}|{}|",f("accepted"),f("OFF"),f("ON"),interval(&a["pairs"][format!("{n}/ON/accepted")]["statistics"]),interval(&a["pairs"][format!("{n}/ON/OFF")]["statistics"]),interval(&a["pairs"][format!("{n}/OFF/accepted")]["statistics"])).unwrap();}
 t.push_str("\n|Context|Accepted / OFF / ON CPU ms/request|Accepted / OFF / ON recorded max-RSS counter MiB (diagnostic)|\n|---|---:|---:|\n");
 for c in cases.as_array().unwrap(){let n=c["name"].as_str().unwrap();let f=|arm:&str,k:&str|a["cells"][format!("{n}/{arm}")][k].as_f64().unwrap();writeln!(t,"|{n}|{:.3} / {:.3} / {:.3}|{:.2} / {:.2} / {:.2}|",f("accepted","CPU_ms"),f("OFF","CPU_ms"),f("ON","CPU_ms"),f("accepted","max_RSS_MiB"),f("OFF","max_RSS_MiB"),f("ON","max_RSS_MiB")).unwrap();}
 doc=doc.replace("{{TIMING}}",&t);let mut t=String::from("|Context|Accepted / OFF / ON sampled RSS max MiB|ON/accepted / ON/OFF|Accepted / OFF / ON snapshots|\n|---|---:|---:|---:|\n");
 for c in cases.as_array().unwrap(){let n=c["name"].as_str().unwrap();let row=|arm:&str|mem["cases"].as_array().unwrap().iter().find(|x|x["name"]==format!("{n}-{arm}")).unwrap();let peak=|arm:&str|row(arm)["sampled_timed_RSS_max_KiB"].as_f64().unwrap()/1024.;writeln!(t,"|{n}|{:.3} / {:.3} / {:.3}|{:.6} / {:.6}|{} / {} / {}|",peak("accepted"),peak("OFF"),peak("ON"),a["sampled_RSS_pairs"][format!("{n}/ON/accepted")]["ratio"].as_f64().unwrap(),a["sampled_RSS_pairs"][format!("{n}/ON/OFF")]["ratio"].as_f64().unwrap(),row("accepted")["frames"].as_array().unwrap().len(),row("OFF")["frames"].as_array().unwrap().len(),row("ON")["frames"].as_array().unwrap().len()).unwrap();}
 writeln!(t,"\nRecorded-counter warnings retained without gating: `{}`.",a["counter_RSS_diagnostic_warnings"]).unwrap();doc=doc.replace("{{MEMORY}}",&t);
 let mut t=String::from("|Context|G1 counters|G2 counters|\n|---|---|---|\n");for c in cases.as_array().unwrap(){let n=c["name"].as_str().unwrap();let log=fs::read_to_string(r(format!("runs-continuation/b0-{n}-ON/stdout.log"))).unwrap();let row:Value=serde_json::from_str(log.lines().find_map(|l|l.strip_prefix("COMPACT_WEIGHTED ")).unwrap()).unwrap();writeln!(t,"|{n}|`{}`|`{}`|",row["counts"][0],row["counts"][1]).unwrap();}doc=doc.replace("{{ROUTES}}",&t);
 let mut t=String::from("|Counter panel|Cycles M/request|Instructions M/request|Cache misses M/request|Branch misses M/request|Profile samples|Unknown share|\n|---|---:|---:|---:|---:|---:|---:|\n");
 for(n,c)in a["counters"].as_object().unwrap(){let p=&c["per_request"];let key=n.replacen("stat-","cycles-",1);writeln!(t,"|{n}|{:.3}|{:.3}|{:.6}|{:.6}|{}|{:.5}|",p["cycles:u"].as_f64().unwrap()/1e6,p["instructions:u"].as_f64().unwrap()/1e6,p["cache-misses:u"].as_f64().unwrap()/1e6,p["branch-misses:u"].as_f64().unwrap()/1e6,a["profiles"][&key]["samples"],a["profiles"][&key]["unknown_period_share"].as_f64().unwrap()).unwrap();}
 t.push_str("\n|ON profile|Named weighted-tree context share|General field-inverse share (shared callers)|\n|---|---:|---:|\n");for(n,p)in i["profiles"].as_object().unwrap().iter().filter(|(n,_)|n.ends_with("-ON")){writeln!(t,"|{n}|{:.3}%|{:.3}%|",100.*p["weighted_tree_named_context_share"].as_f64().unwrap(),100.*p["general_field_inverse_named_share"].as_f64().unwrap()).unwrap();}doc=doc.replace("{{COUNTERS}}",&t);
 let mut t=String::from("|Primary context / arm|Minor faults/request|Voluntary switches/request|Involuntary switches/request|System ms/request|CPU ms / loop wall ms|\n|---|---:|---:|---:|---:|---:|\n");
 for n in["1x2-w16","1x2-w32"]{for arm in["accepted","OFF","ON"]{let p=&i["latency_usage_medians"][n][arm];writeln!(t,"|{n}/{arm}|{:.3}|{:.3}|{:.3}|{:.3}|{:.3}|",p["minor_faults"].as_f64().unwrap(),p["voluntary_switches"].as_f64().unwrap(),p["involuntary_switches"].as_f64().unwrap(),p["system_cpu_ms"].as_f64().unwrap(),p["CPU_ms_divided_by_loop_wall_ms"].as_f64().unwrap()).unwrap();}}doc=doc.replace("{{USAGE}}",&t);
 let mut t=String::from("Fresh experimental ON observations against supplied historical references:\n\n|Shape|ON ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× target ms|Target met|\n|---|---:|---:|---:|---:|---:|---:|---|\n");let mut met=0;
 for c in cases.as_array().unwrap().iter().filter(|c|c["workers"]==32&&!c["reference"].is_null()){let n=c["name"].as_str().unwrap();let s=&a["new_reference_comparisons"][n]["variants"]["ON"];let cpu=c["reference"]["cpu_ms"].as_f64().unwrap();let metal=c["reference"]["metal_ms"].as_f64().unwrap();if s["CPU10x"]==true{met+=1;}writeln!(t,"|{}|{:.6}|{cpu}|{:.3}×|{metal}|{:.3}×|{:.1}|{}|",c["shape"].as_str().unwrap(),s["ms"].as_f64().unwrap(),s["CPU_speedup"].as_f64().unwrap(),s["Metal_speedup"].as_f64().unwrap(),cpu/10.,s["CPU10x"]).unwrap();}assert_eq!(met,0);doc=doc.replace("{{REFERENCES}}",&t);assert!(!doc.contains("{{"));
 fs::create_dir_all(d("artifacts")).unwrap();fs::write(d("README.md"),&doc).unwrap();
 for p in["analysis.json","review.json","review-v2.log","source-review.json","source-audit.json","interpretation.json","cases.json","identities.json","proof-inventory.json","profile-symbols.json","design.md","continuation-note.md","publish.rs","report.md.in"]{fs::copy(r(p),d("artifacts").join(p)).unwrap();}
 let mut groups=BTreeMap::new();for(name,path)in[("source","source"),("proofs","proofs"),("counters","stat"),("memory-audit","memory-audit")]{let mut files=vec![];walk(&r(path),&mut files);if name=="source"{assert_eq!(files.len(),179)}if name=="proofs"{assert_eq!(files.len(),996)}if name=="counters"{assert_eq!(files.len(),9)}groups.insert(name,files);}
 let mut receipts=vec![];for path in["native","links","runs","runs-continuation"]{walk(&r(path),&mut receipts);}receipts.retain(|p|p.extension().is_some_and(|x|x=="log"||x=="json"));
 let mut workflow=vec![];for e in fs::read_dir(R).unwrap(){let e=e.unwrap();if !e.file_type().unwrap().is_file(){continue}let p=e.path();let n=e.file_name();let n=n.to_str().unwrap();if p.extension().is_some_and(|x|x=="log")&&!matches!(n,"publish.log"){receipts.push(p.clone());}if p.extension().is_some_and(|x|x=="rs"||x=="json"||x=="txt"||x=="md"||x=="in")&&!matches!(n,"status-before-capture.json"|"publication.json"){workflow.push(p);}}
 walk(&r("support"),&mut workflow);groups.insert("workflow",workflow);groups.insert("receipts",receipts);
 let archives=groups.into_iter().map(|(n,f)|(n,archive(n,f))).collect::<BTreeMap<_,_>>();
 fs::write(d("artifacts/publication.json"),serde_json::to_vec_pretty(&json!({"archives":archives,"pin_entries_checked":pins.as_object().unwrap().len(),"accepted_production176_and_SDK_unchanged":true,"candidate_SDK_sha256":"74bbd21bbd95013419798c9e2e85468a1a082caeab5e9c921873a2304f524443","gate_passed":false,"adopted":false,"production_changed":false,"goal_achieved":false,"candidate_CPU10x_shape_count":met,"workflow_command_receipts":1049,"successful_structured_commands":1048,"failed_readonly_schema_checks":1,"native_core_tests":130,"native_prover_tests":8,"field_tests_inherited":37,"primary_SDK_requests":34662,"resource_SDK_requests":2100,"total_SDK_requests":36762,"new_Gnark_exports":498,"prior_proof_ledger":28349,"proof_ledger":28847})).unwrap()).unwrap();
 let mut rest=doc.as_str();let mut links=0;while let Some(i)=rest.find("]("){rest=&rest[i+2..];let j=rest.find(')').unwrap();let link=&rest[..j];assert!(!link.starts_with("http"));if link!="SHA256SUMS"{assert!(d(link).is_file(),"{link}");}links+=1;rest=&rest[j+1..];}
 let mut files=vec![];walk(Path::new(D),&mut files);files.sort();let mut sums=String::new();for p in &files{if p!=&d("SHA256SUMS"){writeln!(sums,"{}  {}",hash(p),p.strip_prefix(D).unwrap().display()).unwrap();}}fs::write(d("SHA256SUMS"),sums).unwrap();println!("Publication PASS:6exactarchives/{links}links/{}files;498Gnarkexports/ledger28847;gatefalse,productionunchanged",files.len()+1);
}
