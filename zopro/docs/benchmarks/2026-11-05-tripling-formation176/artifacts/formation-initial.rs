//! Isolated complete a=0 affine tripling and diagnostic callers.
use super::*;
use zopro_bn254::{G1Config,G2Config};
use std::hint::black_box;

fn scalar<C:CurveConfig>(p:Affine<C>)->Affine<C>{if p.is_identity(){return p;}let beta=p.y+p.y;let beta2=beta.square();let a=p.x.square();let a=a+a+a;let xb=p.x*beta2;let z=a.square()-(xb+xb+xb);let Some(inv)=z.inverse()else{return Affine::IDENTITY};let d=beta2*inv;let u=a*inv;let t=u+u+d.square();Affine::new(p.x+beta2*t,beta*(a+beta2*d)*t-p.y)}

struct Group<F:BatchInverse>{x:F::Lanes,y:F::Lanes,beta2:F::Lanes,a:F::Lanes,zs:F::Lanes,norm:FqBatch}
struct Triple<F:BatchInverse>{groups:Vec<Option<Group<F>>>}
impl<F:BatchInverse>Default for Triple<F>{fn default()->Self{Self{groups:Vec::new()}}}
impl<F:BatchInverse>Triple<F>{
 fn apply<C:CurveConfig<Base=F>>(&mut self,input:&[Affine<C>],out:&mut[Affine<C>]){
  assert_eq!(input.len(),out.len());self.groups.clear();let full=input.len()/LANES;self.groups.reserve(full);let mut chains=FqBatch::one();let mut ordinary=0;
  for group in 0..full{let ps=&input[group*LANES..(group+1)*LANES];let xs:[F::Limbs;LANES]=core::array::from_fn(|i|F::to_limbs(ps[i].x));let ys:[F::Limbs;LANES]=core::array::from_fn(|i|F::to_limbs(ps[i].y));let x=F::load(xs.each_ref());let y=F::load(ys.each_ref());let beta=F::lanes_add(&y,&y);let beta2=F::lanes_square(&beta);let a=F::lanes_square(&x);let a=F::lanes_add(&F::lanes_add(&a,&a),&a);let xb=F::lanes_mul(&x,&beta2);let z=F::lanes_sub(&F::lanes_square(&a),&F::lanes_add(&F::lanes_add(&xb,&xb),&xb));let norm=F::lanes_norm(&z);
   if norm.get().iter().any(Field::is_zero){for i in 0..LANES{out[group*LANES+i]=scalar(ps[i]);}self.groups.push(None);continue;}
   let zs=F::lanes_inverse(&z,&chains);chains=chains.mul(&norm);self.groups.push(Some(Group{x,y,beta2,a,zs,norm}));ordinary+=1;
  }
  for i in full*LANES..input.len(){out[i]=scalar(input[i]);}
  if ordinary==0{return;}let mut inverse=direct_chain_inverse(&chains,true,true).unwrap_or_else(||FqBatch::new(&invert_lanes(chains.get(),true)));
  for (j,g)in self.groups.iter().enumerate().rev(){let Some(g)=g else{continue};let inv=F::lanes_scale(&g.zs,&inverse);inverse=inverse.mul(&g.norm);let d=F::lanes_mul(&g.beta2,&inv);let u=F::lanes_mul(&g.a,&inv);let t=F::lanes_add(&F::lanes_add(&u,&u),&F::lanes_square(&d));let x=F::lanes_add(&g.x,&F::lanes_mul(&g.beta2,&t));let coef=F::lanes_add(&g.a,&F::lanes_mul(&g.beta2,&d));let y=F::lanes_sub(&F::lanes_mul(&F::lanes_mul(&F::lanes_add(&g.y,&g.y),&coef),&t),&g.y);let x=F::store(&x);let y=F::store(&y);for i in 0..LANES{out[j*LANES+i]=Affine::new(F::from_limbs(x[i]),F::from_limbs(y[i]));}}
 }
 fn bytes(&self)->usize{self.groups.capacity()*std::mem::size_of::<Option<Group<F>>>()}
}

trait Kernel:BatchInverse{fn add<C:CurveConfig<Base=Self>>(s:&mut BatchAdd<C>,out:&mut[Affine<C>],batch:&[(u32,&Affine<C>)]);}
impl Kernel for Fq{fn add<C:CurveConfig<Base=Self>>(s:&mut BatchAdd<C>,out:&mut[Affine<C>],b:&[(u32,&Affine<C>)]){s.apply_signed_borrowed(out,b);}}
impl Kernel for Fq2{fn add<C:CurveConfig<Base=Self>>(s:&mut BatchAdd<C>,out:&mut[Affine<C>],b:&[(u32,&Affine<C>)]){s.apply_signed_borrowed_g2(out,b);}}

