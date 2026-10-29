use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/g2-part-ready175";
fn main(){let r=Path::new(R);assert!(!r.join("capture.rs").exists());let mut s=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/serial-phi175/capture.rs").unwrap().replace("/home/ubuntu/src/zopro-perf-work/serial-phi175",R).replace("ZOPRO_CPU_SERIAL_PHI_PROBE","ZOPRO_CPU_PHI_PART_READY").replace("\"serial\"","\"ready\"").replace("/serial/","/ready/").replace("17830","20509").replace("18076","20755");
let a=s.find("    let mut prior: BTreeSet<String> =").unwrap();let b=a+s[a..].find("    save(r(\"prior-proof-sha256.json\"), &json!(prior));").unwrap();s.replace_range(a..b,r#"    let previous = "/home/ubuntu/src/zopro-perf-work/proof-pgo-c7a175/proof-sha256-after.json";
    let prior: BTreeSet<String> = serde_json::from_value(read(previous)).unwrap();
    assert_eq!(prior.len(),20509);
    pin(&mut pins,previous);
"#);
s=s.replace("read(MANIFEST)[\"rustflags\"]\n    );", "json!(format!(\"{} --cfg zopro_phi_part_ready\",read(MANIFEST)[\"rustflags\"].as_str().unwrap()))\n    );");
s=s.replace("\"make-capture.rs\",\"history-audit.md\",\"audit.rs\",\"audit\",\"code-audit.json\",\"archive.rs\",\"archive\",\"state.rs\",\"initial-code-audit/audit.rs\",\"initial-code-audit/failure.json\",\"capture.rs\",", "\"make-capture.rs\",\"history-audit.md\",\"audit.rs\",\"audit\",\"code-audit.json\",\"capture.rs\",");
s=s.replace("\"OFF\":124,\"ON\":124","\"OFF\":127,\"ON\":127").replace("completed serial phi_split actual SDK experiment", "completed partition-ready G2 actual SDK experiment");
fs::write(r.join("capture.rs"),s).unwrap();}
