impl<C:FpConfig,const MIXED:bool>Ifma8<C,MIXED>{
    /// a*(anchor-x) in the quadratic extension, for canonical anchor and
    /// a,x below2p. Wider deltas never escape this operation. Outputs<2p.
    #[inline(always)]
    pub fn complex_canonical_delta(a0:&Self,a1:&Self,anchor0:&Self,anchor1:&Self,x0:&Self,x1:&Self)->[Self;2]{
        assert!(C::P[3]<(1u64<<62));
        // delta=2p+anchor-x is nonnegative and <=3p. All five limbs are
        // normalized before IFMA. Since4p<R, each unsigned fused wide
        // product T<=12p²<3pR gives REDC(T)<4p. The existing MIXED fused
        // correction by2p returns<2p. R260 needs no correction forBN254.
        let loose=|anchor:&Self,x:&Self|unsafe{
            let pp=split(crate::portable::add4(C::P,C::P).0).map(Self::splat);
            let mut out=[Self::splat(0);5];let mut carry=Self::splat(0);let mask=Self::splat(MASK);
            for j in 0..5{let v=_mm512_add_epi64(_mm512_sub_epi64(_mm512_add_epi64(pp[j],anchor.0[j]),x.0[j]),carry);out[j]=_mm512_and_si512(v,mask);carry=_mm512_srai_epi64::<52>(v);}
            Self(out,PhantomData)
        };
        let d0=loose(anchor0,x0);let d1=loose(anchor1,x1);
        [a0.difference2(&d0,a1,&d1),a0.sum2(&d1,a1,&d0)]
    }
}
