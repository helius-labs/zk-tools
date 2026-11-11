use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/top-elision177";
const Z:&str="/home/ubuntu/src/zk-tools/zopro";
const PREV:&str="/home/ubuntu/src/zopro-perf-work/packed-ternary176";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn save(n:&str,v:Value){fs::write(format!("{R}/{n}"),serde_json::to_vec_pretty(&v).unwrap()).unwrap()}
fn replace(s:&mut String,a:&str,b:&str){assert_eq!(s.matches(a).count(),1,"{a}");*s=s.replacen(a,b,1)}
fn files(root:&Path,out:&mut Vec<String>){for e in fs::read_dir(root).unwrap(){let e=e.unwrap();assert!(!e.file_type().unwrap().is_symlink());if e.file_type().unwrap().is_dir(){files(&e.path(),out)}else{out.push(e.path().display().to_string());}}out.sort();}
fn pin(paths:Vec<String>)->Value{json!(paths.into_iter().map(|p|{let h=hash(&p);(p,h)}).collect::<BTreeMap<_,_>>())}
fn check_pins(n:&str){for(p,h)in read(format!("{R}/{n}")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap(),"{p}");}}
fn command(name:&str,argv:Vec<String>,env:Value)->Value{json!({"kind":"command","name":name,"argv":argv,"environment":env,"timeout_seconds":1800})}
fn prepare(){
 let mut paths=vec![];files(Path::new(&format!("{R}/source")),&mut paths);let mut originals=BTreeMap::new();
 for p in &paths{let relative=p.strip_prefix(&format!("{R}/source/")).unwrap();let current=format!("{Z}/{relative}");if Path::new(&current).is_file(){fs::copy(&current,p).unwrap();originals.insert(relative.to_string(),hash(&current));}}
 save("production-bindings.json",json!(originals));
 let p=format!("{R}/source/crates/zopro-core/src/msm_affine/g2_phi.rs");let mut s=fs::read_to_string(&p).unwrap();
 replace(&mut s,"fn hb_window(bases: &[G2Affine], short: &[[u64; 4]], w: usize, bits: usize)","fn hb_window(bases: &[G2Affine], short: &[[u64; 4]], w: usize, bits: usize, part: usize)");
 replace(&mut s,"hb_top_component(bases, short, true, w, bits, top);","super::top_elision::call(part * 2, || hb_top_component(bases, short, true, w, bits, top));");
 replace(&mut s,"hb_top_component(bases, short, false, w, bits, top);","super::top_elision::call(part * 2 + 1, || hb_top_component(bases, short, false, w, bits, top));");
 replace(&mut s,"hb_window(&bases[start..end], &short[start..end], w, bits)","hb_window(&bases[start..end], &short[start..end], w, bits, part)");fs::write(&p,s).unwrap();
 fs::copy(format!("{R}/top_elision.rs"),format!("{R}/source/crates/zopro-core/src/msm_affine/top_elision.rs")).unwrap();
 let p=format!("{R}/source/crates/zopro-core/src/msm_affine.rs");let mut s=fs::read_to_string(&p).unwrap();s.push_str("\npub(crate) mod top_elision;\n");fs::write(p,s).unwrap();
 let p=format!("{R}/source/crates/zopro-core/src/lib.rs");let mut s=fs::read_to_string(&p).unwrap();s.push_str("\n/// Private fixed-input diagnostic API; never a production optimization.\npub fn top_probe(bases: &[G2Affine], words: &mut [[u64;4]], ternary: bool) -> G2Affine { assert!(!ternary); msm_affine::g2_query_exclusive(bases, words).to_affine() }\npub fn top_probe_status() -> (usize,usize,usize,usize) { msm_affine::top_elision::status() }\n");fs::write(p,s).unwrap();
 let p=format!("{R}/driver/src/main.rs");let mut s=fs::read_to_string(&p).unwrap();replace(&mut s,"zopro_core::ternary_g2_probe","zopro_core::top_probe");
 replace(&mut s,"let mut outputs=Vec::with_capacity(rounds);","let elide=std::env::var(\"ZOPRO_DIAGNOSTIC_ELIDE_TOP\").unwrap()==\"1\";\n let initial=zopro_core::top_probe_status();assert_eq!(initial,(12,if elide{4}else{12},4,32768));\n let mut outputs=Vec::with_capacity(rounds);");
 replace(&mut s,"let meta=expected_metadata.unwrap();","let status=zopro_core::top_probe_status();assert_eq!(status,(4*(rounds+4),if elide{4}else{4*(rounds+4)},4,32768));\n let meta=expected_metadata.unwrap();");
 replace(&mut s,"\"ternary\":ternary","\"ternary\":ternary,\"elide\":elide,\"top_status\":status,\"scope\":\"fixed-input counterfactual, not a production candidate\"");fs::write(p,s).unwrap();
 let ctl=read("/home/ubuntu/src/zopro-perf-work/g2-coordinate-leaf177/control-plan.json");let mut env=ctl["steps"][0]["environment"].clone();env["CARGO_TARGET_DIR"]=format!("{R}/target").into();
 let argv=["cargo","build","--locked","--offline","--release","--manifest-path",&format!("{R}/driver/Cargo.toml"),"--message-format=json"].map(str::to_owned).to_vec();
 let mut paths=vec![];files(Path::new(&format!("{R}/source")),&mut paths);files(Path::new(&format!("{R}/driver")),&mut paths);paths.extend([format!("{R}/protocol.md"),format!("{R}/manage.rs"),format!("{R}/production-bindings.json")]);save("build-pins.json",pin(paths));save("build-plan.json",json!({"version":1,"source_pins":format!("{R}/build-pins.json"),"steps":[command("build",argv,env)]}));
 println!("Private diagnostic prepared; current sources bound; original top body retained.");
}
fn capture(){
 check_pins("build-pins.json");assert_eq!(read(format!("{R}/build-run/result.json"))["status"],"complete");
 let binary=format!("{R}/target/release/zopro-ternary-g2-caller");let mut paths=vec![binary.clone(),format!("{R}/protocol.md"),format!("{R}/manage.rs")];
 fs::create_dir_all(format!("{R}/bin")).unwrap();for copy in 0..3{let p=format!("{R}/bin/copy{copy}");fs::copy(&binary,&p).unwrap();assert_eq!(hash(&p),hash(&binary));paths.push(p);}
 let cases=read(format!("{PREV}/cases.json"));for c in cases.as_array().unwrap(){paths.push(c["fixture"].as_str().unwrap().into());paths.push(c["oracle"].as_str().unwrap().into());}
 let mut env=read(format!("{PREV}/environment.json"));env["RUSTUP_TOOLCHAIN"]="1.98.1-x86_64-unknown-linux-gnu".into();save("environment.json",env.clone());paths.push(format!("{R}/environment.json"));save("capture-pins.json",pin(paths));
 let mut main=vec![];let mut pre=vec![];let mut jobs=vec![];
 for block in 0..3{let mut order=(0..6usize).map(|i|(i+block)%6).collect::<Vec<_>>();if block%2==1{order.reverse();}
  for cidx in order{let c=&cases[cidx];for j in 0..2{let mode=(j+cidx+block)%2;let name=format!("b{block}-{}-m{mode}",c["name"].as_str().unwrap());let mut e=env.clone();e["ZOPRO_DIAGNOSTIC_ELIDE_TOP"]=mode.to_string().into();let workers=c["workers"].as_u64().unwrap();let argv=vec!["/usr/bin/taskset".into(),"-c".into(),if workers==1{"2".into()}else{format!("0-{}",workers-1)},format!("{R}/bin/copy{block}"),"B".into(),c["fixture"].as_str().unwrap().into(),c["oracle"].as_str().unwrap().into(),workers.to_string(),"17".into()];
   main.push(command(&name,argv.clone(),e.clone()));jobs.push(json!({"name":name,"block":block,"mode":mode,"case":c["name"],"workers":workers,"shape":c["shape"]}));
   if block==0{let mut a=argv;a.last_mut().map(|x|*x="1".into());pre.push(command(&name,a,e));}
  }}
 }
 save("jobs.json",json!(jobs));for(n,steps)in[("preflight",pre),("main",main)]{save(&format!("{n}-plan.json"),json!({"version":1,"source_pins":format!("{R}/capture-pins.json"),"steps":steps}));}println!("Frozen 12 preflight +36 main processes, three independent executable copies.");
}
fn rows(kind:&str)->Vec<Value>{
 check_pins("capture-pins.json");let root=format!("{R}/{kind}-run");assert_eq!(read(format!("{root}/result.json"))["status"],"complete");let plan=read(format!("{R}/{kind}-plan.json"));let mut out=vec![];
 for step in plan["steps"].as_array().unwrap(){let name=step["name"].as_str().unwrap();let dir=format!("{root}/{name}");let e=read(format!("{dir}/exit.json"));assert_eq!(e["success"],true);assert_eq!(e["returncode"],0);for stream in["stdout","stderr"]{assert_eq!(hash(format!("{dir}/{stream}.log")),e[format!("{stream}_sha256")]);}let c=read(format!("{dir}/command.json"));assert_eq!(&c["argv"].as_array().unwrap()[4..],step["argv"].as_array().unwrap());
  let text=fs::read_to_string(format!("{dir}/stdout.log")).unwrap();let mut row:Value=serde_json::from_str(text.trim().strip_prefix("CALLER ").unwrap()).unwrap();assert_eq!(row["every_output_matches_independent_full_MSM"],true);assert_eq!(row["point_inputs_unchanged"],true);let rounds=row["rounds"].as_u64().unwrap();assert_eq!(rounds,if kind=="preflight"{1}else{17});let elide=row["elide"].as_bool().unwrap();assert_eq!(row["top_status"],json!([4*(rounds+4),if elide{4}else{4*(rounds+4)},4,32768]));let samples=row["samples_ns"].as_array().unwrap();assert_eq!(samples.len(),rounds as usize);row["sample_mean_ns"]=json!(samples.iter().map(|x|x.as_u64().unwrap()as f64).sum::<f64>()/rounds as f64);row["name"]=name.into();out.push(row);
 }out
}
fn finish(kind:&str){
 let rows=rows(kind);save(&format!("{kind}-rows.json"),json!(rows));if kind=="preflight"{assert_eq!(rows.len(),12);println!("All 12 preflights: independent query oracle, immutable inputs and exact elision coverage pass.");return}
 assert_eq!(rows.len(),36);let jobs=read(format!("{R}/jobs.json"));let mut summary=vec![];
 for case in read(format!("{PREV}/cases.json")).as_array().unwrap(){let mut pairs=vec![];for block in 0..3{let get=|mode|{let job=jobs.as_array().unwrap().iter().find(|j|j["case"]==case["name"]&&j["block"]==block&&j["mode"]==mode).unwrap();rows.iter().find(|r|r["name"]==job["name"]).unwrap()};let off=get(0);let on=get(1);let a=off["sample_mean_ns"].as_f64().unwrap();let b=on["sample_mean_ns"].as_f64().unwrap();pairs.push(json!({"block":block,"OFF_ms":a/1e6,"ELIDE_ms":b/1e6,"ratio":b/a,"saved_ms":(a-b)/1e6,"OFF_usage":off["usage"],"ELIDE_usage":on["usage"]}));}summary.push(json!({"case":case["name"],"pairs":pairs}));}
 save("result.json",json!({"rows":summary,"new_proofs":0,"SDK_calls":0,"canonical_count":53917,"adopted":false,"goal_complete":false,"scope":"fixed-input query counterfactual; no full-proof bound or production optimization","checked_queries":12*5+36*21}));println!("All 36 main processes checked; 816 total oracle-checked queries; no proof exports.");
}
fn main(){match std::env::args().nth(1).as_deref(){Some("prepare")=>prepare(),Some("capture")=>capture(),Some("preflight")=>finish("preflight"),Some("finish")=>finish("main"),_=>panic!("prepare/capture/preflight/finish")}}
