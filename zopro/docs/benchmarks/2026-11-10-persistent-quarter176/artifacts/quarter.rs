//! Private native feasibility prototype; values carry strict numerical bounds.
use super::{Ifma8, MASK};
use crate::{FpConfig, Fq, FqConfig};
use core::{arch::x86_64::*, marker::PhantomData};
type Raw = Ifma8<FqConfig, false>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Stored([u64; 4]);
impl Stored {
    pub fn words(self) -> [u64; 4] { self.0 }
    pub fn checked(w: [u64; 4]) -> Self {
        assert!(crate::portable::less(w, crate::portable::add4(FqConfig::P,FqConfig::P).0));
        Self(w)
    }
    pub fn encode(y: Fq) -> Self {
        let mut w=y.0;
        for _ in 0..2 {
            if w[0]&1!=0 { w=crate::portable::add4(w,FqConfig::P).0; }
            w=core::array::from_fn(|j|w[j]>>1 | if j<3 {w[j+1]<<63} else {0});
        }
        Self(w)
    }
    pub fn decode(self) -> Fq {
        let w=crate::portable::reduce_once(self.0,FqConfig::P);
        Fq::from_montgomery(w).double().double()
    }
}
#[derive(Clone,Copy)]
pub struct Wide<const B:u64>(Raw);
fn splat(x:u64)->__m512i { unsafe{_mm512_set1_epi64(x as i64)} }
const fn modulus(n:u64)->[u64;5] {
    let p=super::split(FqConfig::P);let mut o=[0;5];let mut c=0u128;let mut j=0;
    while j<5 {let t=p[j] as u128*n as u128+c;o[j]=t as u64&MASK;c=t>>52;j+=1;}
    assert!(c==0);o
}
fn valid<const B:u64>(x:&Raw)->bool {
    let a:[[u64;8];5]=unsafe{core::mem::transmute(x.0)};let p=const{modulus(B)};
    (0..8).all(|i| {let cmp=(0..5).rev().map(|j|a[j][i]).cmp(p.iter().rev().copied());
       (0..5).all(|j|a[j][i]<=MASK) && (cmp.is_lt() || B==3 && cmp.is_eq())})
}
#[inline(always)]
fn bounded<const B:u64>(x:Raw)->Wide<B>{debug_assert!(valid::<B>(&x));Wide(x)}
impl<const B:u64> Wide<B> {
    pub fn words(&self)->[[u64;4];8] {
        assert!(B<=4);export_raw(&self.0).map(|x|x.0)
    }
}
/// Used only by the independent arithmetic qualification, not a field constructor.
pub fn checked<const B:u64>(x:[[u64;4];8])->Wide<B>{let a=Raw::raw(&x);assert!(valid::<B>(&a));Wide(a)}
#[inline(always)]
pub fn x(a:&[Fq;8])->Wide<1>{bounded(Raw::raw(&a.map(|v|v.0)))}
#[inline(always)]
pub fn load_y(a:&[Stored;8])->Wide<2>{bounded(Raw::raw(&a.map(|v|v.0)))}
#[inline(always)]
pub fn incoming_y(a:&[Fq;8])->Wide<1>{
    unsafe {
        let raw=Raw::raw(&a.map(|v|v.0));let p=const{modulus(1)}.map(splat);
        let k=_mm512_and_si512(raw.0[0],splat(3));let mut t=[splat(0);5];let mut c=splat(0);
        for j in 0..5 {let v=_mm512_add_epi64(_mm512_add_epi64(raw.0[j],_mm512_mullo_epi64(k,p[j])),c);
            c=_mm512_srli_epi64::<52>(v);t[j]=_mm512_and_si512(v,splat(MASK));}
        debug_assert_eq!(_mm512_cmpneq_epi64_mask(c,splat(0)),0);
        for j in 0..4 {t[j]=_mm512_or_si512(_mm512_srli_epi64::<2>(t[j]),
            _mm512_and_si512(_mm512_slli_epi64::<50>(t[j+1]),splat(MASK)));}
        t[4]=_mm512_srli_epi64::<2>(t[4]);bounded(Ifma8(t,PhantomData))
    }
}
#[inline(always)]
fn add<const A:u64,const B:u64,const O:u64>(a:&Wide<A>,b:&Wide<B>)->Wide<O>{
    const{assert!(O>=A+B)};
    unsafe {let mut t=core::array::from_fn(|j|_mm512_add_epi64(a.0.0[j],b.0.0[j]));
        for j in 0..4 {t[j+1]=_mm512_add_epi64(t[j+1],_mm512_srli_epi64::<52>(t[j]));t[j]=_mm512_and_si512(t[j],splat(MASK));}
        bounded(Ifma8(t,PhantomData))}
}
#[inline(always)]
fn sub<const A:u64,const B:u64,const C:u64,const P:u64,const O:u64>(a:&Wide<A>,b:&Wide<B>,c:&Wide<C>)->Wide<O>{
    const{assert!(P>=B+C && O>=P+A)};
    unsafe{let p=const{modulus(P)}.map(splat);let mut t=[splat(0);5];let mut carry=splat(0);
        for j in 0..5 {let v=_mm512_add_epi64(_mm512_sub_epi64(_mm512_sub_epi64(
            _mm512_add_epi64(p[j],a.0.0[j]),b.0.0[j]),c.0.0[j]),carry);
            carry=_mm512_srai_epi64::<52>(v);t[j]=_mm512_and_si512(v,splat(MASK));}
        debug_assert_eq!(_mm512_cmpneq_epi64_mask(carry,splat(0)),0);bounded(Ifma8(t,PhantomData))}
}
fn zero()->Wide<0>{Wide(Raw::zero())}
#[inline(always)]
pub fn dx(b:&Wide<1>,a:&Wide<1>)->Wide<2>{sub::<1,1,0,1,2>(b,a,&zero())}
#[inline(always)]
pub fn dy(b:&Wide<1>,a:&Wide<2>,negative:u8)->Wide<3>{
    unsafe {let two=const{modulus(2)};let three=const{modulus(3)};
        let flip=_mm512_maskz_mov_epi64(negative,splat(MASK));
        let mut carry=_mm512_maskz_mov_epi64(negative,splat(1));let mut t=[splat(0);5];
        for j in 0..5 {let p=_mm512_mask_blend_epi64(negative,splat(two[j]),splat(three[j]));
            let v=_mm512_add_epi64(_mm512_sub_epi64(_mm512_add_epi64(p,
                _mm512_xor_si512(b.0.0[j],flip)),a.0.0[j]),carry);
            carry=_mm512_srai_epi64::<52>(v);t[j]=_mm512_and_si512(v,splat(MASK));}
        // Inclusive3p for negative by=ay=0; otherwise strictly less.
        debug_assert_eq!(_mm512_cmpneq_epi64_mask(carry,_mm512_maskz_mov_epi64(negative,splat(1))),0);
        bounded(Ifma8(t,PhantomData))}
}
#[inline(always)]
fn product<const A:u64,const B:u64>(a:&Wide<A>,b:&Wide<B>)->Wide<2>{
    const{assert!(A*B<=64)};bounded(a.0.product_cios::<false>(&b.0,&a.0,&b.0))
}
#[inline(always)]
fn fused<const A:u64,const B:u64,const C:u64,const D:u64>(a:&Wide<A>,b:&Wide<B>,c:&Wide<C>,d:&Wide<D>)->Wide<2>{
    const{assert!(A*B+C*D<=64)};bounded(a.0.product_cios::<true>(&b.0,&c.0,&d.0))
}
impl Wide<2>{
    pub fn one()->Self{bounded(Raw::one())}
    #[inline(always)] pub fn mul(&self,b:&Self)->Self{product(self,b)}
    #[inline] pub fn inverse_nonzero(&self)->Self{
        let swapped=bounded::<2>(Raw::shuffle_pair::<1,0,3,2>(&self.0,&self.0));let pairs=self.mul(&swapped);
        let other=bounded::<2>(Raw::shuffle_pair::<2,3,0,1>(&pairs.0,&pairs.0));let quads=pairs.mul(&other);
        let p=quads.0.get();let inv=(p[0]*p[4]).inverse().unwrap();let(lo,hi)=quads.0.halves();
        let otherq=bounded::<2>(Raw::from_halves(&hi,&lo));bounded::<2>(Raw::new(&[inv;8])).mul(&otherq).mul(&other).mul(&swapped)
    }
}
#[inline(always)]pub fn norm(d:&[Wide<2>;2])->Wide<2>{fused(&d[0],&d[0],&d[1],&d[1])}
#[inline(always)]pub fn numerator(dy:&[Wide<3>;2],dx:&[Wide<2>;2])->[Wide<2>;2]{
    let neg=sub::<0,3,0,3,3>(&zero(),&dy[0],&zero());
    [fused(&dy[0],&dx[0],&dy[1],&dx[1]),fused(&dy[1],&dx[0],&neg,&dx[1])]
}
#[inline(always)]pub fn slope_g1(dy:&Wide<3>,inv:&Wide<2>)->Wide<2>{product(dy,inv)}
#[inline(always)]pub fn square_g1(s:&Wide<2>)->Wide<2>{product(s,s)}
#[inline(always)]pub fn square(s:&[Wide<2>;2])->[Wide<2>;2]{
    let sum=add::<2,2,4>(&s[0],&s[1]);let diff=sub::<2,2,0,2,4>(&s[0],&s[1],&zero());
    let twice=add::<2,2,4>(&s[1],&s[1]);[product(&sum,&diff),product(&s[0],&twice)]
}
#[inline(always)]pub fn xcoord(s:&Wide<2>,a:&Wide<1>,b:&Wide<1>)->Wide<1>{
    let r=sub::<2,1,1,2,4>(s,a,b);let p2=crate::portable::add4(FqConfig::P,FqConfig::P).0;
    bounded(Raw::reduce(Raw::reduce(r.0.0,p2).0,FqConfig::P))
}
#[inline(always)]fn retain_y(p:&Wide<2>,ay:&Wide<2>)->Wide<2>{
    let r=sub::<2,2,0,2,4>(p,ay,&zero());bounded(Raw::reduce(r.0.0,crate::portable::add4(FqConfig::P,FqConfig::P).0))
}
#[inline(always)]pub fn finish_g1(s:&Wide<2>,d:&Wide<2>,ay:&Wide<2>)->Wide<2>{retain_y(&product(s,d),ay)}
#[inline(always)]pub fn finish(s:&[Wide<2>;2],d:&[Wide<2>;2],ay:&[Wide<2>;2])->[Wide<2>;2]{
    let neg=sub::<0,2,0,2,2>(&zero(),&s[1],&zero());
    [retain_y(&fused(&s[0],&d[0],&neg,&d[1]),&ay[0]),
     retain_y(&fused(&s[0],&d[1],&s[1],&d[0]),&ay[1])]
}
#[inline(always)]pub fn store_y(a:&Wide<2>)->[Stored;8]{export_raw(&a.0)}
#[inline(always)]pub fn export_x(a:&Wide<1>)->[Fq;8]{export_raw(&a.0).map(|v|Fq::from_montgomery(v.0))}

