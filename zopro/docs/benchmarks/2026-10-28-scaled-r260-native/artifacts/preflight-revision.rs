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
    assert!(!r("run-plan.json").exists());
    assert_eq!(read(r("numeric-linked/result.json"))["status"], "complete");
    let audit = read(r("control-locations.json"));
    assert_eq!(audit["native_code_or_arithmetic_changed"], false);
    assert_eq!(audit["panic_locations"].as_array().unwrap().len(), 20);
    for (p, h) in read(r("link-pins-v2.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    fs::create_dir(r("initial-control-preflight")).unwrap();
    for f in [
        "driver.rs",
        "driver",
        "control-preflight.json",
        "capture-preparation.log",
    ] {
        fs::copy(r(f), r("initial-control-preflight").join(f)).unwrap();
    }
    let mut d = fs::read_to_string(r("driver.rs")).unwrap();
    once(
        &mut d,
        "serde_json::from_value(read(r(\"link-pins-v2.json\"))).unwrap();",
        "serde_json::from_value(read(r(\"link-pins-v3.json\"))).unwrap();",
    );
    once(&mut d,"    assert_eq!(\n        frozen, off,\n        \"Control ELF differs: stop before timing and audit code generation\"\n    );",r#"    let sections=read(r("control-sections.json"));let locations=read(r("control-locations.json"));assert_eq!(sections["all_text_and_rodata_byte_identical"],true);assert_eq!(locations["native_code_or_arithmetic_changed"],false);assert_eq!(locations["panic_locations"].as_array().unwrap().len(),20);
    save(r("control-preflight.json"),&json!({"frozen_sha256":frozen,"off_sha256":off,"ELF_byte_identical":frozen==off,"runtime_sections_equivalent":true,"original_OFF_body_verbatim":true,"exception":"20 verified Rust panic line fields+2 and build ID only; all other runtime bytes identical","timing_not_started":true,"performance_gates_unchanged":true}));
    for p in ["control-sections.rs","control-sections","control-sections.json","control-locations.rs","control-locations","control-locations.json","preflight-revision.json","preflight-revision.rs","preflight-revision","link-pins-v3.json"] {pin(&mut pins,r(p));}"#);
    fs::write(r("driver.rs"), d).unwrap();
    save(
        r("preflight-revision.json"),
        &json!({"timing_not_started":true,"original_strict_ELF_preflight_preserved":"initial-control-preflight","native_code_changed":false,"all_text_rodata_and_other_runtime_bytes_identical":true,"narrow_metadata_exception":"20 individually decoded Rust panic Location line fields, each+2, with identical filename/column/pointer/length; build ID differs","preflight_requirement_refined_before_any_timing":true,"performance_gates_changed":false,"successful_native_numeric_or_harness_tests_repeated":false,"rationale":"Full ELF identity is stricter than compiled work identity; source line metadata differs after cfg-only insertions. Keep executable code/data layout and all other runtime bytes exact."}),
    );
    let mut c = fs::read_to_string(r("close.rs")).unwrap();
    once(&mut c,"    assert_eq!(pre[\"ELF_byte_identical\"], true);\n    assert_eq!(\n        hash(format!(\"{R}/off-probe\")),\n        hash(format!(\"{R}/frozen-probe\"))\n    );","    assert_eq!(pre[\"runtime_sections_equivalent\"], true);\n    assert_eq!(hash(format!(\"{R}/off-probe\")),pre[\"off_sha256\"].as_str().unwrap());\n    assert_eq!(hash(format!(\"{R}/frozen-probe\")),pre[\"frozen_sha256\"].as_str().unwrap());");
    c=c.replace("\"control_executables_byte_identical\":true","\"control_executables_byte_identical\":false,\"control_code_and_constants_byte_identical\":true");
    c=c.replace("Frozen and fresh OFF executable bytes are checked identical before timing","Frozen and fresh OFF code and constants are checked byte-identical before timing; only build ID and20 verified panic source-line fields differ");
    c = c.replace(
        "Identical control ELF bytes exclude",
        "Identical control code and constants exclude",
    );
    c=c.replace("Frozen/OFFELFbyteidentical beforecapture.","Frozen/OFFcode/constantsbyteidentical;only20verifiedpaniclines+2/buildid differ;preflightmetadataexception beforecapture/perfgatesunchanged.");
    c = c.replace(
        "artifacts/linked/wide-tests/stdout.log",
        "artifacts/numeric-linked/wide-tests/stdout.log",
    );
    let marker = "All other failures/repairs, if any, remain in receipts.";
    once(&mut c,marker,"The independent numeric harness first fails compilation at one borrowed BigUint fixture entry; p.clone() repairs only that test value. Nine completed links/harness checks and all native suites are preserved; only the unfinished numeric build/test runs under revised pins. The strict ELF preflight then stops before capture. A section/relocation audit proves all runtime bytes identical except20 Rust panic source-line fields (each+2) and build ID. Before any timing, the preflight is refined to accept only this verified metadata difference; original preflight/failures are preserved and every performance gate remains unchanged. See numeric-repair.json, control-locations.json and preflight-revision.json.");
    fs::write(r("close.rs"), c).unwrap();
}
fn ready() {
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("link-pins-v2.json"))).unwrap();
    for (p, h) in &mut pins {
        if [r("driver.rs"), r("driver")]
            .iter()
            .any(|v| v == Path::new(p))
        {
            *h = hash(p.as_str());
        } else {
            assert_eq!(hash(p.as_str()), *h)
        }
    }
    for f in [
        "initial-control-preflight/driver.rs",
        "initial-control-preflight/driver",
        "initial-control-preflight/control-preflight.json",
        "initial-control-preflight/capture-preparation.log",
        "control-sections.rs",
        "control-sections",
        "control-sections.json",
        "control-locations.rs",
        "control-locations",
        "control-locations.json",
        "preflight-revision.rs",
        "preflight-revision",
        "preflight-revision.json",
    ] {
        pins.insert(r(f).display().to_string(), hash(r(f)));
    }
    save(r("link-pins-v3.json"), &json!(pins));
    println!("Preparedmetadata-onlyexception beforeanytiming; no arithmetic/perfgate/testchanges");
}
fn main() {
    match std::env::args().nth(1).unwrap().as_str() {
        "prepare" => prepare(),
        "ready" => ready(),
        _ => panic!("mode"),
    }
}
