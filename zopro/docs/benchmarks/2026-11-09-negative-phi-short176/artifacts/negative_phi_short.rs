//! Private native original forward cubic-map prototype. No global arithmetic dispatch.
use crate::{ifma8::Ifma8,Fq,FqConfig,FpConfig,G2Affine};
use core::{arch::x86_64::*,marker::PhantomData};
const MASK:u64=(1<<52)-1;
pub const SHORT_GAMMA:Fq=Fq::from_decimal("2203960485148121921418603742825762020974279258880205651967");
pub const BETA:Fq=Fq::from_decimal("21888242871839275220042445260109153167277707414472061641714758635765020556616");
const fn words()->[u64;4]{let text=b"2203960485148121921418603742825762020974279258880205651967";let mut out=[0u64;4];let mut k=0;while k<text.len(){let mut carry=(text[k]-b'0')as u128;let mut j=0;while j<4{let v=(out[j]as u128)*10+carry;out[j]=v as u64;carry=v>>64;j+=1;}assert!(carry==0);k+=1;}out}
const fn split4(x:[u64;4])->[u64;4]{let mut out=[0u64;4];let mut j=0;while j<4{let pos=j*52;let w=pos/64;let off=pos%64;let mut value=x[w]>>off;if off!=0&&w<3{value|=x[w+1]<<(64-off);}out[j]=value&MASK;j+=1;}assert!(x[3]>>16==0);out}
const fn quotient()->[u64;4]{let mut rem=words();let mut q=[0u64;5];let mut step=0;while step<260{rem=crate::portable::add4(rem,rem).0;let ge=!crate::portable::less(rem,FqConfig::P);if ge{rem=crate::portable::sub4(rem,FqConfig::P).0;}let mut carry=ge as u64;let mut j=0;while j<5{let v=(q[j]<<1)|carry;q[j]=v&MASK;carry=v>>52;j+=1;}assert!(carry==0);step+=1;}assert!(q[4]==0);[q[0],q[1],q[2],q[3]]}
const FACTOR:[u64;4]=split4(words());const QUOTIENT:[u64;4]=quotient();
impl Ifma8<FqConfig,true>{
    /// Multiply normalized numerical R256 words <=2p by canonical beta using a short negative factor.
    #[inline(always)]
    pub fn mul_phi_short(&self)->Self{unsafe{
        let zero=_mm512_setzero_si512();let mask=_mm512_set1_epi64(MASK as i64);
        let factor=FACTOR.map(|v|_mm512_set1_epi64(v as i64));let quotient=QUOTIENT.map(|v|_mm512_set1_epi64(v as i64));
        let p=crate::FqConfig::P;let p:[__m512i;5]=core::array::from_fn(|j|{let bit=j*52;let w=bit/64;let off=bit%64;let mut v=p[w]>>off;if off!=0&&w<3{v|=p[w+1]<<(64-off);}_mm512_set1_epi64((v&MASK)as i64)});
        let mut high=[zero;6];for i in 0..5{for j in 0..4{let k=i+j;if k==3{high[0]=_mm512_madd52hi_epu64(high[0],self.0[i],quotient[j]);}else if k>=4{high[k-4]=_mm512_madd52lo_epu64(high[k-4],self.0[i],quotient[j]);high[k-3]=_mm512_madd52hi_epu64(high[k-3],self.0[i],quotient[j]);}}}
        for j in 0..5{high[j+1]=_mm512_add_epi64(high[j+1],_mm512_srli_epi64::<52>(high[j]));high[j]=_mm512_and_si512(high[j],mask);}let q=[high[1],high[2],high[3],high[4]];
        // Signed low coefficients represent 2p + Qp - a*gamma modulo B^5.
        let mut out=p.map(|v|_mm512_slli_epi64::<1>(v));
        for i in 0..4{for j in 0..5{let k=i+j;if k>4{continue;}out[k]=_mm512_madd52lo_epu64(out[k],q[i],p[j]);if k<4{out[k+1]=_mm512_madd52hi_epu64(out[k+1],q[i],p[j]);}}}
        for i in 0..5{for j in 0..4{let k=i+j;if k>4{continue;}out[k]=_mm512_sub_epi64(out[k],_mm512_madd52lo_epu64(zero,self.0[i],factor[j]));if k<4{out[k+1]=_mm512_sub_epi64(out[k+1],_mm512_madd52hi_epu64(zero,self.0[i],factor[j]));}}}
        // Only the numerical zero input would produce the excluded endpoint 2p.
        let any=_mm512_or_si512(_mm512_or_si512(self.0[0],self.0[1]),_mm512_or_si512(_mm512_or_si512(self.0[2],self.0[3]),self.0[4]));
        let nonzero=_mm512_cmpneq_epi64_mask(any,zero);
        let mut carry=zero;for digit in &mut out{let value=_mm512_add_epi64(*digit,carry);*digit=_mm512_maskz_mov_epi64(nonzero,_mm512_and_si512(value,mask));carry=_mm512_srai_epi64::<52>(value);}Self(out,PhantomData)
    }}
}
/// Complete in-place X transform, including packing/stores and scalar tails.
pub fn transform(points:&mut[G2Affine]){let full=points.len()/4*4;for block in points[..full].chunks_exact_mut(4){let input=core::array::from_fn(|i|if i%2==0{block[i/2].x.c0}else{block[i/2].x.c1});let out=Ifma8::<FqConfig,true>::new(&input).mul_phi_short().get();for j in 0..4{block[j].x.c0=out[j*2];block[j].x.c1=out[j*2+1];}}for point in &mut points[full..]{if !point.is_identity(){point.x=point.x.mul_by_fp(&BETA);}}}

#[cfg(test)] mod tests { include!("negative_phi_short_tests.rs"); }
