use serde_json::{json,Value};use sha2::{Digest,Sha256};
use std::{collections::{BTreeMap,BTreeSet},fs,path::{Path,PathBuf}};
const R:&str="/tmp/zopro-borrowed-g1";
const TOOL:&str="/tmp/zopro-xtask-fast-screen-target/release/xtask";
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn add(p:impl AsRef<Path>,pins:&mut BTreeMap<String,String>){let p=p.as_ref();pins.insert(p.display().to_string(),hash(p));}
fn median(mut v:Vec<f64>)->f64{v.sort_by(f64::total_cmp);let n=v.len();assert!(n>0);if n%2==1{v[n/2]}else{(v[n/2-1]+v[n/2])/2.}}
fn gm(v:&[f64])->f64{(v.iter().map(|x|x.ln()).sum::<f64>()/v.len()as f64).exp()}
fn prior()->BTreeSet<String>{
 let mut set:BTreeSet<String>=serde_json::from_value(read("/tmp/zopro-g2-tree-sdk/context/manifest.json")["prior_proof_sha256"].clone()).unwrap();assert_eq!(set.len(),5958);
 for p in ["/tmp/zopro-g2-tree-sdk/proof-inventory.json","/tmp/zopro-g1-quarter-current/proof-inventory.json","/tmp/zopro-pairing-x-chain-sdk/proof-inventory.json","/tmp/zopro-control-rescheduling/all-proof-inventory.json","/tmp/zopro-pairing-clean/proof-inventory.json","/tmp/zopro-borrowed-pending/proof-inventory.json"]{
  for(file,h)in read(p).as_object().unwrap(){assert_eq!(hash(file),h.as_str().unwrap());if file.ends_with(".proof"){assert!(set.insert(h.as_str().unwrap().into()));}}
 }assert_eq!(set.len(),6886);set
}
fn prepare(){
 let r=Path::new(R);assert_eq!(read(r.join("native-repair/result.json"))["status"],"complete");
 let field=read(r.join("native/field/tests.json"));assert_eq!(field["passed_tests"].as_array().unwrap().len(),33);
 let core=read(r.join("native-repair/core/tests.json"));assert_eq!(core["passed_tests"].as_array().unwrap().len(),114);
 let mut c=read("/tmp/zopro-pairing-x-chain-sdk/perf-plan.json");
 let mut pins:BTreeMap<String,String>=serde_json::from_value(c["pins"].clone()).unwrap();
 pins.extend(serde_json::from_value::<BTreeMap<String,String>>(read(r.join("source-pins.json"))).unwrap());
 let a=read(r.join("native-repair/on/artifacts.json"));add(r.join("native-repair/on/artifacts.json"),&mut pins);
 for(p,h)in a["input_pins"].as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());add(p,&mut pins);}
 assert_eq!(hash(a["binary"].as_str().unwrap()),a["binary_sha256"].as_str().unwrap());add(a["binary"].as_str().unwrap(),&mut pins);
 for p in [r.join("experiment"),r.join("native-plan.json"),r.join("native-repair/result.json"),r.join("implementation-audit.json"),PathBuf::from(TOOL),PathBuf::from("/tmp/zopro-pairing-clean/proof-inventory.json")]{add(p,&mut pins);}
 for(p,h)in &pins{assert_eq!(&hash(p),h,"pin {p}");}
 c["variants"]=json!([
 {"name":"accepted","binary":"/tmp/zopro-grouping-module/native/candidate-sdk","environment":{"ZOPRO_CPU_BORROWED_PENDING":"0","ZOPRO_CPU_BORROWED_ORDINARY":"0"}},
 {"name":"owned","binary":a["binary"],"environment":{"ZOPRO_CPU_BORROWED_PENDING":"0","ZOPRO_CPU_BORROWED_ORDINARY":"0"}},
 {"name":"kh","binary":a["binary"],"environment":{"ZOPRO_CPU_BORROWED_PENDING":"1","ZOPRO_CPU_BORROWED_ORDINARY":"0"}},
 {"name":"all","binary":a["binary"],"environment":{"ZOPRO_CPU_BORROWED_PENDING":"1","ZOPRO_CPU_BORROWED_ORDINARY":"1"}}]);
 c["pins"]=json!(pins);c["prior_proof_sha256"]=json!(prior());save(r.join("perf-before-preflight.json"),&c);
 let mut steps=Vec::new();let mut jobs=Vec::new();
 for case in c["cases"].as_array().unwrap().iter().filter(|x|x["workers"]==32){for variant in ["owned","all"]{
  let name=format!("{}-w4-{variant}",case["shape"].as_str().unwrap());
  let stem=Path::new(case["image"].as_str().unwrap()).file_stem().unwrap();let d=r.join("preflight-proofs").join(&name).join(stem);
  let mut env=c["environment"].clone();env["RAYON_NUM_THREADS"]=json!("4");env["ZOPRO_CPU_BORROWED_PENDING"]=json!(if variant=="all"{"1"}else{"0"});env["ZOPRO_CPU_BORROWED_ORDINARY"]=json!(if variant=="all"{"1"}else{"0"});
  steps.push(json!({"kind":"command","name":name,"argv":["taskset","-c","0-3",a["binary"],case["image"],case["request"],"1",d],"environment":env,"timeout_seconds":600}));
  steps.push(json!({"kind":"command","name":format!("verify-{name}"),"argv":["/tmp/zopro-gnark-verify/verify","/tmp/zopro-keys",d.parent().unwrap(),"/home/ubuntu/src/zk-tools/zopro/tools/zolana/zopro-keys.lock"],"timeout_seconds":120}));
  jobs.push(json!({"name":name,"directory":d,"public_sha256":case["public_sha256"]}));
 }}
 save(r.join("preflight-jobs.json"),&json!(jobs));save(r.join("preflight-pins.json"),&json!(pins));
 save(r.join("preflight-plan.json"),&json!({"version":1,"source_pins":r.join("preflight-pins.json"),"steps":steps}));
 println!("All-nine OFF/ON preflight ready:72SDK18exports; prior6886");
}

