use ark_bn254::{Fq, Fq2, Fr, G1Affine, G2Affine};
use ark_ec::{
    short_weierstrass::{Affine, SWCurveConfig},
    AffineRepr, CurveGroup,
};
use ark_ff::{Field, One, PrimeField, Zero};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/stream-work-census176";
const SIZES: [usize; 14] = [0, 1, 7, 8, 9, 15, 16, 199, 200, 201, 351, 352, 353, 511];
type V = [Fq; 8];
type V2 = [V; 2];
#[derive(Default, Debug, PartialEq, Eq)]
struct Count {
    vm: usize,
    vf: usize,
    sm: usize,
    ss: usize,
    sf: usize,
    si: usize,
    packs: usize,
    exports: usize,
}
impl Count {
    fn json(&self) -> Value {
        json!({"vector_mul":self.vm,"vector_fused2":self.vf,"scalar_mul":self.sm,"scalar_square":self.ss,"scalar_fused2":self.sf,"scalar_inverse":self.si,"coordinate_vector_packs":self.packs,"coordinate_vector_exports":self.exports,"vector_IFMA_source_proxy":100*self.vm+150*self.vf})
    }
}
fn mul(a: V, b: V, c: &mut Count) -> V {
    c.vm += 1;
    std::array::from_fn(|i| a[i] * b[i])
}
fn fused(a: V, b: V, d: V, e: V, negative: bool, c: &mut Count) -> V {
    c.vf += 1;
    std::array::from_fn(|i| {
        if negative {
            a[i] * b[i] - d[i] * e[i]
        } else {
            a[i] * b[i] + d[i] * e[i]
        }
    })
}
fn add(a: V, b: V) -> V {
    std::array::from_fn(|i| a[i] + b[i])
}
fn sub(a: V, b: V) -> V {
    std::array::from_fn(|i| a[i] - b[i])
}
fn sm(a: Fq, b: Fq, c: &mut Count) -> Fq {
    c.sm += 1;
    a * b
}
fn ss(a: Fq, c: &mut Count) -> Fq {
    c.ss += 1;
    a.square()
}
fn sf(a: Fq, b: Fq, d: Fq, e: Fq, negative: bool, c: &mut Count) -> Fq {
    c.sf += 1;
    if negative {
        a * b - d * e
    } else {
        a * b + d * e
    }
}
fn inverse8(a: V, c: &mut Count) -> V {
    let pairs = mul(a, std::array::from_fn(|i| a[i ^ 1]), c);
    let quads = mul(pairs, std::array::from_fn(|i| pairs[i ^ 2]), c);
    let root = sm(quads[0], quads[4], c);
    c.si += 1;
    let inv = root.inverse().expect("ordinary nonzero bridge");
    let v = mul([inv; 8], std::array::from_fn(|i| quads[i ^ 4]), c);
    let v = mul(v, std::array::from_fn(|i| pairs[i ^ 2]), c);
    mul(v, std::array::from_fn(|i| a[i ^ 1]), c)
}
fn cmul(a: V2, b: V2, c: &mut Count) -> V2 {
    [
        fused(a[0], b[0], a[1], b[1], true, c),
        fused(a[0], b[1], a[1], b[0], false, c),
    ]
}
fn cscale(a: V2, b: V, c: &mut Count) -> V2 {
    [mul(a[0], b, c), mul(a[1], b, c)]
}
fn csquare(a: V2, c: &mut Count) -> V2 {
    let p = mul(a[0], a[1], c);
    [mul(add(a[0], a[1]), sub(a[0], a[1]), c), add(p, p)]
}
fn cadd(a: V2, b: V2) -> V2 {
    [add(a[0], b[0]), add(a[1], b[1])]
}
fn csub(a: V2, b: V2) -> V2 {
    [sub(a[0], b[0]), sub(a[1], b[1])]
}
fn sm2(a: Fq2, b: Fq2, c: &mut Count) -> Fq2 {
    Fq2::new(
        sf(a.c0, b.c0, a.c1, b.c1, true, c),
        sf(a.c0, b.c1, a.c1, b.c0, false, c),
    )
}
fn scale2(a: Fq2, b: Fq, c: &mut Count) -> Fq2 {
    Fq2::new(sm(a.c0, b, c), sm(a.c1, b, c))
}
fn square2(a: Fq2, c: &mut Count) -> Fq2 {
    let p = sm(a.c0, a.c1, c);
    Fq2::new(sm(a.c0 + a.c1, a.c0 - a.c1, c), p + p)
}
fn signed(i: usize, mode: usize) -> bool {
    mode == 1 || mode == 2 && i % 3 == 0
}
fn g1(a: &mut [G1Affine], b: &[G1Affine], mode: usize, c: &mut Count) {
    let full = a.len() / 8 * 8;
    let mut chain = [Fq::one(); 8];
    let mut records = vec![];
    for k in (0..full).step_by(8) {
        let ax = std::array::from_fn(|i| a[k + i].x);
        let ay = std::array::from_fn(|i| a[k + i].y);
        let bx = std::array::from_fn(|i| b[k + i].x);
        let by = std::array::from_fn(|i| {
            if signed(k + i, mode) {
                -b[k + i].y
            } else {
                b[k + i].y
            }
        });
        c.packs += 4;
        let dx = sub(bx, ax);
        let numerator = mul(sub(by, ay), chain, c);
        chain = mul(chain, dx, c);
        records.push([ax, ay, add(ax, bx), dx, numerator]);
    }
    let mut tail = vec![];
    for k in full..a.len() {
        let ax = a[k].x;
        let ay = a[k].y;
        let by = if signed(k, mode) { -b[k].y } else { b[k].y };
        let dx = b[k].x - ax;
        assert!(!dx.is_zero());
        let numerator = sm(by - ay, chain[k % 8], c);
        chain[k % 8] = sm(chain[k % 8], dx, c);
        tail.push([ax, ay, ax + b[k].x, dx, numerator]);
    }
    let mut inv = inverse8(chain, c);
    for k in (full..a.len()).rev() {
        let [ax, ay, xsum, dx, num] = tail[k - full];
        let slope = sm(num, inv[k % 8], c);
        inv[k % 8] = sm(inv[k % 8], dx, c);
        let x = ss(slope, c) - xsum;
        let y = sm(slope, ax - x, c) - ay;
        a[k] = G1Affine::new_unchecked(x, y);
    }
    for (j, g) in records.into_iter().enumerate().rev() {
        let slope = mul(g[4], inv, c);
        inv = mul(inv, g[3], c);
        let x = sub(mul(slope, slope, c), g[2]);
        let y = sub(mul(slope, sub(g[0], x), c), g[1]);
        c.exports += 2;
        for i in 0..8 {
            a[j * 8 + i] = G1Affine::new_unchecked(x[i], y[i]);
        }
    }
}
fn pack2(points: impl Fn(usize) -> Fq2, c: &mut Count) -> V2 {
    c.packs += 2;
    let vals = std::array::from_fn::<_, 8, _>(points);
    [vals.map(|v| v.c0), vals.map(|v| v.c1)]
}
fn g2(a: &mut [G2Affine], b: &[G2Affine], mode: usize, c: &mut Count) {
    let full = a.len() / 8 * 8;
    let mut chain = [Fq::one(); 8];
    let mut records = vec![];
    for k in (0..full).step_by(8) {
        let ax = pack2(|i| a[k + i].x, c);
        let bx = pack2(|i| b[k + i].x, c);
        let d = csub(bx, ax);
        let norm = fused(d[0], d[0], d[1], d[1], false, c);
        records.push([chain, norm]);
        chain = mul(chain, norm, c);
    }
    let mut tail = vec![];
    for k in full..a.len() {
        let ax = a[k].x;
        let ay = a[k].y;
        let by = if signed(k, mode) { -b[k].y } else { b[k].y };
        let dx = b[k].x - ax;
        assert!(!dx.is_zero());
        let norm = ss(dx.c0, c) + ss(dx.c1, c);
        let conj = Fq2::new(dx.c0, -dx.c1);
        let num = scale2(sm2(by - ay, conj, c), chain[k % 8], c);
        chain[k % 8] = sm(chain[k % 8], norm, c);
        tail.push((ax, ay, ax + b[k].x, norm, num));
    }
    let mut inv = inverse8(chain, c);
    for k in (full..a.len()).rev() {
        let (ax, ay, xsum, norm, num) = tail[k - full];
        let slope = scale2(num, inv[k % 8], c);
        inv[k % 8] = sm(inv[k % 8], norm, c);
        let x = square2(slope, c) - xsum;
        let y = sm2(slope, ax - x, c) - ay;
        a[k] = G2Affine::new_unchecked(x, y);
    }
    for j in (0..full / 8).rev() {
        let k = j * 8;
        let ax = pack2(|i| a[k + i].x, c);
        let ay = pack2(|i| a[k + i].y, c);
        let bx = pack2(|i| b[k + i].x, c);
        let by = pack2(
            |i| {
                if signed(k + i, mode) {
                    -b[k + i].y
                } else {
                    b[k + i].y
                }
            },
            c,
        );
        let dy = csub(by, ay);
        let dx = csub(bx, ax);
        let factor = mul(records[j][0], inv, c);
        inv = mul(inv, records[j][1], c);
        let slope = cscale(
            [
                fused(dy[0], dx[0], dy[1], dx[1], false, c),
                fused(dy[1], dx[0], dy[0], dx[1], true, c),
            ],
            factor,
            c,
        );
        let x = csub(csquare(slope, c), cadd(ax, bx));
        let y = csub(cmul(slope, csub(ax, x), c), ay);
        c.exports += 4;
        for i in 0..8 {
            a[k + i] =
                G2Affine::new_unchecked(Fq2::new(x[0][i], x[1][i]), Fq2::new(y[0][i], y[1][i]));
        }
    }
}
fn seed(label: &str) -> Fr {
    Fr::from_le_bytes_mod_order(&Sha256::digest(label.as_bytes()))
}
fn points<C: SWCurveConfig<ScalarField = Fr>>(
    n: usize,
    label: &str,
) -> (Vec<Affine<C>>, Vec<Affine<C>>) {
    let g = Affine::<C>::generator();
    let mut a = g.mul_bigint(seed(&format!("{label}/a")).into_bigint());
    let mut b = g.mul_bigint(seed(&format!("{label}/b")).into_bigint());
    let da = g.mul_bigint(seed(&format!("{label}/da")).into_bigint());
    let db = g.mul_bigint(seed(&format!("{label}/db")).into_bigint());
    let mut av = vec![];
    let mut bv = vec![];
    for _ in 0..n {
        av.push(a);
        bv.push(b);
        a += da;
        b += db;
    }
    (
        CurveGroup::normalize_batch(&av),
        CurveGroup::normalize_batch(&bv),
    )
}
fn expected<C: SWCurveConfig>(a: &[Affine<C>], b: &[Affine<C>], mode: usize) -> Vec<Affine<C>> {
    let v = a
        .iter()
        .zip(b)
        .enumerate()
        .map(|(i, (a, b))| {
            a.into_group()
                + if signed(i, mode) {
                    -b.into_group()
                } else {
                    b.into_group()
                }
        })
        .collect::<Vec<_>>();
    CurveGroup::normalize_batch(&v)
}
fn formula(kind: &str, n: usize) -> Count {
    let g = n / 8;
    let t = n % 8;
    if kind == "G1" {
        Count {
            vm: 6 * g + 5,
            sm: 5 * t + 1,
            ss: t,
            si: 1,
            packs: 4 * g,
            exports: 2 * g,
            ..Default::default()
        }
    } else {
        Count {
            vm: 7 * g + 5,
            vf: 5 * g,
            sm: 8 * t + 1,
            sf: 4 * t,
            ss: 2 * t,
            si: 1,
            packs: 12 * g,
            exports: 4 * g,
        }
    }
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    assert!(!Path::new(R).join("result.json").exists());
    let mut cases = vec![];
    let mut checked = 0;
    let mut aggregate = BTreeMap::new();
    for n in SIZES {
        for mode in 0..3 {
            let label = format!("stream-census/{n}/{mode}");
            let (mut a, b) = points::<ark_bn254::g1::Config>(n, &label);
            for rep in 0..2 {
                let oracle = expected(&a, &b, mode);
                assert!(a
                    .iter()
                    .zip(&b)
                    .all(|(a, b)| !a.infinity && !b.infinity && a.x != b.x));
                let mut c = Count::default();
                g1(&mut a, &b, mode, &mut c);
                assert_eq!(a, oracle);
                assert_eq!(c, formula("G1", n));
                checked += n;
                cases.push(json!({"kind":"G1","size":n,"sign_mode":mode,"reuse":rep,"point_outputs_checked":n,"counts":c.json()}));
                aggregate.insert(format!("G1-{n}"), c.json());
            }
            let (mut a, b) = points::<ark_bn254::g2::Config>(n, &label);
            for rep in 0..2 {
                let oracle = expected(&a, &b, mode);
                assert!(a
                    .iter()
                    .zip(&b)
                    .all(|(a, b)| !a.infinity && !b.infinity && a.x != b.x));
                let mut c = Count::default();
                g2(&mut a, &b, mode, &mut c);
                assert_eq!(a, oracle);
                assert_eq!(c, formula("G2", n));
                checked += n;
                cases.push(json!({"kind":"G2","size":n,"sign_mode":mode,"reuse":rep,"point_outputs_checked":n,"counts":c.json()}));
                aggregate.insert(format!("G2-{n}"), c.json());
            }
        }
    }
    assert_eq!(cases.len(), 168);
    assert_eq!(checked, 26676);
    let guard1 = guard::<ark_bn254::g1::Config>();
    let guard2 = guard::<ark_bn254::g2::Config>();
    let source = Path::new("/home/ubuntu/src/zk-tools/zopro");
    let manifest:Value=serde_json::from_slice(&fs::read("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json").unwrap()).unwrap();
    let mut sources = BTreeMap::new();
    for rel in [
        "crates/zopro-core/src/msm.rs",
        "crates/zopro-core/src/msm/g2_borrowed.rs",
        "crates/zopro-core/src/simd.rs",
        "crates/zopro-bn254/src/ifma8.rs",
        "crates/zopro-bn254/src/fq2.rs",
    ] {
        assert_eq!(hash(source.join(rel)), manifest["source_files_sha256"][rel]);
        sources.insert(rel, hash(source.join(rel)));
    }
    let callers: Value = serde_json::from_slice(
        &fs::read("/home/ubuntu/src/zopro-perf-work/inverse-root-callers176/analysis.json")
            .unwrap(),
    )
    .unwrap();
    let mut capacity_checks = 0;
    for row in callers["rows"].as_array().unwrap() {
        let name = row["context"].as_str().unwrap();
        let p = name.split('-').collect::<Vec<_>>();
        let kind = p[0];
        let n = p[1].parse::<usize>().unwrap();
        let record_bytes = n / 8 * 320 * if kind == "G1" { 5 } else { 2 };
        let tail = if n % 8 == 0 {
            0
        } else if kind == "G1" {
            640
        } else {
            1280
        };
        for arm in ["frozen", "OFF", "ON"] {
            assert_eq!(
                row["scratch_capacity_bytes_first_worker"][arm],
                json!([record_bytes, tail])
            );
            capacity_checks += 1;
        }
    }
    assert_eq!(capacity_checks, 72);
    let output = json!({"complete":true,"source_bound":sources,"cases":cases,"source_operation_counts":aggregate,"point_outputs_checked":checked,"batch_executions":168,"guard_classification_checks":guard1+guard2,"inherited_capacity_rows_checked":capacity_checks,"capacity_observations_rerun":false,"native_kernel_executed":false,"performance_claim":false,"production_changed":false,"new_proofs":0,"proof_ledger":30846,"binary_sha256":hash(std::env::current_exe().unwrap())});
    fs::write(
        Path::new(R).join("result.json"),
        serde_json::to_vec_pretty(&output).unwrap(),
    )
    .unwrap();
    println!("PASS168batch executions/26676Arkworks point-output checks/{}guard classifications/72inheritedcapacitychecks; source work-count only",guard1+guard2);
}
fn guard<C: SWCurveConfig>() -> usize {
    let p = Affine::<C>::generator();
    let zero = Affine::<C>::identity();
    assert!(zero.infinity);
    assert_eq!((p + zero).into_affine(), p);
    assert_eq!((zero + p).into_affine(), p);
    assert_eq!(p.x, (-p).x);
    assert_eq!((p + (-p)).into_affine(), zero);
    assert_eq!((p + p).into_affine(), p.mul_bigint([2u64]).into_affine());
    4
}
