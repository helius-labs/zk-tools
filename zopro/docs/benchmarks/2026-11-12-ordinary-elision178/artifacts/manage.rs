use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/ordinary-elision178";
const Z:&str="/home/ubuntu/src/zk-tools/zopro";
const OLD:&str="/home/ubuntu/src/zopro-perf-work/top-elision177";
const MAN:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-12-bundle-adoption178/artifacts/production-source-manifest.json";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn save(n:&str,v:Value){fs::write(format!("{R}/{n}"),serde_json::to_vec_pretty(&v).unwrap()).unwrap()}
fn replace(s:&mut String,a:&str,b:&str){assert_eq!(s.matches(a).count(),1,"{a}");*s=s.replacen(a,b,1)}
fn files(root:&Path,out:&mut Vec<String>){for e in fs::read_dir(root).unwrap(){let e=e.unwrap();assert!(!e.file_type().unwrap().is_symlink());if e.file_type().unwrap().is_dir(){files(&e.path(),out)}else{out.push(e.path().display().to_string());}}out.sort();}
fn pin(paths:Vec<String>)->Value{json!(paths.into_iter().map(|p|{let h=hash(&p);(p,h)}).collect::<BTreeMap<_,_>>())}
fn check_pins(n:&str){for(p,h)in read(format!("{R}/{n}")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap(),"{p}");}}
fn command(name:&str,argv:Vec<String>,env:Value)->Value{json!({"kind":"command","name":name,"argv":argv,"environment":env,"timeout_seconds":1800})}
fn prepare(){
 assert!(!Path::new(&format!("{R}/build-plan.json")).exists());
 let manifest=read(MAN);let original=manifest["source_files_sha256"].as_object().unwrap();assert_eq!(original.len(),177);
 for(rel,h)in original{let src=format!("{Z}/{rel}");assert_eq!(hash(&src),h.as_str().unwrap());let dest=format!("{R}/source/{rel}");fs::create_dir_all(Path::new(&dest).parent().unwrap()).unwrap();fs::copy(src,dest).unwrap();}
 save("production-bindings.json",json!(original));
 let p=format!("{R}/source/crates/zopro-core/src/msm_affine/g2_phi.rs");let mut s=fs::read_to_string(&p).unwrap();
 let mut edits=vec![];
 for(a,b)in[
  ("    let windows = 127usize.div_ceil(bits);", "    let windows = 127usize.div_ceil(bits);\n    assert_eq!((bits,parts,windows),(12,2,11), \"diagnostic fixture geometry\");"),
  ("                    hb_window(&bases[start..end], &short[start..end], w, bits)","                    if w < 10 {\n                        super::phase_elision::fill(w, part, || hb_window(&bases[start..end], &short[start..end], w, bits))\n                    } else {\n                        hb_window(&bases[start..end], &short[start..end], w, bits)\n                    }"),
  ("            let mut combined = buckets.swap_remove(0);\n            for b in buckets {\n                merge_buckets(&mut combined, &b);\n            }\n            crate::msm::weighted_affine(&combined)","            let reduce = || {\n                let mut combined = buckets.swap_remove(0);\n                for b in buckets { merge_buckets(&mut combined, &b); }\n                crate::msm::weighted_affine(&combined)\n            };\n            if w < 10 { super::phase_elision::reduce(w, reduce) } else { reduce() }")
 ]{replace(&mut s,a,b);edits.push(json!({"before":a,"after":b}));}
 fs::write(&p,s).unwrap();save("edits.json",json!(edits));
 fs::copy(format!("{R}/phase_elision.rs"),format!("{R}/source/crates/zopro-core/src/msm_affine/phase_elision.rs")).unwrap();
 let p=format!("{R}/source/crates/zopro-core/src/msm_affine.rs");let mut s=fs::read_to_string(&p).unwrap();s.push_str("\npub(crate) mod phase_elision;\n");fs::write(p,s).unwrap();
 let p=format!("{R}/source/crates/zopro-core/src/lib.rs");let mut s=fs::read_to_string(&p).unwrap();s.push_str("\n/// Private immutable-fixture diagnostic, never a production API.\npub fn phase_probe(bases: &[G2Affine], words: &mut [[u64;4]], ternary: bool) -> G2Affine { assert!(!ternary); msm_affine::g2_query_exclusive(bases, words).to_affine() }\npub fn phase_probe_check(calls: usize) -> [usize;7] { msm_affine::phase_elision::check(calls) }\n");fs::write(p,s).unwrap();
 fs::create_dir_all(format!("{R}/driver/src")).unwrap();for f in["Cargo.toml","Cargo.lock","src/perf_support.rs"]{fs::copy(format!("{OLD}/driver/{f}"),format!("{R}/driver/{f}")).unwrap();}
 let mut s=fs::read_to_string(format!("{OLD}/driver/src/main.rs")).unwrap();
 replace(&mut s,"zopro_core::top_probe(","zopro_core::phase_probe(");
 replace(&mut s,"let elide=std::env::var(\"ZOPRO_DIAGNOSTIC_ELIDE_TOP\").unwrap()==\"1\";\n let initial=zopro_core::top_probe_status();assert_eq!(initial,(12,if elide{4}else{12},4,32768));", "let mode=std::env::var(\"ZOPRO_DIAGNOSTIC_ORDINARY_MODE\").unwrap().parse::<usize>().unwrap();\n zopro_core::phase_probe_check(3);");
 replace(&mut s,"let status=zopro_core::top_probe_status();assert_eq!(status,(4*(rounds+4),if elide{4}else{4*(rounds+4)},4,32768));", "let status=zopro_core::phase_probe_check(rounds+4);");
 replace(&mut s,"\"elide\":elide,\"top_status\":status", "\"mode\":mode,\"phase_status\":status");
 fs::write(format!("{R}/driver/src/main.rs"),s).unwrap();
 let mut env=read(format!("{OLD}/build-plan.json"))["steps"][0]["environment"].clone();env["CARGO_TARGET_DIR"]=format!("{R}/target").into();
 let argv=["cargo","build","--locked","--offline","--release","--manifest-path",&format!("{R}/driver/Cargo.toml"),"--message-format=json"].map(str::to_owned).to_vec();
 let mut paths=vec![];files(Path::new(&format!("{R}/source")),&mut paths);files(Path::new(&format!("{R}/driver")),&mut paths);paths.extend([MAN.into(),format!("{R}/protocol.md"),format!("{R}/manage.rs"),format!("{R}/production-bindings.json"),format!("{R}/edits.json")]);save("build-pins.json",pin(paths));save("build-plan.json",json!({"version":1,"source_pins":format!("{R}/build-pins.json"),"steps":[command("build",argv,env)]}));
 println!("177 current production files verified and copied; three diagnostic parents, one child; four fixed arms.");
}
fn capture(){
 check_pins("build-pins.json");assert_eq!(read(format!("{R}/build-run/result.json"))["status"],"complete");
 let binary=format!("{R}/target/release/zopro-ternary-g2-caller");let mut paths=vec![binary.clone(),format!("{R}/protocol.md"),format!("{R}/manage.rs")];
 fs::create_dir_all(format!("{R}/bin")).unwrap();for copy in 0..3{let p=format!("{R}/bin/copy{copy}");fs::copy(&binary,&p).unwrap();assert_eq!(hash(&p),hash(&binary));paths.push(p);}
 let cases=read("/home/ubuntu/src/zopro-perf-work/packed-ternary176/cases.json");save("cases.json",cases.clone());paths.push(format!("{R}/cases.json"));for c in cases.as_array().unwrap(){paths.push(c["fixture"].as_str().unwrap().into());paths.push(c["oracle"].as_str().unwrap().into());}
 let mut env=read(format!("{OLD}/environment.json"));env["ZOPRO_CPU_INDEXED_TILE_BUNDLES"]="1".into();save("environment.json",env.clone());paths.push(format!("{R}/environment.json"));
 let mut main=vec![];let mut pre=vec![];let mut jobs=vec![];
 for block in 0..3{let mut order=(0..6usize).map(|i|(i+block)%6).collect::<Vec<_>>();if block%2==1{order.reverse();}
  for cidx in order{let c=&cases[cidx];let mut modes=(0..4).map(|i|(i+cidx+block)%4).collect::<Vec<_>>();if block%2==1{modes.reverse();}
   for mode in modes{let name=format!("b{block}-{}-m{mode}",c["name"].as_str().unwrap());let mut e=env.clone();e["ZOPRO_DIAGNOSTIC_ORDINARY_MODE"]=mode.to_string().into();let workers=c["workers"].as_u64().unwrap();let argv=vec!["/usr/bin/taskset".into(),"-c".into(),if workers==1{"2".into()}else{format!("0-{}",workers-1)},format!("{R}/bin/copy{block}"),"B".into(),c["fixture"].as_str().unwrap().into(),c["oracle"].as_str().unwrap().into(),workers.to_string(),"17".into()];
    main.push(command(&name,argv.clone(),e.clone()));jobs.push(json!({"name":name,"block":block,"mode":mode,"case":c["name"],"workers":workers,"shape":c["shape"]}));
    if block==0{let mut a=argv;a.last_mut().map(|x|*x="1".into());pre.push(command(&name,a,e));}
  }}
 }
 save("jobs.json",json!(jobs));paths.push(format!("{R}/jobs.json"));save("capture-pins.json",pin(paths));
 for(n,steps)in[("preflight",pre),("main",main)]{save(&format!("{n}-plan.json"),json!({"version":1,"source_pins":format!("{R}/capture-pins.json"),"steps":steps}));}println!("Frozen 24 preflight +72 main processes; four modes, three executable copies.");
}
fn rows(kind:&str)->Vec<Value>{
 check_pins("capture-pins.json");let root=format!("{R}/{kind}-run");assert_eq!(read(format!("{root}/result.json"))["status"],"complete");let plan=read(format!("{R}/{kind}-plan.json"));let mut out=vec![];
 for step in plan["steps"].as_array().unwrap(){let name=step["name"].as_str().unwrap();let dir=format!("{root}/{name}");let e=read(format!("{dir}/exit.json"));assert_eq!(e["success"],true);assert_eq!(e["returncode"],0);for stream in["stdout","stderr"]{assert_eq!(hash(format!("{dir}/{stream}.log")),e[format!("{stream}_sha256")]);}let c=read(format!("{dir}/command.json"));assert_eq!(&c["argv"].as_array().unwrap()[4..],step["argv"].as_array().unwrap());
  let text=fs::read_to_string(format!("{dir}/stdout.log")).unwrap();let mut row:Value=serde_json::from_str(text.trim().strip_prefix("CALLER ").unwrap()).unwrap();assert_eq!(row["every_output_matches_independent_full_MSM"],true);assert_eq!(row["point_inputs_unchanged"],true);let rounds=row["rounds"].as_u64().unwrap();assert_eq!(rounds,if kind=="preflight"{1}else{17});let mode=row["mode"].as_u64().unwrap();assert_eq!(mode.to_string(),step["environment"]["ZOPRO_DIAGNOSTIC_ORDINARY_MODE"]);assert_eq!(row["phase_status"],json!([20*(rounds+4),if mode&1!=0{20}else{20*(rounds+4)},10*(rounds+4),if mode&2!=0{10}else{10*(rounds+4)},20,10,5244800]));let samples=row["samples_ns"].as_array().unwrap();assert_eq!(samples.len(),rounds as usize);row["sample_mean_ns"]=json!(samples.iter().map(|x|x.as_u64().unwrap()as f64).sum::<f64>()/rounds as f64);row["name"]=name.into();out.push(row);
 }out
}
fn finish(kind:&str){
 let rows=rows(kind);save(&format!("{kind}-rows.json"),json!(rows));if kind=="preflight"{assert_eq!(rows.len(),24);println!("All 24 preflights: independent MSM oracle, input immutability and exact stage-elision counts pass.");return}
 assert_eq!(rows.len(),72);let jobs=read(format!("{R}/jobs.json"));let mut summary=vec![];
 for case in read(format!("{R}/cases.json")).as_array().unwrap(){let mut blocks=vec![];for block in 0..3{let get=|mode|{let job=jobs.as_array().unwrap().iter().find(|j|j["case"]==case["name"]&&j["block"]==block&&j["mode"]==mode).unwrap();rows.iter().find(|r|r["name"]==job["name"]).unwrap()};let ms=(0..4).map(|m|get(m)["sample_mean_ns"].as_f64().unwrap()/1e6).collect::<Vec<_>>();let usage=(0..4).map(|m|get(m)["usage"].clone()).collect::<Vec<_>>();blocks.push(json!({"block":block,"ms":ms,"usage":usage,"fill_elision_ratio":ms[1]/ms[0],"reduction_elision_ratio":ms[2]/ms[0],"both_elision_ratio":ms[3]/ms[0],"nonadditivity_ms":ms[0]-ms[1]-ms[2]+ms[3]}));}summary.push(json!({"case":case["name"],"blocks":blocks}));}
 save("result.json",json!({"rows":summary,"new_proofs":0,"SDK_calls":0,"canonical_count":54927,"adopted":false,"goal_complete":false,"scope":"fixed-input query diagnostic; no full-proof bound or production speedup","checked_queries":24*5+72*21}));println!("72 main processes checked; 1632 complete queries match independent MSM oracle.");
}
fn main(){match std::env::args().nth(1).as_deref(){Some("prepare")=>prepare(),Some("capture")=>capture(),Some("preflight")=>finish("preflight"),Some("finish")=>finish("main"),_=>panic!("prepare/capture/preflight/finish")}}
