use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/proof-pgo175";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("use-pins.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    assert_eq!(
        read(r("native/result.json"))["steps"]
            .as_array()
            .unwrap()
            .len(),
        9
    );
    assert_eq!(
        read(r("native-use/result.json"))["steps"]
            .as_array()
            .unwrap()
            .len(),
        6
    );
    for (label, dir) in [("control", "native"), ("candidate", "native-use")] {
        for (t, n) in [("field", 33), ("core", 124), ("prover", 8)] {
            let p = r(format!("{dir}/{label}-{t}/tests.json"));
            let a = read(&p);
            assert_eq!(a["passed_tests"].as_array().unwrap().len(), n);
            assert_eq!(a["unfiltered"], true);
            pins.insert(p.display().to_string(), hash(&p));
        }
        let f = read(r(format!("frozen-{label}.json")));
        let flags = fs::read_to_string(r(format!("{label}-flags.txt"))).unwrap();
        assert_eq!(f["rustflags"], flags);
        assert_eq!(flags.contains("profile-use="), label == "candidate");
        assert!(!flags.contains("profile-generate="));
        for (p, h) in f["input_pins"].as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
            pins.insert(p.clone(), h.as_str().unwrap().into());
        }
    }
    let functions = read(r("profile-functions.json"));
    let mut hot = vec![];
    for (raw, p) in functions.as_object().unwrap() {
        let n = p["demangled"].as_str().unwrap();
        if p["max_block_count"].as_u64().unwrap() > 0
            && (n.starts_with("zopro_core::msm::stream_g1_impl")
                || n.starts_with("zopro_core::msm::g2_borrowed::apply")
                || n.starts_with("zopro_core::msm_affine::borrowed_pending::balanced::feed")
                || n == "<zopro_zolana::Prover>::prove")
        {
            hot.push(json!({"raw":raw,"profile":p}));
        }
    }
    for prefix in [
        "zopro_core::msm::stream_g1_impl",
        "zopro_core::msm::g2_borrowed::apply",
        "zopro_core::msm_affine::borrowed_pending::balanced::feed",
        "<zopro_zolana::Prover>::prove",
    ] {
        assert!(hot.iter().any(|p| p["profile"]["demangled"]
            .as_str()
            .unwrap()
            .starts_with(prefix)));
    }
    let mut warnings = vec![];
    let mut hot_missing = vec![];
    let mut mismatches = vec![];
    for step in [
        "candidate-field",
        "candidate-core",
        "candidate-prover",
        "libraries-candidate",
        "candidate",
    ] {
        let p = r(format!("native-use/{step}/stderr.log"));
        let text = fs::read_to_string(&p).unwrap();
        pins.insert(p.display().to_string(), hash(&p));
        for line in text.lines() {
            if line.contains("no profile data available") {
                warnings.push(json!({"step":step,"line":line}));
                if step == "libraries-candidate" || step == "candidate" {
                    for h in &hot {
                        let raw = h["raw"].as_str().unwrap().rsplit(';').next().unwrap();
                        if line.contains(raw) {
                            hot_missing.push(json!({"step":step,"line":line,"hot_profile":h}));
                        }
                    }
                }
            }
            if line.contains("profile data may be out of date") || line.contains("hash mismatch") {
                mismatches.push(json!({"step":step,"line":line}));
            }
        }
    }
    assert!(hot_missing.is_empty());
    assert!(mismatches.is_empty());
    fs::create_dir(r("codegen")).unwrap();
    let mut variants = vec![];
    for label in ["accepted", "control", "candidate"] {
        let binary = match label {
            "accepted" => PathBuf::from(
                "/home/ubuntu/src/zopro-perf-work/balanced-kh-production/native/candidate-sdk",
            ),
            "control" => r("native/control-sdk"),
            _ => r("native-use/candidate-sdk"),
        };
        let o = Command::new("nm")
            .args(["-S", "--defined-only"])
            .arg(&binary)
            .output()
            .unwrap();
        assert!(o.status.success());
        let nm = String::from_utf8(o.stdout).unwrap();
        assert!(!nm.contains("__llvm_profile") && !nm.contains("__llvm_prf"));
        let p = r(format!("codegen/{label}-symbols.txt"));
        fs::write(&p, &nm).unwrap();
        pins.insert(p.display().to_string(), hash(&p));
        let o = Command::new("readelf")
            .args(["-SW"])
            .arg(&binary)
            .output()
            .unwrap();
        assert!(o.status.success());
        let sections = String::from_utf8(o.stdout).unwrap();
        assert!(!sections.contains("llvm_prf"));
        let p = r(format!("codegen/{label}-sections.txt"));
        fs::write(&p, sections).unwrap();
        pins.insert(p.display().to_string(), hash(&p));
        let mut selected = vec![];
        for line in nm.lines() {
            let w: Vec<_> = line.split_whitespace().collect();
            if w.len() != 4 || !matches!(w[2], "T" | "t") {
                continue;
            }
            let pretty = rustc_demangle::try_demangle(w[3])
                .map(|d| format!("{d:#}"))
                .unwrap_or_else(|_| w[3].into());
            if !pretty.starts_with("zopro_core::msm::stream_g1_impl")
                && !pretty.starts_with("zopro_core::msm::g2_borrowed::apply")
                && !pretty.starts_with("zopro_core::msm_affine::borrowed_pending::balanced::feed")
                && pretty != "<zopro_zolana::Prover>::prove"
            {
                continue;
            }
            let address = u64::from_str_radix(w[0], 16).unwrap();
            let size = u64::from_str_radix(w[1], 16).unwrap();
            let o = Command::new("objdump")
                .args(["-d", "-C", "--no-show-raw-insn"])
                .arg(format!("--start-address=0x{address:x}"))
                .arg(format!("--stop-address=0x{:x}", address + size))
                .arg(&binary)
                .output()
                .unwrap();
            assert!(o.status.success());
            let p = r(format!("codegen/{label}-{}.asm", selected.len()));
            fs::write(&p, o.stdout).unwrap();
            pins.insert(p.display().to_string(), hash(&p));
            selected.push(json!({"raw":w[3],"demangled":pretty,"bytes":size,"assembly":p}));
        }
        let h = hash(&binary);
        pins.insert(binary.display().to_string(), h.clone());
        variants.push(
            json!({"variant":label,"binary":binary,"sha256":h,"selected_functions":selected}),
        );
    }
    assert_ne!(variants[1]["sha256"], variants[2]["sha256"]);
    let a = json!({"passed":true,"variants":variants,"pins":pins,"hot_profile_records_nonzero":hot,"hot_records_missing_in_consumer_warnings":hot_missing,"mismatch_warnings":mismatches,"all_missing_function_warnings":warnings,"USE_profiling_instrumentation_absent":true,"test_and_driver_unseen_functions_retained":true,"warning_checks_are_not_full_optimization_remark_proof":true,"static_code_not_speed_evidence":true});
    fs::write(r("code-audit.json"), serde_json::to_vec_pretty(&a).unwrap()).unwrap();
    println!("SDKcodeauditPASS;hotG1G2KHSDKrecordsnonzero,no matchinghotmissing/mismatchwarnings;noinstrumentation inall3latencySDKs;allmissingwarnings retained");
}
