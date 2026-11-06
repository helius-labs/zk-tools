use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/inverse-lazy-root-product176";
const D: &str =
    "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-inverse-lazy-root-product176";
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn hash_file(p: &Path) -> String {
    let mut file = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut buffer = [0; 65536];
    loop {
        let n = file.read(&mut buffer).unwrap();
        if n == 0 {
            break;
        }
        h.update(&buffer[..n]);
    }
    format!("{:x}", h.finalize())
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn walk(p: &Path) -> Vec<PathBuf> {
    let mut v = Vec::new();
    for e in fs::read_dir(p).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            v.extend(walk(&p))
        } else {
            v.push(p)
        }
    }
    v.sort();
    v
}
fn member(p: &Path) -> Value {
    let b = fs::read(p).unwrap();
    let text = std::str::from_utf8(&b).expect("only explicit UTF8 helper/delta/receipt members");
    json!({"bytes":b.len(),"sha256":hash(&b),"utf8":text})
}
fn archive(path: &Path, members: &BTreeMap<String, Value>) {
    let bytes = serde_json::to_vec(members).unwrap();
    let mut p = Command::new("gzip")
        .args(["-n", "-c"])
        .stdin(Stdio::piped())
        .stdout(fs::File::create(path).unwrap())
        .spawn()
        .unwrap();
    p.stdin.take().unwrap().write_all(&bytes).unwrap();
    assert!(p.wait().unwrap().success());
    let out = Command::new("gzip")
        .args(["-d", "-c"])
        .arg(path)
        .output()
        .unwrap();
    assert!(out.status.success());
    assert_eq!(out.stdout, bytes);
    let decoded: Value = serde_json::from_slice(&out.stdout).unwrap();
    for v in decoded.as_object().unwrap().values() {
        let b = v["utf8"].as_str().unwrap().as_bytes();
        assert_eq!(b.len(), v["bytes"].as_u64().unwrap() as usize);
        assert_eq!(hash(b), v["sha256"]);
    }
}
fn main() {
    assert!(!Path::new(D).exists());
    let a = read(format!("{R}/analysis.json"));
    assert_eq!(a["range_screen_passed"], true);
    let pins = read(format!("{R}/pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash_file(Path::new(p)), h.as_str().unwrap());
    }
    assert_eq!(pins.as_object().unwrap().len(), 14);
    assert_eq!(read(format!("{R}/runs/result.json"))["status"], "complete");
    let base = Path::new(R).join("runs/raw-root-product-range");
    let e = read(base.join("exit.json"));
    assert_eq!(e["returncode"], 0);
    assert_eq!(e["success"], true);
    for key in ["stdout", "stderr"] {
        assert_eq!(
            hash_file(&base.join(format!("{key}.log"))),
            e[format!("{key}_sha256")]
        );
    }
    let cmd = read(base.join("command.json"));
    let plan = read(format!("{R}/plan.json"));
    assert_eq!(
        &cmd["argv"].as_array().unwrap()[4..],
        plan["steps"][0]["argv"].as_array().unwrap().as_slice()
    );
    let mut cases = 0;
    let mut rows = 0;
    for f in a["fields"].as_array().unwrap() {
        let n = f["distinct_raw_operand_pairs"].as_u64().unwrap();
        assert_eq!(
            f["portable_and_existing_ADX_outputs_canonical_and_equal"],
            n
        );
        assert_eq!(f["CIOS_rows_checked"], 4 * n);
        assert_eq!(f["four_p_less_R"], true);
        assert_eq!(f["three_p_less_R"], true);
        assert_eq!(f["new_unreduced_Fp_objects_constructed"], 0);
        assert!(f["max_observed_row_sum_bits"].as_u64().unwrap() <= 320);
        assert!(f["max_observed_state_bits"].as_u64().unwrap() <= 256);
        cases += n;
        rows += 4 * n;
    }
    assert_eq!(cases, 22616);
    assert_eq!(rows, 90464);
    assert_eq!(a["distinct_raw_pairs"], cases);
    let art = Path::new(D).join("artifacts");
    fs::create_dir_all(&art).unwrap();
    let mut report=String::from("# Deferring inverse-root canonicalization: raw range screen\n\n**Range screen passed; no codec or performance result yet.** The existing scalar Montgomery multiplier can accept the proposed lazy root limbs for BN254 Fq/Fr and still return a canonical result. This permits testing canonicalization after the root product, while the [half-root export route](../2026-11-06-inverse-half-roots176/README.md) remains closed.\n\n");
    report.push_str(&fs::read_to_string(format!("{R}/decision.md")).unwrap());
    report.push_str("\n\n## Exact arithmetic scope\n\nLet R=2^256, B=2^64 and 0≤a,b≤2p. Both primes satisfy 4p<R. Interleaved CIOS starts T₀=0 and computes Tᵢ₊₁=(Tᵢ+a·bᵢ+mᵢp)/B, where 0≤mᵢ<B makes the numerator divisible by B. If Tᵢ<a+p, the numerator is less than B(a+p)≤3pB<2^320 and Tᵢ₊₁<a+p<R. After four rows, T₄<p+ab/R≤p+4p²/R<2p, so the existing single subtraction returns a canonical residue. Canonicalizing either input before multiplication gives the same residue because input values remain congruent modulo p.\n\nThe harness checks the rational final bound without rounding it downward. It compares the exact row model and both actual existing multipliers against an independent BigUint modular product times R inverse. Raw input arrays, including p and 2p boundaries, never become Fp objects. The intended SIMD roots remain strictly below 2p; the scalar screen tests the inclusive boundary too. Mixed R260 cannot use raw R256 interpretation and retains its conversion/fallback.\n\n|Field|Distinct pairs|Noncanonical input pairs|Zero results|CIOS rows|Maximum observed row-sum bits|Maximum observed state bits|\n|---|---:|---:|---:|---:|---:|---:|\n");
    for f in a["fields"].as_array().unwrap() {
        report.push_str(&format!(
            "|{}|{}|{}|{}|{}|{}|{}|\n",
            f["prime"].as_str().unwrap(),
            f["distinct_raw_operand_pairs"],
            f["noncanonical_input_pairs"],
            f["zero_output_pairs"],
            f["CIOS_rows_checked"],
            f["max_observed_row_sum_bits"],
            f["max_observed_state_bits"]
        ));
    }
    report.push_str("\nCounts are distinct within each field; 22,616 is the sum of the two field-tagged populations. Tested pairs cover the Cartesian endpoint set, power/limb boundaries and 8,192 deterministic random pairs per prime, with duplicates removed. The analytic bounds are strict below 2^320 for a row sum and below R for a state; an observed bit length of 320 or 256 fits those bounds. These checks are not a formal instruction-level proof of every CPU flag transition.\n\n## Source and workflow\n\nA normal native Rust harness includes the byte-exact accepted [portable multiplier](../../../crates/zopro-bn254/src/portable.rs#L77) and [BMI2/ADX multiplier](../../../crates/zopro-bn254/src/x86_64.rs#L66), including the latter's existing single final reduce_once. No multiplier, field-crate build, assembly backend or prior field/core test suite is added or rerun. The accepted field library supplies modulus/inverse constants. The [field invariant](../../../crates/zopro-bn254/src/fp.rs#L322) continues to require canonical Fp objects; this screen does not weaken that API.\n\nSource, compiled harness, accepted field and explicit helper-library hashes are frozen before the one successful Rust/xtask execution. The publisher rechecks 14 pins, the command/exit/stream bindings and both field/row totals without repeating the native harness. The harness compiled and ran on its first attempts. No profile, SDK request, solver/FFT extraction, proof, Gnark check or AWS resource was added.\n\nEarlier [direct-chain inversion](../2026-10-24-direct-chain-inverse/README.md) already removes the outer SIMD/host bridges. [Whole-array word export](../2026-11-06-word-export176/README.md), [direct Euclidean finish](../2026-10-28-inverse-finish-sdk/README.md) and [scaled-R260 G2](../2026-10-28-scaled-r260-native/README.md) retain their own results. This screen changes the admissible multiplication boundary; it does not reopen those implementations. History/keyword inspection is not an exhaustive claim about every old temporary source.\n\n## Accepted CPU and Metal comparisons, unchanged\n\n");
    let old=fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-g1-tagged-phi-review176/README.md").unwrap();
    let section = &old[old
        .find("Inherited local 32-worker full-proof observations")
        .unwrap()..];
    report.push_str(&section[..section.find("\n[Analysis]").unwrap()]);
    report.push_str("\n\n## Continuation and artifacts\n\nThe [first unfinished step](artifacts/next.md) is the actual normalized-lazy SIMD root-product codec and its independent output/range checks, followed by final-consumer inspection and prospective complete-caller evidence. Scalar range checks alone do not establish the codec, emitted work reduction or proof speed. All eight verified CPU10× targets remain active and unmet.\n\n[Analysis](artifacts/analysis.json), [design](artifacts/design.md), [decision](artifacts/decision.md), [next](artifacts/next.md), [harness](artifacts/check.rs), [pre-execution freeze](artifacts/freeze.rs), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [helpers and receipts](artifacts/helpers-and-receipts.json.gz), [publication](artifacts/publication.json), [checksums](SHA256SUMS). Full executable/library blobs remain local. No private key, image, witness, point/scalar cache or proof bytes are included. The accepted source files are linked and hash-bound rather than duplicated as a new backend.\n");
    fs::write(Path::new(D).join("README.md"), report).unwrap();
    for f in [
        "analysis.json",
        "design.md",
        "decision.md",
        "next.md",
        "check.rs",
        "freeze.rs",
        "plan.json",
        "pins.json",
        "publish.rs",
    ] {
        fs::copy(format!("{R}/{f}"), art.join(f)).unwrap();
    }
    let mut members = BTreeMap::new();
    for e in fs::read_dir(R).unwrap() {
        let p = e.unwrap().path();
        if p.is_file()
            && p.extension()
                .is_some_and(|e| ["rs", "md", "json", "log"].iter().any(|s| e == *s))
        {
            members.insert(p.strip_prefix(R).unwrap().display().to_string(), member(&p));
        }
    }
    for p in walk(&Path::new(R).join("runs")) {
        members.insert(p.strip_prefix(R).unwrap().display().to_string(), member(&p));
    }
    archive(&art.join("helpers-and-receipts.json.gz"), &members);
    fs::write(art.join("publication.json"),serde_json::to_vec_pretty(&json!({"passed":true,"source_binary_library_pins_checked":14,"native_harness_commands":1,"distinct_field_tagged_raw_pairs":cases,"CIOS_rows":rows,"helper_receipt_UTF8_members":members.len(),"archive_hashes_and_roundtrip_verified":true,"new_field_builds":0,"new_codec_implemented":false,"new_timings":0,"new_proofs":0,"adopted":false,"production_changed":false,"goal_status":"active/unmet0/8"})).unwrap()).unwrap();
    let mut sums = String::new();
    for p in walk(Path::new(D)) {
        sums.push_str(&format!(
            "{}  {}\n",
            hash_file(&p),
            p.strip_prefix(D).unwrap().display()
        ));
    }
    fs::write(Path::new(D).join("SHA256SUMS"), sums).unwrap();
    println!("Published {}files/{}UTF8members; range/native scalar22616cases/90464rows; archive/hash/receipt PASS; no codec or timing",walk(Path::new(D)).len(),members.len());
}
