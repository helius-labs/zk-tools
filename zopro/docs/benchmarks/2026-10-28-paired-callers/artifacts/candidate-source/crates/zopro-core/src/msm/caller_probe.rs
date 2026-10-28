use super::*;
use zopro_bn254::{G1Config,G2Config};
use std::hint::black_box;

struct Fixture<C:CurveConfig>where C::Base:BatchInverse{source:Vec<Affine<C>>,buckets:Vec<Affine<C>>,batch:Vec<(u32,Affine<C>)>,scratch:BatchAdd<C>,signed:bool,coefficients:Vec<u64>,steps:Vec<u64>,seed:u64}
impl<C:CurveConfig>Fixture<C>where C::Base:BatchInverse{
fn new(n:usize,signed:bool,seed:u64)->Self{assert!(n<=512);let size=n.max(1).next_power_of_two();let g=Projective::<C>::generator();let coefficients:Vec<_>=(0..size).map(|i|(1u64<<48)*(i as u64+1)+(seed+1)*100000003).collect();let raw:Vec<_>=coefficients.iter().map(|c|g*Fr::from(*c)).collect();let source=Projective::<C>::batch_to_affine(&raw);let steps:Vec<_>=(0..n).map(|i|11111+(seed+1)*1337+i as u64*101).collect();let incoming=Projective::<C>::batch_to_affine(&steps.iter().map(|c|g*Fr::from(*c)).collect::<Vec<_>>());let batch=incoming.into_iter().enumerate().map(|(i,p)|{let flag=if signed&&i%3!=1{1u32<<31}else{0};((((i*37+seed as usize)%size)as u32)|flag,p)}).collect();let scratch=BatchAdd::default();Self{buckets:source.clone(),source,batch,scratch,signed,coefficients,steps,seed}}
fn reset(&mut self){self.buckets.copy_from_slice(&self.source)}
fn check(&self,rounds:usize){let mut scalars=self.coefficients.iter().map(|c|Fr::from(*c)).collect::<Vec<_>>();for(i,(index,_))in self.batch.iter().enumerate(){let signed=if self.signed&&index>>31!=0{-Fr::ONE}else{Fr::ONE};scalars[(index&0x7fffffff)as usize]+=signed*Fr::from(self.steps[i])*Fr::from(rounds as u64);}let g=Projective::<C>::generator();let expected=Projective::<C>::batch_to_affine(&scalars.iter().map(|s|g* *s).collect::<Vec<_>>());assert_eq!(self.buckets,expected);}
fn bytes(&self)->Vec<u8>{format!("{:?}",self.buckets).into_bytes()}
}
pub struct CallerG1(Fixture<G1Config>);
impl CallerG1{pub fn new(n:usize,signed:bool,seed:u64)->Self{let s=Fixture::new(n,signed,seed);assert!(s.scratch.direct&&s.scratch.fq.canonical&&s.scratch.fq.vector_inverse&&s.scratch.fq.write_groups);Self(s)}pub fn reset(&mut self){self.0.reset()}pub fn run(&mut self,rounds:usize){for _ in 0..rounds{let s=black_box(&mut self.0);if s.signed{s.scratch.apply_signed(&mut s.buckets,&s.batch);}else{s.scratch.apply(&mut s.buckets,&s.batch);}}}pub fn check(&self,n:usize){self.0.check(n)}pub fn bytes(&self)->Vec<u8>{self.0.bytes()}pub fn capacity_bytes(&self)->usize{self.0.scratch.fq.groups.capacity()*std::mem::size_of::<FqAddGroup>()+self.0.scratch.fq.tail.capacity()*std::mem::size_of::<[Fq;5]>()}}
pub struct CallerG2(Fixture<G2Config>);
impl CallerG2{pub fn new(n:usize,seed:u64)->Self{let s=Fixture::new(n,false,seed);assert!(s.scratch.direct&&s.scratch.fq2.canonical&&s.scratch.fq2.vector_inverse&&s.scratch.fq2.prefix_enabled);Self(s)}pub fn reset(&mut self){self.0.reset()}pub fn run(&mut self,rounds:usize){for _ in 0..rounds{let s=black_box(&mut self.0);s.scratch.apply(&mut s.buckets,&s.batch);}}pub fn check(&self,n:usize){self.0.check(n)}pub fn bytes(&self)->Vec<u8>{self.0.bytes()}pub fn capacity_bytes(&self)->usize{self.0.scratch.fq2.prefix_records.capacity()*std::mem::size_of::<[FqBatch;2]>()+self.0.scratch.fq2.tail.capacity()*std::mem::size_of::<[Fq2;5]>()}}
#[cfg(test)]mod tests{use super::*;
#[test]fn g1_real_stream_generator_oracle_and_reset(){for signed in[false,true]{for n in[0,1,7,8,9,31,32,128,512]{let mut f=CallerG1::new(n,signed,19);for rounds in[1,3,17]{f.reset();f.run(rounds);f.check(rounds);}let _=f.capacity_bytes();}}}
#[test]fn g2_real_prefix_generator_oracle_and_reset(){for n in[0,1,7,8,9,31,32,128,512]{let mut f=CallerG2::new(n,23);for rounds in[1,3,17]{f.reset();f.run(rounds);f.check(rounds);}let _=f.capacity_bytes();}}
}
