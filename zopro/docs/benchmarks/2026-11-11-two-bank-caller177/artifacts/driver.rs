use std::{fs,sync::Arc,time::Instant,hint::black_box};
use sha2::{Sha256,Digest};use serde_json::json;
use zopro_bn254::{G1Affine as Z,Fq,Fr};
use ark_bn254::{G1Affine as A,G1Projective as P,Fr as S};
use ark_ec::{AffineRepr,CurveGroup,VariableBaseMSM};use ark_ff::{PrimeField,BigInteger,Zero};
mod loader;mod perf;
#[global_allocator] static ALLOCATOR:zopro_core::alloc::Allocator=zopro_core::alloc::Allocator;
const R:&str="/home/ubuntu/src/zopro-perf-work/two-bank-caller177";
const IMAGE:&str="/home/ubuntu/src/zopro-perf-work/sdk-control-copies/images/transfer_confidential_1_2.image";
const CACHE:&str="/home/ubuntu/src/zopro-perf-work/g1-tagged-phi-review176/private-input-cache.bin";
fn sha(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn words(b:&[u8])->[u64;4]{assert_eq!(b.len(),32);std::array::from_fn(|i|u64::from_le_bytes(b[i*8..i*8+8].try_into().unwrap()))}
fn encode(p:Z)->Vec<u8>{[p.x,p.y].into_iter().flat_map(|f|f.0.into_iter().flat_map(u64::to_le_bytes)).collect()}
fn decode(b:&[u8])->Z{Z::new(Fq::from_montgomery(words(&b[..32])),Fq::from_montgomery(words(&b[32..64])))}
fn to_ark(p:Z)->A{if p.is_identity(){A::identity()}else{let f=|x:Fq|ark_bn254::Fq::from_le_bytes_mod_order(&x.to_canonical().into_iter().flat_map(u64::to_le_bytes).collect::<Vec<_>>());let a=A::new_unchecked(f(p.x),f(p.y));assert!(a.is_on_curve()&&a.is_in_correct_subgroup_assuming_on_curve());a}}
fn from_ark(p:A)->Z{if p.is_zero(){Z::IDENTITY}else{let f=|x:ark_bn254::Fq|Fq::from_canonical(words(&x.into_bigint().to_bytes_le())).unwrap();Z::new(f(p.x),f(p.y))}}
fn fixture(){
 let image=unsafe{memmap2::Mmap::map(&fs::File::open(IMAGE).unwrap()).unwrap()};assert_eq!(sha(&image),"16a3e2a686372d531879d4ff99e63eab0ca374d686f05a9d7343d2bf534dd041");let key=loader::load_key(Arc::new(image)).unwrap();let(k,mut h)=key.bank_points();assert_eq!((k.len(),h.len()),(30529,32768));h.truncate(30922);
 let cache=fs::read(CACHE).unwrap();assert_eq!(sha(&cache),"fceb9c69ab015ea3ae4b556245ae51e1567710a3905df38febb2e805bc6ff945");assert_eq!(&cache[..8],b"TAGPHI01");let nk=u64::from_le_bytes(cache[8..16].try_into().unwrap())as usize;let nh=u64::from_le_bytes(cache[16..24].try_into().unwrap())as usize;assert_eq!((nk,nh),(k.len(),h.len()));assert_eq!(cache.len(),64+(nk+nh)*65);
 let mut bytes=b"KHBANK01".to_vec();bytes.extend((nk as u64).to_le_bytes());bytes.extend((nh as u64).to_le_bytes());let mut points=Vec::new();let mut scalars=Vec::new();let mut identities=0;
 for(p,c)in k.iter().chain(&h).zip(cache[64..].chunks_exact(65)){assert_eq!(p.is_identity(),c[64]!=0);identities+=usize::from(p.is_identity());Fr::from_canonical(words(&c[..32])).unwrap();points.push(to_ark(*p));scalars.push(S::from_le_bytes_mod_order(&c[..32]));bytes.extend(encode(*p));bytes.extend(&c[..32]);}
 let oracle=from_ark(P::msm(&points,&scalars).unwrap().into_affine());let encoded=encode(oracle);fs::write(format!("{R}/fixture.bin"),&bytes).unwrap();fs::write(format!("{R}/oracle.bin"),&encoded).unwrap();
 println!("{}",json!({"fixture_sha256":sha(&bytes),"oracle_sha256":sha(&encoded),"k_rows":nk,"h_rows":nh,"identity_markers_checked":nk+nh,"identity_points":identities,"points_curve_subgroup_checked":nk+nh,"scalar_canonical_checked":nk+nh,"oracle":"Arkworks full combined MSM","new_proofs":0}));
}
struct Input{k:Vec<Z>,h:Vec<Z>,sk:Vec<[u64;4]>,sh:Vec<[u64;4]>}
impl Input{
 fn load()->Self{let b=fs::read(format!("{R}/fixture.bin")).unwrap();let pins:serde_json::Value=serde_json::from_slice(&fs::read(format!("{R}/capture-pins.json")).unwrap()).unwrap();assert_eq!(sha(&b),pins[format!("{R}/fixture.bin")]);assert_eq!(&b[..8],b"KHBANK01");let nk=u64::from_le_bytes(b[8..16].try_into().unwrap())as usize;let nh=u64::from_le_bytes(b[16..24].try_into().unwrap())as usize;assert_eq!((nk,nh),(30529,30922));assert_eq!(b.len(),24+96*(nk+nh));let mut p=Vec::new();let mut s=Vec::new();for c in b[24..].chunks_exact(96){p.push(decode(&c[..64]));s.push(words(&c[64..]));}let h=p.split_off(nk);let sh=s.split_off(nk);Self{k:p,h,sk:s,sh}}
 fn digest(&self)->String{let mut h=Sha256::new();for(p,s)in self.k.iter().zip(&self.sk).chain(self.h.iter().zip(&self.sh)){h.update(encode(*p));for w in s{h.update(w.to_le_bytes());}}format!("{:x}",h.finalize())}
 fn run(&self,on:bool,want:Z){let got=zopro_core::bank_caller(black_box(&self.k),black_box(&self.sk),black_box(&self.h),black_box(&self.sh),on).to_affine();assert_eq!(got,want);black_box(got);}
}
fn pin(cpu:usize){unsafe{let mut mask:libc::cpu_set_t=std::mem::zeroed();libc::CPU_ZERO(&mut mask);libc::CPU_SET(cpu,&mut mask);assert_eq!(libc::sched_setaffinity(0,std::mem::size_of_val(&mask),&mask),0);}}
fn run(workers:usize,arm:&str,rounds:usize){assert!([1,16,32].contains(&workers));assert!(["OFF","ON"].contains(&arm));assert!([1,17].contains(&rounds));let on=arm=="ON";let input=Input::load();let b=fs::read(format!("{R}/oracle.bin")).unwrap();let pins:serde_json::Value=serde_json::from_slice(&fs::read(format!("{R}/capture-pins.json")).unwrap()).unwrap();assert_eq!(sha(&b),pins[format!("{R}/oracle.bin")]);let want=decode(&b);let digest=input.digest();
 let pool=rayon::ThreadPoolBuilder::new().num_threads(workers).stack_size(512*1024).start_handler(move|i|pin(if workers==1{2}else{i})).build().unwrap();let cpus=pool.broadcast(|ctx|{let cpu=unsafe{libc::sched_getcpu()};assert_eq!(cpu,if workers==1{2}else{ctx.index()as i32});cpu});for _ in 0..3{pool.install(||input.run(on,want));}
 let usage0=perf::Usage::now().unwrap();let mut samples=Vec::with_capacity(rounds);for _ in 0..rounds{let now=Instant::now();pool.install(||input.run(on,want));samples.push(now.elapsed().as_nanos()as u64);}let usage1=perf::Usage::now().unwrap();assert_eq!(input.digest(),digest);let cpus_after=pool.broadcast(|ctx|{let cpu=unsafe{libc::sched_getcpu()};assert_eq!(cpu,if workers==1{2}else{ctx.index()as i32});cpu});assert_eq!(cpus,cpus_after);
 println!("{}",json!({"workers":workers,"arm":arm,"rounds":rounds,"primes":3,"samples_ns":samples,"usage":serde_json::from_str::<serde_json::Value>(&usage1.difference(&usage0)).unwrap(),"worker_cpus":cpus,"input_digest":digest,"all_outputs_checked":true,"immutable_inputs":true,"new_proofs":0}));
}
fn main(){let a=std::env::args().collect::<Vec<_>>();if a.len()==2&&a[1]=="fixture"{fixture();}else{assert_eq!(a.len(),4);run(a[1].parse().unwrap(),&a[2],a[3].parse().unwrap());}}
