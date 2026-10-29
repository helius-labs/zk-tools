    /// Exact five-coefficient normalization for unsigned t[j] < 2^61.
    /// The top digit retains the ordinary ripple's unrestricted high word.
    #[inline(always)]
    fn normalize_masked(t: [__m512i; 5]) -> [__m512i; 5] {
        unsafe {
            let mask = Self::splat(MASK);
            let one = Self::splat(1);
            let v1 = _mm512_add_epi64(_mm512_and_si512(t[1],mask),_mm512_srli_epi64::<52>(t[0]));
            let v2 = _mm512_add_epi64(_mm512_and_si512(t[2],mask),_mm512_srli_epi64::<52>(t[1]));
            let v3 = _mm512_add_epi64(_mm512_and_si512(t[3],mask),_mm512_srli_epi64::<52>(t[2]));
            let v4 = _mm512_add_epi64(t[4],_mm512_srli_epi64::<52>(t[3]));
            let c1 = _mm512_cmpgt_epu64_mask(v1,mask);
            let c2 = _mm512_cmpgt_epu64_mask(v2,mask) | (_mm512_cmpeq_epi64_mask(v2,mask) & c1);
            let c3 = _mm512_cmpgt_epu64_mask(v3,mask) | (_mm512_cmpeq_epi64_mask(v3,mask) & c2);
            [
                _mm512_and_si512(t[0],mask),
                _mm512_and_si512(v1,mask),
                _mm512_and_si512(_mm512_mask_add_epi64(v2,c1,v2,one),mask),
                _mm512_and_si512(_mm512_mask_add_epi64(v3,c2,v3,one),mask),
                _mm512_mask_add_epi64(v4,c3,v4,one),
            ]
        }
    }
