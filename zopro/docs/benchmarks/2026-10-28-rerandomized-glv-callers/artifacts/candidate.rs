// Integer decomposition is copied from accepted g2_phi.rs in split.rs.
#[path = "split.rs"]
mod split;
use super::*;
const BETA: Fq = Fq::from_decimal(
    "21888242871839275220042445260109153167277707414472061641714758635765020556616",
);
fn components(s: Fr, g1: bool) -> [i128; 2] {
    let w = split::phi_split(s.to_canonical());
    let signed = |i: usize| {
        let magnitude = w[i] as u128 | (((w[i + 1] & 0x7fff_ffff_ffff_ffff) as u128) << 64);
        if w[i + 1] >> 63 == 0u64 {
            magnitude as i128
        } else {
            -(magnitude as i128)
        }
    };
    let (a, b) = (signed(0), signed(2));
    if g1 {
        [a - b, -b]
    } else {
        [a, b]
    }
}
fn digits(value: i128) -> ([i8; 128], usize) {
    let negative = value < 0;
    let mut magnitude = value.unsigned_abs();
    let mut out = [0; 128];
    let mut used = 0;
    while magnitude != 0 {
        assert!(used < out.len());
        if magnitude & 1 != 0 {
            let mut d = (magnitude & 31) as i8;
            if d > 16 {
                d -= 32;
            }
            if d > 0 {
                magnitude -= d as u128;
            } else {
                magnitude += (-d) as u128;
            }
            out[used] = if negative { -d } else { d };
        }
        magnitude >>= 1;
        used += 1;
    }
    (out, used)
}
fn phi_g1(p: G1Affine) -> G1Affine {
    G1Affine::new(p.x * BETA, p.y)
}
fn phi_g2(p: G2Affine) -> G2Affine {
    G2Affine::new(p.x.mul_by_fp(&BETA), p.y)
}
pub(super) fn pair_g1(base: G1Projective, left: Fr, right: Fr) -> (G1Projective, G1Projective) {
    let table = original::odd_multiples(base);
    let phi: [G1Affine; 8] = core::array::from_fn(|i| phi_g1(table[i]));
    let [a, b] = components(left, true);
    let [c, d] = components(right, true);
    let ds = [digits(a), digits(b), digits(c), digits(d)];
    let mut l = G1Projective::IDENTITY;
    let mut r = G1Projective::IDENTITY;
    for i in (0..ds.iter().map(|d| d.1).max().unwrap()).rev() {
        l = l.double();
        r = r.double();
        original::add_digit(&mut l, &table, ds[0].0[i]);
        original::add_digit(&mut l, &phi, ds[1].0[i]);
        original::add_digit(&mut r, &table, ds[2].0[i]);
        original::add_digit(&mut r, &phi, ds[3].0[i]);
    }
    (l, r)
}
pub(super) fn joint_g2(
    base: G2Projective,
    u: Fr,
    delta: G2Affine,
    uv: Fr,
    prepared: Option<&FixedBase<G2Config>>,
) -> G2Projective {
    let table = original::odd_multiples(base);
    let phi: [G2Affine; 8] = core::array::from_fn(|i| phi_g2(table[i]));
    let delta_table = prepared
        .is_none()
        .then(|| original::odd_multiples(delta.to_projective()));
    let dt: [G2Affine; 8] = core::array::from_fn(|i| {
        prepared.map_or_else(|| delta_table.as_ref().unwrap()[i], |t| t.0[2 * i])
    });
    let dp: [G2Affine; 8] = dt.map(phi_g2);
    let [a, b] = components(u, false);
    let [c, d] = components(uv, false);
    let ds = [digits(a), digits(b), digits(c), digits(d)];
    let mut out = G2Projective::IDENTITY;
    for i in (0..ds.iter().map(|d| d.1).max().unwrap()).rev() {
        out = out.double();
        for (table, digit) in [
            (&table[..], ds[0].0[i]),
            (&phi[..], ds[1].0[i]),
            (&dt[..], ds[2].0[i]),
            (&dp[..], ds[3].0[i]),
        ] {
            original::add_digit(&mut out, table, digit);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn signed_decomposition_and_digits() {
        let l2 = Fr::from_decimal("4407920970296243842393367215006156084916469457145843978461");
        let l1 = -Fr::ONE - l2;
        let signed_field = |x: i128| {
            let w = x.unsigned_abs();
            let f = Fr::from_canonical([w as u64, (w >> 64) as u64, 0, 0]).unwrap();
            if x < 0 {
                -f
            } else {
                f
            }
        };
        let mut seed = 0x47ab_1943_78d2_2631;
        let values = boundary_scalars()
            .into_iter()
            .chain((0..4096).map(|_| random_scalar(&mut seed)));
        for s in values {
            for (g1, lambda) in [(true, l1), (false, l2)] {
                let [a, b] = components(s, g1);
                assert!(a.unsigned_abs() < 1u128 << 127 && b.unsigned_abs() < 1u128 << 127);
                assert_eq!(signed_field(a) + lambda * signed_field(b), s);
                for x in [a, b] {
                    let (d, n) = digits(x);
                    let mut v = 0i128;
                    for &z in d[..n].iter().rev() {
                        v = v.checked_mul(2).unwrap().checked_add(z as i128).unwrap();
                    }
                    assert_eq!(v, x);
                }
            }
        }
    }
    #[test]
    fn pair_matches_independent_group_oracle() {
        let mut seed = 99;
        let scalars: Vec<_> = boundary_scalars()
            .into_iter()
            .chain((0..32).map(|_| random_scalar(&mut seed)))
            .collect();
        for (i, &s) in scalars.iter().enumerate() {
            let t = scalars[(i * 7 + 3) % scalars.len()];
            let k = if i % 7 == 0 {
                Fr::ZERO
            } else {
                Fr::from((i + 17) as u64)
            };
            let base = G1Projective::generator() * k;
            let want = (oracle_g1(k * s), oracle_g1(k * t));
            assert_eq!(original::pair_g1(base, s, t), want);
            assert_eq!(pair_g1(base, s, t), want);
        }
    }
    #[test]
    fn joint_prepared_and_fallback_match_independent_group_oracle() {
        let delta = G2Affine::GENERATOR * Fr::from(37u64);
        let delta = delta.to_affine();
        let prepared = FixedBase::new(delta);
        let mut seed = 105;
        let scalars: Vec<_> = boundary_scalars()
            .into_iter()
            .chain((0..32).map(|_| random_scalar(&mut seed)))
            .collect();
        for (i, &s) in scalars.iter().enumerate() {
            let t = scalars[(i * 7 + 3) % scalars.len()];
            let k = if i % 7 == 0 {
                Fr::ZERO
            } else {
                Fr::from((i + 17) as u64)
            };
            let base = G2Projective::generator() * k;
            let want = oracle_g2(k * s + Fr::from(37u64) * t);
            for p in [None, Some(&prepared)] {
                assert_eq!(original::joint_g2(base, s, delta, t, p), want);
                assert_eq!(joint_g2(base, s, delta, t, p), want);
            }
        }
    }
}
