#![allow(dead_code)]
use num_bigint::BigUint;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};
use zopro_bn254::{FpConfig, FqConfig, FrConfig};
#[path = "/home/ubuntu/src/zk-tools/zopro/crates/zopro-bn254/src/x86_64.rs"]
mod native;
#[path = "/home/ubuntu/src/zk-tools/zopro/crates/zopro-bn254/src/portable.rs"]
mod portable;
const RDIR: &str = "/home/ubuntu/src/zopro-perf-work/inverse-lazy-root-product176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(f: &str, v: &Value) {
    fs::write(format!("{RDIR}/{f}"), serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn big(w: [u64; 4]) -> BigUint {
    w.into_iter()
        .enumerate()
        .map(|(i, w)| BigUint::from(w) << (64 * i))
        .sum()
}
fn words(v: &BigUint) -> [u64; 4] {
    let digits = v.to_u64_digits();
    assert!(digits.len() <= 4);
    core::array::from_fn(|i| digits.get(i).copied().unwrap_or(0))
}
fn next(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}
fn check<C: FpConfig>(label: &str) -> Value {
    let p = big(C::P);
    let zero = BigUint::from(0u8);
    let one = BigUint::from(1u8);
    let two = &p * 2u8;
    let r = &one << 256usize;
    let b = &one << 64usize;
    let mask = &b - &one;
    assert!(&p * 4u8 < r);
    assert!(&p * 3u8 < r);
    assert_eq!(C::P[0].wrapping_mul(C::INV), u64::MAX);
    let rinv = r.modpow(&(&p - 2u8), &p);
    let endpoints = [
        zero.clone(),
        one.clone(),
        &p - &one,
        p.clone(),
        &p + &one,
        &two - 2u8,
        &two - &one,
        two.clone(),
    ];
    let mut pairs = BTreeSet::new();
    for a in &endpoints {
        for c in &endpoints {
            pairs.insert((words(a), words(c)));
        }
    }
    for bit in 0..256usize {
        let power = (&one << bit) % (&two + &one);
        for candidate in [
            Some(power.clone()),
            if power >= one {
                Some(&power - &one)
            } else {
                None
            },
            Some((&power + &one) % (&two + &one)),
        ]
        .into_iter()
        .flatten()
        {
            for (a, c) in [
                (candidate.clone(), candidate.clone()),
                (candidate.clone(), &two - &candidate),
                (candidate.clone(), one.clone()),
                (two.clone(), candidate),
            ] {
                pairs.insert((words(&a), words(&c)));
            }
        }
    }
    let mut state = 0x726f6f745f72616eu64;
    for _ in 0..8192 {
        let a = big(core::array::from_fn(|_| next(&mut state))) % (&two + &one);
        let c = big(core::array::from_fn(|_| next(&mut state))) % (&two + &one);
        pairs.insert((words(&a), words(&c)));
    }
    let mut max_sum_bits = 0;
    let mut max_state_bits = 0;
    let mut high_output_zero = 0;
    let mut noncanonical_inputs = 0;
    let mut zero_residue = 0;
    let mut rows = 0;
    for &(aw, bw) in &pairs {
        let a = big(aw);
        let c = big(bw);
        assert!(a <= two && c <= two);
        if a >= p || c >= p {
            noncanonical_inputs += 1;
        }
        let expected = (&a * &c * &rinv) % &p;
        let mut t = zero.clone();
        for limb in bw {
            let v = &t + &a * BigUint::from(limb);
            let low = (&v & &mask).to_u64_digits().first().copied().unwrap_or(0);
            let m = low.wrapping_mul(C::INV);
            let sum = &v + BigUint::from(m) * &p;
            assert_eq!(&sum % &b, zero);
            assert!(sum < &b * (&a + &p));
            assert!(sum < (&one << 320usize));
            max_sum_bits = max_sum_bits.max(sum.bits());
            t = sum >> 64usize;
            assert!(t < &a + &p);
            assert!(t < r);
            max_state_bits = max_state_bits.max(t.bits());
            rows += 1;
        }
        assert!(t < two);
        assert!(&t * &r < &p * &r + &a * &c);
        let reduced = if t >= p { &t - &p } else { t };
        assert_eq!(reduced, expected);
        let portable = portable::multiply(aw, bw, C::P, C::INV);
        let native = native::multiply(aw, bw, C::P, C::INV);
        assert_eq!(big(portable), expected);
        assert_eq!(native, portable);
        assert!(big(native) < p);
        high_output_zero += 1;
        if expected == zero {
            zero_residue += 1;
        }
    }
    json!({"prime":label,"modulus_hex":p.to_str_radix(16),"R":"2^256","four_p_less_R":true,"three_p_less_R":true,"accepted_INV_lowword_checked":true,"distinct_raw_operand_pairs":pairs.len(),"noncanonical_input_pairs":noncanonical_inputs,"zero_output_pairs":zero_residue,"CIOS_rows_checked":rows,"max_observed_row_sum_bits":max_sum_bits,"max_observed_state_bits":max_state_bits,"analytic_row_sum_bits_upper":320,"analytic_state_strictly_less_R":true,"analytic_final_REDC_strictly_less_2p":true,"portable_and_existing_ADX_outputs_canonical_and_equal":high_output_zero,"actual_input_domain":"0<=a,b<=2p (includes proposed strict lazy domain)","new_unreduced_Fp_objects_constructed":0})
}
fn main() {
    if std::env::args().nth(1).as_deref() == Some("prepare") {
        assert!(!Path::new(RDIR).join("pins.json").exists());
        let mut pins = BTreeMap::new();
        for p in[format!("{RDIR}/check.rs"),format!("{RDIR}/design.md"),format!("{Z}/crates/zopro-bn254/src/portable.rs"),format!("{Z}/crates/zopro-bn254/src/x86_64.rs"),format!("{Z}/crates/zopro-bn254/src/ifma8.rs"),format!("{Z}/crates/zopro-bn254/src/ifma.rs"),format!("{Z}/crates/zopro-bn254/src/fp.rs"),format!("{Z}/crates/zopro-core/src/simd.rs"),"/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175/frozen/candidate/libzopro_bn254-1200e63547baa41c299a48e3221f1cef85990b0369f17452dc12eb17045f9713.rlib".into()]{pins.insert(p.clone(),hash(p));}
        save("pins.json", &json!(pins));
        save(
            "plan.json",
            &json!({"version":1,"source_pins":format!("{RDIR}/pins.json"),"environment":{},"steps":[{"kind":"command","name":"raw-root-product-range","argv":[format!("{RDIR}/check")],"timeout_seconds":120}]}),
        );
        println!("Prepared one range/native-backend harness run; original scalar source included unchanged; no field build/proof/timing");
        return;
    }
    assert!(!Path::new(RDIR).join("analysis.json").exists());
    for (p, h) in read(format!("{RDIR}/pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let fq = check::<FqConfig>("Fq");
    let fr = check::<FrConfig>("Fr");
    let total = fq["distinct_raw_operand_pairs"].as_u64().unwrap()
        + fr["distinct_raw_operand_pairs"].as_u64().unwrap();
    let output = json!({"range_screen_passed":true,"fields":[fq,fr],"distinct_raw_pairs":total,"actual_native_code":"byte-exact accepted x86_64::multiply, not rewritten or newly implemented","portable_code":"byte-exact accepted portable::multiply","independent_reference":"BigUint full modular product/R inverse plus per-word exact CIOS row bounds","scope":"raw scalar range only; no SIMD codec/core integration or speed qualification","new_field_crate_builds":0,"new_root_codec_implemented":false,"new_proofs":0,"new_timing_observations":0,"production_changed":false,"adopted":false,"goal_status":"active/unmet0/8"});
    save("analysis.json", &output);
    println!("Range screen PASS {total}distinct pairs across Fq/Fr; exact CIOS bounds and unchanged portable/ADX canonical outputs; no root codec or timing");
}
