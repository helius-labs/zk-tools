use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    mem::size_of,
    path::{Path, PathBuf},
};
use zopro_bn254::{Field, Fq, Fq2, Fr, G2Affine, G2Projective};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-real-norm-z176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const LENGTHS: &[usize] = &[
    2, 3, 4, 7, 8, 15, 16, 31, 32, 63, 64, 127, 128, 255, 256, 511, 512, 1024, 4096,
];
type NativeV = zopro_bn254::ifma8::Ifma8<zopro_bn254::FqConfig, true>;
#[allow(dead_code)]
struct NativeGroup {
    ax: [NativeV; 2],
    ay: [NativeV; 2],
    xsum: [NativeV; 2],
    norm: NativeV,
    numerator: [NativeV; 2],
}
fn r(p: &str) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(p: impl AsRef<Path>, v: &Value) {
    assert!(!p.as_ref().exists());
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
}
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
struct Cost {
    vm: u64,
    vf: u64,
    sm: u64,
    ss: u64,
    sf: u64,
    i: u64,
}
impl Cost {
    fn json(self) -> Value {
        json!({"VM":self.vm,"VF":self.vf,"SM":self.sm,"SS":self.ss,"SF":self.sf,"I":self.i,"vector_IFMA_proxy":100*self.vm+150*self.vf})
    }
    fn add(&mut self, b: Self) {
        self.vm += b.vm;
        self.vf += b.vf;
        self.sm += b.sm;
        self.ss += b.ss;
        self.sf += b.sf;
        self.i += b.i;
    }
}
#[derive(Clone, Copy)]
struct V([Fq; 8]);
impl V {
    fn one() -> Self {
        Self([Fq::ONE; 8])
    }
    fn zero() -> Self {
        Self([Fq::ZERO; 8])
    }
    fn add(self, b: Self) -> Self {
        Self(std::array::from_fn(|j| self.0[j] + b.0[j]))
    }
    fn sub(self, b: Self) -> Self {
        Self(std::array::from_fn(|j| self.0[j] - b.0[j]))
    }
    fn mul(self, b: Self, c: &mut Cost) -> Self {
        c.vm += 1;
        Self(std::array::from_fn(|j| self.0[j] * b.0[j]))
    }
    fn fused(self, b: Self, d: Self, e: Self, c: &mut Cost) -> Self {
        c.vf += 1;
        Self(std::array::from_fn(|j| {
            Fq::sum_of_products(&self.0[j], &b.0[j], &d.0[j], &e.0[j])
        }))
    }
    fn shuffle(self, mask: usize) -> Self {
        Self(std::array::from_fn(|j| self.0[j ^ mask]))
    }
}
#[derive(Clone, Copy)]
struct Q([V; 2]);
impl Q {
    fn one() -> Self {
        Self([V::one(), V::zero()])
    }
    fn zero() -> Self {
        Self([V::zero(); 2])
    }
    fn new(a: [Fq2; 8]) -> Self {
        Self([V(a.map(|q| q.c0)), V(a.map(|q| q.c1))])
    }
    fn get(self) -> [Fq2; 8] {
        std::array::from_fn(|j| Fq2::new(self.0[0].0[j], self.0[1].0[j]))
    }
    fn add(self, b: Self) -> Self {
        Self([self.0[0].add(b.0[0]), self.0[1].add(b.0[1])])
    }
    fn sub(self, b: Self) -> Self {
        Self([self.0[0].sub(b.0[0]), self.0[1].sub(b.0[1])])
    }
    fn conj(self) -> Self {
        Self([self.0[0], V::zero().sub(self.0[1])])
    }
    fn mul(self, b: Self, c: &mut Cost) -> Self {
        Self([
            self.0[0].fused(b.0[0], V::zero().sub(self.0[1]), b.0[1], c),
            self.0[0].fused(b.0[1], self.0[1], b.0[0], c),
        ])
    }
    fn square(self, c: &mut Cost) -> Self {
        let p = self.0[0].mul(self.0[1], c);
        Self([
            self.0[0].add(self.0[1]).mul(self.0[0].sub(self.0[1]), c),
            p.add(p),
        ])
    }
    fn scale(self, s: V, c: &mut Cost) -> Self {
        Self([self.0[0].mul(s, c), self.0[1].mul(s, c)])
    }
    fn norm(self, c: &mut Cost) -> V {
        self.0[0].fused(self.0[0], self.0[1], self.0[1], c)
    }
    fn shuffle(self, mask: usize) -> Self {
        Self(self.0.map(|v| v.shuffle(mask)))
    }
}
trait Denom: Copy {
    const NAME: &'static str;
    fn one() -> Self;
    fn from_scale(s: Fq) -> Self;
    fn denominator(dx: Q, c: &mut Cost) -> Self;
    fn nonzero(self, lanes: usize) -> bool;
    fn mul(self, b: Self, c: &mut Cost) -> Self;
    fn square(self, c: &mut Cost) -> Self;
    fn shuffle(self, mask: usize) -> Self;
    fn numerator(dy: Q, dx: Q, q: Self, c: &mut Cost) -> Q;
    fn scale(q: Q, s: Self, c: &mut Cost) -> Q;
    fn inverse_scales(self, c: &mut Cost) -> Option<(Self, Self)>;
}
impl Denom for V {
    const NAME: &'static str = "real_norm";
    fn one() -> Self {
        V::one()
    }
    fn from_scale(s: Fq) -> Self {
        V([s; 8])
    }
    fn denominator(dx: Q, c: &mut Cost) -> Self {
        dx.norm(c)
    }
    fn nonzero(self, n: usize) -> bool {
        self.0[..n].iter().all(|x| !x.is_zero())
    }
    fn mul(self, b: Self, c: &mut Cost) -> Self {
        V::mul(self, b, c)
    }
    fn square(self, c: &mut Cost) -> Self {
        V::mul(self, self, c)
    }
    fn shuffle(self, m: usize) -> Self {
        V::shuffle(self, m)
    }
    fn numerator(dy: Q, dx: Q, q: Self, c: &mut Cost) -> Q {
        dy.mul(dx.conj(), c).scale(q, c)
    }
    fn scale(q: Q, s: Self, c: &mut Cost) -> Q {
        q.scale(s, c)
    }
    fn inverse_scales(self, c: &mut Cost) -> Option<(Self, Self)> {
        c.i += 1;
        let z = self.0[0].inverse()?;
        c.ss += 1;
        let z2 = z.square();
        c.sm += 1;
        let z3 = z2 * z;
        Some((V([z2; 8]), V([z3; 8])))
    }
}
impl Denom for Q {
    const NAME: &'static str = "complex_product";
    fn one() -> Self {
        Q::one()
    }
    fn from_scale(s: Fq) -> Self {
        Q::new([Fq2::new(s, Fq::ZERO); 8])
    }
    fn denominator(dx: Q, _: &mut Cost) -> Self {
        dx
    }
    fn nonzero(self, n: usize) -> bool {
        self.get()[..n].iter().all(|x| !x.is_zero())
    }
    fn mul(self, b: Self, c: &mut Cost) -> Self {
        Q::mul(self, b, c)
    }
    fn square(self, c: &mut Cost) -> Self {
        Q::square(self, c)
    }
    fn shuffle(self, m: usize) -> Self {
        Q::shuffle(self, m)
    }
    fn numerator(dy: Q, _: Q, q: Self, c: &mut Cost) -> Q {
        dy.mul(q, c)
    }
    fn scale(q: Q, s: Self, c: &mut Cost) -> Q {
        q.mul(s, c)
    }
    fn inverse_scales(self, c: &mut Cost) -> Option<(Self, Self)> {
        let z = self.get()[0];
        c.ss += 2;
        let norm = z.c0.square() + z.c1.square();
        c.i += 1;
        let inv = norm.inverse()?;
        c.sm += 2;
        let zi = Fq2::new(z.c0 * inv, -z.c1 * inv);
        c.sm += 2;
        let z2 = zi.square();
        c.sf += 2;
        let z3 = z2 * zi;
        Some((Q::new([z2; 8]), Q::new([z3; 8])))
    }
}
fn products<D: Denom>(a: D, c: &mut Cost) -> (D, D) {
    let r1 = a.shuffle(1);
    let p2 = a.mul(r1, c);
    let r2 = p2.shuffle(2);
    let p4 = p2.mul(r2, c);
    let r4 = p4.shuffle(4);
    (p4.mul(r4, c), r1.mul(r2, c).mul(r4, c))
}
type Point = (Fq2, Fq2);
fn pairs(counts: &[usize]) -> impl Iterator<Item = usize> + '_ {
    counts
        .iter()
        .copied()
        .scan(0, |at, n| {
            let begin = *at;
            *at += n;
            Some((0..n / 2).map(move |j| begin + 2 * j))
        })
        .flatten()
}
fn reverse_pairs(counts: &[usize]) -> impl Iterator<Item = usize> + '_ {
    let n: usize = counts.iter().sum();
    counts
        .iter()
        .copied()
        .rev()
        .scan(n, |at, n| {
            *at -= n;
            let begin = *at;
            Some((0..n / 2).rev().map(move |j| begin + 2 * j))
        })
        .flatten()
}
fn carries(counts: &[usize]) -> impl Iterator<Item = usize> + '_ {
    counts
        .iter()
        .copied()
        .scan(0, |at, n| {
            let begin = *at;
            *at += n;
            Some((n % 2 != 0).then_some(begin + n.saturating_sub(1)))
        })
        .flatten()
}
fn load(points: &[Point], idx: &[Option<usize>; 8], part: usize, right: bool) -> Q {
    Q::new(std::array::from_fn(|j| match idx[j] {
        Some(i) => {
            let p = points[i + usize::from(right)];
            if part == 0 {
                p.0
            } else {
                p.1
            }
        }
        None => {
            if right && part == 0 {
                Fq2::ONE
            } else {
                Fq2::ZERO
            }
        }
    }))
}
fn level_formula(real: bool, blocks: usize, carry_blocks: usize, first: bool) -> Cost {
    if real {
        Cost {
            vm: (13 * blocks + 7 + usize::from(!first) + 4 * carry_blocks) as u64,
            vf: (6 * blocks) as u64,
            ..Cost::default()
        }
    } else {
        Cost {
            vm: (2 * blocks + 2) as u64,
            vf: (16 * blocks + 12 + 2 * usize::from(!first) + 4 * carry_blocks) as u64,
            ..Cost::default()
        }
    }
}
struct Tree {
    points: Vec<Point>,
    cost: Cost,
    levels: Vec<Value>,
    payload: usize,
}
fn tree<D: Denom>(input: &[Point], counts: &[usize], scale: Fq) -> Option<Tree> {
    assert_eq!(counts.iter().sum::<usize>(), input.len());
    if counts.iter().all(|&n| n <= 1) {
        return Some(Tree {
            points: input.to_vec(),
            cost: Cost::default(),
            levels: Vec::new(),
            payload: input.len() * size_of::<Point>() + counts.len() * size_of::<usize>(),
        });
    }
    if scale.is_zero() || input.iter().any(|&(x, y)| x.is_zero() && y.is_zero()) {
        return None;
    }
    let s = scale.square();
    let t = s * scale;
    let mut p = input
        .iter()
        .map(|&(x, y)| (x.mul_by_fp(&s), y.mul_by_fp(&t)))
        .collect::<Vec<_>>();
    let mut c = counts.to_vec();
    let max_blocks = pairs(&c).count().div_ceil(8);
    let mut prefix = vec![D::one(); max_blocks];
    let payload = p.capacity() * size_of::<Point>()
        + prefix.capacity() * size_of::<D>()
        + c.capacity() * size_of::<usize>();
    let mut z = D::from_scale(scale);
    let mut cost = Cost::default();
    let mut levels = Vec::new();
    let mut first = true;
    while c.iter().any(|&n| n > 1) {
        let before = cost;
        let m = pairs(&c).count();
        let blocks = m.div_ceil(8);
        let carry_count = carries(&c).count();
        let mut iter = pairs(&c);
        let mut chain = D::one();
        for record in &mut prefix[..blocks] {
            let idx = std::array::from_fn(|_| iter.next());
            let dx = load(&p, &idx, 0, true).sub(load(&p, &idx, 0, false));
            let norm = D::denominator(dx, &mut cost);
            if !norm.nonzero(idx.iter().flatten().count()) {
                return None;
            }
            *record = chain;
            chain = chain.mul(norm, &mut cost);
        }
        drop(iter);
        let (d, mut right) = products(chain, &mut cost);
        let d2 = d.square(&mut cost);
        let d3 = d2.mul(d, &mut cost);
        z = if first && scale == Fq::ONE {
            d
        } else {
            z.mul(d, &mut cost)
        };
        let mut reverse = reverse_pairs(&c);
        for b in (0..blocks).rev() {
            let lanes = (m - 8 * b).min(8);
            let mut idx = [None; 8];
            for j in (0..lanes).rev() {
                idx[j] = reverse.next();
            }
            let ax = load(&p, &idx, 0, false);
            let ay = load(&p, &idx, 1, false);
            let bx = load(&p, &idx, 0, true);
            let by = load(&p, &idx, 1, true);
            let dx = bx.sub(ax);
            let norm = D::denominator(dx, &mut cost);
            let q = prefix[b].mul(right, &mut cost);
            right = right.mul(norm, &mut cost);
            let u = D::numerator(by.sub(ay), dx, q, &mut cost);
            let x = u.square(&mut cost).sub(D::scale(ax.add(bx), d2, &mut cost));
            let y = u
                .mul(D::scale(ax, d2, &mut cost).sub(x), &mut cost)
                .sub(D::scale(ay, d3, &mut cost));
            let x = x.get();
            let y = y.get();
            for j in 0..lanes {
                p[idx[j].unwrap()] = (x[j], y[j]);
            }
        }
        assert!(reverse.next().is_none());
        drop(reverse);
        let mut carry = carries(&c);
        loop {
            let idx: [Option<usize>; 8] = std::array::from_fn(|_| carry.next());
            if idx[0].is_none() {
                break;
            }
            let x = D::scale(load(&p, &idx, 0, false), d2, &mut cost).get();
            let y = D::scale(load(&p, &idx, 1, false), d3, &mut cost).get();
            for j in 0..8 {
                if let Some(i) = idx[j] {
                    p[i] = (x[j], y[j]);
                }
            }
        }
        drop(carry);
        let mut source = 0;
        let mut dest = 0;
        for n in &mut c {
            for j in 0..*n / 2 {
                p[dest + j] = p[source + 2 * j];
            }
            if *n % 2 != 0 {
                p[dest + *n / 2] = p[source + *n - 1];
            }
            source += *n;
            *n = n.div_ceil(2);
            dest += *n;
        }
        p.truncate(dest);
        let expected = level_formula(
            D::NAME == "real_norm",
            blocks,
            carry_count.div_ceil(8),
            first && scale == Fq::ONE,
        );
        let delta = Cost {
            vm: cost.vm - before.vm,
            vf: cost.vf - before.vf,
            ..Cost::default()
        };
        assert_eq!(delta, expected);
        levels.push(json!({"pairs":m,"blocks":blocks,"carried":carry_count,"remaining_points":dest,"cost":delta.json()}));
        first = false;
    }
    drop(prefix);
    let scales = D::inverse_scales(z, &mut cost)?;
    for at in (0..p.len()).step_by(8) {
        let idx = std::array::from_fn(|j| (at + j < p.len()).then_some(at + j));
        let x = D::scale(load(&p, &idx, 0, false), scales.0, &mut cost).get();
        let y = D::scale(load(&p, &idx, 1, false), scales.1, &mut cost).get();
        for j in 0..8.min(p.len() - at) {
            p[at + j] = (x[j], y[j]);
        }
    }
    Some(Tree {
        points: p,
        cost,
        levels,
        payload,
    })
}
fn reference(input: &[Point], counts: &[usize]) -> Vec<Point> {
    let mut at = 0;
    counts
        .iter()
        .filter_map(|&n| {
            let p = input[at..at + n]
                .iter()
                .fold(G2Projective::IDENTITY, |a, &(x, y)| a + G2Affine::new(x, y));
            at += n;
            if n == 0 {
                None
            } else {
                let a = p.to_affine();
                Some((a.x, a.y))
            }
        })
        .collect()
}
fn counted_shape(counts: &[usize]) -> (Cost, Cost, Cost, Value) {
    let mut c = counts.to_vec();
    let mut real = Cost::default();
    let mut complex = Cost::default();
    let mut accepted = Cost::default();
    let mut depth = 0;
    let mut max_prefix = 0;
    let mut max_groups = 0;
    let mut max_pairs = 0;
    let mut max_cofactor = 0;
    while c.iter().any(|&n| n > 1) {
        let m = c.iter().map(|n| n / 2).sum::<usize>();
        let full = m / 8;
        let tail = m % 8;
        let carry = c.iter().filter(|&&n| n % 2 != 0).count();
        real.add(level_formula(
            true,
            m.div_ceil(8),
            carry.div_ceil(8),
            depth == 0,
        ));
        complex.add(level_formula(
            false,
            m.div_ceil(8),
            carry.div_ceil(8),
            depth == 0,
        ));
        accepted.add(Cost {
            vm: ((if m >= 32 { 7 } else { 8 }) * full + 5) as u64,
            vf: (5 * full) as u64,
            sm: (7 * tail + 1) as u64,
            sf: (5 * tail) as u64,
            i: 1,
            ss: 0,
        });
        if m >= 32 {
            max_prefix = max_prefix.max(full);
        } else {
            max_groups = max_groups.max(full);
        }
        max_pairs = max_pairs.max(m);
        max_cofactor = max_cofactor.max(m.div_ceil(8));
        for n in &mut c {
            *n = n.div_ceil(2);
        }
        depth += 1;
    }
    let outputs = c.iter().filter(|&&n| n != 0).count();
    if depth != 0 {
        real.add(Cost {
            vm: (4 * outputs.div_ceil(8)) as u64,
            sm: 1,
            ss: 1,
            i: 1,
            ..Cost::default()
        });
        complex.add(Cost {
            vf: (4 * outputs.div_ceil(8)) as u64,
            sm: 4,
            ss: 2,
            sf: 2,
            i: 1,
            ..Cost::default()
        });
    }
    let n = counts.iter().sum::<usize>();
    let fields_source = 128 * n + 640 * max_prefix + 2880 * max_groups;
    let memory = json!({"point_bytes":128*n,"source_retained_prefix_payload":640*max_prefix,"source_retained_Group_payload":2880*max_groups,"source_required_retained_field_payload":fields_source,"source_pair_index_logical_bytes":size_of::<usize>()*max_pairs,"source_group_metadata_slots":256,"real_point_cofactor_field_payload":128*n+size_of::<V>()*max_cofactor,"complex_point_cofactor_field_payload":128*n+size_of::<Q>()*max_cofactor,"model_group_metadata_bytes":size_of::<usize>()*counts.len(),"Vec_headers_and_stack_excluded":true});
    (real, complex, accepted, memory)
}
fn make_walk(n: usize, seed: u64) -> Vec<Point> {
    let a = Fr::from_canonical([
        0x8cb3196d042b9a73u64.wrapping_mul(seed + 1),
        0x251ac724b17631 + seed,
        0,
        0,
    ])
    .unwrap();
    let step = G2Affine::GENERATOR.mul_limbs(&[113 + 2 * seed]);
    let mut p = G2Affine::GENERATOR.mul(&a);
    (0..n)
        .map(|_| {
            let a = p.to_affine();
            p += step;
            (a.x, a.y)
        })
        .collect()
}
fn qualify() -> Value {
    let mut complete_outputs = 0;
    let mut level_checks = 0;
    let mut declined = 0;
    let scales = [Fq::ONE, Fq::from_u64(13), -Fq::ONE];
    for seed in 0..4 {
        let p = make_walk(512, seed);
        for &n in LENGTHS.iter().filter(|&&n| n <= 512) {
            let input = &p[..n];
            let expected = reference(input, &[n]);
            let saved = input.to_vec();
            for &scale in &scales {
                let real = tree::<V>(input, &[n], scale).unwrap();
                let complex = tree::<Q>(input, &[n], scale).unwrap();
                assert_eq!(real.points, expected);
                assert_eq!(complex.points, expected);
                assert_eq!(input, saved);
                level_checks += real.levels.len() + complex.levels.len();
                complete_outputs += 2;
                if scale == Fq::ONE {
                    let (r, c, _, _) = counted_shape(&[n]);
                    assert_eq!(real.cost, r);
                    assert_eq!(complex.cost, c);
                }
            }
        }
    }
    let mut patterns = Vec::new();
    for a in 0..=8 {
        for b in 0..=8 {
            patterns.push(vec![a, 0, b, 1, 3, 0, 5]);
        }
    }
    patterns.extend([
        vec![1; 32],
        vec![0; 32],
        vec![3; 16],
        vec![7, 8, 15, 16],
        vec![4096],
    ]);
    let p = make_walk(4096, 91);
    for c in patterns {
        let n = c.iter().sum::<usize>();
        let input = &p[..n];
        let expected = reference(input, &c);
        let r = tree::<V>(input, &c, Fq::ONE).unwrap();
        let q = tree::<Q>(input, &c, Fq::ONE).unwrap();
        assert_eq!(r.points, expected);
        assert_eq!(q.points, expected);
        let (rc, qc, _, _) = counted_shape(&c);
        assert_eq!(r.cost, rc);
        assert_eq!(q.cost, qc);
        complete_outputs += 2;
        level_checks += r.levels.len() + q.levels.len();
    }
    let g = G2Affine::GENERATOR;
    let two = g.mul_limbs(&[2]).to_affine();
    let four = g.mul_limbs(&[4]).to_affine();
    for points in [
        vec![g, g],
        vec![g, -g],
        vec![G2Affine::IDENTITY, g],
        vec![g, two, four, -g],
        vec![g, two, -four, g],
        vec![g, two, G2Affine::IDENTITY, four],
    ] {
        let input = points.iter().map(|p| (p.x, p.y)).collect::<Vec<_>>();
        let saved = input.clone();
        assert!(tree::<V>(&input, &[input.len()], Fq::ONE).is_none());
        assert!(tree::<Q>(&input, &[input.len()], Fq::ONE).is_none());
        assert_eq!(input, saved);
        declined += 2;
    }
    assert!(tree::<V>(&p[..2], &[2], Fq::ZERO).is_none());
    assert!(tree::<Q>(&p[..2], &[2], Fq::ZERO).is_none());
    declined += 2;
    json!({"complete_outputs":complete_outputs,"instrumented_level_checks":level_checks,"declines":declined,"scalar_field_shared_with_complete_oracle":true})
}
include!("small_prime.rs");
fn run() {
    assert_eq!(size_of::<NativeV>(), 320);
    assert_eq!(size_of::<[NativeV; 2]>(), 640);
    assert_eq!(size_of::<NativeGroup>(), 2880);
    assert_eq!(size_of::<Point>(), 128);
    let frozen =
        read("/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175/frozen-candidate.json");
    assert!(frozen["rustflags"]
        .as_str()
        .unwrap()
        .contains("--cfg zopro_cios_cancel_carry"));
    let mut pins = read(r("continuation-pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let manifest=read(Path::new(Z).join("docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));
    let src = manifest["source_files_sha256"].as_object().unwrap();
    assert_eq!(src.len(), 176);
    for (p, h) in src {
        let p = Path::new(Z).join(p);
        assert_eq!(hash(&p), h.as_str().unwrap());
        pins[p.display().to_string()] = json!(hash(&p));
    }
    pins[r("model").display().to_string()] = json!(hash(r("model")));
    save(r("pins.json"), &pins);
    let qualified = qualify();
    let small = small_prime();
    let mut rows = Vec::new();
    let mut patterns = LENGTHS
        .iter()
        .map(|&n| (format!("single{n}"), vec![n]))
        .collect::<Vec<_>>();
    patterns.extend([
        ("multi-3x16".into(), vec![3; 16]),
        ("singletons-and3".into(), {
            let mut c = vec![1; 255];
            c.push(3);
            c
        }),
        ("mixed".into(), vec![3, 4, 0, 1, 2, 7, 8, 15, 16]),
    ]);
    for (name, counts) in patterns {
        let (real, complex, accepted, memory) = counted_shape(&counts);
        rows.push(json!({"name":name,"counts":counts,"real_norm":real.json(),"generic_complex":complex.json(),"accepted_source":accepted.json(),"payload":memory,"vector_IFMA_proxy_real_over_complex":(100*real.vm+150*real.vf)as f64/(100*complex.vm+150*complex.vf)as f64,"vector_IFMA_proxy_real_over_source":(100*real.vm+150*real.vf)as f64/(100*accepted.vm+150*accepted.vf)as f64}));
    }
    save(
        r("analysis.json"),
        &json!({"algebra_passed":true,"qualification":qualified,"small_prime":small,"rows":rows,"sizes":{"Fq":size_of::<Fq>(),"Fq2":size_of::<Fq2>(),"point":size_of::<Point>(),"model_real_record":size_of::<V>(),"model_complex_record":size_of::<Q>()},"new_timings":0,"new_proofs":0,"production_changed":false,"goal_achieved":false,"canonical_ledger_count":31387,"scope":"eight-lane arithmetic interpreter, source-derived ordinary-pair counter and typed payloads; no IFMA kernel or latency"}),
    );
    println!("PASS real/complex common-scale algebra; qualification={qualified}; small={small};22 count rows; no native candidate/timings/proofs");
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("run") => run(),
        _ => panic!("run"),
    }
}
