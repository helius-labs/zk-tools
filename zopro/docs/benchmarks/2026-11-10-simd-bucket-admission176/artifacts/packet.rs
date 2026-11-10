//! Conservative eight-update admission. No state changes occur in this helper.
//! Indices and their bitmap words are validated/gathered by the safe caller.
#[inline]
pub(super) fn admissible(indices: &[u64; 8], words: &[u64; 8], bucket_x: &[u64; 8], incoming_x: &[u64; 8]) -> bool {
    use core::arch::x86_64::*;
    // SAFETY: this module is selected only with native AVX512F+AVX512CD.
    // Every unaligned vector load reads exactly one initialized 64-byte array.
    unsafe {
        let zero = _mm512_setzero_si512();
        let ids = _mm512_loadu_si512(indices.as_ptr().cast());
        let conflicts = _mm512_conflict_epi64(ids);
        if _mm512_cmpneq_epi64_mask(conflicts, zero) != 0 { return false; }
        let slots = _mm512_loadu_si512(words.as_ptr().cast());
        let bits = _mm512_sllv_epi64(_mm512_set1_epi64(1), _mm512_and_si512(ids, _mm512_set1_epi64(63)));
        let pending = _mm512_and_si512(slots, bits);
        let a = _mm512_loadu_si512(bucket_x.as_ptr().cast());
        let b = _mm512_loadu_si512(incoming_x.as_ptr().cast());
        (_mm512_cmpneq_epi64_mask(pending, zero)
            | _mm512_cmpeq_epi64_mask(a, zero)
            | _mm512_cmpeq_epi64_mask(b, zero)
            | _mm512_cmpeq_epi64_mask(a, b)) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn reference(ids: &[u64; 8], words: &[u64; 8], a: &[u64; 8], b: &[u64; 8]) -> bool {
        for i in 0..8 {
            if a[i] == 0 || b[i] == 0 || a[i] == b[i] || words[i] >> (ids[i] % 64) & 1 != 0 { return false; }
            for j in 0..i { if ids[i] == ids[j] { return false; } }
        }
        true
    }
    #[test]
    fn masks_duplicates_and_first_limb_conservative_cases() {
        let base = [0, 1, 63, 64, 127, 128, 2047, 8191];
        let a = [2, 3, 4, 5, 6, 7, 8, 9]; let b = [12, 13, 14, 15, 16, 17, 18, 19];
        let clear = [0; 8]; assert!(admissible(&base, &clear, &a, &b));
        let mut checks = 1;
        for mask in 0..256u32 {
            let words = core::array::from_fn(|i| if mask >> i & 1 != 0 { 1 << (base[i] % 64) } else { 0 });
            assert_eq!(admissible(&base,&words,&a,&b),reference(&base,&words,&a,&b)); checks += 1;
            for kind in 0..3 { let mut a = a; let mut b = b;
                for i in 0..8 { if mask >> i & 1 != 0 { match kind {0 => a[i]=0, 1 => b[i]=0, _ => b[i]=a[i]} } }
                assert_eq!(admissible(&base,&clear,&a,&b),reference(&base,&clear,&a,&b)); checks += 1;
            }
        }
        for i in 0..8 { for j in 0..8 { let mut ids = base; ids[i] = ids[j];
            assert_eq!(admissible(&ids,&clear,&a,&b),reference(&ids,&clear,&a,&b)); checks += 1;
        } }
        // Same bitmap word with different bits is eligible; only equal bucket
        // indices conflict. This specifically catches a word-index conflict bug.
        let ids = [0,1,2,3,4,5,6,7];
        assert!(admissible(&ids,&[1<<63;8],&a,&b)); checks += 1;
        let mut state = 0x893b_17f2_4ae5u64;
        for _ in 0..65536 {
            let mut next = || { state ^= state << 13; state ^= state >> 7; state ^= state << 17; state };
            let ids = core::array::from_fn(|_|next() % 8192);
            let words = core::array::from_fn(|_|next() & next() & next() & next());
            let a = core::array::from_fn(|_|next()); let b = core::array::from_fn(|_|next());
            assert_eq!(admissible(&ids,&words,&a,&b),reference(&ids,&words,&a,&b)); checks += 1;
        }
        assert_eq!(checks, 66626);
        println!("PACKET_MASK_CHECKS={checks};independent_scalar_predicate;no_timing_claim");
    }
}
