#[cfg(test)]
mod signed_final_tests {
    use super::*;
    fn words<C: FpConfig>(seed: &mut u64, lazy: bool) -> [u64; 4] {
        let mut bytes = [0; 64];
        for chunk in bytes.chunks_mut(8) {
            *seed ^= *seed << 13;
            *seed ^= *seed >> 7;
            *seed ^= *seed << 17;
            chunk.copy_from_slice(&seed.to_le_bytes());
        }
        let v = Fp::<C>::from_random_bytes(&bytes).0;
        if lazy {
            crate::portable::add4(v, C::P).0
        } else {
            v
        }
    }
    fn raw<C: FpConfig, const MIXED: bool>(v: Ifma8<C, MIXED>) -> [[u64; 8]; 5] {
        v.0.map(|x| unsafe { core::mem::transmute(x) })
    }
    fn normalized_below_twice<C: FpConfig, const MIXED: bool>(v: Ifma8<C, MIXED>) {
        let digits = raw(v);
        let limit = split(crate::portable::add4(C::P, C::P).0);
        for i in 0..8 {
            assert!((0..5).all(|j| digits[j][i] < (1 << 52)));
            assert!((0..5)
                .rev()
                .map(|j| digits[j][i])
                .cmp((0..5).rev().map(|j| limit[j]))
                .is_lt());
        }
    }
    #[test]
    fn signed_coefficients_match_separate_normalization_exactly() {
        fn check<C: FpConfig, const MIXED: bool>() {
            let mut seed = 0x128f961aa734bc21;
            let twice = crate::portable::add4(C::P, C::P).0;
            let edges = [[0; 4], C::P, crate::portable::sub4(twice, [1, 0, 0, 0]).0];
            let subtractors = [
                [0; 4],
                C::P,
                twice,
                crate::portable::sub4(C::P, [1, 0, 0, 0]).0,
            ];
            let mut cases = Vec::new();
            for r in edges {
                for c in subtractors {
                    cases.push((r, c));
                }
            }
            for i in 0..4096 {
                cases.push((
                    words::<C>(&mut seed, i % 2 == 0),
                    words::<C>(&mut seed, i % 3 == 0),
                ));
            }
            for batch in cases.chunks(8) {
                let r: [[_; 4]; 8] = core::array::from_fn(|i| batch[i % batch.len()].0);
                let c: [[_; 4]; 8] = core::array::from_fn(|i| batch[i % batch.len()].1);
                let rv = Ifma8::<C, MIXED>::raw(&r);
                let cv = Ifma8::<C, MIXED>::raw(&c);
                let ordinary = raw(rv);
                let mut coefficients = [[0u64; 8]; 6];
                coefficients[..5].copy_from_slice(&ordinary);
                // Move positive multiples of B from a higher digit to the
                // preceding coefficient. This preserves the exact integer.
                for j in 0..4 {
                    for lane in 0..8 {
                        let k = coefficients[j + 1][lane].min(127);
                        coefficients[j][lane] += k * (1u64 << 52);
                        coefficients[j + 1][lane] -= k;
                    }
                }
                let t = coefficients.map(|v| unsafe { core::mem::transmute(v) });
                let got = Ifma8::<C, MIXED>::finish_sub(t, &cv);
                assert_eq!(raw(got), raw(rv.sub(&cv)));
                normalized_below_twice(got);
            }
        }
        check::<crate::FqConfig, false>();
        check::<crate::FqConfig, true>();
        check::<crate::FrConfig, false>();
        check::<crate::FrConfig, true>();
    }
    #[test]
    fn complete_mul_sub_lazy_edges_chains_match_arkworks() {
        fn check<C: FpConfig, A: ark_ff::PrimeField, const MIXED: bool>() {
            use ark_ff::Field;
            let numeric = |v: [u64; 4]| {
                let mut b = [0; 32];
                for (chunk, w) in b.chunks_mut(8).zip(v) {
                    chunk.copy_from_slice(&w.to_le_bytes());
                }
                A::from_le_bytes_mod_order(&b)
            };
            let inverse = A::from(2u64)
                .pow([if MIXED { 256 } else { 260 }])
                .inverse()
                .unwrap();
            let host_inverse = A::from(2u64).pow([256]).inverse().unwrap();
            let twice = crate::portable::add4(C::P, C::P).0;
            let mut seed = 0x4d554c5355423236;
            let mut edges = vec![
                [0; 4],
                [1, 0, 0, 0],
                C::P,
                crate::portable::sub4(C::P, [1, 0, 0, 0]).0,
                crate::portable::sub4(twice, [1, 0, 0, 0]).0,
            ];
            for bit in [48, 52, 104, 156, 208, 253] {
                let mut v = [0; 4];
                v[bit / 64] = 1u64 << (bit % 64);
                edges.extend([
                    crate::portable::sub4(v, [1, 0, 0, 0]).0,
                    v,
                    crate::portable::add4(v, [1, 0, 0, 0]).0,
                ]);
            }
            let mut inputs = Vec::new();
            for &a in &edges {
                for &b in &edges {
                    for c in [twice, edges[inputs.len() % edges.len()]] {
                        inputs.push([a, b, c]);
                    }
                }
            }
            for i in 0..4096 {
                inputs.push(core::array::from_fn(|j| {
                    words::<C>(&mut seed, (i + j) % 2 == 0)
                }));
            }
            let mut checked = 0;
            for batch in inputs.chunks(8) {
                let tuples: [[[u64; 4]; 8]; 3] =
                    core::array::from_fn(|j| core::array::from_fn(|i| batch[i % batch.len()][j]));
                let v = tuples.map(|x| Ifma8::<C, MIXED>::raw(&x));
                let b = tuples[1].map(|x| numeric(x) * inverse);
                let c = tuples[2].map(|x| numeric(x) * inverse);
                let mut got = v[0];
                let mut want = tuples[0].map(|x| numeric(x) * inverse);
                for _ in 0..9 {
                    let expected_bits = got.mul(&v[1]).sub(&v[2]);
                    got = got.mul_sub_final(&v[1], &v[2]);
                    assert_eq!(raw(got), raw(expected_bits));
                    normalized_below_twice(got);
                    let host = got.get();
                    for i in 0..8 {
                        want[i] = want[i] * b[i] - c[i];
                        assert_eq!(numeric(host[i].0) * host_inverse, want[i]);
                    }
                    checked += 8;
                }
            }
            println!("SIGNED_FINAL_LANES={checked} mixed={MIXED}");
        }
        check::<crate::FqConfig, ark_ref::Fq, false>();
        check::<crate::FqConfig, ark_ref::Fq, true>();
        check::<crate::FrConfig, ark_ref::Fr, false>();
        check::<crate::FrConfig, ark_ref::Fr, true>();
    }
}
