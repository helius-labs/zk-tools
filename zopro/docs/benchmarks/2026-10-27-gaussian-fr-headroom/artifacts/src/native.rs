//! Private eight-way Gaussian Montgomery arithmetic, radix 2^130; canonical coordinates <=2^126+3b.
use std::arch::x86_64::*;
use crate::constants::*;
type V=__m512i;
type D=[V;3];
type W=[V;5];
const MASK:u64=(1u64<<52)-1;
const TOP:u64=(1u64<<26)-1;

#[derive(Clone,Copy)]
pub struct G8 {pub re:D,pub im:D}

#[inline(always)] unsafe fn splat(x:u64)->V{_mm512_set1_epi64(x as i64)}
#[inline(always)] unsafe fn digits(x:u128)->D{[splat(x as u64&MASK),splat((x>>52)as u64&MASK),splat((x>>104)as u64&TOP)]}
#[inline(always)] unsafe fn zero()->V{splat(0)}
#[inline(always)] unsafe fn normalize3(mut t:D)->D{let mask=splat(MASK);for j in 0..2{t[j+1]=_mm512_add_epi64(t[j+1],_mm512_srai_epi64::<52>(t[j]));t[j]=_mm512_and_si512(t[j],mask);}t[2]=_mm512_and_si512(t[2],splat(TOP));t}
#[inline(always)] unsafe fn add3(a:D,b:D)->D{normalize3(std::array::from_fn(|j|_mm512_add_epi64(a[j],b[j])))}
#[inline(always)] unsafe fn sum3(a:D,b:D)->D{let mut t=std::array::from_fn(|j|_mm512_add_epi64(a[j],b[j]));for j in 0..2{t[j+1]=_mm512_add_epi64(t[j+1],_mm512_srli_epi64::<52>(t[j]));t[j]=_mm512_and_si512(t[j],splat(MASK));}t}
#[inline(always)] unsafe fn sub3(a:D,b:D)->D{normalize3(std::array::from_fn(|j|_mm512_sub_epi64(a[j],b[j])))}
#[inline(always)] unsafe fn neg3(a:D)->D{sub3([zero();3],a)}
#[inline(always)] unsafe fn sign(a:D)->u8{_mm512_cmpge_epu64_mask(a[2],splat(1<<25))}
#[inline(always)] unsafe fn masked_add(a:&mut D,b:D,m:u8){for j in 0..3{a[j]=_mm512_add_epi64(a[j],_mm512_maskz_mov_epi64(m,b[j]));}}
#[inline(always)] unsafe fn masked_sub(a:&mut D,b:D,m:u8){for j in 0..3{a[j]=_mm512_sub_epi64(a[j],_mm512_maskz_mov_epi64(m,b[j]));}}

