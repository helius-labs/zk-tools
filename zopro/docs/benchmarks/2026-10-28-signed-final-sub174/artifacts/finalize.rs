// Coefficients still represent the ordinary REDC integer, but need not
// be normalized digits. Fold subtraction into its final carry pass.
#[inline(always)]
fn finish_sub(mut t: [__m512i; 6], c: &Self) -> Self {
    unsafe {
        #[cfg(test)]
        for limb in &t[..5] {
            let values: [u64; 8] = core::mem::transmute(*limb);
            assert!(values.iter().all(|&v| v < (1u64 << 61)));
        }
        let mask = Self::splat(MASK);
        let zero = Self::splat(0);
        let mut carry = zero;
        for j in 0..5 {
            let value = _mm512_add_epi64(_mm512_sub_epi64(t[j], c.0[j]), carry);
            t[j] = _mm512_and_si512(value, mask);
            carry = _mm512_srai_epi64::<52>(value);
        }
        let negative = _mm512_cmpneq_epi64_mask(carry, zero);
        let twice = split(crate::portable::add4(C::P, C::P).0).map(Self::splat);
        for j in 0..5 {
            t[j] = _mm512_add_epi64(t[j], _mm512_maskz_mov_epi64(negative, twice[j]));
        }
        for j in 0..4 {
            t[j + 1] = _mm512_add_epi64(t[j + 1], _mm512_srli_epi64::<52>(t[j]));
            t[j] = _mm512_and_si512(t[j], mask);
        }
        t[4] = _mm512_and_si512(t[4], mask);
        Self([t[0], t[1], t[2], t[3], t[4]], PhantomData)
    }
}
