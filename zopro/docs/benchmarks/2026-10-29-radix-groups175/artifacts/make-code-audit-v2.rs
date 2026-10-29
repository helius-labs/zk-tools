use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/radix-groups175";
fn main() {
    let r = Path::new(R);
    assert!(!r.join("code-audit-v2.rs").exists());
    let old = fs::read_to_string(r.join("code-audit.rs")).unwrap();
    let mut audit = old
        .replace("r(\"codegen\")", "r(\"codegen-v2\")")
        .replace("codegen/{label}", "codegen-v2/{label}");
    assert!(audit.contains("name.starts_with(\"zopro_core::msm_affine::grouping::radix::\")"));
    audit = audit.replace(
        "name.starts_with(\"zopro_core::msm_affine::grouping::radix::\")",
        "name.contains(\"zopro_core::msm_affine::grouping::radix::\")",
    );
    audit = audit
        .replace("radix_child", "collision_sort_instantiation")
        .replace(
            "radix_child_code_only_in_ON",
            "collision_sort_instantiations_only_ON",
        );
    audit = audit.replace("r(\"code-audit.json\")", "r(\"code-audit-v2.json\")");
    audit=audit.replace("Audited grouping entry and ON-only radix code; immutable OFF/ON identities and cfg/features checked","Audited inlined grouping entry and ON-only collision-sort specializations; standalone radix symbol not required");
    fs::write(r.join("code-audit-v2.rs"), &audit).unwrap();
    let native = fs::read_to_string(r.join("native.rs")).unwrap();
    let header = &native[..native.find("include!(\"setup.rs\");").unwrap()];
    let main = r#"fn main(){
    let pins=read(r("build-pins.json"));for(p,h)in pins.as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
    let completed=read(r("native/completed.json"));assert_eq!(completed.as_array().unwrap().len(),10);
    let failure=read(r("native/failure.json"));assert_eq!(failure["completed"],completed);assert!(failure["error"].as_str().unwrap().contains("code-audit"));
    for label in ["control","candidate"]{
        let tests=read(r(format!("native/{label}-core/tests.json")));assert_eq!(tests["passed_tests"].as_array().unwrap().len(),128);assert_eq!(tests["unfiltered"],true);
        assert!(tests["passed_tests"].as_array().unwrap().contains(&json!("msm_affine::grouping::radix::tests::compiled_policy_dispatch_matches_cfg")));
        for(p,h)in read(r(format!("frozen-{label}.json")))["input_pins"].as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
    }
    inspect();
}"#;
    fs::write(
        r.join("audit.rs"),
        format!("{header}\ninclude!(\"code-audit-v2.rs\");\n{main}\n"),
    )
    .unwrap();
    println!("Prepared separate code audit for inlining, without changing any source/build/test/capture inputs");
}
