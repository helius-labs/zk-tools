use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/complex-rowq175";
fn main() {
    let r = Path::new(R);
    assert!(!r.join("driver.rs").exists());
    let old =
        fs::read_to_string("/home/ubuntu/src/zopro-perf-work/positive-gauss-caller175/driver.rs")
            .unwrap();
    let a = old.find("fn link(){").unwrap();
    let b = old.find("fn prepare(){").unwrap();
    let end = old.find("fn archive(){").unwrap();
    let prefix = old[..a].replace("positive-gauss-caller175", "complex-rowq175");
    let mut body = old[b..end]
        .replace("positive-gauss-caller175", "complex-rowq175")
        .replace("zopro_gauss_positive", "zopro_complex_rowq")
        .replace("17811", "18076")
        .replace(
            "inherited-field-qualification.json",
            "field-qualification.json",
        );
    body = body.replace(
        "\"inherited_field_qualification\":",
        "\"new_field_qualification\":",
    );
    let link = r##"fn link(){
 assert_eq!(read(r("native/result.json"))["status"],"complete");assert!(!r("link-plan.json").exists());
 let mut pins:BTreeMap<String,String>=serde_json::from_value(read(r("build-pins.json"))).unwrap();verify(&pins);let mut steps=vec![];let mut tests=BTreeMap::new();
 let frozen=read(r("inherited-frozen-tests.json"));assert_eq!(frozen["passed_tests"].as_array().unwrap().len(),128);tests.insert("frozen",128usize);
 for name in["off","on"]{
  let core=read(r(format!("native/core-{name}/tests.json")));assert_eq!(core["unfiltered"],true);assert_eq!(core["passed_tests"].as_array().unwrap().len(),128);tests.insert(name,128usize);pin(&mut pins,r(format!("native/core-{name}/tests.json")));
  let a=read(r(format!("artifacts-{name}.json")));assert_eq!(a["rustflags"].as_str().unwrap().contains("--cfg zopro_complex_rowq"),name=="on");for(p,h)in a["input_pins"].as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());pin(&mut pins,p);}pin(&mut pins,r(format!("artifacts-{name}.json")));
  let mut argv=vec!["rustc".to_owned()];argv.extend(a["rustflags"].as_str().unwrap().split_whitespace().map(str::to_owned));let dependency=Path::new(a["libraries"]["zopro_core"].as_str().unwrap()).parent().unwrap();argv.extend(["--edition=2021".into(),"-O".into(),r("probe.rs").display().to_string(),"-o".into(),r(format!("{name}-probe")).display().to_string(),"-L".into(),format!("dependency={}",dependency.display())]);for lib in["zopro_core","zopro_bn254","rayon","libc","serde_json","sha2"]{argv.extend(["--extern".into(),format!("{lib}={}",a["libraries"][lib].as_str().unwrap())]);}steps.push(json!({"kind":"command","name":format!("link-{name}"),"argv":argv,"timeout_seconds":300}));
 }
 let field=read(r("native/field-on/tests.json"));assert_eq!(field["passed_tests"].as_array().unwrap().len(),2);pin(&mut pins,r("native/field-on/tests.json"));let log=fs::read_to_string(r("native/field-on/stdout.log")).unwrap();assert!(log.contains("2 passed; 0 failed; 33 filtered out"));pin(&mut pins,r("native/field-on/stdout.log"));pin(&mut pins,r("native/field-on/stderr.log"));
 let qual=json!({"new_field_tests":2,"new_Arkworks_and_exact_raw_output_lanes":690176,"new_complex_model":read(r("model.json")),"unchanged33field_tests_inherited":true,"frozen128core_tests_inherited_byte_exact":true,"fresh_core_tests_each_mode":128,"full_SDK_qualification":false});save(r("field-qualification.json"),&qual);save(r("native-tests.json"),&json!(tests));for f in["field-qualification.json","native-tests.json","driver.rs","driver","make-driver.rs","native/result.json"]{pin(&mut pins,r(f));}save(r("link-pins.json"),&json!(pins));save(r("link-plan.json"),&json!({"version":1,"source_pins":r("link-pins.json"),"steps":steps}));println!("Prepared2freshprobe links; frozenbinary/tests reusedexact; newfield690176 +128coreeachmode");
}
"##;
    let main="fn main(){match std::env::args().nth(1).as_deref(){Some(\"link\")=>link(),Some(\"prepare\")=>prepare(),Some(\"analyze\")=>analyze(),_=>panic!(\"link|prepare|analyze\")}}\n";
    fs::write(r.join("driver.rs"), format!("{prefix}{link}{body}{main}")).unwrap();
    println!("PreparedactualG2callercontroller;837latencies/27preflights/4gatedprofiles; no blanketkernel subsetpromotion");
}