pub trait FormationCaller{fn reset(&mut self);fn run(&mut self,rounds:usize);fn check(&self,rounds:usize);fn bytes(&self)->Vec<u8>;fn capacity_bytes(&self)->usize;fn reference_bytes(&self)->usize;}
struct Fixture<C:CurveConfig>where C::Base:Kernel{input:Vec<Affine<C>>,out:Vec<Affine<C>>,incoming:Vec<(u32,Affine<C>)>,add:BatchAdd<C>,triple:Triple<C::Base>,formation:bool,n:usize}
fn progression<C:CurveConfig>(n:usize,start:u64,step:u64)->Vec<Affine<C>>{let g=Projective::<C>::generator();let mut p=g*Fr::from(start);let step=g*Fr::from(step);let mut v=Vec::with_capacity(n);for _ in 0..n{v.push(p);p+=step;}Projective::<C>::batch_to_affine(&v)}
impl<C:CurveConfig>Fixture<C>where C::Base:Kernel{
 fn new(n:usize,formation:bool,seed:u64)->Self{assert!(n<=1024);let size=n.max(1).next_power_of_two();let input=progression::<C>(size,(1u64<<48)+(seed+1)*100000003,1u64<<48);let incoming=progression::<C>(n,11111+(seed+1)*1337,101).into_iter().enumerate().map(|(i,p)|((((i*37+seed as usize)%size)as u32)|if i%3!=1{1u32<<31}else{0},p)).collect();Self{out:input.clone(),input,incoming,add:BatchAdd::default(),triple:Triple::default(),formation,n}}
 fn with_runner<R>(&mut self,action:impl FnOnce(&mut dyn FormationCaller)->R)->R{let refs=self.incoming.iter().map(|(i,p)|(*i,p)).collect::<Vec<_>>();let mut runner=Runner{input:&self.input,out:&mut self.out,refs,add:&mut self.add,triple:&mut self.triple,formation:self.formation,n:self.n};action(&mut runner)}
}
struct Runner<'a,C:CurveConfig>where C::Base:Kernel{input:&'a[Affine<C>],out:&'a mut[Affine<C>],refs:Vec<(u32,&'a Affine<C>)>,add:&'a mut BatchAdd<C>,triple:&'a mut Triple<C::Base>,formation:bool,n:usize}
impl<C:CurveConfig>FormationCaller for Runner<'_,C>where C::Base:Kernel{
 fn reset(&mut self){self.out.copy_from_slice(self.input);}
 fn run(&mut self,rounds:usize){if self.formation{for _ in 0..rounds{let s=black_box(&mut*self);s.triple.apply(black_box(&s.input[..s.n]),black_box(&mut s.out[..s.n]));}}else{for _ in 0..rounds{let s=black_box(&mut*self);C::Base::add(s.add,s.out,&s.refs);}}}
 fn check(&self,rounds:usize){let mut expected=self.input.iter().map(|p|p.to_projective()).collect::<Vec<_>>();if self.formation{for p in&mut expected[..self.n]{*p=p.double()+*p;}}else{for &(i,p)in&self.refs{let q=p.mul_limbs(&[rounds as u64]);if i>>31!=0{expected[(i&0x7fff_ffff)as usize]-=q;}else{expected[i as usize]+=q;}}}assert_eq!(&Projective::<C>::batch_to_affine(&expected),self.out);}
 fn bytes(&self)->Vec<u8>{format!("{:?}",self.out).into_bytes()}
 fn capacity_bytes(&self)->usize{if self.formation{self.triple.bytes()}else{self.add.fq.groups.capacity()*std::mem::size_of::<FqAddGroup>()+self.add.fq.tail.capacity()*std::mem::size_of::<[Fq;5]>()+self.add.fq2.prefix_records.capacity()*std::mem::size_of::<[FqBatch;2]>()+self.add.fq2.tail.capacity()*std::mem::size_of::<[Fq2;5]>()}}
 fn reference_bytes(&self)->usize{self.refs.capacity()*std::mem::size_of::<(u32,&Affine<C>)>()+(self.input.len()+self.out.len())*std::mem::size_of::<Affine<C>>()}
}
pub struct FormationG1(Fixture<G1Config>);pub struct FormationG2(Fixture<G2Config>);
impl FormationG1{pub fn new(n:usize,formation:bool,seed:u64)->Self{Self(Fixture::new(n,formation,seed))}pub fn with_runner<R>(&mut self,f:impl FnOnce(&mut dyn FormationCaller)->R)->R{self.0.with_runner(f)}}
impl FormationG2{pub fn new(n:usize,formation:bool,seed:u64)->Self{Self(Fixture::new(n,formation,seed))}pub fn with_runner<R>(&mut self,f:impl FnOnce(&mut dyn FormationCaller)->R)->R{self.0.with_runner(f)}}

