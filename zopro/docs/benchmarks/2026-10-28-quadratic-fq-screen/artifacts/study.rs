mod model;
use model::{coupled_sites,final_high_survives,modulo,Pair,Params};
use num_bigint::{BigInt,Sign};
use num_traits::{One,Signed,ToPrimitive,Zero};
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs,io::Read,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/quadratic-fq-screen";
const Z:&str="/home/ubuntu/src/zk-tools/zopro";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{let mut f=fs::File::open(p).unwrap();let mut h=Sha256::new();let mut b=[0;65536];loop{let n=f.read(&mut b).unwrap();if n==0{break}h.update(&b[..n]);}format!("{:x}",h.finalize())}
fn pair(z:&Pair)->Value{json!([z.0.to_string(),z.1.to_string()])}
fn next(seed:&mut u64)->BigInt{let mut bytes=[0u8;32];for c in bytes.chunks_mut(8){*seed^=*seed<<13;*seed^=*seed>>7;*seed^=*seed<<17;c.copy_from_slice(&seed.to_le_bytes());}BigInt::from_bytes_le(Sign::Plus,&bytes)}
fn check(p:&Params,a:&Pair,b:&Pair)->Pair{
 let product=p.mul_three(a,b);assert_eq!(product,p.mul_direct(a,b));
 let result=p.redc(&product);
 assert_eq!(p.decode(&result),modulo(p.decode(a)*p.decode(b),&p.p));
 let bounds=p.bounds();assert!(result.0.abs()<=bounds.0&&result.1.abs()<=bounds.1);
 result
}
fn main(){let r=Path::new(R);assert!(!r.join("analysis.json").exists());let m=read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json"));let mut pins=BTreeMap::new();
 for(f,h)in m["source_files_sha256"].as_object().unwrap(){let p=Path::new(Z).join(f);assert_eq!(hash(&p),h.as_str().unwrap());pins.insert(p.display().to_string(),h.as_str().unwrap().to_owned());}assert_eq!(pins.len(),172);
 let base="/home/ubuntu/src/zopro-perf-work/gaussian-fr-cios/target/release/deps";
 for p in [format!("{R}/model.rs"),format!("{R}/study.rs"),format!("{R}/study"),format!("{R}/design.md"),format!("{R}/tests.log"),format!("{base}/libnum_bigint-6bc1a1b0ca7a2b87.rlib"),format!("{base}/libnum_traits-62d94157db5bd7b9.rlib"),format!("{base}/libserde_json-23322b031c2a5860.rlib"),format!("{base}/libsha2-fd72bf385759a87c.rlib"),"/home/ubuntu/src/zopro-perf-work/gaussian-fr-cios/src/model130.rs".into(),"/home/ubuntu/src/zopro-perf-work/gaussian-fr-cios/src/native/cios.rs".into(),"/home/ubuntu/src/zopro-perf-work/gaussian-fr-cios/analysis.json".into()]{pins.insert(p.clone(),hash(p));}
 assert!(fs::read_to_string(r.join("tests.log")).unwrap().contains("test result: ok. 3 passed; 0 failed"));save(r.join("pins.json"),&json!(pins));
 let u=BigInt::from(4965661367192848881u64);let a=6u32*&u*&u+3u32*&u+1u32;
 let prime=BigInt::parse_bytes(b"21888242871839275222246405745257275088696311157297823662689037894645226208583",10).unwrap();
 assert_eq!(36u32*u.pow(4)+36u32*u.pow(3)+24u32*&u*&u+6u32*&u+1u32,prime);
 let definitions=[("sqrt-minus-three",3,0,Pair::new(a.clone(),u.clone())),("eisenstein",1,1,Pair::new(&a+&u,2u32*&u))];
 let mut rows=Vec::new();let mut suites=Vec::new();
 for (name,d,e,pi)in definitions{
  for bits in [128usize,130,131,132]{let p=Params::new(d,e,pi.clone(),prime.clone(),bits);let b=p.bounds();let c=coupled_sites(bits,2);assert!((p.pi.1.clone()>>104usize).is_zero());
   rows.push(json!({"basis":name,"radix_bits":bits,"d":d,"e":e,"pi":pair(&p.pi),"H":p.h.to_string(),"norm":p.p.to_string(),"root":p.root.to_string(),"negative_inverse_mod_radix":pair(&p.neg_inv),"box_redc_bound":pair(&b),"sufficient_box_closure":b.0<=p.h&&b.1<=p.h,"final_high_coefficient_survives":final_high_survives(bits),"raw_IFMA_sites":c.0,"modulus_IFMA_sites":c.1,"low_cancel_IFMA_sites":c.2,"total_IFMA_sites":c.0+c.1+c.2,"coordinate_vectors":6,"coordinates_fit_signed128":p.h.to_i128().is_some()}));
  }
  let p=Params::new(d,e,pi.clone(),prime.clone(),131);let q=Params::new(d,e,pi,prime.clone(),130);let b=p.bounds();assert!(b.0<=p.h&&b.1<=p.h);
  let v=[BigInt::zero(),BigInt::one(),-BigInt::one(),p.h.clone(),-&p.h,&p.h-1u32,-&p.h+1u32];let corners:Vec<Pair>=v.iter().flat_map(|a|v.iter().map(move|b|Pair::new(a.clone(),b.clone()))).collect();let mut count=0u64;let mut first_counterexample=None;let mut failures130=0u64;
  let mut exercise=|a:&Pair,b:&Pair|{assert!(p.in_box(a)&&p.in_box(b));let z=check(&p,a,b);assert!(p.in_box(&z));let z130=check(&q,a,b);if !q.in_box(&z130){failures130+=1;if first_counterexample.is_none(){first_counterexample=Some(json!({"left":pair(a),"right":pair(b),"result":pair(&z130),"H":q.h.to_string()}));}}count+=1;};
  for a in &corners{for b in &corners{exercise(a,b);}}
  let mut seed=0x9e3779b97f4a7c15u64;for _ in 0..4096{let range=2u32*&p.h+1u32;let mut coordinate=||modulo(next(&mut seed),&range)-&p.h;let a=Pair::new(coordinate(),coordinate());let b=Pair::new(coordinate(),coordinate());exercise(&a,&b);}
  let mut x=BigInt::one();let mut z=p.encode(&x);let mut imports=0u64;for _ in 0..2048{let y=modulo(next(&mut seed),&p.p);let t=p.encode(&y);assert!(p.in_box(&t));assert_eq!(p.decode(&t),y);let out=check(&p,&z,&t);assert!(p.in_box(&out));z=p.reduce(&Pair::new(out.0+&t.0,out.1+&t.1));assert!(p.in_box(&z));x=modulo(x*&y+&y,&p.p);assert_eq!(p.decode(&z),x);imports+=1;}
  suites.push(json!({"basis":name,"corner_pairs":corners.len()*corners.len(),"random_box_pairs":4096,"box_pairs_checked_each_radix":count,"radices_checked":[130,131],"R130_outside_box_cases":failures130,"R130_first_counterexample":first_counterexample,"R131_chain_steps":2048,"R131_import_checks":imports,"R131_all_results_within_box":true}));
 }
 for(f,h)in &pins{assert_eq!(hash(f),*h);}
 let report=json!({"status":"complete exact mathematical screen; native trial not justified","basis_definitions":rows,"validation":suites,"ordinary_cios_template":{"variable_low_high_IFMA":50,"cancellation_low_IFMA":5,"modulus_IFMA_with_cancel_carry":45,"total_IFMA_sites":100,"coordinate_vectors":5},"native_trial_justified":false,"reasons":["Uniform box closure needs radix131 in these sufficient bounds; actual R130 counterexamples are recorded if found, with no adaptive extension","Closed radix131 template retains terminal high halves:111 IFMA sites versus100 in current ordinary CIOS","Six coordinate vectors versus five; extra weighting/carry/centering costs are omitted from the site comparison","Three coupled cancellation rounds and a small offdiagonal constant were already present in rejected Gaussian Fr arithmetic; no distinct dependency advantage identified"],"limits":"Counts concern one specified coupled template, not all possible implementations or measured latency. Only the exact model exists; no native conversion/kernel, full curve, prover, memory or portability qualification.","SDK_requests":0,"new_exports":0,"proof_ledger_unchanged":12524,"production_changed":false,"adopted":false,"goal_achieved":false});save(r.join("analysis.json"),&report);println!("Closed two quadratic Fq models; eight parameter rows; no native trial; ledger12524 unchanged");
}
