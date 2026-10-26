use std::{fs, path::Path};
fn main() {
    let r = Path::new("/tmp/zopro-fused-product-overlap");
    let mut s = fs::read_to_string("/tmp/zopro-deferred-dif32-sdk/protocol.rs").unwrap();
    s = s
        .replace(
            "let root = Path::new(\"/tmp/zopro-deferred-dif32-sdk\");",
            "let root = Path::new(\"/tmp/zopro-fused-product-overlap\");",
        )
        .replace(
            "let parent = Path::new(\"/tmp/zopro-outlined-snapshot-fence\");",
            "let parent = Path::new(\"/tmp/zopro-deferred-dif32-sdk\");",
        );
    s = s
        .replace(
            "assert_eq!(prior.len(), 3774);",
            "assert_eq!(prior.len(), 4084);",
        )
        .replace(
            "assert_eq!(prior.len(), 4084);\n    let variants",
            "assert_eq!(prior.len(), 4321);\n    let variants",
        );
    s = s.replace(
        "ZOPRO_CPU_DEFERRED_DIF32",
        "ZOPRO_CPU_FUSED_PRODUCT_OVERLAP",
    );
    s = s.replace(
        "\"/tmp/zopro-outlined-snapshot-fence\",\n            \"/tmp/zopro-deferred-dif32-sdk\",",
        "\"/tmp/zopro-deferred-dif32-sdk\",\n            \"/tmp/zopro-fused-product-overlap\",",
    );
    s=s.replace("    let mut perf = read(parent.join(\"perf-template.json\"));",r#"    steps.push(json!({"kind":"command","name":"validate-activation","argv":[root.join("activation-audit")],"timeout_seconds":30}));
    let mut perf = read(parent.join("perf-template.json"));"#);
    s = s.replace(
        "assert_eq!(steps.len(), 42);",
        "assert_eq!(steps.len(), 42);",
    ); // Existing assertion precedes appended activation check.
    s=s.replace("\"ZOPRO_CPU_FUSED_PRODUCT_OVERLAP\":if mode==\"on\"{\"1\"}else{\"0\"}","\"ZOPRO_CPU_FUSED_PRODUCT_OVERLAP\":if mode==\"on\"{\"1\"}else{\"0\"},\"ZOPRO_FUSED_PRODUCT_OVERLAP_TRACE\":\"1\"");
    s = s.replace(
        "        \"protocol.rs\",",
        "        \"protocol.rs\",\n        \"activation-audit.rs\",\n        \"activation-audit\",",
    );
    s = s.replace("Frozen3variants", "Frozen3variants").replace(
        "FROZEN3variants8610latencySDK510profileSDK18preflightexports;42steps{}pins4084priorproofs",
        "FROZEN3variants8610latencySDK510profileSDK18preflightexports;43steps{}pins4321priorproofs",
    );
    fs::write(r.join("protocol.rs"), s).unwrap();
    fs::copy(
        "/tmp/zopro-deferred-dif32-sdk/layout.rs",
        r.join("layout.rs"),
    )
    .unwrap();
    let mut s = fs::read_to_string("/tmp/zopro-deferred-dif32-sdk/prepare-perf.rs")
        .unwrap()
        .replace(
            "/tmp/zopro-deferred-dif32-sdk",
            "/tmp/zopro-fused-product-overlap",
        )
        .replace(
            "assert_eq!(prior.len(), 4084);",
            "assert_eq!(prior.len(), 4321);",
        )
        .replace(
            "assert_eq!(prior.len(), 4102);",
            "assert_eq!(prior.len(), 4339);",
        );
    s = s.replace(
        "    let mut config = read(root.join(\"perf-template.json\"));",
        r#"    assert_eq!(read(root.join("activation-audit.json"))["status"],"passed");
    let mut config = read(root.join("perf-template.json"));"#,
    );
    s=s.replace("        \"prepare-perf.rs\",","        \"prepare-perf.rs\",\n        \"activation-audit.json\",\n        \"preflight/validate-activation/stdout.log\",");
    let st = s.find("    let phase = json!").unwrap();
    s.truncate(st);
    s.push_str(r#"    println!("Frozen{}perf pins4339prior;18preflightproofsverified; activation and Planlayout audits passed",pins.len());
}
"#);
    fs::write(r.join("prepare-perf.rs"), s).unwrap();
    let s = fs::read_to_string("/tmp/zopro-deferred-dif32-sdk/analyze.rs")
        .unwrap()
        .replace(
            "/tmp/zopro-deferred-dif32-sdk",
            "/tmp/zopro-fused-product-overlap",
        );
    fs::write(r.join("analyze.rs"), s).unwrap();
}
