//! Independent exact row recurrence and IFMA-half interpreter.
use crate::model::{Params,Z,modulo};
use num_bigint::BigInt;
use num_traits::{One,Signed,Zero};

pub fn rows(p:&Params,a:&Z,b:&Z)->(Z,Z){
 let base=BigInt::one()<<52usize;let n=Z::new(p.a.clone(),p.b.clone());let ar=modulo(a.re.clone(),&p.radix);let ai=modulo(a.im.clone(),&p.radix);let br=modulo(b.re.clone(),&p.radix);let bi=modulo(b.im.clone(),&p.radix);let unsigned=Z::new(ar.clone(),ai.clone());let mut t=Z::new(0,0);let mut mu=Z::new(0,0);let mut scale=BigInt::one();
 for j in 0..3{let bits=if j==2{26}else{52};let beta=BigInt::one()<<bits;let digit=Z::new((&br>>(52*j))%&base,(&bi>>(52*j))%&base);t=t.add(&unsigned.mul(&digit));let candidate=t.mul(&p.neg_inv);let m=Z::new(modulo(candidate.re,&beta),modulo(candidate.im,&beta));mu=mu.add(&Z::new(&m.re*&scale,&m.im*&scale));t=t.add(&m.mul(&n));assert!(modulo(t.re.clone(),&beta).is_zero()&&modulo(t.im.clone(),&beta).is_zero());t=Z::new(t.re/&beta,t.im/&beta);scale*=&beta;}
 let signed_product=a.mul(b);let expected=signed_product.mul(&p.neg_inv);let expected=Z::new(modulo(expected.re,&p.radix),modulo(expected.im,&p.radix));assert_eq!(mu,expected);
 // Correct encoded negative inputs, then center the unsigned cancellation.
 if a.re.is_negative(){t.re-=&br;t.im-=&bi;}if a.im.is_negative(){t.re+=&bi;t.im-=&br;}
 if b.re.is_negative(){t.re-=&ar;t.im-=&ai;}if b.im.is_negative(){t.re+=&ai;t.im-=&ar;}
 // The discarded R multiples do not affect the final130bit representation.
 if mu.re>=&p.radix/2u32{t.re-=&p.a;t.im-=&p.b;}if mu.im>=&p.radix/2u32{t.re+=&p.b;t.im-=&p.a;}
 t=Z::new(p.center(t.re),p.center(t.im));assert_eq!(t,p.redc_raw(&signed_product));(t,mu)
}

