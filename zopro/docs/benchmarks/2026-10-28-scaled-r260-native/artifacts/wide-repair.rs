use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/scaled-r260-native";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn once(s: &mut String, a: &str, b: &str) {
    assert_eq!(s.matches(a).count(), 1, "{a}");
    *s = s.replace(a, b);
}
fn prepare() {
    assert_eq!(
        read(r("linked/failure.json"))["completed"]
            .as_array()
            .unwrap()
            .len(),
        9
    );
    assert!(!r("run-plan.json").exists());
    let pins = read(r("link-pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    fs::create_dir(r("initial-numeric-type-failure")).unwrap();
    for f in [
        "wide-check.rs",
        "driver.rs",
        "driver",
        "link-plan.json",
        "link-pins.json",
    ] {
        fs::copy(r(f), r("initial-numeric-type-failure").join(f)).unwrap();
    }
    let mut s = fs::read_to_string(r("wide-check.rs")).unwrap();
    once(&mut s, "            &p,", "            p.clone(),");
    fs::write(r("wide-check.rs"), s).unwrap();
    let mut d = fs::read_to_string(r("driver.rs")).unwrap();
    once(&mut d,"    assert_eq!(read(r(\"linked/result.json\"))[\"status\"], \"complete\");","    assert_eq!(read(r(\"numeric-linked/result.json\"))[\"status\"], \"complete\");\n    assert_eq!(read(r(\"linked/failure.json\"))[\"completed\"].as_array().unwrap().len(),9);");
    once(
        &mut d,
        "serde_json::from_value(read(r(\"link-pins.json\"))).unwrap();",
        "serde_json::from_value(read(r(\"link-pins-v2.json\"))).unwrap();",
    );
    once(&mut d,"for p in [\"link-plan.json\", \"link-pins.json\", \"linked/result.json\"]","for p in [\"link-plan.json\",\"link-pins.json\",\"link-plan-v2.json\",\"link-pins-v2.json\",\"numeric-linked/result.json\",\"linked/failure.json\",\"numeric-repair.json\"]");
    assert_eq!(d.matches("linked/wide-tests/stdout.log").count(), 2);
    d = d.replace(
        "linked/wide-tests/stdout.log",
        "numeric-linked/wide-tests/stdout.log",
    );
    fs::write(r("driver.rs"), d).unwrap();
    save(
        r("numeric-repair.json"),
        &json!({"failure":"E0308 borrowed BigUint p in a Vec<BigUint> test fixture","fix":"p.clone() at that one fixture entry","native_arithmetic_changed":false,"native_suites_or_successful_harness_tests_repeated":false,"completed_original_link_steps":9,"old_protocol_unchanged":true,"original_sources_pins_preserved":"initial-numeric-type-failure","new_plan":"Only the uncompleted numeric harness build/test, followed by audited prepare; no SDK or latency yet","driver_change":"prepare accepts original9 successful steps plus numeric-linked receipt and revised pins"}),
    );
}
fn ready() {
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("link-pins.json"))).unwrap();
    for (p, h) in &mut pins {
        if [r("wide-check.rs"), r("driver.rs"), r("driver")]
            .iter()
            .any(|v| v == Path::new(p))
        {
            *h = hash(p.as_str());
        } else {
            assert_eq!(hash(p.as_str()), *h);
        }
    }
    for f in [
        "wide-repair.rs",
        "wide-repair",
        "numeric-repair.json",
        "initial-numeric-type-failure/wide-check.rs",
        "initial-numeric-type-failure/driver.rs",
        "initial-numeric-type-failure/driver",
        "initial-numeric-type-failure/link-pins.json",
        "initial-numeric-type-failure/link-plan.json",
        "linked/failure.json",
    ] {
        pins.insert(r(f).display().to_string(), hash(r(f)));
    }
    let p = read(r("link-plan.json"));
    let steps = p["steps"].as_array().unwrap();
    assert_eq!(steps.len(), 11);
    assert_eq!(steps[9]["name"], "build-wide-tests");
    assert_eq!(steps[10]["name"], "wide-tests");
    save(r("link-pins-v2.json"), &json!(pins));
    save(
        r("link-plan-v2.json"),
        &json!({"version":1,"source_pins":r("link-pins-v2.json"),"steps":&steps[9..]}),
    );
    println!("Preparedonly2uncompletednumericsteps;9successes/nativeunchanged;no capturesyet");
}
fn main() {
    match std::env::args().nth(1).unwrap().as_str() {
        "prepare" => prepare(),
        "ready" => ready(),
        _ => panic!("mode"),
    }
}
