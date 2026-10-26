use std::{fs, path::Path};
fn main() {
    let r = Path::new("/tmp/zopro-phased-complex-products");
    let old = fs::read_to_string("/tmp/zopro-deferred-dif32-sdk/publish.rs").unwrap();
    let at = old.find("fn main() {").unwrap();
    let main = r#"fn main(){
 let root=Path::new("/tmp/zopro-phased-complex-products");let report=Path::new("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-26-phased-complex-products");let dst=report.join("artifacts");assert!(!dst.exists());let mut index=BTreeMap::new();
 for name in["experiment.json","range-argument.md","accepted-manifest.json","prepare.json","field-block.rs.txt","implement.rs","generator.stdout.log","generator.stderr.log","source-audit.json","prepare-probe.rs","probe.rs","probe-tests.rs","probe-initial.rs","prepare-build.rs","build-input-pins.json","build-plan.json","build.log","repair.rs","repair.json","build-input-pins-v2.json","build-plan-v2.json","build-v2.log","analyze.rs","timing.rs","codegen.json","codegen.txt","timing-input-pins.json","timing-order.json","timing-plan.json","timing.log","component-result.json","profile.rs","profile-input-pins.json","profile-plan.json","profiles.log","profile-analysis.json","closeout.rs","closeout.json","write-report.rs","prepare-publish.rs","publish.rs"]{file(&root.join(name),&dst.join(name),&dst,&mut index);}
 for name in["candidate","oracle","build","build-v2","timing","profiles","profiles-data"]{tree(&root.join(name),&dst.join(name),&dst,&mut index);}
 let raw=index.keys().filter(|p|p.ends_with("/perf.data")).count();let expanded=index.keys().filter(|p|p.starts_with("profiles/")&&(p.contains("/decode-")||p.contains("/loss-"))&&p.ends_with("/stdout.log")).count();assert_eq!(raw,4);assert_eq!(expanded,8);let fixtures=index.keys().filter(|p|p.ends_with(".proof")||p.ends_with(".public")).count();assert_eq!(fixtures,2);assert!(index.keys().filter(|p|p.ends_with(".proof")||p.ends_with(".public")).all(|p|p.starts_with("candidate/")));let expected=index.len()-raw-expanded+5;
 fs::write(dst.join("SHA256SUMS"),index.iter().map(|(p,h)|format!("{h}  {p}\n")).collect::<String>()).unwrap();
 fs::write(dst.join("artifact-index.json"),serde_json::to_vec_pretty(&json!({"files":index.len(),"expected_git_files":expected,"raw_local_only":raw,"expanded_local_only":expanded,"new_proof_public_members":0,"additional_source_fixture_members":fixtures,"sha256":index,"scope":"Full source/protocol/build/test/latency/profile/archive hash index. Four raw captures and eight expanded streams locally retained, omitted Git; compiler caches and executables represented by exact pinned hashes and sources."})).unwrap()).unwrap();
 fs::write(report.join(".gitignore"),"/artifacts/profiles-data/**/perf.data\n/artifacts/profiles/decode-*/stdout.log\n/artifacts/profiles/loss-*/stdout.log\n").unwrap();fs::write(report.join(".gitattributes"),"artifacts/** -whitespace\n").unwrap();println!("Archive{}hashfiles expectedGit{expected},4raw8expandedlocalonly,0newproof2fixedfixtures",index.len());}
"#;
    fs::write(r.join("publish.rs"), old[..at].to_owned() + main).unwrap();
}
