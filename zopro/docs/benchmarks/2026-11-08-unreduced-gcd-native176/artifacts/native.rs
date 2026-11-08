use zopro_bn254::{Field,Fp,FpConfig};
use core::hint::select_unpredictable;
const BITS:usize=31;
const MASK:u64=(1<<BITS)-1;
include!("integer-helpers.rs.inc");
include!("accepted-matrix.rs.inc");
const WORDS:usize=17;
#[derive(Clone,Copy,Debug)]
pub struct Wide {pub limbs:[u64;WORDS],pub len:usize,pub negative:bool}
impl Wide {
 pub fn small(x:i64)->Self {let mut limbs=[0;WORDS];limbs[0]=x.unsigned_abs();Self{limbs,len:1,negative:x<0}}
 fn trim(&mut self){while self.len>1 && self.limbs[self.len-1]==0 {self.len-=1;}if self.len==1 && self.limbs[0]==0{self.negative=false;}}
 pub fn linear(a:&Self,b:&Self,f:i64,g:i64)->Self {
  assert!(f!=i64::MIN && g!=i64::MIN);
  let (af,bg)=(f.unsigned_abs(),g.unsigned_abs());
  let (an,bn)=(a.negative^(f<0),b.negative^(g<0));
  let n=a.len.max(b.len);assert!(n<WORDS);
  let mut out=Self{limbs:[0;WORDS],len:n+1,negative:false};
  if an==bn {
   let mut carry=0u128;
   for i in 0..n {let t=a.limbs[i] as u128*af as u128+b.limbs[i] as u128*bg as u128+carry;out.limbs[i]=t as u64;carry=t>>64;}
   out.limbs[n]=carry as u64;out.negative=an;
  }else{
   let (sf,sg)=(if an{-(af as i128)}else{af as i128},if bn{-(bg as i128)}else{bg as i128});
   let mut carry=0i128;
   for i in 0..n {let t=a.limbs[i] as i128*sf+b.limbs[i] as i128*sg+carry;out.limbs[i]=t as u64;carry=t>>64;}
   out.limbs[n]=carry as u64;
   if carry<0 {out.negative=true;let mut carry=1u128;for w in &mut out.limbs[..out.len] {carry+=(!*w) as u128;*w=carry as u64;carry>>=64;}}
  }
  out.trim();out
 }
}
fn coefficients(c:&mut[Wide;2],m:[i64;4],first:&mut bool){
 if *first {*c=[Wide::small(m[0]),Wide::small(m[2])];*first=false;}
 else {let previous=*c;*c=[Wide::linear(&previous[0],&previous[1],m[0],m[1]),Wide::linear(&previous[0],&previous[1],m[2],m[3])];}
}
fn less(a:[u64;4],b:[u64;4])->bool {for i in(0..4).rev(){if a[i]!=b[i] {return a[i]<b[i]}}false}
fn subtract(a:[u64;4],b:[u64;4])->[u64;4]{let mut borrow=false;core::array::from_fn(|i|{let(x,b0)=a[i].overflowing_sub(b[i]);let(y,b1)=x.overflowing_sub(u64::from(borrow));borrow=b0|b1;y})}
pub fn correction<C:FpConfig>(value:Wide,mut k:usize)->Fp<C>{
 assert!(k>0 && k<=992);
 let mut a=[0u64;18];a[..value.len].copy_from_slice(&value.limbs[..value.len]);let mut n=value.len.max(4);
 while k!=0 {
  let shift=k.min(64);let mask=if shift==64{u64::MAX}else{(1u64<<shift)-1};let m=a[0].wrapping_mul(C::INV)&mask;
  let mut carry=0u128;
  for j in 0..4 {let t=a[j] as u128+m as u128*C::P[j] as u128+carry;a[j]=t as u64;carry=t>>64;}
  let mut j=4;while carry!=0 {assert!(j<a.len());let t=a[j] as u128+carry;a[j]=t as u64;carry=t>>64;j+=1;}n=n.max(j);
  assert_eq!(a[0]&mask,0);
  if shift==64 {a.copy_within(1..n,0);a[n-1]=0;n=n.saturating_sub(1).max(1);}
  else {for j in 0..n {a[j]=(a[j]>>shift)|(a.get(j+1).copied().unwrap_or(0)<<(64-shift));}}
  while n>1 && a[n-1]==0 {n-=1;}k-=shift;
 }
 assert!(a[4..].iter().all(|w|*w==0));let mut out=core::array::from_fn(|i|a[i]);
 if !less(out,C::P){out=subtract(out,C::P);}
 assert!(less(out,C::P));
 if value.negative && out!=[0;4]{out=subtract(C::P,out);}
 let scale=const{match Fp::<C>::from_canonical(C::R2){Some(x)=>x,None=>panic!("R2 is canonical")}};
 Fp::<C>::from_montgomery(out)*scale
}
pub fn inverse<C:FpConfig>(x:&Fp<C>)->Option<Fp<C>>{
 if x.0==[0;4] {return None;}
 let(mut a,mut b)=(x.0,C::P);let mut c=[Wide::small(1),Wide::small(0)];let mut first=true;
 let mut pending=None::<[i64;4]>;let mut rounds=0usize;
 while a!=[0;4] {
  let bits=bit_len(&a).max(bit_len(&b));
  if bits<=64 {
   if let Some(m)=pending.take(){coefficients(&mut c,m,&mut first);}
   if b[0]==1 {return Some(correction::<C>(c[1],rounds*31));}
   let(mut r0,mut r1)=(a[0],b[0]);let(mut f0,mut f1)=(1i64,0i64);let(mut g0,mut g1)=(0i64,1i64);
   loop {let q=r0/r1;let rem=r0%r1;if rem==0{assert_eq!(r1,1);break;}
    (r0,r1)=(r1,rem);(f0,f1)=(f1,f0-(q as i64)*f1);(g0,g1)=(g1,g0-(q as i64)*g1);
   }
   let end=Wide::linear(&c[0],&c[1],f1,g1);return Some(correction::<C>(end,rounds*31));
  }
  let(_,_,mut f,mut g,mut h,mut j)=zero_run_matrix(approximate(&a,bits),approximate(&b,bits));
  let old_a=a;a=integer_update(&old_a,&b,&mut f,&mut g);b=integer_update(&old_a,&b,&mut h,&mut j);
  if let Some(m)=pending.take(){let combined=[f*m[0]+g*m[2],f*m[1]+g*m[3],h*m[0]+j*m[2],h*m[1]+j*m[3]];coefficients(&mut c,combined,&mut first);}
  else{pending=Some([f,g,h,j]);}
  rounds+=1;assert!(rounds<=32,"batched GCD did not converge");
 }
 if let Some(m)=pending.take(){c[1]=if first{Wide::small(m[2])}else{Wide::linear(&c[0],&c[1],m[2],m[3])};}
 assert_eq!(b,[1,0,0,0]);Some(correction::<C>(c[1],rounds*31))
}
