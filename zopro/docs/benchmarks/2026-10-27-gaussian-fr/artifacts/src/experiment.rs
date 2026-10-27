use crate::{model::{Params,Z},native::G8};
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use zopro_bn254::{Fr,FrConfig,ifma8::Ifma8};
type F8=Ifma8<FrConfig>;

trait Arithmetic:Copy {
 fn add(self,b:Self)->Self;fn sub(self,b:Self)->Self;fn mul(self,b:Self)->Self;fn rot(self,i:Self)->Self;
}
impl Arithmetic for F8 {
 #[inline(always)]fn add(self,b:Self)->Self{F8::add(&self,&b)}
 #[inline(always)]fn sub(self,b:Self)->Self{F8::sub(&self,&b)}
 #[inline(always)]fn mul(self,b:Self)->Self{F8::mul(&self,&b)}
 #[inline(always)]fn rot(self,i:Self)->Self{F8::mul(&self,&i)}
}
impl Arithmetic for G8 {
 #[inline(always)]fn add(self,b:Self)->Self{unsafe{G8::add(self,b)}}
 #[inline(always)]fn sub(self,b:Self)->Self{unsafe{G8::sub(self,b)}}
 #[inline(always)]fn mul(self,b:Self)->Self{unsafe{G8::mul(self,b)}}
 #[inline(always)]fn rot(self,_:Self)->Self{unsafe{G8::rot(self)}}
}

#[inline(never)]fn chain<T:Arithmetic>(mut x:T,b:T,c:T,n:usize)->T{for _ in 0..n{x=x.mul(b).add(c);}std::hint::black_box(x)}
#[inline(never)]fn streams<T:Arithmetic>(mut x:[T;4],b:T,c:T,n:usize)->[T;4]{for _ in 0..n{for v in &mut x{*v=v.mul(b).add(c);}}std::hint::black_box(x)}
#[inline(always)]fn butterfly<T:Arithmetic,const RADIX4:bool>(x:[T;4],w:T,w2:T,w3:T,iw:T,i:T)->[T;4]{
 let[a,b,c,d]=x;let s0=a.add(c);let s1=b.add(d);let d0=a.sub(c);let d1=b.sub(d);
 if RADIX4 {let di=d1.rot(i);[s0.add(s1),s0.sub(s1).mul(w2),d0.add(di).mul(w),d0.sub(di).mul(w3)]}
 else {let c=d0.mul(w);let d=d1.mul(iw);[s0.add(s1),s0.sub(s1).mul(w2),c.add(d),c.sub(d).mul(w2)]}
}
#[inline(never)]fn fft<T:Arithmetic,const RADIX4:bool>(mut x:[T;4],w:T,w2:T,w3:T,iw:T,i:T,c:T,n:usize)->[T;4]{for _ in 0..n{x=butterfly::<T,RADIX4>(x,w,w2,w3,iw,i);x[0]=x[0].add(c);}std::hint::black_box(x)}

