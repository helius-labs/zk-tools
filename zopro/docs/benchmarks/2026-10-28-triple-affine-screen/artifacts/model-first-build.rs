use ark_bn254::{Fq,Fq2};use ark_ff::Field;use std::{cell::Cell,marker::PhantomData};
#[derive(Clone,Copy,Default,Debug,PartialEq,Eq)]pub struct Cost{pub mul:u64,pub square:u64,pub fused:u64,pub linear:u64,pub inverse:u64}
impl Cost{pub fn elementary(&self)->u64{self.mul+self.square+2*self.fused}pub fn ifma_sites(&self)->u64{100*(self.mul+self.square)+150*self.fused}}
pub trait PointField:Field{
    fn mul_count(a:Self,b:Self,c:&mut Cost)->Self;fn sq_count(a:Self,c:&mut Cost)->Self;
    fn plus_count(a:Self,b:Self,c:&mut Cost)->Self;fn minus_count(a:Self,b:Self,c:&mut Cost)->Self;
    fn sop(a:Self,b:Self,d:Self,e:Self,c:&mut Cost)->Self;fn minus_square(a:Self,b:Self,n:Self,c:&mut Cost)->Self;
    fn norm(a:Self,c:&mut Cost)->Fq;fn norm_inverse(a:Self,f:Fq,c:&mut Cost)->Self;
}
impl PointField for Fq{
fn mul_count(a:Self,b:Self,c:&mut Cost)->Self{c.mul+=1;a*b}fn sq_count(a:Self,c:&mut Cost)->Self{c.square+=1;a.square()}
fn plus_count(a:Self,b:Self,c:&mut Cost)->Self{c.linear+=1;a+b}fn minus_count(a:Self,b:Self,c:&mut Cost)->Self{c.linear+=1;a-b}
fn sop(a:Self,b:Self,d:Self,e:Self,c:&mut Cost)->Self{c.fused+=1;a*b+d*e}fn minus_square(a:Self,b:Self,n:Self,c:&mut Cost)->Self{c.fused+=1;a*b-n*n}
fn norm(a:Self,_:&mut Cost)->Fq{a}fn norm_inverse(_:Self,f:Fq,_:&mut Cost)->Self{f}
}
impl PointField for Fq2{
fn mul_count(a:Self,b:Self,c:&mut Cost)->Self{c.fused+=2;Self::new(a.c0*b.c0-a.c1*b.c1,a.c0*b.c1+a.c1*b.c0)}
fn sq_count(a:Self,c:&mut Cost)->Self{c.mul+=2;c.linear+=3;Self::new((a.c0+a.c1)*(a.c0-a.c1),(a.c0*a.c1).double())}
fn plus_count(a:Self,b:Self,c:&mut Cost)->Self{c.linear+=2;a+b}fn minus_count(a:Self,b:Self,c:&mut Cost)->Self{c.linear+=2;a-b}
fn sop(a:Self,b:Self,d:Self,e:Self,c:&mut Cost)->Self{Self::plus_count(Self::mul_count(a,b,c),Self::mul_count(d,e,c),c)}
fn minus_square(a:Self,b:Self,n:Self,c:&mut Cost)->Self{Self::minus_count(Self::mul_count(a,b,c),Self::sq_count(n,c),c)}
fn norm(a:Self,c:&mut Cost)->Fq{c.fused+=1;a.c0.square()+a.c1.square()}
fn norm_inverse(a:Self,f:Fq,c:&mut Cost)->Self{c.mul+=2;Self::new(a.c0*f,-a.c1*f)}
}
pub struct Ops<F:PointField>{pub cost:Cell<Cost>,p:PhantomData<F>}
impl<F:PointField>Ops<F>{pub fn new()->Self{Self{cost:Cell::new(Cost::default()),p:PhantomData}}fn apply(&self,f:impl FnOnce(&mut Cost)->F)->F{let mut c=self.cost.get();let v=f(&mut c);self.cost.set(c);v}
pub fn add(&self,a:F,b:F)->F{self.apply(|c|F::plus_count(a,b,c))}pub fn sub(&self,a:F,b:F)->F{self.apply(|c|F::minus_count(a,b,c))}pub fn mul(&self,a:F,b:F)->F{self.apply(|c|F::mul_count(a,b,c))}pub fn sq(&self,a:F)->F{self.apply(|c|F::sq_count(a,c))}pub fn sop(&self,a:F,b:F,d:F,e:F)->F{self.apply(|c|F::sop(a,b,d,e,c))}pub fn minus_square(&self,a:F,b:F,n:F)->F{self.apply(|c|F::minus_square(a,b,n,c))}
pub fn norm(&self,a:F)->Fq{let mut c=self.cost.get();let v=F::norm(a,&mut c);self.cost.set(c);v}pub fn denominator_inverse(&self,a:F,f:Fq)->F{self.apply(|c|F::norm_inverse(a,f,c))}
pub fn base_mul(&self,a:Fq,b:Fq)->Fq{let mut c=self.cost.get();c.mul+=1;self.cost.set(c);a*b}pub fn base_inv(&self,a:Fq)->Fq{let mut c=self.cost.get();c.inverse+=1;self.cost.set(c);a.inverse().unwrap()}
}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct Point<F:Field>{pub x:F,pub y:F,pub infinity:bool}
impl<F:Field>Point<F>{pub fn infinity()->Self{Self{x:F::ZERO,y:F::ZERO,infinity:true}}pub fn finite(x:F,y:F)->Self{Self{x,y,infinity:false}}}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum Guard{Infinity,FirstDenominator,SecondDenominator}
pub struct Prepared<F:PointField>{pub input:[Point<F>;3],pub denominator:F,pub numerator1:F,pub numerator2:F}
pub fn prepare<F:PointField>(s:[Point<F>;3],op:&Ops<F>)->Result<Prepared<F>,Guard>{if s.iter().any(|p|p.infinity){return Err(Guard::Infinity)}let[a,b,r]=s;let d=op.sub(b.x,a.x);if d==F::ZERO{return Err(Guard::FirstDenominator)}let n=op.sub(b.y,a.y);let d2=op.sq(d);let h=op.minus_square(d2,op.add(op.add(a.x,b.x),r.x),n);if h==F::ZERO{return Err(Guard::SecondDenominator)}let u=op.sop(d,op.add(a.y,r.y),n,op.sub(r.x,a.x));let denominator=op.mul(d,h);let numerator1=op.mul(n,h);let numerator2=op.mul(d2,u);Ok(Prepared{input:s,denominator,numerator1,numerator2})}
pub fn direct_batch<F:PointField>(input:&[[Point<F>;3]],op:&Ops<F>)->Result<Vec<Point<F>>,Guard>{let mut records=Vec::with_capacity(input.len());let mut chain=Fq::ONE;for s in input{let p=prepare(*s,op)?;let norm=op.norm(p.denominator);let prefix=chain;chain=op.base_mul(chain,norm);records.push((p,prefix,norm));}if records.is_empty(){return Ok(Vec::new())}let mut inverse=op.base_inv(chain);let mut out=vec![Point::infinity();input.len()];for(i,(p,prefix,norm))in records.into_iter().enumerate().rev(){let f=op.base_mul(prefix,inverse);inverse=op.base_mul(inverse,norm);let dinv=op.denominator_inverse(p.denominator,f);let l=op.mul(p.numerator1,dinv);let w=op.mul(p.numerator2,dinv);let k=op.sub(w,l);let[a,b,r]=p.input;let xs=op.sub(op.sub(op.sq(l),a.x),b.x);let x=op.sub(op.sub(op.sq(k),xs),r.x);let y=op.add(op.sop(w,op.sub(xs,x),l,op.sub(x,a.x)),a.y);out[i]=Point::finite(x,y);}Ok(out)}
pub fn binary_batch<F:PointField>(input:&[(Point<F>,Point<F>)],op:&Ops<F>)->Result<Vec<Point<F>>,Guard>{let mut records=Vec::with_capacity(input.len());let mut chain=Fq::ONE;for(a,b)in input{if a.infinity||b.infinity{return Err(Guard::Infinity)}let d=op.sub(b.x,a.x);if d==F::ZERO{return Err(Guard::FirstDenominator)}let dy=op.sub(b.y,a.y);let prefix=chain;let norm=op.norm(d);chain=op.base_mul(chain,norm);records.push((*a,*b,d,dy,prefix,norm));}if records.is_empty(){return Ok(Vec::new())}let mut inverse=op.base_inv(chain);let mut out=vec![Point::infinity();input.len()];for(i,(a,b,d,dy,prefix,norm))in records.into_iter().enumerate().rev(){let f=op.base_mul(prefix,inverse);inverse=op.base_mul(inverse,norm);let dinv=op.denominator_inverse(d,f);let l=op.mul(dy,dinv);let x=op.sub(op.sub(op.sq(l),a.x),b.x);let y=op.sub(op.mul(l,op.sub(a.x,x)),a.y);out[i]=Point::finite(x,y);}Ok(out)}
pub fn two_binary<F:PointField>(input:&[[Point<F>;3]],op:&Ops<F>)->Result<Vec<Point<F>>,Guard>{let first=binary_batch(&input.iter().map(|s|(s[0],s[1])).collect::<Vec<_>>(),op)?;binary_batch(&first.into_iter().zip(input.iter().map(|s|s[2])).collect::<Vec<_>>(),op)}