pub fn formation_oracle_records(g2:bool,n:usize)->Vec<u8>{fn data<C:CurveConfig>(n:usize)->(Vec<Affine<C>>,Vec<Affine<C>>)where C::Base:BatchInverse{let mut input=progression::<C>(n,799999937,100000003);for i in 0..n{if i%17==0{input[i]=Affine::IDENTITY;}else if i%5==0{input[i]=-input[i];}}let mut out=vec![Affine::IDENTITY;n];Triple::default().apply(&input,&mut out);(input,out)}let mut bytes=Vec::new();if g2{let(a,b)=data::<G2Config>(n);for p in a.into_iter().chain(b){for x in[p.x.c0,p.x.c1,p.y.c0,p.y.c1]{for w in x.to_canonical(){bytes.extend(w.to_le_bytes());}}}}else{let(a,b)=data::<G1Config>(n);for p in a.into_iter().chain(b){for x in[p.x,p.y]{for w in x.to_canonical(){bytes.extend(w.to_le_bytes());}}}}bytes}

#[cfg(test)]mod tests{
 use super::*;
 #[test]fn scalar_and_vector_tripling_match_jacobian_oracle_tails_identities_and_reuse(){fn check<C:CurveConfig>()where C::Base:BatchInverse{let mut scratch=Triple::default();for n in[0usize,1,7,8,9,31,32,199,200,351,352,1023,1024]{let mut input=progression::<C>(n,1234567,9933);for i in 0..n{if i%17==0{input[i]=Affine::IDENTITY;}else if i%3==0{input[i]=-input[i];}}let before=input.clone();let mut out=vec![Affine::IDENTITY;n];for _ in 0..3{scratch.apply(&input,&mut out);let expected=Projective::<C>::batch_to_affine(&input.iter().map(|p|p.to_projective().double()+*p).collect::<Vec<_>>());assert_eq!(out,expected);assert_eq!(input,before);}for(i,p)in input.into_iter().enumerate(){assert_eq!(scalar(p),out[i]);}}}check::<G1Config>();check::<G2Config>();}
 #[test]fn borrowed_reference_and_formation_callers_keep_exact_outputs(){for n in[0usize,7,200,352,1024]{for formation in[false,true]{let mut a=FormationG1::new(n,formation,17);a.with_runner(|f|{for rounds in[1,3,17]{f.reset();f.run(rounds);f.check(rounds);}});let mut b=FormationG2::new(n,formation,17);b.with_runner(|f|{for rounds in[1,3,17]{f.reset();f.run(rounds);f.check(rounds);}});}}}
 #[test]fn complete_formula_exhausts_toy_curves_including_two_and_three_torsion(){type P=Option<(i64,i64)>;fn inv(x:i64,p:i64)->Option<i64>{(1..p).find(|i|x*i%p==1)}fn add(a:P,b:P,p:i64)->P{let(Some((x,y)),Some((u,v)))=(a,b)else{return a.or(b)};let numerator;if x==u{if(y+v)%p==0{return None;}numerator=3*x*x;}else{numerator=v-y;}let denominator=if x==u{2*y}else{u-x};let slope=(numerator*inv(denominator.rem_euclid(p),p).unwrap()).rem_euclid(p);let a=(slope*slope-x-u).rem_euclid(p);Some((a,(slope*(x-a)-y).rem_euclid(p)))}fn formula(a:P,p:i64)->P{let(x,y)=a?;let beta=2*y%p;let b=beta*beta%p;let aa=3*x*x%p;let z=(aa*aa-3*x*b).rem_euclid(p);let inv=inv(z,p)?;let d=b*inv%p;let t=(2*aa*inv+d*d)%p;Some(((x+b*t)%p,(beta*(aa+b*d)*t-y).rem_euclid(p)))}let(mut points,mut three,mut two)=(0,0,0);for p in[7,11,13,17,19,31]{for b in 1..p{for x in 0..p{for y in 0..p{if(y*y-x*x*x-b).rem_euclid(p)!=0{continue;}let a=Some((x,y));let twice=add(a,a,p);let target=add(twice,a,p);assert_eq!(formula(a,p),target);points+=1;three+=usize::from(target.is_none());two+=usize::from(twice.is_none());}}}assert_eq!(formula(None,p),None);}assert!(points>1000&&three>0&&two>0);}
}
