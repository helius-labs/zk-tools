//! Fresh native oracle population for uniform prefix primitives.
use crate::{Fp, FpConfig, FqConfig, FrConfig};
use ark_ff::{Field, PrimeField};
use core::{
    arch::x86_64::{__m256i, __m512i},
    marker::PhantomData,
};
fn bytes(x: [u64; 4]) -> [u8; 32] {
    let mut b = [0; 32];
    for i in 0..4 {
        b[i * 8..i * 8 + 8].copy_from_slice(&x[i].to_le_bytes());
    }
    b
}
fn split(x: [u64; 4]) -> [u64; 5] {
    let m = (1u64 << 52) - 1;
    [
        x[0] & m,
        ((x[0] >> 52) | (x[1] << 12)) & m,
        ((x[1] >> 40) | (x[2] << 24)) & m,
        ((x[2] >> 28) | (x[3] << 36)) & m,
        x[3] >> 16,
    ]
}
fn random(seed: &mut u64, twice: [u64; 4]) -> [u64; 4] {
    let mut x = core::array::from_fn(|_| {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 7;
        *seed ^= *seed << 17;
        *seed
    });
    x[3] &= (1u64 << 63) - 1;
    if !crate::portable::less(x, twice) {
        x = crate::portable::sub4(x, twice).0;
    }
    x
}
fn canonical<C: FpConfig>(mut x: [u64; 4]) -> Fp<C> {
    for _ in 0..2 {
        if !crate::portable::less(x, C::P) {
            x = crate::portable::sub4(x, C::P).0;
        }
    }
    Fp::from_montgomery(x)
}
macro_rules! checks {
    ($name:ident,$width:literal,$module:ident,$field:ident,$kind:ident) => {
        fn $name<C: FpConfig, A: PrimeField>() {
            let p = C::P;
            let twice = crate::portable::add4(p, p).0;
            let edges = [
                [0; 4],
                [1, 0, 0, 0],
                crate::portable::sub4(p, [1, 0, 0, 0]).0,
                p,
                crate::portable::add4(p, [1, 0, 0, 0]).0,
                crate::portable::sub4(twice, [1, 0, 0, 0]).0,
                twice,
            ];
            let hi = A::from(2).pow([256u64]).inverse().unwrap();
            let qi = A::from(16).inverse().unwrap();
            let mut seed = 0x723f_d04a_91b8_ce65;
            let mut chain = [[0; 4]; $width];
            let mut differences = 0;
            let mut masks = [false; 256];
            fn pack<C: FpConfig>(raw: [[u64; 4]; $width]) -> crate::$module::$field<C, true> {
                let digits = raw.map(split);
                let vectors = core::array::from_fn(|j| {
                    let words: [u64; $width] = core::array::from_fn(|i| digits[i][j]);
                    // SAFETY: test-only equal-size reinterpretation of initialized lane words.
                    unsafe { core::mem::transmute::<[u64; $width], $kind>(words) }
                });
                crate::$module::$field(vectors, PhantomData)
            }
            for round in 0..5211usize {
                let raw: [[[u64; 4]; $width]; 4] = core::array::from_fn(|operand| {
                    core::array::from_fn(|lane| {
                        if round < 49 {
                            edges[if operand % 2 == 0 {
                                (round + lane) % 7
                            } else {
                                ((round + lane) / 7) % 7
                            }]
                        } else if round < 91 {
                            let bits = [
                                1usize, 51, 52, 53, 63, 64, 103, 104, 155, 156, 207, 208, 253, 254,
                            ][(round - 49) / 3];
                            let mut x = [0; 4];
                            x[bits / 64] = 1u64 << (bits % 64);
                            match (round - 49) % 3 {
                                1 => x = crate::portable::add4(x, [1, 0, 0, 0]).0,
                                2 => x = crate::portable::sub4(x, [1, 0, 0, 0]).0,
                                _ => {}
                            }
                            if operand % 2 == 0 {
                                x
                            } else {
                                crate::portable::sub4(twice, x).0
                            }
                        } else if round >= 4187 && operand == 0 {
                            chain[lane]
                        } else {
                            random(&mut seed, twice)
                        }
                    })
                });
                let [a, b, c, d] = raw.map(pack::<C>);
                let by = core::array::from_fn(|i| canonical::<C>(raw[0][i]));
                let ay = core::array::from_fn(|i| canonical::<C>(raw[1][i]));
                let negative = (round & 255) as u8;
                masks[negative as usize] = true;
                let signed = crate::$module::$field::<C, true>::new(&by)
                    .signed_difference_scaled_div16(&crate::$module::$field::new(&ay), &c, negative);
                for (kind, value) in [
                    a.mul_div16(&b),
                    a.sum2_div16(&b, &c, &d),
                    a.difference2_div16(&b, &c, &d),
                    signed,
                ]
                .into_iter()
                .enumerate()
                {
                    let words: [[u64; $width]; 5] = value.0.map(|v| {
                        // SAFETY: each initialized SIMD value contains exactly width u64 lanes.
                        unsafe { core::mem::transmute::<$kind, [u64; $width]>(v) }
                    });
                    let exported = value.get();
                    for lane in 0..$width {
                        let ds: [u64; 5] = core::array::from_fn(|j| words[j][lane]);
                        assert!(ds.iter().all(|x| *x < (1u64 << 52)));
                        assert!(ds[4] < (1u64 << 48));
                        let out = [
                            ds[0] | ds[1] << 52,
                            ds[1] >> 12 | ds[2] << 40,
                            ds[2] >> 24 | ds[3] << 28,
                            ds[3] >> 36 | ds[4] << 16,
                        ];
                        assert!(crate::portable::less(out, twice));
                        let x: [A; 4] = core::array::from_fn(|j| {
                            A::from_le_bytes_mod_order(&bytes(raw[j][lane])) * hi
                        });
                        let ordinary = match kind {
                            0 => x[0] * x[1],
                            1 => x[0] * x[1] + x[2] * x[3],
                            2 => x[0] * x[1] - x[2] * x[3],
                            _ => {
                                let by =
                                    A::from_le_bytes_mod_order(&bytes(by[lane].to_canonical()));
                                let ay =
                                    A::from_le_bytes_mod_order(&bytes(ay[lane].to_canonical()));
                                (if negative >> lane & 1 == 0 { by } else { -by } - ay) * x[2]
                            }
                        };
                        let want = ordinary * qi;
                        let got = A::from_le_bytes_mod_order(&bytes(exported[lane].to_canonical()));
                        assert_eq!(got, want, "round {round} kind {kind} lane {lane}");
                        differences += usize::from(want != ordinary);
                        if kind == 0 {
                            chain[lane] = out;
                        }
                    }
                }
            }
            assert!(masks.into_iter().all(|x| x));
            assert!(differences > 1000);
        }
    };
}
checks!(check4, 4, ifma, Ifma, __m256i);
checks!(check8, 8, ifma8, Ifma8, __m512i);
#[test]
fn uniform_prefix_raw_four_lanes() {
    check4::<FqConfig, ark_ref::Fq>();
    check4::<FrConfig, ark_ref::Fr>();
}
#[test]
fn uniform_prefix_raw_eight_lanes() {
    check8::<FqConfig, ark_ref::Fq>();
    check8::<FrConfig, ark_ref::Fr>();
}
