use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::Read,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-sdk";
const P: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-queue";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const M:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut b = [0; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    format!("{:x}", h.finalize())
}
fn pin(p: &mut BTreeMap<String, String>, f: impl AsRef<Path>) {
    let f = f.as_ref();
    p.insert(f.display().to_string(), hash(f));
}
fn once(s: &mut String, a: &str, b: &str) {
    assert_eq!(s.matches(a).count(), 1, "{a}");
    *s = s.replace(a, b);
}
fn main() {
    assert!(!r("build-plan.json").exists());
    let m = read(M);
    let audit = read(format!("{P}/source-audit.json"));
    assert_eq!(audit["reverse_all172_exact"], true);
    assert_eq!(audit["private_originals_exact"], 168);
    let mut pins = BTreeMap::new();
    pin(&mut pins, M);
    let mut exact = 0;
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        let original = Path::new(Z).join(f);
        assert_eq!(hash(&original), h.as_str().unwrap());
        pin(&mut pins, original);
        let source = Path::new(P).join("source").join(f);
        pin(&mut pins, &source);
        let bytes = fs::read(&source).unwrap();
        if hash(&source) == h.as_str().unwrap() {
            exact += 1
        } else {
            let mut back = String::from_utf8(bytes.clone()).unwrap();
            if f == "crates/zopro-core/src/msm_affine/g2_phi.rs" {
                once(
                    &mut back,
                    audit["runtime_dispatch"]["new"].as_str().unwrap(),
                    audit["runtime_dispatch"]["old"].as_str().unwrap(),
                );
            }
            for s in audit["suffixes"]
                .as_array()
                .unwrap()
                .iter()
                .rev()
                .filter(|s| s["file"] == *f)
            {
                back = back
                    .strip_suffix(s["suffix"].as_str().unwrap())
                    .unwrap()
                    .to_string();
            }
            assert_eq!(back.as_bytes(), fs::read(Path::new(Z).join(f)).unwrap());
            let dest = r("source").join(f);
            fs::create_dir_all(dest.parent().unwrap()).unwrap();
            fs::write(&dest, bytes).unwrap();
            pin(&mut pins, dest);
        }
    }
    assert_eq!(exact, 168);
    for f in [
        "crates/zopro-core/src/msm/g2_borrowed.rs",
        "crates/zopro-core/src/msm_affine/g2_borrowed.rs",
    ] {
        let p = Path::new(P).join("source").join(f);
        let q = r("source").join(f);
        fs::create_dir_all(q.parent().unwrap()).unwrap();
        fs::copy(&p, &q).unwrap();
        pin(&mut pins, p);
        pin(&mut pins, q);
    }
    fs::copy(format!("{P}/source-audit.json"), r("source-audit.json")).unwrap();
    for p in [
        "source-audit.json",
        "native/result.json",
        "native/core-off/stdout.log",
        "native/core-off/tests.json",
        "native/core-on/stdout.log",
        "native/core-on/tests.json",
        "linked/heap-accounting-test/stdout.log",
        "linked/heap-accounting-test/tests.json",
        "analysis.json",
        "control-locations.json",
    ] {
        let p = Path::new(P).join(p);
        pin(&mut pins, &p);
    }
    for (label, n) in [("off", 119), ("on", 122)] {
        let t = fs::read_to_string(format!("{P}/native/core-{label}/stdout.log")).unwrap();
        assert!(t.contains(&format!("test result: ok. {n} passed; 0 failed")));
    }
    let driver = fs::read_to_string(format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs")).unwrap();
    let probe = fs::read_to_string(format!("{P}/probe.rs")).unwrap();
    let wrapper =
        probe[probe.find("struct Alloc;").unwrap()..probe.find("fn cpus(").unwrap()].to_string();
    let mut heap = driver.clone();
    once(&mut heap,"#[global_allocator]\nstatic ALLOCATOR: zopro_prover::zopro_core::alloc::Allocator =\n    zopro_prover::zopro_core::alloc::Allocator;",&format!("use std::alloc::{{GlobalAlloc,Layout}};\n{wrapper}"));
    once(
        &mut heap,
        "    let loop_started = Instant::now();",
        "    let heap_before=heap::reset();\n    let loop_started = Instant::now();",
    );
    once(
        &mut heap,
        "    let after = perf::Usage::now()?;",
        "    let memory=heap::report(heap_before);\n    let after = perf::Usage::now()?;",
    );
    once(&mut heap,"    println!(\"SUMMARY {summary}\");","    summary[\"heap\"]=memory;\n    summary[\"heap_instrumented\"]=true.into();\n    println!(\"SUMMARY {summary}\");");
    fs::write(r("heap-sdk.rs"), heap).unwrap();
    fs::create_dir_all(r("support")).unwrap();
    fs::copy(
        format!("{Z}/crates/zopro-bench/examples/support/perf.rs"),
        r("support/perf.rs"),
    )
    .unwrap();
    save(
        r("qualification-inheritance.json"),
        &json!({"native_core_off":119,"native_core_on":122,"new_native_tests":0,"source_originals_exact":168,"reversed172_exact":true,"field_unchanged":true,"parent_heap_wrapper_exact_sha256":format!("{:x}",Sha256::digest(wrapper.as_bytes())),"heap_wrapper_exact":true,"parent_heap_test_inherited":true,"native_libraries_rebuilt":false,"SDK_driver_uninstrumented_and_unchanged":true}),
    );
    let mut steps = vec![];
    for label in ["frozen", "off", "on"] {
        let a = read(format!("{P}/artifacts-{label}.json"));
        let expected = format!(
            "{} --cfg zopro_g2_queue_probe{}",
            m["rustflags"].as_str().unwrap(),
            if label == "on" {
                " --cfg zopro_g2_borrow_queue"
            } else {
                ""
            }
        );
        assert_eq!(a["rustflags"], expected);
        for (p, h) in a["input_pins"].as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
            pin(&mut pins, p);
        }
        for f in a["fingerprints"].as_array().unwrap() {
            let p = f["path"].as_str().unwrap();
            assert_eq!(read(p), f["receipt"]);
            pin(&mut pins, p);
        }
        let mut lines = String::new();
        for name in [
            "zopro_core",
            "zopro_bn254",
            "zopro_prover",
            "zopro_zolana",
            "memmap2",
            "serde_json",
            "libc",
            "rayon",
        ] {
            let p = a["libraries"][name].as_str().unwrap();
            assert_eq!(a["input_pins"][p], hash(p));
            lines.push_str(
                &serde_json::to_string(
                    &json!({"reason":"compiler-artifact","target":{"name":name},"filenames":[p]}),
                )
                .unwrap(),
            );
            lines.push('\n');
        }
        let artifacts = r(format!("libraries-{label}.jsonl"));
        fs::write(&artifacts, lines).unwrap();
        pin(&mut pins, &artifacts);
        pin(&mut pins, format!("{P}/artifacts-{label}.json"));
        save(r(format!("parent-artifacts-{label}.json")), &a);
        steps.push(json!({"kind":"link-sdk","name":label,"artifacts":artifacts,"driver":format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs"),"additional_sources":[format!("{Z}/crates/zopro-bench/examples/support/perf.rs")],"timeout_seconds":600}));
    }
    let accepted =
        read("/home/ubuntu/src/zopro-perf-work/mixed-sdk/native-control/control/artifacts.json");
    let mut lines = String::new();
    for (name, p) in accepted["libraries"].as_object().unwrap() {
        lines.push_str(
            &serde_json::to_string(
                &json!({"reason":"compiler-artifact","target":{"name":name},"filenames":[p]}),
            )
            .unwrap(),
        );
        lines.push('\n');
    }
    fs::write(r("libraries-accepted.jsonl"), lines).unwrap();
    pin(&mut pins, r("libraries-accepted.jsonl"));
    for (p, h) in accepted["input_pins"].as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
        pin(&mut pins, p);
    }
    save(r("accepted-artifacts.json"), &accepted);
    for label in ["accepted", "frozen", "off", "on"] {
        steps.push(json!({"kind":"link-sdk","name":format!("heap-{label}"),"artifacts":r(format!("libraries-{label}.jsonl")),"driver":r("heap-sdk.rs"),"additional_sources":[r("support/perf.rs")],"rustc_flags":["-O","-C","target-cpu=native","-C","lto=fat","-C","codegen-units=1","--cfg","zopro_queue_heap_probe"],"timeout_seconds":600}));
    }
    for name in ["control-sections", "control-locations"] {
        let mut s = fs::read_to_string(format!("{P}/{name}.rs")).unwrap();
        once(&mut s, "\"frozen-probe\"", "\"native/frozen-sdk\"");
        once(&mut s, "\"off-probe\"", "\"native/off-sdk\"");
        fs::write(r(format!("{name}.rs")), s).unwrap();
        pin(&mut pins, r(format!("{name}.rs")));
    }
    for p in [
        "design.md",
        "native.rs",
        "native",
        "source-audit.json",
        "heap-sdk.rs",
        "support/perf.rs",
        "qualification-inheritance.json",
    ] {
        pin(&mut pins, r(p));
    }
    pin(
        &mut pins,
        format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs"),
    );
    pin(
        &mut pins,
        format!("{Z}/crates/zopro-bench/examples/support/perf.rs"),
    );
    save(r("build-pins.json"), &json!(pins));
    save(
        r("build-plan.json"),
        &json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}),
    );
    println!("Prepared7SDKlinks from exact frozen libraries; source168exact/reverse172; native suites inherited119/122");
}
