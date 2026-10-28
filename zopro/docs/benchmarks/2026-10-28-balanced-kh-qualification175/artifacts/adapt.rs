use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/balanced-kh-qualification175";
const P:&str="/home/ubuntu/src/zopro-perf-work/balanced-kh-windows174";
fn copy_dir(a:&Path,b:&Path){fs::create_dir_all(b).unwrap();for e in fs::read_dir(a).unwrap(){let e=e.unwrap();if e.file_type().unwrap().is_dir(){copy_dir(&e.path(),&b.join(e.file_name()));}else{fs::copy(e.path(),b.join(e.file_name())).unwrap();}}}
fn main(){
 for d in ["source","native","native-repair"]{copy_dir(&Path::new(P).join(d),&Path::new(R).join(d));}
 for f in ["source-audit.json","native-flags.txt","frozen-control.json","frozen-candidate.json","history-audit.json","native-completion.json"]{fs::copy(Path::new(P).join(f),Path::new(R).join(f)).unwrap();}
 fs::write(Path::new(R).join("native-reuse.json"),format!("{{\"pilot_root\":\"{P}\",\"pilot_commit\":\"822560e9dc2f89fc33c3f8423e57eae9f590ef18\",\"source_files\":175,\"native_tests_and_builds_reused_not_rerun\":true,\"new_native_tests\":0,\"executables_reused_at_original_paths\":true}}\n")).unwrap();
 let mut s=fs::read_to_string(Path::new(P).join("capture.rs")).unwrap().replace(P,R);
 let a=s.find("    assert_eq!(read(r(\"native/result.json\"))[\"status\"], \"complete\");").unwrap();
 let b=s[a..].find("    for (f, h) in &pins {").unwrap()+a;
 s.replace_range(a..b,&format!("    assert_eq!(read(\"{P}/analysis.json\")[\"gate_passed\"],true);\n    let mut pins:BTreeMap<String,String>=serde_json::from_value(read(\"{P}/pins.json\")).unwrap();\n    assert_eq!(read(r(\"native-reuse.json\"))[\"native_tests_and_builds_reused_not_rerun\"],true);\n"));
 s=s.replace("const OLD: &str = \"/home/ubuntu/src/zopro-perf-work/loose-g1-finish174\";",&format!("const OLD: &str = \"{P}\";"));
 s=s.replace("15919","16411").replace("15865","15919").replace("15811","15865");
 s=s.replace("            r(if v==\"OFF\" {\"native/control-sdk\"} else {\"native/candidate-sdk\"})",&format!("            Path::new(\"{P}\").join(if v==\"OFF\" {{\"native/control-sdk\"}} else {{\"native/candidate-sdk\"}})"));
 let old="    let all=read(\"/home/ubuntu/src/zopro-perf-work/g2-phased-inverse174/cases.json\");let cases=json!([all[0].clone()]);\n    assert_eq!(cases.as_array().unwrap().len(), 1);";
 assert!(s.contains(old));s=s.replace(old,"    let mut cases=read(\"/home/ubuntu/src/zopro-perf-work/sdk-control-copies/cases.json\");assert_eq!(cases.as_array().unwrap().len(),9);let mut physical=cases[0].clone();physical[\"name\"]=json!(\"1x2-w16\");physical[\"workers\"]=json!(16);physical[\"cpu_affinity\"]=json!(\"0-15\");cases.as_array_mut().unwrap().push(physical);assert_eq!(cases.as_array().unwrap().len(),10);");
 s=s.replace("j[\"preflight\"]==true&&j[\"variant\"]==\"ON\"","j[\"preflight\"]==true&&j[\"variant\"]==\"ON\"&&j[\"case\"][\"name\"]==\"1x2-w32\"");
 s=s.replace("if pre && v==\"ON\"", "if pre && v==\"ON\" && c[\"name\"]==\"1x2-w32\"");
 s=s.replace("let mut order = vec![0];\n        order.rotate_left(0);","let mut order = (0..cases.as_array().unwrap().len()).collect::<Vec<usize>>();\n        let count=order.len();order.rotate_left(b%count);");
 s=s.replace("jobs.len(), 48)","jobs.len(), 480)").replace("steps.len(), 96)","steps.len(), 960)").replace("jobs.len(), 54)","jobs.len(), 492)").replace("steps.len(), 117)","steps.len(), 1002)");
 s=s.replace("7806","27948").replace("54 exports","492 exports").replace("117 steps","1002 steps").replace("/54Gnark/","/492Gnark/").replace("inventory.len(), 108","inventory.len(), 984").replace("\"new_independent_exports\":54","\"new_independent_exports\":492");
 s=s.replace("counters.len(),3","counters.len(),6").replace("profiles.len(),3","profiles.len(),6");
 for f in ["native-completion.json","native-repair/result.json","native/failure.json","repair-note.json","repair-plan.json","build-pins.json"]{s=s.replace(&format!("        \"{f}\",\n"),"");}
 s=s.replace("        \"native/result.json\",","        \"native/result.json\",\n        \"native-reuse.json\",");
 let a=s.find("            if a == \"OFF\" {").unwrap();let b=a+s[a..].find("\n        if c[\"workers\"] == 32 && !c[\"reference\"].is_null()").unwrap();
 s.replace_range(a..b,"            let pass=if a==\"OFF\"{let value=s[3]>=1./1.03&&s[11]<=1.03;calibration.insert(key.clone(),value);value}else if case==\"1x2-w32\"{s[11]<1.&&wins>=10}else if c[\"workers\"]==16{s[11]<=1.02}else{s[11]<=1.03};\n            checks.insert(key.clone(),pass);if !pass{failed.push(key);}\n        }");
 s=s.replace("completed balanced K/H windows primary SDK screen;no adoption","completed balanced K/H full SDK performance qualification;no adoption").replace("balanced12/13bit K/H windows;15blocks primary32 only;not larger/all8/clean/resource qualification","balanced12/13bit K/H windows;15blocks all9shapes32workers plus primary16;portable/resource/production qualification outstanding");
 s=s.replace("    let a = json!({\"status\"", "    assert_eq!(refs.len(),8);let candidate_rows=refs.values().filter(|r|r[\"variants\"][\"ON\"][\"CPU10x\"]==true).count();let accepted_rows=refs.values().filter(|r|r[\"variants\"][\"accepted\"][\"CPU10x\"]==true).count();\n    let a = json!({\"status\"");
 s=s.replace("\"accepted_original8_CPU10x_rows\":0","\"accepted_original8_CPU10x_rows\":accepted_rows,\"candidate_original8_CPU10x_rows\":candidate_rows,\"native_tests_reused_from_pilot\":true");
 fs::write(Path::new(R).join("capture.rs"),s).unwrap();
 let s=fs::read_to_string(Path::new(P).join("close.rs")).unwrap().replace(P,R).replace("2026-10-28-balanced-kh-windows174","2026-10-28-balanced-kh-qualification175").replace("15919","16411");fs::write(Path::new(R).join("close.rs"),s).unwrap();
}
