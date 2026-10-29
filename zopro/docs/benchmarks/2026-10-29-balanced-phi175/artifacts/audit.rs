use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/balanced-phi175";
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
    let native = read(r("native/result.json"));
    assert_eq!(native["status"], "complete");
    assert_eq!(native["steps"].as_array().unwrap().len(), 10);
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("build-pins.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    for label in ["control", "candidate"] {
        let t = read(r(format!("native/{label}-core/tests.json")));
        assert_eq!(t["passed_tests"].as_array().unwrap().len(), 127);
        assert_eq!(t["unfiltered"], true);
        assert!(t["passed_tests"].as_array().unwrap().contains(&json!(
            "msm_affine::g2_borrowed::balanced::tests::compiled_dispatch_matches_policy"
        )));
        let f = read(r(format!("frozen-{label}.json")));
        assert_eq!(
            f["rustflags"]
                .as_str()
                .unwrap()
                .contains("--cfg zopro_balanced_phi"),
            label == "candidate"
        );
        for (p, h) in f["input_pins"].as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
            pins.insert(p.clone(), h.as_str().unwrap().into());
        }
    }
    fs::create_dir(r("codegen")).unwrap();
    let mut variants = vec![];
    for label in ["accepted", "control", "candidate"] {
        let binary = if label == "accepted" {
            PathBuf::from(
                "/home/ubuntu/src/zopro-perf-work/balanced-kh-production/native/candidate-sdk",
            )
        } else {
            r(format!("native/{label}-sdk"))
        };
        let o = Command::new("nm")
            .args(["-S", "--size-sort", "--defined-only", "-C"])
            .arg(&binary)
            .output()
            .unwrap();
        assert!(o.status.success());
        let nm = String::from_utf8(o.stdout).unwrap();
        let to = r(format!("codegen/{label}-symbols.txt"));
        fs::write(&to, &nm).unwrap();
        pins.insert(to.display().to_string(), hash(&to));
        let mut symbols = vec![];
        let mut balanced = 0;
        for line in nm.lines() {
            let w: Vec<_> = line.split_whitespace().collect();
            if w.len() < 4 || !matches!(w[2], "t" | "T") {
                continue;
            }
            let name = w[3..].join(" ");
            let is_balanced = name.contains("zopro_core::msm_affine::g2_borrowed::balanced::");
            if !is_balanced && !name.starts_with("zopro_core::msm_affine::g2_phi::") {
                continue;
            }
            if is_balanced {
                balanced += 1;
            }
            let address = u64::from_str_radix(w[0], 16).unwrap();
            let bytes = u64::from_str_radix(w[1], 16).unwrap();
            let o = Command::new("objdump")
                .args(["-d", "-C", "--no-show-raw-insn"])
                .arg(format!("--start-address=0x{address:x}"))
                .arg(format!("--stop-address=0x{:x}", address + bytes))
                .arg(&binary)
                .output()
                .unwrap();
            assert!(o.status.success());
            let p = r(format!("codegen/{label}-{}.asm", symbols.len()));
            fs::write(&p, o.stdout).unwrap();
            pins.insert(p.display().to_string(), hash(&p));
            symbols.push(json!({"symbol":name,"bytes":bytes,"balanced_generic_or_defining_context":is_balanced,"assembly":p}));
        }
        assert_eq!(balanced > 0, label == "candidate");
        let h = hash(&binary);
        pins.insert(binary.display().to_string(), h.clone());
        variants.push(
            json!({"variant":label,"binary":binary,"sha256":h,"selected_text_symbols":symbols}),
        );
    }
    assert_ne!(variants[1]["sha256"], variants[2]["sha256"]);
    for name in ["zopro_core", "zopro_prover"] {
        assert_ne!(
            read(r("frozen-control.json"))["libraries"][name],
            read(r("frozen-candidate.json"))["libraries"][name]
        );
    }
    let result = json!({"passed":true,"pins":pins,"variants":variants,"cfg_and_dispatch_test_receipts_checked":true,"only_ON_linked_balanced_contexts":true,"symbol_context_not_exclusive_profile_attribution":true,"static_code_not_speed_evidence":true});
    fs::write(
        r("code-audit.json"),
        serde_json::to_vec_pretty(&result).unwrap(),
    )
    .unwrap();
    println!("Balanced phi linked audit passed;actualcfg/libs/testdispatch andON-onlybalanced text contexts");
}
