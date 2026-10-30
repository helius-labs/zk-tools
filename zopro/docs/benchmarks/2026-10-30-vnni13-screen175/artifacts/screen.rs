use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::arch::x86_64::*;
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

const ROOT: &str = "/home/ubuntu/src/zopro-perf-work/vnni13-screen175";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn root(p: &str) -> PathBuf {
    Path::new(ROOT).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(p: impl AsRef<Path>, v: Value) {
    fs::write(p, serde_json::to_vec_pretty(&v).unwrap()).unwrap();
}
fn setup() {
    assert!(!root("plan.json").exists());
    let manifest = Path::new(Z).join(
        "docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json",
    );
    let m = read(&manifest);
    assert!(m["rustflags"]
        .as_str()
        .unwrap()
        .contains("--cfg zopro_cios_cancel_carry"));
    let mut pins = BTreeMap::new();
    for (p, h) in m["source_files_sha256"].as_object().unwrap() {
        let path = Path::new(Z).join(p);
        assert_eq!(hash(&path), h.as_str().unwrap());
        pins.insert(path, h.as_str().unwrap().to_owned());
    }
    assert_eq!(pins.len(), 175);
    for p in [
        "design.md",
        "screen.rs",
        "screen",
        "extract.rs",
        "extract-initial.rs",
        "extract",
        "extract.log",
        "instruction-rows.json",
        "sources/amd-58455.zip",
        "sources/Zen5_Instruction_Latencies_version_1-00.xlsx",
        "sources/58455_1.00.pdf",
        "sources/sharedStrings.xml",
        "sources/sheet2.xml",
    ] {
        pins.insert(root(p), hash(root(p)));
    }
    for p in [
        manifest,
        Path::new(Z).join("docs/benchmarks/2026-10-30-current-bench-snapshot175/data.json"),
        PathBuf::from("/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask"),
    ] {
        pins.insert(p.clone(), hash(p));
    }
    save(root("pins.json"), json!(pins));
    save(
        root("plan.json"),
        json!({"version":1,"source_pins":root("pins.json"),"steps":[{"kind":"command","name":"isa-and-instruction-budget","argv":[root("screen"),"run"],"timeout_seconds":120}]}),
    );
}
#[target_feature(enable = "avx512f,avx512vnni")]
unsafe fn dot(acc: [i32; 16], a: [i16; 32], b: [i16; 32]) -> [i32; 16] {
    unsafe {
        let v = _mm512_dpwssd_epi32(
            _mm512_loadu_si512(acc.as_ptr().cast()),
            _mm512_loadu_si512(a.as_ptr().cast()),
            _mm512_loadu_si512(b.as_ptr().cast()),
        );
        let mut out = [0i32; 16];
        _mm512_storeu_si512(out.as_mut_ptr().cast(), v);
        out
    }
}
fn check(acc: [i32; 16], a: [i16; 32], b: [i16; 32]) {
    let got = unsafe { dot(acc, a, b) };
    for lane in 0..16 {
        let want = acc[lane] as i64
            + (a[2 * lane] as i64) * (b[2 * lane] as i64)
            + (a[2 * lane + 1] as i64) * (b[2 * lane + 1] as i64);
        assert_eq!(got[lane], want as i32);
    }
}
fn columns(n: usize, first: usize, last: usize) -> Vec<u64> {
    (first..=last)
        .map(|k| (0..n).filter(|&i| k >= i && k - i < n).count() as u64)
        .collect()
}
fn run() {
    let pins = read(root("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    assert!(is_x86_feature_detected!("avx512f") && is_x86_feature_detected!("avx512vnni"));
    let mut vectors = 0;
    for edge in [i16::MIN, -8191, 0, 8191, i16::MAX] {
        for acc in [i32::MIN, -1, 0, i32::MAX] {
            check([acc; 16], [edge; 32], [edge; 32]);
            vectors += 1;
        }
    }
    let mut seed = 0x93c467e37db0c7a5u64;
    let mut next = || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        seed
    };
    for _ in 0..1024 {
        let acc = std::array::from_fn(|_| next() as i32);
        let a = std::array::from_fn(|_| next() as i16);
        let b = std::array::from_fn(|_| next() as i16);
        check(acc, a, b);
        vectors += 1;
    }
    let mut acc = [0; 16];
    for _ in 0..20 {
        acc = unsafe { dot(acc, [8191; 32], [8191; 32]) };
    }
    for x in acc {
        assert_eq!(x as u32, 40 * 8191u32 * 8191u32);
    }
    let n = 20;
    let full = columns(n, 0, 2 * n - 2);
    let low = columns(n, 0, n - 1);
    let upper = columns(n, n - 2, 2 * n - 2);
    let sum = |v: &Vec<u64>| v.iter().sum::<u64>();
    assert_eq!((sum(&full), sum(&low), sum(&upper)), (400, 210, 229));
    let padding = |v: &Vec<u64>| v.iter().map(|v| v.div_ceil(2)).sum::<u64>();
    let doc = read(root("instruction-rows.json"));
    assert_eq!(doc["rows"].as_array().unwrap().len(), 3);
    for row in doc["rows"].as_array().unwrap() {
        assert_eq!(row["Latency"], "4");
        assert_eq!(row["Throughput"], "2");
        assert_eq!(row["Unit"], "FP0/1");
    }
    let ordinary_products = sum(&full) + sum(&low) + sum(&upper);
    let fused_products = 2 * sum(&full) + sum(&low) + sum(&upper);
    let ordinary = (ordinary_products * 8).div_ceil(32);
    let fused = (fused_products * 8).div_ceil(32);
    let ordinary_padded = ((padding(&full) + padding(&low) + padding(&upper)) * 8).div_ceil(16);
    let fused_padded = ((sum(&full) + padding(&low) + padding(&upper)) * 8).div_ceil(16);
    assert_eq!(
        (ordinary, fused, ordinary_padded, fused_padded),
        (210, 310, 220, 315)
    );
    // Five CIOS rows: ten raw IFMAs (twenty fused), one inverse IFMA,
    // and nine reduction IFMAs with the accepted cancel-carry shortcut.
    let accepted_ordinary = 5 * (10 + 1 + 9);
    let accepted_fused = 5 * (20 + 1 + 9);
    let reject = ordinary >= accepted_ordinary && fused >= accepted_fused;
    assert!(reject);
    let digit_product = 8191u64.pow(2);
    assert!(20 * digit_product < i32::MAX as u64);
    assert!(40 * digit_product > i32::MAX as u64 && 40 * digit_product < u32::MAX as u64);
    save(
        root("analysis.json"),
        json!({
            "complete":true,"production_changed":false,"adopted":false,
            "recipe_rejected_by_instruction_budget":reject,
            "scope":"One direct radix13 convolution/truncated-REDC recipe, no general VNNI impossibility or runtime claim",
            "isa_checks":{"avx512f":true,"avx512vnni":true,"scalar_oracle_vectors":vectors,"scalar_oracle_dwords":vectors*16,"radix13_fused_accumulated_dwords":16,"wrapping_i32_semantics":true},
            "digit_bits":13,"digits":20,"radix_bits":260,
            "ordinary_column_max":20*digit_product,"fused_column_max":40*digit_product,
            "ordinary_word_products_per_field":ordinary_products,"fused_word_products_per_field":fused_products,
            "eight_field_vector_instruction_capacity_bounds":{"ordinary":ordinary,"fused":fused,"per_coefficient_odd_padding_ordinary":ordinary_padded,"per_coefficient_odd_padding_fused":fused_padded},
            "accepted_cancel_carry_CIOS_IFMAs":{"ordinary":accepted_ordinary,"fused":accepted_fused},
            "equal_published_multiply_characteristics":true,
            "full_Montgomery_model_qualified":false,"native_field_kernel_implemented":false,
            "new_timing_samples":0,"SDK_requests":0,"new_independent_exports":0,"distinct_proof_ledger":25231
        }),
    );
    println!("ISA wrapping oracle passed: {} vectors; instruction-capacity bounds ordinary {ordinary}/100 and fused {fused}/150; reject recipe before field implementation",vectors);
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("setup") => setup(),
        Some("run") => run(),
        _ => panic!("setup or run required"),
    }
}
