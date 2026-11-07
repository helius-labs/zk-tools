use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
use zopro_bn254::{Affine, CurveConfig, Field, Fq, Fq2, Fr, G1Config, G2Config, Projective};

const ROOT: &str = "/home/ubuntu/src/zopro-perf-work/common-z-product-tree176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const MANIFEST: &str = "docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json";
const LENGTHS: &[usize] = &[
    2, 3, 4, 7, 8, 15, 16, 31, 32, 63, 64, 127, 128, 255, 256, 511, 512, 1024, 4096,
];
fn path(p: &str) -> PathBuf {
    Path::new(ROOT).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    assert!(!p.as_ref().exists());
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
struct Cost {
    m: u64,
    s: u64,
    i: u64,
}
impl Cost {
    fn add(&mut self, b: Self) {
        self.m += b.m;
        self.s += b.s;
        self.i += b.i;
    }
    fn json(self) -> Value {
        json!({"M":self.m,"S":self.s,"I":self.i})
    }
}
fn mul<F: Field>(a: F, b: F, c: &mut Cost) -> F {
    c.m += 1;
    a * b
}
fn square<F: Field>(a: F, c: &mut Cost) -> F {
    c.s += 1;
    a.square()
}

// The caller owns one shared nonzero Z. The input buffer holds only X and Y.
// All denominators are inspected before the first coordinate write at a level.
fn level<F: Field>(p: &mut Vec<(F, F)>, z: F, known_one: bool, c: &mut Cost) -> Option<F> {
    let n = p.len();
    let m = n / 2;
    assert!(m > 0);
    let ds = (0..m)
        .map(|j| p[2 * j + 1].0 - p[2 * j].0)
        .collect::<Vec<_>>();
    if ds.iter().any(|d| d.is_zero()) {
        return None;
    }
    let carry = if n % 2 == 1 { Some(p[n - 1]) } else { None };
    if m == 1 {
        let d = ds[0];
        let e = p[1].1 - p[0].1;
        let a = square(d, c);
        let b = mul(p[0].0, a, c);
        let v = mul(p[1].0, a, c);
        let x = square(e, c) - b - v;
        let y = mul(e, b - x, c) - mul(p[0].1, v - b, c);
        let next = if known_one { d } else { mul(z, d, c) };
        p[0] = (x, y);
        if let Some((x, y)) = carry {
            let t = mul(a, d, c);
            p[1] = (mul(x, a, c), mul(y, t, c));
        }
        p.truncate(m + usize::from(carry.is_some()));
        return Some(next);
    }
    let mut q = vec![F::ONE; m];
    let mut d = ds[0];
    for j in 1..m {
        q[j] = d;
        d = mul(d, ds[j], c);
    }
    let mut right = ds[m - 1];
    for j in (1..m - 1).rev() {
        q[j] = mul(q[j], right, c);
        right = mul(right, ds[j], c);
    }
    q[0] = right;
    let s = square(d, c);
    let t = mul(s, d, c);
    let next = if known_one { d } else { mul(z, d, c) };
    for j in 0..m {
        let (ax, ay) = p[2 * j];
        let (bx, by) = p[2 * j + 1];
        let u = mul(by - ay, q[j], c);
        let x = square(u, c) - mul(ax + bx, s, c);
        let b = mul(ax, s, c);
        let y = mul(u, b - x, c) - mul(ay, t, c);
        p[j] = (x, y);
    }
    if let Some((x, y)) = carry {
        p[m] = (mul(x, s, c), mul(y, t, c));
    }
    p.truncate(m + usize::from(carry.is_some()));
    Some(next)
}
fn level_formula(n: usize, known_one: bool) -> Cost {
    let m = (n / 2) as u64;
    let carry = (n % 2) as u64;
    Cost {
        m: if m == 1 {
            4 + u64::from(!known_one) + 3 * carry
        } else {
            8 * m - 4 + u64::from(!known_one) + 2 * carry
        },
        s: if m == 1 { 2 } else { m + 1 },
        i: 0,
    }
}
fn tree_formula(mut n: usize) -> (Cost, Cost, usize) {
    let mut c = Cost::default();
    let mut a = Cost::default();
    let mut depth = 0;
    while n > 1 {
        let m = (n / 2) as u64;
        c.add(level_formula(n, depth == 0));
        a.add(Cost {
            m: 5 * m - 3,
            s: m,
            i: 1,
        });
        n = n.div_ceil(2);
        depth += 1;
    }
    (c, a, depth)
}
fn fast<C: CurveConfig>(
    inputs: &[Affine<C>],
    scale: C::Base,
) -> Option<(Projective<C>, Cost, usize)> {
    if inputs.len() < 2 || scale.is_zero() || inputs.iter().any(Affine::is_identity) {
        return None;
    }
    // Initial research rescaling is test setup, outside the recipe count.
    let s = scale.square();
    let t = s * scale;
    let mut p = inputs
        .iter()
        .map(|p| (p.x * s, p.y * t))
        .collect::<Vec<_>>();
    let mut z = scale;
    let mut c = Cost::default();
    let mut depth = 0;
    while p.len() > 1 {
        let n = p.len();
        let before = c;
        let known_one = depth == 0 && scale == C::Base::ONE;
        z = level(&mut p, z, known_one, &mut c)?;
        assert_eq!(
            c,
            Cost {
                m: before.m + level_formula(n, known_one).m,
                s: before.s + level_formula(n, known_one).s,
                i: 0
            }
        );
        depth += 1;
    }
    Some((Projective::new(p[0].0, p[0].1, z), c, depth))
}
fn complete<C: CurveConfig>(p: &[Affine<C>]) -> Projective<C> {
    p.iter().fold(Projective::IDENTITY, |a, p| a + *p)
}
fn checked_sum<C: CurveConfig>(p: &[Affine<C>], scale: C::Base) -> (Projective<C>, bool) {
    match fast::<C>(p, scale) {
        Some((p, _, _)) => (p, true),
        None => (complete(p), false),
    }
}
fn normalize<C: CurveConfig>(p: Projective<C>, c: &mut Cost) -> Affine<C> {
    c.i += 1;
    let zi = p.z.inverse().unwrap();
    let s = square(zi, c);
    let t = mul(s, zi, c);
    Affine::new(mul(p.x, s, c), mul(p.y, t, c))
}
fn curve_checks<C: CurveConfig>(scales: &[C::Base]) -> Value {
    let mut outputs = 0;
    let mut declines = 0;
    let mut levels = 0;
    for seed in 0..4u64 {
        let start = Fr::from_canonical([
            0x74f0a9bc9812234u64.wrapping_mul(seed + 1),
            0x123456789abcde + seed,
            0,
            0,
        ])
        .unwrap();
        let step = Fr::from_u64(37 + 2 * seed);
        let dp = Affine::<C>::GENERATOR.mul(&step);
        let mut q = Affine::<C>::GENERATOR.mul(&start);
        let walk = (0..512)
            .map(|_| {
                let p = q.to_affine();
                q += dp;
                p
            })
            .collect::<Vec<_>>();
        for &n in LENGTHS.iter().filter(|&&n| n <= 512) {
            let p = &walk[..n];
            let immutable = p.to_vec();
            let scalar =
                start * Fr::from_u64(n as u64) + step * Fr::from_u64((n * (n - 1) / 2) as u64);
            let oracle = Affine::<C>::GENERATOR.mul(&scalar).to_affine();
            assert_eq!(complete(p).to_affine(), oracle);
            for &scale in scales {
                let (actual, c, d) = fast::<C>(p, scale).expect("ordinary walk must qualify");
                assert_eq!(actual.to_affine(), oracle);
                levels += d;
                let mut nc = Cost::default();
                assert_eq!(normalize(actual, &mut nc), oracle);
                assert_eq!(nc, Cost { m: 3, s: 1, i: 1 });
                if scale == C::Base::ONE {
                    assert_eq!(c, tree_formula(n).0);
                }
                assert_eq!(p, immutable);
                outputs += 1;
            }
        }
    }
    let g = Affine::<C>::GENERATOR;
    let two = g.mul_limbs(&[2]).to_affine();
    let four = g.mul_limbs(&[4]).to_affine();
    let cases = vec![
        vec![],
        vec![g],
        vec![g, g],
        vec![g, -g],
        vec![Affine::IDENTITY, g],
        vec![g, two, four, -g],
        vec![g, two, -four, g],
        vec![g, two, Affine::IDENTITY, four],
    ];
    for p in cases {
        for &scale in scales {
            assert!(fast::<C>(&p, scale).is_none());
            let (out, used) = checked_sum::<C>(&p, scale);
            assert!(!used);
            assert_eq!(out.to_affine(), complete(&p).to_affine());
            declines += 1;
        }
    }
    let p = [g, two];
    assert!(fast::<C>(&p, C::Base::ZERO).is_none());
    declines += 1;
    json!({"ordinary_complete_outputs":outputs,"instrumented_level_formula_checks":levels,"declined_complete_fallback_cases":declines,"shared_field_oracle":true})
}

// Independent prime-field/rational group oracle. No native Field, curve,
// prefix/suffix routine, or coordinate buffer from the candidate is imported.
const P: u64 = 1019;
fn md(a: u64, b: u64) -> u64 {
    (a + P - b) % P
}
fn inv(mut x: u64) -> u64 {
    assert!(x != 0);
    let mut e = P - 2;
    let mut y = 1;
    while e != 0 {
        if e & 1 != 0 {
            y = y * x % P;
        }
        x = x * x % P;
        e >>= 1;
    }
    y
}
fn rational(a: (u64, u64), b: (u64, u64)) -> Option<(u64, u64)> {
    if a.0 == b.0 {
        return None;
    }
    let l = md(b.1, a.1) * inv(md(b.0, a.0)) % P;
    let x = md(md(l * l % P, a.0), b.0);
    let y = md(l * md(a.0, x) % P, a.1);
    Some((x, y))
}
fn small_oracle() -> usize {
    let points = (0..P)
        .filter_map(|x| {
            (1..P)
                .find(|&y| y * y % P == (x * x % P * x + 7) % P)
                .map(|y| (x, y))
        })
        .take(100)
        .collect::<Vec<_>>();
    assert_eq!(points.len(), 100);
    let mut cases = 0;
    for m in 1..=16 {
        for shift in 0..4 {
            for z in [1, 3, P - 1] {
                let pairs = (0..m)
                    .map(|j| (points[shift + 2 * j], points[shift + 2 * j + 1]))
                    .collect::<Vec<_>>();
                let z2 = z * z % P;
                let z3 = z2 * z % P;
                let scaled = pairs
                    .iter()
                    .map(|&(a, b)| ((a.0 * z2 % P, a.1 * z3 % P), (b.0 * z2 % P, b.1 * z3 % P)))
                    .collect::<Vec<_>>();
                let ds = scaled
                    .iter()
                    .map(|&(a, b)| md(b.0, a.0))
                    .collect::<Vec<_>>();
                assert!(ds.iter().all(|&x| x != 0));
                let d = ds.iter().fold(1, |a, &b| a * b % P);
                let s = d * d % P;
                let t = s * d % P;
                let iz = inv(z * d % P);
                let iz2 = iz * iz % P;
                let iz3 = iz2 * iz % P;
                for j in 0..m {
                    let q = (0..m).filter(|&k| k != j).fold(1, |a, k| a * ds[k] % P);
                    let (a, b) = scaled[j];
                    let u = md(b.1, a.1) * q % P;
                    let x = md(u * u % P, (a.0 + b.0) % P * s % P);
                    let y = md(u * md(a.0 * s % P, x) % P, a.1 * t % P);
                    assert_eq!(
                        (x * iz2 % P, y * iz3 % P),
                        rational(pairs[j].0, pairs[j].1).unwrap()
                    );
                    cases += 1;
                }
                // An odd carried point must be unchanged after common rescaling.
                let carry = points[90];
                assert_eq!(
                    (
                        carry.0 * z2 % P * s % P * iz2 % P,
                        carry.1 * z3 % P * t % P * iz3 % P
                    ),
                    carry
                );
                cases += 1;
            }
        }
    }
    cases
}
fn run() {
    let mut pins = read(path("bootstrap-pins.json"))
        .as_object()
        .unwrap()
        .iter()
        .map(|(p, h)| (p.clone(), h.as_str().unwrap().to_string()))
        .collect::<BTreeMap<_, _>>();
    for (p, h) in &pins {
        assert_eq!(&hash(p), h);
    }
    let manifest = read(Path::new(Z).join(MANIFEST));
    let sources = manifest["source_files_sha256"].as_object().unwrap();
    assert_eq!(sources.len(), 176);
    for (p, h) in sources {
        let p = Path::new(Z).join(p);
        assert_eq!(hash(&p), h.as_str().unwrap());
        pins.insert(p.display().to_string(), hash(&p));
    }
    pins.insert(path("model").display().to_string(), hash(path("model")));
    save(path("pins.json"), &json!(pins));
    let small = small_oracle();
    let g1 = curve_checks::<G1Config>(&[Fq::ONE, Fq::from_u64(7), -Fq::ONE]);
    let g2 = curve_checks::<G2Config>(&[
        Fq2::ONE,
        Fq2::new(Fq::from_u64(2), Fq::from_u64(3)),
        -Fq2::ONE,
    ]);
    let rows=LENGTHS.iter().map(|&n|{
        let(c,a,depth)=tree_formula(n);let mut affine=c;affine.add(Cost{m:3,s:1,i:1});
        assert!(a.i>1||n<=3);
        json!({"points":n,"levels":depth,"projective":c.json(),"affine":affine.json(),"abstract_affine_tree":a.json(),
            "projective_break_even":{"extra_M":c.m as i64-a.m as i64,"extra_S":c.s as i64-a.s as i64,"saved_I":a.i},
            "affine_break_even":{"extra_M":affine.m as i64-a.m as i64,"extra_S":affine.s as i64-a.s as i64,"saved_I":a.i-1},
            "scratch_peak_field_elements":2*n+2*(n/2),"scratch_excludes_input_points":true})
    }).collect::<Vec<_>>();
    save(
        path("analysis.json"),
        &json!({"schema":1,"checks_pass":true,"small_prime_rational_outputs":small,"G1":g1,"G2":g2,"rows":rows,
        "new_timings":0,"new_SDK_proofs":0,"production_changed":false,"goal_achieved":false,"canonical_proof_ledger_count":31387,
        "scope":"mathematical recipe and exact abstract M/S/I counts; not AVX native qualification or latency"}),
    );
    println!(
        "Common-scale algebra PASS: small-prime outputs={small}; G1={g1}; G2={g2}; {} count rows",
        rows.len()
    );
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("run") => run(),
        _ => panic!("run"),
    }
}
