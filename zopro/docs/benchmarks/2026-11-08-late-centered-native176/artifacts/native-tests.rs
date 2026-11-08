use ark_bn254::{Fq as Aq,Fr as Ar,G1Affine as A1,G2Affine as A2,G1Projective as P1,G2Projective as P2};
use ark_ec::{AffineRepr,CurveGroup,PrimeGroup};use ark_ff::{BigInteger,Field,PrimeField};
use num_bigint::{BigInt as Z,Sign};use std::{alloc::{GlobalAlloc,Layout,System},cell::Cell};
use zopro_bn254::{Fp,FpConfig,Fq,Fq2,FqConfig,FrConfig,G1Affine,G2Affine};
use zopro_bn254::ifma8::late_centered::Signed;
mod callers;
thread_local!{static COUNTING:Cell<bool>=const{Cell::new(false)};static REQUESTS:Cell<usize>=const{Cell::new(0)};}
struct CountAlloc;
unsafe impl GlobalAlloc for CountAlloc{
 unsafe fn alloc(&self,l:Layout)->*mut u8{if COUNTING.try_with(Cell::get).unwrap_or(false){REQUESTS.with(|v|v.set(v.get()+1));}unsafe{System.alloc(l)}}
 unsafe fn alloc_zeroed(&self,l:Layout)->*mut u8{if COUNTING.try_with(Cell::get).unwrap_or(false){REQUESTS.with(|v|v.set(v.get()+1));}unsafe{System.alloc_zeroed(l)}}
 unsafe fn realloc(&self,p:*mut u8,l:Layout,n:usize)->*mut u8{if COUNTING.try_with(Cell::get).unwrap_or(false){REQUESTS.with(|v|v.set(v.get()+1));}unsafe{System.realloc(p,l,n)}}
 unsafe fn dealloc(&self,p:*mut u8,l:Layout){unsafe{System.dealloc(p,l)}}
}
#[global_allocator]static ALLOC:CountAlloc=CountAlloc;
fn z(x:i64)->Z{Z::from(x)}fn abs(x:&Z)->Z{if x<&z(0){-x}else{x.clone()}}fn modulo(x:&Z,p:&Z)->Z{let v=x%p;if v<z(0){v+p}else{v}}
fn inverse(x:&Z,p:&Z)->Z{let(mut a,mut b,mut u,mut v)=(p.clone(),modulo(x,p),z(0),z(1));while b!=z(0){let q=&a/&b;(a,b)=(b.clone(),a-&q*&b);(u,v)=(v.clone(),u-q*v);}assert_eq!(a,z(1));modulo(&u,p)}
fn words(x:[u64;4])->Z{Z::from_bytes_le(Sign::Plus,&x.into_iter().flat_map(u64::to_le_bytes).collect::<Vec<_>>())}
fn digits(x:&Z)->[i64;5]{let mut t=x.clone();let b=z(1)<<52usize;let mut out=[0;5];for v in &mut out[..4]{*v=modulo(&t,&b).to_string().parse().unwrap();t>>=52usize;}out[4]=t.to_string().parse().unwrap();out}
fn integer(x:[i64;5])->Z{x.iter().enumerate().map(|(i,&v)|Z::from(v)<<(i*52)).sum()}
fn af<A:PrimeField>(x:&Z)->A{let(sign,b)=x.to_bytes_le();let v=A::from_le_bytes_mod_order(&b);if sign==Sign::Minus{-v}else{v}}
fn ai<A:PrimeField>(x:A)->Z{Z::from_bytes_le(Sign::Plus,&x.into_bigint().to_bytes_le())}
struct Rng(u64);impl Rng{fn next(&mut self)->u64{self.0^=self.0<<13;self.0^=self.0>>7;self.0^=self.0<<17;self.0}fn signed(&mut self,bound:&Z)->Z{let bytes=(0..4).flat_map(|_|self.next().to_le_bytes()).collect::<Vec<_>>();let v=Z::from_bytes_le(Sign::Plus,&bytes)%bound;if self.next()&1!=0{-v}else{v}}}
fn raw<C:FpConfig,const M:bool>(x:&[Z;8],p:&Z)->Signed<C,M>{for v in x{assert!(abs(v)<=p*z(5));}unsafe{Signed::from_raw_signed_lanes(&x.each_ref().map(digits))}}
fn check<C:FpConfig,A:PrimeField,const M:bool>(a:&[Z;8],b:&[Z;8],c:&[Z;8],d:&[Z;8],p:&Z,r:&Z,pinv:&Z,rf:A)->[Signed<C,M>;3]{
 let av=raw(a,p);let bv=raw(b,p);let cv=raw(c,p);let dv=raw(d,p);
 let out=[av.mul(&bv),av.sum2(&bv,&cv,&dv),av.difference2(&bv,&cv,&dv)];
 for(op,value)in out.iter().enumerate(){let lanes=value.raw_signed_lanes();for i in 0..8{for &v in &lanes[i][..4]{assert!((0..(1i64<<52)).contains(&v));}
  let t=&a[i]*&b[i]+if op==0{z(0)}else if op==1{&c[i]*&d[i]}else{-&c[i]*&d[i]};let mut q=modulo(&(-&t*pinv),r);if q>=(r>>1usize){q-=r;}
  let exact=(&t+q*p)/r;let got=integer(lanes[i]);assert_eq!(got,exact);let expected=(af::<A>(&a[i])*af::<A>(&b[i])+if op==0{A::ZERO}else if op==1{af::<A>(&c[i])*af::<A>(&d[i])}else{-af::<A>(&c[i])*af::<A>(&d[i])})*rf;assert_eq!(modulo(&got,p),ai(expected));
 }}out
}
fn primitive<C:FpConfig,A:PrimeField,const M:bool>()->usize{
 let p=words(C::P);assert_eq!(p,Z::from_bytes_le(Sign::Plus,&A::MODULUS.to_bytes_le()));let bits=if M{256}else{260};let r=z(1)<<bits;let pinv=inverse(&p,&r);let rf=A::from(2u64).pow([bits as u64]).inverse().unwrap();
 let half=&p>>1usize;let edge=vec![-&p*z(5),-&p*z(2),-&p,-&p+z(1),-&half,z(-1),z(0),z(1),half,&p-z(1),p.clone(),&p*z(2),&p*z(5)];let mut count=0;
 for i in 0..13{for j in 0..13{let a=core::array::from_fn(|k|edge[(i+k)%13].clone());let b=core::array::from_fn(|k|edge[(j+2*k)%13].clone());let c=core::array::from_fn(|k|edge[(i+j+3*k)%13].clone());let d=core::array::from_fn(|k|edge[(i*3+j+5*k)%13].clone());check::<C,A,M>(&a,&b,&c,&d,&p,&r,&pinv,rf);count+=24;}}
 let mut rng=Rng(0x6911beef458134ff);for _ in 0..512{let a=core::array::from_fn(|_|rng.signed(&(&p*z(5))));let b=core::array::from_fn(|_|rng.signed(&(&p*z(5))));let c=core::array::from_fn(|_|rng.signed(&(&p*z(5))));let d=core::array::from_fn(|_|rng.signed(&(&p*z(5))));check::<C,A,M>(&a,&b,&c,&d,&p,&r,&pinv,rf);count+=24;}
 let mut state: [[Z;8];4]=core::array::from_fn(|_|core::array::from_fn(|_|rng.signed(&p)));
 for step in 0..128{let out=check::<C,A,M>(&state[0],&state[1],&state[2],&state[3],&p,&r,&pinv,rf);let lanes=out[step%3].raw_signed_lanes();let next=core::array::from_fn(|i|integer(lanes[i]));for v in &next{assert!(abs(v)<p);}state.rotate_left(1);state[3]=next;count+=24;}
 assert_eq!(count,19416);count
}
#[test]fn signed_products_match_integer_and_arkworks(){let n=primitive::<FqConfig,Aq,true>()+primitive::<FqConfig,Aq,false>()+primitive::<FrConfig,Ar,true>()+primitive::<FrConfig,Ar,false>();assert_eq!(n,77664);println!("COUNTS native_signed_outputs={n} independent_canonical_meanings={n}");}

