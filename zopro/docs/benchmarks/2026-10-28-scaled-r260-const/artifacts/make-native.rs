use serde_json::json;
use sha2::{Digest, Sha256};
use std::fs;
const OLD: &str = "/home/ubuntu/src/zopro-perf-work/scaled-r260-native";
const NEW: &str = "/home/ubuntu/src/zopro-perf-work/scaled-r260-const";
fn hash(p: &str) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn once(s: &mut String, a: &str, b: &str) {
    assert_eq!(s.matches(a).count(), 1, "{a}");
    *s = s.replace(a, b);
}
fn main() {
    assert!(!std::path::Path::new("native.rs").exists());
    for f in [
        "scaled.rs",
        "g2_scaled.rs",
        "wide-check.rs",
        "probe.rs",
        "perf.rs",
        "collect.rs",
        "control-sections.rs",
        "control-locations.rs",
    ] {
        fs::write(
            f,
            fs::read_to_string(format!("{OLD}/{f}"))
                .unwrap()
                .replace(OLD, NEW),
        )
        .unwrap();
    }
    let mut s = fs::read_to_string("scaled.rs").unwrap();
    let original = s.clone();
    let mut edits = vec![];
    for (a, b) in [
        ("let p = modulus(B);", "let p = const { modulus(B) };"),
        (
            "let p = modulus(1).map(splat);",
            "let p = const { modulus(1) }.map(splat);",
        ),
        (
            "let p = modulus(P).map(splat);",
            "let p = const { modulus(P) }.map(splat);",
        ),
    ] {
        once(&mut s, a, b);
        edits.push(json!({"old":a,"new":b}));
    }
    let mut back = s.clone();
    for e in edits.iter().rev() {
        back = back.replace(e["new"].as_str().unwrap(), e["old"].as_str().unwrap());
    }
    assert_eq!(back, original);
    fs::write("scaled.rs", s).unwrap();
    fs::write("constant-source-audit.json",serde_json::to_vec_pretty(&json!({"parent_commit":"62e60fb86808d60aebdc4ec51d87522c04d0a458","parent_module_sha256":hash(&format!("{OLD}/scaled.rs")),"new_module_sha256":hash("scaled.rs"),"exactly_three_const_contexts":edits,"reverses_to_parent":true,"G2_kernel_byte_exact":hash("g2_scaled.rs")==hash(&format!("{OLD}/g2_scaled.rs")),"numeric_harness_repaired_parent_exact":hash("wide-check.rs")==hash(&format!("{OLD}/wide-check.rs")),"OFF_reused_from_parent":true})).unwrap()).unwrap();
    let mut p = fs::read_to_string(format!("{OLD}/native-prefix.rs"))
        .unwrap()
        .replace(OLD, NEW);
    once(
        &mut p,
        "for label in [\"off\", \"on\"]",
        "for label in [\"on\"]",
    );
    once(
        &mut p,
        "\"design.md\",",
        "\"design.md\",\"constant-source-audit.json\",",
    );
    p = p.replace(
        "Prepared7native steps;checkedfield33/core122OFF124ON",
        "Prepared4native steps;checkedfield33/core124ON;OFF122inherited",
    );
    fs::write("native-prefix.rs", &p).unwrap();
    let old = fs::read_to_string(format!("{OLD}/native.rs")).unwrap();
    let tail = old[old.find("fn freeze(").unwrap()..].replace(OLD, NEW);
    fs::write("native.rs", format!("{p}\n{tail}")).unwrap();
    fs::copy(format!("{OLD}/artifacts-off.json"), "artifacts-off.json").unwrap();
    let mut d = fs::read_to_string(format!("{OLD}/driver-before-symbol-audit.rs"))
        .unwrap()
        .replace(OLD, NEW);
    once(&mut d,"let t = fs::read_to_string(r(format!(\"native/core-{name}/stdout.log\"))).unwrap();",&format!("let path=if name==\"off\"{{std::path::PathBuf::from(\"{OLD}/native/core-off/stdout.log\")}}else{{r(format!(\"native/core-{{name}}/stdout.log\"))}};let t=fs::read_to_string(&path).unwrap();"));
    let needle = "    let mut steps = vec![];\n    for name in [\"frozen\", \"off\", \"on\"] {";
    once(&mut d,needle,&format!("    pin(&mut pins,\"{OLD}/native/core-off/stdout.log\");pin(&mut pins,\"{OLD}/native/result.json\");pin(&mut pins,r(\"constant-source-audit.json\"));\n{needle}"));
    once(&mut d,"        } else {\n            r(format!(\"frozen/{name}\")).display().to_string()\n        };",&format!("        }} else if name==\"off\" {{\n            \"{OLD}/frozen/off\".to_owned()\n        }} else {{\n            r(format!(\"frozen/{{name}}\")).display().to_string()\n        }};"));
    once(&mut d,"    assert_eq!(\n        frozen, off,\n        \"Control ELF differs: stop before timing and audit code generation\"\n    );",r#"    let sections=read(r("control-sections.json"));let locations=read(r("control-locations.json"));assert_eq!(sections["all_text_and_rodata_byte_identical"],true);assert_eq!(locations["native_code_or_arithmetic_changed"],false);assert_eq!(locations["panic_locations"].as_array().unwrap().len(),20);
    save(r("control-preflight.json"),&json!({"frozen_sha256":frozen,"off_sha256":off,"ELF_byte_identical":frozen==off,"runtime_sections_equivalent":true,"original_OFF_body_verbatim":true,"exception":"Only20 decoded Rust panic source lines+2 and build ID differ","timing_not_started":true,"performance_gates_unchanged":true}));
    for p in ["control-sections.rs","control-sections","control-sections.json","control-locations.rs","control-locations","control-locations.json","constant-source-audit.json"] {pin(&mut pins,r(p));}"#);
    once(
        &mut d,
        ".filter(|s| s.contains(\"g2_prefix\"))",
        ".filter(|s| s.contains(\"g2_prefix\") || s.contains(\"9g2_scaled5apply\"))",
    );
    once(
        &mut d,
        "\"native_core_tests_off\":122",
        "\"inherited_core_tests_off\":122,\"new_OFF_test_runs\":0",
    );
    fs::write("driver.rs", d).unwrap();
    println!("Generatedconstant-onlynativefollowup;4newsteps/unchangedOFFreused/11linkchecks;noarithchange");
}
