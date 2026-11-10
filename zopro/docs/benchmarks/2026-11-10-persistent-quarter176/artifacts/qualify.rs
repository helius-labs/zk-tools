use zopro_bn254::{Fq,Fq2,FqConfig,FpConfig,G1Affine,G2Affine,Field};
use zopro_bn254::ifma8::persistent_quarter as q;
use ark_ff::{PrimeField,BigInteger};
use ark_ec::{PrimeGroup,CurveGroup};
use num_bigint::BigUint as U;
use serde_json::json;
use std::{fs,mem::size_of};
const ROOT:&str="/home/ubuntu/src/zopro-perf-work/persistent-quarter176";
fn big(w:[u64;4])->U{U::from_bytes_le(&w.into_iter().flat_map(u64::to_le_bytes).collect::<Vec<_>>())}
fn words(v:&U)->[u64;4]{let w=v.to_u64_digits();assert!(w.len()<=4);core::array::from_fn(|i|*w.get(i).unwrap_or(&0))}
fn field(v:&U)->Fq{Fq::from_montgomery(words(v))}
fn next(seed:&mut u64)->U{let mut b=[0;32];for part in b.chunks_mut(8){*seed^=*seed<<13;*seed^=*seed>>7;*seed^=*seed<<17;part.copy_from_slice(&seed.to_le_bytes());}U::from_bytes_le(&b)}
struct Oracle{p:U,r:U,pinv:U}
impl Oracle{
 fn new()->Self{let p=big(FqConfig::P);let r=U::from(1u32)<<260usize;let mut pinv=U::from(1u32);
  for _ in 0..9 {pinv=(&pinv*(&r+2u32-(&p*&pinv)%&r))%&r;}assert_eq!((&pinv*&p)%&r,U::from(1u32));
  assert!(&p*64u32<r);Self{p,r,pinv}}
 fn redc(&self,t:U)->U{let m=(&self.r-(((&t%&self.r)*&self.pinv)%&self.r))%&self.r;
  let full=&t+&m*&self.p;assert_eq!(&full%&self.r,U::from(0u32));full/&self.r}
}
fn check<const B:u64>(v:&q::Wide<B>,expect:&[U;8],count:&mut usize){let w=v.words();for i in 0..8{assert_eq!(big(w[i]),expect[i]);*count+=1;}}

