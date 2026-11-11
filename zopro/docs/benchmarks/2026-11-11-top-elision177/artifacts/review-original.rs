use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{fs,collections::BTreeSet,os::unix::fs::MetadataExt};
const R:&str="/home/ubuntu/src/zopro-perf-work/top-elision177";
const Z:&str="/home/ubuntu/src/zk-tools/zopro";
fn read(p:&str)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:&str)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn replace(s:&mut String,a:&str,b:&str){assert_eq!(s.matches(a).count(),1);*s=s.replacen(a,b,1)}
fn main(){
 let mut exact=0;let mut restored=0;let bindings=read(&format!("{R}/production-bindings.json"));
 for (relative,h) in bindings.as_object().unwrap(){let current=format!("{Z}/{relative}");assert_eq!(hash(&current),h.as_str().unwrap());let private=format!("{R}/source/{relative}");if hash(&private)==h.as_str().unwrap(){exact+=1;continue}
  let original=fs::read_to_string(&current).unwrap();let mut s=fs::read_to_string(&private).unwrap();match relative.as_str(){
   "crates/zopro-core/src/lib.rs"=>{assert!(s.starts_with(&original));assert!(s[original.len()..].starts_with("\n/// Private fixed-input diagnostic API;"));restored+=1;continue},
   "crates/zopro-core/src/msm_affine.rs"=>replace(&mut s,"\npub(crate) mod top_elision;\n",""),
   "crates/zopro-core/src/msm_affine/g2_phi.rs"=>{
    replace(&mut s,"fn hb_window(bases: &[G2Affine], short: &[[u64; 4]], w: usize, bits: usize, part: usize)","fn hb_window(bases: &[G2Affine], short: &[[u64; 4]], w: usize, bits: usize)");
    replace(&mut s,"super::top_elision::call(part * 2, || hb_top_component(bases, short, true, w, bits, top));","hb_top_component(bases, short, true, w, bits, top);");
    replace(&mut s,"super::top_elision::call(part * 2 + 1, || hb_top_component(bases, short, false, w, bits, top));","hb_top_component(bases, short, false, w, bits, top);");
    replace(&mut s,"hb_window(&bases[start..end], &short[start..end], w, bits, part)","hb_window(&bases[start..end], &short[start..end], w, bits)");
   },_=>panic!("Unexpected change {relative}"),
  }assert_eq!(s,original);restored+=1;
 }assert_eq!(restored,3);
 let binary=format!("{R}/target/release/zopro-ternary-g2-caller");let digest=hash(&binary);let mut inodes=BTreeSet::new();for i in 0..3{let p=format!("{R}/bin/copy{i}");assert_eq!(hash(&p),digest);let m=fs::metadata(&p).unwrap();assert!(inodes.insert((m.dev(),m.ino())));}
 let fingerprint=read(&format!("{R}/target/release/.fingerprint/zopro-core-2daf7c54e96aeead/lib-zopro_core.json"));let flags=fingerprint["rustflags"].as_array().unwrap().iter().map(|s|s.as_str().unwrap()).collect::<Vec<_>>().join(" ");for flag in["target-cpu=znver4","--cfg zopro_g2_horner_buckets","--cfg zopro_g2_prefix_records","--cfg zopro_dense_busy_reset","--cfg zopro_phi_grain"]{assert!(flags.contains(flag),"{flag}");}assert!(!flags.contains("--cfg zopro_g2_coordinate_leaf"));
 let mut count=0;let mut calls=0;let mut elapsed=0.;let mut raw_rows=vec![];
 for kind in["preflight","main"]{let plan=read(&format!("{R}/{kind}-plan.json"));let rows=read(&format!("{R}/{kind}-rows.json"));let rounds=if kind=="main"{17u64}else{1};
  for (step,row)in plan["steps"].as_array().unwrap().iter().zip(rows.as_array().unwrap()){
   let name=step["name"].as_str().unwrap();assert_eq!(row["name"],name);let p=format!("{R}/{kind}-run/{name}");let raw=fs::read_to_string(format!("{p}/stdout.log")).unwrap();let data:Value=serde_json::from_str(raw.trim().strip_prefix("CALLER ").unwrap()).unwrap();assert_eq!(data["rounds"],rounds);let mut sum=0u128;for n in data["samples_ns"].as_array().unwrap(){sum+=n.as_u64().unwrap()as u128;}let mean=sum as f64/rounds as f64;assert_eq!(mean,row["sample_mean_ns"]);assert_eq!(data["every_output_matches_independent_full_MSM"],true);let mode=step["environment"]["ZOPRO_DIAGNOSTIC_ELIDE_TOP"].as_str().unwrap()=="1";assert_eq!(data["elide"],mode);assert_eq!(data["top_status"],json!([4*(rounds+4),if mode{4}else{4*(rounds+4)},4,32768]));
   let e=read(&format!("{p}/exit.json"));assert_eq!(e["returncode"],0);elapsed+=e["elapsed_ms"].as_f64().unwrap();count+=1;calls+=rounds+4;if kind=="main"{raw_rows.push((name.to_string(),mean));}
  }
 }
 assert_eq!((count,calls),(48,816));let result=read(&format!("{R}/result.json"));
 for row in result["rows"].as_array().unwrap(){for pair in row["pairs"].as_array().unwrap(){let b=pair["block"].as_u64().unwrap();let case=row["case"].as_str().unwrap();let get=|m|raw_rows.iter().find(|(n,_)|n==&format!("b{b}-{case}-m{m}")).unwrap().1;let a=get(0);let c=get(1);assert_eq!(pair["ratio"],c/a);assert_eq!(pair["saved_ms"],(a-c)/1e6);}}
 let ledger="/home/ubuntu/src/zopro-perf-work/g2-coordinate-leaf-profile177/proof-sha256-after-profile.json";assert_eq!(read(ledger).as_array().unwrap().len(),53917);assert_eq!(hash(ledger),"a033132ee87cf5d5260c396acf1894bbb1afc95c07d8ba22c55ed9c8e5a85873");
 let review=json!({"passed":true,"unchanged_source_files":exact,"recoverable_parents":restored,"same_agent_separate_reader":true,"caller_processes":count,"checked_queries":calls,"caller_command_seconds":elapsed/1000.,"native_build_seconds":read(&format!("{R}/build-run/build/exit.json"))["elapsed_ms"].as_f64().unwrap()/1000.,"executable_sha256":digest,"distinct_copy_inodes":inodes,"core_fingerprint":fingerprint,"canonical_ledger_unchanged":53917,"new_proofs":0});fs::write(format!("{R}/review.json"),serde_json::to_vec_pretty(&review).unwrap()).unwrap();println!("Source recovery, native flags, distinct executable copies, all 816 query results and raw paired ratios verified; production/ledger unchanged.");
}
