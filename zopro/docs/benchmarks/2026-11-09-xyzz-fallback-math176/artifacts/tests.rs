mod xyzz;
use ark_ec::{AdditiveGroup, AffineRepr, CurveGroup, PrimeGroup};
use ark_ff::{PrimeField, Zero};
use zopro_bn254::{CurveConfig, Field, Fq, Fq2};
fn fq(v: ark_bn254::Fq) -> Fq {
    Fq::from_canonical(v.into_bigint().0).unwrap()
}
fn arkfq(v: Fq) -> ark_bn254::Fq {
    ark_bn254::Fq::from_bigint(ark_ff::BigInt(v.to_canonical())).unwrap()
}
fn g1(p: ark_bn254::G1Affine) -> zopro_bn254::G1Affine {
    if p.is_zero() {
        zopro_bn254::G1Affine::IDENTITY
    } else {
        zopro_bn254::G1Affine::new(fq(p.x), fq(p.y))
    }
}
fn g2(p: ark_bn254::G2Affine) -> zopro_bn254::G2Affine {
    if p.is_zero() {
        zopro_bn254::G2Affine::IDENTITY
    } else {
        zopro_bn254::G2Affine::new(
            Fq2::new(fq(p.x.c0), fq(p.x.c1)),
            Fq2::new(fq(p.y.c0), fq(p.y.c1)),
        )
    }
}
fn q1(x: Fq) -> ark_bn254::Fq {
    arkfq(x)
}
fn q2(x: Fq2) -> ark_bn254::Fq2 {
    ark_bn254::Fq2::new(arkfq(x.c0), arkfq(x.c1))
}
fn weights() -> Vec<ark_bn254::Fr> {
    let mut seed = 0x78797a7a73706172u64;
    let mut out = vec![
        ark_bn254::Fr::zero(),
        ark_bn254::Fr::from(1u64),
        -ark_bn254::Fr::from(1u64),
    ];
    for _ in 0..61 {
        let mut b = [0; 32];
        for word in b.chunks_mut(8) {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            word.copy_from_slice(&seed.to_le_bytes());
        }
        out.push(ark_bn254::Fr::from_le_bytes_mod_order(&b));
    }
    out
}
macro_rules! curve_tests {
    ($module:ident,$config:ty,$point:ty,$ark:ty,$convert:ident,$field:ty,$toark:ident,$scale:expr) => {
        mod $module {
            use super::*;
            type Point = xyzz::Xyzz<$config>;
            type Ark = $ark;
            fn check(p: &Point, w: Ark) {
                let w = w.into_affine();
                if w.is_zero() {
                    assert!(p.is_identity());
                    assert!(p.zzz.is_zero());
                    assert!($toark(p.zz).is_zero());
                } else {
                    assert!(!p.is_identity());
                    let zz = $toark(p.zz);
                    let zzz = $toark(p.zzz);
                    assert_eq!(zz * zz * zz, zzz * zzz);
                    assert_eq!($toark(p.x), w.x * zz);
                    assert_eq!($toark(p.y), w.y * zzz);
                }
                assert_eq!(p.to_affine(), $convert(w));
            }
            fn scaled(w: Ark, z: $field) -> Point {
                let a = $convert(w.into_affine());
                if a.is_identity() {
                    return Point::identity();
                }
                let zz = z.square();
                let zzz = zz * z;
                Point {
                    x: a.x * zz,
                    y: a.y * zzz,
                    zz,
                    zzz,
                }
            }
            #[test]
            fn complete_add_double_negate_and_variable_scales() {
                let g = Ark::generator();
                let weights = weights();
                let pool = Ark::normalize_batch(&weights.iter().map(|s| g * s).collect::<Vec<_>>());
                let mut checks = 0;
                for i in 0..64 {
                    for j in 0..64 {
                        let a = g * weights[i];
                        let b = pool[j];
                        let z: $field = ($scale)(i + j + 1);
                        let p = scaled(a, z);
                        check(&p, a);
                        check(&p.add_affine(&$convert(b)), a + b);
                        check(&p.double(), a.double());
                        check(&p.negate(), -a);
                        checks += 4;
                    }
                }
                // Noncanonical identity numerators are valid projective infinity state.
                let p = Point {
                    x: <$config as CurveConfig>::Base::ONE,
                    y: <$config as CurveConfig>::Base::ONE,
                    zz: <$config as CurveConfig>::Base::ZERO,
                    zzz: <$config as CurveConfig>::Base::ZERO,
                };
                check(&p, Ark::zero());
                check(&p.add_affine(&$convert(pool[3])), g * weights[3]);
                println!(
                    "XYZZ_{}_PAIR_RAW_ORACLE_CHECKS={}",
                    stringify!($module),
                    checks + 2
                );
            }
            #[test]
            fn changing_chains_batch_normalization_and_final_merge() {
                let g = Ark::generator();
                let weights = weights();
                let pool = Ark::normalize_batch(&weights.iter().map(|s| g * s).collect::<Vec<_>>());
                let mut checks = 0;
                let mut states = vec![];
                let mut expected = vec![];
                for pattern in 0..6 {
                    let (mut p, mut want) = (Point::identity(), Ark::zero());
                    for i in 0..257 {
                        let b = match pattern {
                            0 => pool[0],
                            1 => pool[3],
                            2 => {
                                if i % 2 == 0 {
                                    pool[3]
                                } else {
                                    -pool[3]
                                }
                            }
                            3 => {
                                if i % 5 == 0 {
                                    want.into_affine()
                                } else {
                                    pool[(i * 17 + 3) % 64]
                                }
                            }
                            4 => {
                                if i % 7 == 0 {
                                    -want.into_affine()
                                } else {
                                    pool[(i * 13 + 5) % 64]
                                }
                            }
                            _ => pool[(i * 29 + 7) % 64],
                        };
                        p = p.add_affine(&$convert(b));
                        want += b;
                        check(&p, want);
                        checks += 1;
                        if i % 17 == 0 {
                            p = p.double();
                            want = want.double();
                            check(&p, want);
                            checks += 1;
                        }
                        if i % 11 == 0 {
                            p = p.negate();
                            want = -want;
                            check(&p, want);
                            checks += 1;
                        }
                        if i % 31 == 0 {
                            p = scaled(want, ($scale)(i + pattern + 3));
                            check(&p, want);
                            checks += 1;
                        }
                    }
                    states.push(p);
                    expected.push(want);
                }
                for n in [0usize, 1, 7, 8, 9, 31, 128, 257] {
                    let points = (0..n).map(|i| states[i % states.len()]).collect::<Vec<_>>();
                    let saved = points.clone();
                    let out = Point::batch_to_affine(&points);
                    assert_eq!(out.len(), n);
                    for i in 0..n {
                        assert_eq!(out[i], $convert(expected[i % expected.len()].into_affine()));
                        check(&points[i], expected[i % expected.len()]);
                        assert_eq!(points[i].x, saved[i].x);
                        assert_eq!(points[i].y, saved[i].y);
                        assert_eq!(points[i].zz, saved[i].zz);
                        assert_eq!(points[i].zzz, saved[i].zzz);
                        checks += 1;
                    }
                    let merged = points
                        .iter()
                        .enumerate()
                        .map(|(i, p)| p.add_affine(&$convert(pool[(i * 7 + 3) % 64])))
                        .collect::<Vec<_>>();
                    let out = Point::batch_to_affine(&merged);
                    for i in 0..n {
                        assert_eq!(
                            out[i],
                            $convert(
                                (expected[i % expected.len()] + pool[(i * 7 + 3) % 64])
                                    .into_affine()
                            )
                        );
                        checks += 1;
                    }
                }
                assert_eq!(
                    core::mem::size_of::<Point>(),
                    4 * core::mem::size_of::<<$config as CurveConfig>::Base>()
                );
                println!(
                    "XYZZ_{}_CHAIN_BATCH_MERGE_CHECKS={};point_bytes={}",
                    stringify!($module),
                    checks,
                    core::mem::size_of::<Point>()
                );
            }
        }
    };
}
curve_tests!(
    g1_cases,
    zopro_bn254::G1Config,
    zopro_bn254::G1Affine,
    ark_bn254::G1Projective,
    g1,
    Fq,
    q1,
    |i: usize| Fq::from((i as u64 + 3) * 179)
);
curve_tests!(
    g2_cases,
    zopro_bn254::G2Config,
    zopro_bn254::G2Affine,
    ark_bn254::G2Projective,
    g2,
    Fq2,
    q2,
    |i: usize| Fq2::new(
        Fq::from((i as u64 + 3) * 179),
        Fq::from((i as u64 + 5) * 313)
    )
);
