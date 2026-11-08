//! Diagnostic streaming callers; ordinary entrypoints inherit distinct finite
//! destinations and nonzero denominators. The checked adapter is test-only policy.
use zopro_bn254::{Affine,CurveConfig,Fq,Fq2,FqConfig,G1Affine,G2Affine};
use zopro_bn254::ifma8::late_centered::{Signed,Loose};
type S=Signed<FqConfig,true>;type L=Loose<FqConfig,true>;
#[derive(Clone,Copy)]struct G1Record{ax:S,ay:S,xsum:L,denominator:S,numerator:S}
#[derive(Default)]pub struct G1Scratch{groups:Vec<G1Record>,tail:Vec<[Fq;5]>}
#[derive(Default)]pub struct G2Scratch{groups:Vec<[S;2]>,tail:Vec<[Fq2;5]>}
impl G1Scratch{pub fn capacities(&self)->(usize,usize){(self.groups.capacity()*core::mem::size_of::<G1Record>(),self.tail.capacity()*core::mem::size_of::<[Fq;5]>())}}
impl G2Scratch{pub fn capacities(&self)->(usize,usize){(self.groups.capacity()*core::mem::size_of::<[S;2]>(),self.tail.capacity()*core::mem::size_of::<[Fq2;5]>())}}
#[inline(always)]fn index(encoded:u32)->usize{(encoded&0x7fffffff)as usize}
#[inline(always)]fn mask<C:CurveConfig>(batch:&[(u32,&Affine<C>)])->u8{let mut bits=0;for i in 0..8{bits|=((batch[i].0>>31)as u8)<<i;}bits}
#[inline(always)]fn pack(v:[Fq2;8])->[S;2]{[S::new(&v.map(|x|x.c0)),S::new(&v.map(|x|x.c1))]}
#[inline(always)]fn unpack(v:[L;2])->[Fq2;8]{let a=v[0].wide_get();let b=v[1].wide_get();core::array::from_fn(|i|Fq2::new(a[i],b[i]))}
#[inline(always)]fn invert_host(values:[Fq;8])->[Fq;8]{S::new(&values).inverse_nonzero().root_get()}

#[inline(never)]
pub fn g1(buckets:&mut[G1Affine],batch:&[(u32,&G1Affine)],s:&mut G1Scratch){
 let full=batch.len()/8*8;let mut chains=S::one();s.groups.clear();s.groups.reserve(full/8);
 for k in(0..full).step_by(8){
  let a=core::array::from_fn::<_,8,_>(|i|buckets[index(batch[k+i].0)]);let b=core::array::from_fn::<_,8,_>(|i|*batch[k+i].1);
  // Same initialized-record publication discipline as the accepted G1 caller.
  unsafe{
   let slot=s.groups.as_mut_ptr().add(k/8);let ax=S::new(&a.map(|p|p.x));core::ptr::addr_of_mut!((*slot).ax).write(ax);
   let bx=S::new(&b.map(|p|p.x));let denominator=bx.sub(&ax).normalize();core::ptr::addr_of_mut!((*slot).xsum).write(ax.add(&bx));core::ptr::addr_of_mut!((*slot).denominator).write(denominator);
   let ay=S::new(&a.map(|p|p.y));core::ptr::addr_of_mut!((*slot).ay).write(ay);let by=S::new(&b.map(|p|p.y));
   let dy=by.negate_if(mask(&batch[k..k+8])).sub(&ay).normalize();core::ptr::addr_of_mut!((*slot).numerator).write(dy.mul(&chains));chains=chains.mul(&denominator);
   s.groups.set_len(k/8+1);
  }
 }
 s.tail.clear();let mut inverse=if full==batch.len(){chains.inverse_nonzero()}else{
  let mut tail=chains.root_get();for k in full..batch.len(){let(encoded,b)=batch[k];let b=if encoded>>31!=0{-*b}else{*b};let a=buckets[index(encoded)];let denominator=b.x-a.x;let numerator=(b.y-a.y)*tail[k%8];tail[k%8]*=denominator;s.tail.push([a.x,a.y,a.x+b.x,denominator,numerator]);}
  let mut inverse=invert_host(tail);for k in(full..batch.len()).rev(){let[ax,ay,xsum,denominator,numerator]=s.tail[k-full];let slope=numerator*inverse[k%8];inverse[k%8]*=denominator;let x=slope.square()-xsum;let y=slope*(ax-x)-ay;buckets[index(batch[k].0)]=G1Affine::new(x,y);}S::new(&inverse)
 };
 for(j,g)in s.groups.iter().enumerate().rev(){let slope=g.numerator.mul(&inverse);inverse=inverse.mul(&g.denominator);let x=slope.mul(&slope).sub_loose(&g.xsum);let y=slope.mul(&g.ax.sub_loose(&x).normalize()).sub(&g.ay);let(x,y)=(x.wide_get(),y.wide_get());for i in 0..8{buckets[index(batch[j*8+i].0)]=G1Affine::new(x[i],y[i]);}}
}

