use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/serial-phi175";
fn main(){let r=Path::new(R);assert!(!r.join("native.rs").exists());let mut s=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/b2-group-only174/native.rs").unwrap().replace("/home/ubuntu/src/zopro-perf-work/b2-group-only174",R).replace("2026-10-28-g2-borrowed-adoption","2026-10-29-balanced-kh-adoption").replace("/home/ubuntu/src/zopro-perf-work/g2-borrowed-production/native.rs","/home/ubuntu/src/zopro-perf-work/balanced-kh-production/native.rs").replace("ZOPRO_CPU_SKIP_B2_GROUP_PROBE","ZOPRO_CPU_SERIAL_PHI_PROBE");
s=s.replace("len(), 174)","len(), 175)").replace("exact, 173)","exact, 174)").replace("\"original_files\":174","\"original_files\":175").replace("\"originals_exact\":173","\"originals_exact\":174").replace("reverse174_exact","reverse175_exact").replace("173exact/reverse174, onlyB2callguard/globalDEDUP1/forecastsunchanged","174exact/reverse175, onlyscalar-loop/globalDEDUP1/forecastsunchanged");
let a=s.find("    let file = \"crates/zopro-core/src/msm_affine.rs\";").unwrap();let b=a+s[a..].find("    let p = r(\"source\").join(file);").unwrap();s.replace_range(a..b,r#"    let file = "crates/zopro-core/src/msm_affine/g2_phi.rs";
    let before="    short.par_iter_mut().for_each(|s| *s = phi_split(*s));";
    let after="    if std::env::var(\"ZOPRO_CPU_SERIAL_PHI_PROBE\").as_deref() == Ok(\"1\") {\n        short.iter_mut().for_each(|s| *s = phi_split(*s));\n    } else {\n        short.par_iter_mut().for_each(|s| *s = phi_split(*s));\n    }";
"#);
s=s.replace("\"native.rs\",","\"bootstrap.rs\",\"history-audit.md\",\"native.rs\",");
fs::write(r.join("native.rs"),s).unwrap();println!("Prepared native controller, one inverse-audited loop edit; exact current175manifest");}
