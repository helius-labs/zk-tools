use serde_json::{json, Value};
use std::{fs, path::Path};
const R: &str = "/tmp/zopro-tree-kernel-split";
fn main() {
    let r = Path::new(R);
    fs::copy("/tmp/zopro-adaptive-tree/stats.rs", r.join("stats.rs")).unwrap();
    let mut p = fs::read_to_string("/tmp/zopro-adaptive-tree/prepare.rs")
        .unwrap()
        .replace("/tmp/zopro-adaptive-tree", R);
    let a = p.find("    for (name, path) in [").unwrap();
    let b = p[a..].find("    for name in [").unwrap() + a;
    p.replace_range(a..b,r#"    for (name, path) in [
        ("accepted","/tmp/zopro-grouping-module/native/candidate/artifacts.json"),
        ("fresh","/tmp/zopro-poseidon-fused-dot/native/fresh/artifacts.json"),
        ("parent","/tmp/zopro-adaptive-tree/native/adaptive/artifacts.json"),
        ("split","/tmp/zopro-tree-kernel-split/native/split/artifacts.json"),
    ] {
        let a=read(path);assert_eq!(hash(a["binary"].as_str().unwrap()),a["binary_sha256"].as_str().unwrap());
        for(f,h)in a["input_pins"].as_object().unwrap(){assert_eq!(hash(f),h.as_str().unwrap());add(f,&mut pins);}
        add(path,&mut pins);add(a["binary"].as_str().unwrap(),&mut pins);
        variants.push(json!({"name":name,"binary":a["binary"]}));
    }
"#);
    p=p.replace("        \"/tmp/zopro-g2-tree-records/proof-inventory.json\",","        \"/tmp/zopro-g2-tree-records/proof-inventory.json\",\n        \"/tmp/zopro-adaptive-tree/proof-inventory.json\",");
    p = p
        .replace("7560", "7933")
        .replace("7569", "7942")
        .replace("-w4-adaptive", "-w4-split")
        .replace("native/adaptive-sdk", "native/split-sdk");
    let pos = p
        .find("    let mut p = read(\"/tmp/zopro-pairing-x-chain-sdk/perf-plan.json\");")
        .unwrap();
    let pos =
        pos + "    let mut p = read(\"/tmp/zopro-pairing-x-chain-sdk/perf-plan.json\");".len();
    p.insert_str(pos,"\n    assert_eq!(read(r.join(\"codegen-audit.json\"))[\"split_structure_observed\"],true);\n    p[\"profile_cases\"]=json!([\"1x2-w32\",\"4x2-w32\"]);\n");
    p=p.replace("        \"stats.rs\",","        \"stats.rs\",\n        \"adapt.rs\",\n        \"codegen-audit.json\",\n        \"inspect.rs\",\n        \"inspect\",");
    fs::write(r.join("prepare.rs"), p).unwrap();
    let mut a = fs::read_to_string("/tmp/zopro-adaptive-tree/analyze.rs")
        .unwrap()
        .replace("/tmp/zopro-adaptive-tree", R);
    a = a
        .replace("\"tree\"", "\"parent\"")
        .replace("\"adaptive\"", "\"split\"")
        .replace("adaptive/", "split/")
        .replace("Adaptivefullpass", "Splitfullpass");
    a = a
        .replace("364", "368")
        .replace("result[\"profiles\"], 4", "result[\"profiles\"], 8")
        .replace("17576", "18592")
        .replace("7569", "7942")
        .replace("746", "754")
        .replace("7933", "8310")
        .replace("373", "377")
        .replace("17612", "18628");
    fs::write(r.join("analyze.rs"), a).unwrap();
    let audit = json!({"parent_harness":"/tmp/zopro-adaptive-tree","new_control_variants":["accepted","fresh","parent","split"],"prior":7933,"preflight_next":7942,"performance_exports":368,"total_exports":377,"next_prior":8310,"profiles":8,"total_SDK_requests":18628,"old_gate_rules_preserved":true,"additional_profile_case":"4x2-w32"});
    fs::write(
        r.join("harness-adaptation.json"),
        serde_json::to_vec_pretty(&audit).unwrap(),
    )
    .unwrap();
    println!("Adapted explicitcontroltuples/counts;no generic source-word substitutions");
}