#[inline(never)]
pub fn g2(buckets:&mut[G2Affine],batch:&[(u32,&G2Affine)],s:&mut G2Scratch){
 let full=batch.len()/8*8;let mut chains=S::one();s.groups.clear();s.groups.reserve(full/8);
 for k in(0..full).step_by(8){let ax=pack(core::array::from_fn(|i|buckets[index(batch[k+i].0)].x));let bx=pack(core::array::from_fn(|i|batch[k+i].1.x));let dx=core::array::from_fn::<_,2,_>(|j|bx[j].sub(&ax[j]).normalize());let norm=dx[0].sum2(&dx[0],&dx[1],&dx[1]);s.groups.push([chains,norm]);chains=chains.mul(&norm);}
 s.tail.clear();let mut inverse=if full==batch.len(){chains.inverse_nonzero()}else{
  let mut tail=chains.root_get();for k in full..batch.len(){let(encoded,b)=batch[k];let b=if encoded>>31!=0{-*b}else{*b};let a=buckets[index(encoded)];let denominator=b.x-a.x;let norm=denominator.c0.square()+denominator.c1.square();let numerator=((b.y-a.y)*denominator.conjugate()).mul_by_fp(&tail[k%8]);tail[k%8]*=norm;s.tail.push([a.x,a.y,a.x+b.x,Fq2::new(norm,Fq::ZERO),numerator]);}
  let mut inverse=invert_host(tail);for k in(full..batch.len()).rev(){let[ax,ay,xsum,norm,numerator]=s.tail[k-full];let slope=numerator.mul_by_fp(&inverse[k%8]);inverse[k%8]*=norm.c0;let x=slope.square()-xsum;let y=slope*(ax-x)-ay;buckets[index(batch[k].0)]=G2Affine::new(x,y);}S::new(&inverse)
 };
 for j in(0..full/8).rev(){let k=j*8;let a=core::array::from_fn::<_,8,_>(|i|buckets[index(batch[k+i].0)]);let b=core::array::from_fn::<_,8,_>(|i|*batch[k+i].1);let ax=pack(a.map(|p|p.x));let ay=pack(a.map(|p|p.y));let bx=pack(b.map(|p|p.x));let by=pack(b.map(|p|p.y));let negative=mask(&batch[k..k+8]);
  let dy=core::array::from_fn::<_,2,_>(|i|by[i].negate_if(negative).sub(&ay[i]).normalize());let dx=core::array::from_fn::<_,2,_>(|i|bx[i].sub(&ax[i]).normalize());
  let factor=s.groups[j][0].mul(&inverse);inverse=inverse.mul(&s.groups[j][1]);let num=[dy[0].sum2(&dx[0],&dy[1],&dx[1]),dy[1].difference2(&dx[0],&dy[0],&dx[1])];let slope=[num[0].mul(&factor),num[1].mul(&factor)];
  let product=slope[0].mul(&slope[1]);let square=[slope[0].add(&slope[1]).normalize().mul(&slope[0].sub(&slope[1]).normalize()).sub(&ax[0]).sub(&bx[0]),product.add(&product).sub(&ax[1]).sub(&bx[1])];
  let delta=[ax[0].sub_loose(&square[0]).normalize(),ax[1].sub_loose(&square[1]).normalize()];let y=[slope[0].difference2(&delta[0],&slope[1],&delta[1]).sub(&ay[0]),slope[0].sum2(&delta[1],&slope[1],&delta[0]).sub(&ay[1])];
  let(x,y)=(unpack(square),unpack(y));for i in 0..8{buckets[index(batch[k+i].0)]=G2Affine::new(x[i],y[i]);}
 }
}

/// No point is modified before this diagnostic adapter decides the whole route.
pub fn ordinary<C:CurveConfig>(buckets:&[Affine<C>],batch:&[(u32,&Affine<C>)])->bool{
 for(i,&(encoded,b))in batch.iter().enumerate(){let k=index(encoded);assert!(k<buckets.len());if buckets[k].is_identity()||b.is_identity()||buckets[k].x==b.x||batch[..i].iter().any(|&(other,_)|index(other)==k){return false;}}true
}
pub fn scalar_fallback<C:CurveConfig>(buckets:&mut[Affine<C>],batch:&[(u32,&Affine<C>)]){for&(encoded,b)in batch{let k=index(encoded);let b=if encoded>>31!=0{-*b}else{*b};buckets[k]=(buckets[k]+b).to_affine();}}
pub fn checked_g1(buckets:&mut[G1Affine],batch:&[(u32,&G1Affine)],s:&mut G1Scratch)->bool{if !ordinary(buckets,batch){scalar_fallback(buckets,batch);false}else{g1(buckets,batch,s);true}}
pub fn checked_g2(buckets:&mut[G2Affine],batch:&[(u32,&G2Affine)],s:&mut G2Scratch)->bool{if !ordinary(buckets,batch){scalar_fallback(buckets,batch);false}else{g2(buckets,batch,s);true}}
