use num_bigint::BigInt;
use num_traits::{One,Signed,ToPrimitive,Zero};
#[derive(Clone,Debug,PartialEq,Eq)]pub struct Z{pub re:BigInt,pub im:BigInt}
impl Z{
 pub fn new(re:impl Into<BigInt>,im:impl Into<BigInt>)->Self{Self{re:re.into(),im:im.into()}}
 pub fn mul(&self,b:&Self)->Self{Self::new(&self.re*&b.re-&self.im*&b.im,&self.re*&b.im+&self.im*&b.re)}
 pub fn add(&self,b:&Self)->Self{Self::new(&self.re+&b.re,&self.im+&b.im)}
 pub fn rot(&self)->Self{Self::new(-&self.im,self.re.clone())}
}
pub struct Params{pub a:BigInt,pub b:BigInt,pub prime:BigInt,pub radix:BigInt,pub half:BigInt,pub bound:BigInt,pub neg_inv:Z,pub i:BigInt,pub radix_inverse:BigInt,pub normal_limit:BigInt,pub twiddle_limit:BigInt}
pub fn modulo(x:BigInt,m:&BigInt)->BigInt{let r=x%m;if r.is_negative(){r+m}else{r}}
pub fn inverse(x:&BigInt,m:&BigInt)->BigInt{let(mut a,mut b)=(m.clone(),modulo(x.clone(),m));let(mut s,mut t)=(BigInt::zero(),BigInt::one());while !b.is_zero(){let q=&a/&b;(a,b)=(b.clone(),a-&q*&b);(s,t)=(t.clone(),s-q*t);}assert_eq!(a,BigInt::one());modulo(s,m)}
fn round_div(x:BigInt,d:&BigInt)->BigInt{if x.is_negative(){-round_div(-x,d)}else{(x+d/2u32)/d}}
fn ceil_div(x:BigInt,d:&BigInt)->BigInt{(x+d-1u32)/d}
impl Params{
 pub fn new()->Self{
  let u=BigInt::from(4965661367192848881u64);let a=6u32*&u*&u+3u32*&u;let b=3u32*&u+1u32;
  let prime=BigInt::parse_bytes(b"21888242871839275222246405745257275088548364400416034343698204186575808495617",10).unwrap();assert_eq!(&a*&a+&b*&b,prime);
  let radix=BigInt::one()<<130usize;let half=BigInt::one()<<126usize;let bound=&half+3u32*&b;let k=inverse(&prime,&radix);
  let neg_inv=Z::new(modulo(-&a*&k,&radix),modulo(&b*&k,&radix));let i=modulo(-&a*inverse(&b,&prime),&prime);let radix_inverse=inverse(&radix,&prime);assert_eq!(modulo(&i*&i,&prime),&prime-1u32);
  let product=neg_inv.mul(&Z::new(a.clone(),b.clone()));assert!(modulo(product.re+1u32,&radix).is_zero());assert!(modulo(product.im,&radix).is_zero());
  let correction=(&a+&b+1u32)/2u32;let normal_limit=ceil_div(2u32*&bound*&bound,&radix)+&correction;let twiddle_limit=ceil_div(8u32*&bound*&bound,&radix)+&correction;
  assert!(normal_limit<=half);assert!(&twiddle_limit+&b<&half+&a);assert!(twiddle_limit<BigInt::one()<<127usize);assert!(2u32*&bound+&b<&half+&a);
  assert!(5u32*&bound+3u32*&b<&half+3u32*&a);assert!(a<2u32*&half);assert!(5u32*&bound<&radix/2u32);assert!(bound<BigInt::one()<<127usize);
  Self{a,b,prime,radix,half,bound,neg_inv,i,radix_inverse,normal_limit,twiddle_limit}
 }
 fn assert_bound(&self,z:&Z,b:&BigInt){assert!(z.re.abs()<=*b&&z.im.abs()<=*b,"value{z:?} bound{b}");}
 pub fn reduce(&self,z:&Z)->Z{let q0=round_div(&z.re*&self.a+&z.im*&self.b,&self.prime);let q1=round_div(&z.im*&self.a-&z.re*&self.b,&self.prime);let out=Z::new(&z.re-&q0*&self.a+&q1*&self.b,&z.im-&q0*&self.b-&q1*&self.a);self.assert_bound(&out,&self.bound);out}
 pub fn balance_steps(&self,mut z:Z,steps:usize)->Z{for _ in 0..steps{if z.re>=self.half{z.re-=&self.a;z.im-=&self.b;}else if z.re< -&self.half{z.re+=&self.a;z.im+=&self.b;}}for _ in 0..steps{if z.im>=self.half{z.re+=&self.b;z.im-=&self.a;}else if z.im< -&self.half{z.re-=&self.b;z.im+=&self.a;}}self.assert_bound(&z,&self.bound);z}
 pub fn balance(&self,z:Z)->Z{self.balance_steps(z,1)}
 pub fn center(&self,x:BigInt)->BigInt{modulo(x+&self.radix/2u32,&self.radix)-&self.radix/2u32}
 pub fn redc_raw(&self,t:&Z)->Z{let m=t.mul(&self.neg_inv);let m=Z::new(self.center(m.re),self.center(m.im));let s=t.add(&m.mul(&Z::new(self.a.clone(),self.b.clone())));assert!(modulo(s.re.clone(),&self.radix).is_zero()&&modulo(s.im.clone(),&self.radix).is_zero());Z::new(s.re/&self.radix,s.im/&self.radix)}
 pub fn redc(&self,t:&Z)->Z{let z=self.redc_raw(t);self.assert_bound(&z,&self.normal_limit);self.assert_bound(&z,&self.bound);z}
 pub fn twiddle(&self,a:&Z,b:&Z)->Z{self.assert_bound(a,&(4u32*&self.bound));self.assert_bound(b,&self.bound);let z=self.redc_raw(&a.mul(b));self.assert_bound(&z,&self.twiddle_limit);self.balance(z)}
 pub fn finish(&self,z:Z)->Z{self.assert_bound(&z,&(5u32*&self.bound));self.balance_steps(z,3)}
 pub fn encode(&self,x:&BigInt)->Z{self.reduce(&Z::new(modulo(x*&self.radix,&self.prime),0))}
 pub fn decode(&self,z:&Z)->BigInt{modulo((&z.re+&z.im*&self.i)*&self.radix_inverse,&self.prime)}
}
#[cfg(test)]mod tests{
 use super::*;
 #[test]fn exact_modulus_and_headroom_inequalities(){let p=Params::new();assert!(p.bound.to_i128().is_some());assert_eq!(std::mem::size_of::<(i128,i128)>(),32);assert!(p.normal_limit<p.half);}
 #[test]fn canonical_products_need_no_correction(){let p=Params::new();let v=vec![BigInt::zero(),BigInt::one(),-BigInt::one(),p.half.clone(),-&p.half,p.bound.clone(),-&p.bound];let zs:Vec<_>=v.iter().flat_map(|x|v.iter().map(move|y|Z::new(x.clone(),y.clone()))).collect();for a in &zs{for b in &zs{let got=p.redc(&a.mul(b));assert_eq!(p.decode(&got),modulo(p.decode(a)*p.decode(b),&p.prime));let sum=p.balance(a.add(b));assert_eq!(p.decode(&sum),modulo(p.decode(a)+p.decode(b),&p.prime));}}}
 #[test]fn lazy_twiddle_and_three_step_finish_boundaries(){let p=Params::new();let v=vec![BigInt::zero(),BigInt::one(),-BigInt::one(),p.half.clone(),-&p.half,p.bound.clone(),-&p.bound];for x in &v{for y in &v{let z=Z::new(4u32*x,4u32*y);for a in &v{for b in &v{let rhs=Z::new(a.clone(),b.clone());let got=p.twiddle(&z,&rhs);assert_eq!(p.decode(&got),modulo(p.decode(&z)*p.decode(&rhs),&p.prime));}}}}let mut edges=vec![BigInt::zero(),p.half.clone(),-&p.half,5u32*&p.bound,-(5u32*&p.bound),&p.half+&p.a,&p.half+2u32*&p.a];let base=edges.clone();for e in base{edges.push(&e-1u32);edges.push(&e+1u32);edges.push(-e);}for x in &edges{for y in &edges{if x.abs()>5u32*&p.bound||y.abs()>5u32*&p.bound{continue}let z=Z::new(x.clone(),y.clone());assert_eq!(p.decode(&p.finish(z.clone())),p.decode(&z));}}}
 #[test]fn fullwidth_chains_and_uniform_host_import(){let p=Params::new();let mut seed=0x6a09e667f3bcc909u64;let mut x=BigInt::one();let mut z=p.encode(&x);for _ in 0..4096{let mut bytes=[0u8;32];for chunk in bytes.chunks_mut(8){seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;chunk.copy_from_slice(&seed.to_le_bytes());}let y=modulo(BigInt::from_bytes_le(num_bigint::Sign::Plus,&bytes),&p.prime);let g=p.encode(&y);z=p.balance(p.redc(&z.mul(&g)).add(&g));x=modulo(x*&y+&y,&p.prime);assert_eq!(p.decode(&z),x);let uniform=modulo(&y*&p.radix*&p.radix,&p.prime);let imported=p.balance(p.redc_raw(&Z::new(uniform,0)));assert_eq!(p.decode(&imported),y);}}
}
