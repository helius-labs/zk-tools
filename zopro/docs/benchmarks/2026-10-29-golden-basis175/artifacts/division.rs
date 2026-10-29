
impl<C:FpConfig,const MIXED:bool>Ifma8<C,MIXED>{
    /// Divide an inclusive [0,2p] Montgomery representative by three modulo p.
    /// Only p=1 mod3 is supported. All digits are normalized radix 2^52.
    #[inline(always)]
    pub fn golden_div3(&self)->Self {
        assert_eq!(C::P,crate::FqConfig::P);
        unsafe {
            let zero=Self::splat(0);let mask=Self::splat(MASK);let three=Self::splat(3);
            let sum=self.0.into_iter().fold(zero,|s,v|_mm512_add_epi64(s,v));
            let low=_mm512_and_si512(sum,mask);let high=_mm512_srli_epi64::<52>(sum);
            let q=_mm512_madd52hi_epu64(zero,low,Self::splat(MASK/3));
            let triple=_mm512_add_epi64(q,_mm512_slli_epi64::<1>(q));
            let mut residue=_mm512_add_epi64(_mm512_sub_epi64(low,triple),high);
            for _ in 0..2 {residue=_mm512_mask_sub_epi64(residue,_mm512_cmpge_epu64_mask(residue,three),residue,three);}
            let add_p=_mm512_cmpeq_epi64_mask(residue,Self::splat(2));
            let add_two=_mm512_cmpeq_epi64_mask(residue,Self::splat(1));
            let p=split(C::P).map(Self::splat);let two=split(crate::portable::add4(C::P,C::P).0).map(Self::splat);
            let mut carry=zero;let mut value=[zero;5];
            for j in 0..5 {
                let added=_mm512_or_si512(_mm512_maskz_mov_epi64(add_p,p[j]),_mm512_maskz_mov_epi64(add_two,two[j]));
                let v=_mm512_add_epi64(_mm512_add_epi64(self.0[j],added),carry);
                value[j]=_mm512_and_si512(v,mask);carry=_mm512_srli_epi64::<52>(v);
            }
            let inv3=Self::splat((2*(1u64<<52)+1)/3);carry=zero;
            let mut out=[zero;5];
            for j in 0..5 {
                let v=_mm512_sub_epi64(value[j],carry);
                out[j]=_mm512_and_si512(_mm512_madd52lo_epu64(zero,v,inv3),mask);
                let triple=_mm512_add_epi64(out[j],_mm512_slli_epi64::<1>(out[j]));
                carry=_mm512_srli_epi64::<52>(_mm512_sub_epi64(triple,v));
            }
            Self(out,PhantomData)
        }
    }
    #[inline(always)]
    pub fn golden_square(a:&Self,b:&Self)->[Self;2] {
        let twice=b.add(b);let p=a.mul(&a.sub(&twice));
        let q=a.add(b).mul(&a.add(&twice.add(b)));
        let v=q.sub(&p).golden_div3();[p.add(&v),v]
    }
}
