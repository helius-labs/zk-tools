pub use zopro_bn254::{G2Config,CurveConfig};
use zopro_bn254::{Fq,Fq2,Fq6,Fq12,G1Affine,G2Affine,pairing::{self,G2Prepared}};
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{fs,time::Instant,hint::black_box};
mod prepared;mod packed;
mod perf{include!("/home/ubuntu/src/zopro-perf-work/g2-last-finisher-query176/perf_support.rs");}
const ROOT:&str="/home/ubuntu/src/zopro-perf-work/three-lane-miller176";
struct Fixture{points:[G1Affine;3],old:[G2Prepared;3],new:[prepared::G2Prepared;3],miller:Fq12,full:Fq12}
fn fq(v:&Value)->Fq{Fq::from_canonical(serde_json::from_value(v.clone()).unwrap()).unwrap()}
fn fq12(v:&Value)->Fq12{let a:[Fq2;6]=std::array::from_fn(|i|Fq2::new(fq(&v[2*i]),fq(&v[2*i+1])));Fq12::new(Fq6::new(a[0],a[2],a[4]),Fq6::new(a[1],a[3],a[5]))}
fn fixtures()->Vec<Fixture>{let v:Value=serde_json::from_slice(&fs::read(format!("{ROOT}/synthetic-fixtures.json")).unwrap()).unwrap();[3usize,4,6,8,9,11,12,13].iter().map(|&index|{let row=&v["fixtures"][index];let p=std::array::from_fn(|i|G1Affine::new(fq(&row["points"][i]["g1"][0]),fq(&row["points"][i]["g1"][1])));let q:[G2Affine;3]=std::array::from_fn(|i|G2Affine::new(Fq2::new(fq(&row["points"][i]["g2"][0]),fq(&row["points"][i]["g2"][1])),Fq2::new(fq(&row["points"][i]["g2"][2]),fq(&row["points"][i]["g2"][3]))));assert!(p.iter().all(|p|!p.is_identity())&&q.iter().all(|q|!q.is_identity()));Fixture{points:p,old:q.map(|q|G2Prepared::new(&q)),new:q.map(|q|prepared::G2Prepared::new(&q)),miller:fq12(&row["miller"]),full:fq12(&row["final"])}}).collect()}
fn call(f:&Fixture,arm:&str,full:bool,workers:usize)->Fq12{
 let old=std::array::from_fn::<_,3,_>(|i|(f.points[i],&f.old[i]));let new=std::array::from_fn::<_,3,_>(|i|(f.points[i],&f.new[i]));
 let value=match arm{
  "serial"=>pairing::multi_miller_loop(&old),
  "native"=>if workers>=3{let(a,(b,c))=rayon::join(||pairing::multi_miller_loop(&old[..1]),||rayon::join(||pairing::multi_miller_loop(&old[1..2]),||pairing::multi_miller_loop(&old[2..])));a*b*c}else{pairing::multi_miller_loop(&old)},
  "fresh"=>if workers>=3{let(a,(b,c))=rayon::join(||prepared::multi_miller_loop(&new[..1]),||rayon::join(||prepared::multi_miller_loop(&new[1..2]),||prepared::multi_miller_loop(&new[2..])));a*b*c}else{prepared::multi_miller_loop(&new)},
  "packed"=>packed::miller(&new),_=>panic!("arm")};
 if full{pairing::final_exponentiation(value).unwrap()}else{value}
}
fn main(){
 let a=std::env::args().collect::<Vec<_>>();let arm=&a[1];let endpoint=&a[2];let full=match endpoint.as_str(){"miller"=>false,"full"=>true,_=>panic!("endpoint")};let workers=a[3].parse::<usize>().unwrap();assert!([1,16,32].contains(&workers));let rounds=a[4].parse::<usize>().unwrap();let fixtures=fixtures();
 let pool=rayon::ThreadPoolBuilder::new().num_threads(workers).stack_size(512*1024).start_handler(move|i|{let cpu=if workers==1{2}else{i};unsafe{let mut set:libc::cpu_set_t=std::mem::zeroed();libc::CPU_ZERO(&mut set);libc::CPU_SET(cpu,&mut set);assert_eq!(libc::sched_setaffinity(0,std::mem::size_of_val(&set),&set),0);}}).build().unwrap();
 let pins=pool.broadcast(|context|{let cpu=unsafe{libc::sched_getcpu()};let expected=if workers==1{2}else{context.index()};assert_eq!(cpu,expected as i32);json!({"index":context.index(),"cpu":cpu,"tid":unsafe{libc::syscall(libc::SYS_gettid)}})});
 assert_eq!(pool.install(rayon::current_num_threads),workers);
 for i in 0..3{let f=&fixtures[i%fixtures.len()];let out=pool.install(||call(f,arm,full,workers));assert_eq!(out,if full{f.full}else{f.miller});}
 let mut samples=Vec::with_capacity(rounds);let mut outputs=Vec::with_capacity(rounds);let mut control=perf::Control::from_env().unwrap();if let Some(c)=control.as_mut(){c.enable().unwrap();}let usage=perf::Usage::now().unwrap();let entire=Instant::now();
 for i in 0..rounds{let f=&fixtures[i%fixtures.len()];let start=Instant::now();let out=pool.install(||black_box(call(black_box(f),arm,full,workers)));samples.push(start.elapsed().as_nanos()as u64);outputs.push(out);}
 let wall=entire.elapsed().as_nanos()as u64;let usage=perf::Usage::now().unwrap().difference(&usage);if let Some(c)=control.as_mut(){c.disable().unwrap();}
 let mut digest=Sha256::new();for(i,v)in outputs.iter().enumerate(){let f=&fixtures[i%fixtures.len()];assert_eq!(*v,if full{f.full}else{f.miller});for q in [v.c0.c0,v.c1.c0,v.c0.c1,v.c1.c1,v.c0.c2,v.c1.c2]{for field in[q.c0,q.c1]{for word in field.to_canonical(){digest.update(word.to_le_bytes());}}}}
 let mut sorted=samples.clone();sorted.sort_unstable();println!("{}",json!({"arm":arm,"endpoint":endpoint,"workers":workers,"rounds":rounds,"primes":3,"samples_ns":samples,"median_ns":sorted[rounds/2],"loop_wall_ns":wall,"usage":usage,"checked_outputs":outputs.len(),"output_sha256":format!("{:x}",digest.finalize()),"worker_pins":pins,"source":"native prototype; synthetic tuples reused from qualification cache","SDK_requests":0,"new_proofs":0}));
}
