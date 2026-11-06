//! Experimental direct gathers from unchanged AoS coordinate rows.
use crate::{ifma8::Ifma8, FpConfig, FqConfig};
use core::{arch::x86_64::*, marker::PhantomData};

pub struct Gather<'a>(&'a [[u64; 16]]);
impl<'a> Gather<'a> {
    /// Validate canonical Montgomery words without copying or changing storage.
    pub fn checked(points: &'a [[u64; 16]]) -> Option<Self> {
        for point in points {
            for words in point.chunks_exact(4) {
                if !crate::portable::less(words.try_into().unwrap(), FqConfig::P) {
                    return None;
                }
            }
        }
        Some(Self(points))
    }

    #[inline(always)]
    pub fn pack<const FIELD: usize>(&self, indices: [usize; 8]) -> [Ifma8<FqConfig, true>; 2] {
        const {
            assert!(FIELD == 0 || FIELD == 2);
        }
        for index in indices {
            assert!(index < self.0.len(), "source index outside AoS root");
        }
        // SAFETY: this module requires AVX512 IFMA/VL/DQ. The checked indices
        // address initialized u64s in one borrowed allocation. Slice size is
        // bounded by isize::MAX bytes, so all offsets fit signed i64 even at
        // the last word. No padding, cross-allocation or absolute pointers.
        unsafe {
            let offsets: [i64; 8] = indices.map(|i| (i * 16 + FIELD * 4) as i64);
            let offsets = _mm512_loadu_si512(offsets.as_ptr().cast());
            let words: [__m512i; 8] = core::array::from_fn(|word| {
                let at = _mm512_add_epi64(offsets, _mm512_set1_epi64(word as i64));
                _mm512_i64gather_epi64::<8>(at, self.0.as_ptr().cast())
            });
            core::array::from_fn(|part| {
                let [a, b, c, d] = words[part * 4..part * 4 + 4].try_into().unwrap();
                let mask = _mm512_set1_epi64(((1u64 << 52) - 1) as i64);
                Ifma8(
                    [
                        _mm512_and_si512(a, mask),
                        _mm512_and_si512(
                            _mm512_or_si512(_mm512_srli_epi64::<52>(a), _mm512_slli_epi64::<12>(b)),
                            mask,
                        ),
                        _mm512_and_si512(
                            _mm512_or_si512(_mm512_srli_epi64::<40>(b), _mm512_slli_epi64::<24>(c)),
                            mask,
                        ),
                        _mm512_and_si512(
                            _mm512_or_si512(_mm512_srli_epi64::<28>(c), _mm512_slli_epi64::<36>(d)),
                            mask,
                        ),
                        _mm512_srli_epi64::<16>(d),
                    ],
                    PhantomData,
                )
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Fq;
    fn scalar_digits(words: [u64; 4]) -> [u64; 5] {
        let mut out = [0; 5];
        for bit in 0..256 {
            out[bit / 52] |= ((words[bit / 64] >> (bit % 64)) & 1) << (bit % 52);
        }
        out
    }
    fn points(n: usize) -> Vec<[u64; 16]> {
        let mut edges = vec![
            [0; 4],
            [1, 0, 0, 0],
            FqConfig::R,
            crate::portable::sub4(FqConfig::P, [1, 0, 0, 0]).0,
        ];
        for bit in 0..254 {
            let mut w = [0; 4];
            w[bit / 64] = 1u64 << (bit % 64);
            if crate::portable::less(w, FqConfig::P) {
                edges.push(w);
            }
        }
        let mut seed = Fq::from(72913u64);
        (0..n)
            .map(|i| {
                let mut p = [0; 16];
                for f in 0..4 {
                    seed = seed * Fq::from(6364136223846793005u64) + Fq::from(1u64);
                    let w = if i < edges.len() {
                        edges[(i + f) % edges.len()]
                    } else {
                        seed.0
                    };
                    p[f * 4..f * 4 + 4].copy_from_slice(&w);
                }
                p
            })
            .collect()
    }
    fn check(input: &Gather<'_>, points: &[[u64; 16]], ids: [usize; 8]) -> usize {
        for field in [0, 2] {
            let got = if field == 0 {
                input.pack::<0>(ids)
            } else {
                input.pack::<2>(ids)
            };
            for part in 0..2 {
                let old = Ifma8::<FqConfig, true>::new(&ids.map(|i| {
                    Fq::from_montgomery(
                        points[i][(field + part) * 4..(field + part + 1) * 4]
                            .try_into()
                            .unwrap(),
                    )
                }));
                for digit in 0..5 {
                    let mut actual = [0u64; 8];
                    let mut reference = [0u64; 8];
                    // SAFETY: both arrays have exactly eight writable u64s.
                    unsafe {
                        _mm512_storeu_si512(actual.as_mut_ptr().cast(), got[part].0[digit]);
                        _mm512_storeu_si512(reference.as_mut_ptr().cast(), old.0[digit]);
                    }
                    assert_eq!(actual, reference);
                    for lane in 0..8 {
                        let w = points[ids[lane]][(field + part) * 4..(field + part + 1) * 4]
                            .try_into()
                            .unwrap();
                        assert_eq!(actual[lane], scalar_digits(w)[digit]);
                    }
                }
                assert_eq!(got[part].get(), old.get());
            }
        }
        32
    }
    #[test]
    fn raw_limb_oracle_all_indices_and_immutable_inputs() {
        let mut lanes = 0;
        for n in [0, 1, 7, 8, 9, 15, 16, 17, 33, 67, 257] {
            let p = points(n);
            let before = p.clone();
            let view = Gather::checked(&p).unwrap();
            if n > 0 {
                for base in 0..n {
                    for ids in [
                        [base; 8],
                        core::array::from_fn(|i| (base + i) % n),
                        core::array::from_fn(|i| (base + n - i % n) % n),
                        core::array::from_fn(|i| (base + 37 * i) % n),
                    ] {
                        lanes += check(&view, &p, ids);
                    }
                }
            }
            assert_eq!(p, before);
        }
        let p = points(1031);
        let before = p.clone();
        let view = Gather::checked(&p).unwrap();
        let mut seed = 0x923498a8f36387a5u64;
        for _ in 0..8192 {
            let ids = core::array::from_fn(|_| {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                seed as usize % p.len()
            });
            lanes += check(&view, &p, ids);
        }
        assert_eq!(p, before);
        assert_eq!(lanes, 317184);
        eprintln!("AOS_GATHER_COMPONENT_LANES={lanes}");
    }
    #[test]
    fn invalid_roots_and_indices_preserve_input() {
        let mut p = points(9);
        p[8][12..16].copy_from_slice(&FqConfig::P);
        let before = p.clone();
        assert!(Gather::checked(&p).is_none());
        assert_eq!(p, before);
        let p = points(9);
        let before = p.clone();
        let view = Gather::checked(&p).unwrap();
        for at in 0..8 {
            let mut ids = [0; 8];
            ids[at] = 9;
            assert!(std::panic::catch_unwind(|| view.pack::<2>(ids)).is_err());
        }
        assert_eq!(p, before);
        let empty = [];
        let view = Gather::checked(&empty).unwrap();
        assert!(std::panic::catch_unwind(|| view.pack::<0>([0; 8])).is_err());
    }
}
