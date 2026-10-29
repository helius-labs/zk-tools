impl<C: FpConfig, const MIXED: bool> Ifma8<C, MIXED> {
    #[inline(always)]
    fn pair_negative(&self) -> Self {
        unsafe {
            let p=split(crate::portable::add4(C::P,C::P).0).map(Self::splat);
            let mask=Self::splat(MASK);let mut out=[Self::splat(0);5];let mut carry=Self::splat(0);
            for j in 0..5{let v=_mm512_add_epi64(_mm512_sub_epi64(p[j],self.0[j]),carry);carry=_mm512_srai_epi64::<52>(v);out[j]=_mm512_and_si512(v,mask);}
            Self(out,PhantomData)
        }
    }
    #[inline(always)]
    pub fn complex_rowq<const CONJ:bool>(a0:&Self,a1:&Self,b0:&Self,b1:&Self)->[Self;2]{
        unsafe {
            let zero=Self::splat(0);let inv=Self::splat(C::INV&MASK);
            let u=_mm512_madd52lo_epu64(zero,a0.0[0],inv);let v=_mm512_madd52lo_epu64(zero,a1.0[0],inv);
            let uv=_mm512_add_epi64(u,v);let two=Self::splat(2*(1u64<<52));let four=Self::splat(4*(1u64<<52));let base=Self::splat(1u64<<52);
            let mut real=[zero;5];let mut imag=[zero;5];
            for i in 0..5{
                let both=_mm512_add_epi64(b0.0[i],b1.0[i]);let twice_b1=_mm512_slli_epi64::<1>(b1.0[i]);
                if CONJ {
                    let p=_mm512_madd52lo_epu64(zero,uv,b0.0[i]);
                    let q=_mm512_madd52lo_epu64(zero,u,both);
                    let t=_mm512_madd52lo_epu64(zero,v,_mm512_sub_epi64(_mm512_add_epi64(base,b1.0[i]),b0.0[i]));
                    real[i]=_mm512_add_epi64(p,t);
                    imag[i]=_mm512_sub_epi64(_mm512_sub_epi64(_mm512_add_epi64(p,four),q),twice_b1);
                } else {
                    let x=_mm512_madd52lo_epu64(zero,u,b0.0[i]);let y=_mm512_madd52lo_epu64(zero,v,b1.0[i]);let z=_mm512_madd52lo_epu64(zero,uv,both);
                    real[i]=_mm512_sub_epi64(_mm512_sub_epi64(_mm512_add_epi64(x,four),y),twice_b1);
                    imag[i]=_mm512_sub_epi64(_mm512_sub_epi64(_mm512_add_epi64(z,two),x),y);
                }
            }
            if CONJ {
                let neg=a0.pair_negative();
                [a0.product_pairrow::<true>(b0,a1,b1,&real),a1.product_pairrow::<true>(b0,&neg,b1,&imag)]
            } else {
                let neg=a1.pair_negative();
                [a0.product_pairrow::<true>(b0,&neg,b1,&real),a0.product_pairrow::<true>(b1,a1,b0,&imag)]
            }
        }
    }