#[inline(always)] unsafe fn mul_raw(a:D,b:D)->W{
 let mut t=[zero();5];for i in 0..3{for j in 0..3{let k=i+j;t[k]=_mm512_madd52lo_epu64(t[k],a[i],b[j]);if k+1<5{t[k+1]=_mm512_madd52hi_epu64(t[k+1],a[i],b[j]);}}}t
}
#[inline(always)] unsafe fn mul_raw_constant(a:D,b:u128)->W{
 let d=[b as u64&MASK,(b>>52)as u64&MASK,(b>>104)as u64&TOP];let mut t=[zero();5];for i in 0..3{for j in 0..3{if d[j]!=0{let k=i+j;let v=splat(d[j]);t[k]=_mm512_madd52lo_epu64(t[k],a[i],v);if k+1<5{t[k+1]=_mm512_madd52hi_epu64(t[k+1],a[i],v);}}}}t
}
#[inline(always)] unsafe fn mul_low(a:D,b:D)->D{
 let mut t=[zero();3];for i in 0..3{for j in 0..3{let k=i+j;if k<3{t[k]=_mm512_madd52lo_epu64(t[k],a[i],b[j]);}if k+1<3{t[k+1]=_mm512_madd52hi_epu64(t[k+1],a[i],b[j]);}}}normalize3(t)
}
#[inline(always)] unsafe fn complex_raw(a:G8,b:G8)->[W;2]{let p=mul_raw(a.re,b.re);let q=mul_raw(a.im,b.im);let t=mul_raw(sum3(a.re,a.im),sum3(b.re,b.im));[std::array::from_fn(|j|_mm512_sub_epi64(p[j],q[j])),std::array::from_fn(|j|_mm512_sub_epi64(_mm512_sub_epi64(t[j],p[j]),q[j]))]}
#[inline(always)] unsafe fn complex_constant(a:G8)->[W;2]{let p=mul_raw_constant(a.re,A);let q=mul_raw_constant(a.im,B);let t=mul_raw_constant(sum3(a.re,a.im),A+B);[std::array::from_fn(|j|_mm512_sub_epi64(p[j],q[j])),std::array::from_fn(|j|_mm512_sub_epi64(_mm512_sub_epi64(t[j],p[j]),q[j]))]}
#[inline(always)] unsafe fn low(t:W)->D{normalize3([t[0],t[1],t[2]])}
#[inline(always)] unsafe fn high(mut t:W)->D{
 for j in 0..4{t[j+1]=_mm512_add_epi64(t[j+1],_mm512_srai_epi64::<52>(t[j]));t[j]=_mm512_and_si512(t[j],splat(MASK));}t[4]=_mm512_and_si512(t[4],splat(MASK));
 [_mm512_and_si512(_mm512_or_si512(_mm512_srli_epi64::<26>(t[2]),_mm512_slli_epi64::<26>(t[3])),splat(MASK)),_mm512_and_si512(_mm512_or_si512(_mm512_srli_epi64::<26>(t[3]),_mm512_slli_epi64::<26>(t[4])),splat(MASK)),_mm512_srli_epi64::<26>(t[4])]
}
#[inline(always)] unsafe fn overflow(a:D)->(u8,u8){
 let q=_mm512_srai_epi64::<60>(_mm512_slli_epi64::<38>(a[2]));
 (_mm512_cmpgt_epi64_mask(q,zero()),_mm512_cmplt_epi64_mask(q,splat(u64::MAX)))
}
impl G8 {
 #[inline(always)] pub unsafe fn new(values:[(i128,i128);8])->Self{
  let load=|part:usize|{let words:[[u64;8];3]=std::array::from_fn(|j|std::array::from_fn(|k|{let n=if part==0{values[k].0}else{values[k].1};let v=n as u128;let w=(v>>(52*j))as u64;if j==2{(w&((1<<24)-1))|if n<0{3<<24}else{0}}else{w&MASK}}));words.map(|w|_mm512_loadu_si512(w.as_ptr().cast()))};Self{re:load(0),im:load(1)}
 }
 pub unsafe fn get(self)->[(i128,i128);8]{let unpack=|d:D|{let mut w=[[0u64;8];3];for j in 0..3{_mm512_storeu_si512(w[j].as_mut_ptr().cast(),d[j]);}std::array::from_fn::<_,8,_>(|k|{let x=(w[0][k]as u128|((w[1][k]as u128)<<52)|(((w[2][k]&((1<<24)-1))as u128)<<104))as i128;assert_eq!(w[2][k]>>24,if x<0{3}else{0},"not a signed128 coordinate");x})};let a=unpack(self.re);let b=unpack(self.im);std::array::from_fn(|i|(a[i],b[i]))}
 #[inline(always)] unsafe fn balance<const STEPS:usize>(mut self)->Self{
  let a=digits(A);let b=digits(B);
  for _ in 0..STEPS{let(pos,neg)=overflow(self.re);masked_sub(&mut self.re,a,pos);masked_add(&mut self.re,a,neg);masked_sub(&mut self.im,b,pos);masked_add(&mut self.im,b,neg);self.re=normalize3(self.re);}
  self.im=normalize3(self.im);
  for _ in 0..STEPS{let(pos,neg)=overflow(self.im);masked_add(&mut self.re,b,pos);masked_sub(&mut self.re,b,neg);masked_sub(&mut self.im,a,pos);masked_add(&mut self.im,a,neg);self.im=normalize3(self.im);}
  self.re=normalize3(self.re);self
 }
 #[inline(always)] pub unsafe fn add(self,b:Self)->Self{self.add_lazy(b).balance::<1>()}
 #[inline(always)] pub unsafe fn sub(self,b:Self)->Self{self.sub_lazy(b).balance::<1>()}
 /// Intermediate linear coordinates may be larger than signed128, bounded below R/2.
 #[inline(always)] pub unsafe fn add_lazy(self,b:Self)->Self{Self{re:add3(self.re,b.re),im:add3(self.im,b.im)}}
 #[inline(always)] pub unsafe fn sub_lazy(self,b:Self)->Self{Self{re:sub3(self.re,b.re),im:sub3(self.im,b.im)}}
 /// The complete all-sum plus recurrence constant is bounded by5H.
 #[inline(always)] pub unsafe fn finish_linear(self)->Self{self.balance::<3>()}
 /// Left coordinates<=4H; canonical twiddle coordinates<=H.
 #[inline(always)] pub unsafe fn mul_twiddle(self,b:Self)->Self{self.mul(b).balance::<1>()}
 #[inline(always)] pub unsafe fn rot(self)->Self{Self{re:neg3(self.im),im:self.re}}
 #[inline(always)] pub unsafe fn mul(self,b:Self)->Self{
  let mut c0=[zero();3];let mut c1=[zero();3];let ar=sign(self.re);let ai=sign(self.im);let br=sign(b.re);let bi=sign(b.im);
  masked_sub(&mut c0,b.re,ar);masked_sub(&mut c0,self.re,br);masked_add(&mut c0,b.im,ai);masked_add(&mut c0,self.im,bi);
  masked_sub(&mut c1,b.im,ar);masked_sub(&mut c1,self.re,bi);masked_sub(&mut c1,b.re,ai);masked_sub(&mut c1,self.im,br);
  let t=complex_raw(self,b);let tr=low(t[0]);let ti=low(t[1]);let p=mul_low(tr,KRE.map(|v|splat(v)));let q=mul_low(ti,KIM.map(|v|splat(v)));let v=mul_low(add3(tr,ti),KSUM.map(|v|splat(v)));let m=Self{re:sub3(p,q),im:sub3(sub3(v,p),q)};
  let mr=sign(m.re);let mi=sign(m.im);masked_sub(&mut c0,digits(A),mr);masked_add(&mut c0,digits(B),mi);masked_sub(&mut c1,digits(B),mr);masked_sub(&mut c1,digits(A),mi);
  let mn=complex_constant(m);let re=high(std::array::from_fn(|j|_mm512_add_epi64(t[0][j],mn[0][j])));let im=high(std::array::from_fn(|j|_mm512_add_epi64(t[1][j],mn[1][j])));
  Self{re:add3(re,normalize3(c0)),im:add3(im,normalize3(c1))}
 }
}

