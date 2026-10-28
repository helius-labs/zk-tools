// Integer decomposition is copied from accepted g2_phi.rs in split.rs.
mod split {
const C1: [u64; 3] = [7978627105577135859, 15644699364383830999, 2];
const C2: [u64; 4] = [
    11953552847224317658,
    8825887400277225869,
    5534624963584316111,
    2,
];
const V1X: u128 = 9931322734385697763;
const V1Y_ABS: u128 = 147946756881789319000765030803803410728;
const V2X: u128 = 147946756881789319010696353538189108491;
const V2Y: u128 = 9931322734385697763;
#[inline(always)]
fn product<const N: usize, const M: usize, const K: usize>(a: [u64; N], b: [u64; M]) -> [u64; K] {
    assert_eq!(K, N + M);
    let mut out = [0u64; K];
    for i in 0..N {
        let mut carry = 0u128;
        for j in 0..M {
            let t = a[i] as u128 * b[j] as u128 + out[i + j] as u128 + carry;
            out[i + j] = t as u64;
            carry = t >> 64;
        }
        out[i + M] = carry as u64;
    }
    out
}
#[inline(always)]
fn sign_magnitude(value: u128) -> u128 {
    let sign = ((value as i128) >> 127) as u128;
    let m = (value ^ sign).wrapping_sub(sign);
    assert!(m < 1u128 << 126);
    m | (sign & (1u128 << 127))
}
pub(super) fn phi_split(s: [u64; 4]) -> [u64; 4] {
    let a = product::<2, 3, 5>([s[2], s[3]], C1);
    let b = product::<3, 4, 7>([s[1], s[2], s[3]], C2);
    debug_assert_eq!(a[4], 0);
    debug_assert_eq!(b[6], 0);
    let q1 = a[3] as u128 + u128::from(a[2] >> 63);
    let q2 = (b[4] as u128 | ((b[5] as u128) << 64)) + u128::from(b[3] >> 63);
    let low = s[0] as u128 | ((s[1] as u128) << 64);
    let k1 = low
        .wrapping_sub(q1.wrapping_mul(V1X))
        .wrapping_sub(q2.wrapping_mul(V2X));
    let k2 = q1.wrapping_mul(V1Y_ABS).wrapping_sub(q2.wrapping_mul(V2Y));
    let (a, b) = (sign_magnitude(k1), sign_magnitude(k2));
    [a as u64, (a >> 64) as u64, b as u64, (b >> 64) as u64]
}

}
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
    let table = super::odd_multiples(base);
    let phi: [G1Affine; 8] = core::array::from_fn(|i| phi_g1(table[i]));
    let [a, b] = components(left, true);
    let [c, d] = components(right, true);
    let ds = [digits(a), digits(b), digits(c), digits(d)];
    let mut l = G1Projective::IDENTITY;
    let mut r = G1Projective::IDENTITY;
    for i in (0..ds.iter().map(|d| d.1).max().unwrap()).rev() {
        l = l.double();
        r = r.double();
        super::add_digit(&mut l, &table, ds[0].0[i]);
        super::add_digit(&mut l, &phi, ds[1].0[i]);
        super::add_digit(&mut r, &table, ds[2].0[i]);
        super::add_digit(&mut r, &phi, ds[3].0[i]);
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
    let table = super::odd_multiples(base);
    let phi: [G2Affine; 8] = core::array::from_fn(|i| phi_g2(table[i]));
    let delta_table = prepared
        .is_none()
        .then(|| super::odd_multiples(delta.to_projective()));
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
            super::add_digit(&mut out, table, digit);
        }
    }
    out
}

