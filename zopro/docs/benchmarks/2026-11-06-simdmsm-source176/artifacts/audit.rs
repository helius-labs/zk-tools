use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/simdmsm-source176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const M:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json";
fn h(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
// Replace C comments and literals with spaces; preserve byte positions/newlines.
fn active(s: &str) -> String {
    let b = s.as_bytes();
    let mut o = b.to_vec();
    let mut i = 0;
    while i < b.len() {
        if b[i..].starts_with(b"//") {
            while i < b.len() && b[i] != b'\n' {
                o[i] = b' ';
                i += 1;
            }
        } else if b[i..].starts_with(b"/*") {
            o[i] = b' ';
            o[i + 1] = b' ';
            i += 2;
            while i + 1 < b.len() && !b[i..].starts_with(b"*/") {
                if b[i] != b'\n' {
                    o[i] = b' ';
                }
                i += 1;
            }
            assert!(i + 1 < b.len());
            o[i] = b' ';
            o[i + 1] = b' ';
            i += 2;
        } else if b[i] == b'"' || b[i] == b'\'' {
            let q = b[i];
            o[i] = b' ';
            i += 1;
            let mut ended = false;
            while i < b.len() {
                let c = b[i];
                if c == b'\\' {
                    o[i] = b' ';
                    i += 1;
                    assert!(i < b.len());
                    if b[i] != b'\n' {
                        o[i] = b' ';
                    }
                    i += 1;
                    continue;
                }
                if c != b'\n' {
                    o[i] = b' ';
                }
                i += 1;
                if c == q {
                    ended = true;
                    break;
                }
            }
            assert!(ended);
        } else {
            i += 1;
        }
    }
    String::from_utf8(o).unwrap()
}
fn body<'a>(s: &'a str, name: &str) -> (&'a str, usize, usize) {
    let pos = s.find(&format!("{name}(")).unwrap();
    let start = pos + s[pos..].find('{').unwrap();
    let mut depth = 0;
    for (i, c) in s[start..].bytes().enumerate() {
        if c == b'{' {
            depth += 1;
        } else if c == b'}' {
            depth -= 1;
            if depth == 0 {
                return (
                    &s[start + 1..start + i],
                    s[..pos].bytes().filter(|&b| b == b'\n').count() + 1,
                    s[..start + i].bytes().filter(|&b| b == b'\n').count() + 1,
                );
            }
        }
    }
    panic!("unclosed {name}")
}
fn calls(s: &str) -> BTreeMap<String, usize> {
    let b = s.as_bytes();
    let mut i = 0;
    let mut out = BTreeMap::new();
    while i < b.len() {
        if b[i].is_ascii_alphabetic() || b[i] == b'_' {
            let start = i;
            i += 1;
            while i < b.len() && (b[i].is_ascii_alphanumeric() || b[i] == b'_') {
                i += 1;
            }
            let end = i;
            while i < b.len() && b[i].is_ascii_whitespace() {
                i += 1;
            }
            if i < b.len() && b[i] == b'(' {
                *out.entry(s[start..end].into()).or_default() += 1;
            }
        } else {
            i += 1;
        }
    }
    out
}
fn main() {
    let arg = std::env::args().nth(1).unwrap();
    if arg == "prepare" {
        assert!(!Path::new(R).join("analysis.json").exists());
        let mut pins = BTreeMap::new();
        for f in [
            "audit.rs",
            "audit",
            "design.md",
            "reading.md",
            "source-pins.sha256",
            "pair_ifma.c",
            "pip_ifma.c",
            "padd.c",
            "types.h",
        ] {
            let p = format!("{R}/{f}");
            pins.insert(p.clone(), h(p));
        }
        pins.insert(M.into(), h(M));
        let manifest = read(M);
        assert_eq!(
            manifest["source_files_sha256"].as_object().unwrap().len(),
            176
        );
        for (p, sha) in manifest["source_files_sha256"].as_object().unwrap() {
            let p = format!("{Z}/{p}");
            assert_eq!(h(&p), sha.as_str().unwrap());
            pins.insert(p, sha.as_str().unwrap().into());
        }
        for report in [
            "2026-10-22-universal-g1-projective-reduction-sdk",
            "2026-10-23-hybrid-reduction-component",
            "2026-10-24-g1-kernel-split",
            "2026-10-23-pmns-field-component",
            "2026-10-25-operation-model",
            "2026-10-23-exact-fma-rne-component",
            "2026-11-06-aeglos-msm-mechanism176",
            "2026-11-06-g2-two-chain-stream176",
            "2026-11-06-g2-descriptor-chunks176",
            "2026-11-06-g1-control-codegen176",
        ] {
            let p = format!("{Z}/docs/benchmarks/{report}/README.md");
            pins.insert(p.clone(), h(p));
        }
        fs::write(
            format!("{R}/pins.json"),
            serde_json::to_vec_pretty(&pins).unwrap(),
        )
        .unwrap();
        let plan = json!({"version":1,"source_pins":format!("{R}/pins.json"),"steps":[{"kind":"command","name":"source-formula-audit","argv":[format!("{R}/audit"),"analyze"],"timeout_seconds":120}]});
        fs::write(
            format!("{R}/plan.json"),
            serde_json::to_vec_pretty(&plan).unwrap(),
        )
        .unwrap();
        println!("Prepared {} pins; one read-only audit", pins.len());
        return;
    }
    assert_eq!(arg, "analyze");
    assert!(!Path::new(R).join("analysis.json").exists());
    let pins = read(format!("{R}/pins.json"));
    for (p, sha) in pins.as_object().unwrap() {
        assert_eq!(h(p), sha.as_str().unwrap());
    }
    for line in fs::read_to_string(format!("{R}/source-pins.sha256"))
        .unwrap()
        .lines()
    {
        let (sha, p) = line.split_once("  ").unwrap();
        assert_eq!(h(p), sha);
    }
    assert_eq!(
        calls(&active("f();/* f(); */g(\"f()\"); // f();\nh();")),
        BTreeMap::from([("f".into(), 1), ("g".into(), 1), ("h".into(), 1)])
    );
    let padd = active(&fs::read_to_string(format!("{R}/padd.c")).unwrap());
    let mut formulas = vec![];
    for (name, m, s, dig) in [
        ("PADD_G2_projc_8x1w_z1eqz2", 5, 2, 0),
        ("PADD_G2_projc_8x1w_mix", 9, 2, 0),
        ("PADD_G2_projc_8x1w_z1neqz2", 12, 0, 2),
    ] {
        let (b, start, end) = body(&padd, name);
        let c = calls(b);
        assert_eq!(c.get("gfp2_mul_8x1w").copied().unwrap_or(0), m);
        assert_eq!(c.get("gfp2_sqr_8x1w").copied().unwrap_or(0), s);
        assert_eq!(c.get("gfp2_mul_dig_8x1w").copied().unwrap_or(0), dig);
        formulas.push(json!({"file":"padd.c","function":name,"line_start":start,"line_end":end,"Fq2_multiply_sites":m,"Fq2_square_sites":s,"coefficient_specific_sites":dig,"active_calls":c}));
    }
    let pair = active(&fs::read_to_string(format!("{R}/pair_ifma.c")).unwrap());
    let mut routes = vec![];
    for (name, g1, g2) in [
        ("avx1_pair_withIFMA", "simdPADDaff_G1", "simdPADDaff_G2"),
        ("avx2_pair_withIFMA", "simdPADDprj_G1", "simdPADDprj_G2"),
    ] {
        let (b, start, end) = body(&pair, name);
        let c = calls(b);
        assert_eq!(c.get(g1), Some(&1));
        assert_eq!(c.get(g2), Some(&1));
        routes.push(json!({"function":name,"line_start":start,"line_end":end,"separate_G1_call":g1,"separate_G2_call":g2}));
    }
    assert!(body(&pair, "pippenger_pair_ifma")
        .0
        .contains("b = get_wval(k[j], i, WBITS)"));
    let mut wrappers = vec![];
    for (file, name, call) in [
        (
            "pair_ifma.c",
            "simdPADDaff_G1",
            "PADD_projc_8x1w_z1eqz2(&c, &a, &b)",
        ),
        (
            "pair_ifma.c",
            "simdPADDaff_G2",
            "PADD_G2_projc_8x1w_z1eqz2(&c, &a, &b)",
        ),
        (
            "pair_ifma.c",
            "simdPADDmix_G1",
            "PADD_projc_8x1w_mix(&c, &a, &b)",
        ),
        (
            "pair_ifma.c",
            "simdPADDmix_G2",
            "PADD_G2_projc_8x1w_mix(&c, &a, &b)",
        ),
        (
            "pip_ifma.c",
            "simdPADDaff",
            "PADD_projc_8x1w_z1eqz2(&c, &a, &b)",
        ),
    ] {
        let s = active(&fs::read_to_string(format!("{R}/{file}")).unwrap());
        let (b, start, end) = body(&s, name);
        assert!(b.contains(call));
        assert!(!b.contains("inverse") && !b.contains("norm"));
        wrappers.push(json!({"file":file,"function":name,"line_start":start,"line_end":end,"formula_call":call,"inverse_or_norm_text_in_wrapper":false}));
    }
    let g = fs::read_to_string(format!("{Z}/crates/zopro-core/src/groth16.rs")).unwrap();
    assert!(g.contains("// The B1 bytes are intentionally not read."));
    let f = fs::read_to_string(format!("{Z}/crates/zopro-core/src/msm.rs")).unwrap();
    for marker in [
        "dy[0].sum2(&dx[0], &dy[1], &dx[1])",
        "dy[1].difference2(&dx[0], &dy[0], &dx[1])",
        "a[0].difference2(&b[0], &a[1], &b[1])",
        "a[0].sum2(&b[1], &a[1], &b[0])",
        "value[0].sum2(&value[0], &value[1], &value[1])",
    ] {
        assert!(f.contains(marker));
    }
    let fma = fs::read_to_string(format!(
        "{Z}/docs/benchmarks/2026-10-23-exact-fma-rne-component/README.md"
    ))
    .unwrap();
    assert!(fma.contains("Inputs and outputs retain Montgomery radix 2^256."));
    assert!(fma.contains("All tracks failed."));
    let result = json!({"scope":"Read-only structural audit; source call sites are not timing, correctness proof or dynamic costs","upstream_commit":"5e28e01f84f3c56e63952fd2099dabddc5b55809","pins_verified":pins.as_object().unwrap().len(),"accepted_production_files_verified":176,"formulas":formulas,"paired_routes":routes,"wrappers":wrappers,"shared_scalar_digit":true,"accepted_primary_omits_B1":true,"native_Fq2_product_uses_two_fused_operations":true,"paired_projective_coordinate_arrays":9,"paired_affine_coordinate_arrays":6,"bytes_per_coordinate_array":"HT_NWORDS * sizeof(uint64_t); definition outside audited files","complete_live_memory_bound_established":false,"BN254_port_or_exception_correctness_established":false,"R256_FMA_trial_already_closed":true,"decision":"No distinct runtime candidate; no unchanged paired/projective/FMA reroll","new_runtime_candidate":false,"new_native_tests":0,"new_benchmarks":0,"new_proofs":0,"new_AWS_resources":0,"goal_status":"active/unmet0/8"});
    fs::write(
        format!("{R}/analysis.json"),
        serde_json::to_vec_pretty(&result).unwrap(),
    )
    .unwrap();
    println!("PASS {}pins/176originals/3activeG2formulas/2pairedroutes/5wrappers; no native build/timing/proof",pins.as_object().unwrap().len());
}
