// Canonical native R256 X with deferred pure host export.
#[derive(Clone,Copy)]
pub struct CanonicalX<C: FpConfig> { value: Ifma8<C,true> }
impl<C:FpConfig> Ifma8<C,true> {
    #[inline(always)] pub fn canonical_x(&self)->CanonicalX<C> { CanonicalX { value: Self::reduce(self.0,C::P) } }
}
impl<C:FpConfig> CanonicalX<C> {
    #[inline(always)] pub fn lanes(&self)->&Ifma8<C,true> { &self.value }
    #[inline(always)] pub fn get(&self)->[Fp<C>;8] {
        let x = self.value;
        // Fully initialize eight repr-transparent Fp values in four stores.
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
            let mut out = core::mem::MaybeUninit::<[Fp<C>; 8]>::uninit();
            let p = out.as_mut_ptr().cast::<__m512i>();
            _mm512_storeu_si512(p, _mm512_shuffle_i64x2::<0x44>(even_lo, odd_lo));
            _mm512_storeu_si512(p.add(1), _mm512_shuffle_i64x2::<0xee>(even_lo, odd_lo));
            _mm512_storeu_si512(p.add(2), _mm512_shuffle_i64x2::<0x44>(even_hi, odd_hi));
            _mm512_storeu_si512(p.add(3), _mm512_shuffle_i64x2::<0xee>(even_hi, odd_hi));
            out.assume_init()
        }
    }
}
#[cfg(test)]
mod early_x_tests {
    use super::*;
    use ark_ff::{Field,UniformRand,AdditiveGroup};
    use ark_ref as ark;
    #[test]
    fn canonical_x_ranges_and_export_match_arkworks() {
        let mut rng=ark_std::test_rng();
        let p=crate::Fq::MODULUS;
        let mut count=0;
        for step in 0..4096 {
            let expected:[ark::Fq;8]=core::array::from_fn(|i|match (step+i)%11 {0=>ark::Fq::ZERO,1=>ark::Fq::ONE,2=>-ark::Fq::ONE,_=>ark::Fq::rand(&mut rng)});
            let raw=core::array::from_fn(|i|{let x=expected[i].0.0;if (step+i)%2==0{crate::portable::add4(x,p).0}else{x}});
            let loose=Ifma8::<crate::FqConfig,true>::raw(&raw);
            let canonical=loose.canonical_x();
            let got=canonical.get();
            assert_eq!(got,loose.get());
            assert_eq!(got,canonical.lanes().get());
            for i in 0..8 {assert_eq!(got[i].0,expected[i].0.0);assert!(crate::portable::less(got[i].0,p));count+=1;}
            // Both operands are canonical here; unreduced p+a-b is <2p.
            let a=core::array::from_fn(|i|crate::Fq::from_montgomery(expected[(i+3)%8].0.0));
            let d=Ifma8::<crate::FqConfig,true>::new(&a).sub_canonical(canonical.lanes()).get();
            for i in 0..8 {assert_eq!(d[i].0,(expected[(i+3)%8]-expected[i]).0.0);count+=1;}
        }
        assert_eq!(count,65536);
        println!("65536 independent canonical-export/difference lane checks; lazy input spans0..2p");
    }
}
