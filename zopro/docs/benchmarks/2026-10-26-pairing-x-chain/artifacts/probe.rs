use std::{fs,fmt::Debug,hint::black_box,time::Instant};
use serde_json::{json,Value};
use frozen_bn254 as old;
use zopro_bn254 as new;
#[path="support/perf.rs"]mod perf;
fn rng(s:&mut u64)->u64{*s^=*s<<13;*s^=*s>>7;*s^=*s<<17;*s}
fn fq(s:&mut u64)->old::Fq{let mut b=[0u8;64];for c in b.chunks_exact_mut(8){c.copy_from_slice(&rng(s).to_le_bytes());}old::Fq::from_random_bytes(&b)}
fn fixtures()->Vec<old::Fq12>{
 let mut seed=0x638549abcdef1873u64;let mut v=vec![old::Fq12::default(),old::Fq12::ONE,-old::Fq12::ONE];
 for _ in 0..256{let mut q=||old::Fq2::new(fq(&mut seed),fq(&mut seed));v.push(old::Fq12::new(old::Fq6::new(q(),q(),q()),old::Fq6::new(q(),q(),q())));}
 for i in 1..=16{let p=old::G1Projective::generator().mul_limbs(&[i]).to_affine();let q=old::G2Projective::generator().mul_limbs(&[i+3]).to_affine();let prepared=old::pairing::G2Prepared::new(&q);v.push(old::pairing::multi_miller_loop(&[(p,&prepared)]));}assert_eq!(v.len(),275);v
}
fn convert(x:old::Fq12)->new::Fq12{let q=|x:old::Fq2|new::Fq2::new(new::Fq::from_montgomery(x.c0.0),new::Fq::from_montgomery(x.c1.0));let s=|x:old::Fq6|new::Fq6::new(q(x.c0),q(x.c1),q(x.c2));new::Fq12::new(s(x.c0),s(x.c1))}
fn run<F:Copy+PartialEq+Debug>(v:&[F],expected:&[Option<F>],call:impl Fn(F)->Option<F>,rounds:usize,block:usize,mode:&str)->Result<(),Box<dyn std::error::Error>>{
 for(i,&f)in v.iter().enumerate(){assert_eq!(call(f),expected[i]);}
 let index=|r:usize|3+(block*31+r)%256;
 for r in 0..3{let i=index(r);assert_eq!(call(black_box(v[i])),expected[i]);}
 let mut control=perf::Control::from_env()?;let mut samples=Vec::with_capacity(rounds);
 if let Some(c)=control.as_mut(){c.enable()?;}let a=perf::Usage::now()?;
 for r in 0..rounds{let i=index(r);let start=Instant::now();let out=black_box(call(black_box(v[i])));let us=start.elapsed().as_secs_f64()*1e6;assert_eq!(out,expected[i]);samples.push(us);}
 let b=perf::Usage::now()?;if let Some(c)=control.as_mut(){c.disable()?;}
 for(r,us)in samples.iter().enumerate(){println!("SAMPLE {}",json!({"round":r,"us":us}));}
 let mut summary=b.difference(&a);summary["mode"]=json!(mode);summary["rounds"]=json!(rounds);summary["priming"]=json!(3);summary["fixtures"]=json!(v.len());summary["block"]=json!(block);println!("SUMMARY {summary}");Ok(())
}
fn main()->Result<(),Box<dyn std::error::Error>>{
 let args:Vec<_>=std::env::args().collect();assert_eq!(args.len(),4);let mode=args[1].as_str();let rounds=args[2].parse()?;let block=args[3].parse()?;
 let v=fixtures();let expected:Vec<_>=v.iter().map(|&f|old::pairing::final_exponentiation(f)).collect();
 match mode{"frozen"=>run(&v,&expected,old::pairing::final_exponentiation,rounds,block,mode),"off"|"on"=>{let x:Vec<_>=v.into_iter().map(convert).collect();let e:Vec<_>=expected.into_iter().map(|x|x.map(convert)).collect();run(&x,&e,new::pairing::final_exponentiation,rounds,block,mode)},_=>panic!("mode")}
}
