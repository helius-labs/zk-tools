//! Private investigation types. No signed representative is an Fp/Ifma8 value.
use super::*;

#[derive(Clone, Copy)]
pub struct Signed<C: FpConfig, const MIXED: bool = true>([__m512i; 5], PhantomData<C>);
#[derive(Clone, Copy)]
pub struct Loose<C: FpConfig, const MIXED: bool = true>([__m512i; 5], PhantomData<C>);
#[inline(always)] fn splat(x:u64)->__m512i{unsafe{_mm512_set1_epi64(x as i64)}}
#[inline(always)] fn normalized(mut t:[__m512i;5])->[__m512i;5]{unsafe{
 let mask=splat(MASK);for j in 0..4{t[j+1]=_mm512_add_epi64(t[j+1],_mm512_srai_epi64::<52>(t[j]));t[j]=_mm512_and_si512(t[j],mask);}t
}}
impl<C:FpConfig,const M:bool> Loose<C,M>{
 #[inline(always)] pub fn normalize(self)->Signed<C,M>{Signed(normalized(self.0),PhantomData)}
 #[inline(always)] pub fn sub(self,b:&Signed<C,M>)->Self{unsafe{Self(core::array::from_fn(|j|_mm512_sub_epi64(self.0[j],b.0[j])),PhantomData)}}
 #[inline(always)] pub fn add(self,b:&Signed<C,M>)->Self{unsafe{Self(core::array::from_fn(|j|_mm512_add_epi64(self.0[j],b.0[j])),PhantomData)}}
 /// Caller must establish |value|<4p. This is an internal affine-circuit bound.
 #[inline(always)] pub fn wide_get(&self)->[Fp<C>;8]{self.normalize().wide_get()}
}
impl<C:FpConfig,const M:bool> Signed<C,M>{
 #[inline(always)] pub fn new(x:&[Fp<C>;8])->Self{Self(Ifma8::<C,M>::new(x).0,PhantomData)}
 #[inline(always)] pub fn one()->Self{Self(Ifma8::<C,M>::one().0,PhantomData)}
 #[inline(always)] pub fn zero()->Self{Self([splat(0);5],PhantomData)}
 /// # Safety
 /// Low four digits must be in[0,2^52), signed top encodes an integer of
 /// magnitude <=5p. Every later operation must preserve its documented bounds.
 pub unsafe fn from_raw_signed_lanes(x:&[[i64;5];8])->Self{unsafe{
  let t=core::array::from_fn(|j|{let column=core::array::from_fn::<i64,8,_>(|i|x[i][j]);_mm512_loadu_si512(column.as_ptr().cast())});Self(t,PhantomData)
 }}
 pub fn raw_signed_lanes(&self)->[[i64;5];8]{unsafe{
  let mut out=[[0i64;5];8];for j in 0..5{let mut column=[0i64;8];_mm512_storeu_si512(column.as_mut_ptr().cast(),self.0[j]);for i in 0..8{out[i][j]=column[i];}}out
 }}
 #[inline(always)] pub fn add(&self,b:&Self)->Loose<C,M>{unsafe{Loose(core::array::from_fn(|j|_mm512_add_epi64(self.0[j],b.0[j])),PhantomData)}}
 #[inline(always)] pub fn sub(&self,b:&Self)->Loose<C,M>{unsafe{Loose(core::array::from_fn(|j|_mm512_sub_epi64(self.0[j],b.0[j])),PhantomData)}}
 #[inline(always)] pub fn sub_loose(&self,b:&Loose<C,M>)->Loose<C,M>{unsafe{Loose(core::array::from_fn(|j|_mm512_sub_epi64(self.0[j],b.0[j])),PhantomData)}}
 #[inline(always)] pub fn negate_if(&self,negative:u8)->Loose<C,M>{unsafe{
  let zero=splat(0);Loose(core::array::from_fn(|j|_mm512_mask_sub_epi64(self.0[j],negative,zero,self.0[j])),PhantomData)
 }}
 #[inline(always)] pub fn mul(&self,b:&Self)->Self{self.product::<false>(b,self,b)}
 #[inline(always)] pub fn sum2(&self,b:&Self,c:&Self,d:&Self)->Self{self.product::<true>(b,c,d)}
 #[inline(always)] pub fn difference2(&self,b:&Self,c:&Self,d:&Self)->Self{
  let negative=c.negate_if(255).normalize();self.product::<true>(b,&negative,d)
 }
 #[inline(always)] fn product<const FUSED:bool>(&self,b:&Self,c:&Self,d:&Self)->Self{unsafe{
  let zero=splat(0);let mask=splat(MASK);let p=split(C::P).map(splat);let inv=splat(C::INV&MASK);
  let raw=|x:&Self|{let mut a=x.0;a[4]=_mm512_and_si512(a[4],mask);a};
  let a=raw(self);let bb=raw(b);let cc=raw(c);let dd=raw(d);
  let sign=|x:&Self|_mm512_cmplt_epi64_mask(x.0[4],zero);
  let sa=sign(self);let sb=sign(b);let sc=sign(c);let sd=sign(d);
  let mut t=[zero;6];let center;
  macro_rules! row{($i:literal,$bits:literal)=>{{
   macro_rules! product{($j:literal,$k:literal)=>{{
    t[$j]=_mm512_madd52lo_epu64(t[$j],a[$j],bb[$i]);t[$k]=_mm512_madd52hi_epu64(t[$k],a[$j],bb[$i]);
    if FUSED{t[$j]=_mm512_madd52lo_epu64(t[$j],cc[$j],dd[$i]);t[$k]=_mm512_madd52hi_epu64(t[$k],cc[$j],dd[$i]);}
   }};}
   product!(0,1);product!(1,2);product!(2,3);product!(3,4);product!(4,5);
   let digitmask=splat((1u64<<$bits)-1);let q=_mm512_and_si512(_mm512_madd52lo_epu64(zero,t[0],inv),digitmask);
   let carry=_mm512_srli_epi64::<$bits>(_mm512_add_epi64(t[0],digitmask));
   let qscaled=if $bits==48{_mm512_slli_epi64::<4>(q)}else{q};
   let low=_mm512_madd52hi_epu64(carry,qscaled,p[0]);
   macro_rules! reduction{($j:literal,$k:literal)=>{{t[$j]=_mm512_madd52lo_epu64(t[$j],q,p[$j]);t[$k]=_mm512_madd52hi_epu64(t[$k],q,p[$j]);}};}
   reduction!(1,2);reduction!(2,3);reduction!(3,4);reduction!(4,5);
   if $bits==52{t[1]=_mm512_add_epi64(t[1],low);t=[t[1],t[2],t[3],t[4],t[5],zero];}
   else{t=[_mm512_add_epi64(low,_mm512_slli_epi64::<4>(_mm512_and_si512(t[1],digitmask))),
    _mm512_add_epi64(_mm512_srli_epi64::<48>(t[1]),_mm512_slli_epi64::<4>(_mm512_and_si512(t[2],digitmask))),
    _mm512_add_epi64(_mm512_srli_epi64::<48>(t[2]),_mm512_slli_epi64::<4>(_mm512_and_si512(t[3],digitmask))),
    _mm512_add_epi64(_mm512_srli_epi64::<48>(t[3]),_mm512_slli_epi64::<4>(_mm512_and_si512(t[4],digitmask))),
    _mm512_add_epi64(_mm512_srli_epi64::<48>(t[4]),_mm512_slli_epi64::<4>(t[5])),zero];}
   q
  }};}
  row!(0,52);row!(1,52);row!(2,52);row!(3,52);
  let q=if M{row!(4,48)}else{row!(4,52)};
  center=_mm512_cmpge_epu64_mask(q,splat(if M{1u64<<47}else{1u64<<51}));
  for j in 0..5{
   let scale=|v|if M{_mm512_slli_epi64::<4>(v)}else{v};
   t[j]=_mm512_mask_sub_epi64(t[j],sa,t[j],scale(bb[j]));t[j]=_mm512_mask_sub_epi64(t[j],sb,t[j],scale(a[j]));
   if FUSED{t[j]=_mm512_mask_sub_epi64(t[j],sc,t[j],scale(dd[j]));t[j]=_mm512_mask_sub_epi64(t[j],sd,t[j],scale(cc[j]));}
   t[j]=_mm512_mask_sub_epi64(t[j],center,t[j],p[j]);
  }
  let mut out=normalized([t[0],t[1],t[2],t[3],t[4]]);out[4]=_mm512_srai_epi64::<12>(_mm512_slli_epi64::<12>(out[4]));Self(out,PhantomData)
 }}
 #[inline(always)] fn positive_root(&self)->Ifma8<C,M>{unsafe{
  let p=split(C::P).map(splat);let negative=_mm512_cmplt_epi64_mask(self.0[4],splat(0));
  let t=core::array::from_fn(|j|_mm512_mask_add_epi64(self.0[j],negative,self.0[j],p[j]));Ifma8(normalized(t),PhantomData)
 }}
 /// Requires a nonzero representative of magnitude<p in each lane.
 #[inline(always)] pub fn inverse_nonzero(&self)->Self{
  let root=self.positive_root();let swapped=Ifma8::shuffle_pair::<1,0,3,2>(&root,&root);let pairs=root.mul(&swapped);
  let other_pair=Ifma8::shuffle_pair::<2,3,0,1>(&pairs,&pairs);let quads=pairs.mul(&other_pair);let products=quads.get();
  let inverse=(products[0]*products[4]).inverse().unwrap();let(lo,hi)=quads.halves();let other_quad=Ifma8::from_halves(&hi,&lo);
  let out=Ifma8::<C,M>::new(&[inverse;8]).mul(&other_quad).mul(&other_pair).mul(&swapped);
  Self(Ifma8::<C,M>::reduce(out.0,C::P).0,PhantomData)
 }
 #[inline(always)] pub fn root_get(&self)->[Fp<C>;8]{self.positive_root().get()}
 /// Requires |value|<4p, including the signed affine finish's Loose values.
 #[inline(always)] pub fn wide_get(&self)->[Fp<C>;8]{unsafe{
  let p4=const{crate::portable::add4(crate::portable::add4(C::P,C::P).0,crate::portable::add4(C::P,C::P).0).0};
  let offset=split(p4).map(splat);let mut t=normalized(core::array::from_fn(|j|_mm512_add_epi64(self.0[j],offset[j])));
  let q=_mm512_madd52hi_epu64(splat(0),t[4],splat(Ifma8::<C,M>::RECIPROCAL));let p=split(C::P).map(splat);let mut carry=splat(0);let mask=splat(MASK);
  for j in 0..5{let v=_mm512_add_epi64(_mm512_sub_epi64(t[j],_mm512_mullo_epi64(q,p[j])),carry);carry=_mm512_srai_epi64::<52>(v);t[j]=_mm512_and_si512(v,mask);}
  let canonical=Ifma8::<C,M>::reduce(t,C::P);
  if !M{return canonical.get();}
  pack_canonical(canonical.0)
 }}
}

