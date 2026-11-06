use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path, process::Command};
const R: &str = "/home/ubuntu/src/zopro-perf-work/split-redc-native176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const OLD: &str = "/home/ubuntu/src/zopro-perf-work/inverse-root-codec176";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(f: &str, v: &Value) {
    fs::write(format!("{R}/{f}"), serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn close(s: &str, start: usize) -> usize {
    let b = s.as_bytes();
    assert_eq!(b[start], b'{');
    let mut depth = 0;
    for (i, &c) in b.iter().enumerate().skip(start) {
        if c == b'{' {
            depth += 1;
        }
        if c == b'}' {
            depth -= 1;
            if depth == 0 {
                return i + 1;
            }
        }
    }
    panic!("unclosed braces")
}
fn method(s: &str, needle: &str) -> String {
    let i = s.find(needle).unwrap();
    let b = s[i..].find('{').unwrap() + i;
    s[i..close(s, b)].to_string()
}
fn replace(v: &Value) -> Value {
    match v {
        Value::String(s) => json!(s.replace(OLD, R)),
        Value::Array(a) => json!(a.iter().map(replace).collect::<Vec<_>>()),
        Value::Object(o) => json!(o
            .iter()
            .map(|(k, v)| (k.clone(), replace(v)))
            .collect::<BTreeMap<_, _>>()),
        _ => v.clone(),
    }
}
fn step(name: &str, args: Vec<String>) -> Value {
    json!({"kind":"command","name":name,"argv":args,"timeout_seconds":600})
}
fn prepare() {
    assert!(!Path::new(R).join("source").exists());
    let oldplan = replace(&read(format!("{OLD}/build-plan.json")));
    let templates = oldplan["steps"].as_array().unwrap();
    let test = templates
        .iter()
        .find(|s| s["name"] == "build-codec-tests")
        .unwrap();
    let env = test["environment"].clone();
    let manifest=read(format!("{Z}/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));
    let files = manifest["source_files_sha256"].as_object().unwrap();
    assert_eq!(files.len(), 176);
    let oldrows =
        "            row!(0);\n            row!(1);\n            row!(2);\n            row!(3);\n";
    let newrows = fs::read_to_string(format!("{R}/first-four-rows.rs")).unwrap();
    let gen_before="source = subprocess.check_output([\"rustfmt\", \"--emit\", \"stdout\", \"--edition\", \"2021\"], input=source, text=True)";
    let gen_after=format!("source += '\\n#[cfg(test)]\\n#[path = \"split_redc_tests.rs\"]\\nmod split_redc_tests;\\n'\n{gen_before}");
    for (name, h) in files {
        let original = Path::new(Z).join(name);
        assert_eq!(hash(&original), h.as_str().unwrap());
        let mut bytes = fs::read(&original).unwrap();
        if name == "crates/zopro-bn254/src/ifma.rs" {
            let s = String::from_utf8(bytes).unwrap();
            assert_eq!(s.matches(oldrows).count(), 1);
            bytes = s.replacen(oldrows, &newrows, 1).into_bytes();
        }
        if name == "scripts/ifma8.py" {
            let s = String::from_utf8(bytes).unwrap();
            assert_eq!(s.matches(gen_before).count(), 1);
            bytes = s.replacen(gen_before, &gen_after, 1).into_bytes();
        }
        let out = Path::new(R).join("source").join(name);
        fs::create_dir_all(out.parent().unwrap()).unwrap();
        fs::write(out, bytes).unwrap();
    }
    let original8 = fs::read_to_string(format!("{Z}/crates/zopro-bn254/src/ifma8.rs")).unwrap();
    let retained = method(&original8, "    #[inline(always)]\n    fn product_cios").replacen(
        "fn product_cios",
        "fn retained",
        1,
    );
    let diff = method(&original8, "    #[inline(always)]\n    pub fn difference2")
        .replacen("pub fn difference2", "fn retained_difference2", 1)
        .replace("self.product::<true>", "self.retained::<true>");
    assert!(diff.contains("self.retained::<true>"));
    let body = fs::read_to_string(format!("{R}/tests-body.rs")).unwrap();
    let tests=format!("use super::*;\nimpl<C:FpConfig,const MIXED:bool> Ifma8<C,MIXED>{{\n{retained}\n{diff}\n}}\n{body}\n");
    fs::write(
        format!("{R}/source/crates/zopro-bn254/src/split_redc_tests.rs"),
        tests,
    )
    .unwrap();
    let generated = Command::new("/usr/bin/python3")
        .arg(format!("{R}/source/scripts/ifma8.py"))
        .env("PATH", env["PATH"].as_str().unwrap())
        .output()
        .unwrap();
    fs::write(format!("{R}/generator.stdout.log"), generated.stdout).unwrap();
    fs::write(format!("{R}/generator.stderr.log"), generated.stderr).unwrap();
    assert!(generated.status.success());
    let mut pins = BTreeMap::new();
    let mut hashes = BTreeMap::new();
    let mut exact = 0;
    for (name, h) in files {
        let path = Path::new(R).join("source").join(name);
        let bytes = fs::read(&path).unwrap();
        let original = fs::read(Path::new(Z).join(name)).unwrap();
        exact += (bytes == original) as usize;
        let mut recovered = bytes.clone();
        if name == "crates/zopro-bn254/src/ifma.rs" {
            recovered = String::from_utf8(recovered)
                .unwrap()
                .replacen(&newrows, oldrows, 1)
                .into_bytes();
        } else if name == "scripts/ifma8.py" {
            recovered = String::from_utf8(recovered)
                .unwrap()
                .replacen(&gen_after, gen_before, 1)
                .into_bytes();
        } else if name == "crates/zopro-bn254/src/ifma8.rs" {
            let mut s = String::from_utf8(recovered).unwrap();
            let suffix =
                "\n#[cfg(test)]\n#[path = \"split_redc_tests.rs\"]\nmod split_redc_tests;\n";
            assert!(s.ends_with(suffix));
            s.truncate(s.len() - suffix.len());
            let start = s
                .find("            if cfg!(zopro_split_redc_planes) && LANES == 8 {")
                .unwrap();
            let if_brace = s[start..].find('{').unwrap() + start;
            let if_end = close(&s, if_brace);
            assert!(s[if_end..].starts_with(" else {"));
            let else_brace = if_end + 6;
            assert_eq!(s.as_bytes()[else_brace], b'{');
            let end = close(&s, else_brace);
            assert_eq!(s.as_bytes()[end], b'\n');
            s.replace_range(start..end + 1, oldrows);
            recovered = s.into_bytes();
        }
        assert_eq!(recovered, original, "reverse recovery {name}");
        assert_eq!(
            format!("{:x}", Sha256::digest(&recovered)),
            h.as_str().unwrap()
        );
        pins.insert(path.display().to_string(), hash(&path));
        hashes.insert(name.clone(), hash(&path));
    }
    assert_eq!(exact, 173);
    let child = format!("{R}/source/crates/zopro-bn254/src/split_redc_tests.rs");
    pins.insert(child.clone(), hash(&child));
    hashes.insert(
        "crates/zopro-bn254/src/split_redc_tests.rs".into(),
        hash(&child),
    );
    let mut steps = vec![
        json!({"kind":"command","name":"generator-check","argv":["/usr/bin/python3",format!("{R}/source/scripts/ifma8.py"),"--check"],"environment":env,"timeout_seconds":120}),
    ];
    let required = [
        "ifma8::split_redc_tests::fq_mixed",
        "ifma8::split_redc_tests::fq_uniform",
        "ifma8::split_redc_tests::fr_mixed",
        "ifma8::split_redc_tests::fr_uniform",
    ];
    for role in ["OFF", "ON"] {
        let mut build = test.clone();
        build["name"] = json!(format!("build-tests-{role}"));
        let a = build["argv"].as_array_mut().unwrap();
        let out = a.iter().position(|v| v == "-o").unwrap() + 1;
        a[out] = json!(format!("{R}/field-tests-{role}"));
        a.extend([json!("-C"), json!("overflow-checks=yes")]);
        if role == "ON" {
            a.extend([json!("--cfg"), json!("zopro_split_redc_planes")]);
        }
        steps.push(build);
        let mut run = step(
            &format!("tests-{role}"),
            vec![
                format!("{R}/field-tests-{role}"),
                "--nocapture".into(),
                "--test-threads=1".into(),
            ],
        );
        if role == "OFF" {
            run["argv"]
                .as_array_mut()
                .unwrap()
                .push(json!("ifma8::split_redc_tests::"));
        } else {
            run["unfiltered"] = json!(true);
        }
        run["required_tests"] = json!(required);
        run["environment"] = env.clone();
        steps.push(run);
    }
    for role in ["OFF", "ON"] {
        fs::create_dir_all(format!("{R}/field/{role}")).unwrap();
        let mut lib = templates
            .iter()
            .find(|s| s["name"] == "field-library")
            .unwrap()
            .clone();
        lib["name"] = json!(format!("field-{role}"));
        let a = lib["argv"].as_array_mut().unwrap();
        let out = a.iter().position(|v| v == "-o").unwrap() + 1;
        a[out] = json!(format!("{R}/field/{role}/libzopro_bn254.rlib"));
        if role == "ON" {
            a.extend([json!("--cfg"), json!("zopro_split_redc_planes")]);
        }
        steps.push(lib);
        steps.push(step(
            &format!("freeze-{role}"),
            vec![format!("{R}/setup"), "freeze".into(), role.into()],
        ));
    }
    for s in &steps {
        for a in s["argv"].as_array().unwrap() {
            let a = a.as_str().unwrap();
            let path = a
                .split_once('=')
                .filter(|(k, _)| {
                    matches!(
                        *k,
                        "dependency" | "ark_ref" | "ark_ec" | "ark_ff" | "ark_std"
                    )
                })
                .map(|(_, v)| v)
                .unwrap_or(a);
            if Path::new(path).is_file() {
                pins.insert(path.to_string(), hash(path));
            }
        }
    }
    for f in [
        "setup.rs",
        "setup",
        "first-four-rows.rs",
        "tests-body.rs",
        "design.md",
    ] {
        let p = format!("{R}/{f}");
        pins.insert(p.clone(), hash(&p));
    }
    for f in ["model.json", "review.json", "design.md", "bounds.md"] {
        let p = format!("/home/ubuntu/src/zopro-perf-work/split-redc-planes176/{f}");
        pins.insert(p.clone(), hash(&p));
    }
    save(
        "source-audit.json",
        &json!({"source_files":177,"exact_originals":exact,"recoverable_originals":176,"hashes":hashes,"runtime_changes":"only first four eight-lane CIOS rows under private cfg; final row/normalization/correction and all other runtime source original","test_reference":"original product_cios/difference2 bodies cloned test-only"}),
    );
    save("build-pins.json", &json!(pins));
    save(
        "build-plan.json",
        &json!({"version":1,"source_pins":format!("{R}/build-pins.json"),"environment":{},"steps":steps}),
    );
    println!(
        "prepared177sources/173exact/all176recoverable; {}commands/{}pins; no clock",
        steps.len(),
        pins.len()
    );
}
fn freeze(role: &str) {
    assert!(["OFF", "ON"].contains(&role));
    let src = format!("{R}/field/{role}/libzopro_bn254.rlib");
    let h = hash(&src);
    let dir = format!("{R}/frozen/{role}");
    fs::create_dir_all(&dir).unwrap();
    let dest = format!("{dir}/libzopro_bn254-{h}.rlib");
    assert!(!Path::new(&dest).exists());
    fs::copy(&src, &dest).unwrap();
    assert_eq!(hash(&dest), h);
    save(
        &format!("field-{role}.json"),
        &json!({"role":role,"source":src,"frozen":dest,"sha256":h,"bytes":fs::metadata(&dest).unwrap().len()}),
    );
    println!("frozen {role} field {h}");
}
fn main() {
    let a = std::env::args().collect::<Vec<_>>();
    match a.get(1).map(String::as_str) {
        Some("prepare") => prepare(),
        Some("freeze") => freeze(&a[2]),
        _ => panic!("prepare|freeze"),
    }
}
