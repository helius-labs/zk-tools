use std::fs;
const R:&str="/home/ubuntu/src/zopro-perf-work/scalar-square-sdk";
fn main(){
    let old=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/inverse-finish-sdk/experiment.rs").unwrap();
    let mut head=old[..old.find("fn pins()").unwrap()].to_string();head=head.replace("/zopro-perf-work/inverse-finish-sdk","/zopro-perf-work/scalar-square-sdk");
    let pins=r#"fn pins() -> BTreeMap<String,String> {
        let mut p:BTreeMap<String,String>=serde_json::from_value(read(format!("{R}/build-pins.json"))).unwrap();
        for(f,h)in &p{assert_eq!(hash(f),*h);}
        for name in["design.md","capture.rs","capture","finish.rs","finish","make-capture.rs","make-finish.rs","build-plan.json","native/result.json"]{pin(&mut p,Path::new(R).join(name));}
        for label in["off","on"]{let a=read(format!("{R}/frozen-{label}.json"));for(f,h)in a["input_pins"].as_object().unwrap(){assert_eq!(hash(f),h.as_str().unwrap());pin(&mut p,f);}pin(&mut p,format!("{R}/frozen-{label}.json"));}
        pin(&mut p,X);pin(&mut p,format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs"));pin(&mut p,format!("{Z}/crates/zopro-bench/examples/support/perf.rs"));p
    }
"#;
    let mut prepare=old[old.find("fn prepare()").unwrap()..old.find("fn main()").unwrap()].to_string();
    prepare=prepare.replace("frozen-{label}-cargo-v2.jsonl","frozen-{label}-cargo.jsonl");
    prepare=prepare.replace("/zopro-perf-work/euler-fused-columns/","/zopro-perf-work/inverse-finish-sdk/");
    prepare=prepare.replace("assert_eq!(prior.len(), 12785);","assert_eq!(prior.len(), 12884);");
    prepare=prepare.replace("assert_eq!(prior.len(), 12686);","assert_eq!(prior.len(), 12785);");
    fs::write(format!("{R}/capture.rs"),format!("{head}{pins}\n{prepare}\nfn main(){{prepare();}}\n")).unwrap();
    println!("Prepared full SDK capture source from fixed mature schedule,with new pure-library pins and12884prior");
}
