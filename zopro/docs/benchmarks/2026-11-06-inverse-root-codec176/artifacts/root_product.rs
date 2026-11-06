impl<C: FpConfig, const MIXED: bool> Ifma8<C, MIXED> {
    /// Product of lanes 0 and 4, returned as a canonical host field.
    /// The mixed representation is normalized and strictly below 2p.
    #[inline(always)]
    pub fn root_product(&self) -> Fp<C> {
        if !MIXED || C::P[3] >= 1u64 << 62 {
            let values = self.get();
            return values[0] * values[4];
        }
        // 4p<R=2^256. Raw a,b<2p give REDC(ab)<p+4p²/R<2p;
        // the existing backend's final subtraction is therefore sufficient.
        // Keep the raw limbs out of Fp until that canonicalization completes.
        let limbs: [[u64; 8]; 5] = self.0.map(|v| {
            // SAFETY: an initialized 512-bit vector is eight initialized u64s.
            unsafe { core::mem::transmute(v) }
        });
        let pack = |i: usize| {
            [
                limbs[0][i] | (limbs[1][i] << 52),
                (limbs[1][i] >> 12) | (limbs[2][i] << 40),
                (limbs[2][i] >> 24) | (limbs[3][i] << 28),
                (limbs[3][i] >> 36) | (limbs[4][i] << 16),
            ]
        };
        Fp::from_montgomery(crate::backend::multiply(pack(0), pack(4), C::P, C::INV))
    }
}

#[cfg(test)]
mod root_product_tests {
    use super::*;
    use ark_ff::{Field, PrimeField};

    // A normalized raw SIMD value is allowed to be lazy; no input Fp is built.
    fn raw<C: FpConfig, const MIXED: bool>(values: [[u64; 4]; 8]) -> Ifma8<C, MIXED> {
        let digits = values.map(split);
        Ifma8(
            core::array::from_fn(|j| {
                let column = core::array::from_fn::<u64, 8, _>(|i| digits[i][j]);
                // SAFETY: [u64;8] and __m512i have the same initialized bit width.
                unsafe { core::mem::transmute(column) }
            }),
            PhantomData,
        )
    }
    fn check<C: FpConfig, A: PrimeField, const MIXED: bool>() -> usize {
        let inv_input = A::from(2u64)
            .pow([if MIXED { 256 } else { 260 }])
            .inverse()
            .unwrap();
        let inv_host = A::from(2u64).pow([256]).inverse().unwrap();
        let convert = |words: [u64; 4]| {
            A::from_le_bytes_mod_order(
                &words
                    .into_iter()
                    .flat_map(u64::to_le_bytes)
                    .collect::<Vec<_>>(),
            )
        };
        let twice = crate::portable::add4(C::P, C::P).0;
        let edges = [
            [0; 4],
            [1, 0, 0, 0],
            crate::portable::sub4(C::P, [1, 0, 0, 0]).0,
            C::P,
            crate::portable::add4(C::P, [1, 0, 0, 0]).0,
            crate::portable::sub4(twice, [2, 0, 0, 0]).0,
            crate::portable::sub4(twice, [1, 0, 0, 0]).0,
        ];
        let mut state = 0x726f6f745f636f64u64;
        let mut checked = 0;
        for round in 0..1073usize {
            let mut random = || {
                let mut bytes = [0u8; 64];
                for b in bytes.chunks_mut(8) {
                    state ^= state << 13;
                    state ^= state >> 7;
                    state ^= state << 17;
                    b.copy_from_slice(&state.to_le_bytes());
                }
                Fp::<C>::from_random_bytes(&bytes).0
            };
            let mut left = core::array::from_fn(|i| {
                let v = random();
                if (i + round) % 3 == 0 {
                    v
                } else {
                    crate::portable::add4(v, C::P).0
                }
            });
            let right = core::array::from_fn(|i| {
                let v = random();
                if (i + round) % 2 == 0 {
                    v
                } else {
                    crate::portable::add4(v, C::P).0
                }
            });
            if round < 49 {
                left[0] = edges[round / 7];
                left[4] = edges[round % 7];
            }
            let want_left = left.map(|v| convert(v) * inv_input);
            let want_right = right.map(|v| convert(v) * inv_input);
            let l = raw::<C, MIXED>(left);
            let r = raw::<C, MIXED>(right);
            for (value, want) in [
                (l, want_left),
                (
                    l.add(&r),
                    core::array::from_fn(|i| want_left[i] + want_right[i]),
                ),
                (
                    l.sub(&r),
                    core::array::from_fn(|i| want_left[i] - want_right[i]),
                ),
                (
                    l.mul(&r),
                    core::array::from_fn(|i| want_left[i] * want_right[i]),
                ),
            ] {
                let result = value.root_product();
                assert!(result.is_reduced());
                assert_eq!(convert(result.0) * inv_host, want[0] * want[4]);
                let original = value.get();
                assert_eq!(result, original[0] * original[4]);
                checked += 1;
            }
        }
        assert_eq!(checked, 4292);
        checked
    }
    #[test]
    fn mixed_lazy_root_products_match_arkworks() {
        let fq = check::<crate::FqConfig, ark_ref::Fq, true>();
        let fr = check::<crate::FrConfig, ark_ref::Fr, true>();
        println!("ROOT_PRODUCT_MIXED_PASS fq={fq} fr={fr}; normalizedraw/lazy/chains/zero/boundaries;noinvalidFpinputs");
    }
    #[test]
    fn uniform_root_products_preserve_conversion() {
        let fq = check::<crate::FqConfig, ark_ref::Fq, false>();
        let fr = check::<crate::FrConfig, ark_ref::Fr, false>();
        println!("ROOT_PRODUCT_UNIFORM_PASS fq={fq} fr={fr};originalR260conversionfallback");
    }
}
