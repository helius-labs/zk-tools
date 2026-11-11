use num_bigint::BigInt as I;
use num_traits::{Zero,One,Signed,ToPrimitive};
use serde_json::json;
#[derive(Clone,Debug)]struct Q{ x:I,y:I }
fn q(x:I,y:I)->Q{Q{x,y}}
fn add(a:&Q,b:&Q)->Q{q(&a.x+&b.x,&a.y+&b.y)}
fn sub(a:&Q,b:&Q)->Q{q(&a.x-&b.x,&a.y-&b.y)}
fn mul(a:&Q,b:&Q)->Q{
 let v0=&a.x*&b.x;let v1=&a.y*&b.y;let v2=(&a.x+&a.y)*(&b.x+&b.y);
 q(&v0-3*&v1,v2-v0-v1)
}
fn rem(x:&I,m:&I)->I{let r=x%m;if r.is_negative(){r+m}else{r}}
fn inverse(x:&I,m:&I)->I{
 let(mut r0,mut r1)=(m.clone(),rem(x,m));let(mut t0,mut t1)=(I::zero(),I::one());
 while !r1.is_zero(){let d=&r0/&r1;(r0,r1)=(r1.clone(),&r0-&d*&r1);(t0,t1)=(t1.clone(),&t0-d*&t1);}
 assert_eq!(r0,I::one());rem(&t0,m)
}
struct Ring{p:I,a:I,u:I,r:I,h:I,root:I,ri:I,neg_inv:Q}
impl Ring{
 fn new()->Self{
  let u=I::parse_bytes(b"4965661367192848881",10).unwrap();let a:I=6*&u*&u+3*&u+1;let p:I=&a*&a+3*&u*&u;
  assert_eq!(p,I::parse_bytes(b"21888242871839275222246405745257275088696311157297823662689037894645226208583",10).unwrap());
  let r=I::one()<<131usize;let h=I::one()<<127usize;let root=rem(&(-&a*inverse(&u,&p)),&p);assert_eq!(rem(&(&root*&root+3),&p),I::zero());
  let pi=inverse(&p,&r);let neg_inv=q(rem(&(-&a*&pi),&r),rem(&(&u*&pi),&r));let check=mul(&q(a.clone(),u.clone()),&neg_inv);assert_eq!(rem(&check.x,&r),&r-1);assert_eq!(rem(&check.y,&r),I::zero());
  let ri=inverse(&r,&p);Self{p,a,u,r,h,root,ri,neg_inv}
 }
 fn center_mod(&self,x:&I)->I{let half=&self.r>>1usize;rem(&(x+&half),&self.r)-half}
 fn canonical(&self,v:&Q)->Q{
  let nearest=|v:I|{let t=v+(&self.p>>1usize);(&t-rem(&t,&self.p))/&self.p};
  let k0=nearest(&self.a*&v.x+3*&self.u*&v.y);let k1=nearest(&self.a*&v.y-&self.u*&v.x);
  let z=q(&v.x-&self.a*&k0+3*&self.u*&k1,&v.y-&self.u*&k0-&self.a*&k1);
  assert!(2*z.x.abs()<=&self.a+3*&self.u);assert!(2*z.y.abs()<=&self.a+&self.u);self.check(&z);z
 }
 fn encode(&self,v:&I)->Q{self.canonical(&q(rem(&(v*&self.r),&self.p),I::zero()))}
 fn decode(&self,v:&Q)->I{rem(&((&v.x+&self.root*&v.y)*&self.ri),&self.p)}
 fn check(&self,v:&Q){assert!(v.x>=-&self.h&&v.x<self.h);assert!(v.y>=-&self.h&&v.y<self.h);assert!(v.x.to_i128().is_some()&&v.y.to_i128().is_some());}
 fn redc(&self,t:&Q)->Q{
  let low=q(rem(&t.x,&self.r),rem(&t.y,&self.r));let raw=mul(&low,&self.neg_inv);let k=q(self.center_mod(&raw.x),self.center_mod(&raw.y));let numerator=add(t,&mul(&k,&q(self.a.clone(),self.u.clone())));
  assert_eq!(rem(&numerator.x,&self.r),I::zero());assert_eq!(rem(&numerator.y,&self.r),I::zero());let z=q(&numerator.x/&self.r,&numerator.y/&self.r);self.check(&z);z
 }
 fn product(&self,a:&Q,b:&Q)->Q{self.redc(&mul(a,b))}
 fn fused(&self,a:&Q,b:&Q,c:&Q,d:&Q,minus:bool)->Q{self.redc(&if minus{sub(&mul(a,b),&mul(c,d))}else{add(&mul(a,b),&mul(c,d))})}
}
fn next(s:&mut u64)->u64{*s^=*s<<13;*s^=*s>>7;*s^=*s<<17;*s}
fn random(s:&mut u64)->I{let lo=next(s)as u128;let hi=next(s)as u128;I::from((lo|(hi<<64))as i128)}
fn main(){
 let f=Ring::new();let h2=&f.h*&f.h;
 // Integer cross-multiplication proves strict full-box closure, including fused2.
 let real_num:I=16*&h2+&f.r*(&f.a+3*&f.u);let imag_num:I=8*&h2+&f.r*(&f.a+&f.u);
 assert!(real_num<2*&f.r*&f.h);assert!(imag_num<2*&f.r*&f.h);
 let ordinary_real_num:I=8*&h2+&f.r*(&f.a+3*&f.u);assert!(ordinary_real_num<2*&f.r*&f.h);
 let mut values=vec![-&f.h,-&f.h+1,-&f.a,-(&f.a>>1usize),-I::one(),I::zero(),I::one(),&f.a>>1usize,f.a.clone(),&f.h-1];values.sort();values.dedup();
 let mut points=vec![];for x in &values{for y in &values{points.push(q(x.clone(),y.clone()));}}
 let mut seed=0x64931b85a7c2d0efu64;for _ in 0..4096{points.push(q(random(&mut seed),random(&mut seed)));}
 let mut ordinary=0;let mut fused=0;let mut codec=0;
 for (i,a) in points.iter().enumerate(){
  f.check(a);let a0=f.decode(a);let canon=f.encode(&a0);assert_eq!(f.decode(&canon),a0);codec+=1;
  let b=&points[(i*37+19)%points.len()];let c=&points[(i*61+43)%points.len()];let d=&points[(i*97+7)%points.len()];let b0=f.decode(b);let c0=f.decode(c);let d0=f.decode(d);
  let product=f.product(a,b);assert_eq!(f.decode(&product),rem(&(&a0*&b0),&f.p));ordinary+=1;
  for minus in[false,true]{let got=f.fused(a,b,c,d,minus);let expected=if minus{&a0*&b0-&c0*&d0}else{&a0*&b0+&c0*&d0};assert_eq!(f.decode(&got),rem(&expected,&f.p));fused+=1;}
  for v in [add(a,b),sub(a,b)]{let before=f.decode(&v);let after=f.canonical(&v);assert_eq!(f.decode(&after),before);codec+=1;}
 }
 // Every ±H boundary orientation, using H-1 for the positive endpoint.
 let edge=[-&f.h,&f.h-1];let mut corners=0;
 for ax in &edge{for ay in &edge{for bx in &edge{for by in &edge{
  let a=q(ax.clone(),ay.clone());let b=q(bx.clone(),by.clone());let aa=f.decode(&a);let bb=f.decode(&b);assert_eq!(f.decode(&f.product(&a,&b)),rem(&(&aa*&bb),&f.p));ordinary+=1;
  for minus in[false,true]{let z=f.fused(&a,&b,&b,&a,minus);assert_eq!(f.decode(&z),if minus{I::zero()}else{rem(&(2*&aa*&bb),&f.p)});corners+=1;}
 }}}}
 let mut state=f.encode(&I::from(19));let mut expected=I::from(19);let mut chain=0;
 for i in 0..8192{let b=&points[(i*71+17)%points.len()];let c=&points[(i*29+3)%points.len()];let bb=f.decode(b);let cc=f.decode(c);
  state=match i%4{0=>{expected=rem(&(&expected*&bb),&f.p);f.product(&state,b)},1=>{expected=rem(&(&expected+&bb),&f.p);f.canonical(&add(&state,b))},2=>{expected=rem(&(&expected-&bb),&f.p);f.canonical(&sub(&state,b))},_=>{expected=rem(&(&expected*&bb-&cc*&bb),&f.p);f.fused(&state,b,c,b,true)}};assert_eq!(f.decode(&state),expected);chain+=1;
 }
 // Fq2 products/norms, with its independent ordinary complex modular oracle.
 let mut extension=0;for i in 0..1024{let a=&points[(i*3)%points.len()];let b=&points[(i*7+1)%points.len()];let c=&points[(i*11+2)%points.len()];let d=&points[(i*13+3)%points.len()];let aa=f.decode(a);let bb=f.decode(b);let cc=f.decode(c);let dd=f.decode(d);
  let real=f.fused(a,c,b,d,true);let imag=f.fused(a,d,b,c,false);let norm=f.fused(a,a,b,b,false);
  assert_eq!(f.decode(&real),rem(&(&aa*&cc-&bb*&dd),&f.p));assert_eq!(f.decode(&imag),rem(&(&aa*&dd+&bb*&cc),&f.p));assert_eq!(f.decode(&norm),rem(&(&aa*&aa+&bb*&bb),&f.p));extension+=3;
 }
 // Explicit radix52 schoolbook site counts. Last high half is known zero.
 let full=|m:usize,n:usize|2*m*n-1;let low131=(0..3usize).flat_map(|i|(0..3).map(move|j|(i,j))).map(|(i,j)|usize::from(52*(i+j)<131)+usize::from(52*(i+j+1)<131)).sum::<usize>();assert_eq!(low131,9);
 let raw=3*full(3,3);let quotient=3*low131;let constant=2*full(3,3)+full(3,2);let product_sites=raw+quotient+constant;let fused_sites=2*raw+quotient+constant;assert_eq!((product_sites,fused_sites),(123,174));
 let out=json!({"passed":true,"p":f.p.to_string(),"u":f.u.to_string(),"a":f.a.to_string(),"root":f.root.to_string(),"R":f.r.to_string(),"H":f.h.to_string(),"coefficient_storage_bytes":32,"ordinary_vectors":6,"current_vectors":5,"bounds":{"fused_real_numerator":real_num.to_string(),"fused_imag_numerator":imag_num.to_string(),"strict_limit":(I::from(2)*&f.r*&f.h).to_string()},"checks":{"input_pairs":points.len(),"ordinary":ordinary,"fused":fused,"codec_and_linear":codec,"fused_corner_cases":corners,"changing_chain":chain,"Fq2_components":extension},"IFMA_recipe":{"raw_quadratic":raw,"low_quotient":quotient,"constant_modulus":constant,"ordinary":product_sites,"fused2":fused_sites,"current_ordinary":100,"current_fused2":150},"native_selected":false,"unpriced":["signed lowering","constant factors","carry chains","canonical recentering","input and output conversion","packing","inversion bridge","memory and scheduling"],"new_proofs":0,"production_changed":false,"goal_complete":false});println!("{}",serde_json::to_string_pretty(&out).unwrap());
}
