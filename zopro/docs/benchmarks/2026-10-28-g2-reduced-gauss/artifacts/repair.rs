use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-reduced-gauss";
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
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn links() {
    assert!(!r("relink-plan.json").exists());
    let before = fs::read_to_string(r("initial-argument-failure/probe.rs")).unwrap();
    let after = fs::read_to_string(r("probe.rs")).unwrap();
    assert_eq!(
        after.replace(
            "(a[1] == \"g2-prefix\" && [32, 200].contains(&n))",
            "(a[1] == \"g2-prefix\" && n == 200)"
        ),
        before
    );
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("link-pins.json"))).unwrap();
    for (p, h) in &pins {
        if p != &r("probe.rs").display().to_string() {
            assert_eq!(hash(p), *h);
        }
    }
    pins.insert(r("probe.rs").display().to_string(), hash(r("probe.rs")));
    save(r("relink-pins.json"), &json!(pins));
    let plan = read(r("link-plan.json"));
    let steps: Vec<_> = plan["steps"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|s| {
            s["name"].as_str().unwrap().starts_with("link-")
                && !s["name"].as_str().unwrap().ends_with("-tests")
        })
        .cloned()
        .collect();
    assert_eq!(steps.len(), 3);
    save(
        r("relink-plan.json"),
        &json!({"version":1,"source_pins":r("relink-pins.json"),"steps":steps}),
    );
    let code = fs::read_to_string(r("driver.rs")).unwrap();
    assert_eq!(code.matches("read(r(\"pins.json\"))").count(), 1);
    fs::write(
        r("offline.rs"),
        code.replace("read(r(\"pins.json\"))", "read(r(\"pins-v2.json\"))"),
    )
    .unwrap();
    println!(
        "Prepared3benchmark-onlylinks;tests/librariesunchanged;oneharnessadmissioncheckextended"
    );
}
fn ready() {
    assert_eq!(read(r("relinked/result.json"))["status"], "complete");
    assert!(read(r("runs/failure.json"))["completed"]
        .as_array()
        .unwrap()
        .is_empty());
    assert!(!r("pins-v2.json").exists());
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("initial-argument-failure/pins.json"))).unwrap();
    let mut changed = vec![];
    for (p, h) in &mut pins {
        let actual = hash(p);
        if &actual != h {
            let rel = Path::new(p).strip_prefix(R).unwrap().to_str().unwrap();
            assert!(
                ["probe.rs", "frozen-probe", "off-probe", "on-probe"].contains(&rel),
                "unexpectedchangedpin {p}"
            );
            changed.push(json!({"path":p,"before":h,"after":actual}));
            *h = actual;
        }
    }
    assert_eq!(changed.len(), 4);
    let mut code = vec![];
    for variant in ["off", "on"] {
        let binary = r(format!("{variant}-probe"));
        let out = Command::new("nm")
            .arg("--defined-only")
            .arg(&binary)
            .output()
            .unwrap();
        assert!(out.status.success());
        let text = String::from_utf8(out.stdout).unwrap();
        let mut count = 0;
        for (i, line) in text.lines().filter(|s| s.contains("g2_prefix")).enumerate() {
            let symbol = line.split_whitespace().last().unwrap();
            let out = Command::new("objdump")
                .args(["-d", "--no-show-raw-insn"])
                .arg(format!("--disassemble={symbol}"))
                .arg(&binary)
                .output()
                .unwrap();
            assert!(out.status.success());
            let text = String::from_utf8(out.stdout).unwrap();
            let ifma = text.matches("vpmadd52").count();
            assert!(ifma > 0);
            count += 1;
            let p = r(format!("codegen-{variant}-{i}.asm.txt"));
            fs::write(&p, &text).unwrap();
            pins.insert(p.display().to_string(), hash(&p));
            code.push(json!({"variant":variant,"symbol":symbol,"ifma_sites":ifma,"path":p,"actual_executable_sha256":hash(&binary)}));
        }
        assert!(count > 0);
    }
    save(
        r("codegen.json"),
        &json!({"read_only_objdump":true,"actual_G2_prefix_symbols":code,"static_counts_not_executed_work":true,"after_argument_only_relink":true}),
    );
    pins.insert(
        r("codegen.json").display().to_string(),
        hash(r("codegen.json")),
    );
    save(
        r("argument-repair.json"),
        &json!({"initial_failure":"runs-initial-argument-failure/failure.json","initial_completed_steps":0,"priming_or_timed_loops_before_failure":0,"change":"Only allow planned G2 length32 alongside200 at main argument check; native algorithms/libraries/tests/fixtures/barriers/timing/gates unchanged","changed_pins":changed,"new_links":"relinked/result.json","successful_core_tests_repeated":false,"successful_harness_tests_repeated":false,"successful_captures_repeated":false,"old_root_runs_relocated":"runs-initial-argument-failure","offline_analyzer_change":"reads final pins-v2 rather than initial pins; statistical/functional checks unchanged"}),
    );
    for p in [
        "repair.rs",
        "repair",
        "argument-repair.json",
        "offline.rs",
        "offline",
        "relink-plan.json",
        "relink-pins.json",
        "relinked/result.json",
        "initial-argument-failure/probe.rs",
        "initial-argument-failure/pins.json",
        "initial-argument-failure/run-plan.json",
        "initial-argument-failure/codegen.json",
    ] {
        pins.insert(r(p).display().to_string(), hash(r(p)));
    }
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    fs::rename(r("runs"), r("runs-initial-argument-failure")).unwrap();
    save(r("pins-v2.json"), &json!(pins));
    let mut plan = read(r("initial-argument-failure/run-plan.json"));
    plan["source_pins"] = json!(r("pins-v2.json"));
    save(r("run-plan-v2.json"), &plan);
    println!(
        "Preparedidentical837latency+8PMUplanwithrepairedharness;oldzero-capturefailurepreserved"
    );
}
fn main() {
    match std::env::args().nth(1).unwrap().as_str() {
        "links" => links(),
        "ready" => ready(),
        _ => panic!("mode"),
    }
}