fn qualify() {
    let r = Path::new(R);
    assert_eq!(read(r.join("preflight/result.json"))["status"], "complete");
    let mut c = read(r.join("perf-before-preflight.json"));
    let mut pins: BTreeMap<String, String> = serde_json::from_value(c["pins"].clone()).unwrap();
    let mut seen = prior();
    let mut inventory = BTreeMap::new();
    for job in read(r.join("preflight-jobs.json")).as_array().unwrap() {
        let d = Path::new(job["directory"].as_str().unwrap());
        let proof = d.join("0.proof");
        let public = d.join("0.public");
        assert_eq!(fs::metadata(&proof).unwrap().len(), 164);
        assert_eq!(hash(&public), job["public_sha256"].as_str().unwrap());
        assert!(seen.insert(hash(&proof)));
        for p in [proof, public] {
            add(&p, &mut pins);
            inventory.insert(p.display().to_string(), hash(p));
        }
        let name = job["name"].as_str().unwrap();
        let log = fs::read_to_string(r.join(format!("preflight/{name}/stdout.log"))).unwrap();
        let summary: Value = serde_json::from_str(
            log.lines()
                .find_map(|l| l.strip_prefix("SUMMARY "))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(summary["rounds"], 1);
        assert_eq!(summary["priming"], 3);
        assert_eq!(summary["workers"], 4);
        let gnark =
            fs::read_to_string(r.join(format!("preflight/verify-{name}/stdout.log"))).unwrap();
        assert!(gnark.contains("verified 1 distinct proofs across 1 pinned keys"));
    }
    assert_eq!(seen.len(), 6904);
    assert_eq!(inventory.len(), 36);
    save(r.join("preflight-proof-inventory.json"), &json!(inventory));
    save(
        r.join("preflight-analysis.json"),
        &json!({"passed":true,"sdk_requests":72,"independent_exports":18,"next_prior_count":6904}),
    );
    for f in [
        "preflight-plan.json",
        "preflight/result.json",
        "preflight-analysis.json",
        "preflight-proof-inventory.json",
    ] {
        add(r.join(f), &mut pins);
    }
    c["pins"] = json!(pins);
    c["prior_proof_sha256"] = json!(seen);
    save(r.join("perf-plan-screen.json"), &c);
    println!("All-nine preflight passes72SDK18Gnark; ready for short screen");
}

fn evaluate(stage:&str){
 let r=Path::new(R);let dir=r.join(stage);let result=read(dir.join("result.json"));assert_eq!(result["status"],"complete");
 let config=read(dir.join("protocol.json"));let rows=read(dir.join("rows.json"));let rows=rows.as_array().unwrap();let screen=stage=="screen";let blocks=if screen{3}else{7};
 let mut cases=BTreeMap::new();let mut calibration=Vec::new();let mut primary=Vec::new();let mut regressions=Vec::new();let mut original:BTreeMap<&str,Vec<f64>>=[("accepted",vec![]),("owned",vec![]),("kh",vec![])].into_iter().collect();
 for case in config["cases"].as_array().unwrap(){let name=case["name"].as_str().unwrap();let mut modes=BTreeMap::new();let mut pairs=BTreeMap::new();
  for variant in ["accepted","owned","kh","all"]{
   let rs:Vec<_>=rows.iter().filter(|x|x["phase"]=="latency"&&x["case"]==name&&x["variant"]==variant).collect();assert_eq!(rs.len(),blocks);
   modes.insert(variant,json!({"ms":median(rs.iter().map(|x|x["median_ms"].as_f64().unwrap()).collect()),"maxrss_mib":rs.iter().map(|x|x["summary"]["whole_process_maxrss_kib"].as_f64().unwrap()/1024.).fold(0.,f64::max),"cpu_ms_per_request":median(rs.iter().map(|x|(x["summary"]["user_cpu_ms"].as_f64().unwrap()+x["summary"]["system_cpu_ms"].as_f64().unwrap())/x["summary"]["rounds"].as_f64().unwrap()).collect())}));
  }
  for(a,b)in [("owned","accepted"),("all","accepted"),("all","owned"),("all","kh")]{
   let mut savings=Vec::new();let ratios:Vec<f64>=(0..blocks).map(|block|{
    let ms=|v:&str|rows.iter().find(|x|x["phase"]=="latency"&&x["case"]==name&&x["variant"]==v&&x["block"]==block).unwrap()["median_ms"].as_f64().unwrap();
    savings.push((ms(b)-ms(a))*1000.);ms(a)/ms(b)
   }).collect();let ratio=gm(&ratios);let wins=ratios.iter().filter(|x|**x<1.).count();
   if a=="owned" {
    for(block,v)in ratios.iter().enumerate(){if !(1./1.05..=1.05).contains(v){calibration.push(format!("{name} block{block} owned/accepted={v}"));}}
    let med=median(ratios.clone());if !(1./1.03..=1.03).contains(&med){calibration.push(format!("{name} median owned/accepted={med}"));}
   }else{
    if name=="1x2-w32"&&(ratio>=1.||wins<if screen{2}else{4}){primary.push(format!("{name} all/{b}={ratio},wins{wins}"));}
    if ratio>1.02{regressions.push(format!("{name} all/{b}={ratio}"));}
    if !screen&&case["workers"]==32&&case["shape"]!="merge-54x1"{original.get_mut(b).unwrap().push(ratio);}
   }
   pairs.insert(format!("{a}/{b}"),json!({"geometric_ratio":ratio,"ratios":ratios,"median_ratio":median(ratios),"wins":wins,"saving_us":median(savings)}));
  }
  cases.insert(name.to_string(),json!({"shape":case["shape"],"workers":case["workers"],"reference":case["reference"],"modes":modes,"pairs":pairs}));
 }
 let mut original_geos=BTreeMap::new();if !screen{for(control,values)in original{assert_eq!(values.len(),8);original_geos.insert(control,gm(&values));}}
 let pass=calibration.is_empty()&&primary.is_empty()&&regressions.is_empty()&&original_geos.values().all(|x|*x<=1.);
 let exports=if screen{24}else{284};assert_eq!(result["distinct_verified_exports"],exports);let sdk:u64=rows.iter().map(|x|x["summary"]["rounds"].as_u64().unwrap()+3).sum();assert_eq!(sdk,if screen{480}else{13896});
 if !screen{assert_eq!(result["profiles"],4);for row in rows.iter().filter(|x|x["phase"]=="profile"){assert_eq!(row["profile"]["lost_records"],0);assert_eq!(row["profile"]["sampled_pids"].as_array().unwrap().len(),1);}}
 let mut inventory:BTreeMap<String,String>=serde_json::from_value(read(r.join("preflight-proof-inventory.json"))).unwrap();
 for sub in if screen{vec!["screen"]}else{vec!["screen","performance"]}{let d=r.join(sub);for(rel,h)in read(d.join("proof-inventory.json")).as_object().unwrap(){let p=d.join(rel);assert_eq!(hash(&p),h.as_str().unwrap());assert!(inventory.insert(p.display().to_string(),h.as_str().unwrap().into()).is_none());}}
 let mut seen=prior();for(p,h)in &inventory{assert_eq!(hash(p),*h);if p.ends_with(".proof"){assert_eq!(fs::metadata(p).unwrap().len(),164);assert!(seen.insert(h.clone()));}}
 assert_eq!(seen.len(),if screen{6928}else{7212});save(r.join("proof-inventory.json"),&json!(inventory));if screen{save(r.join("screen-combined-proof-inventory.json"),&json!(inventory));}
 let a=json!({"stage":stage,"cases":cases,"calibration_failures":calibration,"primary_failures":primary,"regressions":regressions,"original8":original_geos,"screen_advances_to_full":screen&&pass,"full_gate_passed":!screen&&pass,"sdk_requests":sdk+72+if screen{0}else{480},"independent_exports":exports+18+if screen{0}else{24},"next_prior_count":seen.len(),"adopted":false,"goal_achieved":false,"candidate_retained":true});save(r.join(format!("{stage}-analysis.json")),&a);
 if screen&&pass{let mut c=read(r.join("perf-plan-screen.json"));c["prior_proof_sha256"]=json!(seen);let mut pins:BTreeMap<String,String>=serde_json::from_value(c["pins"].clone()).unwrap();for f in ["screen-analysis.json","screen/result.json","screen/freshness.json","screen-combined-proof-inventory.json"]{add(r.join(f),&mut pins);}c["pins"]=json!(pins);save(r.join("perf-plan-full.json"),&c);}
 println!("{}",serde_json::to_string_pretty(&json!({"stage":stage,"passes":pass,"calibration":a["calibration_failures"],"primary":a["primary_failures"],"regressions":a["regressions"],"next_prior":seen.len()})).unwrap());
}

fn main(){match std::env::args().nth(1).as_deref(){Some("prepare")=>prepare(),Some("qualify")=>qualify(),Some("screen")=>evaluate("screen"),Some("full")=>evaluate("performance"),_=>panic!("mode")}}
