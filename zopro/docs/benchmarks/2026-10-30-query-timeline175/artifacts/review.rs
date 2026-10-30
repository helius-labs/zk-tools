use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs,
    io::Read,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/query-timeline175";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn r(p: &str) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
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
        h.update(&b[..n])
    }
    format!("{:x}", h.finalize())
}
fn main() {
    assert!(!r("runs/result.json").exists());
    let mut pins = read(r("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap(), "{p}");
    }
    let manifest = read(Path::new(Z).join(
        "docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json",
    ));
    let audit = read(r("source-audit.json"));
    assert_eq!(
        manifest["source_files_sha256"].as_object().unwrap().len(),
        175
    );
    let mut exact = 0;
    let mut field = 0;
    for (p, h) in manifest["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(Path::new(Z).join(p)), h.as_str().unwrap());
        let copy = r("source").join(p);
        if hash(&copy) == h.as_str().unwrap() {
            exact += 1
        } else {
            assert_eq!(p, "crates/zopro-core/src/lib.rs");
            let text = fs::read_to_string(copy).unwrap();
            let mut reversed = text
                .strip_suffix(audit["suffix"].as_str().unwrap())
                .unwrap()
                .to_string();
            for e in audit["edits"].as_array().unwrap().iter().rev() {
                let a = e["after"].as_str().unwrap();
                assert_eq!(reversed.matches(a).count(), 1);
                reversed = reversed.replace(a, e["before"].as_str().unwrap());
            }
            assert_eq!(
                format!("{:x}", Sha256::digest(reversed)),
                h.as_str().unwrap()
            );
        }
        if p.starts_with("crates/zopro-bn254/") {
            field += 1;
            assert_eq!(hash(r("source").join(p)), h.as_str().unwrap());
        }
    }
    assert_eq!(exact, 174);
    assert_eq!(field, 17);
    assert_eq!(
        hash(r("query_timeline.rs")),
        hash(r("source/crates/zopro-core/src/query_timeline.rs"))
    );
    for (suite, count) in [("control-core", 126), ("control-prover", 8)] {
        let tests = read(r(&format!("native/{suite}/tests.json")));
        assert_eq!(tests["unfiltered"], true);
        assert_eq!(tests["passed_tests"].as_array().unwrap().len(), count);
    }
    assert_eq!(read(r("native/result.json"))["status"], "complete");
    assert_eq!(read(r("sdk/runs/result.json"))["status"], "complete");
    let frozen = read(r("frozen-control.json"));
    assert_eq!(frozen["libraries"].as_object().unwrap().len(), 8);
    assert_eq!(
        frozen["rustflags"].as_str().unwrap(),
        fs::read_to_string(r("native-flags.txt")).unwrap()
    );
    assert_eq!(
        read(r("sdk/source-audit.json"))["original_driver_reversed_exact"],
        true
    );
    let jobs = read(r("jobs.json"));
    let plan = read(r("plan.json"));
    let js = jobs.as_array().unwrap();
    let steps = plan["steps"].as_array().unwrap();
    assert_eq!(js.len(), 288);
    assert_eq!(steps.len(), 576);
    assert_eq!(
        js.iter()
            .map(|j| j["rounds"].as_u64().unwrap() + 3)
            .sum::<u64>(),
        43002
    );
    let mut names = BTreeSet::new();
    for (i, j) in js.iter().enumerate() {
        let name = j["name"].as_str().unwrap();
        assert!(names.insert(name));
        let position = if i < 9 { i * 2 } else { 18 + i - 9 };
        assert_eq!(steps[position]["name"], name);
        assert_eq!(steps[position]["argv"][2], j["case"]["cpu_affinity"]);
        assert_eq!(
            steps[position]["environment"]["RAYON_NUM_THREADS"],
            j["case"]["workers"].as_u64().unwrap().to_string()
        );
        assert_eq!(
            steps[position]["environment"]["ZOPRO_QUERY_TIMELINE"],
            if j["variant"] == "traced" { "1" } else { "0" }
        );
        if j["variant"] == "accepted" {
            assert_eq!(
                hash(j["binary"].as_str().unwrap()),
                "20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f"
            );
        } else {
            assert_eq!(j["binary"], r("sdk/runs/timeline-sdk").to_str().unwrap());
        }
        if i < 9 {
            assert_eq!(j["panel"], "preflight");
            assert_eq!(j["rounds"], 1);
            assert_eq!(steps[position + 1]["name"], format!("verify-{name}"));
        } else {
            assert_eq!(j["panel"], "latency");
            assert_eq!(j["rounds"], 151);
            let verifypos = 297 + i - 9;
            assert_eq!(steps[verifypos]["name"], format!("verify-{name}"));
        }
    }
    for b in 0..31 {
        let part = &js[9 + 9 * b..18 + 9 * b];
        let mut seen = BTreeSet::new();
        for j in part {
            assert_eq!(j["block"], b);
            assert!(seen.insert((
                j["case"]["workers"].as_u64().unwrap(),
                j["variant"].as_str().unwrap()
            )));
        }
        assert_eq!(seen.len(), 9);
    }
    let out = json!({"review_passed":true,"source_files":176,"originals_exact":174,"all175_reversed":true,"field_files_exact":17,"native_core_tests":126,"native_prover_tests":8,"SDK_libraries":8,"same_OFF_traced_binary":true,"clock_loop_original_exact":true,"workflow_steps":576,"SDK_processes":288,"planned_SDK_requests":43002,"required_new_distinct_Gnark_exports":288,"prior_ledger":24943,"original8_CPU10x":0,"new_measurements":0,"production_changed":false,"capture_helpers_precompiled":true});
    fs::write(
        r("pre-capture-review.json"),
        serde_json::to_vec_pretty(&out).unwrap(),
    )
    .unwrap();
    pins[r("pre-capture-review.json").to_str().unwrap()] =
        json!(hash(r("pre-capture-review.json")));
    fs::write(r("pins.json"), serde_json::to_vec_pretty(&pins).unwrap()).unwrap();
    println!("Pre-capture review passed176sources/175reversed,126core/8prover,576steps/288processes; all pins close, no performance capture yet");
}