#[cfg(test)]mod tests{use super::*;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]struct Tiny{ x:u64,y:u64 }
fn tadd(a:Option<Tiny>,b:Option<Tiny>)->Option<Tiny>{let(a,b)=match(a,b){(None,b)=>return b,(a,None)=>return a,(Some(a),Some(b))=>(a,b)};if a.x==b.x&&(a.y+b.y)%17==0{return None}let(n,d)=if a==b{(3*a.x*a.x%17,2*a.y%17)}else{((b.y+17-a.y)%17,(b.x+17-a.x)%17)};let inv=(1..17).find(|i|d*i%17==1).unwrap();let l=n*inv%17;let x=(l*l+34-a.x-b.x)%17;let y=(l*((a.x+17-x)%17)+17-a.y)%17;Some(Tiny{x,y})}
#[test]fn exhaustive_tiny_three_point_formula_and_guards(){let mut points=vec![None];for x in 0..17{for y in 0..17{if y*y%17==(x*x*x+3)%17{points.push(Some(Tiny{x,y}));}}}let mut ordinary=0;let mut guards=[0;3];for a in &points{for b in &points{for r in &points{let want=tadd(tadd(*a,*b),*r);let(a,b,r)=match(a,b,r){(Some(a),Some(b),Some(r))=>(*a,*b,*r),_=>{guards[0]+=1;continue}};let d=(b.x+17-a.x)%17;if d==0{guards[1]+=1;continue}let n=(b.y+17-a.y)%17;let d2=d*d%17;let h=(d2*((a.x+b.x+r.x)%17)+17-(n*n%17))%17;if h==0{assert_eq!(tadd(Some(a),Some(b)).unwrap().x,r.x);guards[2]+=1;continue}let u=(d*((a.y+r.y)%17)+n*((r.x+17-a.x)%17))%17;let den=d*h%17;let f=(1..17).find(|i|i*den%17==1).unwrap();let l=n*h*f%17;let w=d2*u*f%17;let k=(w+17-l)%17;let xs=(l*l+34-a.x-b.x)%17;let x=(k*k+34-xs-r.x)%17;let y=(w*((xs+17-x)%17)+l*((x+17-a.x)%17)+a.y)%17;assert_eq!(Some(Tiny{x,y}),want);ordinary+=1;}}}assert!(ordinary>0&&guards.iter().all(|n|*n>0));println!("tiny:{}points;{ordinary}ordinary;guards{guards:?}",points.len());}
#[test]fn extension_lowerings_match_arkworks(){let mut seed=0x3bd39e10cb0ef593u64;let mut next=||{seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;Fq::from(seed)};for _ in 0..1024{let[a,b,c,d,n]=std::array::from_fn::<_,5,_>(|_|Fq2::new(next(),next()));let op=Ops::<Fq2>::new();assert_eq!(op.mul(a,b),a*b);assert_eq!(op.sq(a),a.square());assert_eq!(op.sop(a,b,c,d),a*b+c*d);assert_eq!(op.minus_square(a,b,n),a*b-n.square());let norm=op.norm(a);if norm!=Fq::ZERO{assert_eq!(op.denominator_inverse(a,norm.inverse().unwrap()),a.inverse().unwrap());}}}
#[test]fn full_recipe_ledgers_have_expected_counts(){fn check<F:PointField>(p:[Point<F>;3],expected:[u64;4],binary:[u64;4]){let op=Ops::new();let out=direct_batch(&[p],&op).unwrap();let c=op.cost.get();assert_eq!([c.mul,c.square,c.fused,c.inverse],expected);let op=Ops::new();assert_eq!(two_binary(&[p],&op).unwrap(),out);let c=op.cost.get();assert_eq!([c.mul,c.square,c.fused,c.inverse],binary);}
let p=[Point::finite(Fq::from(1u64),Fq::from(2u64)),Point::finite(Fq::from(4u64),Fq::from(7u64)),Point::finite(Fq::from(9u64),Fq::from(13u64))];check(p,[8,3,3,1],[10,2,0,2]);check(p.map(|p|Point::finite(Fq2::new(p.x,Fq::ONE),Fq2::new(p.y,Fq::from(3u64)))),[13,0,21,1],[14,0,10,2]);}
}