#[repr(C)]#[derive(Clone,Copy,Debug,PartialEq,Eq)]struct G1{ x:Fq,y:q::Stored }
impl G1{fn encode(p:G1Affine)->Self{Self{x:p.x,y:q::Stored::encode(p.y)}}fn decode(self)->G1Affine{G1Affine::new(self.x,self.y.decode())}}
#[repr(C)]#[derive(Clone,Copy,Debug,PartialEq,Eq)]struct G2{ x:Fq2,y:[q::Stored;2] }
impl G2{fn encode(p:G2Affine)->Self{Self{x:p.x,y:[q::Stored::encode(p.y.c0),q::Stored::encode(p.y.c1)]}}fn decode(self)->G2Affine{G2Affine::new(self.x,Fq2::new(self.y[0].decode(),self.y[1].decode()))}}
fn g1_scalar(a:G1,b:G1Affine,negative:bool)->G1{G1::encode((a.decode()+if negative{-b}else{b}).to_affine())}
fn g2_scalar(a:G2,b:G2Affine,negative:bool)->G2{G2::encode((a.decode()+if negative{-b}else{b}).to_affine())}
#[inline(never)]fn g1_eight(a:&mut[G1],b:&[G1Affine],mask:u8)->bool{
 if (0..8).any(|i|a[i].decode().is_identity()||b[i].is_identity()||a[i].x==b[i].x){for i in 0..8{a[i]=g1_scalar(a[i],b[i],mask>>i&1!=0);}return false;}
 let ax=q::x(&core::array::from_fn(|i|a[i].x));let bx=q::x(&core::array::from_fn(|i|b[i].x));
 let ay=q::load_y(&core::array::from_fn(|i|a[i].y));let by=q::incoming_y(&core::array::from_fn(|i|b[i].y));
 let d=q::dx(&bx,&ax);let inv=d.inverse_nonzero();let s=q::slope_g1(&q::dy(&by,&ay,mask),&inv);
 let x=q::xcoord(&q::square_g1(&s),&ax,&bx);let y=q::finish_g1(&s,&q::dx(&ax,&x),&ay);
 let xs=q::export_x(&x);let ys=q::store_y(&y);for i in 0..8{a[i]=G1{x:xs[i],y:ys[i]};}true
}
#[inline(never)]fn g2_eight(a:&mut[G2],b:&[G2Affine],mask:u8)->bool{
 if (0..8).any(|i|a[i].decode().is_identity()||b[i].is_identity()||a[i].x==b[i].x){for i in 0..8{a[i]=g2_scalar(a[i],b[i],mask>>i&1!=0);}return false;}
 let ax=[q::x(&core::array::from_fn(|i|a[i].x.c0)),q::x(&core::array::from_fn(|i|a[i].x.c1))];
 let bx=[q::x(&core::array::from_fn(|i|b[i].x.c0)),q::x(&core::array::from_fn(|i|b[i].x.c1))];
 let ay=core::array::from_fn::<_,2,_>(|j|q::load_y(&core::array::from_fn(|i|a[i].y[j])));
 let by=[q::incoming_y(&core::array::from_fn(|i|b[i].y.c0)),q::incoming_y(&core::array::from_fn(|i|b[i].y.c1))];
 let d=core::array::from_fn(|j|q::dx(&bx[j],&ax[j]));let inv=q::norm(&d).inverse_nonzero();
 let dy=core::array::from_fn(|j|q::dy(&by[j],&ay[j],mask));let n=q::numerator(&dy,&d);
 let s=n.map(|v|v.mul(&inv));let sq=q::square(&s);let x=core::array::from_fn::<_,2,_>(|j|q::xcoord(&sq[j],&ax[j],&bx[j]));
 let d=core::array::from_fn(|j|q::dx(&ax[j],&x[j]));let y=q::finish(&s,&d,&ay);
 let xs=x.map(|v|q::export_x(&v));let ys=y.map(|v|q::store_y(&v));for i in 0..8{a[i]=G2{x:Fq2::new(xs[0][i],xs[1][i]),y:[ys[0][i],ys[1][i]]};}true
}
fn fq(a:ark_bn254::Fq)->Fq{Fq::from_canonical(a.into_bigint().0).unwrap()}
fn g1(s:ark_bn254::Fr)->G1Affine{let a=(ark_bn254::G1Projective::generator()*s).into_affine();if a.infinity{G1Affine::new(Fq::ZERO,Fq::ZERO)}else{G1Affine::new(fq(a.x),fq(a.y))}}
fn g2(s:ark_bn254::Fr)->G2Affine{let a=(ark_bn254::G2Projective::generator()*s).into_affine();if a.infinity{G2Affine::new(Fq2::ZERO,Fq2::ZERO)}else{G2Affine::new(Fq2::new(fq(a.x.c0),fq(a.x.c1)),Fq2::new(fq(a.y.c0),fq(a.y.c1)))}}
fn verify_g1(a:G1,s:ark_bn254::Fr){assert_eq!(a.decode(),g1(s));}
fn verify_g2(a:G2,s:ark_bn254::Fr){assert_eq!(a.decode(),g2(s));}
fn main(){
 assert!(std::is_x86_feature_detected!("avx512ifma"));assert_eq!(size_of::<G1>(),64);assert_eq!(size_of::<G2>(),128);
 let o=Oracle::new();let p=&o.p;let mut seed=0xad49ca1178d173u64;let mut arithmetic_checks=0;let mut codec_checks=0;
 // 4096 random vector tuples; exact raw REDC, not just modular congruence.
 // 288 boundary tuples occupy36 vector groups. An explicit inclusive3p
 // vector is separate so padding is never counted as a distinct tuple.
 let byedge=[U::from(0u32),U::from(1u32),p-2u32,p-1u32];
 let ayedge=[U::from(0u32),U::from(1u32),p-1u32,p.clone(),p*2u32-2u32,p*2u32-1u32];
 let dxedge=ayedge.clone();let mut cases=Vec::new();for b in &byedge{for a in &ayedge{for d in &dxedge{for neg in[false,true]{cases.push((b.clone(),a.clone(),d.clone(),neg));}}}}
 assert_eq!(cases.len(),288);for _ in 0..4096{let b=next(&mut seed)%p;let a=next(&mut seed)%(p*2u32);let d=next(&mut seed)%(p*2u32);let neg=seed&1!=0;cases.push((b,a,d,neg));}
 let mut groups=0;for chunk in cases.chunks_exact(8){
  let by=core::array::from_fn(|i|chunk[i].0.clone());let ay=core::array::from_fn(|i|chunk[i].1.clone());let d=core::array::from_fn(|i|chunk[i].2.clone());
  let sign=(0..8).fold(0u8,|m,i|m|((chunk[i].3 as u8)<<i));
  let expect=core::array::from_fn::<_,8,_>(|i|if chunk[i].3{p*3u32-&by[i]-&ay[i]}else{p*2u32+&by[i]-&ay[i]});
  let byq=q::checked::<1>(by.each_ref().map(words));let ayq=q::checked::<2>(ay.each_ref().map(words));let dq=q::checked::<2>(d.each_ref().map(words));
  let dy=q::dy(&byq,&ayq,sign);check(&dy,&expect,&mut arithmetic_checks);
  let d1=q::checked::<2>(core::array::from_fn(|i|words(&d[(i+1)%8])));let dys=[dy,q::dy(&byq,&ayq,sign^255)];let ds=[dq,d1];
  let e1=core::array::from_fn::<_,8,_>(|i|if !chunk[i].3{p*3u32-&by[i]-&ay[i]}else{p*2u32+&by[i]-&ay[i]});
  check(&q::norm(&ds),&core::array::from_fn(|i|o.redc(&d[i]*&d[i]+&d[(i+1)%8]*&d[(i+1)%8])),&mut arithmetic_checks);
  let num=q::numerator(&dys,&ds);let nr=core::array::from_fn(|i|o.redc(&expect[i]*&d[i]+&e1[i]*&d[(i+1)%8]));
  let ni=core::array::from_fn(|i|o.redc(&e1[i]*&d[i]+(p*3u32-&expect[i])*&d[(i+1)%8]));
  check(&num[0],&nr,&mut arithmetic_checks);check(&num[1],&ni,&mut arithmetic_checks);
  let sq=q::square(&num);let sr=core::array::from_fn(|i|o.redc((&nr[i]+&ni[i])*(p*2u32+&nr[i]-&ni[i])));
  let si=core::array::from_fn(|i|o.redc(&nr[i]*(&ni[i]*2u32)));check(&sq[0],&sr,&mut arithmetic_checks);check(&sq[1],&si,&mut arithmetic_checks);
  // Decode/export invariant boundaries and incoming quarter on unrelated H words.
  let inc=by.each_ref().map(|v|field(v));let got=q::incoming_y(&inc).words();
  for i in 0..8 {let k=(&by[i]%4u32).to_u64_digits().first().copied().unwrap_or(0);assert_eq!(big(got[i]),(&by[i]+p*k)/4u32);
   let stored=q::Stored::checked(words(&ay[i]));let dec=big(stored.decode().0);assert_eq!(dec,(&ay[i]*4u32)%p);
   assert_eq!(q::Stored::encode(inc[i]).words(),got[i]);codec_checks+=3;}
  groups+=1;
 }
 let zero1=q::checked::<1>([[0;4];8]);let zero2=q::checked::<2>([[0;4];8]);let inc=q::dy(&zero1,&zero2,255);check(&inc,&core::array::from_fn(|_|p*3u32),&mut arithmetic_checks);
 // Output-fed raw chains retain lazy Y; every operation checked exactly.
 let mut ys=core::array::from_fn::<_,8,_>(|_|next(&mut seed)%(p*2u32));let mut fed_checks=0;
 for _ in 0..512 {let a=core::array::from_fn::<_,8,_>(|_|next(&mut seed)%p);let b=core::array::from_fn::<_,8,_>(|_|next(&mut seed)%p);
  let s=core::array::from_fn::<_,8,_>(|_|next(&mut seed)%(p*2u32));let aq=q::checked::<1>(a.each_ref().map(words));let bq=q::checked::<1>(b.each_ref().map(words));let sq=q::checked::<2>(s.each_ref().map(words));
  let square=q::square_g1(&sq);let squarev=core::array::from_fn::<_,8,_>(|i|o.redc(&s[i]*&s[i]));let x=q::xcoord(&square,&aq,&bq);
  let xv=core::array::from_fn::<_,8,_>(|i|(p*2u32+&squarev[i]-&a[i]-&b[i])%p);check(&x,&xv,&mut fed_checks);
  let d=q::dx(&aq,&x);let ay=q::checked::<2>(ys.each_ref().map(words));let y=q::finish_g1(&sq,&d,&ay);
  let yv=core::array::from_fn::<_,8,_>(|i|(p*2u32+o.redc(&s[i]*(p+&a[i]-&xv[i]))-&ys[i])%(p*2u32));check(&y,&yv,&mut fed_checks);
  let stored=q::store_y(&y);assert_eq!(q::load_y(&stored).words(),y.words());ys=yv;
 }
 let weights=core::array::from_fn::<_,8,_>(|_|ark_bn254::Fr::from_le_bytes_mod_order(&next(&mut seed).to_bytes_le()));
 let adds=core::array::from_fn::<_,8,_>(|_|ark_bn254::Fr::from_le_bytes_mod_order(&next(&mut seed).to_bytes_le()));
 let b1=adds.map(g1);let b2=adds.map(g2);let original=(b1,b2);let mut point_checks=0;let mut normal_groups=0;let mut lifted=0;
 for mask in 0..256u16 {let mut a1=weights.map(|s|G1::encode(g1(s)));let mut a2=weights.map(|s|G2::encode(g2(s)));let mut expected=weights;
  // Exercise noncanonical persisted Y explicitly, independent of random outputs.
  for i in 0..8 {if (i+mask as usize)%2==0{a1[i].y=q::Stored::checked(words(&(big(a1[i].y.words())+p)));lifted+=1;
    for j in 0..2{a2[i].y[j]=q::Stored::checked(words(&(big(a2[i].y[j].words())+p)));lifted+=1;}}}
  for _ in 0..4{assert!(g1_eight(&mut a1,&b1,mask as u8));assert!(g2_eight(&mut a2,&b2,mask as u8));normal_groups+=2;
   for i in 0..8{if mask>>i&1!=0{expected[i]-=adds[i];}else{expected[i]+=adds[i];}verify_g1(a1[i],expected[i]);verify_g2(a2[i],expected[i]);point_checks+=2;}}
 }
 assert_eq!((b1,b2),original);
 let mut tail_point_checks=0;let mut fallback_groups=0;for n in[0,1,7,8,9,15,16,31,32,33]{for kind in 0..4{let mut a1=Vec::new();let mut a2=Vec::new();let mut b1s=Vec::new();let mut b2s=Vec::new();let mut expected=Vec::new();
  for i in 0..n {let w=if i==0&&kind==1{ark_bn254::Fr::from(0)}else{weights[i%8]};let b=if i==0&&kind>=2{w}else{adds[i%8]};
   a1.push(G1::encode(g1(w)));a2.push(G2::encode(g2(w)));b1s.push(g1(b));b2s.push(g2(b));expected.push((w,b));}
  for round in 0..3{let negative=kind==3||round==1;for j in(0..n/8*8).step_by(8){if !g1_eight(&mut a1[j..],&b1s[j..],if negative{255}else{0}){fallback_groups+=1;}
    if !g2_eight(&mut a2[j..],&b2s[j..],if negative{255}else{0}){fallback_groups+=1;}}
   for i in n/8*8..n{a1[i]=g1_scalar(a1[i],b1s[i],negative);a2[i]=g2_scalar(a2[i],b2s[i],negative);}
   for i in 0..n{let (ref mut w,b)=expected[i];if negative{*w-=b}else{*w+=b};verify_g1(a1[i],*w);verify_g2(a2[i],*w);tail_point_checks+=2;}}
 }}
 // Exact qualified native path on small stacks. Threads are a test harness;
 // no production pool or concurrent-throughput result is introduced.
 let target=core::array::from_fn::<_,8,_>(|i|weights[i]+adds[i]);let ref1=target.map(g1);let ref2=target.map(g2);let mut small_stack_groups=0;
 for workers in[1,16,32]{let mut handles=Vec::new();for _ in 0..workers{handles.push(std::thread::Builder::new().stack_size(512*1024).spawn(move||{
   let mut a1=weights.map(|s|G1::encode(g1(s)));let mut a2=weights.map(|s|G2::encode(g2(s)));
   assert!(g1_eight(&mut a1,&b1,0));assert!(g2_eight(&mut a2,&b2,0));assert_eq!(a1.map(G1::decode),ref1);assert_eq!(a2.map(G2::decode),ref2);
  }).unwrap());}for h in handles{h.join().unwrap();small_stack_groups+=2;}}
 let v=json!({"passed":true,"random_tuples":4096,"boundary_tuples":288,"arithmetic_vector_groups":groups,"exact_arithmetic_lane_checks":arithmetic_checks,"codec_checks":codec_checks,"fed_groups":512,"fed_lane_checks":fed_checks,"ordinary_point_checks":point_checks,"ordinary_native_groups":normal_groups,"deliberately_lifted_components":lifted,"tail_and_exception_point_checks":tail_point_checks,"whole_group_fallbacks":fallback_groups,"small_stack_groups":small_stack_groups,"small_stack_workers":[1,16,32],"requested_stack_bytes":524288,"bucket_bytes":{"G1":size_of::<G1>(),"G2":size_of::<G2>()},"native_timing":false,"proofs_added":0,"canonical_proofs":44265,"production_changed":false});
 fs::write(format!("{ROOT}/qualification.json"),serde_json::to_vec_pretty(&v).unwrap()).unwrap();println!("{v}");
}
