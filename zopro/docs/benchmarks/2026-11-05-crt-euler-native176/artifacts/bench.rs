#[cfg(crt_native)]#[path="base.rs"]mod base;
#[path="perf.rs"]mod perf;
use zopro_bn254::Fr;use zopro_core::fft::{Plan,denominator};use serde_json::json;use sha2::{Digest,Sha256};use std::{hint::black_box,time::Instant};
enum Engine{Original(Plan),#[cfg(crt_native)]Native(base::direct::native::NativeContext)}
impl Engine{fn call(&self,a:Vec<Fr>,b:Vec<Fr>)->Vec<Fr>{match self{Self::Original(p)=>p.euler_product_study(a,b),#[cfg(crt_native)]Self::Native(p)=>p.product(a,b)}}}
fn original(n:usize)->Plan{let root=Fr::root_of_unity(n).unwrap();let coset=Fr::from(7u64);let mut p=Plan::new(n,root,coset,denominator(n,coset).unwrap());p.prepare_euler_study();p}
fn digest(values:&[Fr])->String{let mut h=Sha256::new();for x in values{for w in x.to_canonical(){h.update(w.to_le_bytes());}}format!("{:x}",h.finalize())}
fn cpus()->Vec<usize>{unsafe{let mut set:libc::cpu_set_t=std::mem::zeroed();assert_eq!(libc::sched_getaffinity(0,std::mem::size_of_val(&set),&mut set),0);(0..libc::CPU_SETSIZE as usize).filter(|&c|libc::CPU_ISSET(c,&set)).collect()}}
fn main(){
 let args=std::env::args().collect::<Vec<_>>();assert_eq!(args.len(),6);let mode=&args[1];let n=args[2].parse::<usize>().unwrap();let workers=args[3].parse::<usize>().unwrap();let rounds=args[4].parse::<usize>().unwrap();let profile=args[5]=="profile";assert!(args[5]=="latency"||profile);assert!(n==1024||n==32768);assert!([1,16,32].contains(&workers));assert!(rounds>0);
 let allowed=cpus();let expected=if workers==1{vec![2]}else{(0..workers).collect::<Vec<_>>()};assert_eq!(allowed,expected);
 rayon::ThreadPoolBuilder::new().num_threads(workers).stack_size(512*1024).build_global().unwrap();assert_eq!(rayon::current_num_threads(),workers);
 let setup=Instant::now();let engine=if mode=="ON"{#[cfg(crt_native)]{Engine::Native(base::direct::native::NativeContext::new(n,&base::choose_primes(32768)))}#[cfg(not(crt_native))]{panic!("frozen driver has no candidate")}}else{assert!(mode=="OFF"||mode=="FROZEN");Engine::Original(original(n))};let preparation_ns=setup.elapsed().as_nanos()as u64;
 let mut state=Fr::from(0x9e3779b97f4a7c15u64);let fixtures=(0..2).map(|phase|{let a=(0..n).map(|i|{state=state.square()+Fr::from(17u64);if(i+phase)%37==0{Fr::ZERO}else{state}}).collect::<Vec<_>>();let b=(0..n).map(|i|{state=state.square()+Fr::from(19u64);if(i+phase)%53==0{-Fr::ONE}else{state}}).collect::<Vec<_>>();(a,b)}).collect::<Vec<_>>();
 let oracle=original(n);let expected=fixtures.iter().map(|(a,b)|oracle.euler_product_study(a.clone(),b.clone())).collect::<Vec<_>>();drop(oracle);let input_hashes=fixtures.iter().map(|(a,b)|vec![digest(a),digest(b)]).collect::<Vec<_>>();let output_hashes=expected.iter().map(|e|digest(e)).collect::<Vec<_>>();
 for _ in 0..3{for((a,b),expected)in fixtures.iter().zip(&expected){assert_eq!(engine.call(a.clone(),b.clone()),*expected);}}
 let mut control=perf::Control::from_env().unwrap();if let Some(p)=&mut control{p.enable().unwrap();}let before=perf::Usage::now().unwrap();let mut times=Vec::with_capacity(rounds);let mut last=[None,None];
 for i in 0..rounds{let phase=i%2;let(a,b)=&fixtures[phase];if profile{last[phase]=Some(black_box(engine.call(black_box(a.clone()),black_box(b.clone()))));}else{let start=Instant::now();let result=black_box(engine.call(black_box(a.clone()),black_box(b.clone())));times.push(start.elapsed().as_nanos()as u64);assert_eq!(result,expected[phase]);}}
 let after=perf::Usage::now().unwrap();if let Some(p)=&mut control{p.disable().unwrap();}if profile{for(i,value)in last.iter().enumerate(){if let Some(v)=value{assert_eq!(*v,expected[i]);}}}
 assert_eq!(cpus(),allowed);for((a,b),hashes)in fixtures.iter().zip(&input_hashes){assert_eq!(digest(a),hashes[0]);assert_eq!(digest(b),hashes[1]);}
 let mut sorted=times.clone();sorted.sort_unstable();let median=if sorted.is_empty(){None}else{Some(sorted[sorted.len()/2])};println!("RESULT {}",json!({"mode":mode,"domain":n,"workers":workers,"allowed_cpus":allowed,"profile":profile,"rounds":rounds,"primes":6,"preparation_ns":preparation_ns,"times_ns":times,"median_ns":median,"input_hashes":input_hashes,"output_hashes":output_hashes,"outputs_checked":if profile{rounds.min(2)}else{rounds}+6,"inputs_unchanged":true,"usage_includes_outside_clock_checks_for_latency":true,"usage":after.difference(&before),"pid":std::process::id(),"new_proofs":0}));
}