fn codecs<C:FpConfig,A:PrimeField,const M:bool>()->(usize,usize,usize){
 let p=words(C::P);let bits=if M{256}else{260};let rf=A::from(2u64).pow([bits as u64]).inverse().unwrap();let mut rng=Rng(0xf00d1111348abda3);let mut exports=0;let mut imports=0;let mut inverses=0;
 let edge=[-&p*z(4)+z(1),-&p*z(3),-&p*z(2),-&p,-&p+z(1),z(-1),z(0),z(1),&p-z(1),p.clone(),&p*z(2),&p*z(3),&p*z(4)-z(1)];
 for row in 0..269{let input=core::array::from_fn(|i|if row<13{edge[(row+i)%13].clone()}else{rng.signed(&(&p*z(4)))});let v=raw::<C,M>(&input,&p);let host=v.wide_get();for i in 0..8{assert_eq!(words(host[i].to_canonical()),ai(af::<A>(&input[i])*rf));exports+=1;}}
 for _ in 0..128{let canon=core::array::from_fn::<_,8,_>(|_|modulo(&rng.signed(&p),&p));let host=canon.each_ref().map(|v|{let digits=v.to_u64_digits().1;let mut w=[0;4];w[..digits.len()].copy_from_slice(&digits);Fp::<C>::from_canonical(w).unwrap()});let packed=Signed::<C,M>::new(&host);let packed_raw=packed.raw_signed_lanes();for i in 0..8{assert_eq!(modulo(&integer(packed_raw[i]),&p),modulo(&(&canon[i]*(z(1)<<bits)),&p));imports+=1;}
  let input=core::array::from_fn(|_|{let mut v=rng.signed(&p);if v==z(0){v=z(1);}v});let signed=raw::<C,M>(&input,&p);let roots=signed.root_get();let inv=signed.inverse_nonzero();let output=inv.root_get();let inverse_raw=inv.raw_signed_lanes();for i in 0..8{assert_eq!(words(roots[i].to_canonical()),ai(af::<A>(&input[i])*rf));assert_eq!(words(output[i].to_canonical()),ai((af::<A>(&input[i])*rf).inverse().unwrap()));let n=integer(inverse_raw[i]);assert!(n>=z(0)&&n<p);inverses+=1;}
 }
 (exports,imports,inverses)
}
#[test]fn codecs_and_inverse_boundaries(){let rows=[codecs::<FqConfig,Aq,true>(),codecs::<FqConfig,Aq,false>(),codecs::<FrConfig,Ar,true>(),codecs::<FrConfig,Ar,false>()];let sums=rows.iter().fold((0,0,0),|a,b|(a.0+b.0,a.1+b.1,a.2+b.2));assert_eq!(sums,(8608,4096,4096));println!("COUNTS wide_exports={} imports={} root_inverses={}",sums.0,sums.1,sums.2);}

