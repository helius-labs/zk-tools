//! Every operation against arkworks' (the original ark-bn254): both keep
//! field elements as four limbs in Montgomery form with R = 2^256, so an
//! element converts by its limbs.
use ark_ec::{pairing::Pairing, AdditiveGroup, AffineRepr, CurveGroup, PrimeGroup};
use ark_ff::{BigInt, Field as _, PrimeField, UniformRand, Zero};
use ark_ref as ark;

use crate::{pairing, Fq, Fq12, Fq2, Fq6, Fr, G1Affine, G1Projective, G2Affine, G2Projective};

fn fq(x: ark::Fq) -> Fq {
    Fq::from_montgomery(x.0 .0)
}
fn ark_fq(x: Fq) -> ark::Fq {
    ark::Fq::new_unchecked(BigInt(x.0))
}
fn fr(x: ark::Fr) -> Fr {
    Fr::from_montgomery(x.0 .0)
}
fn fq2(x: ark::Fq2) -> Fq2 {
    Fq2::new(fq(x.c0), fq(x.c1))
}
fn fq6(x: ark::Fq6) -> Fq6 {
    Fq6::new(fq2(x.c0), fq2(x.c1), fq2(x.c2))
}
fn fq12(x: ark::Fq12) -> Fq12 {
    Fq12::new(fq6(x.c0), fq6(x.c1))
}
fn g1(p: ark::G1Affine) -> G1Affine {
    if p.infinity {
        G1Affine::IDENTITY
    } else {
        G1Affine::new(fq(p.x), fq(p.y))
    }
}
fn g2(p: ark::G2Affine) -> G2Affine {
    if p.infinity {
        G2Affine::IDENTITY
    } else {
        G2Affine::new(fq2(p.x), fq2(p.y))
    }
}

fn edges() -> Vec<ark::Fq> {
    vec![
        ark::Fq::zero(),
        ark::Fq::ONE,
        -ark::Fq::ONE,
        -ark::Fq::ONE - ark::Fq::ONE,
        ark::Fq::from(2u64),
    ]
}

#[test]
fn constants() {
    assert_eq!(Fq::MODULUS, <ark::Fq as PrimeField>::MODULUS.0);
    assert_eq!(Fr::MODULUS, <ark::Fr as PrimeField>::MODULUS.0);
    assert_eq!(Fq::ONE, fq(ark::Fq::ONE));
    assert_eq!(Fr::ONE, fr(ark::Fr::ONE));
    assert_eq!(
        Fr::TWO_ADIC_ROOT_OF_UNITY,
        fr(<ark::Fr as ark_ff::FftField>::TWO_ADIC_ROOT_OF_UNITY)
    );
    assert_eq!(Fr::TWO_ADICITY, <ark::Fr as ark_ff::FftField>::TWO_ADICITY);
    assert_eq!(G1Affine::GENERATOR, g1(ark::G1Affine::generator()));
    assert_eq!(G2Affine::GENERATOR, g2(ark::G2Affine::generator()));
}

#[test]
fn base_field() {
    let mut rng = ark_std::test_rng();
    let mut values = edges();
    values.extend((0..200).map(|_| ark::Fq::rand(&mut rng)));
    for a in &values {
        for b in values.iter().take(60) {
            assert_eq!(fq(*a) + fq(*b), fq(*a + b));
            assert_eq!(fq(*a) - fq(*b), fq(*a - b));
            assert_eq!(fq(*a) * fq(*b), fq(*a * b));
            assert_eq!(
                Fq::sum_of_products(&fq(*a), &fq(*b), &fq(*b), &fq(*a)),
                fq(*a * b + *b * a)
            );
        }
        assert_eq!(-fq(*a), fq(-*a));
        assert_eq!(fq(*a).square(), fq(a.square()));
        assert_eq!(fq(*a).double(), fq(a.double()));
        assert_eq!(fq(*a).inverse(), a.inverse().map(fq));
        assert_eq!(fq(*a).sqrt(), a.sqrt().map(|r| {
            // Either root: arkworks may return the other.
            let ours = fq(*a).sqrt().unwrap();
            if ours == fq(r) { fq(r) } else { fq(-r) }
        }));
        assert_eq!(fq(*a).to_canonical(), a.into_bigint().0);
        assert_eq!(Fq::from_canonical(a.into_bigint().0), Some(fq(*a)));
        assert_eq!(fq(*a).num_bits(), a.into_bigint().num_bits_u32());
    }
    assert_eq!(Fq::from_canonical(Fq::MODULUS), None);
    assert_eq!(Fq::from_u64(12345), fq(ark::Fq::from(12345u64)));
    let bytes: Vec<u8> = (0..48).map(|i| (i * 37 + 11) as u8).collect();
    assert_eq!(
        Fq::from_be_bytes_mod_order(&bytes),
        fq(ark::Fq::from_be_bytes_mod_order(&bytes))
    );
    let e = [3u64, 0x1234_5678, 0, 1];
    for a in values.iter().take(20) {
        assert_eq!(fq(*a).pow(&e), fq(a.pow(e)));
    }
}

