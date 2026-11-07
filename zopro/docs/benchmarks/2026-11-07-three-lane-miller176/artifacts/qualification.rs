pub use zopro_bn254::{G2Config,CurveConfig};
use zopro_bn254::{Fq,Fq2,Fq6,Fq12,Fr,G1Affine,G2Affine,pairing::{self,G2Prepared}};
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs,path::{Path,PathBuf}};
mod prepared;
mod packed;
const ROOT:&str="/home/ubuntu/src/zopro-perf-work/three-lane-miller176";
const Z:&str="/home/ubuntu/src/zk-tools/zopro";
const FIELD:&str="/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175/frozen/candidate/libzopro_bn254-1200e63547baa41c299a48e3221f1cef85990b0369f17452dc12eb17045f9713.rlib";
fn r(p:&str)->PathBuf{Path::new(ROOT).join(p)}
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn save(p:impl AsRef<Path>,v:&Value){assert!(!p.as_ref().exists());fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn terms(v:Fq12)->[Fq2;6]{[v.c0.c0,v.c1.c0,v.c0.c1,v.c1.c1,v.c0.c2,v.c1.c2]}
fn from_terms(v:[Fq2;6])->Fq12{Fq12::new(Fq6::new(v[0],v[2],v[4]),Fq6::new(v[1],v[3],v[5]))}
fn convolution(a:Fq12,b:Fq12)->Fq12{
 let a=terms(a);let b=terms(b);let mut c=[Fq2::ZERO;11];
 for i in 0..6{for j in 0..6{c[i+j]+=a[i]*b[j];}}
 for i in (6..11).rev(){c[i-6]+=c[i].mul_by_nonresidue();}
 from_terms(std::array::from_fn(|i|c[i]))
}
fn random(seed:&mut u64)->Fq{let mut bytes=[0u8;32];for chunk in bytes.chunks_exact_mut(8){*seed^=*seed<<13;*seed^=*seed>>7;*seed^=*seed<<17;chunk.copy_from_slice(&seed.to_le_bytes());}Fq::from_le_bytes_mod_order(&bytes)}
fn word(v:Fq)->[u64;4]{v.to_canonical()}
fn word12(v:Fq12)->Vec<[u64;4]>{terms(v).iter().flat_map(|v|[word(v.c0),word(v.c1)]).collect()}
fn qualify()->(Value,Value){
 let mut state=0x39bfc71a408de22du64;let mut extension_checks=0;
 for iteration in 0..128{
  let mut value=||from_terms(std::array::from_fn(|_|Fq2::new(random(&mut state),random(&mut state))));
  let mut a:[Fq12;4]=std::array::from_fn(|_|value());let mut b:[Fq12;4]=std::array::from_fn(|_|value());
  if iteration==0{a=[Fq12::new(Fq6::ZERO,Fq6::ZERO);4];}if iteration==1{a=[Fq12::ONE;4];}if iteration==2{a=[-Fq12::ONE;4];b=[-Fq12::ONE;4];}
  packed::check_q(a.map(|v|v.c0.c0),b.map(|v|v.c0.c0));
  let x=packed::T::new(a);let y=packed::T::new(b);assert_eq!(x.get(),a);assert_eq!(y.get(),b);
  assert_eq!(x.mul(y).get(),std::array::from_fn(|i|convolution(a[i],b[i])));
  assert_eq!(x.square().get(),std::array::from_fn(|i|convolution(a[i],a[i])));
  let c0=b.map(|v|v.c0.c0);let c3=b.map(|v|v.c1.c0);let c4=b.map(|v|v.c1.c1);
  assert_eq!(x.sparse(c0,c3,c4).get(),std::array::from_fn(|i|{let line=Fq12::new(Fq6::new(c0[i],Fq2::ZERO,Fq2::ZERO),Fq6::new(c3[i],c4[i],Fq2::ZERO));convolution(a[i],line)}));extension_checks+=12;
 }
 let mut fixtures=vec![];let mut miller_checks=0;
 for i in 0..16usize{
  let mut points:[(G1Affine,G2Affine);3]=std::array::from_fn(|j|{
   let p=(G1Affine::GENERATOR*Fr::from(((i+3)*(j+2)+1)as u64)).to_affine();let q=(G2Affine::GENERATOR*Fr::from(((i+5)*(j+3)+1)as u64)).to_affine();(if (i+j)%3==0{-p}else{p},if(i+j)%4==0{-q}else{q})
  });
  if i==0{points=[(G1Affine::IDENTITY,G2Affine::IDENTITY);3];}
  if i==1||i==2{points=[((G1Affine::GENERATOR*Fr::from(7u64)).to_affine(),(G2Affine::GENERATOR*Fr::from(11u64)).to_affine()),((G1Affine::GENERATOR*Fr::from(if i==1{77u64}else{78u64})).to_affine(),-G2Affine::GENERATOR),(G1Affine::IDENTITY,G2Affine::GENERATOR)];}
  if i>=3&&i%5==0{points[0].0=G1Affine::IDENTITY;}if i>=3&&i%7==0{points[1].1=G2Affine::IDENTITY;}
  let old=points.map(|(_,q)|G2Prepared::new(&q));let new=points.map(|(_,q)|prepared::G2Prepared::new(&q));
  let old_pairs=std::array::from_fn::<_,3,_>(|j|(points[j].0,&old[j]));let new_pairs=std::array::from_fn::<_,3,_>(|j|(points[j].0,&new[j]));
  let expected=pairing::multi_miller_loop(&old_pairs);let independent=old_pairs.iter().map(|p|pairing::multi_miller_loop(&[*p])).fold(Fq12::ONE,|a,b|a*b);assert_eq!(expected,independent);
  let fresh=prepared::multi_miller_loop(&new_pairs);assert_eq!(fresh,expected);let actual=packed::miller(&new_pairs);assert_eq!(actual,expected);
  let final_expected=pairing::final_exponentiation(expected);assert_eq!(pairing::final_exponentiation(actual),final_expected);if i<=1{assert_eq!(final_expected,Some(Fq12::ONE));}if i==2{assert_ne!(final_expected,Some(Fq12::ONE));}
  fixtures.push(json!({"index":i,"points":points.map(|(p,q)|json!({"g1":[word(p.x),word(p.y)],"g2":[word(q.x.c0),word(q.x.c1),word(q.y.c0),word(q.y.c1)]})),"miller":word12(expected),"final":word12(final_expected.unwrap())}));miller_checks+=1;
 }
 (json!({"passed":true,"extension_output_comparisons":extension_checks,"complex_batches":128,"Miller_triples":miller_checks,"independent_extension_oracle":"degree-five Fq2 polynomial convolution reduced by w^6=9+u","Miller_oracles":"accepted shared product; product of accepted single-pair loops; exact copied fresh shared loop","final_exponentiation":"unchanged accepted field implementation","worker_stack_bytes":512*1024,"packed_Fq12_payload_bytes":std::mem::size_of::<packed::T>(),"timings":0,"SDK_requests":0,"new_proofs":0,"production_changed":false,"candidate_qualified_for_component_timing":true}),json!({"format":"synthetic canonical fields; Miller/Fq12 terms in increasing w power, real then imaginary","fixtures":fixtures}))
}
fn prepare(){
 let mut pins=BTreeMap::new();for p in [r("setup.rs"),r("copy-review.txt"),r("prepared.rs"),r("packed.rs"),r("qualification.rs"),r("qualify"),r("design.md"),PathBuf::from(FIELD)]{pins.insert(p.display().to_string(),hash(p));}
 let accepted=read(Path::new(Z).join("docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));
 for name in ["crates/zopro-bn254/src/pairing.rs","crates/zopro-bn254/src/curve.rs","crates/zopro-bn254/src/fq12.rs","crates/zopro-bn254/src/fq6.rs","crates/zopro-bn254/src/ifma.rs","crates/zopro-bn254/src/ifma8.rs","crates/zopro-core/src/verify.rs"]{let p=Path::new(Z).join(name);let h=hash(&p);assert_eq!(h,accepted["source_files_sha256"][name].as_str().unwrap());pins.insert(p.display().to_string(),h);}
 assert_eq!(hash(FIELD),"1200e63547baa41c299a48e3221f1cef85990b0369f17452dc12eb17045f9713");save(r("qualification-pins.json"),&json!(pins));
 save(r("qualification-plan.json"),&json!({"version":1,"source_pins":r("qualification-pins.json"),"steps":[{"kind":"command","name":"packed-extension-and-Miller-qualification","argv":[r("qualify"),PathBuf::from("check")],"timeout_seconds":300}]}));println!("Prepared exact accepted-field and native prototype qualification pins");
}
fn main(){match std::env::args().nth(1).as_deref(){Some("prepare")=>prepare(),Some("check")=>{for(p,h)in read(r("qualification-pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}let(a,cache)=std::thread::Builder::new().name("packed-miller-512k".into()).stack_size(512*1024).spawn(qualify).unwrap().join().unwrap();save(r("synthetic-fixtures.json"),&cache);save(r("qualification.json"),&a);println!("{}",serde_json::to_string(&a).unwrap());},_=>panic!("prepare | check")}}
