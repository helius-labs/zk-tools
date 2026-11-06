use num_bigint::{BigInt, BigUint};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/split-redc-planes176";
const B: u128 = 1u128 << 52;
const MASK: u128 = B - 1;
fn u(x: u128) -> BigUint {
    BigUint::from(x)
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn inv(a: &BigInt, m: &BigInt) -> BigInt {
    let (mut old, mut rem) = (m.clone(), a.clone());
    let (mut x, mut y) = (BigInt::from(0), BigInt::from(1));
    while rem != BigInt::from(0) {
        let q = &old / &rem;
        (old, rem) = (rem.clone(), old - &q * &rem);
        (x, y) = (y.clone(), x - q * &y);
    }
    assert_eq!(old, BigInt::from(1));
    ((x % m) + m) % m
}
fn digit(x: &BigUint) -> [u128; 5] {
    let mut n = x.clone();
    std::array::from_fn(|_| {
        let d = (&n & u(MASK)).to_u64_digits().first().copied().unwrap_or(0) as u128;
        n >>= 52;
        d
    })
}
fn number(t: &[u128]) -> BigUint {
    t.iter().rev().fold(u(0), |n, &v| (n << 52) + u(v))
}
struct Field {
    p: BigUint,
    two: BigUint,
    pd: [u128; 5],
    r: BigUint,
    negative: BigUint,
    ri: BigUint,
    inv52: u128,
    bits: usize,
}
impl Field {
    fn new(p: BigUint, bits: usize) -> Self {
        let r = u(1) << bits;
        assert!(&p * u(4) < r);
        let neg = ((BigInt::from(r.clone())
            - inv(&BigInt::from(p.clone()), &BigInt::from(r.clone())))
            % BigInt::from(r.clone()))
        .to_biguint()
        .unwrap();
        let pd = digit(&p);
        let mut v = 1u64;
        for _ in 0..6 {
            v = v.wrapping_mul(2u64.wrapping_sub((pd[0] as u64).wrapping_mul(v)));
        }
        let inv52 = (v.wrapping_neg() as u128) & MASK;
        assert_eq!((inv52 * pd[0]) & MASK, MASK);
        let ri = (&r % &p).modpow(&(&p - u(2)), &p);
        assert_eq!((&ri * &r) % &p, u(1));
        Self {
            two: &p * u(2),
            pd,
            r,
            negative: neg,
            ri,
            p,
            inv52,
            bits,
        }
    }
}
#[derive(Default)]
struct Stats {
    outputs: u64,
    rows: u64,
    max_plane: u128,
    max_sum: u128,
    max_frontier: u128,
    max_finish: u128,
    endpoint_inputs: u64,
    corrections: u64,
    digest: Sha256,
}
impl Stats {
    fn check(&mut self, x: u128, finish: bool) {
        assert!(x < 64 * B && x < u64::MAX as u128);
        if finish {
            self.max_finish = self.max_finish.max(x);
        } else {
            self.max_plane = self.max_plane.max(x);
        }
    }
}
fn product(t: &mut [u128; 6], a: [u128; 5], b: u128, s: &mut Stats) {
    for j in 0..5 {
        let z = a[j] * b;
        assert!(a[j] < B && b < B);
        t[j] += z & MASK;
        t[j + 1] += z >> 52;
        s.check(t[j], false);
        s.check(t[j + 1], false);
    }
}
fn reduce_q(q: &mut [u128; 6], frontier: u128, m: u128, p: [u128; 5], bits: usize, s: &mut Stats) {
    let div = 1u128 << bits;
    assert!((frontier + m * p[0]) % div == 0);
    let carry = (frontier + div - 1) / div;
    let low = carry + (m * p[0] >> bits);
    assert_eq!(low, (frontier + m * p[0]) / div);
    q[1] += low;
    for j in 1..5 {
        let z = m * p[j];
        q[j] += z & MASK;
        q[j + 1] += z >> 52;
    }
    for &x in q.iter() {
        s.check(x, true);
    }
}
fn model(
    f: &Field,
    a: &BigUint,
    b: &BigUint,
    c: &BigUint,
    d: &BigUint,
    op: usize,
    s: &mut Stats,
) -> BigUint {
    for x in [a, b, c, d] {
        assert!(x <= &f.two);
        s.endpoint_inputs += (x == &u(0) || x == &f.two) as u64;
    }
    let cc = if op == 2 { &f.two - c } else { c.clone() };
    let (ad, bd, cd, dd) = (digit(a), digit(b), digit(&cc), digit(d));
    let raw = a * b + if op == 0 { u(0) } else { &cc * d };
    let mut reference = u(0);
    let mut pp = [0u128; 6];
    let mut qq = [0u128; 6];
    let mut cumulative_q = u(0);
    for row in 0..4 {
        product(&mut pp, ad, bd[row], s);
        if op != 0 {
            product(&mut pp, cd, dd[row], s);
        }
        reference += a * u(bd[row]);
        if op != 0 {
            reference += &cc * u(dd[row]);
        }
        assert_eq!(number(&pp) + number(&qq), reference);
        let frontier = pp[0] + qq[0];
        s.max_frontier = s.max_frontier.max(frontier);
        s.max_sum = s
            .max_sum
            .max(pp.iter().zip(qq).map(|(a, b)| a + b).max().unwrap());
        assert!(frontier < 64 * B);
        let m = (frontier * f.inv52) & MASK;
        let qr = ((&reference & u(MASK)) * u(f.inv52)) & u(MASK);
        assert_eq!(u(m), qr);
        cumulative_q += &qr << (52 * row);
        reduce_q(&mut qq, frontier, m, f.pd, 52, s);
        for j in 0..5 {
            pp[j] = pp[j + 1];
            qq[j] = qq[j + 1];
        }
        pp[5] = 0;
        qq[5] = 0;
        reference = (reference + &qr * &f.p) >> 52;
        assert_eq!(number(&pp) + number(&qq), reference);
        for j in 0..6 {
            s.check(pp[j] + qq[j], false);
        }
        s.rows += 1;
    }
    let mut t = std::array::from_fn::<_, 6, _>(|j| pp[j] + qq[j]);
    assert_eq!(number(&t), reference);
    product(&mut t, ad, bd[4], s);
    if op != 0 {
        product(&mut t, cd, dd[4], s);
    }
    reference += a * u(bd[4]);
    if op != 0 {
        reference += &cc * u(dd[4]);
    }
    assert_eq!(number(&t), reference);
    let bits = f.bits - 208;
    let mask = (1u128 << bits) - 1;
    let m = (t[0] * f.inv52) & mask;
    let qr = ((&reference & u(mask)) * u(f.inv52)) & u(mask);
    assert_eq!(u(m), qr);
    cumulative_q += &qr << 208;
    assert_eq!(cumulative_q, (&raw * &f.negative) % &f.r);
    let frontier = t[0];
    reduce_q(&mut t, frontier, m, f.pd, bits, s);
    let mut out = if bits == 48 {
        [t[1] << 4, 0, 0, 0, 0]
    } else {
        [t[1], t[2], t[3], t[4], t[5]]
    };
    if bits == 48 {
        // t[1] already includes the complete cancelled-low quotient. Separate its
        // pre-carry contribution from the higher radix52 coefficient before packing.
        let low = (frontier + (1u128 << 48) - 1) / (1u128 << 48) + (m * f.pd[0] >> 48);
        let higher = t[1] - low;
        out[0] = low + ((higher & mask) << 4);
        out[1] = (higher >> 48) + ((t[2] & mask) << 4);
        out[2] = (t[2] >> 48) + ((t[3] & mask) << 4);
        out[3] = (t[3] >> 48) + ((t[4] & mask) << 4);
        out[4] = (t[4] >> 48) + (t[5] << 4);
    }
    for j in 0..4 {
        out[j + 1] += out[j] >> 52;
        out[j] &= MASK;
    }
    assert!(out.iter().all(|v| *v < B));
    let value = number(&out);
    let raw_redc = (&raw + &cumulative_q * &f.p) / &f.r;
    assert_eq!(value, raw_redc);
    assert_eq!(value, (reference + &qr * &f.p) >> bits);
    s.rows += 1;
    let value = if op != 0 && bits == 48 && value >= f.two {
        s.corrections += 1;
        value - &f.two
    } else {
        value
    };
    assert!(value < f.two);
    let oracle = (&raw * &f.ri) % &f.p;
    assert_eq!(&value % &f.p, oracle);
    let algebra = if op == 0 {
        a * b
    } else if op == 1 {
        a * b + c * d
    } else {
        a * b + &f.p * (&f.two * d + u(1)) - c * d
    };
    assert_eq!((&algebra * &f.ri) % &f.p, oracle);
    assert_eq!(number(&digit(&value)), value);
    s.outputs += 1;
    s.digest.update(value.to_bytes_le());
    value
}
fn next(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}
fn random(f: &Field, seed: &mut u64) -> BigUint {
    let mut n = u(0);
    for _ in 0..4 {
        n = (n << 64) + u(next(seed) as u128);
    }
    n % (&f.two + u(1))
}
#[derive(Clone, Copy)]
struct Node {
    depth: u64,
    nonzero: bool,
}
fn zero() -> Node {
    Node {
        depth: 0,
        nonzero: false,
    }
}
fn add(a: Node, b: Node) -> Node {
    if !a.nonzero {
        return b;
    }
    if !b.nonzero {
        return a;
    }
    Node {
        depth: a.depth.max(b.depth) + 2,
        nonzero: true,
    }
}
fn fma(a: Node, b: Node, c: Node) -> Node {
    Node {
        depth: a.depth.max(b.depth).max(c.depth) + 4,
        nonzero: true,
    }
}
fn work(fused: bool, split: bool) -> Value {
    let ready = Node {
        depth: 0,
        nonzero: true,
    };
    let mut t = [zero(); 6];
    let mut p = [zero(); 6];
    let mut q = [zero(); 6];
    let mut depths = vec![];
    let mut ifmas = 0;
    for _ in 0..4 {
        let a = if split { &mut p } else { &mut t };
        for j in 0..5 {
            a[j] = fma(a[j], ready, ready);
            a[j + 1] = fma(a[j + 1], ready, ready);
            ifmas += 2;
        }
        if fused {
            for j in 0..5 {
                a[j] = fma(a[j], ready, ready);
                a[j + 1] = fma(a[j + 1], ready, ready);
                ifmas += 2;
            }
        }
        let z = if split { add(p[0], q[0]) } else { t[0] };
        let m = fma(zero(), z, ready);
        ifmas += 1;
        depths.push(m.depth);
        let carry = Node {
            depth: z.depth + 4,
            nonzero: true,
        };
        let low = fma(carry, m, ready);
        ifmas += 1;
        let a = if split { &mut q } else { &mut t };
        for j in 1..5 {
            a[j] = fma(a[j], m, ready);
            a[j + 1] = fma(a[j + 1], m, ready);
            ifmas += 2;
        }
        a[1] = add(a[1], low);
        if split {
            for j in 0..5 {
                p[j] = p[j + 1];
                q[j] = q[j + 1];
            }
            p[5] = zero();
            q[5] = zero();
        } else {
            for j in 0..5 {
                t[j] = t[j + 1];
            }
            t[5] = zero();
        }
    }
    if split {
        for j in 0..6 {
            t[j] = add(p[j], q[j]);
        }
    }
    json!({"fused":fused,"split_planes":split,"first_four_rows_ifma":ifmas,"quotient_depths":depths,"merged_coefficient_depths":t[..5].iter().map(|n|n.depth).collect::<Vec<_>>(),"next_row_frontier_ready_depth":t[0].depth,"max_merged_depth":t[..5].iter().map(|n|n.depth).max().unwrap(),"weights":"IFMA4,add2,right52shift2; zero inputs/constant-folded sums excluded. Final common row/normalization not scored; no spills/allocation model"})
}
fn main() {
    assert!(!Path::new(R).join("model.json").exists());
    let mut exhaustive = 0u64;
    for bits in 2..=10 {
        let b = 1u128 << bits;
        for p0 in (1..b).step_by(2) {
            let mut iv = 1u128;
            while (p0 * iv) & (b - 1) != b - 1 {
                iv += 1;
            }
            for z in 0..4 * b {
                let m = (z * iv) & (b - 1);
                assert_eq!((z + m * p0) % b, 0);
                assert_eq!((z + b - 1) / b + (m * p0 >> bits), (z + m * p0) >> bits);
                exhaustive += 1;
            }
        }
    }
    let primes = [
        (
            "Fq",
            "30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47",
        ),
        (
            "Fr",
            "30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001",
        ),
    ];
    let mut s = Stats::default();
    let mut contexts = vec![];
    let mut seed = 0x9824f3b771ca656du64;
    for (name, hex) in primes {
        for bits in [256, 260] {
            let f = Field::new(BigUint::parse_bytes(hex.as_bytes(), 16).unwrap(), bits);
            let edge = [
                u(0),
                u(1),
                &f.p - u(1),
                f.p.clone(),
                &f.p + u(1),
                &f.two - u(1),
                f.two.clone(),
            ];
            for op in 0..3 {
                let before = s.outputs;
                for a in &edge {
                    for b in &edge {
                        if op == 0 {
                            model(&f, a, b, &u(0), &u(0), op, &mut s);
                        } else {
                            for c in &edge {
                                for d in &edge {
                                    model(&f, a, b, c, d, op, &mut s);
                                }
                            }
                        }
                    }
                }
                for _ in 0..4096 {
                    let a = random(&f, &mut seed);
                    let b = random(&f, &mut seed);
                    let c = random(&f, &mut seed);
                    let d = random(&f, &mut seed);
                    model(&f, &a, &b, &c, &d, op, &mut s);
                }
                let mut a = f.two.clone();
                for _ in 0..1024 {
                    let b = random(&f, &mut seed);
                    let c = random(&f, &mut seed);
                    let d = random(&f, &mut seed);
                    a = model(&f, &a, &b, &c, &d, op, &mut s);
                }
                let count = s.outputs - before;
                assert_eq!(count, if op == 0 { 5169 } else { 7521 });
                contexts.push(json!({"field":name,"radix_bits":bits,"operation":(["ordinary","sum2","difference2_original_complement"][op]),"outputs":count}));
            }
        }
    }
    assert_eq!(s.outputs, 80844);
    assert_eq!(s.rows, 404220);
    let mut dag = vec![];
    let mut passed = true;
    for fused in [false, true] {
        let off = work(fused, false);
        let on = work(fused, true);
        assert_eq!(off["first_four_rows_ifma"], on["first_four_rows_ifma"]);
        passed &= on["quotient_depths"][3].as_u64().unwrap()
            < off["quotient_depths"][3].as_u64().unwrap();
        dag.extend([off, on]);
    }
    let result = json!({"status":"pass","mathematical_screen":"pass","necessary_dependency_work_screen_passed":passed,"native_qualification_started":false,"contexts":contexts,"exact_reduced_outputs":s.outputs,"exact_row_states":s.rows,"reduced_radix_cancellation_checks":exhaustive,"coefficient_bound":format!("strictly below 64B = 2^58 = {}",64*B),"max_plane":s.max_plane.to_string(),"max_sum":s.max_sum.to_string(),"max_frontier":s.max_frontier.to_string(),"max_finish":s.max_finish.to_string(),"endpoint_inputs":s.endpoint_inputs,"mixed_fused_subtract2p":s.corrections,"digest":format!("{:x}",s.digest.finalize()),"dependency_dag":dag,"new_proofs":0,"proof_ledger":30846,"production_changed":false,"limits":"Exact finite-model/range/work evidence; no native instruction count, register allocation, spill, caller/proof latency or resource qualification. No formal proof or complete hardware schedule model."});
    fs::write(
        format!("{R}/model.json"),
        serde_json::to_vec_pretty(&result).unwrap(),
    )
    .unwrap();
    let files = ["model.rs", "model", "design.md"];
    let pins = files
        .into_iter()
        .map(|f| (format!("{R}/{f}"), hash(format!("{R}/{f}"))))
        .collect::<std::collections::BTreeMap<_, _>>();
    fs::write(
        format!("{R}/pins.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    println!(
        "PASS {} outputs/{} exact row states; dependency screen={passed}",
        s.outputs, s.rows
    );
}
