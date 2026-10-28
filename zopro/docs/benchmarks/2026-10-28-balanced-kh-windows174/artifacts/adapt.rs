use std::fs;
const R:&str="/home/ubuntu/src/zopro-perf-work/balanced-kh-windows174";
fn main(){let old="/home/ubuntu/src/zopro-perf-work/dual-plane-g2-174";
 let s=fs::read_to_string(format!("{old}/native.rs")).unwrap().replace(old,R).replace("zopro_g2_dual_plane","zopro_balanced_kh");fs::write(format!("{R}/native.rs"),s).unwrap();
 let mut s=fs::read_to_string(format!("{old}/capture.rs")).unwrap().replace(old,R).replace("const OLD: &str = \"/home/ubuntu/src/zopro-perf-work/signed-final-sub-sdk174\";","const OLD: &str = \"/home/ubuntu/src/zopro-perf-work/loose-g1-finish174\";").replace("zopro_g2_dual_plane","zopro_balanced_kh").replace("dual_plane_compile_cfg","balanced_kh_compile_cfg");
 s=s.replace("15811","15919").replace("15757","15865").replace("15553","15811");
 s=s.replace("completed joint two-plane G2 primary SDK screen;no adoption","completed balanced K/H windows primary SDK screen;no adoption").replace("joint G2 high-low bucket planes;15blocks primary32 only;one query resource reservation;not larger/all8/clean/resource qualification","balanced12/13bit K/H windows;15blocks primary32 only;not larger/all8/clean/resource qualification");
 let before="    let dir = r(\"proofs\").join(&name);";assert_eq!(s.matches(before).count(),2);s=s.replacen(before,"    if pre && v==\"ON\" {env[\"ZOPRO_BALANCED_KH_TRACE\"]=json!(\"1\");}\n    let dir = r(\"proofs\").join(&name);",1);
 let before="    assert!(!r(\"run-plan.json\").exists());";assert_eq!(s.matches(before).count(),1);s=s.replace(before,"    assert!(std::env::var_os(\"ZOPRO_BALANCED_KH_TRACE\").is_none());\n    assert!(!r(\"run-plan.json\").exists());");
 let before="    let diagnostics = analyze_diagnostics();";assert_eq!(s.matches(before).count(),1);s=s.replace(before,"    let trace=fs::read_to_string(r(\"runs/pre-1x2-w32-ON/stderr.log\")).unwrap();\n    let route_records=trace.lines().filter(|s|s.starts_with(\"BALANCED_KH windows=20 bits=255 short=5 long=15 \" )).count();assert_eq!(route_records,4);\n    let diagnostics = analyze_diagnostics();");
 fs::write(format!("{R}/capture.rs"),s).unwrap();
 let s=fs::read_to_string(format!("{old}/close.rs")).unwrap().replace(old,R).replace("2026-10-28-dual-plane-g2-174","2026-10-28-balanced-kh-windows174").replace("15811","15919");fs::write(format!("{R}/close.rs"),s).unwrap();
}
