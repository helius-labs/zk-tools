use num_bigint::{BigInt as Z,Sign};use serde_json::json;use sha2::{Digest,Sha256};use std::fs;
const ROOT:&str="/home/ubuntu/src/zopro-perf-work/late-centered176";
fn z(x:i64)->Z{Z::from(x)}fn modulo(x:&Z,m:&Z)->Z{let v=x%m;if v<z(0){v+m}else{v}}fn abs(x:&Z)->Z{if x<&z(0){-x}else{x.clone()}}
fn inverse(x:&Z,m:&Z)->Z{let(mut a,mut b,mut u,mut v)=(m.clone(),modulo(x,m),z(0),z(1));while b!=z(0){let q=&a/&b;(a,b)=(b.clone(),a-&q*&b);(u,v)=(v.clone(),u-q*v);}assert_eq!(a,z(1));modulo(&u,m)}
fn unsigned_digits(x:&Z)->[u64;5]{let b=z(1)<<52usize;let mut t=x.clone();assert!(t>=z(0));let out=core::array::from_fn(|_|{let low=modulo(&t,&b).to_string().parse().unwrap();t>>=52usize;low});assert_eq!(t,z(0));out}
fn polynomial(c:&[i128])->Z{c.iter().enumerate().map(|(i,&v)|Z::from(v)<<(i*52)).sum()}
struct Kernel{bits:usize,p:Z,r:Z,s:Z,pinv:Z,limbs:[u64;5],ninv:u64,max:i128,ifmas:u64,negative:u64,centers:u64,hash:Sha256}
impl Kernel{
 fn new(p:Z,bits:usize)->Self{let r=z(1)<<bits;let s=z(1)<<260usize;let pinv=inverse(&p,&r);let limbs=unsigned_digits(&p);let mut a=1u64;for _ in 0..6{a=a.wrapping_mul(2u64.wrapping_sub(limbs[0].wrapping_mul(a)));}assert_eq!(a.wrapping_mul(limbs[0]),1);assert!(&p*&p*z(100)+&p*&r<&s*&r);Self{bits,p,r,s,pinv,limbs,ninv:a.wrapping_neg(),max:0,ifmas:0,negative:0,centers:0,hash:Sha256::new()}}
 fn check(&mut self,t:&[i128]){for &v in t{assert!(v>=i64::MIN as i128&&v<=i64::MAX as i128);self.max=self.max.max(v.abs());}}
 fn cios(&mut self,a:[u64;5],b:[u64;5],c:[u64;5],d:[u64;5],fused:bool)->([i128;6],Z,bool){
  let mut t=[0i128;6];let mut qsum=z(0);let mut centered=false;let mask52=(1u128<<52)-1;
  for row in 0..5{
   for j in 0..5{let w=a[j]as u128*b[row]as u128;t[j]+=(w&mask52)as i128;t[j+1]+=(w>>52)as i128;self.ifmas+=2;self.check(&t);
    if fused{let w=c[j]as u128*d[row]as u128;t[j]+=(w&mask52)as i128;t[j+1]+=(w>>52)as i128;self.ifmas+=2;self.check(&t);}}
   let width=if row==4{self.bits-208}else{52};let mask=(1u64<<width)-1;
   let q=(t[0]as u64).wrapping_mul(self.ninv)&mask;self.ifmas+=1;qsum+=Z::from(q)<<(row*52);if row==4{centered=q>>(width-1)!=0;}
   let ceil=(t[0]+mask as i128)>>width;let low=ceil+((((q as u128)<<(52-width))*self.limbs[0]as u128)>>52)as i128;self.ifmas+=1;
   // This explicitly omits the cancelled low q*p0 product, as accepted CIOS does.
   let qref=modulo(&(-Z::from(t[0])*&self.pinv),&z(1).shl(width));assert_eq!(qref,Z::from(q));
   for j in 1..5{let w=q as u128*self.limbs[j]as u128;t[j]+=(w&mask52)as i128;t[j+1]+=(w>>52)as i128;self.ifmas+=2;self.check(&t);}
   if width==52{t[1]+=low;self.check(&t);t=[t[1],t[2],t[3],t[4],t[5],0];}
   else{let m=mask as i128;t=[low+((t[1]&m)<<4),(t[1]>>48)+((t[2]&m)<<4),(t[2]>>48)+((t[3]&m)<<4),(t[3]>>48)+((t[4]&m)<<4),(t[4]>>48)+(t[5]<<4),0];self.check(&t);}
  }(t,qsum,centered)
 }
 fn run(&mut self,tuple:&[Z;4],sign:i64){
  let actual=[tuple[0].clone(),tuple[1].clone(),if sign<0{-&tuple[2]}else{tuple[2].clone()},tuple[3].clone()];
  let u=actual.iter().map(|v|modulo(v,&self.s)).collect::<Vec<_>>();let digits=u.iter().map(unsigned_digits).collect::<Vec<_>>();let neg=actual.iter().map(|v|v<&z(0)).collect::<Vec<_>>();
  let total=&actual[0]*&actual[1]+if sign==0{z(0)}else{&actual[2]*&actual[3]};let unsigned=&u[0]*&u[1]+if sign==0{z(0)}else{&u[2]*&u[3]};
  let before=self.ifmas;let(mut t,mut q,centered)=self.cios(digits[0],digits[1],digits[2],digits[3],sign!=0);
  assert_eq!(self.ifmas-before,if sign==0{100}else{150});assert_eq!(polynomial(&t),(&unsigned+&q*&self.p)/&self.r);
  let qr=modulo(&(-&unsigned*&self.pinv),&self.r);assert_eq!(qr,q);assert_eq!(modulo(&(-&total*&self.pinv),&self.r),q);
  let scale=if self.bits==256{16}else{1};
  for(pair,(left,right))in[(0usize,1usize),(2,3)].iter().enumerate(){if pair==1&&sign==0{continue;}for(source,other)in[(*left,*right),(*right,*left)]{if neg[source]{for j in 0..5{t[j]-=scale*digits[other][j]as i128;self.check(&t);}}}}
  if centered{q-=&self.r;for j in 0..5{t[j]-=self.limbs[j]as i128;self.check(&t);}self.centers+=1;}
  for j in 0..4{let carry=t[j]>>52;t[j]&=(1i128<<52)-1;t[j+1]+=carry;self.check(&t);}
  t[4]&=(1i128<<52)-1;if t[4]>=1i128<<51{t[4]-=1i128<<52;}
  let out=polynomial(&t);let reference=(&total+&q*&self.p)/&self.r;assert_eq!(out,reference);assert_eq!(&out*&self.r,&total+&q*&self.p);assert!(abs(&out)<(&self.s>>1usize));
  assert_eq!(modulo(&out,&self.p),modulo(&(&total*inverse(&self.r,&self.p)),&self.p));
  self.negative+=u64::from(out<z(0));self.hash.update(out.to_string().as_bytes());self.hash.update([0]);
 }
}
struct Rng(u64);impl Rng{fn next(&mut self)->u64{self.0^=self.0<<13;self.0^=self.0>>7;self.0^=self.0<<17;self.0}}
use std::ops::Shl;
fn main(){assert!(!std::path::Path::new(&format!("{ROOT}/compensation-result.json")).exists());let mut rows=vec![];
 for(field,prime)in[("Fq","21888242871839275222246405745257275088696311157297823662689037894645226208583"),("Fr","21888242871839275222246405745257275088548364400416034343698204186575808495617")]{let p:Z=prime.parse().unwrap();let h=&p>>1usize;let edges=vec![-&p,-&p+z(1),-&h,z(-1),z(0),z(1),h,&p-z(1),p.clone(),-&p*z(2),&p*z(2),-&p*z(5),&p*z(5)];
  for bits in[256,260]{for sign in[0,1,-1]{let mut kernel=Kernel::new(p.clone(),bits);let mut rng=Rng(0xe0f3a5a7891123cc);let mut count=0;
   for(i,a)in edges.iter().enumerate(){for(j,b)in edges.iter().enumerate(){kernel.run(&[a.clone(),b.clone(),edges[(i+j*2)%13].clone(),edges[(i*5+j)%13].clone()],sign);count+=1;}}
   for _ in 0..1024{let tuple=core::array::from_fn(|_|{let bytes=(0..4).flat_map(|_|rng.next().to_le_bytes()).collect::<Vec<_>>();let v=Z::from_bytes_le(Sign::Plus,&bytes)%(&p*z(5));if rng.next()&1!=0{-v}else{v}});kernel.run(&tuple,sign);count+=1;}
   rows.push(json!({"field":field,"bits":bits,"operation":if sign==0{"ordinary"}else if sign==1{"sum2"}else{"difference2"},"outputs":count,"ifma_recipe_total":kernel.ifmas,"ifma_recipe_per_output":if sign==0{100}else{150},"max_signed_coefficient_abs":kernel.max.to_string(),"negative_outputs":kernel.negative,"centered_quotients":kernel.centers,"sha256":format!("{:x}",kernel.hash.finalize())}));
  }}
 }
 let result=json!({"late_compensation_passed":true,"rows":rows,"old_mathematical_model_reexecuted":false,"new_exact_outputs":14316,"original_unsigned_CIOS_schedule_with_late_compensation":true,"native_executed":false,"timings":0,"new_proofs":0,"canonical_unchanged":35171,"selects_native_implementation_and_codegen_qualification":true,"selects_performance_or_SDK":false,"adopted":false,"goal_complete":false});fs::write(format!("{ROOT}/compensation-result.json"),serde_json::to_vec_pretty(&result).unwrap()).unwrap();println!("New late-compensation lowering PASS14316outputs; unsigned100/150IFMA recipe plus charged sign/linear/export work; no native timing");
}
