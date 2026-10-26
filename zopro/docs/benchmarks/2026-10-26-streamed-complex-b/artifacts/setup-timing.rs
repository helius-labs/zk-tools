use serde_json::Value;
use std::{fs, path::Path};
fn main() {
    let root = Path::new("/tmp/zopro-streamed-complex-b");
    let mut analyze = fs::read_to_string("/tmp/zopro-phased-complex-products/analyze.rs").unwrap();
    analyze = analyze
        .replace(
            "/tmp/zopro-phased-complex-products",
            "/tmp/zopro-streamed-complex-b",
        )
        .replace("\"phased\"", "\"streamed\"");
    fs::write(root.join("analyze.rs"), analyze).unwrap();
    let mut text = fs::read_to_string("/tmp/zopro-phased-complex-products/timing.rs").unwrap();
    text = text
        .replace(
            "/tmp/zopro-phased-complex-products",
            "/tmp/zopro-streamed-complex-b",
        )
        .replace("\"phased\"", "\"streamed\"")
        .replace("complex_phased_codegen", "complex_streamed_codegen")
        .replace("\"repair.json\"", "\"harness-repair.json\"");
    let logs = fs::read_to_string(root.join("build-v2/fresh-bench-build/stdout.log")).unwrap();
    let mut libs = Vec::new();
    for line in logs.lines() {
        if let Ok(v) = serde_json::from_str::<Value>(line) {
            if v["reason"] == "compiler-artifact" && v["target"]["name"] == "zopro_bn254" {
                for p in v["filenames"].as_array().unwrap() {
                    if p.as_str().unwrap().ends_with(".rlib") {
                        libs.push(p.as_str().unwrap().to_owned());
                    }
                }
            }
        }
    }
    assert_eq!(libs.len(), 1);
    let rel = Path::new(&libs[0])
        .strip_prefix(root)
        .unwrap()
        .to_str()
        .unwrap();
    assert!(text.contains("target/release/deps/libzopro_bn254-fdaae4a8f3010cbc.rlib"));
    text = text.replace(
        "target/release/deps/libzopro_bn254-fdaae4a8f3010cbc.rlib",
        rel,
    );
    text = text.replace(
        "    let mut records = Vec::new();",
        "    let mut records = Vec::new();\n    let mut sequences=BTreeMap::new();",
    );
    let key = "            let subs = lines";
    assert_eq!(text.matches(key).count(), 1);
    text=text.replace(key,"            sequences.insert(format!(\"{label}:{}\",if symbol.starts_with(\"probe::run\"){\"repeat\"}else{\"wrapper\"}),normalize(&lines));\n            let subs = lines");
    let insert = r#"
    let candidate=records.iter().find(|r|r["variant"]=="streamed"&&r["symbol"]=="complex_streamed_codegen").unwrap();
    let wrapper_same=sequences["shared:wrapper"]==sequences["streamed:wrapper"];
    let repeat_same=sequences["shared:repeat"]==sequences["streamed:repeat"];
    let codegen_pass=candidate["ifma_sites"]==250&&!(wrapper_same&&repeat_same);
    fs::write(r.join("codegen-screen.json"),serde_json::to_vec_pretty(&json!({"passed":codegen_pass,"standalone_expected_ifma":250,"standalone_actual_ifma":candidate["ifma_sites"],"shared_vs_streamed_wrapper_normalized_equal":wrapper_same,"shared_vs_streamed_repeat_normalized_equal":repeat_same,"normalization":"Remove instruction addresses, symbol-address comments and RIP-relative displacement only; map repeat strategy labels to same self name. Static equivalence screen only.","sequence_sha256":sequences.iter().map(|(k,v)|(k,format!("{:x}",Sha256::digest(v.as_bytes())))).collect::<BTreeMap<_,_>>(),"limitation":"Static instruction/stack counts are not dynamic spills or performance."})).unwrap()).unwrap();
    assert!(codegen_pass,"declared codegen screen fails; close without timing");
"#;
    let key = "    let mut pins: BTreeMap<String, String> =";
    assert_eq!(text.matches(key).count(), 1);
    text = text.replace(key, &(insert.to_owned() + key));
    text = text.replace(
        "        \"codegen.txt\",",
        "        \"codegen.txt\",\n        \"codegen-screen.json\",\n        \"setup-timing.rs\",",
    );
    let helper = r#"
fn normalize(lines:&[&str])->String {
 lines.iter().map(|line|{
  let line=line.split('#').next().unwrap().trim();
  let line=line.replace("probe::run::<1>","repeat").replace("probe::run::<2>","repeat");
  if let Some(at)=line.find("(%rip)"){let start=line[..at].rfind(|c:char|c.is_whitespace()||c==',').map(|i|i+1).unwrap_or(0);format!("{}<rip>{}",&line[..start],&line[at..])}else{line}
 }).collect::<Vec<_>>().join("\n")
}
"#;
    text = text.replacen("fn main() {", &(helper.to_owned() + "\nfn main() {"), 1);
    fs::write(root.join("timing.rs"), text).unwrap();
}
