use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/r256-truncated-caller175";
fn main() {
    let p=Path::new(R).join("capture.rs");assert!(!p.exists());
    let mut s=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/early-x175/capture.rs").unwrap();
    for(a,b)in[
        ("/home/ubuntu/src/zopro-perf-work/early-x175",R),
        ("[\"solo\",\"logical32\"]","[\"solo\",\"physical16\",\"logical32\"]"),
        ("[(\"off\",33,128),(\"on\",34,129)]","[(\"off\",33,128),(\"on\",35,128)]"),
        ("let base_dir=if v==\"off\"{\"initial-native\"}else{\"native\"};","let base_dir=\"native\";"),
        ("let binary=r(format!(\"{v}-probe\"));","let binary=PathBuf::from(ids[v][\"binary\"].as_str().unwrap());"),
        ("assert_eq!(cs.len(),12)","assert_eq!(cs.len(),18)"),
        ("0..15","0..31"),("0..12","0..18"),("b%12","b%18"),
        ("jobs.len(),580","jobs.len(),1732"),("steps.len(),596","steps.len(),1748"),
        ("\"-F\".into(),\"199\".into()","\"-c\".into(),\"16000000\".into(),\"--timestamp\".into(),\"--sample-cpu\".into()"),
        ("v.len(),15","v.len(),31"),("s[7]","s[15]"),("s[3]","s[9]"),("s[11]","s[21]"),
        ("else{32};assert_eq!(out[\"worker_count\"]","else if j[\"topology\"]==\"physical16\"{16}else{32};assert_eq!(out[\"worker_count\"]"),
        ("ev.contains(\"199\")","ev.contains(\"16000000\")&&ev.contains(\"TIME\")&&ev.contains(\"CPU\")"),
        ("all12","all18"),("{12}else{6}","{18}else{6}"),("as_u64().unwrap()>=10","as_u64().unwrap()>=21"),
        ("\"processes_checked\":580","\"processes_checked\":1732"),
        ("\"preflights\":36","\"preflights\":54"),("\"latencies\":540","\"latencies\":1674"),
        ("\"workflow_steps\":596","\"workflow_steps\":1748"),
        ("\"on\":34","\"on\":35"),("\"on\":129","\"on\":128"),
        ("20755","25231"),
        ("Prepared596steps/36preflight540latency4profiles","Prepared1748steps/54preflight1674latency4profiles"),
    ] {
        assert!(s.contains(a),"missing replacement {a}");s=s.replace(a,b);
    }
    // The reused frozen executable lives in its original pinned directory.
    assert!(!s.contains("let binary=r(format!"));
    fs::write(p,s).unwrap();
}
