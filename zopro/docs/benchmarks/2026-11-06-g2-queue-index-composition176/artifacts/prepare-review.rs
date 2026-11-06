use std::fs;
const R:&str="/home/ubuntu/src/zopro-perf-work/g2-queue-index-composition176";
const P:&str="/home/ubuntu/src/zopro-perf-work/g2-contained-index176";
fn change(s:&mut String,a:&str,b:&str){assert_eq!(s.matches(a).count(),1,"{a}");*s=s.replacen(a,b,1);}
fn main(){
let mut s=fs::read_to_string(format!("{P}/review.rs")).unwrap().replace(P,R);
let a=s.find("    let sourceaudit = load(r(\"source-audit.json\"));").unwrap();let b=s.find("    let mut commands = 0;").unwrap();
let source=r#"
    let sourceaudit=load(r("source-audit.json"));
    let parentaudit=load("/home/ubuntu/src/zopro-perf-work/g2-contained-index176/source-audit.json");
    let manifest=load(Path::new(Z).join("docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));
    let mut files=vec![];collect(&r("source"),&mut files);assert_eq!(files.len(),180);
    let mut exact=0;
    for(p,h)in manifest["source_files_sha256"].as_object().unwrap(){let original=fs::read(Path::new(Z).join(p)).unwrap();assert_eq!(digest(Path::new(Z).join(p)),h.as_str().unwrap());let actual=fs::read(r("source").join(p)).unwrap();if actual==original{exact+=1;}let mut expected=original;if let Some(t)=parentaudit["suffixes"][p].as_str(){expected.extend_from_slice(t.as_bytes());}for e in sourceaudit["edits"].as_array().unwrap().iter().filter(|e|e["file"]==p.as_str()){if let Some(t)=e["suffix"].as_str(){expected.extend_from_slice(t.as_bytes());}else{let text=String::from_utf8(expected).unwrap();let old=e["old"].as_str().unwrap();assert_eq!(text.matches(old).count(),1);expected=text.replacen(old,e["new"].as_str().unwrap(),1).into_bytes();}}assert_eq!(actual,expected,"forward production reconstruction {p}");}assert_eq!(exact,170);
    for p in["crates/zopro-core/src/msm/contained_index.rs","crates/zopro-core/src/msm_affine/g2_phi/contained_indexed.rs"]{assert_eq!(fs::read(r("source").join(p)).unwrap(),fs::read(Path::new("/home/ubuntu/src/zopro-perf-work/g2-contained-index176/source").join(p)).unwrap());}
    assert_eq!(digest(r("source/crates/zopro-core/src/msm_affine/queue_drain.rs")),digest("/home/ubuntu/src/zopro-perf-work/queue-drain175/queue_drain.rs"));
    let queue=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/queue-drain175/g2-suffix.rs").unwrap().replace("cfg!(zopro_queue_drain)","(cfg!(zopro_queue_drain)&&crate::queue_index_composition::drain_enabled())");assert!(fs::read_to_string(r("source/crates/zopro-core/src/msm_affine/g2_borrowed.rs")).unwrap().ends_with(&queue));
    for(name,c)in parentaudit["clones"].as_object().unwrap(){let mut restored=c["candidate"].as_str().unwrap().to_owned();for change in c["edits"].as_array().unwrap().iter().rev(){restored=restored.replacen(change[1].as_str().unwrap(),change[0].as_str().unwrap(),1);}assert_eq!(restored,c["original"]);let(old,new)=match name.as_str(){"query"=>("crates/zopro-core/src/msm_affine.rs","crates/zopro-core/src/msm_affine.rs"),"old_global_packing"=>("docs/benchmarks/2026-10-29-g2-packed-index175/artifacts/source/crates/zopro-core/src/msm/packed_index.rs","crates/zopro-core/src/msm/contained_index.rs"),_=>("crates/zopro-core/src/msm_affine/g2_phi.rs","crates/zopro-core/src/msm_affine/g2_phi/contained_indexed.rs")};assert!(fs::read_to_string(Path::new(Z).join(old)).unwrap().contains(&restored));assert!(fs::read_to_string(r("source").join(new)).unwrap().contains(c["candidate"].as_str().unwrap()));}
    let d=load(r("driver-audit.json"));let mut driver=fs::read_to_string(r("driver/src/main.rs")).unwrap();assert_eq!(driver,fs::read_to_string(r("driver.rs")).unwrap());for e in d["edits"].as_array().unwrap().iter().rev(){let from=e["new"].as_str().unwrap();assert_eq!(driver.matches(from).count(),1);driver=driver.replacen(from,e["old"].as_str().unwrap(),1);}assert_eq!(driver,fs::read_to_string(d["parent"].as_str().unwrap()).unwrap());assert_eq!(format!("{:x}",Sha256::digest(driver.as_bytes())),d["parent_sha256"]);
    let ids=load(r("identities.json"));let binary_hash=digest(r("candidate-bin"));for arm in["ORIGINAL","DRAIN","PACK","BOTH"]{assert_eq!(ids[arm]["binary_sha256"],binary_hash);assert_eq!(ids[arm]["binary"],r("candidate-bin").display().to_string());let actual=ids[arm]["actual_rustc"].as_str().unwrap();assert!(fs::read_to_string(r("native/driver-build/stderr.log")).unwrap().lines().any(|l|l==actual));for flag in["-C opt-level=3","-C lto=fat","-C codegen-units=1","-C target-cpu=native","--cfg zopro_queue_drain","--cfg zopro_queue_index_composition","--cfg zopro_contained_index"]{assert!(actual.contains(flag));}}
    let mut pins=0;for f in["build-pins.json","capture-pins.json"]{for(p,h)in load(r(f)).as_object().unwrap(){assert_eq!(digest(p),h.as_str().unwrap());pins+=1;}}
    for line in fs::read_to_string(r("analysis-pins.sha256")).unwrap().lines(){let(h,p)=line.split_once("  ").unwrap();assert_eq!(digest(p),h);}
"#;
s.replace_range(a..b,source);
let a=s.find("    let native = fs::read_to_string").unwrap();let b=s.find("    let mut callers = BTreeMap::new();").unwrap();
s.replace_range(a..b,r#"
    for tag in["OFF","ON"]{let native=fs::read_to_string(r(format!("native/core-drain-{tag}/stdout.log"))).unwrap();assert!(native.contains("130 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out"));let chosen=native.lines().filter(|l|l.starts_with("test ")&&(l.contains("contained_tests::")||l.contains("queue_drain_tests::")||l.contains("queue_drain::tests::"))).collect::<Vec<_>>();assert_eq!(chosen.len(),5);assert!(chosen.iter().all(|l|l.ends_with(" ... ok")));}
"#);
change(&mut s,"if j[\"variant\"] == \"ON\" && j[\"case\"][\"query\"] == \"B\"","if [\"PACK\",\"BOTH\"].contains(&j[\"variant\"].as_str().unwrap()) && j[\"case\"][\"query\"] == \"B\"");
change(&mut s,"        assert_eq!(v[\"query\"], j[\"actual_query\"]);","        assert_eq!(v[\"query\"], j[\"actual_query\"]);\n        assert_eq!(v[\"composition\"][\"drain_enabled\"], [\"DRAIN\",\"BOTH\"].contains(&j[\"variant\"].as_str().unwrap()));\n        assert_eq!(v[\"composition\"][\"pack_enabled\"], [\"PACK\",\"BOTH\"].contains(&j[\"variant\"].as_str().unwrap()));");
change(&mut s,"for arm in [\"FROZEN\", \"OFF\", \"ON\"]","for arm in [\"ORIGINAL\", \"DRAIN\", \"PACK\", \"BOTH\"]");
change(&mut s,"for (num, den) in [(\"OFF\", \"FROZEN\"), (\"ON\", \"OFF\"), (\"ON\", \"FROZEN\")]","for (num, den) in [(\"DRAIN\",\"ORIGINAL\"),(\"PACK\",\"ORIGINAL\"),(\"BOTH\",\"ORIGINAL\"),(\"PACK\",\"DRAIN\"),(\"BOTH\",\"DRAIN\"),(\"BOTH\",\"PACK\")]");
change(&mut s,"if den == \"FROZEN\" || case.contains(\"-A-\")", "if case.contains(\"-A-\")");
change(&mut s,"sorted[11] <= 1.02","sorted[11] <= 1.03");
change(&mut s,"            assert_eq!(s[\"selection_gate\"], den == \"OFF\");","            let selected=case.contains(\"-A-\")||(num==\"BOTH\"&&[\"ORIGINAL\",\"DRAIN\"].contains(&den));\n            assert_eq!(s[\"selection_gate\"], selected);");
change(&mut s,"if den == \"OFF\" && !pass", "if selected && !pass");
change(&mut s,"\"negative-control\"", "\"unchanged-A-stability\"");
change(&mut s,"if num == \"ON\" && den == \"OFF\" && case == \"1x2-B-w32\"", "if num == \"BOTH\" && [\"ORIGINAL\",\"DRAIN\"].contains(&den) && case == \"1x2-B-w32\"");
let a=s.find("    for den in [\"OFF\", \"FROZEN\"] {").unwrap();let b=s.find("    let mut samples_all = 0;").unwrap();
s.replace_range(a..b,r#"
    for(num,den)in[("DRAIN","ORIGINAL"),("PACK","ORIGINAL"),("BOTH","ORIGINAL"),("PACK","DRAIN"),("BOTH","DRAIN"),("BOTH","PACK")]{let name=format!("{num}/{den}");let mut values=(0..15).map(|b|(ratios[&("1x2-B-w32".into(),name.clone())][b]*ratios[&("2x2-B-w32".into(),name.clone())][b]).sqrt()).collect::<Vec<_>>();let wins=values.iter().filter(|&&r|r<1.).count();values.sort_by(f64::total_cmp);let stat=&a["aggregates"][&name];near(values[7],stat["median"].as_f64().unwrap());near(values[3],stat["low"].as_f64().unwrap());near(values[11],stat["high"].as_f64().unwrap());assert_eq!(wins,stat["wins"].as_u64().unwrap()as usize);if num=="BOTH"&&["ORIGINAL","DRAIN"].contains(&den)&&(values[11]>=1.||wins<10){failures.push(format!("B32-two-shape/{num}/{den}/gain"));}}
"#);
change(&mut s,"assert_eq!(commands, 375);","assert_eq!(commands, 481);");change(&mut s,"assert_eq!(allcalls, 15462);","assert_eq!(allcalls, 15688);");change(&mut s,"assert_eq!(reusedcalls, 35);","assert_eq!(reusedcalls, 0);");change(&mut s,"assert_eq!(samples_all, 124386);","assert_eq!(samples_all, 77807);");
change(&mut s,"\"snapshot_files\":178,\"originals_exact\":171,\"all_originals_prefix_recovered\":true","\"snapshot_files\":180,\"originals_exact\":170,\"all_originals_forward_reconstructed\":true");
change(&mut s,"\"native_tests\":128,\"new_native_tests\":3","\"native_tests_each_drain_mode\":130,\"native_drain_modes\":2,\"new_native_tests\":0");
change(&mut s,"\"new_capture_commands\":372,\"reused_preflight_receipts\":7", "\"new_capture_commands\":477,\"reused_preflight_receipts\":0");
change(&mut s,"\"caller_rows\":348", "\"caller_rows\":456");change(&mut s,"\"raw_loss_checks\":6", "\"raw_loss_checks\":4");
assert!(!std::path::Path::new(R).join("review.rs").exists());fs::write(format!("{R}/review.rs"),s).unwrap();println!("Prepared separate forward-source/raw-ratio/argv/env/profile reviewer; no capture/test rerun.");
}
