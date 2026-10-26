use std::{fs, path::Path};
fn main() {
    let root = Path::new("/tmp/zopro-streamed-complex-b");
    let mut text = fs::read_to_string("/tmp/zopro-phased-complex-products/closeout.rs").unwrap();
    text=text.replace("/tmp/zopro-phased-complex-products","/tmp/zopro-streamed-complex-b").replace("\"phased\"","\"streamed\"").replace("EXACT_PHASED","EXACT_STREAMED").replace("ARK_PHASED","ARK_STREAMED").replace("assert_eq!(pins.len(), 213)","assert_eq!(pins.len(), 218)").replace("\"performance_gate_passed\":false","\"performance_gate_passed\":read(r.join(\"component-result.json\"))[\"passed\"]").replace("First oracle compilation type error retained; test-only annotation repair before actual oracle execution and timing.","Initial inline previous coefficient tests retained; relative include repair before all timing; new row model and raw shared comparison qualified in build-v2.").replace("candidate negative; acceptedunchanged","component positive, not adopted; acceptedunchanged");
    let at = text.find("    let close = json!").unwrap();
    let validation = r#"
    assert_eq!(read(r.join("component-result.json"))["passed"],true);
    assert_eq!(read(r.join("codegen-screen.json"))["passed"],true);
    for (file,relocate) in [("build-input-pins.json",true),("build-input-pins-v2.json",false)] {
      let original:BTreeMap<String,String>=serde_json::from_value(read(r.join(file))).unwrap();
      for (path,digest) in original {let p=if relocate&&path==r.join("probe.rs").to_str().unwrap(){r.join("probe-initial.rs")}else{path.into()};assert_eq!(hash(&p),digest,"originalpin{}",p.display());}
    }
"#;
    text.insert_str(at, validation);
    fs::write(root.join("closeout.rs"), text).unwrap();
}