trait NumBits {
    fn num_bits_u32(&self) -> u32;
}
impl NumBits for BigInt<4> {
    fn num_bits_u32(&self) -> u32 {
        ark_ff::BigInteger::num_bits(self)
    }
}

#[test]
fn scalar_field() {
    let mut rng = ark_std::test_rng();
    for _ in 0..500 {
        let (a, b) = (ark::Fr::rand(&mut rng), ark::Fr::rand(&mut rng));
        assert_eq!(fr(a) * fr(b), fr(a * b));
        assert_eq!(fr(a) + fr(b), fr(a + b));
        assert_eq!(fr(a) - fr(b), fr(a - b));
        assert_eq!(fr(a).inverse(), a.inverse().map(fr));
    }
}

#[test]
fn quadratic_extension() {
    let mut rng = ark_std::test_rng();
    let e = edges();
    let mut values: Vec<ark::Fq2> = (0..150).map(|_| ark::Fq2::rand(&mut rng)).collect();
    for x in &e {
        for y in &e {
            values.push(ark::Fq2::new(*x, *y));
        }
    }
    for a in &values {
        for b in values.iter().take(40) {
            assert_eq!(fq2(*a) * fq2(*b), fq2(*a * b));
            assert_eq!(fq2(*a) + fq2(*b), fq2(*a + b));
            assert_eq!(fq2(*a) - fq2(*b), fq2(*a - b));
        }
        assert_eq!(fq2(*a).square(), fq2(a.square()));
        assert_eq!(fq2(*a).inverse(), a.inverse().map(fq2));
        assert_eq!(fq2(*a).frobenius(1), fq2(a.frobenius_map(1)));
        let ours = fq2(*a).sqrt();
        match a.sqrt() {
            Some(r) => {
                let ours = ours.expect("a root");
                assert!(ours == fq2(r) || ours == -fq2(r));
            }
            None => assert_eq!(ours, None),
        }
        assert_eq!(
            fq2(*a).mul_by_nonresidue(),
            fq2(*a * ark::Fq2::new(ark::Fq::from(9u64), ark::Fq::ONE))
        );
    }
}

#[test]
fn tower() {
    let mut rng = ark_std::test_rng();
    for _ in 0..100 {
        let (a, b) = (ark::Fq6::rand(&mut rng), ark::Fq6::rand(&mut rng));
        assert_eq!(fq6(a) * fq6(b), fq6(a * b));
        assert_eq!(fq6(a).square(), fq6(a.square()));
        assert_eq!(fq6(a).inverse(), a.inverse().map(fq6));
        for power in 0..6 {
            assert_eq!(fq6(a).frobenius(power), fq6(a.frobenius_map(power)));
        }
        let (c0, c1) = (ark::Fq2::rand(&mut rng), ark::Fq2::rand(&mut rng));
        let mut expected = a;
        expected.mul_by_01(&c0, &c1);
        assert_eq!(fq6(a).mul_by_01(&fq2(c0), &fq2(c1)), fq6(expected));
        let mut expected = a;
        expected.mul_by_1(&c1);
        assert_eq!(fq6(a).mul_by_1(&fq2(c1)), fq6(expected));

        let (x, y) = (ark::Fq12::rand(&mut rng), ark::Fq12::rand(&mut rng));
        assert_eq!(fq12(x) * fq12(y), fq12(x * y));
        assert_eq!(fq12(x).square(), fq12(x.square()));
        assert_eq!(fq12(x).inverse(), x.inverse().map(fq12));
        for power in 0..12 {
            assert_eq!(fq12(x).frobenius(power), fq12(x.frobenius_map(power)));
        }
        let c4 = ark::Fq2::rand(&mut rng);
        let mut expected = x;
        expected.mul_by_034(&c0, &c1, &c4);
        assert_eq!(
            fq12(x).mul_by_034(&fq2(c0), &fq2(c1), &fq2(c4)),
            fq12(expected)
        );
    }
}