#[cfg(test)]mod tests{
 use super::*;use crate::model::{Params,Z,modulo};use num_bigint::{BigInt,Sign};use num_traits::{ToPrimitive,Signed};
 fn decode(p:&Params,x:(i128,i128))->BigInt{p.decode(&Z::new(x.0,x.1))}
 #[test]fn native_signed_corners_exact_montgomery_and_bounds(){let p=Params::new();let h=p.bound.to_i128().unwrap();let half=p.half.to_i128().unwrap();let vals=[0,1,-1,half,-half,h,-h];let mut cases=Vec::new();for a in vals{for b in vals{for c in vals{for d in vals{cases.push(((a,b),(c,d)));}}}}for chunk in cases.chunks(8){let input:Vec<_>=chunk.iter().copied().cycle().take(8).collect();unsafe{let a=G8::new(std::array::from_fn(|j|input[j].0));let b=G8::new(std::array::from_fn(|j|input[j].1));for(j,got)in a.mul(b).get().iter().enumerate(){let want=p.redc(&Z::new(input[j].0.0,input[j].0.1).mul(&Z::new(input[j].1.0,input[j].1.1)));assert_eq!(*got,(want.re.to_i128().unwrap(),want.im.to_i128().unwrap()),"lane{j} inputs{:?}",input[j]);assert!(BigInt::from(got.0).abs()<=p.bound&&BigInt::from(got.1).abs()<=p.bound);}let sum=a.add(b).get();let sub=a.sub(b).get();let rot=a.rot().get();for j in 0..8{let x=decode(&p,input[j].0);let y=decode(&p,input[j].1);assert_eq!(decode(&p,sum[j]),modulo(&x+&y,&p.prime));assert_eq!(decode(&p,sub[j]),modulo(&x-&y,&p.prime));assert_eq!(decode(&p,rot[j]),modulo(x*&p.i,&p.prime));}}}}
 #[test]fn native_fullwidth_changing_chains_match_integer_oracle(){let p=Params::new();let mut seed=0xbb67ae8584caa73bu64;let mut values:Vec<_>=(0..8).map(|i|BigInt::from(i+1)).collect();let encode=|v:&BigInt|{let z=p.encode(v);(z.re.to_i128().unwrap(),z.im.to_i128().unwrap())};let mut state=unsafe{G8::new(std::array::from_fn(|i|encode(&values[i])))};for _ in 0..4096{let next:Vec<_>=(0..8).map(|_|{let mut bytes=[0u8;32];for c in bytes.chunks_mut(8){seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;c.copy_from_slice(&seed.to_le_bytes());}modulo(BigInt::from_bytes_le(Sign::Plus,&bytes),&p.prime)}).collect();let b=unsafe{G8::new(std::array::from_fn(|i|encode(&next[i])))};state=unsafe{state.mul(b).add(b)};let got=unsafe{state.get()};for i in 0..8{values[i]=modulo(&values[i]*&next[i]+&next[i],&p.prime);assert_eq!(decode(&p,got[i]),values[i]);assert!(BigInt::from(got[i].0).abs()<=p.bound&&BigInt::from(got[i].1).abs()<=p.bound);}}}

 #[test]fn lazy_twiddle_and_linear_finish_match_exact_model(){let p=Params::new();let h=p.bound.to_i128().unwrap();let v=[0,1,-1,h,-h,HALF as i128,-(HALF as i128)];unsafe{for start in 0..v.len(){let inputs:[[(i128,i128);8];5]=std::array::from_fn(|j|std::array::from_fn(|k|(v[(start+j+k)%v.len()],v[(start+j*3+k+1)%v.len()])));let g=inputs.map(|x|G8::new(x));let linear=g[0].add_lazy(g[1]).add_lazy(g[2]).add_lazy(g[3]);let product=linear.mul_twiddle(g[4]).get();let finish=linear.add_lazy(g[4]).finish_linear().get();for k in 0..8{let z=(0..4).fold(Z::new(0,0),|z,j|z.add(&Z::new(inputs[j][k].0,inputs[j][k].1)));let b=Z::new(inputs[4][k].0,inputs[4][k].1);let expected=p.twiddle(&z,&b);assert_eq!(product[k],(expected.re.to_i128().unwrap(),expected.im.to_i128().unwrap()));let expected=p.finish(z.add(&b));assert_eq!(finish[k],(expected.re.to_i128().unwrap(),expected.im.to_i128().unwrap()));}}}}
}
