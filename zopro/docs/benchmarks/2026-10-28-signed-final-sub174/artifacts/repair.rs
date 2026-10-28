use serde_json::{json,Value};use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs,io::Read,path::{Path,PathBuf},process::Command};
const R:&str="/home/ubuntu/src/zopro-perf-work/signed-final-sub174";
fn r(p:impl AsRef<Path>)->PathBuf{Path::new(R).join(p)}
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{let mut f=fs::File::open(p).unwrap();let mut h=Sha256::new();let mut b=[0;65536];loop{let n=f.read(&mut b).unwrap();if n==0{break;}h.update(&b[..n]);}format!("{:x}",h.finalize())}
fn prepare(){
    assert!(!r("native-initial").exists());let plan=read(r("build-plan.json"));
    let mut pins:BTreeMap<String,String>=serde_json::from_value(read(r("build-pins.json"))).unwrap();for(p,h)in &pins{assert_eq!(&hash(p),h);}
    fs::rename(r("native"),r("native-initial")).unwrap();
    let mut steps=plan["steps"].as_array().unwrap()[4..].to_vec();assert_eq!(steps.len(),6);
    for step in &mut steps{if step["kind"]=="link-sdk"{let label=step["name"].as_str().unwrap().to_string();*step=json!({"kind":"command","name":label,"argv":[r("repair-helper"),"link",label],"timeout_seconds":600});}}
    for p in ["repair.rs","repair-helper","native-initial/protocol.json","native-initial/source-pins.json","native-initial/completed.json","native-initial/failure.json","native-initial/control/stderr.log","frozen-control.json"]{pins.insert(r(p).display().to_string(),hash(r(p)));}
    save(r("repair-pins.json"),&json!(pins));save(r("build-plan-repair.json"),&json!({"version":1,"source_pins":r("repair-pins.json"),"steps":steps}));
    save(r("harness-repair.json"),&json!({"failure":"OFF caller link omitted direct sha2 extern in standard8-library link-sdk","repair":"resolve exact coherent sha2 rlib from frozen input pins, link custom caller with same flags;finish remaining six steps","completed_OFF_field_core_libraries_freeze_reused":true,"no_capture_started":true,"source_and_driver_unchanged":true,"original_protocol_and_failure_retained":"native-initial"}));
}
fn link(label:&str){
    assert!(["control","candidate"].contains(&label));let frozen=read(r(format!("frozen-{label}.json")));let mut libs=frozen["libraries"].as_object().unwrap().clone();
    let sha:Vec<_>=frozen["input_pins"].as_object().unwrap().iter().filter(|(p,_)|{let p=Path::new(p);p.file_name().unwrap().to_string_lossy().starts_with("libsha2-")&&p.extension().is_some_and(|x|x=="rlib")}).collect();assert_eq!(sha.len(),1);assert_eq!(hash(sha[0].0),sha[0].1.as_str().unwrap());libs.insert("sha2".into(),json!(sha[0].0));
    let mut c=Command::new("rustc");c.args(["--edition=2021","-O","-C","target-cpu=native","-C","lto=fat","-C","codegen-units=1","-L"]).arg(format!("dependency={}",r(format!("frozen/{label}")).display()));
    for(n,p)in &libs{c.arg("--extern").arg(format!("{n}={}",p.as_str().unwrap()));}
    let out=r(format!("native/{label}-sdk"));c.arg(r("probe.rs")).arg("-o").arg(&out);
    save(r(format!("link-{label}-command.json")),&json!({"program":"rustc","argv":c.get_args().map(|s|s.to_string_lossy().into_owned()).collect::<Vec<_>>()}));
    let result=c.output().unwrap();fs::write(r(format!("link-{label}.stdout.log")),&result.stdout).unwrap();fs::write(r(format!("link-{label}.stderr.log")),&result.stderr).unwrap();assert!(result.status.success(),"{}",String::from_utf8_lossy(&result.stderr));
    save(r(format!("native/{label}/artifacts.json")),&json!({"libraries":libs,"driver":r("probe.rs"),"driver_sha256":hash(r("probe.rs")),"additional_source_sha256":hash(r("perf.rs")),"binary":out,"binary_sha256":hash(&out),"direct_sha2_extern_repaired":true}));
    println!("Linked{label} caller with coherent9 direct libraries including sha2");
}
fn close(){
    assert_eq!(read(r("native/result.json"))["status"],"complete");
    for name in ["zopro-bn254-control","zopro-core-control","libraries-control","freeze-control"]{
        let from=r("native-initial").join(name);let to=r("native").join(name);assert!(!to.exists());fs::create_dir(&to).unwrap();
        for e in fs::read_dir(&from).unwrap(){let e=e.unwrap();assert!(e.file_type().unwrap().is_file());fs::copy(e.path(),to.join(e.file_name())).unwrap();}
    }
    save(r("native-completion.json"),&json!({"unique_successful_steps":10,"repaired_workflow_steps":6,"reused_initial_steps":4,"old_link_failed_before_any_capture":true,"no_tests_or_measurements_repeated":true,"status":"complete"}));
}
fn main(){let a:Vec<_>=std::env::args().collect();match a[1].as_str(){"prepare"=>prepare(),"link"=>link(&a[2]),"close"=>close(),_=>panic!("prepare/link/close")}}
