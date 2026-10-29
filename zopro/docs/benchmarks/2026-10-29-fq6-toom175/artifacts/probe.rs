use zopro_bn254::{Fq,Fq2,Fq6,Fq12,pairing};
use ark_ff::{BigInt,Field};use ark_ec::pairing::{Pairing,MillerLoopOutput};use ark_ref as ark;
use std::{hint::black_box,time::Instant,alloc::{GlobalAlloc,Layout,System},sync::atomic::{AtomicUsize,Ordering::Relaxed}};
#[path="perf.rs"]mod perf;
struct Counted;static ALLOCS:AtomicUsize=AtomicUsize::new(0);
unsafe impl GlobalAlloc for Counted {
 unsafe fn alloc(&self,l:Layout)->*mut u8{ALLOCS.fetch_add(1,Relaxed);unsafe{System.alloc(l)}}
 unsafe fn dealloc(&self,p:*mut u8,l:Layout){unsafe{System.dealloc(p,l)}}
 unsafe fn realloc(&self,p:*mut u8,l:Layout,n:usize)->*mut u8{ALLOCS.fetch_add(1,Relaxed);unsafe{System.realloc(p,l,n)}}
 unsafe fn alloc_zeroed(&self,l:Layout)->*mut u8{ALLOCS.fetch_add(1,Relaxed);unsafe{System.alloc_zeroed(l)}}
}
#[global_allocator]static A:Counted=Counted;
fn f6(seed:u64)->Fq6 {let x=|i|Fq::from(seed*113+i);Fq6::new(Fq2::new(x(1),x(3)),Fq2::new(x(7),x(11)),Fq2::new(x(17),x(19)))}
fn a2(x:Fq2)->ark::Fq2{ark::Fq2::new(ark::Fq::new_unchecked(BigInt(x.c0.0)),ark::Fq::new_unchecked(BigInt(x.c1.0)))}
fn a6(x:Fq6)->ark::Fq6{ark::Fq6::new(a2(x.c0),a2(x.c1),a2(x.c2))}
fn a12(x:Fq12)->ark::Fq12{ark::Fq12::new(a6(x.c0),a6(x.c1))}
fn closed<T:Field>(start:T,b:T,c:T,n:usize)->T{let p=b.pow([n as u64]);start*p+c*(p-T::ONE)*(b-T::ONE).inverse().unwrap()}
#[inline(never)]fn work(pattern:&str,n:usize)->[Fq12;4]{let mut out=[Fq12::ONE;4];match pattern{
 "chain"|"streams4"=>{let mut state=core::array::from_fn::<_,4,_>(|i|f6(i as u64+1));let b=core::array::from_fn::<_,4,_>(|i|f6(i as u64+21));let c=f6(51);let count=if pattern=="chain"{1}else{4};for _ in 0..n{for j in 0..count{state[j]=black_box(state[j])*black_box(b[j])+c;}}for j in 0..count{out[j]=Fq12::new(state[j],Fq6::ZERO);}},
 "fq12"=>{let mut state=Fq12::new(f6(1),f6(2));let b=Fq12::new(f6(21),f6(22));let c=Fq12::new(f6(51),f6(52));for _ in 0..n{state=black_box(state)*black_box(b)+c;}out[0]=state;},
 "final"=>{let input=Fq12::new(f6(1),f6(2));let mut sum=Fq12::new(Fq6::ZERO,Fq6::ZERO);for _ in 0..n{sum+=black_box(pairing::final_exponentiation(black_box(input)).unwrap());}out[0]=sum;},_=>panic!("pattern")};black_box(out)}
fn check(pattern:&str,n:usize,out:&[Fq12;4]){match pattern{
 "chain"|"streams4"=>{let count=if pattern=="chain"{1}else{4};for j in 0..count{let want=closed(a6(f6(j as u64+1)),a6(f6(j as u64+21)),a6(f6(51)),n);assert_eq!(a6(out[j].c0),want);assert_eq!(out[j].c1,Fq6::ZERO);}},
 "fq12"=>assert_eq!(a12(out[0]),closed(a12(Fq12::new(f6(1),f6(2))),a12(Fq12::new(f6(21),f6(22))),a12(Fq12::new(f6(51),f6(52))),n)),
 "final"=>{let input=a12(Fq12::new(f6(1),f6(2)));let want=ark::Bn254::final_exponentiation(MillerLoopOutput(input)).unwrap().0*ark::Fq12::from(n as u64);assert_eq!(a12(out[0]),want);},_=>unreachable!()}}
fn main(){let args:Vec<_>=std::env::args().collect();assert_eq!(args.len(),3);let pattern=&args[1];let n:usize=args[2].parse().unwrap();assert!(n>0);let prime=if pattern=="final"{1}else{128};for _ in 0..3{check(pattern,prime,&work(pattern,prime));}let mut control=perf::Control::from_env().unwrap();if let Some(p)=control.as_mut(){p.enable().unwrap();}let allocs=ALLOCS.load(Relaxed);let before=perf::Usage::now().unwrap();let start=Instant::now();let out=work(pattern,n);let elapsed=start.elapsed().as_secs_f64();let after=perf::Usage::now().unwrap();let allocations=ALLOCS.load(Relaxed)-allocs;if let Some(p)=control.as_mut(){p.disable().unwrap();}check(pattern,n,&out);assert_eq!(allocations,0);let count=if pattern=="streams4"{4}else{1};let mut bytes=vec![];for v in out{for f in[v.c0,v.c1]{for f in[f.c0,f.c1,f.c2]{for c in[f.c0,f.c1]{for w in c.to_canonical(){bytes.extend_from_slice(&w.to_le_bytes());}}}}}use sha2::Digest;println!("{}",serde_json::json!({"pattern":pattern,"iterations":n,"updates_per_iteration":count,"ns":elapsed*1e9/(n*count)as f64,"elapsed_s":elapsed,"timed_allocations":allocations,"digest":format!("{:x}",sha2::Sha256::digest(&bytes)),"resource":after.difference(&before),"oracle":"independent Arkworks closed-form field recurrence or complete final exponent","checked":true}));}