fn fq(x:Aq)->Fq{Fq::from_canonical(x.into_bigint().0).unwrap()}
fn aq(x:Fq)->Aq{Aq::from_bigint(ark_ff::BigInt(x.to_canonical())).unwrap()}
fn n1(p:A1)->G1Affine{if p.is_zero(){G1Affine::IDENTITY}else{G1Affine::new(fq(p.x),fq(p.y))}}
fn n2(p:A2)->G2Affine{if p.is_zero(){G2Affine::IDENTITY}else{G2Affine::new(Fq2::new(fq(p.x.c0),fq(p.x.c1)),Fq2::new(fq(p.y.c0),fq(p.y.c1)))}}
fn a1(p:G1Affine)->A1{if p.is_identity(){A1::identity()}else{A1::new_unchecked(aq(p.x),aq(p.y))}}
fn a2(p:G2Affine)->A2{if p.is_identity(){A2::identity()}else{A2::new_unchecked(ark_bn254::Fq2::new(aq(p.x.c0),aq(p.x.c1)),ark_bn254::Fq2::new(aq(p.y.c0),aq(p.y.c1)))}}
struct Fixtures{initial1:Vec<G1Affine>,incoming1:Vec<G1Affine>,initial2:Vec<G2Affine>,incoming2:Vec<G2Affine>}
fn fixtures(seed:u8)->Fixtures{
 let scalar=Ar::from_le_bytes_mod_order(&[seed;64]);let g1=P1::generator();let g2=P2::generator();let mut a=g1*scalar;let mut b=g1*(scalar+Ar::from(9988776655u64));let mut c=g2*scalar;let mut d=g2*(scalar+Ar::from(9988776655u64));let step1=g1*Ar::from(137u64);let step2=g2*Ar::from(137u64);let(mut aa,mut bb,mut cc,mut dd)=(vec![],vec![],vec![],vec![]);
 for _ in 0..512{aa.push(a);bb.push(b);cc.push(c);dd.push(d);a+=g1;b+=step1;c+=g2;d+=step2;}
 Fixtures{initial1:P1::normalize_batch(&aa).into_iter().map(n1).collect(),incoming1:P1::normalize_batch(&bb).into_iter().map(n1).collect(),initial2:P2::normalize_batch(&cc).into_iter().map(n2).collect(),incoming2:P2::normalize_batch(&dd).into_iter().map(n2).collect()}
}
fn negative(mode:usize,i:usize)->bool{mode==1||mode==2&&i%3==0}
fn descriptor(mode:usize,i:usize)->u32{((i*37%512)as u32)|if negative(mode,i){1<<31}else{0}}
fn oracle1(state:&mut[A1],batch:&[(u32,&G1Affine)]){for&(encoded,b)in batch{let k=(encoded&0x7fffffff)as usize;let b=a1(*b);state[k]=(state[k].into_group()+if encoded>>31!=0{-b.into_group()}else{b.into_group()}).into_affine();}}
fn oracle2(state:&mut[A2],batch:&[(u32,&G2Affine)]){for&(encoded,b)in batch{let k=(encoded&0x7fffffff)as usize;let b=a2(*b);state[k]=(state[k].into_group()+if encoded>>31!=0{-b.into_group()}else{b.into_group()}).into_affine();}}
fn counted(run:impl FnOnce()){REQUESTS.with(|v|v.set(0));COUNTING.with(|v|v.set(true));run();COUNTING.with(|v|v.set(false));assert_eq!(REQUESTS.with(Cell::get),0);}
#[test]fn complete_callers_tails_reuse_fallback_and_allocations(){
 let f=fixtures(0x6d);let keys1=f.incoming1.clone();let keys2=f.incoming2.clone();let(mut s1,mut s2)=(callers::G1Scratch::default(),callers::G2Scratch::default());let mut compared=0;
 for n in[0usize,1,7,8,9,15,16,31,32,33,199,200,201,351,352,353,511]{for mode in 0..3{let(mut actual1,mut actual2)=(f.initial1.clone(),f.initial2.clone());let(mut expect1,mut expect2)=(actual1.iter().copied().map(a1).collect::<Vec<_>>(),actual2.iter().copied().map(a2).collect::<Vec<_>>());let b1=(0..n).map(|i|(descriptor(mode,i),&f.incoming1[i])).collect::<Vec<_>>();let b2=(0..n).map(|i|(descriptor(mode,i),&f.incoming2[i])).collect::<Vec<_>>();
  for _ in 0..3{oracle1(&mut expect1,&b1);oracle2(&mut expect2,&b2);assert!(callers::checked_g1(&mut actual1,&b1,&mut s1));assert!(callers::checked_g2(&mut actual2,&b2,&mut s2));for i in 0..512{assert_eq!(actual1[i],n1(expect1[i]));assert_eq!(actual2[i],n2(expect2[i]));compared+=2;}}
 }}
 let mut allocation_cases=0;for n in[200usize,201,352,353]{let(mut v1,mut v2)=(f.initial1.clone(),f.initial2.clone());let b1=(0..n).map(|i|(descriptor(2,i),&f.incoming1[i])).collect::<Vec<_>>();let b2=(0..n).map(|i|(descriptor(2,i),&f.incoming2[i])).collect::<Vec<_>>();for _ in 0..3{callers::g1(&mut v1,&b1,&mut s1);callers::g2(&mut v2,&b2,&mut s2);}let cap1=s1.capacities();let cap2=s2.capacities();let(mut ex1,mut ex2)=(v1.iter().copied().map(a1).collect::<Vec<_>>(),v2.iter().copied().map(a2).collect::<Vec<_>>());oracle1(&mut ex1,&b1);oracle2(&mut ex2,&b2);
  counted(||callers::g1(&mut v1,&b1,&mut s1));counted(||callers::g2(&mut v2,&b2,&mut s2));assert_eq!(cap1,s1.capacities());assert_eq!(cap2,s2.capacities());for i in 0..512{assert_eq!(v1[i],n1(ex1[i]));assert_eq!(v2[i],n2(ex2[i]));compared+=2;}allocation_cases+=2;
 }
 let mut fallbacks=0;for n in[1usize,8,9,33]{for kind in 0..6{if kind>=4&&n==1{continue;}let(mut v1,mut v2)=(f.initial1.clone(),f.initial2.clone());let(mut k1,mut k2)=(keys1.clone(),keys2.clone());let(mut ids1,mut ids2)=((0..n).map(|i|descriptor(2,i)).collect::<Vec<_>>(),(0..n).map(|i|descriptor(2,i)).collect::<Vec<_>>());let idx=(ids1[0]&0x7fffffff)as usize;
  match kind{0=>{v1[idx]=G1Affine::IDENTITY;v2[idx]=G2Affine::IDENTITY;},1=>{k1[0]=G1Affine::IDENTITY;k2[0]=G2Affine::IDENTITY;},2=>{k1[0]=v1[idx];k2[0]=v2[idx];},3=>{k1[0]=-v1[idx];k2[0]=-v2[idx];},4=>{ids1[1]=ids1[0];ids2[1]=ids2[0];},_=>{ids1[1]=ids1[0]^(1<<31);ids2[1]=ids2[0]^(1<<31);k1[1]=k1[0];k2[1]=k2[0];}}
  let b1=(0..n).map(|i|(ids1[i],&k1[i])).collect::<Vec<_>>();let b2=(0..n).map(|i|(ids2[i],&k2[i])).collect::<Vec<_>>();let(mut ex1,mut ex2)=(v1.iter().copied().map(a1).collect::<Vec<_>>(),v2.iter().copied().map(a2).collect::<Vec<_>>());oracle1(&mut ex1,&b1);oracle2(&mut ex2,&b2);assert!(!callers::checked_g1(&mut v1,&b1,&mut s1));assert!(!callers::checked_g2(&mut v2,&b2,&mut s2));for i in 0..512{assert_eq!(v1[i],n1(ex1[i]));assert_eq!(v2[i],n2(ex2[i]));compared+=2;}fallbacks+=2;
 }}
 assert_eq!(f.incoming1,keys1);assert_eq!(f.incoming2,keys2);println!("COUNTS complete_point_comparisons={compared} allocation_cases={allocation_cases} checked_whole_call_fallbacks={fallbacks}");
}

