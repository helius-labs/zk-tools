use std::{fs,path::Path};
const OLD:&str="/home/ubuntu/src/zopro-perf-work/bounded-gather-profile177";
const R:&str="/home/ubuntu/src/zopro-perf-work/g1-offset-profile177";
const P:&str="/home/ubuntu/src/zopro-perf-work/g1-offset177";
fn once(s:&mut String,a:&str,b:&str){assert_eq!(s.matches(a).count(),1,"{a}");*s=s.replace(a,b);}
fn main(){assert!(!Path::new(&format!("{R}/profile.rs")).exists());
 let common=fs::read_to_string(format!("{OLD}/common.rs")).unwrap().replace(OLD,R).replace("/home/ubuntu/src/zopro-perf-work/bounded-gather-sdk177/proof-sha256-after-main.json",&format!("{P}/proof-sha256-after-main.json"));
 fs::write(format!("{R}/common.rs"),common).unwrap();for f in["sampling.rs","metadata.rs","counter.rs"]{fs::copy(format!("{OLD}/{f}"),format!("{R}/{f}")).unwrap();}
 let mut s=fs::read_to_string(format!("{OLD}/profile.rs")).unwrap().replace("/home/ubuntu/src/zopro-perf-work/bounded-gather-sdk177",P);
 once(&mut s,"let ids=read(format!(\"{parent}/checkpoint.json\"));let case=read(format!(\"{parent}/main-jobs.json\"))[0][\"case\"].clone();assert_eq!(case[\"name\"],\"1x2-w16\");let base=read(format!(\"{parent}/main-jobs.json\"))[0][\"environment\"].clone();","let ids=read(format!(\"{parent}/runtime-consumers.json\"));check_copies();let prior_jobs=read(format!(\"{parent}/main-jobs.json\"));let job=prior_jobs.as_array().unwrap().iter().find(|j|j[\"case\"][\"name\"]==\"1x2-w32\").unwrap();let case=job[\"case\"].clone();let base=job[\"environment\"].clone();");
 once(&mut s,"let binary=ids[*arm][\"path\"].as_str().unwrap();","let binary=ids[*arm][\"copies\"][block][\"path\"].as_str().unwrap();");
 s=s.replace("\"0-15\".into()","\"0-31\".into()").replace("51892","52927").replace("51910","52945");
 once(&mut s,"let mut pins=BTreeMap::new();","let mut pins=BTreeMap::new();for arm in[\"OFF\",\"ON\"]{for f in ids[arm][\"copies\"].as_array().unwrap(){let p=f[\"path\"].as_str().unwrap();pins.insert(p.into(),hash(p));}}pins.insert(format!(\"{parent}/runtime-consumers.json\"),hash(format!(\"{parent}/runtime-consumers.json\")));");
 once(&mut s,"fn finish(){assert!(!r(\"result.json\").exists());","fn finish(){assert!(!r(\"result.json\").exists());check_copies();");
 s.push_str(r#"
fn check_copies(){use std::os::unix::fs::{MetadataExt,PermissionsExt};let ids=read("/home/ubuntu/src/zopro-perf-work/g1-offset177/runtime-consumers.json");let mut inodes=BTreeSet::new();for arm in["REF","OFF","ON"]{for f in ids[arm]["copies"].as_array().unwrap(){let p=f["path"].as_str().unwrap();let m=fs::metadata(p).unwrap();assert_eq!(hash(p),f["sha256"]);assert_eq!(m.dev(),f["device"].as_u64().unwrap());assert_eq!(m.ino(),f["inode"].as_u64().unwrap());assert_eq!(m.permissions().mode()as u64,f["mode"].as_u64().unwrap());assert!(inodes.insert((m.dev(),m.ino())));}}assert_eq!(inodes.len(),9);}
"#);
 fs::write(format!("{R}/profile.rs"),s).unwrap();println!("Prepared profile source for exact three OFF/ON backing pairs, primary32; no recording yet.");
}
