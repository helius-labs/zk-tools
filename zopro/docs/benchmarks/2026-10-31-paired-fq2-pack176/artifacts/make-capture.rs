use std::{fs,path::Path};
fn main(){let root="/home/ubuntu/src/zopro-perf-work/paired-fq2-pack176";let r=Path::new(root);
 let mut s=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/r256-truncated-caller175/capture.rs").unwrap();
 for(a,b)in[
 ("/home/ubuntu/src/zopro-perf-work/r256-truncated-caller175",root),
 ("build-pins.json","build-pins-rest.json"),
 ("[\"g1-owned\", \"g1-borrowed-signed\", \"g2-borrowed-signed\"]","[\"g2-borrowed-signed\"]"),
 ("[(\"off\", 33, 128), (\"on\", 35, 128)]","[(\"off\", 36, 129), (\"on\", 36, 129)]"),
 ("cs.len(), 18","cs.len(), 6"),("0..31","0..15"),("0..18","0..6"),("b % 18","b % 6"),
 ("c[\"size\"] == 200 && c[\"topology\"] == \"logical32\" && c[\"kind\"] != \"g1-owned\"","c[\"size\"] == 200 && c[\"topology\"] != \"solo\""),
 ("jobs.len(), 1732","jobs.len(), 292"),("steps.len(), 1748","steps.len(), 308"),
 ("v.len(), 31","v.len(), 15"),("s[15]","s[7]"),("s[9]","s[3]"),("s[21]","s[11]"),
 ("all18","all6"),("logical32_6","logical32_2"),("{ 18 } else { 6 }","{ 6 } else { 2 }"),
 ("as_u64().unwrap() >= 21","as_u64().unwrap() >= 10"),
 ("\"processes_checked\":1732","\"processes_checked\":292"),("\"preflights\":54","\"preflights\":18"),("\"latencies\":1674","\"latencies\":270"),("\"workflow_steps\":1748","\"workflow_steps\":308"),
 ("\"off\":33,\"on\":35","\"off\":36,\"on\":36"),("\"off\":128,\"on\":128","\"off\":129,\"on\":129"),
 ("25231","29255"),("2026-10-29-balanced-kh-adoption","2026-10-30-g2-vector-sign-adoption176"),
 ("actualG1owned/G1borrowed/G2borrowedkernels onsyntheticvalidgeometry; noSDKspeedclaim","actualsignedborrowedG2kernel onsyntheticvalidgeometry; noSDKspeedclaim"),
 ("Prepared1748steps/54preflight1674latency4profiles","Prepared308steps/18preflight270latency4profiles"),
 ]{assert!(s.contains(a),"missing {a}");s=s.replace(a,b);}
 let begin=s.find("    for f in [\n        \"capture.rs\",").unwrap();let end=s[begin..].find("    ] {").unwrap()+begin+7;
 s.replace_range(begin..end,"    for f in [\"capture.rs\",\"capture-helper\",\"native/result.json\",\"codegen.json\",\"make-capture.rs\",\"design.md\",\"jobs.json\",\"contexts.json\"] {");
 assert!(!s.contains("native-attempt0"));
 s=s.replace("\"make-capture.rs\",\"design.md\",\"jobs.json\"","\"make-capture.rs\",\"design.md\",\"design-corrected.md\",\"review.rs\",\"review\",\"jobs.json\"");
 s=s.replace("        assert!(hits >= 2, \"missingG1/G2 {v}\");","        assert!(hits >= 2, \"missingG1/G2 {v}\"); if v==\"on\"{assert!(code.iter().any(|c|c[\"variant\"]==\"on\"&&c[\"nm\"].as_str().unwrap().contains(\"paired_g2_borrowed\")),\"missing actual paired constructor caller\");}");
 fs::write(r.join("capture.rs"),s).unwrap();
}
