use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn main() {
    let r = Path::new("/tmp/zopro-clean-fft-qualification");
    let original = read(r.join("portable-plan.json"));
    let mut link = original["steps"]
        .as_array()
        .unwrap()
        .last()
        .unwrap()
        .clone();
    assert_eq!(link["kind"], "link-sdk");
    assert!(!Path::new(link["driver"].as_str().unwrap()).exists());
    let manifest = read("/tmp/zopro-composed-adoption/production-source-manifest.json");
    link["driver"] = manifest["standard_cli_source"].clone();
    link["additional_sources"] = json!([manifest["standard_cli_helper"]]);
    assert_eq!(
        hash(link["driver"].as_str().unwrap()),
        manifest["standard_cli_source_sha256"].as_str().unwrap()
    );
    assert_eq!(
        hash(link["additional_sources"][0].as_str().unwrap()),
        manifest["standard_cli_helper_sha256"].as_str().unwrap()
    );
    for (name, n) in [("field", 11), ("core", 99), ("prover", 8)] {
        assert_eq!(
            read(r.join(format!("portable/{name}/tests.json")))["passed_tests"]
                .as_array()
                .unwrap()
                .len(),
            n
        );
    }
    save(
        r.join("portable-link-repair.json"),
        &json!({"failure":"initial portable workflow reached link-sdk with nonexistent snapshot-local standard driver/support; source manifest intentionally keeps standard CLI external","successful_steps_repeated":0,"SDK_requests_before_failure":0,"source_or_native_library_changed":false,"repair":"single link-only workflow using exact manifest-pinned standard CLI/support and existing portable Cargo artifact log","original_plan":"portable-plan.json","original_log":"portable.log","original_tests":"portable/field,core,prover/tests.json","new_output":"portable-linked/portable/artifacts.json"}),
    );
    let mut pins = read(r.join("build-pins.json"));
    for name in [
        "portable-plan.json",
        "portable.log",
        "portable/result.json",
        "repair-portable-link.rs",
        "portable-link-repair.json",
    ] {
        let p = r.join(name);
        pins[p.display().to_string()] = json!(hash(p));
    }
    save(r.join("portable-link-pins.json"), &pins);
    save(
        r.join("portable-link-plan.json"),
        &json!({"version":1,"source_pins":r.join("portable-link-pins.json"),"steps":[link]}),
    );
    let path = r.join("qualify.rs");
    let mut q = fs::read_to_string(&path).unwrap();
    let before="    for n in [\"native\", \"portable\"] {\n        assert_eq!(read(r.join(n).join(\"result.json\"))[\"status\"], \"complete\")\n    }";
    let after="    for n in [\"native\", \"portable-linked\"] {\n        assert_eq!(read(r.join(n).join(\"result.json\"))[\"status\"], \"complete\")\n    }";
    assert_eq!(q.matches(before).count(), 1);
    q = q.replace(before, after).replace(
        "portable/portable/artifacts.json",
        "portable-linked/portable/artifacts.json",
    );
    let before = "        \"portable/result.json\",";
    let after="        \"portable/result.json\",\n        \"portable-linked/result.json\",\n        \"portable-link-repair.json\",\n        \"portable-link-plan.json\",\n        \"repair-portable-link.rs\",";
    assert_eq!(q.matches(before).count(), 1);
    q = q.replace(before, after);
    fs::write(path, q).unwrap();
    println!("Prepared single portable link repair;11field99core8prover retained;no successful step repeated");
}