// The preparer appends the original normalized packing/transpose export here,
// changing only its transparent target from Fp to Stored, with no reduction.

#[inline(always)]
fn export_raw(x:&Raw)->[Stored;8]{
        unsafe {
            let [t0, t1, t2, t3, t4] = x.0;
            let w0 = _mm512_or_si512(t0, _mm512_slli_epi64::<52>(t1));
            let w1 = _mm512_or_si512(_mm512_srli_epi64::<12>(t1), _mm512_slli_epi64::<40>(t2));
            let w2 = _mm512_or_si512(_mm512_srli_epi64::<24>(t2), _mm512_slli_epi64::<28>(t3));
            let w3 = _mm512_or_si512(_mm512_srli_epi64::<36>(t3), _mm512_slli_epi64::<16>(t4));
            let a = _mm512_unpacklo_epi64(w0, w1);
            let b = _mm512_unpackhi_epi64(w0, w1);
            let c = _mm512_unpacklo_epi64(w2, w3);
            let d = _mm512_unpackhi_epi64(w2, w3);
            let low = _mm512_set_epi64(11, 10, 3, 2, 9, 8, 1, 0);
            let high = _mm512_set_epi64(15, 14, 7, 6, 13, 12, 5, 4);
            let even_lo = _mm512_permutex2var_epi64(a, low, c);
            let even_hi = _mm512_permutex2var_epi64(a, high, c);
            let odd_lo = _mm512_permutex2var_epi64(b, low, d);
            let odd_hi = _mm512_permutex2var_epi64(b, high, d);
            let mut out = core::mem::MaybeUninit::<[Stored; 8]>::uninit();
            let p = out.as_mut_ptr().cast::<__m512i>();
            _mm512_storeu_si512(p, _mm512_shuffle_i64x2::<0x44>(even_lo, odd_lo));
            _mm512_storeu_si512(p.add(1), _mm512_shuffle_i64x2::<0xee>(even_lo, odd_lo));
            _mm512_storeu_si512(p.add(2), _mm512_shuffle_i64x2::<0x44>(even_hi, odd_hi));
            _mm512_storeu_si512(p.add(3), _mm512_shuffle_i64x2::<0xee>(even_hi, odd_hi));
            out.assume_init()
        }
}
