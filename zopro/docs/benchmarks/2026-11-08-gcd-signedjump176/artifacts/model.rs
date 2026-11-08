use num_bigint::{BigInt, BigUint, Sign};
use serde_json::{json, Value};
use std::{collections::BTreeMap, fs};
use zopro_bn254::{Field, Fp, FpConfig, FqConfig, FrConfig};
const R: &str = "/home/ubuntu/src/zopro-perf-work/gcd-signedjump176";
mod accepted_matrix {
    use core::hint::select_unpredictable;
    const BITS: usize = 31;
    include!("accepted-matrix.rs.inc");
}
#[derive(Clone, Copy, Debug, PartialEq)]
enum Mode {
    Old,
    Jump,
}
impl Mode {
    fn bits(self) -> usize {
        if self == Self::Old {
            31
        } else {
            30
        }
    }
}
#[derive(Default, Debug)]
struct Counts {
    jumps: u64,
    rounds: u64,
    integer_rows: u64,
    modular_rows: u64,
    divs: u64,
    quotient_multiplies: u64,
    hist: BTreeMap<i64, u64>,
    max_norm: u64,
}
impl Counts {
    fn value(&self) -> Value {
        json!({"small_matrix_jumps":self.jumps,"outer_rounds":self.rounds,"full_integer_rows":self.integer_rows,"modular_coefficient_rows":self.modular_rows,"finish_Euclid_quotients":self.divs,"additional_source_quotient_multiply_expressions":self.quotient_multiplies,"quotient_histogram":self.hist,"max_observed_row_L1":self.max_norm})
    }
    fn add(&mut self, o: &Self) {
        self.jumps += o.jumps;
        self.rounds += o.rounds;
        self.integer_rows += o.integer_rows;
        self.modular_rows += o.modular_rows;
        self.divs += o.divs;
        self.quotient_multiplies += o.quotient_multiplies;
        self.max_norm = self.max_norm.max(o.max_norm);
        for (&q, &n) in &o.hist {
            *self.hist.entry(q).or_default() += n;
        }
    }
}
fn abs(x: &BigInt) -> BigInt {
    if x.sign() == Sign::Minus {
        -x
    } else {
        x.clone()
    }
}
fn modulo(x: BigInt, p: &BigInt) -> BigInt {
    let x = x % p;
    if x.sign() == Sign::Minus {
        x + p
    } else {
        x
    }
}
fn words(x: &BigUint) -> [u64; 4] {
    let v = x.to_u64_digits();
    assert!(v.len() <= 4);
    std::array::from_fn(|i| v.get(i).copied().unwrap_or(0))
}
fn big(w: [u64; 4]) -> BigUint {
    BigUint::from_bytes_le(&w.into_iter().flat_map(u64::to_le_bytes).collect::<Vec<_>>())
}
fn next(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}
fn decode(e: u64) -> (i64, i64) {
    let f = if e == 1 << 31 {
        1i64 << 31
    } else {
        e as u32 as i32 as i64
    };
    let g = (e.wrapping_sub(f as u64) as i64) >> 32;
    (f, g)
}
fn encode(row: [i128; 2]) -> u64 {
    (row[0] as u64).wrapping_add((row[1] as u64).wrapping_shl(32))
}
fn matrix(mode: Mode, mut a: u64, mut b: u64, counts: &mut Counts) -> [i64; 4] {
    let bits = mode.bits();
    assert!(b & 1 != 0);
    if mode == Mode::Jump {
        assert!(a < 1 << 62 && b < 1 << 62);
    }
    let original = (a, b);
    let mut rows = [[1i128, 0i128], [0, 1]];
    let mut e = [1u64, 1u64 << 32];
    let mut left = bits as u32;
    let first = a.trailing_zeros().min(left);
    a >>= first;
    e[1] <<= first;
    rows[1] = rows[1].map(|x| x << first);
    left -= first;
    while left != 0 {
        assert_eq!(a & 1, 1);
        assert_eq!(b & 1, 1);
        if a < b {
            std::mem::swap(&mut a, &mut b);
            rows.swap(0, 1);
            e.swap(0, 1);
        }
        let q = if mode == Mode::Old || left == 1 {
            1i64
        } else {
            let k = left.min(3);
            let raw = (a.wrapping_mul(b) & ((1u64 << k) - 1)) as i64;
            counts.quotient_multiplies += 3;
            if raw >= 1i64 << (k - 1) {
                raw - (1i64 << k)
            } else {
                raw
            }
        };
        let product = b.checked_mul(q.unsigned_abs()).unwrap();
        let (mag, negative) = if q < 0 {
            (a.checked_add(product).unwrap(), false)
        } else if a < product {
            (product - a, true)
        } else {
            (a - product, false)
        };
        let reference = a as i128 - q as i128 * b as i128;
        assert_eq!(reference.unsigned_abs(), mag as u128);
        assert_eq!(reference < 0, negative);
        e[0] = e[0].wrapping_sub(e[1].wrapping_mul(q as u64));
        rows[0] = std::array::from_fn(|j| rows[0][j] - q as i128 * rows[1][j]);
        if negative {
            e[0] = e[0].wrapping_neg();
            rows[0] = rows[0].map(|x| -x);
        }
        let zeros = mag.trailing_zeros().min(left);
        assert!(zeros > 0);
        if mode == Mode::Jump && left >= 2 {
            assert!(zeros >= left.min(3));
        }
        a = mag >> zeros;
        e[1] <<= zeros;
        rows[1] = rows[1].map(|x| x << zeros);
        left -= zeros;
        for j in 0..2 {
            assert_eq!(e[j], encode(rows[j]));
            let norm = rows[j][0].unsigned_abs() + rows[j][1].unsigned_abs();
            assert!(norm <= 1u128 << (bits as u32 - left));
            counts.max_norm = counts.max_norm.max(norm as u64);
        }
        *counts.hist.entry(q).or_default() += 1;
        counts.jumps += 1;
    }
    let (f0, g0) = decode(e[0]);
    let (f1, g1) = decode(e[1]);
    let out = [f0, g0, f1, g1];
    for j in 0..2 {
        let d = [out[j * 2] as i128, out[j * 2 + 1] as i128];
        if mode == Mode::Jump {
            assert_eq!(d, rows[j]);
        } else {
            assert!(d == rows[j] || d == rows[j].map(|x| -x));
        }
        let value = d[0] * original.0 as i128 + d[1] * original.1 as i128;
        assert_eq!(value % (1i128 << bits), 0);
        assert_eq!(
            value.unsigned_abs() >> bits,
            if j == 0 { a as u128 } else { b as u128 }
        );
    }
    assert_eq!(
        (f0 as i128 * g1 as i128 - g0 as i128 * f1 as i128).unsigned_abs(),
        1u128 << bits
    );
    if mode == Mode::Old {
        let gold = accepted_matrix::zero_run_matrix(original.0, original.1);
        assert_eq!([gold.2, gold.3, gold.4, gold.5], out);
    }
    out
}
fn approximate(x: &BigInt, wide_bits: usize, k: usize) -> u64 {
    assert!(wide_bits > 64);
    let w = words(&x.to_biguint().unwrap());
    let shift = wide_bits - (k + 2);
    let word = shift / 64;
    let offset = shift % 64;
    let mut hi = w[word] >> offset;
    if offset != 0 && word < 3 {
        hi |= w[word + 1] << (64 - offset);
    }
    let v = (hi << k) | (w[0] & ((1u64 << k) - 1));
    if k == 30 {
        assert!(v < 1 << 62);
    }
    v
}
fn invariant(a: &BigInt, b: &BigInt, u: &BigInt, v: &BigInt, x: &BigInt, r2: &BigInt, p: &BigInt) {
    assert_eq!(modulo(x * u, p), modulo(a * r2, p));
    assert_eq!(modulo(x * v, p), modulo(b * r2, p));
}
fn inverse<C: FpConfig>(mode: Mode, x: &BigUint, c: &mut Counts) -> BigUint {
    let p = BigInt::from(big(C::P));
    let xx = BigInt::from(x.clone());
    let r2 = BigInt::from(big(C::R2));
    let k = mode.bits();
    let inv = BigInt::from((BigUint::from(1u64) << k).modpow(
        &(&p - BigInt::from(2u64)).to_biguint().unwrap(),
        &p.to_biguint().unwrap(),
    ));
    let inv2 = modulo(&inv * &inv, &p);
    let (mut a, mut b) = (xx.clone(), p.clone());
    let (mut u, mut v) = (r2.clone(), BigInt::from(0));
    let mut pending = None::<[i64; 4]>;
    while a != BigInt::from(0) {
        let bits = a.bits().max(b.bits()) as usize;
        if bits <= 64 {
            if let Some(m) = pending.take() {
                let old = u.clone();
                u = modulo((&old * m[0] + &v * m[1]) * &inv, &p);
                v = modulo((&old * m[2] + &v * m[3]) * &inv, &p);
                c.modular_rows += 2;
            }
            invariant(&a, &b, &u, &v, &xx, &r2, &p);
            if b == BigInt::from(1) {
                return v.to_biguint().unwrap();
            }
            let (mut r0, mut r1) = (
                a.to_biguint()
                    .unwrap()
                    .to_u64_digits()
                    .first()
                    .copied()
                    .unwrap_or(0),
                b.to_biguint()
                    .unwrap()
                    .to_u64_digits()
                    .first()
                    .copied()
                    .unwrap_or(0),
            );
            let (mut f0, mut f1, mut g0, mut g1) = (1i128, 0i128, 0i128, 1i128);
            loop {
                let q = r0 / r1;
                let rem = r0 % r1;
                c.divs += 1;
                if rem == 0 {
                    assert_eq!(r1, 1);
                    break;
                }
                (r0, r1) = (r1, rem);
                (f0, f1) = (f1, f0 - q as i128 * f1);
                (g0, g1) = (g1, g0 - q as i128 * g1);
                assert!(f1.unsigned_abs() < 1u128 << 63 && g1.unsigned_abs() < 1u128 << 63);
            }
            return modulo(u * BigInt::from(f1) + v * BigInt::from(g1), &p)
                .to_biguint()
                .unwrap();
        }
        let mut m = matrix(mode, approximate(&a, bits, k), approximate(&b, bits, k), c);
        let original_max = a.clone().max(b.clone());
        let old_a = a.clone();
        let row = |f: &mut i64, g: &mut i64| {
            let raw = &old_a * (*f) + &b * (*g);
            assert_eq!(&raw % BigInt::from(1u64 << k), BigInt::from(0));
            if raw.sign() == Sign::Minus {
                *f = -*f;
                *g = -*g;
            }
            abs(&raw) >> k
        };
        let (mut f, mut g) = (m[0], m[1]);
        let new_a = row(&mut f, &mut g);
        m[0] = f;
        m[1] = g;
        let (mut f, mut g) = (m[2], m[3]);
        let new_b = row(&mut f, &mut g);
        m[2] = f;
        m[3] = g;
        a = new_a;
        b = new_b;
        assert!(a <= original_max && b <= original_max);
        assert!(b.to_biguint().unwrap().bit(0));
        c.integer_rows += 2;
        c.rounds += 1;
        assert!(c.rounds <= 32, "nonconvergence {mode:?} input{x}");
        if let Some(prev) = pending.take() {
            let composed = std::array::from_fn::<_, 4, _>(|i| {
                let r = i / 2;
                let col = i % 2;
                let t = m[r * 2] as i128 * prev[col] as i128
                    + m[r * 2 + 1] as i128 * prev[2 + col] as i128;
                i64::try_from(t).unwrap()
            });
            for r in 0..2 {
                assert!(
                    composed[r * 2].unsigned_abs() as u128
                        + composed[r * 2 + 1].unsigned_abs() as u128
                        <= 1u128 << (2 * k)
                );
            }
            let old = u.clone();
            u = modulo((&old * composed[0] + &v * composed[1]) * &inv2, &p);
            v = modulo((&old * composed[2] + &v * composed[3]) * &inv2, &p);
            c.modular_rows += 2;
            invariant(&a, &b, &u, &v, &xx, &r2, &p);
        } else {
            let tu = modulo((&u * m[0] + &v * m[1]) * &inv, &p);
            let tv = modulo((&u * m[2] + &v * m[3]) * &inv, &p);
            invariant(&a, &b, &tu, &tv, &xx, &r2, &p);
            pending = Some(m);
        }
    }
    if let Some(m) = pending {
        v = modulo((u * m[2] + v * m[3]) * inv, &p);
        c.modular_rows += 1;
    }
    assert_eq!(b, BigInt::from(1));
    assert_eq!(modulo(xx * &v, &p), modulo(r2, &p));
    v.to_biguint().unwrap()
}
fn field<C: FpConfig>(name: &str) -> Value {
    let p = big(C::P);
    let r2 = big(C::R2);
    let mut cases = vec![];
    for x in [
        BigUint::from(1u64),
        &p - BigUint::from(1u64),
        &p >> 1,
        (&p >> 1) + BigUint::from(1u64),
        &p - BigUint::from(3u64),
        &p - BigUint::from(4u64),
    ] {
        cases.push(("boundary", x));
    }
    for bit in 0..254 {
        let n: BigUint = BigUint::from(1u64) << bit;
        for x in [
            &n - BigUint::from(1u64),
            n.clone(),
            &n + BigUint::from(1u64),
        ] {
            if x != BigUint::from(0u64) && x < p {
                cases.push(("raw-power-neighbor", x));
            }
        }
    }
    let mut seed = 0x817abc5f4306d29eu64;
    let mut first = None;
    for _ in 0..2048 {
        let mut bytes = [0u8; 32];
        for c in bytes.chunks_exact_mut(8) {
            c.copy_from_slice(&next(&mut seed).to_le_bytes());
        }
        let x = BigUint::from_bytes_le(&bytes) % (&p - BigUint::from(1u64)) + BigUint::from(1u64);
        if first.is_none() {
            first = Some(x.clone());
        }
        cases.push(("fullwidth-random", x));
    }
    let base = first.unwrap();
    for i in 0..512u64 {
        let x = (&base + BigUint::from(i)) % (&p - BigUint::from(1u64)) + BigUint::from(1u64);
        cases.push(("nearby", x));
    }
    let mut cohorts = BTreeMap::<String, [Counts; 2]>::new();
    let mut rows = vec![];
    let mut max_rounds = [0u64; 2];
    for (i, (cohort, x)) in cases.iter().enumerate() {
        let expected = x.modpow(&(&p - BigUint::from(2u64)), &p) * &r2 % &p;
        let library = Fp::<C>::from_montgomery(words(x))
            .inverse()
            .unwrap()
            .to_canonical();
        let mut observed = Vec::new();
        for (index, mode) in [Mode::Old, Mode::Jump].into_iter().enumerate() {
            let mut count = Counts::default();
            let got = inverse::<C>(mode, x, &mut count);
            assert_eq!(got, expected);
            assert_eq!(
                Fp::<C>::from_montgomery(words(&got)).to_canonical(),
                library
            );
            max_rounds[index] = max_rounds[index].max(count.rounds);
            let group = cohorts.entry(cohort.to_string()).or_default();
            group[index].add(&count);
            observed.push(count.value());
        }
        rows.push(json!({"index":i,"cohort":cohort,"old":observed[0],"signedjump":observed[1]}));
    }
    let random = &cohorts["fullwidth-random"];
    let old = &random[0];
    let new = &random[1];
    let dominated = new.jumps >= old.jumps
        && new.integer_rows >= old.integer_rows
        && new.modular_rows >= old.modular_rows
        && new.divs >= old.divs
        && (new.jumps > old.jumps
            || new.integer_rows > old.integer_rows
            || new.modular_rows > old.modular_rows
            || new.divs > old.divs);
    json!({"field":name,"inputs":cases.len(),"raw_results_checked":cases.len()*2,"max_rounds":max_rounds,"fullwidth_random_work_dominated":dominated,"cohorts":cohorts.into_iter().map(|(n,c)|(n,json!({"old":c[0].value(),"signedjump":c[1].value()}))).collect::<BTreeMap<_,_>>(),"per_input":rows})
}
fn main() {
    let mut toy = 0u64;
    let mut toy_counts = [Counts::default(), Counts::default()];
    for a in 0..=1023 {
        for b in (1..=1023).step_by(2) {
            for (i, m) in [Mode::Old, Mode::Jump].into_iter().enumerate() {
                let _ = matrix(m, a, b, &mut toy_counts[i]);
                toy += 1;
            }
        }
    }
    let mut boundary = 0;
    for mode in [Mode::Old, Mode::Jump] {
        let max = if mode == Mode::Old {
            u64::MAX
        } else {
            (1u64 << 62) - 1
        };
        for a in [0, 1, 2, 3, 7, 8, 9, max - 2, max - 1, max] {
            for b in [1, 3, 7, 9, max - 2, max] {
                let mut c = Counts::default();
                let _ = matrix(mode, a, b, &mut c);
                boundary += 1;
            }
        }
    }
    let fq = field::<FqConfig>("Fq");
    let fr = field::<FrConfig>("Fr");
    let dominated = fq["fullwidth_random_work_dominated"] == true
        && fr["fullwidth_random_work_dominated"] == true;
    let result = json!({"passed_mathematical_checks":true,"toy_matrix_cases":toy,"matrix_boundaries":boundary,"fields":[fq,fr],"work_dominated_both_primes":dominated,"selects_native":!dominated,"native_selection_requires_complete_tradeoff_review":true,"no_speed_prediction":true,"new_timings":0,"new_SDK_requests":0,"new_proofs":0,"canonical_unchanged":35171,"adopted":false,"goal_complete":false});
    let path = format!("{R}/model-result.json");
    assert!(!std::path::Path::new(&path).exists());
    fs::write(path, serde_json::to_vec_pretty(&result).unwrap()).unwrap();
    println!("PASS toy{toy}/boundaries{boundary}; signedjumpworkdominated={dominated}; mathematicalmodelonly,no timings");
}
