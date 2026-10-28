use ark_bn254::{Fr, G1Projective};
use ark_ec::{CurveGroup, PrimeGroup, VariableBaseMSM};
use ark_ff::{AdditiveGroup, BigInt as ArkInt, PrimeField};
use num_bigint::{BigInt, BigUint};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};

include!("native_booth.rs");
const SEED: u64 = 0x381a_2094_5b6c_74d1;
const MODULUS: [u64; 4] = Fr::MODULUS.0;
const STRATEGIES: [&str; 5] = ["canonical", "centered", "factor2", "priority3_2", "priority5_3_2"];

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
    fn scalar(&mut self) -> [u64; 4] {
        loop {
            let mut k = [self.next(), self.next(), self.next(), self.next()];
            k[3] &= (1u64 << 62) - 1;
            if less(&k, &MODULUS) { return k; }
        }
    }
}
fn less(a: &[u64; 4], b: &[u64; 4]) -> bool {
    a.iter().rev().cmp(b.iter().rev()).is_lt()
}
fn sub(a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    assert!(!less(&a, &b));
    let mut out = [0; 4];
    let mut borrow = false;
    for i in 0..4 {
        let (v, b1) = a[i].overflowing_sub(b[i]);
        let (v, b2) = v.overflowing_sub(u64::from(borrow));
        out[i] = v; borrow = b1 || b2;
    }
    assert!(!borrow); out
}
fn divrem(a: [u64; 4], d: u64) -> ([u64; 4], u64) {
    let mut out = [0; 4]; let mut rem = 0u128;
    for i in (0..4).rev() {
        let v = (rem << 64) | u128::from(a[i]);
        out[i] = (v / u128::from(d)) as u64; rem = v % u128::from(d);
    }
    (out, rem as u64)
}
fn integer(k: [u64; 4]) -> BigUint {
    BigUint::from_bytes_le(&k.into_iter().flat_map(u64::to_le_bytes).collect::<Vec<_>>())
}
fn fr(k: [u64; 4]) -> Fr { Fr::from_bigint(ArkInt(k)).unwrap() }
#[derive(Clone, Copy, Debug)]
struct Recoded { words: [u64; 4], negative: bool, factor: u64 }
fn recode(k: [u64; 4], strategy: &str) -> Recoded {
    let neg = sub(MODULUS, k);
    let original = Recoded { words: k, negative: false, factor: 1 };
    match strategy {
        "canonical" => original,
        "centered" => if less(&neg, &k) { Recoded { words: neg, negative: true, factor: 1 } } else { original },
        "factor2" | "priority3_2" | "priority5_3_2" => {
            let factors: &[u64] = match strategy { "factor2" => &[2], "priority3_2" => &[3, 2], _ => &[5, 3, 2] };
            for &factor in factors {
                let (q, rem) = divrem(k, factor);
                if rem == 0 { return Recoded { words: q, negative: false, factor }; }
                let (q, rem) = divrem(neg, factor);
                if rem == 0 { return Recoded { words: q, negative: true, factor }; }
            }
            unreachable!()
        }
        _ => panic!("unknown strategy"),
    }
}
fn groups(strategy: &str) -> &'static [u64] {
    match strategy { "canonical" | "centered" => &[1], "factor2" => &[2], "priority3_2" => &[2, 3], _ => &[2, 3, 5] }
}
fn magnitude_bits(strategy: &str, factor: u64) -> usize {
    if strategy == "canonical" { 254 } else if strategy == "centered" { 253 }
    else { integer(divrem(sub(MODULUS, [1, 0, 0, 0]), factor).0).bits() as usize }
}
fn check_identity(k: [u64; 4], t: Recoded) {
    let signed = if t.negative { -fr(t.words) } else { fr(t.words) };
    assert_eq!(signed * Fr::from(t.factor), fr(k));
    let val = BigInt::from(integer(t.words)) * BigInt::from(t.factor);
    let val = if t.negative { -val } else { val };
    let modulus = BigInt::from(integer(MODULUS));
    assert_eq!(((val % &modulus) + &modulus) % &modulus, BigInt::from(integer(k)));
}
fn check_digits(k: [u64; 4], width: usize, bits: usize) {
    let windows = (bits + 1).div_ceil(width);
    let rebuilt = (0..windows).rev().fold(BigInt::from(0), |v, w| {
        (v << width) + BigInt::from(booth_digit(&k, w, width))
    });
    assert_eq!(rebuilt, BigInt::from(integer(k)));
    // Independent direct bit-vector formula, deliberately avoiding native word extraction.
    let value = integer(k);
    let bit = |i: usize| value.bit(i as u64) as i64;
    for w in 0..windows {
        let offset = w * width;
        let mut d = if offset == 0 { 0 } else { bit(offset - 1) };
        for i in 0..width - 1 { d += bit(offset + i) << i; }
        d -= bit(offset + width - 1) << (width - 1);
        assert_eq!(i64::from(booth_digit(&k, w, width)), d);
    }
}
struct Group {
    factor: u64, bits: usize, count: u64, negated: u64,
    hist: Vec<Vec<u64>>, negative: Vec<u64>,
}
fn census(input: &[[u64; 4]], strategy: &str, width: usize) -> Value {
    let mut by_factor: BTreeMap<u64, Group> = groups(strategy).iter().map(|&factor| {
        let bits = magnitude_bits(strategy, factor);
        let windows = (bits + 1).div_ceil(width);
        (factor, Group { factor, bits, count: 0, negated: 0,
            hist: vec![vec![0; (1 << (width - 1)) + 1]; windows], negative: vec![0; windows] })
    }).collect();
    for &k in input {
        let t = recode(k, strategy);
        let g = by_factor.get_mut(&t.factor).unwrap();
        g.count += 1; g.negated += u64::from(t.negative);
        for (w, hist) in g.hist.iter_mut().enumerate() {
            let mut d = booth_digit(&t.words, w, width);
            if t.negative { d = -d; }
            if d == 0 { continue; }
            hist[d.unsigned_abs() as usize] += 1;
            g.negative[w] += u64::from(d < 0);
        }
    }
    let mut total_terms = 0u64; let mut total_occupied = 0u64; let mut total_span = 0u64;
    let rows: Vec<Value> = by_factor.values().map(|g| {
        let windows: Vec<Value> = g.hist.iter().enumerate().map(|(w, hist)| {
            let terms: u64 = hist.iter().sum();
            let occupied = hist.iter().filter(|&&n| n != 0).count() as u64;
            let span = hist.iter().rposition(|&n| n != 0).unwrap_or(0) as u64;
            total_terms += terms; total_occupied += occupied; total_span += span;
            json!({"window":w,"terms":terms,"negative_terms":g.negative[w],"occupied_destinations":occupied,
                "ideal_insertion_additions":terms-occupied,"max_absolute_digit":span})
        }).collect();
        json!({"factor":g.factor,"magnitude_bit_bound":g.bits,"windows":windows,
            "input_count":g.count,"negated_inputs":g.negated,
            "dense_bucket_capacity_per_window":1 << (width-1)})
    }).collect();
    json!({"strategy":strategy,"width":width,"input_count":input.len(),"groups":rows,
        "nonzero_terms":total_terms,"occupied_destinations":total_occupied,
        "ideal_insertion_additions":total_terms-total_occupied,"sum_actual_max_bucket_index":total_span})
}
fn audit(repo: &Path) -> Value {
    let manifest: Value = serde_json::from_slice(&fs::read(repo.join("zopro/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json")).unwrap()).unwrap();
    let files = manifest["source_files_sha256"].as_object().unwrap();
    for (path, expected) in files {
        let path = repo.join("zopro").join(path);
        let hash = format!("{:x}", Sha256::digest(fs::read(&path).unwrap()));
        assert_eq!(hash, expected.as_str().unwrap(), "{}", path.display());
    }
    assert_eq!(files.len(), 172);
    let native = fs::read_to_string(repo.join("zopro/crates/zopro-core/src/msm_affine.rs")).unwrap();
    assert!(native.contains(include_str!("native_booth.rs").trim()));
    let sdk = "/home/ubuntu/src/zopro-perf-work/batch-local-steal/native/control-sdk";
    let sdk_hash = format!("{:x}", Sha256::digest(fs::read(sdk).unwrap()));
    assert_eq!(sdk_hash, manifest["native_binary_sha256"].as_str().unwrap());
    json!({"production_files":files.len(),"all_exact":true,"native_booth_exact":true,"sdk_sha256":sdk_hash})
}
fn main() {
    let repo = std::env::args().nth(1).unwrap();
    let source = audit(Path::new(&repo));
    let mut rng = Rng(SEED);
    let random: Vec<_> = (0..32768).map(|_| rng.scalar()).collect();
    let small: Vec<_> = (0..16384).map(|k| [k, 0, 0, 0]).collect();
    let mut rows = vec![];
    for (name, input) in [("uniform_full_width", random), ("small_0_through_16383", small)] {
        for width in 4..=15 {
            for strategy in STRATEGIES {
                let mut row = census(&input, strategy, width);
                row["workload"] = json!(name); rows.push(row);
            }
        }
    }
    let window_counts: Vec<_> = (4usize..=15).map(|width| {
        assert_eq!(255usize.div_ceil(width), 254usize.div_ceil(width));
        json!({"width":width,"canonical":255usize.div_ceil(width),"factor2":254usize.div_ceil(width)})
    }).collect();
    println!("{}", serde_json::to_string_pretty(&json!({"seed":SEED,"source_audit":source,
        "modulus_decimal":integer(MODULUS).to_string(),"window_counts":window_counts,"rows":rows,
        "scope":"exact digit census, not a native runtime or full-proof benchmark"})).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    fn cases() -> Vec<[u64; 4]> {
        let mut data: Vec<_> = (0..256).map(|k| [k, 0, 0, 0]).collect();
        for k in 1..=256 { data.push(sub(MODULUS, [k, 0, 0, 0])); }
        for bit in 0..254 {
            let mut k = [0; 4]; k[bit / 64] = 1 << (bit % 64);
            if less(&k, &MODULUS) { data.push(k); data.push(sub(k, [1,0,0,0])); }
        }
        let mut rng = Rng(SEED);
        data.extend((0..4096).map(|_| rng.scalar())); data
    }
    #[test]
    fn recoding_matches_independent_integers_and_field() {
        for k in cases() {
            for strategy in STRATEGIES {
                let t = recode(k, strategy); check_identity(k, t);
                assert!(integer(t.words).bits() as usize <= magnitude_bits(strategy, t.factor));
            }
        }
    }
    #[test]
    fn native_booth_matches_bit_reference_and_rebuilds_integers() {
        for k in cases() {
            for strategy in STRATEGIES {
                let t = recode(k, strategy);
                for width in 4..=15 { check_digits(t.words, width, magnitude_bits(strategy, t.factor)); }
            }
        }
    }
    #[test]
    fn separate_factor_msms_match_independent_g1_msm() {
        let scalars = cases();
        let mut rng = Rng(SEED ^ 32);
        let ks: Vec<_> = scalars.iter().step_by(37).copied().collect();
        let points: Vec<_> = ks.iter().enumerate().map(|(i,_)| {
            if i % 17 == 0 { G1Projective::ZERO.into_affine() }
            else { (G1Projective::generator() * fr(rng.scalar())).into_affine() }
        }).collect();
        let original = G1Projective::msm_bigint(&points, &ks.iter().map(|&k| ArkInt(k)).collect::<Vec<_>>());
        for strategy in STRATEGIES {
            let mut result = G1Projective::ZERO;
            for &factor in groups(strategy) {
                let mut bases = vec![]; let mut words = vec![];
                for (&p, &k) in points.iter().zip(&ks) {
                    let t = recode(k, strategy);
                    if t.factor != factor { continue; }
                    bases.push(if t.negative { -p } else { p }); words.push(ArkInt(t.words));
                }
                result += G1Projective::msm_bigint(&bases, &words) * Fr::from(factor);
            }
            assert_eq!(result, original, "{strategy}");
        }
    }
}
