use std::{fs, path::Path};
fn main() {
    let r = Path::new("/tmp/zopro-fused-product-overlap");
    let mut s = fs::read_to_string(r.join("protocol.rs")).unwrap();
    s = s
        .replace("\"native/", "\"native-v2/")
        .replace("\"native-plan.json\"", "\"native-plan-v2.json\"")
        .replace("build-input-pins.json", "build-input-pins-v2.json")
        .replace("all-nine-template.json", "all-nine-template-v2.json")
        .replace("perf-template.json", "perf-template-v2.json")
        .replace(
            "parent.join(\"all-nine-template-v2.json\")",
            "parent.join(\"all-nine-template.json\")",
        )
        .replace(
            "parent.join(\"perf-template-v2.json\")",
            "parent.join(\"perf-template.json\")",
        );
    s = s
        .replace(
            "preflight-proof-inventory.json",
            "preflight-proof-inventory-v2.json",
        )
        .replace("inventory-preflight.sh", "inventory-preflight-v2.sh")
        .replace("validate-preflight.jq", "validate-preflight-v2.jq")
        .replace(
            "parent.join(\"inventory-preflight-v2.sh\")",
            "parent.join(\"inventory-preflight.sh\")",
        )
        .replace(
            "parent.join(\"validate-preflight-v2.jq\")",
            "parent.join(\"validate-preflight.jq\")",
        )
        .replace(
            "root.join(\"preflight-proofs\")",
            "root.join(\"preflight-proofs-v2\")",
        )
        .replace("preflight-plan.json", "preflight-plan-v2.json")
        .replace("input-pins.json", "input-pins-v2.json")
        .replace("\"protocol.rs\"", "\"protocol-v2.rs\"")
        .replace("activation-audit.rs", "activation-audit-v2.rs")
        .replace(
            "root.join(\"activation-audit\")",
            "root.join(\"activation-audit-v2\")",
        )
        .replace("\"activation-audit\",", "\"activation-audit-v2\",");
    let injection = r#"    assert_eq!(prior.len(),4321);
    fn collect(p:&Path,out:&mut BTreeMap<String,String>){for e in fs::read_dir(p).unwrap(){let p=e.unwrap().path();if p.is_dir(){collect(&p,out)}else if p.extension().is_some_and(|e|e=="proof"||e=="public"){assert!(out.insert(p.to_str().unwrap().into(),hash(&p)).is_none());}}}
    let mut initial=BTreeMap::new();collect(&root.join("preflight-proofs"),&mut initial);assert_eq!(initial.len(),18);
    let mut initial_exports=0;for(p,h)in&initial{add(p,&mut pins);if p.ends_with(".proof"){assert!(prior.insert(h.clone()));initial_exports+=1;}}
    assert_eq!(initial_exports,9);assert_eq!(prior.len(),4330);save(root.join("initial-proof-inventory.json"),&json!(initial));add(root.join("initial-proof-inventory.json"),&mut pins);
"#;
    s = s.replace("    assert_eq!(prior.len(), 4321);", injection);
    // The inventory helper must read the versioned preflight exports, leaving v1 exports as prior evidence.
    s=s.replace(".replace(\n            \"/tmp/zopro-deferred-dif32-sdk\",\n            \"/tmp/zopro-fused-product-overlap\",\n        );",".replace(\n            \"/tmp/zopro-deferred-dif32-sdk\",\n            \"/tmp/zopro-fused-product-overlap\",\n        ).replace(\"/preflight-proofs\",\"/preflight-proofs-v2\").replace(\"preflight-proof-inventory.json\",\"preflight-proof-inventory-v2.json\");");
    s = s.replace(
        "43steps{}pins4321priorproofs",
        "43steps{}pins4330priorproofs",
    );
    s = s.replace(
        "parent.join(\"input-pins-v2.json\")",
        "parent.join(\"input-pins.json\")",
    );
    fs::write(r.join("protocol-v2.rs"), s).unwrap();
    let s = fs::read_to_string(r.join("activation-audit.rs"))
        .unwrap()
        .replace("all-nine-template.json", "all-nine-template-v2.json")
        .replace("r.join(\"preflight\")", "r.join(\"preflight-v2\")")
        .replace("activation-audit.json", "activation-audit-v2.json");
    fs::write(r.join("activation-audit-v2.rs"), s).unwrap();
    let s = fs::read_to_string(r.join("prepare-perf.rs"))
        .unwrap()
        .replace("preflight/result.json", "preflight-v2/result.json")
        .replace("preflight/", "preflight-v2/")
        .replace("activation-audit.json", "activation-audit-v2.json")
        .replace("perf-template.json", "perf-template-v2.json")
        .replace("prior.len(), 4321", "prior.len(), 4330")
        .replace("prior.len(), 4339", "prior.len(), 4348")
        .replace(
            "preflight-proof-inventory.json",
            "preflight-proof-inventory-v2.json",
        )
        .replace("input-pins.json", "input-pins-v2.json")
        .replace("\"prepare-perf.rs\"", "\"prepare-perf-v2.rs\"")
        .replace("preflight-plan.json", "preflight-plan-v2.json")
        .replace("perf-plan.json", "perf-plan-v2.json")
        .replace("pins4339prior", "pins4348prior");
    fs::write(r.join("prepare-perf-v2.rs"), s).unwrap();
}