#[test]
fn curves() {
    let mut rng = ark_std::test_rng();
    for _ in 0..50 {
        let (a, b) = (ark::G1Projective::rand(&mut rng), ark::G1Projective::rand(&mut rng));
        let (pa, pb) = (g1(a.into_affine()), g1(b.into_affine()));
        assert!(pa.is_on_curve());
        assert_eq!((pa.to_projective() + pb.to_projective()).to_affine(), g1((a + b).into_affine()));
        assert_eq!((pa.to_projective() + pb).to_affine(), g1((a + b).into_affine()));
        assert_eq!(pa.to_projective().double().to_affine(), g1(a.double().into_affine()));
        assert_eq!((pa.to_projective() + pa).to_affine(), g1(a.double().into_affine()));
        assert!((pa.to_projective() + (-pa)).is_identity());
        let s = ark::Fr::rand(&mut rng);
        assert_eq!(pa.mul(&fr(s)).to_affine(), g1((a * s).into_affine()));

        let (c, d) = (ark::G2Projective::rand(&mut rng), ark::G2Projective::rand(&mut rng));
        let (qc, qd) = (g2(c.into_affine()), g2(d.into_affine()));
        assert!(qc.is_on_curve() && qc.is_in_subgroup());
        assert_eq!((qc.to_projective() + qd).to_affine(), g2((c + d).into_affine()));
        assert_eq!(qc.to_projective().double().to_affine(), g2(c.double().into_affine()));
        assert_eq!(qc.mul(&fr(s)).to_affine(), g2((c * s).into_affine()));
    }
    let points: Vec<G1Projective> = (0..20)
        .map(|i| {
            if i % 7 == 3 {
                G1Projective::IDENTITY
            } else {
                G1Projective::generator().mul(&Fr::from_u64(i as u64 + 1))
            }
        })
        .collect();
    let affine = G1Projective::batch_to_affine(&points);
    for (p, a) in points.iter().zip(&affine) {
        assert_eq!(p.to_affine(), *a);
    }
    assert!(G1Affine::IDENTITY.is_on_curve());
    assert_eq!(G1Projective::IDENTITY.to_affine(), G1Affine::IDENTITY);
    let g2s: Vec<G2Projective> = (1..5).map(|i| G2Projective::generator().mul(&Fr::from_u64(i))).collect();
    let a2 = G2Projective::batch_to_affine(&g2s);
    assert_eq!(a2[3], g2s[3].to_affine());
}

#[test]
fn g2_subgroup() {
    // A point on the twist outside the prime-order subgroup: arkworks'
    // unchecked point from x.
    let mut rng = ark_std::test_rng();
    let mut outside = 0;
    for _ in 0..40 {
        let x = ark::Fq2::rand(&mut rng);
        if let Some(p) = ark::G2Affine::get_point_from_x_unchecked(x, false) {
            let ours = g2(p);
            assert!(ours.is_on_curve());
            assert_eq!(ours.is_in_subgroup(), p.is_in_correct_subgroup_assuming_on_curve());
            outside += !ours.is_in_subgroup() as usize;
        }
    }
    assert!(outside > 0);
}

#[test]
fn pairings() {
    let mut rng = ark_std::test_rng();
    for _ in 0..8 {
        let p = ark::G1Projective::rand(&mut rng).into_affine();
        let q = ark::G2Projective::rand(&mut rng).into_affine();
        let expected = ark::Bn254::pairing(p, q).0;
        assert_eq!(pairing::pairing(&g1(p), &g2(q)), fq12(expected));
    }
    // e(a P, b Q) e(-ab P, Q) = 1.
    let (a, b) = (ark::Fr::rand(&mut rng), ark::Fr::rand(&mut rng));
    let p = ark::G1Projective::generator();
    let q = ark::G2Projective::generator();
    let left = g1((p * a).into_affine());
    let right = pairing::G2Prepared::new(&g2((q * b).into_affine()));
    let third = g1((-(p * (a * b))).into_affine());
    let gen = pairing::G2Prepared::new(&g2(q.into_affine()));
    assert!(pairing::pairing_product_is_one(&[(left, &right), (third, &gen)]));
    assert!(!pairing::pairing_product_is_one(&[(left, &right), (left, &gen)]));
    let _ = ark_fq;
}