#[test]fn shared_fixtures_small_stack_workers(){
 let f=fixtures(0xc7);let mut expected1=vec![];let mut expected2=vec![];for mode in 0..3{let b1=(0..512).map(|i|(descriptor(mode,i),&f.incoming1[i])).collect::<Vec<_>>();let b2=(0..512).map(|i|(descriptor(mode,i),&f.incoming2[i])).collect::<Vec<_>>();let(mut v1,mut v2)=(f.initial1.iter().copied().map(a1).collect::<Vec<_>>(),f.initial2.iter().copied().map(a2).collect::<Vec<_>>());let(mut passes1,mut passes2)=(vec![],vec![]);for _ in 0..2{oracle1(&mut v1,&b1);oracle2(&mut v2,&b2);passes1.push(v1.iter().copied().map(n1).collect::<Vec<_>>());passes2.push(v2.iter().copied().map(n2).collect::<Vec<_>>());}expected1.push(passes1);expected2.push(passes2);}
 let mut total=0;for workers in[1usize,4,16,32]{let count=std::thread::scope(|scope|{let mut handles=vec![];for worker in 0..workers{let(f,e1,e2)=(&f,&expected1,&expected2);handles.push(std::thread::Builder::new().stack_size(512*1024).spawn_scoped(scope,move||{let mode=worker%3;let(mut s1,mut s2)=(callers::G1Scratch::default(),callers::G2Scratch::default());let mut count=0;for n in[200usize,201]{let(mut v1,mut v2)=(f.initial1.clone(),f.initial2.clone());let b1=(0..n).map(|i|(descriptor(mode,i),&f.incoming1[i])).collect::<Vec<_>>();let b2=(0..n).map(|i|(descriptor(mode,i),&f.incoming2[i])).collect::<Vec<_>>();for pass in 0..2{assert!(callers::checked_g1(&mut v1,&b1,&mut s1));assert!(callers::checked_g2(&mut v2,&b2,&mut s2));for i in 0..n{let idx=i*37%512;assert_eq!(v1[idx],e1[mode][pass][idx]);assert_eq!(v2[idx],e2[mode][pass][idx]);count+=2;}}}count}).unwrap());}handles.into_iter().map(|h|h.join().unwrap()).sum::<usize>()});total+=count;}
 assert_eq!(total,85012);println!("COUNTS threaded_point_outputs={total} workers=1,4,16,32 stack_bytes=524288");
}
