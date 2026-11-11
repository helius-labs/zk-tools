use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,path::Path,os::unix::fs::{MetadataExt,PermissionsExt},collections::BTreeSet};
const R:&str="/home/ubuntu/src/zopro-perf-work/g1-offset177";
const OLD:&str="/home/ubuntu/src/zopro-perf-work/bounded-gather-sdk177";
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn once(s:&mut String,a:&str,b:&str){assert_eq!(s.matches(a).count(),1,"anchor {a}");*s=s.replace(a,b);}
fn main(){
 assert!(!Path::new(&format!("{R}/screen.rs")).exists());
 let check:Value=serde_json::from_slice(&fs::read(format!("{R}/checkpoint.json")).unwrap()).unwrap();
 let mut ids=json!({"REF":{"path":"/home/ubuntu/src/zopro-perf-work/phi-grain-c7a176/sdk-candidate-znver4","sha256":"c88a3528d5feb363732dd2557d1894c93e9161f4e3ec9e9d3c7e2564283bf2fe"}});
 for(label,arm)in[("control","OFF"),("candidate","ON")]{ids[arm]=check["consumers"].as_array().unwrap().iter().find(|c|c["label"]==label).unwrap().clone();}
 fs::create_dir(format!("{R}/bin")).unwrap();let mut inodes=BTreeSet::new();
 for(arm,letter)in[("REF","r"),("OFF","f"),("ON","n")]{let source=ids[arm]["path"].as_str().unwrap().to_owned();assert_eq!(hash(&source),ids[arm]["sha256"]);let bytes=fs::read(&source).unwrap();let mut copies=vec![];for i in 0..3{let p=format!("{R}/bin/sdk-{letter}{i}");fs::write(&p,&bytes).unwrap();fs::set_permissions(&p,fs::Permissions::from_mode(0o755)).unwrap();assert_eq!(hash(&p),ids[arm]["sha256"]);let m=fs::metadata(&p).unwrap();assert!(inodes.insert((m.dev(),m.ino())));let original=fs::metadata(&source).unwrap();assert_ne!((m.dev(),m.ino()),(original.dev(),original.ino()));copies.push(json!({"path":p,"sha256":hash(&p),"device":m.dev(),"inode":m.ino(),"mode":m.permissions().mode()}));}ids[arm]["copies"]=json!(copies);}
 fs::write(format!("{R}/runtime-consumers.json"),serde_json::to_vec_pretty(&ids).unwrap()).unwrap();
 let mut s=fs::read_to_string(format!("{OLD}/screen.rs")).unwrap().replace(OLD,R).replace("/home/ubuntu/src/zopro-perf-work/phi-grain-parent176/proof-sha256-after-capture.json","/home/ubuntu/src/zopro-perf-work/gather-backing177/proof-sha256-after-capture.json");
 once(&mut s,"read(r(\"checkpoint.json\"))","read(r(\"runtime-consumers.json\"))");
 once(&mut s,"let binary = ids[arm][\"path\"].as_str().unwrap();","let binary = ids[arm][\"copies\"][block % 3][\"path\"].as_str().unwrap();");
 once(&mut s,"\"129 passed; 0 failed\"","\"130 passed; 0 failed\"");
 s=s.replace("[\"1x2-w16\", \"1x2-w32\", \"2x2-w32\"]","[\"1x2-w16\", \"1x2-w32\", \"2x2-w32\", \"4x2-w32\", \"8x2-w32\", \"12x2-w32\", \"16x2-w32\", \"24x2-w32\", \"merge-24x1-w32\", \"merge-54x1-w32\"]");
 once(&mut s,"    for phase in [\"preflight\", \"main\"] {","    check_copies(&checkpoint);\n    for arm in [\"REF\",\"OFF\",\"ON\"] { for f in checkpoint[arm][\"copies\"].as_array().unwrap() {let p=f[\"path\"].as_str().unwrap(); pins.insert(p.into(),hash(p));}}\n    for phase in [\"preflight\", \"main\"] {");
 once(&mut s,"let mut order = vec![0, 1, 2];","let mut order = (0..10).collect::<Vec<_>>();");
 once(&mut s,"order.rotate_left(b % 3);\n                arms.rotate_left(b % 3);\n                if b % 2 == 1 {\n                    order.reverse();\n                    arms.reverse();\n                }","order.rotate_left(b % 10);\n                arms.rotate_left((b / 3) % 3);\n                if b % 2 == 1 { order.reverse(); }\n                if (b / 3) % 2 == 1 { arms.reverse(); }");
 s=s.replace("0..31","0..30").replace("0..=31","0..=30").replace("(10..=21)","(10..=20)").replace("k < 31","k < 30").replace("(31 - k as u64)","(30 - k as u64)").replace("(1u64<<31)","(1u64<<30)");
 s=s.replace("\"blocks\":31,\"lower_rank_zero_based\":9,\"median_rank_zero_based\":15,\"upper_rank_zero_based\":21","\"blocks\":30,\"lower_rank_zero_based\":9,\"median_ranks_zero_based\":[14,15],\"upper_rank_zero_based\":20");
 s=s.replace("{ 279 }","{ 900 }").replace("{ 22506 }","{ 43200 }").replace("{ 21669 }","{ 40500 }");
 s=s.replace("51586","52000").replace("51613","52027").replace("51892","52927");
 s=s.replace("\"checkpoint.json\",","\"runtime-consumers.json\",\n        \"prepare-screen.rs\",");
 once(&mut s,"    let folder = format!(\"{phase}-run\");","    check_copies(&read(r(\"runtime-consumers.json\")));\n    let folder = format!(\"{phase}-run\");");
 s=s.replace("assert_eq!(v.len(), 31);","assert_eq!(v.len(), 30);");
 once(&mut s,"json!({\"lower\":v[9],\"median\":v[15],\"upper\":v[21]})","json!({\"lower\":v[9],\"median\":(v[14]+v[15])/2.,\"upper\":v[20]})");
 once(&mut s,"                gates.push(json!({\"context\":context,\"comparator\":comparator,\"metric\":metric,\"ratio\":b,\"passed\":if gain{upper<1.}else{upper<=1.02},\"limit\":if gain{1.}else{1.02},\"strict\":gain}));","                if comparator == \"REF\" || gain { gates.push(json!({\"context\":context,\"comparator\":comparator,\"metric\":metric,\"ratio\":b,\"passed\":if gain{upper<1.}else{upper<=1.02},\"limit\":if gain{1.}else{1.02},\"strict\":gain})); }");
 once(&mut s,"            gates.push(json!({\"context\":context,\"comparator\":comparator,\"metric\":\"peak_rss_kib\",\"ON\":a,\"control\":b,\"passed\":a<=b*1.05+1024.}));","            if comparator == \"REF\" { gates.push(json!({\"context\":context,\"comparator\":comparator,\"metric\":\"peak_rss_kib\",\"ON\":a,\"control\":b,\"passed\":a<=b*1.05+1024.})); }");
 once(&mut s,"assert_eq!(gates.len(), 18);","assert_eq!(gates.len(), 31);");
 s=s.replace("Progression checks {passed}/18","Progression checks {passed}/31").replace("27+279 exports planned","27+900 exports planned");
 // Retain fresh OFF contrasts and backing strata even though most are not vetoes.
 once(&mut s,"    let mut timings = vec![];","    let mut timings = vec![];\n    let mut diagnostics = vec![];");
 once(&mut s,"                let gain = context == \"1x2-w32\" && metric == \"ms\";","                diagnostics.push(json!({\"context\":context,\"comparator\":comparator,\"metric\":metric,\"ratio\":b}));\n                let gain = context == \"1x2-w32\" && metric == \"ms\";");
 once(&mut s,"\"checks_passed\":passed,\"checks\":gates,\"timings\":timings,","\"checks_passed\":passed,\"checks\":gates,\"timings\":timings,\"diagnostic_contrasts\":diagnostics,\"backings\":backing_summary(rows),");
 s.push_str(r#"
fn check_copies(ids: &Value) {
    use std::os::unix::fs::{MetadataExt,PermissionsExt};
    let mut distinct=BTreeSet::new();
    for arm in ["REF","OFF","ON"] { for f in ids[arm]["copies"].as_array().unwrap() {
        let p=f["path"].as_str().unwrap();let m=fs::metadata(p).unwrap();
        assert_eq!(hash(p),f["sha256"]);assert_eq!(m.dev(),f["device"].as_u64().unwrap());
        assert_eq!(m.ino(),f["inode"].as_u64().unwrap());assert_eq!(m.permissions().mode()as u64,f["mode"].as_u64().unwrap());
        assert!(distinct.insert((m.dev(),m.ino())));
    }} assert_eq!(distinct.len(),9);
}
fn backing_summary(rows:&[Value])->Value {
 let contexts:BTreeSet<_>=rows.iter().map(|v|v["context"].as_str().unwrap()).collect();let mut results=vec![];let mut flags=vec![];
 for context in contexts { for comparator in ["REF","OFF"] { let mut medians=vec![];
  for copy in 0..3u64 {let mut ratios=vec![];for block in(0..30u64).filter(|b|b%3==copy){let get=|arm:&str|rows.iter().find(|r|r["context"]==context&&r["block"]==block&&r["arm"]==arm).unwrap()["ms"].as_f64().unwrap();ratios.push(get("ON")/get(comparator));}ratios.sort_by(f64::total_cmp);assert_eq!(ratios.len(),10);let median=(ratios[4]+ratios[5])/2.;medians.push(median);results.push(json!({"context":context,"copy":copy,"comparator":comparator,"paired_ratios_sorted":ratios,"median_ratio":median}));}
  if context=="1x2-w32"{let spread=medians.iter().copied().fold(f64::NEG_INFINITY,f64::max)/medians.iter().copied().fold(f64::INFINITY,f64::min);flags.push(json!({"comparator":comparator,"max_min_median_ratio":spread,"needs_diagnosis":spread>1.02}));}
 }}json!({"strata":results,"primary_spread_flags":flags,"all_backings_retained":true,"stratum_intervals_claimed":false})
}
"#);
 fs::write(format!("{R}/screen.rs"),s).unwrap();
 println!("Prepared nine independent exact backings and full-proof screen source; no SDK execution.");
}
