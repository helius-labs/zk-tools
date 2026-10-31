use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs,io::Read,path::{Path,PathBuf},process::Command};
const R:&str="/home/ubuntu/src/zopro-perf-work/semantic-cse176";
const D:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-31-semantic-cse176";
fn r(s:&str)->PathBuf{Path::new(R).join(s)}
fn d(s:&str)->PathBuf{Path::new(D).join(s)}
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{let mut f=fs::File::open(p).unwrap();let mut h=Sha256::new();let mut b=[0;65536];loop{let n=f.read(&mut b).unwrap();if n==0{break}h.update(&b[..n]);}format!("{:x}",h.finalize())}
fn walk(p:&Path,out:&mut Vec<PathBuf>){for e in fs::read_dir(p).unwrap(){let e=e.unwrap();let t=e.file_type().unwrap();assert!(!t.is_symlink());let p=e.path();if t.is_dir(){walk(&p,out)}else{assert!(t.is_file());out.push(p)}}}
fn pins(name:&str){for(p,h)in read(r(name)).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap(),"{p}");}}
fn receipts(root:&str,steps:usize)->Value{
    let v=read(r(&format!("{root}/result.json")));assert_eq!(v["status"],"complete");assert_eq!(v["steps"].as_array().unwrap().len(),steps);
    for s in v["steps"].as_array().unwrap(){let p=r(&format!("{root}/{}",s.as_str().unwrap()));let e=read(p.join("exit.json"));assert_eq!(e["success"],true);assert_eq!(e["returncode"],0);
        for stream in ["stdout","stderr"]{assert_eq!(hash(p.join(format!("{stream}.log"))),e[format!("{stream}_sha256")]);}
        if p.join("artifacts.json").exists(){let a=read(p.join("artifacts.json"));for(p,h)in a["input_pins"].as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}assert_eq!(hash(a["binary"].as_str().unwrap()),a["binary_sha256"]);}
    }
    let mut paths=vec![];walk(&r(root),&mut paths);paths.sort();let mut out=BTreeMap::new();for p in paths{let name=p.strip_prefix(r(root)).unwrap().display().to_string();if name.ends_with("-sdk"){continue}assert!(p.extension().is_some_and(|x|x=="json"||x=="log"));out.insert(name,fs::read_to_string(p).unwrap());}json!(out)
}
fn main(){
    pins("pins.json");pins("review-pins.json");
    let a=read(r("analysis.json"));let v=read(r("review.json"));assert_eq!(a["complete"],true);assert_eq!(a["opportunity"],true);assert_eq!(v["passed"],true);assert_eq!(a["new_proofs"],0);assert_eq!(a["new_timing_samples"],0);
    let rec=json!({"runs":receipts("runs",5),"review-runs":receipts("review-runs",2)});let raw=serde_json::to_vec(&rec).unwrap();fs::write(r("receipts.json"),&raw).unwrap();
    assert!(!Path::new(D).exists());fs::create_dir_all(d("artifacts")).unwrap();
    for name in ["analysis.json","1x2.json","2x2.json","review.json","model.rs","semantic.rs","semantic-tests.rs","census.rs","workflow.rs","setup-model.rs","setup-workflow.rs","prepare-review.rs","review.rs","publish.rs","publish-attempt-1.rs","publication-repair.md","design.md","hint-coverage.md","inputs.json","pins.json","plan.json","review-pins.json","review-plan.json","setup.log","setup-model.log","setup-workflow.log","workflow.log","finish.log","review-setup.log","review-workflow.log"]{fs::copy(r(name),d("artifacts").join(name)).unwrap();}
    fs::copy(r("report.md"),d("README.md")).unwrap();
    fs::copy("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json",d("artifacts/accepted-manifest.json")).unwrap();
    let gz=Command::new("gzip").args(["-n","-c"]).arg(r("receipts.json")).output().unwrap();assert!(gz.status.success());fs::write(d("artifacts/receipts.json.gz"),gz.stdout).unwrap();
    let dec=Command::new("gzip").args(["-d","-c"]).arg(d("artifacts/receipts.json.gz")).output().unwrap();assert!(dec.status.success());assert_eq!(dec.stdout,raw);
    let mut files=vec![];walk(Path::new(D),&mut files);files.sort();let mut sums=String::new();for f in &files{sums.push_str(&format!("{}  {}\n",hash(f),f.strip_prefix(D).unwrap().display()));}fs::write(d("SHA256SUMS"),sums).unwrap();
    let text=fs::read_to_string(d("README.md")).unwrap();let mut links=0;for part in text.split("](").skip(1){let target=part.split(')').next().unwrap();assert!(!target.contains("://"));assert!(Path::new(D).join(target).is_file(),"{target}");links+=1;}
    fs::write(r("publication-review.json"),serde_json::to_vec_pretty(&json!({"complete":true,"published_files":files.len()+1,"checksums":files.len(),"relative_links":links,"receipt_archive_exact":true,"main_steps":5,"independent_steps":2,"all_pins_exact":true,"private_solver_or_alias_records_published":false,"new_proofs":0,"new_timing_samples":0})).unwrap()).unwrap();
    println!("Published semantic census: {} files, {} checksums, {links} links; all pins/receipts and archive roundtrip exact",files.len()+1,files.len());
}
