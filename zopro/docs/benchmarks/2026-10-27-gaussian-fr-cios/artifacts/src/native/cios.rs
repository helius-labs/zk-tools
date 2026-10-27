//! Three coupled cancellation rounds for one Gaussian base-field value.
use super::*;
type C=[V;4];

/// Add x*(yr+i*yi), using three real products and explicit 53-bit sum carry.
#[inline(always)]
unsafe fn row<const LAST:bool>(re:&mut C,im:&mut C,x:G8,xsum:D,yr:V,yi:V){
 let sum=_mm512_add_epi64(yr,yi);
 let low=_mm512_and_si512(sum,splat(MASK));
 let carry=_mm512_cmpneq_epi64_mask(_mm512_srli_epi64::<52>(sum),zero());
 for i in 0..3{
  let p=_mm512_madd52lo_epu64(zero(),x.re[i],yr);
  let q=_mm512_madd52lo_epu64(zero(),x.im[i],yi);
  let s=_mm512_madd52lo_epu64(zero(),xsum[i],low);
  re[i]=_mm512_add_epi64(re[i],_mm512_sub_epi64(p,q));
  im[i]=_mm512_add_epi64(im[i],_mm512_sub_epi64(_mm512_sub_epi64(s,p),q));
  if !(LAST&&i==2){
   let p=_mm512_madd52hi_epu64(zero(),x.re[i],yr);
   let q=_mm512_madd52hi_epu64(zero(),x.im[i],yi);
   let s=_mm512_madd52hi_epu64(zero(),xsum[i],low);
   re[i+1]=_mm512_add_epi64(re[i+1],_mm512_sub_epi64(p,q));
   im[i+1]=_mm512_add_epi64(im[i+1],_mm512_sub_epi64(_mm512_sub_epi64(s,p),q));
   im[i+1]=_mm512_add_epi64(im[i+1],_mm512_maskz_mov_epi64(carry,xsum[i]));
  }
 }
}

#[inline(always)]
unsafe fn cancel<const LAST:bool>(re:&mut C,im:&mut C)->(V,V){
 let p=_mm512_madd52lo_epu64(zero(),re[0],splat(KRE[0]));
 let q=_mm512_madd52lo_epu64(zero(),im[0],splat(KIM[0]));
 let s=_mm512_madd52lo_epu64(zero(),_mm512_add_epi64(re[0],im[0]),splat(KSUM[0]));
 let mask=splat(if LAST{TOP}else{MASK});
 let mr=_mm512_and_si512(_mm512_sub_epi64(p,q),mask);
 let mi=_mm512_and_si512(_mm512_sub_epi64(_mm512_sub_epi64(s,p),q),mask);
 let n=G8{re:digits(A),im:digits(B)};
 row::<LAST>(re,im,n,digits(A+B),mr,mi);
 (mr,mi)
}

#[inline(always)]
unsafe fn shift(t:&mut C){
 t[1]=_mm512_add_epi64(t[1],_mm512_srai_epi64::<52>(t[0]));
 *t=[t[1],t[2],t[3],zero()];
}

#[inline(always)]
unsafe fn repack(t:C)->D{
 let mask=splat(TOP);
 normalize3([
  _mm512_add_epi64(_mm512_srai_epi64::<26>(t[0]),_mm512_slli_epi64::<26>(_mm512_and_si512(t[1],mask))),
  _mm512_add_epi64(_mm512_srai_epi64::<26>(t[1]),_mm512_slli_epi64::<26>(_mm512_and_si512(t[2],mask))),
  _mm512_srai_epi64::<26>(t[2])
 ])
}

#[inline(always)]
pub(super) unsafe fn multiply(a:G8,b:G8)->G8{
 let mut re=[zero();4];let mut im=[zero();4];let asum=sum3(a.re,a.im);
 row::<false>(&mut re,&mut im,a,asum,b.re[0],b.im[0]);
 cancel::<false>(&mut re,&mut im);shift(&mut re);shift(&mut im);
 row::<false>(&mut re,&mut im,a,asum,b.re[1],b.im[1]);
 cancel::<false>(&mut re,&mut im);shift(&mut re);shift(&mut im);
 row::<true>(&mut re,&mut im,a,asum,b.re[2],b.im[2]);
 let(mr,mi)=cancel::<true>(&mut re,&mut im);
 let mut c0=[zero();3];let mut c1=[zero();3];
 let ar=sign(a.re);let ai=sign(a.im);let br=sign(b.re);let bi=sign(b.im);
 masked_sub(&mut c0,b.re,ar);masked_sub(&mut c0,a.re,br);masked_add(&mut c0,b.im,ai);masked_add(&mut c0,a.im,bi);
 masked_sub(&mut c1,b.im,ar);masked_sub(&mut c1,a.re,bi);masked_sub(&mut c1,b.re,ai);masked_sub(&mut c1,a.im,br);
 let mr=_mm512_cmpge_epu64_mask(mr,splat(1<<25));let mi=_mm512_cmpge_epu64_mask(mi,splat(1<<25));
 masked_sub(&mut c0,digits(A),mr);masked_add(&mut c0,digits(B),mi);masked_sub(&mut c1,digits(B),mr);masked_sub(&mut c1,digits(A),mi);
 G8{re:add3(repack(re),normalize3(c0)),im:add3(repack(im),normalize3(c1))}
}
