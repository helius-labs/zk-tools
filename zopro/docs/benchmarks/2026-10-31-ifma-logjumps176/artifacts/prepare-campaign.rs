use std::fs;
const R:&str="/home/ubuntu/src/zopro-perf-work/ifma-logjumps176";
const OLD:&str="/home/ubuntu/src/zopro-perf-work/inverse-hint-alias176";
fn main(){
 let mut source=fs::read_to_string(format!("{OLD}/campaign.rs")).unwrap().replace(OLD,R);
 let lo=source.find("fn routes(").unwrap();let hi=source.find("fn prepare(){").unwrap();source.replace_range(lo..hi,"fn routes(j:&Value,_text:&str)->Value{json!({\"dispatch\":\"compile-time native8 mixed radix\",\"arm\":j[\"variant\"]})}\n");
 source=source.replace("env[\"ZOPRO_CPU_INVERSE_ALIASES\"]=json!(if arm==\"ON\"{\"1\"}else{\"0\"});","");
 let lo=source.find(" let linked=read(r(\"build/sdk/artifacts.json\"))").unwrap();let hi=source[lo..].find("save(r(\"identities.json\"),&ids);").unwrap()+lo+"save(r(\"identities.json\"),&ids);".len();
 let ids=r#" assert_eq!(hash(ACCEPTED),"6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661");pin(&mut pins,ACCEPTED);
 let mut ids=json!({"accepted":{"binary":ACCEPTED,"sha256":hash(ACCEPTED)}});
 for arm in["OFF","ON"]{let linked=read(r(format!("build/sdk-{arm}/artifacts.json")));let binary=linked["binary"].as_str().unwrap();assert_eq!(hash(binary),linked["binary_sha256"]);for(p,h)in linked["input_pins"].as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());pin(&mut pins,p);}pin(&mut pins,binary);
 let cmd=read(r(format!("build/sdk-{arm}/command.json")));for flag in["-O","target-cpu=native","lto=fat","codegen-units=1"]{assert!(cmd["argv"].as_array().unwrap().iter().any(|x|x==flag));}
 let frozen=read(r(format!("frozen-{arm}.json")));let f=frozen["rustflags"].as_str().unwrap();assert_eq!(f.contains("--cfg zopro_ifma_logjumps"),arm=="ON");assert_eq!(f,fs::read_to_string(r(format!("{arm}-flags.txt"))).unwrap());ids[arm]=json!({"binary":binary,"sha256":hash(binary),"native_cfg":arm=="ON"});}
 save(r("identities.json"),&ids);"#;
 source.replace_range(lo..hi,ids);source=source.replace("/home/ubuntu/src/zopro-perf-work/weighted-sdk176/proof-sha256-after.json","/home/ubuntu/src/zopro-perf-work/inverse-hint-alias176/proof-sha256-after.json").replace("28847","29051").replace("source-v2-pins.json","source-pins.json").replace("\"capture-scope.md\",","").replace("\"resource-notes.md\",","").replace("alias counts checked","compile-time sources/flags checked");
 source=source.replace("\"source-pins.json\",\"source-audit.json\"","\"source-pins.json\",\"source-audit.json\",\"source-review.json\",\"audit-source.rs\",\"audit-source\",\"audit-source.log\",\"model.json\",\"prepare-campaign.rs\"");
 fs::write(format!("{R}/campaign.rs"),source).unwrap();
 let s=fs::read_to_string(format!("{OLD}/finish.rs")).unwrap().replace("29051","29255").replace("28847","29051").replace("\"core_tests\":128","\"core_tests_each_arm\":125,\"field_tests_each_arm\":39").replace("\"prover_tests\":8","\"prover_tests_each_arm\":8").replace("\"field_source_unchanged\":true","\"field_source_unchanged\":false").replace("let aliases=routes","let dispatch=routes").replace("\"aliases\":aliases","\"dispatch\":dispatch");fs::write(format!("{R}/finish.rs"),s).unwrap();
 for f in["sampling.rs","counter.rs","metadata.rs"]{fs::copy(format!("{OLD}/{f}"),format!("{R}/{f}")).unwrap();}
 println!("Prepared4context SDK protocol from closed prior harness, both compile-time binaries, ledger29051->29255 if completed; no timing");
}