/// Identical transpose/store sequence to the accepted R256 get after its reduce.
#[inline(always)] fn pack_canonical<C:FpConfig>(t:[__m512i;5])->[Fp<C>;8]{unsafe{
 let[t0,t1,t2,t3,t4]=t;
 let w0=_mm512_or_si512(t0,_mm512_slli_epi64::<52>(t1));let w1=_mm512_or_si512(_mm512_srli_epi64::<12>(t1),_mm512_slli_epi64::<40>(t2));
 let w2=_mm512_or_si512(_mm512_srli_epi64::<24>(t2),_mm512_slli_epi64::<28>(t3));let w3=_mm512_or_si512(_mm512_srli_epi64::<36>(t3),_mm512_slli_epi64::<16>(t4));
 let a=_mm512_unpacklo_epi64(w0,w1);let b=_mm512_unpackhi_epi64(w0,w1);let c=_mm512_unpacklo_epi64(w2,w3);let d=_mm512_unpackhi_epi64(w2,w3);
 let low=_mm512_set_epi64(11,10,3,2,9,8,1,0);let high=_mm512_set_epi64(15,14,7,6,13,12,5,4);
 let even_lo=_mm512_permutex2var_epi64(a,low,c);let even_hi=_mm512_permutex2var_epi64(a,high,c);let odd_lo=_mm512_permutex2var_epi64(b,low,d);let odd_hi=_mm512_permutex2var_epi64(b,high,d);
 let mut out=core::mem::MaybeUninit::<[Fp<C>;8]>::uninit();let p=out.as_mut_ptr().cast::<__m512i>();
 _mm512_storeu_si512(p,_mm512_shuffle_i64x2::<0x44>(even_lo,odd_lo));_mm512_storeu_si512(p.add(1),_mm512_shuffle_i64x2::<0xee>(even_lo,odd_lo));
 _mm512_storeu_si512(p.add(2),_mm512_shuffle_i64x2::<0x44>(even_hi,odd_hi));_mm512_storeu_si512(p.add(3),_mm512_shuffle_i64x2::<0xee>(even_hi,odd_hi));out.assume_init()
}}
