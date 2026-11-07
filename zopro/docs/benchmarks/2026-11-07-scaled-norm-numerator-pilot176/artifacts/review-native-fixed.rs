use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Read, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/scaled-norm-numerator176/native";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn load(p: &str) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: &str) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut d = Sha256::new();
    let mut b = [0; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        d.update(&b[..n]);
    }
    format!("{:x}", d.finalize())
}
fn digest(s: &str) -> String {
    format!("{:x}", Sha256::digest(s.as_bytes()))
}
fn replace(s: &mut String, a: &str, b: &str) {
    assert_eq!(s.matches(a).count(), 1, "missing/nonunique {a}");
    *s = s.replacen(a, b, 1);
}
fn remove_function(s: &mut String, marker: &str) {
    assert_eq!(s.matches(marker).count(), 1);
    let start = s.find(marker).unwrap();
    let brace = start + s[start..].find('{').unwrap();
    let mut depth = 0;
    let mut end = 0;
    for (i, c) in s[brace..].char_indices() {
        if c == '{' {
            depth += 1;
        }
        if c == '}' {
            depth -= 1;
            if depth == 0 {
                end = brace + i + 1;
                break;
            }
        }
    }
    assert!(end > brace);
    if s[end..].starts_with('\n') {
        end += 1;
    }
    s.replace_range(start..end, "");
}
fn original(mut s: String, p: &str) -> String {
    match p{
 "crates/zopro-bn254/src/ifma.rs"|"crates/zopro-bn254/src/ifma8.rs"=>{let class=if p.ends_with("ifma8.rs"){"Ifma8"}else{"Ifma"};let marker=format!("impl<C: FpConfig> {class}<C, true> {{");assert_eq!(s.matches(&marker).count(),1);let start=s.find(&marker).unwrap();remove_function(&mut s,&marker);if s[start..].starts_with('\n'){s.remove(start);}},
 "crates/zopro-bn254/src/lib.rs"=>replace(&mut s,"\n#[cfg(all(test,target_arch=\"x86_64\",target_feature=\"avx512ifma\",target_feature=\"avx512vl\",target_feature=\"avx512dq\",not(no_ifma)))]\nmod scaled_fused_tests;\n",""),
 "crates/zopro-core/src/simd.rs"=>{remove_function(&mut s,"    #[inline(always)]\n    pub(crate) fn sum2_div16");remove_function(&mut s,"    #[inline(always)]\n    pub(crate) fn difference2_div16");},
 "crates/zopro-core/src/msm.rs"=>replace(&mut s,"\n#[cfg(all(zopro_g2_prefix_records,feature=\"ifma8\",target_arch=\"x86_64\",target_feature=\"avx512ifma\",target_feature=\"avx512vl\",target_feature=\"avx512dq\",not(no_ifma)))]\nmod g2_scaled;\n",""),
 "crates/zopro-core/src/msm/g2_prefix.rs"=>{replace(&mut s,"g2_scaled::norm(&denominator)","Fq2::lanes_norm(&denominator)");replace(&mut s,"g2_scaled::numerator(&dy,&denominator,&factor)","Fq2::lanes_scaled_numerator(&dy,&denominator,&factor)");},
 "crates/zopro-core/src/msm/g2_borrowed.rs"=>{replace(&mut s,"g2_scaled::norm(&denominator)","Fq2::lanes_norm(&denominator)");replace(&mut s,"g2_scaled::numerator(&dy, &denominator, &factor)","Fq2::lanes_scaled_numerator(&dy, &denominator, &factor)");},_=>panic!("unexpected parent {p}")};
    s
}
fn receipts(plan: &str, dir: &str, expected_tests: usize) -> Vec<Value> {
    let plan = load(plan);
    assert_eq!(load(&format!("{dir}/result.json"))["status"], "complete");
    let mut out = vec![];
    for step in plan["steps"].as_array().unwrap() {
        let name = step["name"].as_str().unwrap();
        let base = format!("{dir}/{name}");
        let exit = load(&format!("{base}/exit.json"));
        assert_eq!(exit["success"], true);
        assert_eq!(exit["returncode"], 0);
        let cmd = load(&format!("{base}/command.json"));
        let a = cmd["argv"].as_array().unwrap();
        let b = step["argv"].as_array().unwrap();
        assert_eq!(&a[a.len() - b.len()..], b);
        for (k, v) in step["environment"].as_object().unwrap() {
            if k != "RUSTC" && k != "RUSTUP_TOOLCHAIN" {
                assert_eq!(&cmd["environment"][k], v);
            }
        }
        for stream in ["stdout", "stderr"] {
            assert_eq!(
                hash(&format!("{base}/{stream}.log")),
                exit[format!("{stream}_sha256")].as_str().unwrap()
            );
        }
        let output = fs::read_to_string(format!("{base}/stdout.log")).unwrap();
        assert!(output.contains(&format!(
            "test result: ok. {expected_tests} passed; 0 failed; 0 ignored; 0 measured;"
        )));
        if expected_tests == 2 {
            assert!(output.contains("35 filtered out"));
        } else {
            assert!(output.contains("0 filtered out"));
            assert!(output.contains("scaled_norm_activation ... ok"));
            assert!(output.contains("scaled_norm_signed_owned_tails_reuse ... ok"));
        }
        out.push(json!({"name":name,"tests":expected_tests,"exit_sha256":hash(&format!("{base}/exit.json"))}));
    }
    out
}
fn main() {
    let manifest=load(&format!("{Z}/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));
    let mut snapshots = BTreeMap::new();
    for snapshot in ["source", "core-source"] {
        let mut exact = 0;
        let mut changed = BTreeMap::new();
        for (p, h) in manifest["source_files_sha256"].as_object().unwrap() {
            assert_eq!(hash(&format!("{Z}/{p}")), h.as_str().unwrap());
            let file = format!("{R}/{snapshot}/{p}");
            let got = hash(&file);
            if got == h.as_str().unwrap() {
                exact += 1;
            } else {
                let recovered = original(fs::read_to_string(&file).unwrap(), p);
                assert_eq!(
                    digest(&recovered),
                    h.as_str().unwrap(),
                    "recovery {snapshot}/{p}"
                );
                changed.insert(p, json!({"actual_sha256":got,"recovered_sha256":h}));
            }
        }
        let expected = if snapshot == "source" { 3 } else { 7 };
        assert_eq!(changed.len(), expected);
        assert_eq!(exact, 176 - expected);
        snapshots.insert(snapshot,json!({"exact_originals":exact,"reversible_parents":changed,"new_children":if snapshot=="source"{1}else{2}}));
    }
    let mut pins = 0;
    for file in ["field-pins.json", "core-pins.json"] {
        for (p, h) in load(&format!("{R}/{file}")).as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
            pins += 1;
        }
    }
    for (p, _) in load(&format!("{R}/field-pins.json")).as_object().unwrap() {
        let rel = p.strip_prefix(&format!("{R}/source/")).unwrap();
        if rel.starts_with("crates/zopro-bn254/") {
            assert_eq!(hash(p), hash(&format!("{R}/core-source/{rel}")));
        }
    }
    let field = receipts(
        &format!("{R}/field-plan.json"),
        &format!("{R}/field-runs"),
        2,
    );
    let core = receipts(
        &format!("{R}/core-plan.json"),
        &format!("{R}/core-runs"),
        127,
    );
    assert_eq!(field.len(), 1);
    assert_eq!(core.len(), 2);
    let p = load(&format!("{R}/core-plan.json"));
    assert_eq!(
        p["steps"][0]["environment"]["RUSTFLAGS"],
        manifest["rustflags"]
    );
    assert_eq!(
        p["steps"][1]["environment"]["RUSTFLAGS"],
        format!(
            "{} --cfg zopro_scaled_norm_numerator",
            manifest["rustflags"].as_str().unwrap()
        )
    );
    let helper = fs::read_to_string(format!(
        "{R}/core-source/crates/zopro-core/src/msm/g2_scaled.rs"
    ))
    .unwrap();
    let runtime = helper.split("#[cfg(test)]").next().unwrap();
    assert_eq!(runtime.matches("sum2_div16").count(), 2);
    assert_eq!(runtime.matches("difference2_div16").count(), 1);
    assert!(!runtime.contains("unsafe") && !runtime.contains("Vec<"));
    let out = format!("{R}/native-review.json");
    assert!(!Path::new(&out).exists());
    fs::write(out,serde_json::to_vec_pretty(&json!({"review_passed":true,"snapshots":snapshots,"source_pin_checks":pins,"field_receipts":field,"core_receipts":core,"new_field_tests":2,"old_field_tests_filtered":35,"native_scaled_lane_outputs":4096*2*(4+8)*2,"new_core_tests":2,"complete_core_tests_each_mode":127,"new_native_point_outputs_each_mode":2413*8*3,"scaled_core_API_calls":3,"same_qualified_field_bytes_in_both_snapshots":true,"production_changed":false,"new_proofs":0,"new_timings":0,"performance_qualified":false,"review_scope":"Source recovery, receipts, coverage and flags; no successful test/model rerun and no second derivation/ISA decoder."})).unwrap()).unwrap();
    println!(
        "PASSfield2/core127each;196608scaledlaneoutputs;originalparentsrecover;no timing claim"
    );
}
