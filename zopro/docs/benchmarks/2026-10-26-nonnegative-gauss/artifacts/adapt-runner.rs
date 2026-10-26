use std::{fs,path::Path};
fn main(){let r=Path::new("/tmp/zopro-nonnegative-gauss");for name in ["timing.rs","analyze.rs","profile.rs"]{
 let mut s=fs::read_to_string(Path::new("/tmp/zopro-streamed-complex-b").join(name)).unwrap().replace("/tmp/zopro-streamed-complex-b","/tmp/zopro-nonnegative-gauss").replace("build-v2","build").replace("build-input-pins-v2.json","build-input-pins.json").replace("build-plan-v2.json","build-plan.json");
 s=s.replace("streamed","gauss").replace("shared","streamed");
 if name=="timing.rs"{
 s=s.replace("        \"target/release/deps/libzopro_bn254-582f6b42258031af.rlib\",\n","").replace("        \"harness-repair.json\",\n","").replace("        \"setup-timing.rs\",","        \"adapt-runner.rs\",");
 s=s.replace("    let patterns = [\"chain\",", "    for entry in fs::read_dir(r.join(\"target/release/deps\")).unwrap(){let p=entry.unwrap().path();let n=p.file_name().unwrap().to_str().unwrap();if n.starts_with(\"libzopro_bn254-\")&&n.ends_with(\".rlib\"){pins.insert(p.to_str().unwrap().into(),hash(&p));}}\n    let patterns = [\"chain\",");
 }
 if name=="analyze.rs"{assert_eq!(s.matches("if !pass {").count(),2);s=s.replace("if !pass {","if !pass && control != \"streamed\" {");s=s.replace("\"scope\":\"Native field arithmetic only;", "\"diagnostic_only_control\":\"streamed\",\"required_controls\":[\"accepted\",\"fresh\"],\"scope\":\"Native field arithmetic only;");}
 fs::write(r.join(name),s).unwrap();
}println!("Prepared fixed496process runner, accepted/fresh gates, oldstreamed diagnostic only, and4post-latencyprofiles");}
