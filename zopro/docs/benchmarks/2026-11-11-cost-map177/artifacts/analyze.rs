use serde_json::{json,Value};use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/cost-map177";
const Z:&str="/home/ubuntu/src/zk-tools/zopro";
const S:&str="/home/ubuntu/src/zopro-perf-work/g2-coordinate-leaf177";
const P:&str="/home/ubuntu/src/zopro-perf-work/g2-coordinate-leaf-profile177";
const RUSTC:&str="/home/ubuntu/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn save(name:&str,v:Value){let p=format!("{R}/{name}");assert!(!Path::new(&p).exists());fs::write(p,serde_json::to_vec_pretty(&v).unwrap()).unwrap()}
fn source_bindings()->Value{
 let old=read(format!("{Z}/docs/benchmarks/2026-11-06-stream-work-census176/artifacts/pins.json"));let mut rows=vec![];
 for rel in["crates/zopro-core/src/msm.rs","crates/zopro-core/src/msm/g2_borrowed.rs","crates/zopro-core/src/simd.rs","crates/zopro-bn254/src/fq2.rs","crates/zopro-bn254/src/ifma8.rs"]{let p=format!("{Z}/{rel}");let h=hash(&p);assert_eq!(h,old[&p]);rows.push(json!({"path":p,"sha256":h,"unchanged_from_qualified_stream_model":true}));}
 json!(rows)
}
fn prepare(){
 let bindings=source_bindings();let frozen=read(format!("{S}/frozen-control.json"));let mut pins=BTreeMap::<String,String>::new();
 for row in bindings.as_array().unwrap(){pins.insert(row["path"].as_str().unwrap().into(),row["sha256"].as_str().unwrap().into());}
 for entry in fs::read_dir(format!("{S}/frozen/control")).unwrap(){let p=entry.unwrap().path();if p.is_file(){pins.insert(p.display().to_string(),hash(p));}}
 for p in[format!("{R}/fold.rs"),format!("{R}/analyze.rs"),format!("{R}/analyze"),format!("{R}/protocol.md"),RUSTC.into(),format!("{S}/frozen-control.json"),format!("{S}/independent-main-review.json"),format!("{P}/result.json"),format!("{P}/proof-sha256-after-profile.json"),format!("{Z}/docs/benchmarks/2026-11-06-stream-work-census176/artifacts/pins.json"),format!("{Z}/crates/zopro-core/src/msm_affine/g2_phi.rs"),format!("{Z}/crates/zopro-core/src/msm_affine/borrowed_pending.rs"),"/usr/bin/taskset".into()]{pins.insert(p.clone(),hash(p));}
 let argv=vec![RUSTC.to_owned(),"--edition=2021".into(),format!("{R}/fold.rs"),"-C".into(),"target-cpu=znver4".into(),"-C".into(),"opt-level=3".into(),"-C".into(),"lto=fat".into(),"-C".into(),"codegen-units=1".into(),"-L".into(),format!("dependency={S}/frozen/control"),"--extern".into(),format!("zopro_bn254={}",frozen["libraries"]["zopro_bn254"].as_str().unwrap()),"--extern".into(),format!("serde_json={}",frozen["libraries"]["serde_json"].as_str().unwrap()),"-o".into(),format!("{R}/fold")];
 save("source-bindings.json",bindings);save("pins.json",json!(pins));save("plan.json",json!({"version":1,"source_pins":format!("{R}/pins.json"),"steps":[{"kind":"command","name":"compile-fold","argv":argv,"timeout_seconds":1800},{"kind":"command","name":"measure-fold","argv":["/usr/bin/taskset","-c","2",format!("{R}/fold")],"timeout_seconds":1800}]}));
 println!("Pinned reused model/profile sources and one six-context final-fold probe; no SDK calls.");
}
fn finish(){
 assert_eq!(read(format!("{R}/run/result.json"))["status"],"complete");
 for(p,h)in read(format!("{R}/pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
 for step in read(format!("{R}/plan.json"))["steps"].as_array().unwrap(){let dir=format!("{R}/run/{}",step["name"].as_str().unwrap());let e=read(format!("{dir}/exit.json"));assert_eq!(e["success"],true);assert_eq!(e["returncode"],0);for stream in["stdout","stderr"]{assert_eq!(hash(format!("{dir}/{stream}.log")),e[format!("{stream}_sha256")]);}let c=read(format!("{dir}/command.json"));assert_eq!(&c["argv"].as_array().unwrap()[4..],step["argv"].as_array().unwrap());}
 let profile=read(format!("{P}/result.json"));assert_eq!(profile["quality_passed"],true);assert_eq!(profile["observer_checks_passed"],true);
 let mut categories=BTreeMap::<String,u64>::new();let mut symbols=BTreeMap::<String,u64>::new();let mut total=0u64;let mut observations=0;
 for row in profile["rows"].as_array().unwrap().iter().filter(|r|r["arm"]=="OFF"&&r["panel"]=="cycles"){
  observations+=1;let d=&row["diagnostic"];assert_eq!(d["qualified"],true);let mut count=0;
  for item in d["symbols"].as_array().unwrap(){let s=item["symbol"].as_str().unwrap();let n=item["samples"].as_u64().unwrap();count+=n;*symbols.entry(s.into()).or_default()+=n;
   let cat=if s.contains("msm::stream_g1_impl"){"G1 streaming"}else if s.contains("msm::g2_borrowed::apply"){"G2 streaming"}else if s.contains("msm::reduce_layer"){"affine reduction layers"}else if s.contains("inverse::inverse")||s.contains("FqBatch>::inverse_nonzero"){"inverse routines"}else if s.contains("crossbeam_epoch"){"epoch symbols"}else if s.contains("rayon_core"){"Rayon symbols"}else if s.contains("::fft::"){"FFT symbols"}else if s.contains("::solver::"){"solver symbols"}else if s.starts_with("[unknown]"){"unknown"}else{"other"};*categories.entry(cat.into()).or_default()+=n;
  }assert_eq!(count,d["samples"].as_u64().unwrap());total+=count;
 }assert_eq!(observations,3);assert_eq!(total,37911);assert_eq!(categories.values().sum::<u64>(),total);
 let mut cats=categories.iter().map(|(name,count)|json!({"name":name,"samples":count,"share":*count as f64/total as f64})).collect::<Vec<_>>();cats.sort_by_key(|r|std::cmp::Reverse(r["samples"].as_u64().unwrap()));
 let comparison=read(format!("{S}/independent-main-review.json"));assert_eq!(comparison["passed"],true);
 let goals=comparison["M5_comparisons"].as_array().unwrap().iter().filter(|r|r["arm"]=="REF").map(|r|{let ms=r["ms"].as_f64().unwrap();let target=r["CPU10x_limit_ms"].as_f64().unwrap();json!({"shape":r["shape"],"accepted_ms":ms,"peak_RSS_MiB":r["peak_RSS_MiB"],"M5_CPU_ms":r["M5_CPU_ms"],"M5_Metal_ms":r["M5_Metal_ms"],"CPU_speedup":r["CPU_speedup"],"Metal_speedup":r["Metal_speedup"],"target_ms":target,"required_ms_removed":ms-target,"required_latency_fraction_removed":1.-target/ms,"additional_speedup_required":ms/target})}).collect::<Vec<_>>();assert_eq!(goals.len(),8);
 let fold=read(format!("{R}/run/measure-fold/stdout.log"));assert_eq!(fold["rows"].as_array().unwrap().len(),6);
 for row in fold["rows"].as_array().unwrap(){assert_eq!(row["generator_scalar_oracle_passed"],true);let mut samples=row["nanoseconds_per_fold"].as_array().unwrap().iter().map(|v|v.as_f64().unwrap()).collect::<Vec<_>>();assert_eq!(samples.len(),5);assert!(samples.iter().all(|x|x.is_finite()&&*x>0.));samples.sort_by(f64::total_cmp);assert_eq!(samples[2],row["median_ns"].as_f64().unwrap());}
 let ledger=format!("{P}/proof-sha256-after-profile.json");assert_eq!(read(&ledger).as_array().unwrap().len(),53917);assert_eq!(hash(&ledger),"a033132ee87cf5d5260c396acf1894bbb1afc95c07d8ba22c55ed9c8e5a85873");
 let result=json!({"source_bindings":source_bindings(),"all_eight_goals":goals,"profile_source":"fresh OFF consumer, same release arithmetic; not exact accepted binary","OFF_profile_samples":total,"OFF_profile_categories":cats,"symbol_counts":symbols,"hypothetical_half_streaming_CPU_sample_fraction_saved":0.5*(categories["G1 streaming"]+categories["G2 streaming"])as f64/total as f64,"CPU_proxy_is_not_latency_prediction":true,"fold_probe":fold,"probe_sha256":hash(format!("{R}/fold")),"canonical_count":53917,"new_proofs":0,"new_SDK_calls":0,"adopted":false,"goal_complete":false});
 save("result.json",result);println!("Source/model binding, profile conservation, all-eight target gaps and six fold results checked; ledger unchanged.");
}
fn main(){match std::env::args().nth(1).as_deref(){Some("prepare")=>prepare(),Some("finish")=>finish(),_=>panic!("prepare/finish")}}
