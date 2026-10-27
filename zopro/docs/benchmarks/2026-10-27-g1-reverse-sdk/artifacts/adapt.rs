use serde_json::json;
use std::{fs, path::Path};
const R: &str = "/tmp/zopro-g1-reverse-sdk";
fn main() {
    let r = Path::new(R);
    fs::copy("/tmp/zopro-tree-kernel-split/stats.rs", r.join("stats.rs")).unwrap();
    let mut p = fs::read_to_string("/tmp/zopro-tree-kernel-split/prepare.rs")
        .unwrap()
        .replace("/tmp/zopro-tree-kernel-split", R);
    p = p.replace(
        "read(r.join(\"native/core/tests.json\"))",
        "read(\"/tmp/zopro-g1-reverse-overlap/native/core/tests.json\")",
    );
    let a = p
        .find("    assert_eq!(\n        read(r.join(\"codegen-audit.json\"))")
        .unwrap();
    let b = a + p[a..].find("    p[\"profile_cases\"]").unwrap();
    p.replace_range(a..b,"    assert_eq!(read(r.join(\"source-audit.json\"))[\"exact_component_stream_body\"],true);\n");
    let a = p.find("    for (name, path) in [").unwrap();
    let b = a + p[a..].find("    for name in [").unwrap();
    p.replace_range(a..b,r#"    for (name,path) in [
        ("accepted","/tmp/zopro-grouping-module/native/candidate/artifacts.json"),
        ("fresh","/tmp/zopro-poseidon-fused-dot/native/fresh/artifacts.json"),
        ("overlap","/tmp/zopro-g1-reverse-sdk/native/overlap/artifacts.json"),
    ] {
        let a=read(path);assert_eq!(hash(a["binary"].as_str().unwrap()),a["binary_sha256"].as_str().unwrap());
        for(f,h)in a["input_pins"].as_object().unwrap(){assert_eq!(hash(f),h.as_str().unwrap());add(f,&mut pins);}
        add(path,&mut pins);add(a["binary"].as_str().unwrap(),&mut pins);variants.push(json!({"name":name,"binary":a["binary"]}));
    }
"#);
    for line in [
        "        \"codegen-audit.json\",\n",
        "        \"inspect.rs\",\n",
        "        \"inspect\",\n",
    ] {
        assert_eq!(p.matches(line).count(), 1);
        p = p.replace(line, "");
    }
    p=p.replace("        \"/tmp/zopro-adaptive-tree/proof-inventory.json\",","        \"/tmp/zopro-adaptive-tree/proof-inventory.json\",\n        \"/tmp/zopro-tree-kernel-split/proof-inventory.json\",")
        .replace("7933","8310").replace("7942","8319").replace("native/split-sdk","native/overlap-sdk").replace("-w4-split","-w4-overlap").replace("9block4variant","9block3variant");
    fs::write(r.join("prepare.rs"), p).unwrap();
    let mut a = fs::read_to_string("/tmp/zopro-tree-kernel-split/analyze.rs")
        .unwrap()
        .replace("/tmp/zopro-tree-kernel-split", R);
    a = a
        .replace(
            "[\"accepted\", \"fresh\", \"parent\", \"split\"]",
            "[\"accepted\", \"fresh\", \"overlap\"]",
        )
        .replace("            (\"parent\", \"accepted\"),\n", "")
        .replace("            (\"split\", \"parent\"),\n", "")
        .replace(
            "[\"accepted\", \"fresh\", \"parent\"]",
            "[\"accepted\", \"fresh\"]",
        );
    let old="            if control != \"parent\"\n                && [\"24x2-w32\", \"merge-54x1-w32\"].contains(&name.as_str())\n                && ci[\"upper\"].as_f64().unwrap() >= 1.";
    assert_eq!(a.matches(old).count(), 1);
    a = a.replace(
        old,
        "            if name == \"1x2-w32\" && ci[\"upper\"].as_f64().unwrap() >= 1.",
    );
    a = a
        .replace("\"split\"", "\"overlap\"")
        .replace("split/", "overlap/")
        .replace("large_gain_failures", "primary_gain_failures")
        .replace(
            "result[\"distinct_verified_exports\"], 368",
            "result[\"distinct_verified_exports\"], 276",
        )
        .replace("result[\"profiles\"], 8", "result[\"profiles\"], 6")
        .replace(
            "p[\"variants\"].as_array().unwrap().len(), 4",
            "p[\"variants\"].as_array().unwrap().len(), 3",
        )
        .replace("18592", "13944")
        .replace("754", "570")
        .replace("377", "285")
        .replace("8310", "8595")
        .replace("7942", "8319")
        .replace(
            "\"native_core_tests\":111",
            "\"component_native_core_tests_reused\":111",
        )
        .replace(
            "\"threshold_actual_pairs\":32",
            "\"reverse_overlap_groups\":2",
        )
        .replace("Splitfullpass", "Overlapfullpass")
        .replace("18628SDK285Gnark", "13980SDK285Gnark");
    assert!(!a.contains("\"parent\"") && !a.contains("\"split\"") && !a.contains("368"));
    fs::write(r.join("analyze.rs"), a).unwrap();
    fs::write(r.join("harness-adaptation.json"),serde_json::to_vec_pretty(&json!({"parent_harness":"/tmp/zopro-tree-kernel-split","variants":["accepted","fresh","overlap"],"prior":8310,"preflight_next":8319,"latency_exports":270,"profile_exports":6,"total_exports":285,"next_prior":8595,"SDK_requests":13980,"profiles":6,"blocks":9,"gain_requirement":"primary32upper<1vsbothcontrols, original8aggregateupper<=1both","nonregression":"upper<=1.02all10both","calibration":"fresh/acceptedCIwithin1.03all10"})).unwrap()).unwrap();
    println!(
        "Adapted explicit3variants,285exports13980SDK,9blocks;frozenprimary+aggregategainchecks."
    );
}