fn digits(x:&BigInt)->[BigInt;3]{let mask=(BigInt::one()<<52usize)-1u32;std::array::from_fn(|i|(x>>(52*i))& &mask)}
fn value(t:&[BigInt;4])->BigInt{t.iter().enumerate().fold(BigInt::zero(),|s,(i,x)|s+(x<<(52*i)))}
fn mac(t:&mut[BigInt;4],x:&[BigInt;3],y:&BigInt,sign:i32,finalrow:bool,peak:&mut BigInt){
 let base=BigInt::one()<<52usize;for i in 0..3{let product=&x[i]*y;let lo=modulo(product.clone(),&base);let hi=product/&base;t[i]+=sign*&lo;*peak=peak.clone().max(t[i].abs());if i+1<4&&!(finalrow&&i==2){t[i+1]+=sign*hi;*peak=peak.clone().max(t[i+1].abs());}}
}
pub fn coefficients(p:&Params,a:&Z,b:&Z)->(Z,BigInt){
 let base=BigInt::one()<<52usize;let ar=modulo(a.re.clone(),&p.radix);let ai=modulo(a.im.clone(),&p.radix);let br=modulo(b.re.clone(),&p.radix);let bi=modulo(b.im.clone(),&p.radix);let ax=digits(&ar);let ay=digits(&ai);let asum=digits(&(&ar+&ai));let bx=digits(&br);let by=digits(&bi);let nr=digits(&p.a);let ni=digits(&p.b);let ns=digits(&(&p.a+&p.b));let mut re:[BigInt;4]=std::array::from_fn(|_|BigInt::zero());let mut im=re.clone();let mut peak=BigInt::zero();let mut mu=Z::new(0,0);let mut scale=BigInt::one();let mut expected_t=Z::new(0,0);let mut repacked=Z::new(0,0);
 for j in 0..3{let finalrow=j==2;let beta=BigInt::one()<<if finalrow{26usize}else{52usize};
  // P=Ar*Br,Q=Ai*Bi,S=(Ar+Ai)*(Br+Bi),real=P-Q,imag=S-P-Q.
  mac(&mut re,&ax,&bx[j],1,finalrow,&mut peak);mac(&mut re,&ay,&by[j],-1,finalrow,&mut peak);
  let bsum=&bx[j]+&by[j];let bsum_lo=modulo(bsum.clone(),&base);let bsum_carry=bsum/&base;
  mac(&mut im,&asum,&bsum_lo,1,finalrow,&mut peak);mac(&mut im,&ax,&bx[j],-1,finalrow,&mut peak);mac(&mut im,&ay,&by[j],-1,finalrow,&mut peak);
  for i in 0..3{if !(finalrow&&i==2){im[i+1]+=&bsum_carry*&asum[i];peak=peak.max(im[i+1].abs());}}
  let m0=Z::new(re[0].clone(),im[0].clone()).mul(&p.neg_inv);let m=Z::new(modulo(m0.re,&beta),modulo(m0.im,&beta));mu=mu.add(&Z::new(&m.re*&scale,&m.im*&scale));
  let sum=&m.re+&m.im;let lo=modulo(sum.clone(),&base);let carry=sum/&base;
  mac(&mut re,&nr,&m.re,1,finalrow,&mut peak);mac(&mut re,&ni,&m.im,-1,finalrow,&mut peak);
  mac(&mut im,&ns,&lo,1,finalrow,&mut peak);mac(&mut im,&nr,&m.re,-1,finalrow,&mut peak);mac(&mut im,&ni,&m.im,-1,finalrow,&mut peak);
  for i in 0..3{if !(finalrow&&i==2){im[i+1]+=&carry*&ns[i];peak=peak.max(im[i+1].abs());}}
  assert!(modulo(re[0].clone(),&beta).is_zero()&&modulo(im[0].clone(),&beta).is_zero());
  let digit=Z::new(bx[j].clone(),by[j].clone());expected_t=expected_t.add(&Z::new(ar.clone(),ai.clone()).mul(&digit)).add(&m.mul(&Z::new(p.a.clone(),p.b.clone())));expected_t=Z::new(expected_t.re/&beta,expected_t.im/&beta);
  if !finalrow{re[1]+=&re[0]/&base;im[1]+=&im[0]/&base;peak=peak.max(re[1].abs()).max(im[1].abs());re=[re[1].clone(),re[2].clone(),re[3].clone(),BigInt::zero()];im=[im[1].clone(),im[2].clone(),im[3].clone(),BigInt::zero()];assert_eq!(value(&re),expected_t.re);assert_eq!(value(&im),expected_t.im);}
  else{let repack=|t:&[BigInt;4]|{let mask=&beta-1u32;let q0=(&t[0]>>26usize)+((modulo(t[1].clone(),&beta))<<26usize);let q1=(&t[1]>>26usize)+((modulo(t[2].clone(),&beta))<<26usize);let q2=&t[2]>>26usize;let _=mask;modulo(q0+(q1<<52usize)+(q2<<104usize),&p.radix)};repacked=Z::new(repack(&re),repack(&im));assert_eq!(repacked.re,modulo(value(&re)/&beta,&p.radix));assert_eq!(repacked.im,modulo(value(&im)/&beta,&p.radix));assert_eq!(repacked.re,modulo(expected_t.re.clone(),&p.radix));assert_eq!(repacked.im,modulo(expected_t.im.clone(),&p.radix));}
  scale*=&beta;
 }
 assert!(peak<(BigInt::one()<<59usize));let (_,expected_mu)=rows(p,a,b);assert_eq!(mu,expected_mu);
 if a.re.is_negative(){repacked.re-=&br;repacked.im-=&bi;}if a.im.is_negative(){repacked.re+=&bi;repacked.im-=&br;}
 if b.re.is_negative(){repacked.re-=&ar;repacked.im-=&ai;}if b.im.is_negative(){repacked.re+=&ai;repacked.im-=&ar;}
 if mu.re>=&p.radix/2u32{repacked.re-=&p.a;repacked.im-=&p.b;}if mu.im>=&p.radix/2u32{repacked.re+=&p.b;repacked.im-=&p.a;}
 repacked=Z::new(p.center(repacked.re),p.center(repacked.im));assert_eq!(repacked,p.redc_raw(&a.mul(b)));(repacked,peak)
}

#[cfg(test)]mod tests{
 use super::*;
 #[test]fn coupled_rows_and_ifma_coefficients_match_comba_corners(){let p=Params::new();let h=&p.bound;let vals=[BigInt::zero(),BigInt::one(),-BigInt::one(),h.clone(),-h,4u32*h,-(4u32*h)];let ys=[BigInt::zero(),BigInt::one(),-BigInt::one(),h.clone(),-h];for a in &vals{for b in &vals{for c in &ys{for d in &ys{let x=Z::new(a.clone(),b.clone());let y=Z::new(c.clone(),d.clone());rows(&p,&x,&y);coefficients(&p,&x,&y);}}}}}
 #[test]fn coupled_rows_fullwidth_and_low_digit_boundaries(){let p=Params::new();let mut seed=0x510e527fade682d1u64;for k in 0..4096{let mut next=||{let mut bytes=[0u8;32];for out in bytes.chunks_mut(8){seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;out.copy_from_slice(&seed.to_le_bytes());}let n=modulo(BigInt::from_bytes_le(num_bigint::Sign::Plus,&bytes),&p.prime);p.encode(&n)};let mut a=next();let b=next();if k%2==0{a.re*=4u32;a.im*=4u32;}rows(&p,&a,&b);coefficients(&p,&a,&b);}let beta=BigInt::one()<<52usize;for bit in [0usize,1,25,26,51,52,103,104,125,126]{for delta in [-1i32,0,1]{let x:BigInt=(BigInt::one()<<bit)+delta;if x.abs()>4u32*&p.bound{continue}let a=Z::new(x.clone(),-x);let b=Z::new(&beta-1u32,-&beta+1u32);rows(&p,&a,&b);coefficients(&p,&a,&b);}}}
}
