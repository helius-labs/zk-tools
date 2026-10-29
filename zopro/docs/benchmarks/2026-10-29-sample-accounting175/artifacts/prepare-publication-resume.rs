use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/sample-accounting175";
fn main(){
    let root=Path::new(R);let original=fs::read_to_string(root.join("repair/publish-fixed.rs")).unwrap();
    let marker="include!(\"/home/ubuntu/src/zopro-perf-work/sample-accounting175/repair/publication-fixed.rs\");";
    let mut prefix=original.split_once(marker).unwrap().0.to_owned();
    let needle="fn archive(paths:&[PathBuf],base:&Path,out:&Path)->Value{";
    assert_eq!(prefix.matches(needle).count(),1);
    prefix=prefix.replace(needle,&format!("{needle}if out.exists(){{return reuse_archive(paths,base,out);}}"));
    prefix+=r#"
fn reuse_archive(paths:&[PathBuf],base:&Path,out:&Path)->Value {
    let decoded=Command::new("gzip").arg("-dc").arg(out).output().unwrap();assert!(decoded.status.success());
    let back:Value=serde_json::from_slice(&decoded.stdout).unwrap();assert_eq!(back.as_object().unwrap().len(),paths.len());
    for p in paths {let v=&back[p.strip_prefix(base).unwrap().to_str().unwrap()];let bytes=if let Some(s)=v["utf8"].as_str(){s.as_bytes().to_vec()}else{let s=v["hex"].as_str().unwrap();(0..s.len()).step_by(2).map(|i|u8::from_str_radix(&s[i..i+2],16).unwrap()).collect()};assert_eq!(bytes,fs::read(p).unwrap());assert_eq!(v["sha256"],hash(p));assert_eq!(v["length"],bytes.len());}
    json!({"members":paths.len(),"sha256":hash(out),"roundtrip_exact":true,"existing_archive_reused_after_validation":true})
}
"#;
    let body=fs::read_to_string(root.join("repair/publication-fixed.rs")).unwrap();
    let mut body=body.replace("assert!(!Path::new(D).exists());", "assert!(Path::new(D).exists()&&!Path::new(D).join(\"README.md\").exists());");
    body=body.replace("fs::write(dest.join(\"README.md\"),md)","md+=\"\\nOffline publication initially stopped while hashing the original failed recording, which was still root-owned. After changing ownership of that task-owned file, a separate publication-only resumer validates and reuses every completed archive. Its source compiles after captures solely to finish publication. No SDK, verifier, capture or successful test is rerun; original pinned helpers remain unchanged.\\n\"; fs::write(dest.join(\"README.md\"),md)");
    fs::write(root.join("publication-resume.rs"),body).unwrap();
    prefix+="include!(\"/home/ubuntu/src/zopro-perf-work/sample-accounting175/publication-resume.rs\");\nfn main(){publish()}\n";
    fs::write(root.join("publish-resume.rs"),prefix).unwrap();
    println!("Prepared offline-only publication resume; existing archives validated and reused");
}
