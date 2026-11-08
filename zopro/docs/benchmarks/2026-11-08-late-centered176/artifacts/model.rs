use ark_bn254::{Fq, Fr, G1Projective, G2Projective};
use ark_ec::{AffineRepr, CurveGroup, PrimeGroup};
use ark_ff::{BigInteger, Field, PrimeField};
use num_bigint::{BigInt as Z, Sign};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs};
const ROOT:&str="/home/ubuntu/src/zopro-perf-work/late-centered176";
fn z(x:i64)->Z{Z::from(x)}
fn abs(x:&Z)->Z{if x<&z(0){-x}else{x.clone()}}
fn modulo(x:&Z,m:&Z)->Z{let t=x%m;if t<z(0){t+m}else{t}}
fn inv(x:&Z,m:&Z)->Z{
 let(mut a,mut b,mut u,mut v)=(m.clone(),modulo(x,m),z(0),z(1));
 while b!=z(0){let q=&a/&b;(a,b)=(b.clone(),a-&q*&b);(u,v)=(v.clone(),u-q*v);}
 assert_eq!(a,z(1));modulo(&u,m)
}
fn int<F:PrimeField>(x:F)->Z{Z::from_bytes_le(Sign::Plus,&x.into_bigint().to_bytes_le())}
fn field<F:PrimeField>(x:&Z)->F{let(sign,b)=x.to_bytes_le();let y=F::from_le_bytes_mod_order(&b);if sign==Sign::Minus{-y}else{y}}
struct Rng(u64);
impl Rng{fn next(&mut self)->u64{self.0^=self.0<<13;self.0^=self.0>>7;self.0^=self.0<<17;self.0}}
struct Domain{p:Z,r:Z,rinv:Z,pinv:Z,bits:usize,limbs:[i128;5],ninv:u64,rows:u64,max_coeff:i128,calls:u64,negative_outputs:u64,centered_quotients:u64,digest:Sha256}
impl Domain{
 fn new(p:Z,bits:usize)->Self{
  let r=z(1)<<bits;let rinv=inv(&r,&p);let pinv=inv(&p,&r);let limbs=Self::digits(&p);
  let p0=limbs[0]as u64;let mut a=1u64;for _ in 0..6{a=a.wrapping_mul(2u64.wrapping_sub(p0.wrapping_mul(a)));}
  assert_eq!(p0.wrapping_mul(a),1);assert!(&p*z(5)<r);
  Self{p,r,rinv,pinv,bits,limbs,ninv:a.wrapping_neg(),rows:0,max_coeff:0,calls:0,negative_outputs:0,centered_quotients:0,digest:Sha256::new()}
 }
 fn digits(x:&Z)->[i128;5]{let b=z(1)<<52usize;let mut t=x.clone();let mut out=[0;5];for v in &mut out[..4]{*v=modulo(&t,&b).to_string().parse().unwrap();t>>=52usize;}out[4]=t.to_string().parse().unwrap();assert!(out[4]>=-(1i128<<52)&&out[4]<(1i128<<52));out}
 fn polynomial(c:&[i128])->Z{c.iter().enumerate().map(|(i,&x)|Z::from(x)<<(i*52)).sum()}
 fn record(&mut self,c:&[i128]){for &x in c{assert!(x>=i64::MIN as i128&&x<=i64::MAX as i128);self.max_coeff=self.max_coeff.max(x.abs());}}
 fn raw(&mut self,a:&Z,b:&Z)->[i128;11]{
  let aa=Self::digits(a);let bb=Self::digits(b);let mask=(1i128<<52)-1;let mut c=[0;11];
  for i in 0..5{for j in 0..5{let u=aa[i]&mask;let v=bb[j]&mask;let w=u*v;c[i+j]+=w&mask;c[i+j+1]+=w>>52;self.record(&c);
   if aa[i]<0{c[i+j+1]-=v;self.record(&c);}if bb[j]<0{c[i+j+1]-=u;self.record(&c);}if aa[i]<0&&bb[j]<0{c[i+j+2]+=1;self.record(&c);}
  }}assert_eq!(Self::polynomial(&c),a*b);c
 }
 fn redc(&mut self,t:&Z,mut c:[i128;11])->Z{
  self.record(&c);assert_eq!(Self::polynomial(&c),*t);let mut qsum=z(0);let mut high_negative=false;
  let mask52=(1u128<<52)-1;
  for i in 0..5{
   let width=if i==4{self.bits-208}else{52};let mask=(1u64<<width)-1;
   let q=(c[i]as u64).wrapping_mul(self.ninv)&mask;
   qsum+=Z::from(q)<<(i*52);
   if i==4{high_negative=q>>(width-1)!=0;}
   for j in 0..5{let w=(q as u128)*(self.limbs[j]as u128);c[i+j]+=(w&mask52)as i128;c[i+j+1]+=(w>>52)as i128;self.record(&c);}
   assert_eq!(c[i]&mask as i128,0);self.rows+=1;
   if i<4{let carry=c[i]>>52;c[i]=0;c[i+1]+=carry;self.record(&c);}
  }
  let width=self.bits-208;
  let mut out=Z::from(c[4]>>width);
  for(j,&x)in c.iter().enumerate().skip(5){out+=Z::from(x)<<(52*(j-4)-width);}
  if high_negative{out-=&self.p;qsum-=&self.r;self.centered_quotients+=1;}
  let mut qref=modulo(&(-t*&self.pinv),&self.r);if qref>=(&self.r>>1usize){qref-=&self.r;}
  assert_eq!(qsum,qref);assert_eq!(&out*&self.r,t+&qsum*&self.p);
  assert!(abs(&out)*&self.r<=abs(t)+(&self.p*&self.r>>1usize));
  let ds=Self::digits(&out);assert_eq!(Self::polynomial(&ds),out);
  self.calls+=1;self.negative_outputs+=u64::from(out<z(0));
  self.digest.update(out.to_string().as_bytes());self.digest.update([0]);out
 }
 fn product(&mut self,a:&Z,b:&Z)->Z{let c=self.raw(a,b);self.redc(&(a*b),c)}
 fn fused(&mut self,a:&Z,b:&Z,c:&Z,d:&Z,sign:i128)->Z{
  let x=self.raw(a,b);let y=self.raw(c,d);let rows=core::array::from_fn(|i|x[i]+sign*y[i]);
  self.redc(&(a*b+Z::from(sign)*c*d),rows)
 }
 fn canon(&self,x:&Z)->Z{modulo(x,&self.p)}
 fn encode(&self,x:&Z)->Z{modulo(&(x*&self.r),&self.p)}
 fn decode(&self,x:&Z)->Z{modulo(&(x*&self.rinv),&self.p)}
 fn inverse(&self,x:&Z)->Z{
  let a=self.decode(x);assert_ne!(a,z(0));let f=a.modpow(&(&self.p-z(2)),&self.p);
  assert_eq!(modulo(&(&a*&f),&self.p),z(1));assert_eq!(int(field::<Fq>(&a).inverse().unwrap()),f);self.encode(&f)
 }
 fn output(&self)->Value{json!({"bits":self.bits,"reductions":self.calls,"cancellation_rows":self.rows,"negative_outputs":self.negative_outputs,"centered_quotients":self.centered_quotients,"max_signed_coefficient_abs":self.max_coeff.to_string(),"max_signed_coefficient_below_i64":self.max_coeff<i64::MAX as i128,"digest":format!("{:x}",self.digest.clone().finalize())})}
}
fn primitive<F:PrimeField>(name:&str,bits:usize)->Value{
 let p=Z::from_bytes_le(Sign::Plus,&F::MODULUS.to_bytes_le());let mut dom=Domain::new(p.clone(),bits);let h=&p>>1usize;
 let edges=vec![-&p,-&p+z(1),-&h,z(-1),z(0),z(1),h,&p-z(1),p.clone(),-&p*z(2),&p*z(2),-&p*z(5),&p*z(5)];assert_eq!(edges.len(),13);
 let rf=F::from(2u64).pow([bits as u64]).inverse().unwrap();assert_eq!(int(rf),dom.rinv);
 let mut rng=Rng(0xa3566ccdd80f1235);let mut tuples=vec![];
 for(i,a)in edges.iter().enumerate(){for(j,b)in edges.iter().enumerate(){tuples.push([a.clone(),b.clone(),edges[(i+j)%13].clone(),edges[(i*3+j*5)%13].clone()]);}}
 for _ in 0..2048{tuples.push(core::array::from_fn(|_|{let bytes=(0..4).flat_map(|_|rng.next().to_le_bytes()).collect::<Vec<_>>();let v=Z::from_bytes_le(Sign::Plus,&bytes)%&p;if rng.next()&1!=0{-v}else{v}}));}
 let mut closure_checks=0;let mut outputs=0;
 for a in &tuples{for sign in[0i128,1,-1]{let got=if sign==0{dom.product(&a[0],&a[1])}else{dom.fused(&a[0],&a[1],&a[2],&a[3],sign)};
  let expected=(field::<F>(&a[0])*field::<F>(&a[1])+if sign==0{F::ZERO}else if sign==1{field::<F>(&a[2])*field::<F>(&a[3])}else{-field::<F>(&a[2])*field::<F>(&a[3])})*rf;
  assert_eq!(dom.canon(&got),int(expected));
  if a.iter().all(|x|abs(x)<=p){assert!(abs(&got)<p);closure_checks+=1;}outputs+=1;
 }}
 let mut state=[&p-z(17),-&p+z(29),z(117),z(-871)];
 for step in 0..256{let old=state.clone();let mut results=vec![];for sign in[0i128,1,-1]{let got=if sign==0{dom.product(&old[0],&old[1])}else{dom.fused(&old[0],&old[1],&old[2],&old[3],sign)};
  let expected=(field::<F>(&old[0])*field::<F>(&old[1])+if sign==0{F::ZERO}else{field::<F>(&old[2])*field::<F>(&old[3])*if sign<0{-F::ONE}else{F::ONE}})*rf;
  assert_eq!(dom.canon(&got),int(expected));assert!(abs(&got)<p);results.push(got);outputs+=1;closure_checks+=1;}
  state.rotate_left(1);state[3]=results[step%3].clone();
 }
 json!({"field":name,"tuple_population":tuples.len(),"primitive_outputs":outputs,"closure_checks":closure_checks,"actual_native_executed":false,"domain":dom.output()})
}
#[derive(Clone,Debug,PartialEq)]struct Point{x:Vec<Z>,y:Vec<Z>}
fn p1(p:ark_bn254::G1Affine)->Point{assert!(!p.is_zero());Point{x:vec![int(p.x)],y:vec![int(p.y)]}}
fn p2(p:ark_bn254::G2Affine)->Point{assert!(!p.is_zero());Point{x:vec![int(p.x.c0),int(p.x.c1)],y:vec![int(p.y.c0),int(p.y.c1)]}}
fn negative(mode:usize,i:usize)->bool{mode==1||mode==2&&i%3==0}
struct Fixture{degree:usize,initial:Vec<Point>,incoming:Vec<Point>,expected:Vec<Vec<Vec<Point>>>}
fn fixtures()->Vec<Fixture>{
 let mut out=vec![];
 let g1=G1Projective::generator();let g2=G2Projective::generator();let start=Fr::from_le_bytes_mod_order(&[0xa3;64]);let step=Fr::from(1777u64);
 let mut a1=g1*start;let mut b1=g1*(start+Fr::from(987654321u64));let mut a2=g2*start;let mut b2=g2*(start+Fr::from(987654321u64));
 let mut a1s=vec![];let mut b1s=vec![];let mut a2s=vec![];let mut b2s=vec![];
 for _ in 0..200{a1s.push(a1);b1s.push(b1);a2s.push(a2);b2s.push(b2);a1+=g1;b1+=g1*step;a2+=g2;b2+=g2*step;}
 let a1s=G1Projective::normalize_batch(&a1s);let b1s=G1Projective::normalize_batch(&b1s);let a2s=G2Projective::normalize_batch(&a2s);let b2s=G2Projective::normalize_batch(&b2s);
 let(mut e1,mut e2)=(vec![],vec![]);
 for mode in 0..3{let mut c1=a1s.clone();let mut c2=a2s.clone();let(mut ps1,mut ps2)=(vec![],vec![]);for _ in 0..3{
  for i in 0..200{c1[i]=(c1[i].into_group()+if negative(mode,i){-b1s[i].into_group()}else{b1s[i].into_group()}).into_affine();c2[i]=(c2[i].into_group()+if negative(mode,i){-b2s[i].into_group()}else{b2s[i].into_group()}).into_affine();}
  ps1.push(c1.iter().copied().map(p1).collect());ps2.push(c2.iter().copied().map(p2).collect());
 }e1.push(ps1);e2.push(ps2);}
 out.push(Fixture{degree:1,initial:a1s.into_iter().map(p1).collect(),incoming:b1s.into_iter().map(p1).collect(),expected:e1});
 out.push(Fixture{degree:2,initial:a2s.into_iter().map(p2).collect(),incoming:b2s.into_iter().map(p2).collect(),expected:e2});out
}
fn range(x:&Z,p:&Z,k:i64){assert!(abs(x)<p*z(k));}
fn point_model(f:&Fixture,bits:usize)->Value{
 let p=Z::from_bytes_le(Sign::Plus,&Fq::MODULUS.to_bytes_le());let mut d=Domain::new(p,bits);let p=d.p.clone();let mut rows=vec![];let mut total=0;
 for n in[8usize,16,200]{for mode in 0..3{
  let mut state=f.initial[..n].to_vec();
  for pass in 0..3{
   let raw=state.iter().map(|v|Point{x:v.x.iter().map(|v|d.encode(v)).collect(),y:v.y.iter().map(|v|d.encode(v)).collect()}).collect::<Vec<_>>();
   let input=f.incoming[..n].iter().map(|v|Point{x:v.x.iter().map(|v|d.encode(v)).collect(),y:v.y.iter().map(|v|d.encode(v)).collect()}).collect::<Vec<_>>();
   let mut chains=vec![d.encode(&z(1));8];let mut records:Vec<(Z,Z,Option<Z>)>=vec![];
   for i in 0..n{let lane=i%8;let dx=&input[i].x[0]-&raw[i].x[0];
    if f.degree==1{let dy=if negative(mode,i){-&input[i].y[0]-&raw[i].y[0]}else{&input[i].y[0]-&raw[i].y[0]};range(&dy,&p,2);let num=d.product(&dy,&chains[lane]);range(&num,&p,1);let before=chains[lane].clone();chains[lane]=d.product(&before,&dx);range(&chains[lane],&p,1);records.push((dx,num,None));}
    else{let dx1=&input[i].x[1]-&raw[i].x[1];let norm=d.fused(&dx,&dx,&dx1,&dx1,1);range(&norm,&p,1);let before=chains[lane].clone();chains[lane]=d.product(&before,&norm);range(&chains[lane],&p,1);records.push((before,norm,None));}
   }
   let mut inverse=chains.iter().map(|x|d.inverse(x)).collect::<Vec<_>>();
   for i in(0..n).rev(){let lane=i%8;let a=&raw[i];let b=&input[i];let signed_y=|j:usize|if negative(mode,i){-&b.y[j]-&a.y[j]}else{&b.y[j]-&a.y[j]};
    let(x,y)=if f.degree==1{
     let slope=d.product(&records[i].1,&inverse[lane]);range(&slope,&p,1);inverse[lane]=d.product(&inverse[lane].clone(),&records[i].0);
     let xx=d.product(&slope,&slope)-&a.x[0]-&b.x[0];range(&xx,&p,3);let delta=&a.x[0]-&xx;range(&delta,&p,4);
     let product=d.product(&slope,&delta);range(&product,&p,2);let yy=product-&a.y[0];range(&yy,&p,3);(vec![xx],vec![yy])
    }else{
     let factor=d.product(&records[i].0,&inverse[lane]);range(&factor,&p,1);inverse[lane]=d.product(&inverse[lane].clone(),&records[i].1);
     let dx=[&b.x[0]-&a.x[0],&b.x[1]-&a.x[1]];let dy=[signed_y(0),signed_y(1)];
     let num=[d.fused(&dy[0],&dx[0],&dy[1],&dx[1],1),d.fused(&dy[1],&dx[0],&dy[0],&dx[1],-1)];for v in &num{range(v,&p,2);}
     let slope=[d.product(&num[0],&factor),d.product(&num[1],&factor)];for v in &slope{range(v,&p,1);}
     let square=[d.product(&(&slope[0]+&slope[1]),&(&slope[0]-&slope[1])),d.product(&slope[0],&slope[1])*z(2)];for v in &square{range(v,&p,2);}
     let xx=(0..2).map(|j|&square[j]-&a.x[j]-&b.x[j]).collect::<Vec<_>>();for v in &xx{range(v,&p,4);}
     let delta=[&a.x[0]-&xx[0],&a.x[1]-&xx[1]];for v in &delta{range(v,&p,5);}
     let product=[d.fused(&slope[0],&delta[0],&slope[1],&delta[1],-1),d.fused(&slope[0],&delta[1],&slope[1],&delta[0],1)];for v in &product{range(v,&p,3);}
     let yy=(0..2).map(|j|&product[j]-&a.y[j]).collect::<Vec<_>>();for v in &yy{range(v,&p,4);}(xx,yy)
    };
    let got=Point{x:x.iter().map(|v|d.decode(v)).collect(),y:y.iter().map(|v|d.decode(v)).collect()};assert_eq!(got,f.expected[mode][pass][i]);state[i]=got;total+=1;
   }
   rows.push(json!({"n":n,"sign_mode":mode,"pass":pass,"point_outputs":n}));
  }
 }}json!({"degree":f.degree,"radix_bits":bits,"ordinary_full_vector_cases":rows,"point_outputs":total,"domain":d.output(),"tails_exceptions_and_native_inverse_bridge_qualified":false})
}
fn toy()->Value{
 let(p,r)=(z(13),z(256));let pi=inv(&p,&r);let ri=inv(&r,&p);let mut count=0;
 for a in -13..=13{for b in -13..=13{for sign in[0,1,-1]{let t=z(a)*z(b)+z(sign)*z(a-1)*z(b+1);let mut v=t.clone();let mut qs=z(0);for i in 0..2{let q=modulo(&(-&v*&pi),&z(16));v=(&v+&q*&p)/z(16);qs+=q<<(i*4);}if qs>=z(128){qs-=&r;v-=&p;}assert_eq!(&v*&r,&t+&qs*&p);assert_eq!(modulo(&v,&p),modulo(&(&t*&ri),&p));count+=1;}}}
 json!({"prime":13,"radix":256,"quotient_digit_bits":[4,4],"outputs":count})
}
fn main(){
 assert!(!std::path::Path::new(&format!("{ROOT}/result.json")).exists());
 let mut primitive_rows=vec![];for bits in[256,260]{primitive_rows.push(primitive::<Fq>("Fq",bits));primitive_rows.push(primitive::<Fr>("Fr",bits));}
 let fs=fixtures();let mut curves=vec![];for f in &fs{for bits in[256,260]{curves.push(point_model(f,bits));}}
 let mut inequalities=BTreeMap::new();for(field,p)in[("Fq",Z::from_bytes_le(Sign::Plus,&Fq::MODULUS.to_bytes_le())),("Fr",Z::from_bytes_le(Sign::Plus,&Fr::MODULUS.to_bytes_le()))]{for bits in[256,260]{let r=z(1)<<bits;assert!(&p*z(5)<r);assert!(&p*z(4)<r);inequalities.insert(format!("{field}-R{bits}"),json!({"5p_less_than_R":true,"2product_closure":"p/2+2p^2/R<p","mixed_last_digit_bits":bits-208}));}}
 let result=json!({"mathematics_passed":true,"primitive_rows":primitive_rows,"curve_rows":curves,"toy":toy(),"range_inequalities":inequalities,"signed_output_persists_within_batch":true,"host_points_canonical_between_updates":true,"native_execution":false,"timings":0,"new_SDK_requests":0,"new_proofs":0,"canonical_unchanged":35171,"adopted":false,"goal_complete":false,"next":"Explicit complete native lowering/tradeoff review; mathematical pass alone selects no performance or SDK trial"});
 fs::write(format!("{ROOT}/result.json"),serde_json::to_vec_pretty(&result).unwrap()).unwrap();println!("Late-centered exact reduction and full-vector affine model PASS; native tradeoff review required; no speed claim");
}