pub struct Input {
 pub x:[[Fr;8];4],pub b:[Fr;8],pub c:[Fr;8],pub w:[Fr;8],pub w2:[Fr;8],pub w3:[Fr;8],pub iw:[Fr;8],pub i:Fr,pub params:Params,
}
fn field(v:&BigInt)->Fr{let(_,bytes)=v.to_bytes_le();let mut out=[0u8;32];out[..bytes.len()].copy_from_slice(&bytes);Fr::from_le_bytes_mod_order(&out)}
impl Input {
 pub fn new()->Self{let params=Params::new();let i=field(&params.i);assert_eq!(i*i,-Fr::ONE);let root=Fr::root_of_unity(1024).unwrap();let w=std::array::from_fn(|k|root.pow(&[(k*13+7)as u64]));let mut seed=0x3c6ef372fe94f82bu64;let mut next=||{let mut out=[0u8;32];for bytes in out.chunks_mut(8){seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;bytes.copy_from_slice(&seed.to_le_bytes());}Fr::from_le_bytes_mod_order(&out)};let x=std::array::from_fn(|_|std::array::from_fn(|_|next()));let b=std::array::from_fn(|_|next());let c=std::array::from_fn(|_|next());Self{x,b,c,w,w2:w.map(|v|v*v),w3:w.map(|v|v*v*v),iw:w.map(|v|v*i),i,params}}
 pub fn ordinary(&self)->Ordinary {Ordinary{x:self.x.map(|v|F8::new(&v)),b:F8::new(&self.b),c:F8::new(&self.c),w:F8::new(&self.w),w2:F8::new(&self.w2),w3:F8::new(&self.w3),iw:F8::new(&self.iw),i:F8::new(&[self.i;8])}}
 pub fn gaussian(&self)->Gaussian {let encode=|a:[Fr;8]|unsafe{G8::new(a.map(|v|{let x=BigInt::from_bytes_le(num_bigint::Sign::Plus,&v.to_canonical().into_iter().flat_map(u64::to_le_bytes).collect::<Vec<_>>());let z=self.params.encode(&x);(z.re.to_i128().unwrap(),z.im.to_i128().unwrap())}))};Gaussian{x:self.x.map(encode),b:encode(self.b),c:encode(self.c),w:encode(self.w),w2:encode(self.w2),w3:encode(self.w3),iw:encode(self.iw),i:encode([self.i;8])}}
 pub fn decode(&self,a:G8)->[Fr;8]{unsafe{a.get().map(|(a,b)|field(&self.params.decode(&Z::new(a,b))))}}
}
pub struct Ordinary {x:[F8;4],b:F8,c:F8,w:F8,w2:F8,w3:F8,iw:F8,i:F8}
pub struct Gaussian {x:[G8;4],b:G8,c:G8,w:G8,w2:G8,w3:G8,iw:G8,i:G8}
impl Ordinary {
 pub fn execute(&self,pattern:&str,direct:bool,n:usize)->Vec<F8>{let x=std::hint::black_box(self.x);match pattern{"chain"=>vec![chain(x[0],self.b,self.c,n)],"streams4"=>streams(x,self.b,self.c,n).into(),"butterfly"=>if direct{fft::<_,true>(x,self.w,self.w2,self.w3,self.iw,self.i,self.c,n).into()}else{fft::<_,false>(x,self.w,self.w2,self.w3,self.iw,self.i,self.c,n).into()},_=>panic!("pattern")}}
}
impl Gaussian {
 pub fn execute(&self,pattern:&str,direct:bool,n:usize)->Vec<G8>{let x=std::hint::black_box(self.x);match pattern{"chain"=>vec![chain(x[0],self.b,self.c,n)],"streams4"=>streams(x,self.b,self.c,n).into(),"butterfly"=>if direct{fft::<_,true>(x,self.w,self.w2,self.w3,self.iw,self.i,self.c,n).into()}else{fft::<_,false>(x,self.w,self.w2,self.w3,self.iw,self.i,self.c,n).into()},_=>panic!("pattern")}}
}

type Matrix=[[Fr;5];5];
fn matmul(a:&Matrix,b:&Matrix)->Matrix{std::array::from_fn(|i|std::array::from_fn(|j|(0..5).fold(Fr::ZERO,|s,k|s+a[i][k]*b[k][j])))}
pub fn reference(input:&Input,pattern:&str,n:usize)->Vec<[Fr;8]>{
 if pattern!="butterfly"{return (0..if pattern=="chain"{1}else{4}).map(|s|std::array::from_fn(|k|{let b=input.b[k];let c=input.c[k];let power=b.pow(&[n as u64]);let sum=if b==Fr::ONE{Fr::from(n as u64)}else{(power-Fr::ONE)*(b-Fr::ONE).inverse().unwrap()};input.x[s][k]*power+c*sum})).collect()}
 let mut out=vec![[Fr::ZERO;8];4];for lane in 0..8{let w=input.w[lane];let w2=input.w2[lane];let w3=input.w3[lane];let i=input.i;let mut m=[[Fr::ONE,Fr::ONE,Fr::ONE,Fr::ONE,input.c[lane]],[w2,-w2,w2,-w2,Fr::ZERO],[w,i*w,-w,-i*w,Fr::ZERO],[w3,-i*w3,-w3,i*w3,Fr::ZERO],[Fr::ZERO,Fr::ZERO,Fr::ZERO,Fr::ZERO,Fr::ONE]];let mut p=std::array::from_fn(|i|std::array::from_fn(|j|if i==j{Fr::ONE}else{Fr::ZERO}));let mut n=n;while n!=0{if n&1!=0{p=matmul(&p,&m);}m=matmul(&m,&m);n>>=1;}let x=[input.x[0][lane],input.x[1][lane],input.x[2][lane],input.x[3][lane],Fr::ONE];for row in 0..4{out[row][lane]=(0..5).fold(Fr::ZERO,|s,k|s+p[row][k]*x[k]);}}out
}
#[cfg(test)]mod tests{use super::*;#[test]fn all_outputs_and_affine_power_oracle_match(){let i=Input::new();let o=i.ordinary();let g=i.gaussian();for n in [0,1,2,3,17,64]{for p in ["chain","streams4","butterfly"]{for direct in [false,true]{let want=reference(&i,p,n);let actual:Vec<_>=o.execute(p,direct,n).into_iter().map(|v|v.get()).collect();let gaussian:Vec<_>=g.execute(p,direct,n).into_iter().map(|v|i.decode(v)).collect();assert_eq!(actual,want,"ordinary {p} n{n} direct{direct}");assert_eq!(gaussian,want,"gaussian {p} n{n} direct{direct}");}}}}}
