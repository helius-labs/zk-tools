impl Wide<2> {
    /// Two independent R260 Montgomery products, sharing the left operand.
    /// Each stream has the same coefficient recurrence and bounds as mul.
    #[inline(always)]
    pub fn mul_pair(&self,b:&Self,c:&Self)->(Self,Self){
        unsafe {
            let z=splat(0);let mask=splat(MASK);let p=const{modulus(1)}.map(splat);
            let inv=splat(FqConfig::INV&MASK);let mut l=[z;6];let mut r=[z;6];
            macro_rules! row {($i:literal)=>{{
                macro_rules! term {($j:literal,$k:literal)=>{{
                    l[$j]=_mm512_madd52lo_epu64(l[$j],self.0.0[$j],b.0.0[$i]);
                    r[$j]=_mm512_madd52lo_epu64(r[$j],self.0.0[$j],c.0.0[$i]);
                    l[$k]=_mm512_madd52hi_epu64(l[$k],self.0.0[$j],b.0.0[$i]);
                    r[$k]=_mm512_madd52hi_epu64(r[$k],self.0.0[$j],c.0.0[$i]);
                }};}
                term!(0,1);term!(1,2);term!(2,3);term!(3,4);term!(4,5);
                let ml=_mm512_and_si512(_mm512_madd52lo_epu64(z,l[0],inv),mask);
                let mr=_mm512_and_si512(_mm512_madd52lo_epu64(z,r[0],inv),mask);
                let cl=_mm512_srli_epi64::<52>(_mm512_add_epi64(l[0],mask));
                let cr=_mm512_srli_epi64::<52>(_mm512_add_epi64(r[0],mask));
                let ll=_mm512_madd52hi_epu64(cl,ml,p[0]);
                let lr=_mm512_madd52hi_epu64(cr,mr,p[0]);
                macro_rules! reduce {($j:literal,$k:literal)=>{{
                    l[$j]=_mm512_madd52lo_epu64(l[$j],ml,p[$j]);
                    r[$j]=_mm512_madd52lo_epu64(r[$j],mr,p[$j]);
                    l[$k]=_mm512_madd52hi_epu64(l[$k],ml,p[$j]);
                    r[$k]=_mm512_madd52hi_epu64(r[$k],mr,p[$j]);
                }};}
                reduce!(1,2);reduce!(2,3);reduce!(3,4);reduce!(4,5);
                l[1]=_mm512_add_epi64(l[1],ll);r[1]=_mm512_add_epi64(r[1],lr);
                l=[l[1],l[2],l[3],l[4],l[5],z];r=[r[1],r[2],r[3],r[4],r[5],z];
            }};}
            row!(0);row!(1);row!(2);row!(3);row!(4);
            for j in 0..4 {
                l[j+1]=_mm512_add_epi64(l[j+1],_mm512_srli_epi64::<52>(l[j]));
                r[j+1]=_mm512_add_epi64(r[j+1],_mm512_srli_epi64::<52>(r[j]));
                l[j]=_mm512_and_si512(l[j],mask);r[j]=_mm512_and_si512(r[j],mask);
            }
            // Separate products <4p^2 give REDC <p+4p^2/2^260 <2p.
            (bounded(Ifma8([l[0],l[1],l[2],l[3],l[4]],PhantomData)),
             bounded(Ifma8([r[0],r[1],r[2],r[3],r[4]],PhantomData)))
        }
    }
}
