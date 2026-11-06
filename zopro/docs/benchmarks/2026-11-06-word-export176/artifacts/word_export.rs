// Diagnostic extension included in the unchanged generated ifma8 module.
impl Ifma8<crate::FqConfig, true> {
    /// Canonical mixed-R256 Fq export from normalized values strictly below 2p.
    #[inline(always)]
    pub fn get_word_canonical(&self) -> [crate::Fq; 8] {
        const P: [u64; 4] = crate::FqConfig::P;
        const {
            assert!(P[0] < u64::MAX && P[1] < u64::MAX && P[2] < u64::MAX && P[3] < u64::MAX);
        }
        // SAFETY: same AVX512 requirements and initialized four-store output
        // contract as get(). Packing is lossless for normalized values <2p.
        unsafe {
            let original = self.compact().0;
            let mut difference = [Self::splat(0); 4];
            let mut borrow = 0u8;
            for j in 0..4 {
                let adjusted = _mm512_mask_add_epi64(
                    Self::splat(P[j]),
                    borrow,
                    Self::splat(P[j]),
                    Self::splat(1),
                );
                difference[j] = _mm512_sub_epi64(original[j], adjusted);
                borrow = _mm512_cmplt_epu64_mask(original[j], adjusted);
            }
            let [w0, w1, w2, w3] = core::array::from_fn(|j| {
                _mm512_mask_blend_epi64(borrow, difference[j], original[j])
            });
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
            let mut out = core::mem::MaybeUninit::<[Fp<crate::FqConfig>; 8]>::uninit();
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
mod word_export_tests {
    use super::*;
    use crate::{Fq, FqConfig};
    use ark_ff::{Field, PrimeField};
    type V = Ifma8<FqConfig, true>;
    fn scalar_sub(a: [u64; 4], b: [u64; 4]) -> ([u64; 4], bool) {
        let mut borrow = false;
        let out = core::array::from_fn(|j| {
            let (d, b0) = a[j].overflowing_sub(b[j]);
            let (d, b1) = d.overflowing_sub(borrow as u64);
            borrow = b0 || b1;
            d
        });
        (out, borrow)
    }
    fn plus(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
        let mut carry = 0u128;
        let out = core::array::from_fn(|j| {
            let s = a[j] as u128 + b[j] as u128 + carry;
            carry = s >> 64;
            s as u64
        });
        assert_eq!(carry, 0);
        out
    }
    fn numeric(w: [u64; 4]) -> ark_ref::Fq {
        let b: Vec<_> = w.into_iter().flat_map(u64::to_le_bytes).collect();
        ark_ref::Fq::from_le_bytes_mod_order(&b)
    }
    fn check(v: V) -> usize {
        let limbs: [[u64; 8]; 5] = v.0.map(|x| unsafe { core::mem::transmute(x) });
        let mut raw = [[0; 4]; 8];
        for lane in 0..8 {
            for j in 0..5 {
                assert!(limbs[j][lane] < 1 << 52);
                for b in 0..52 {
                    let bit = j * 52 + b;
                    let set = (limbs[j][lane] >> b) & 1;
                    if bit < 256 {
                        raw[lane][bit / 64] |= set << (bit % 64);
                    } else {
                        assert_eq!(set, 0);
                    }
                }
            }
            assert!(scalar_sub(raw[lane], plus(FqConfig::P, FqConfig::P)).1);
        }
        let original = v.get();
        let got = v.get_word_canonical();
        for lane in 0..8 {
            let (sub, borrow) = scalar_sub(raw[lane], FqConfig::P);
            let expected = if borrow { raw[lane] } else { sub };
            assert_eq!(got[lane].0, expected);
            assert_eq!(got[lane], original[lane]);
            assert!(scalar_sub(got[lane].0, FqConfig::P).1);
            assert_eq!(numeric(got[lane].0), numeric(raw[lane]));
        }
        8
    }
    #[test]
    fn boundary_random_and_lane_borrow_oracles() {
        let p = FqConfig::P;
        let twice = plus(p, p);
        let mut cases = vec![
            [0; 4],
            [1, 0, 0, 0],
            scalar_sub(p, [1, 0, 0, 0]).0,
            p,
            plus(p, [1, 0, 0, 0]),
            scalar_sub(twice, [1, 0, 0, 0]).0,
        ];
        for bit in 0..255 {
            let mut v = [0; 4];
            v[bit / 64] = 1 << (bit % 64);
            for d in [scalar_sub(v, [1, 0, 0, 0]).0, v, plus(v, [1, 0, 0, 0])] {
                if scalar_sub(d, twice).1 {
                    cases.push(d);
                }
            }
        }
        let mut patterns = [false; 8];
        // Force each of the three low-word outgoing borrows independently.
        for pattern in 0..8 {
            let mut v = p;
            for j in 0..3 {
                v[j] = if pattern & (1 << j) != 0 {
                    p[j] - 1
                } else {
                    p[j] + 1
                };
            }
            cases.push(v);
        }
        let mut state = 0x8c37132a6157u64;
        for _ in 0..16384 {
            let mut v = core::array::from_fn(|_| {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                state
            });
            v[3] &= (1 << 62) - 1;
            cases.push(v); // <2^254 <2p.
        }
        let mut checked = 0;
        for chunk in cases.chunks(8) {
            let values = core::array::from_fn(|i| chunk[i % chunk.len()]);
            checked += check(V::raw(&values));
            for v in values {
                let mut borrow = false;
                let mut pattern = 0;
                for j in 0..3 {
                    let (_, b0) = v[j].overflowing_sub(p[j]);
                    let (d, _) = v[j].overflowing_sub(p[j]);
                    let (_, b1) = d.overflowing_sub(borrow as u64);
                    borrow = b0 || b1;
                    pattern |= (borrow as usize) << j;
                }
                patterns[pattern] = true;
            }
        }
        assert!(patterns.into_iter().all(|v| v));
        for mask in 0..256 {
            checked += check(V::raw(&core::array::from_fn(|i| {
                if mask & (1 << i) == 0 {
                    [0; 4]
                } else {
                    scalar_sub(twice, [1, 0, 0, 0]).0
                }
            })));
        }
        eprintln!("WORD_EXPORT_DIRECT_LANES={checked}");
    }
    #[test]
    fn changing_lazy_arithmetic_matches_arkworks() {
        let ri = ark_ref::Fq::from(2u64).pow([256]).inverse().unwrap();
        let mut checked = 0;
        for batch in 0..512 {
            let a = core::array::from_fn(|lane| Fq::from_u64(7919 * (batch * 8 + lane + 1) as u64));
            let b = core::array::from_fn(|lane| Fq::from_u64(1237 * (batch * 8 + lane + 7) as u64));
            let av = V::new(&a);
            let bv = V::new(&b);
            let mut v = av;
            let mut expected = a.map(|x| numeric(x.0) * ri);
            for round in 0..32 {
                match round % 4 {
                    0 => {
                        v = v.mul(&bv);
                        for i in 0..8 {
                            expected[i] *= numeric(b[i].0) * ri;
                        }
                    }
                    1 => {
                        v = v.add(&av);
                        for i in 0..8 {
                            expected[i] += numeric(a[i].0) * ri;
                        }
                    }
                    2 => {
                        v = v.sub(&bv);
                        for i in 0..8 {
                            expected[i] -= numeric(b[i].0) * ri;
                        }
                    }
                    _ => {
                        v = v.mul(&v);
                        for x in &mut expected {
                            *x = x.square();
                        }
                    }
                }
                checked += check(v);
                for (got, want) in v.get_word_canonical().into_iter().zip(expected) {
                    assert_eq!(numeric(got.0) * ri, want);
                }
            }
        }
        eprintln!("WORD_EXPORT_CHAIN_LANES={checked}");
    }
}
