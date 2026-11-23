//! Independent field oracle for the default module, including raw range bounds.
use super::*;
use ark_ff::{Field, PrimeField};
fn bytes(w: [u64; 4]) -> [u8; 32] {
    let mut out = [0; 32];
    for (chunk, word) in out.chunks_mut(8).zip(w) {
        chunk.copy_from_slice(&word.to_le_bytes());
    }
    out
}
fn next(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}
fn check<C: FpConfig, A: PrimeField, const MIXED: bool>() -> usize {
    let p = C::P;
    let twice = crate::portable::add4(p, p).0;
    let edges = [
        [0; 4],
        [1, 0, 0, 0],
        crate::portable::sub4(p, [1, 0, 0, 0]).0,
    ];
    let lazy_edges = [
        edges[0],
        edges[1],
        edges[2],
        p,
        crate::portable::add4(p, [1, 0, 0, 0]).0,
        crate::portable::sub4(twice, [1, 0, 0, 0]).0,
    ];
    let scale = A::from(2)
        .pow([if MIXED { 256 } else { 260 }])
        .inverse()
        .unwrap();
    let mut seed = 0x7873_756d_2d72_616e;
    let mut outputs = 0;
    for round in 0..2054 {
        let mut random = |bound: [u64; 4]| {
            let mut w = core::array::from_fn(|_| next(&mut seed));
            w[3] &= (1u64 << 62) - 1;
            while !crate::portable::less(w, bound) {
                w = crate::portable::sub4(w, bound).0;
            }
            w
        };
        let a: [[u64; 4]; 8] = core::array::from_fn(|i| {
            if round < 6 {
                lazy_edges[(round + i) % 6]
            } else {
                random(twice)
            }
        });
        let x: [[u64; 4]; 8] = core::array::from_fn(|i| {
            if round < 6 {
                edges[(round + i) % 3]
            } else {
                random(p)
            }
        });
        let y: [[u64; 4]; 8] = core::array::from_fn(|i| {
            if round < 6 {
                edges[(round + 2 * i) % 3]
            } else {
                random(p)
            }
        });
        let a8 = Ifma8::<C, MIXED>::linear_diagnostic_raw(&a, false);
        let x8 = Ifma8::<C, MIXED>::linear_diagnostic_raw(&x, true);
        let y8 = Ifma8::<C, MIXED>::linear_diagnostic_raw(&y, true);
        let sum = UncarriedSum8::from_canonical(&x8, &y8);
        let got = sum.subtract_from(&a8);
        let raw = got.linear_diagnostic_digits();
        let fields = got.get();
        for i in 0..8 {
            for limb in raw {
                assert!(limb[i] < 1u64 << 52);
            }
            assert!(raw[4][i] < 1u64 << 48);
            let words = [
                raw[0][i] | raw[1][i] << 52,
                raw[1][i] >> 12 | raw[2][i] << 40,
                raw[2][i] >> 24 | raw[3][i] << 28,
                raw[3][i] >> 36 | raw[4][i] << 16,
            ];
            assert!(crate::portable::less(words, twice));
            let expected = (A::from_le_bytes_mod_order(&bytes(a[i]))
                - A::from_le_bytes_mod_order(&bytes(x[i]))
                - A::from_le_bytes_mod_order(&bytes(y[i])))
                * scale;
            assert_eq!(
                A::from_le_bytes_mod_order(&bytes(fields[i].to_canonical())),
                expected
            );
            outputs += 1;
        }
    }
    outputs
}
#[test]
fn canonical_sum_lazy_subtraction_both_fields_and_radices() {
    let n = check::<crate::FqConfig, ark_ref::Fq, true>()
        + check::<crate::FqConfig, ark_ref::Fq, false>()
        + check::<crate::FrConfig, ark_ref::Fr, true>()
        + check::<crate::FrConfig, ark_ref::Fr, false>();
    assert_eq!(n, 65728);
    println!("DEFAULT_CARRY_ORACLE Arkworks_outputs={n} fields=2 radices=2 ranges=canonical-X,lazy-minuend,normalized-result");
}
