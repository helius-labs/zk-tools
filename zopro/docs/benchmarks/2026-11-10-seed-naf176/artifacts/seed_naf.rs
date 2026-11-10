//! Exact fixed-integer NAF for the G2 subgroup seed; no point table.
use super::{G2Affine, G2Projective};

const POSITIVE: u64 = 0x450a14044a890a01;
const NEGATIVE: u64 = 0x0020815000200010;

/// POSITIVE - NEGATIVE is the integer seed, not a scalar-field reduction.
/// The top digit is +1 at bit62. All later digits are fixed public data.
pub(super) fn multiply(point: &G2Affine) -> G2Projective {
    let negative = -*point;
    let mut result = point.to_projective();
    for bit in (0..62).rev() {
        result = result.double();
        if POSITIVE >> bit & 1 != 0 {
            result = result.add_affine(point);
        } else if NEGATIVE >> bit & 1 != 0 {
            result = result.add_affine(&negative);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Fq, Fq2, FrConfig, FpConfig};
    use ark_ec::{AffineRepr, CurveGroup, PrimeGroup};
    use ark_ff::{PrimeField, UniformRand, Zero};
    use ark_ref as ark;

    fn convert(p: ark::G2Affine) -> G2Affine {
        if p.is_zero() { return G2Affine::IDENTITY; }
        let fq = |v: ark::Fq| Fq::from_montgomery(v.0.0);
        G2Affine::new(Fq2::new(fq(p.x.c0),fq(p.x.c1)), Fq2::new(fq(p.y.c0),fq(p.y.c1)))
    }

    #[test]
    fn subgroup_naf_exact_seed_and_independent_full_curve_results() {
        const SEED: u64 = 4965661367192848881;
        assert_eq!(POSITIVE as u128 - NEGATIVE as u128, SEED as u128);
        assert_eq!(POSITIVE & NEGATIVE, 0);
        let active = POSITIVE | NEGATIVE;
        assert_eq!(active & (active << 1),0);
        assert_eq!(active.count_ones(),24);
        assert_eq!(SEED.count_ones(),28);
        assert_eq!(active >> 62,1);
        assert_eq!(POSITIVE >> 62,1);
        let mut value=1i128;
        for bit in (0..62).rev() { value=2*value+((POSITIVE>>bit)&1)as i128-((NEGATIVE>>bit)&1)as i128; }
        assert_eq!(value,SEED as i128);
        let mut rng=ark_std::test_rng();
        let mut points=vec![ark::G2Affine::zero(),ark::G2Projective::generator().into_affine(),(-ark::G2Projective::generator()).into_affine()];
        for _ in 0..64 { points.push(ark::G2Projective::rand(&mut rng).into_affine()); }
        let mut raw=0;
        for _ in 0..256 {
            if let Some(p)=ark::G2Affine::get_point_from_x_unchecked(ark::Fq2::rand(&mut rng),false) {
                raw+=1;
                points.extend([p,-p,p.mul_bigint(FrConfig::P).into_affine()]);
            }
        }
        assert!(raw>0);
        let mut outside=0;
        for p in &points {
            let ours=convert(*p);
            assert!(ours.is_on_curve());
            let expected=convert(p.mul_bigint([SEED]).into_affine());
            assert_eq!(multiply(&ours).to_affine(),expected);
            assert_eq!(multiply(&ours),ours.mul_limbs(&[SEED]));
            let subgroup=p.mul_bigint(FrConfig::P).is_zero();
            assert_eq!(ours.is_in_subgroup(),subgroup);
            assert_eq!(subgroup,p.is_in_correct_subgroup_assuming_on_curve());
            outside+=usize::from(!subgroup);
        }
        assert!(outside>0);
        eprintln!("SEED_NAF_CASES points={} raw_twist_points={raw} outside_subgroup={outside}; exact_integer_seed/Ark_seed/full_order/Ark_subgroup",points.len());
    }

    #[test]
    fn subgroup_naf_small_stack_workers_and_shared_immutable_inputs() {
        let generator=ark::G2Projective::generator();
        let mut fixtures=vec![(G2Affine::IDENTITY,true)];
        for i in 1..=4 { fixtures.push((convert((generator*ark::Fr::from(i)).into_affine()),true)); }
        let mut rng=ark_std::test_rng();
        while fixtures.len()<12 {
            if let Some(p)=ark::G2Affine::get_point_from_x_unchecked(ark::Fq2::rand(&mut rng),false) {
                fixtures.push((convert(p),p.mul_bigint(FrConfig::P).is_zero()));
            }
        }
        let original=fixtures.clone();
        for workers in [1,4,16,32] {
            std::thread::scope(|scope| {
                let mut jobs=vec![];
                for _ in 0..workers {
                    let values=&fixtures;
                    jobs.push(std::thread::Builder::new().stack_size(512*1024).spawn_scoped(scope,move||{
                        for (p,expected) in values {
                            assert_eq!(p.is_in_subgroup(),*expected);
                            assert_eq!(multiply(p),p.mul_limbs(&[4965661367192848881]));
                        }
                    }).unwrap());
                }
                for j in jobs { j.join().unwrap(); }
            });
        }
        assert_eq!(fixtures,original);
        eprintln!("SEED_NAF_THREADS workers=53 fixtures=12 subgroup_checks=636 seed_results=636 requested_stack=524288 immutable_inputs=true");
    }
}
