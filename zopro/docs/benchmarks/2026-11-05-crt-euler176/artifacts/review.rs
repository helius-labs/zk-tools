use num_bigint::{BigInt,BigUint,Sign};use num_traits::{One,Zero,ToPrimitive};
use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,path::Path,collections::BTreeMap};
const R:&str="/home/ubuntu/src/zopro-perf-work/crt-euler176";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn euclid_inverse(a:u64,p:u64)->u64{let(mut r0,mut r1,mut s0,mut s1)=(p as i64,a as i64,0i64,1i64);while r1!=0{let q=r0/r1;(r0,r1)=(r1,r0-q*r1);(s0,s1)=(s1,s0-q*s1);}assert_eq!(r0,1);s0.rem_euclid(p as i64)as u64}
fn main(){
 let pins=read(format!("{R}/pins-fixed.json"));for(p,h)in pins.as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
 let original=read(format!("{R}/plan.json"));let mut fixed=read(format!("{R}/plan-fixed.json"));
 fixed["source_pins"]=original["source_pins"].clone();for(step,old)in fixed["steps"].as_array_mut().unwrap().iter_mut().zip(original["steps"].as_array().unwrap()){for key in["RUSTUP_TOOLCHAIN","RUSTC","RUSTDOC"]{step["environment"].as_object_mut().unwrap().remove(key);}if step["name"]!="full-domain-algebra"{step["argv"][0]=old["argv"][0].clone();}}assert_eq!(fixed,original);
 let initial=read(format!("{R}/runs/algebra-tests/exit.json"));assert_eq!(initial["success"],false);assert_eq!(initial["returncode"],101);assert!(fs::read_to_string(format!("{R}/runs/algebra-tests/stderr.log")).unwrap().contains("failed to install component"));assert!(fs::read(format!("{R}/runs/algebra-tests/stdout.log")).unwrap().is_empty());
 let result=read(format!("{R}/runs-fixed/result.json"));assert_eq!(result["status"],"complete");assert_eq!(result["steps"].as_array().unwrap().len(),3);
 for s in["algebra-tests","build-full-domain","full-domain-algebra"]{let exit=read(format!("{R}/runs-fixed/{s}/exit.json"));assert_eq!(exit["success"],true);assert_eq!(exit["returncode"],0);for stream in["stdout","stderr"]{assert_eq!(hash(format!("{R}/runs-fixed/{s}/{stream}.log")),exit[format!("{stream}_sha256")].as_str().unwrap());}}
 let tests=read(format!("{R}/runs-fixed/algebra-tests/tests.json"));assert_eq!(tests["passed_tests"].as_array().unwrap().len(),3);assert_eq!(tests["unfiltered"],true);
 let a=read(format!("{R}/algebra.json"));assert_eq!(read(format!("{R}/runs-fixed/full-domain-algebra/stdout.log")),a);assert_eq!(a["domain"],32768);assert_eq!(a["full_operator_coordinates_checked"],32768);
 let qs=a["auxiliary_primes"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()).collect::<Vec<_>>();assert_eq!(qs.len(),17);let mut sorted=qs.clone();sorted.sort_unstable();sorted.dedup();assert_eq!(qs.len(),sorted.len());
 // Independent deterministic trial division, rather than the model's Miller-Rabin.
 for &q in&qs{assert_eq!((q-1)%65536,0);assert!(q<1u64<<31&&q>1u64<<30);let mut d=3;while d*d<=q{assert_ne!(q%d,0);d+=2;}}
 let m=qs.iter().fold(BigUint::one(),|m,&q|m*q);let r=BigUint::parse_bytes(b"21888242871839275222246405745257275088548364400416034343698204186575808495617",10).unwrap();let c=(&r-BigUint::one()).pow(2)*32768u32;
 assert!(m> &c*16u32);assert!(&m/ *qs.last().unwrap()<=&c*2u32);assert_eq!(a["modulus_product_bits"],m.bits());assert_eq!(a["signed_coefficient_bound_bits"],c.bits());
 let coefficient_mi=qs.iter().map(|&q|&m/q).collect::<Vec<_>>();let inverses=coefficient_mi.iter().zip(&qs).map(|(mi,&q)|euclid_inverse((mi%q).to_u64().unwrap(),q)).collect::<Vec<_>>();
 let cb=BigInt::from(c.clone());let mb=BigInt::from(m.clone());let rb=BigInt::from(r.clone());let mut coefficients=vec![BigInt::zero(),BigInt::one(),-BigInt::one(),cb.clone(),-&cb,&cb-1,-&cb+1,rb.clone(),-&rb,&rb-1,-&rb+1];
 let mut seed=BigUint::from(0x4352545f454c4552u64);for i in 0..1024{seed=(&seed*BigUint::from(6364136223846793005u64)+BigUint::from(1442695040888963407u64))%(&c+BigUint::one());let v=BigInt::from(seed.clone());coefficients.push(if i%2==0{v}else{-v});}
 for v in &coefficients{
  let mut exact=BigUint::zero();let mut approx=0f64;let mut modular=BigUint::zero();
  for(i,&q)in qs.iter().enumerate(){let qbig=BigInt::from(q);let res=((v%&qbig+&qbig)%&qbig).to_u64().unwrap();let t=res*inverses[i]%q;exact+=&coefficient_mi[i]*t;modular+=(&coefficient_mi[i]%&r)*t;approx+=t as f64/q as f64;}
  let k=(approx+0.5).floor()as u64;assert_eq!(BigInt::from(exact)-&mb*BigInt::from(k),*v);
  let reconstructed=(BigInt::from(modular)-BigInt::from(&m%&r)*BigInt::from(k))%&rb;let reconstructed=if reconstructed.sign()==Sign::Minus{reconstructed+&rb}else{reconstructed};let expected=(v%&rb+&rb)%&rb;assert_eq!(reconstructed,expected);
 }
 let meta=read(format!("{R}/metadata.json"));let tfhe=meta["packages"].as_array().unwrap().iter().find(|p|p["name"]=="tfhe-ntt").unwrap();assert_eq!(tfhe["version"],"0.7.1");let id=tfhe["id"].as_str().unwrap();let node=meta["resolve"]["nodes"].as_array().unwrap().iter().find(|n|n["id"]==id).unwrap();assert!(node["features"].as_array().unwrap().iter().any(|f|f=="avx512"));
 let source=Path::new(tfhe["manifest_path"].as_str().unwrap()).parent().unwrap();let plan_source=fs::read_to_string(source.join("src/prime32.rs")).unwrap();assert!(plan_source.contains("twid: ABox<[u32]>"));assert!(plan_source.contains("inv_twid_shoup: ABox<[u32]>"));
 let roots=4*32768*17*4;let kernels=32768*17*4;let twists=32768*2*32;assert_eq!(a["public_kernel_transform_u32_payload_bytes"],kernels);assert_eq!(a["Fr_twist_and_restore_payload_bytes"],twists);
 let mut artifact_hashes=BTreeMap::new();for f in["target/release/zopro-crt-euler-study","Cargo.toml","Cargo.lock","src/lib.rs","src/main.rs","next-math.md","review.rs"]{artifact_hashes.insert(f,hash(format!("{R}/{f}")));}
 let ledger=read("/home/ubuntu/src/zopro-perf-work/finalize-drain-bundle176/proof-sha256-after.json");assert_eq!(ledger.as_array().unwrap().len(),29953);
 let out=json!({"passed":true,"pins":pins.as_object().unwrap().len(),"successful_commands":3,"failed_build_commands_before_tests":1,"failed_fetch_before_target_creation":1,"new_tests":3,"full_domain_outputs":32768,"independent_trial_division_primes":17,"independent_CRT_rounding_formula_examples":coefficients.len(),"numerical_examples_are_not_a_universal_rounding_error_proof":true,"Q_greater_than_16_times_coefficient_bound":true,"production_originals_exact":176,"domain_context_array_payloads":{"plan_four_root_arrays":roots,"kernel_transform_arrays":kernels,"Fr_twist_restore_arrays":twists,"sum":roots+kernels+twists,"excludes_ownership_headers_alignment_BigUint_and_query_scratch":true},"tfhe_ntt_version":"0.7.1","tfhe_ntt_vcs":read(source.join(".cargo_vcs_info.json")),"tfhe_ntt_license":tfhe["license"],"AVX512_feature_compiled_not_dynamic_instruction_count":true,"artifact_hashes":artifact_hashes,"new_proofs":0,"proof_ledger":29953,"adopted":false,"goal_achieved":false});
 fs::write(format!("{R}/review.json"),serde_json::to_vec_pretty(&out).unwrap()).unwrap();println!("ReviewPASS {}pins/3successful commands/1pretest buildfailure/3tests/32768outputs/{}CRT examples;public array payload {}B nottotalRAM",pins.as_object().unwrap().len(),coefficients.len(),roots+kernels+twists);
}
