use zopro_bn254::{ifma::Ifma,ifma8::Ifma8,FqConfig,Fq,Fq2,Fq6,Fq12,G1Affine};
use std::ops::{Add,Sub};
type Half=Ifma<FqConfig,true>;
type Wide=Ifma8<FqConfig,true>;
#[derive(Clone,Copy)]
struct Q(Wide);
impl Q{
 #[inline(always)] fn join(a:Half,b:Half)->Self{Self(Wide::from_halves(&a,&b))}
 #[inline(always)] fn new(a:[Fq2;4])->Self{Self::join(Half::new(&a.map(|q|q.c0)),Half::new(&a.map(|q|q.c1)))}
 #[inline(always)] fn get(self)->[Fq2;4]{let v=self.0.get();std::array::from_fn(|i|Fq2::new(v[i],v[i+4]))}
 #[inline(always)] fn zero()->Self{Self(Wide::zero())}
 #[inline(always)] fn one()->Self{Self::join(Half::one(),Half::zero())}
 #[inline(always)] fn mul(self,b:Self)->Self{
  let(a0,a1)=self.0.halves();let(b0,b1)=b.0.halves();
  let left=Wide::from_halves(&a0,&a0);let right=Wide::from_halves(&b0,&b1);
  let negative=Half::zero().sub(&a1);let cross=Wide::from_halves(&negative,&a1);let swap=Wide::from_halves(&b1,&b0);
  Self(left.sum2(&right,&cross,&swap))
 }
 #[inline(always)] fn square(self)->Self{let(a,b)=self.0.halves();Self::join(a.add(&b),a.add(&a)).mul_fp_halves(a.sub(&b),b)}
 #[inline(always)] fn mul_fp_halves(self,a:Half,b:Half)->Self{Self(self.0.mul(&Wide::from_halves(&a,&b)))}
 #[inline(always)] fn scale(self,a:Half)->Self{self.mul_fp_halves(a,a)}
 #[inline(always)] fn double(self)->Self{Self(self.0.add(&self.0))}
 #[inline(always)] fn nr(self)->Self{let(a,b)=self.0.halves();let times9=|v:Half|{let v2=v.add(&v);let v4=v2.add(&v2);v4.add(&v4).add(&v)};Self::join(times9(a).sub(&b),a.add(&times9(b)))}
}
impl Add for Q{type Output=Self;#[inline(always)]fn add(self,b:Self)->Self{Self(self.0.add(&b.0))}}
impl Sub for Q{type Output=Self;#[inline(always)]fn sub(self,b:Self)->Self{Self(self.0.sub(&b.0))}}
#[derive(Clone,Copy)]
struct C([Q;3]);
impl C{
 #[inline(always)]fn zero()->Self{Self([Q::zero();3])}
 #[inline(always)]fn one()->Self{Self([Q::one(),Q::zero(),Q::zero()])}
 #[inline(always)]fn nr(self)->Self{Self([self.0[2].nr(),self.0[0],self.0[1]])}
 #[inline(always)]fn mul(self,b:Self)->Self{
  let[a0,a1,a2]=self.0;let[b0,b1,b2]=b.0;let v0=a0.mul(b0);let v1=a1.mul(b1);let v2=a2.mul(b2);
  Self([((a1+a2).mul(b1+b2)-v1-v2).nr()+v0,(a0+a1).mul(b0+b1)-v0-v1+v2.nr(),(a0+a2).mul(b0+b2)-v0+v1-v2])
 }
 #[inline(always)]fn mul_fp2(self,b:Q)->Self{Self(self.0.map(|a|a.mul(b)))}
 #[inline(always)]fn mul01(self,b0:Q,b1:Q)->Self{
  let[a0,a1,a2]=self.0;let aa=a0.mul(b0);let bb=a1.mul(b1);
  Self([(b1.mul(a1+a2)-bb).nr()+aa,(b0+b1).mul(a0+a1)-aa-bb,b0.mul(a0+a2)-aa+bb])
 }
}
impl Add for C{type Output=Self;#[inline(always)]fn add(self,b:Self)->Self{Self(std::array::from_fn(|i|self.0[i]+b.0[i]))}}
impl Sub for C{type Output=Self;#[inline(always)]fn sub(self,b:Self)->Self{Self(std::array::from_fn(|i|self.0[i]-b.0[i]))}}
#[derive(Clone,Copy)]
pub struct T([C;2]);
impl T{
 pub fn new(a:[Fq12;4])->Self{Self(std::array::from_fn(|half|C(std::array::from_fn(|j|Q::new(a.map(|v|{let c=if half==0{v.c0}else{v.c1};[c.c0,c.c1,c.c2][j]}))))))}
 pub fn get(self)->[Fq12;4]{let q=self.0.map(|c|c.0.map(Q::get));std::array::from_fn(|i|Fq12::new(Fq6::new(q[0][0][i],q[0][1][i],q[0][2][i]),Fq6::new(q[1][0][i],q[1][1][i],q[1][2][i])))}
 pub fn one()->Self{Self([C::one(),C::zero()])}
 #[inline(never)] pub fn mul(self,b:Self)->Self{let v0=self.0[0].mul(b.0[0]);let v1=self.0[1].mul(b.0[1]);Self([v0+v1.nr(),(self.0[0]+self.0[1]).mul(b.0[0]+b.0[1])-v0-v1])}
 #[inline(never)] pub fn square(self)->Self{let[a,b]=self.0;let p=a.mul(b);Self([(a+b).mul(a+b.nr())-p-p.nr(),p+p])}
 #[inline(never)] fn line(self,c0:Q,c3:Q,c4:Q)->Self{let a=self.0[0].mul_fp2(c0);let b=self.0[1].mul01(c3,c4);let e=(self.0[0]+self.0[1]).mul01(c0+c3,c4);Self([b.nr()+a,e-(a+b)])}
 pub fn sparse(self,c0:[Fq2;4],c3:[Fq2;4],c4:[Fq2;4])->Self{self.line(Q::new(c0),Q::new(c3),Q::new(c4))}
}
pub fn miller(pairs:&[(G1Affine,&crate::prepared::G2Prepared);3])->Fq12{
 let active=pairs.map(|(p,q)|!p.is_identity()&&!q.is_identity());
 let x=Half::new(&std::array::from_fn(|i|if i<3&&active[i]{pairs[i].0.x}else{Fq::ONE}));
 let y=Half::new(&std::array::from_fn(|i|if i<3&&active[i]{pairs[i].0.y}else{Fq::ONE}));
 let mut f=T::one();let mut index=0;
 let mut ell=|f:T|{let lines:[(Fq2,Fq2,Fq2);4]=std::array::from_fn(|i|if i<3&&active[i]{pairs[i].1.coefficients()[index]}else{(Fq2::ONE,Fq2::ZERO,Fq2::ZERO)});index+=1;f.line(Q::new(lines.map(|l|l.0)).scale(y),Q::new(lines.map(|l|l.1)).scale(x),Q::new(lines.map(|l|l.2)))};
 let schedule=crate::prepared::schedule();
 for i in (1..schedule.len()).rev(){if i!=schedule.len()-1{f=f.square();}f=ell(f);if schedule[i-1]!=0{f=ell(f);}}
 for _ in 0..2{f=ell(f);}
 let values=f.get();values[0]*values[1]*values[2]
}
pub fn check_q(a:[Fq2;4],b:[Fq2;4]){
 let x=Q::new(a);let y=Q::new(b);assert_eq!(x.mul(y).get(),std::array::from_fn(|i|a[i]*b[i]));assert_eq!(x.square().get(),a.map(|v|v.square()));assert_eq!(x.nr().get(),a.map(|v|v.mul_by_nonresidue()));assert_eq!(x.double().get(),a.map(|v|v.double()));
}
