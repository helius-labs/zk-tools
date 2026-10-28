//! Pack eight referenced host fields without a contiguous field staging array.
use crate::{ifma8::Ifma8, Fp, FpConfig};
use core::{arch::x86_64::*, marker::PhantomData};

impl<C: FpConfig> Ifma8<C, true> {
    /// Preserve raw mixed-R256 Montgomery words, as the original constructor.
    #[inline(always)]
    pub fn from_refs(x: [&Fp<C>; 8]) -> Self {
        // SAFETY: each reference owns four initialized u64 words for the full
        // 32-byte load. Loads permit read aliasing and need no 32-byte alignment.
        // The module guard requires all used CPU features. Every permutation
        // index addresses one of the two provided vector operands.
        unsafe {
            let r: [__m512i; 4] = core::array::from_fn(|i| {
                let lo = _mm256_loadu_si256(x[2 * i].0.as_ptr().cast());
                let hi = _mm256_loadu_si256(x[2 * i + 1].0.as_ptr().cast());
                _mm512_inserti64x4::<1>(_mm512_castsi256_si512(lo), hi)
            });
            let a = _mm512_unpacklo_epi64(r[0], r[1]);
            let b = _mm512_unpackhi_epi64(r[0], r[1]);
            let c = _mm512_unpacklo_epi64(r[2], r[3]);
            let d = _mm512_unpackhi_epi64(r[2], r[3]);
            let low = _mm512_set_epi64(13, 9, 12, 8, 5, 1, 4, 0);
            let high = _mm512_set_epi64(15, 11, 14, 10, 7, 3, 6, 2);
            let w0 = _mm512_permutex2var_epi64(a, low, c);
            let w1 = _mm512_permutex2var_epi64(b, low, d);
            let w2 = _mm512_permutex2var_epi64(a, high, c);
            let w3 = _mm512_permutex2var_epi64(b, high, d);
            let mask = _mm512_set1_epi64(((1u64 << 52) - 1) as i64);
            Self(
                [
                    _mm512_and_si512(w0, mask),
                    _mm512_and_si512(
                        _mm512_or_si512(_mm512_srli_epi64::<52>(w0), _mm512_slli_epi64::<12>(w1)),
                        mask,
                    ),
                    _mm512_and_si512(
                        _mm512_or_si512(_mm512_srli_epi64::<40>(w1), _mm512_slli_epi64::<24>(w2)),
                        mask,
                    ),
                    _mm512_and_si512(
                        _mm512_or_si512(_mm512_srli_epi64::<28>(w2), _mm512_slli_epi64::<36>(w3)),
                        mask,
                    ),
                    _mm512_srli_epi64::<16>(w3),
                ],
                PhantomData,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{portable, FqConfig, FrConfig};

    #[repr(C)]
    struct Record<C: FpConfig> {
        tag: u64,
        value: Fp<C>,
        other: Fp<C>,
    }

    fn check<C: FpConfig>() {
        let p = C::P;
        let two_p = portable::add4(p, p).0;
        let boundaries = [
            [0; 4],
            [1, 0, 0, 0],
            portable::sub4(p, [1, 0, 0, 0]).0,
            p,
            portable::add4(p, [1, 0, 0, 0]).0,
            portable::sub4(two_p, [1, 0, 0, 0]).0,
        ];
        let mut seed = 0x3417_b0fc_a820_6de9u64;
        let mut next = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        for block in 0..518 {
            let records: Vec<_> = (0..16)
                .map(|i| {
                    let mut a = if block < 6 {
                        boundaries[(block + i) % 6]
                    } else {
                        [next(), next(), next(), next() & ((1 << 63) - 1)]
                    };
                    if !portable::less(a, two_p) {
                        a = portable::sub4(a, two_p).0;
                    }
                    Record {
                        tag: i as u64,
                        value: Fp::<C>::from_montgomery(a),
                        other: Fp::<C>::ONE,
                    }
                })
                .collect();
            assert!(records
                .iter()
                .any(|r| std::ptr::from_ref(&r.value) as usize % 32 != 0));
            for kind in 0..3 {
                let refs = core::array::from_fn(|i| {
                    &records[match kind {
                        0 => i * 2,
                        1 => 15 - i * 2,
                        _ => 3,
                    }]
                    .value
                });
                let got = Ifma8::<C, true>::from_refs(refs);
                let old = Ifma8::<C, true>::new(&refs.map(|x| *x));
                // SAFETY: vector integer bits are initialized by the constructors.
                let actual: [[u64; 8]; 5] = unsafe { core::mem::transmute(got.0) };
                let original: [[u64; 8]; 5] = unsafe { core::mem::transmute(old.0) };
                assert_eq!(actual, original);
                for i in 0..8 {
                    let x = refs[i].0;
                    let mask = (1 << 52) - 1;
                    let split = [
                        x[0] & mask,
                        ((x[0] >> 52) | (x[1] << 12)) & mask,
                        ((x[1] >> 40) | (x[2] << 24)) & mask,
                        ((x[2] >> 28) | (x[3] << 36)) & mask,
                        x[3] >> 16,
                    ];
                    for j in 0..5 {
                        assert_eq!(actual[j][i], split[j]);
                    }
                }
                assert_eq!(got.get(), old.get());
                assert_eq!(got.mul(&got).get(), old.mul(&old).get());
            }
        }
    }
    #[test]
    fn direct_reference_words_match_split_for_unaligned_permuted_and_aliased_inputs() {
        check::<FqConfig>();
        check::<FrConfig>();
    }
}
