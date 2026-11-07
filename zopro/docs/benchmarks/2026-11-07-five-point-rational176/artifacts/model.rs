use ark_bn254::{Fq, Fq2, Fr};
use ark_ec::{
    short_weierstrass::{Affine, SWCurveConfig},
    AffineRepr, CurveGroup,
};
use ark_ff::{Field, Fp64, MontBackend, MontConfig, One, PrimeField, Zero};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/five-point-rational176";
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
struct Cost {
    m: u64,
    s: u64,
    i: u64,
}
fn mul<F: Field>(a: F, b: F, c: &mut Cost) -> F {
    c.m += 1;
    a * b
}
fn sq<F: Field>(a: F, c: &mut Cost) -> F {
    c.s += 1;
    a.square()
}
fn inv<F: Field>(a: F, c: &mut Cost) -> F {
    c.i += 1;
    a.inverse().unwrap()
}
fn eval<F: Field>(p: &[F], x: F) -> F {
    p.iter().rev().fold(F::zero(), |a, b| a * x + b)
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Decline {
    EqualX,
    LeadingZero,
    Pole,
}
struct ResultPoint<F: Field> {
    x: F,
    y: F,
    cost: Cost,
    pre_m: u64,
}
fn five<F: Field>(p: &[(F, F); 5], b: F) -> Result<ResultPoint<F>, Decline> {
    let mut c = Cost::default();
    let mut d = [[F::zero(); 5]; 5];
    for i in 0..5 {
        for j in i + 1..5 {
            d[i][j] = p[i].0 - p[j].0;
            if d[i][j].is_zero() {
                return Err(Decline::EqualX);
            }
        }
    }
    // Products of K4 edge complements, shared explicitly:21products.
    let a = mul(d[0][3], d[0][4], &mut c);
    let bb = mul(d[1][3], d[1][4], &mut c);
    let cc = mul(d[0][2], d[0][4], &mut c);
    let dd = mul(d[1][2], d[1][4], &mut c);
    let e = mul(d[0][2], d[0][3], &mut c);
    let f = mul(d[1][2], d[1][3], &mut c);
    let t = mul(d[2][3], d[2][4], &mut c);
    let t = mul(t, d[3][4], &mut c);
    let mut n = [F::zero(); 5];
    n[0] = mul(t, f, &mut c);
    n[0] = mul(n[0], d[1][4], &mut c);
    n[1] = mul(t, e, &mut c);
    n[1] = mul(n[1], d[0][4], &mut c);
    n[2] = mul(d[0][1], d[3][4], &mut c);
    n[2] = mul(n[2], a, &mut c);
    n[2] = mul(n[2], bb, &mut c);
    n[3] = mul(d[0][1], d[2][4], &mut c);
    n[3] = mul(n[3], cc, &mut c);
    n[3] = mul(n[3], dd, &mut c);
    n[4] = mul(d[0][1], d[2][3], &mut c);
    n[4] = mul(n[4], e, &mut c);
    n[4] = mul(n[4], f, &mut c);
    assert_eq!(c.m, 21);
    let delta = mul(d[0][1], e, &mut c);
    let delta = mul(delta, d[0][4], &mut c);
    let delta = mul(delta, n[0], &mut c);
    assert_eq!(c.m, 24);
    let mut w = [F::zero(); 5];
    for i in 0..5 {
        w[i] = mul(p[i].1, n[i], &mut c);
        if i % 2 == 1 {
            w[i] = -w[i];
        }
    }
    assert_eq!(c.m, 29);
    let pre_m = c.m;
    // Monic V product: known leading units are assigned, not multiplied.
    let mut v = vec![F::one()];
    for point in p {
        let degree = v.len() - 1;
        let mut next = vec![F::zero(); v.len() + 1];
        for k in 0..=degree {
            next[k + 1] += v[k];
            next[k] -= if k == degree {
                point.0
            } else {
                mul(point.0, v[k], &mut c)
            };
        }
        v = next;
    }
    assert_eq!(c.m, 39);
    let mut y = [F::zero(); 5];
    for i in 0..5 {
        let x = p[i].0;
        let mut q = [F::zero(); 5];
        q[4] = F::one();
        q[3] = v[4] + x;
        for k in (0..=2).rev() {
            q[k] = v[k + 1] + mul(x, q[k + 1], &mut c);
        }
        for k in 0..4 {
            y[k] += mul(w[i], q[k], &mut c);
        }
        y[4] += w[i];
    }
    assert_eq!(c.m, 74);
    // Correctness-only polynomial identities are outside counted arithmetic.
    for i in 0..5 {
        assert_eq!(eval(&v, p[i].0), F::zero());
        assert_eq!(eval(&y, p[i].0), delta * p[i].1);
    }
    let u = y[4];
    let vv = y[3] - mul(u, v[4], &mut c);
    let u2 = sq(u, &mut c);
    let mut nn = [F::zero(); 4];
    for k in 0..4 {
        let left = if k == 0 {
            F::zero()
        } else {
            mul(u, y[k - 1], &mut c)
        };
        nn[k] = left - mul(vv, y[k], &mut c) - mul(u2, v[k], &mut c);
    }
    assert_eq!(u * y[3] - vv * y[4] - u2 * v[4], F::zero());
    assert_eq!(u * y[4] - u2, F::zero());
    let d1 = mul(delta, u, &mut c);
    let d0 = -mul(delta, vv, &mut c);
    for &(x, yy) in p {
        assert_eq!(eval(&nn, x), (d1 * x + d0) * yy);
    }
    let z = nn[3];
    if z.is_zero() {
        return Err(Decline::LeadingZero);
    }
    let z2 = sq(z, &mut c);
    let cross = mul(nn[3], nn[2], &mut c);
    let x = sq(d1, &mut c) - cross - cross + mul(v[4], z2, &mut c);
    let h = mul(d1, x, &mut c) + mul(d0, z2, &mut c);
    if h.is_zero() {
        return Err(Decline::Pole);
    }
    let z4 = sq(z2, &mut c);
    let z6 = mul(z4, z2, &mut c);
    let nh = mul(nn[3], x, &mut c) + mul(nn[2], z2, &mut c);
    let nh = mul(nh, x, &mut c) + mul(nn[1], z4, &mut c);
    let nh = mul(nh, x, &mut c) + mul(nn[0], z6, &mut c);
    let combined = mul(z2, h, &mut c);
    let inverse = inv(combined, &mut c);
    let xx = mul(x, h, &mut c);
    let xx = mul(xx, inverse, &mut c);
    let ii = sq(inverse, &mut c);
    let yy = -mul(nh, h, &mut c);
    let yy = mul(yy, ii, &mut c);
    assert_eq!(c, Cost { m: 104, s: 5, i: 1 });
    assert_eq!(yy.square(), xx.square() * xx + b);
    // Independently expand N²-D²*(x³+b)=z²*V*(x-x6), all7coefficients.
    let mut left = [F::zero(); 7];
    for i in 0..4 {
        for j in 0..4 {
            left[i + j] += nn[i] * nn[j];
        }
    }
    let dp = [d0.square(), (d0 * d1).double(), d1.square()];
    for k in 0..3 {
        left[k] -= b * dp[k];
        left[k + 3] -= dp[k];
    }
    let mut right = [F::zero(); 7];
    for k in 0..6 {
        right[k] -= z2 * xx * v[k];
        right[k + 1] += z2 * v[k];
    }
    assert_eq!(left, right);
    for &(xi, _) in p {
        assert!(!(d1 * xi + d0).is_zero());
    }
    Ok(ResultPoint {
        x: xx,
        y: yy,
        cost: c,
        pre_m,
    })
}
fn scalar(label: &str) -> Fr {
    Fr::from_le_bytes_mod_order(&Sha256::digest(label.as_bytes()))
}
fn curve<C: SWCurveConfig>(name: &str) -> Value {
    assert!(C::COEFF_A.is_zero());
    let mut successes = 0;
    let mut declined = BTreeMap::new();
    let generator = Affine::<C>::generator();
    let mut digest = Sha256::new();
    for k in 0..256 {
        let points: [Affine<C>; 5] = std::array::from_fn(|j| {
            generator
                .mul_bigint(
                    scalar(&format!("five-rational176/seed-6f219ac8/{name}/{k}/{j}")).into_bigint(),
                )
                .into_affine()
        });
        let pairs = points.map(|p| (p.x, p.y));
        let expected = points
            .iter()
            .fold(
                ark_ec::short_weierstrass::Projective::<C>::zero(),
                |a, p| a + p,
            )
            .into_affine();
        match five(&pairs, C::COEFF_B) {
            Ok(r) => {
                assert_eq!(Affine::<C>::new_unchecked(r.x, r.y), expected);
                assert_eq!(r.cost, Cost { m: 104, s: 5, i: 1 });
                assert_eq!(r.pre_m, 29);
                digest.update(format!("{:?}{:?}", r.x, r.y));
                successes += 1;
            }
            Err(e) => {
                *declined.entry(format!("{e:?}")).or_insert(0) += 1;
            }
        }
    }
    assert_eq!(successes, 256);
    let base: [Affine<C>; 5] =
        std::array::from_fn(|j| generator.mul_bigint([(j as u64 + 2) * 17]).into_affine());
    let pair = base.map(|p| (p.x, p.y));
    let mut duplicate = pair;
    duplicate[4] = duplicate[0];
    assert_eq!(five(&duplicate, C::COEFF_B).err(), Some(Decline::EqualX));
    let mut opposite = pair;
    opposite[4] = (opposite[0].0, -opposite[0].1);
    assert_eq!(five(&opposite, C::COEFF_B).err(), Some(Decline::EqualX));
    let mut zero = base;
    let s = zero[..4].iter().fold(
        ark_ec::short_weierstrass::Projective::<C>::zero(),
        |a, p| a + p,
    );
    zero[4] = (-s).into_affine();
    assert!(!zero[4].is_zero());
    let d = five(&zero.map(|p| (p.x, p.y)), C::COEFF_B);
    assert_eq!(d.err(), Some(Decline::LeadingZero));
    let mut pole = base;
    let s = pole[1..4].iter().fold(
        ark_ec::short_weierstrass::Projective::<C>::zero(),
        |a, p| a + p,
    );
    pole[4] = (-s).into_affine();
    assert_eq!(
        five(&pole.map(|p| (p.x, p.y)), C::COEFF_B).err(),
        Some(Decline::Pole)
    );
    // Infinity is classified before entering the finite-coordinate recipe.
    let mut with_identity = base;
    with_identity[2] = Affine::<C>::zero();
    assert!(with_identity.iter().any(AffineRepr::is_zero));
    json!({"curve":name,"new_full_width_five_point_sums":successes,"unexpected_declines":declined,"explicit_guard_checks":5,"M":104,"S":5,"I":1,"preprocessing_M":29,"output_digest":format!("{:x}",digest.finalize())})
}
#[derive(MontConfig)]
#[modulus = "43"]
#[generator = "3"]
struct F43Config;
type F43 = Fp64<MontBackend<F43Config, 1>>;
fn rem(x: i64) -> i64 {
    x.rem_euclid(43)
}
fn power(mut a: i64, mut n: u32) -> i64 {
    let mut r = 1;
    while n > 0 {
        if n & 1 != 0 {
            r = rem(r * a);
        }
        a = rem(a * a);
        n >>= 1;
    }
    r
}
fn add(a: Option<(i64, i64)>, b: Option<(i64, i64)>) -> Option<(i64, i64)> {
    match (a, b) {
        (None, v) | (v, None) => v,
        (Some((x, y)), Some((u, v))) => {
            if x == u && rem(y + v) == 0 {
                return None;
            }
            let slope = if x == u {
                rem(3 * x * x * power(rem(2 * y), 41))
            } else {
                rem((v - y) * power(rem(u - x), 41))
            };
            let xx = rem(slope * slope - x - u);
            Some((xx, rem(slope * (x - xx) - y)))
        }
    }
}
fn small() -> Value {
    let mut pts = vec![];
    for x in 0..43 {
        for y in 0..43 {
            if rem(y * y) == rem(x * x * x + 3) {
                pts.push((x, y));
            }
        }
    }
    let mut success = 0;
    let mut declines = BTreeMap::new();
    let mut state = 0x83ad9bc71302f6e1u64;
    for _ in 0..2048 {
        let raw: [(i64, i64); 5] = std::array::from_fn(|_| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            pts[state as usize % pts.len()]
        });
        let ff = raw.map(|(x, y)| (F43::from(x as u64), F43::from(y as u64)));
        let expected = raw.iter().fold(None, |a, b| add(a, Some(*b)));
        match five(&ff, F43::from(3)) {
            Ok(r) => {
                let ex = expected.unwrap();
                assert_eq!(r.x, F43::from(ex.0 as u64));
                assert_eq!(r.y, F43::from(ex.1 as u64));
                success += 1;
            }
            Err(e) => {
                *declines.entry(format!("{e:?}")).or_insert(0) += 1;
            }
        }
    }
    assert!(success > 500);
    assert!(declines.len() >= 2);
    json!({"independent_integer_group_oracle":true,"tuples":2048,"accepted":success,"declines":declines,"curve_affine_points":pts.len()})
}
fn main() {
    let results = json!([
        curve::<ark_bn254::g1::Config>("G1"),
        curve::<ark_bn254::g2::Config>("G2")
    ]);
    let small = small();
    let mut rows = vec![];
    for g in [1u64, 2, 4, 8, 16, 32, 64, 128, 256] {
        let g1 = 100 * (112 * g + 2);
        let g2 = 100 * (15 * g + 2) + 150 * (209 * g);
        let b1 = 100 * (24 * g + 15);
        let b2 = 100 * (28 * g + 15) + 150 * (20 * g);
        rows.push(json!({"eight_lane_five_input_groups":g,"G1_candidate_VM":112*g+2,"G1_candidate_proxy":g1,"G1_binary_proxy":b1,"G2_candidate_VM":15*g+2,"G2_candidate_VF":209*g,"G2_candidate_proxy":g2,"G2_binary_proxy":b2,"G2_binary_conservative_extra_VM_proxy":b2+400*g,"candidate_scalar_inverse_barriers":1,"binary_inverse_barriers":3,"G1_gain":g1<b1,"G2_gain":g2<b2}));
    }
    assert!(rows
        .iter()
        .all(|r| r["G1_gain"] == false && r["G2_gain"] == false));
    let analysis = json!({"model_passed":true,"native_candidate_selected":false,"variable_vector_work_gate_passed":false,"model_scope":"division-free rational five-point construction; full BN254G1/G2 plus independentF43integergroup oracle","full_width_results":results,"small_field":small,"work":{"M":104,"S":5,"I":1,"preprocessing_M":29,"complement_products_M":21,"Delta_M":3,"weighted_inputs_M":5},"rows":rows,"memory":"fixed local polynomial/point arrays; no key-sized retained table; not native peak or minimumRAM","limits":"Source-work proxies exclude additions, packing, masks, spills, scalar inverse prices, scheduling and memory. Rejects this fixed recipe, not all rational/higher-arity algorithms. No native kernel/proof/timing or production change."});
    let out = format!("{R}/analysis.json");
    assert!(!Path::new(&out).exists());
    fs::write(out, serde_json::to_vec_pretty(&analysis).unwrap()).unwrap();
    println!("PASS512fullwidthfivepointsums+2048smallfieldtuples+10guards;fixed rational recipe arithmetic gate fails");
}
