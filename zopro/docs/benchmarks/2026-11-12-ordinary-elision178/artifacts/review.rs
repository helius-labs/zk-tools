use serde_json::{json,Value};use sha2::{Digest,Sha256};
use std::{fs,collections::{BTreeMap,BTreeSet},os::unix::fs::MetadataExt};
const R:&str="/home/ubuntu/src/zopro-perf-work/ordinary-elision178";
const Z:&str="/home/ubuntu/src/zk-tools/zopro";
fn read(p:&str)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:&str)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){
 let bindings=read(&format!("{R}/production-bindings.json"));let mut exact=0;let mut restored=0;
 for(relative,h)in bindings.as_object().unwrap(){assert_eq!(hash(&format!("{Z}/{relative}")),h.as_str().unwrap());let p=format!("{R}/source/{relative}");if hash(&p)==h.as_str().unwrap(){exact+=1;continue}
  let original=fs::read_to_string(format!("{Z}/{relative}")).unwrap();let mut s=fs::read_to_string(&p).unwrap();match relative.as_str(){
   "crates/zopro-core/src/lib.rs"=>{assert!(s.starts_with(&original));assert_eq!(s[original.len()..].matches("pub fn ").count(),2);},
   "crates/zopro-core/src/msm_affine.rs"=>assert_eq!(s.strip_suffix("\npub(crate) mod phase_elision;\n").unwrap(),original),
   "crates/zopro-core/src/msm_affine/g2_phi.rs"=>{for e in read(&format!("{R}/edits.json")).as_array().unwrap().iter().rev(){let a=e["after"].as_str().unwrap();let b=e["before"].as_str().unwrap();assert_eq!(s.matches(a).count(),1);s=s.replacen(a,b,1);}assert_eq!(s,original);},
   _=>panic!("Unexpected source change {relative}"),
  }restored+=1;
 }assert_eq!((exact,restored),(174,3));
 for pins in["build-pins.json","capture-pins.json"]{for(p,h)in read(&format!("{R}/{pins}")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}}
 let digest=hash(&format!("{R}/target/release/zopro-ternary-g2-caller"));let mut inodes=BTreeSet::new();for i in 0..3{let p=format!("{R}/bin/copy{i}");assert_eq!(hash(&p),digest);let m=fs::metadata(&p).unwrap();assert!(inodes.insert((m.dev(),m.ino())));}
 let mut raw=BTreeMap::new();let mut elapsed=BTreeMap::new();let mut processes=0;let mut queries=0;
 for kind in["preflight","main"]{assert_eq!(read(&format!("{R}/{kind}-run/result.json"))["status"],"complete");let mut command_ms=0.;
  for step in read(&format!("{R}/{kind}-plan.json"))["steps"].as_array().unwrap(){let name=step["name"].as_str().unwrap();let p=format!("{R}/{kind}-run/{name}");let e=read(&format!("{p}/exit.json"));assert_eq!(e["returncode"],0);assert_eq!(e["success"],true);for stream in["stdout","stderr"]{assert_eq!(hash(&format!("{p}/{stream}.log")),e[format!("{stream}_sha256")]);}command_ms+=e["elapsed_ms"].as_f64().unwrap();
   let command=read(&format!("{p}/command.json"));assert_eq!(&command["argv"].as_array().unwrap()[4..],step["argv"].as_array().unwrap());
   let text=fs::read_to_string(format!("{p}/stdout.log")).unwrap();let row:Value=serde_json::from_str(text.trim().strip_prefix("CALLER ").unwrap()).unwrap();let m=step["environment"]["ZOPRO_DIAGNOSTIC_ORDINARY_MODE"].as_str().unwrap().parse::<u64>().unwrap();let n=if kind=="main"{17}else{1};assert_eq!(row["mode"],m);assert_eq!(row["rounds"],n);assert_eq!(row["phase_status"],json!([20*(n+4),if m%2==1{20}else{20*(n+4)},10*(n+4),if m>=2{10}else{10*(n+4)},20,10,5244800]));for field in["every_output_matches_independent_full_MSM","point_inputs_unchanged","all_timed_outputs_checked","normalization_inside_timer","B_word_reset_inside_timer"]{assert_eq!(row[field],true);}
   assert_eq!(row["samples_ns"].as_array().unwrap().len(),n as usize);let sum=row["samples_ns"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u128).sum::<u128>();let ms=sum as f64/n as f64/1e6;if kind=="main"{assert!(raw.insert(name.to_owned(),(ms,row["usage"].clone())).is_none());}processes+=1;queries+=n+4;
  }elapsed.insert(kind,command_ms/1000.);
 }assert_eq!((processes,queries),(96,1632));
 let result=read(&format!("{R}/result.json"));assert_eq!(result["checked_queries"],queries);let mut summary=vec![];
 for row in result["rows"].as_array().unwrap(){let case=row["case"].as_str().unwrap();let mut all=vec![];
  for block in row["blocks"].as_array().unwrap(){let b=block["block"].as_u64().unwrap();let mut ms=vec![];for m in 0..4{let (t,u)=&raw[&format!("b{b}-{case}-m{m}")];assert!((block["ms"][m].as_f64().unwrap()-t).abs()<1e-11);assert_eq!(block["usage"][m],*u);ms.push(*t);}for(k,v)in[("fill_elision_ratio",ms[1]/ms[0]),("reduction_elision_ratio",ms[2]/ms[0]),("both_elision_ratio",ms[3]/ms[0]),("nonadditivity_ms",ms[0]-ms[1]-ms[2]+ms[3])]{assert!((block[k].as_f64().unwrap()-v).abs()<1e-11);}all.push(ms);
  }let median=|mut a:Vec<f64>|{a.sort_by(f64::total_cmp);assert_eq!(a.len(),3);a[1]};let medians=(0..4).map(|m|median(all.iter().map(|a|a[m]).collect())).collect::<Vec<_>>();let ratios=(1..4).map(|m|median(all.iter().map(|a|a[m]/a[0]).collect())).collect::<Vec<_>>();summary.push(json!({"case":case,"median_ms":medians,"paired_ratios":ratios}));
 }
 let ledger="/home/ubuntu/src/zopro-perf-work/bundle-all178/proof-sha256-after-main.json";assert_eq!(hash(ledger),"48e866327435bbc85f7c8d0433115337d7d903481594f307bab4373c39ec3119");assert_eq!(read(ledger).as_array().unwrap().len(),54927);
 let build=read(&format!("{R}/build-run/build/exit.json"));assert_eq!(build["returncode"],0);elapsed.insert("native_build",build["elapsed_ms"].as_f64().unwrap()/1000.);
 let review=json!({"passed":true,"same_agent_separate_reader":true,"unchanged_parents":exact,"recoverable_parents":restored,"processes":processes,"checked_queries":queries,"command_seconds":elapsed,"executable_sha256":digest,"distinct_copy_inodes":inodes,"summary":summary,"canonical_ledger_unchanged":54927,"new_proofs":0});fs::write(format!("{R}/review.json"),serde_json::to_vec_pretty(&review).unwrap()).unwrap();println!("{}",serde_json::to_string(&review).unwrap());
}
