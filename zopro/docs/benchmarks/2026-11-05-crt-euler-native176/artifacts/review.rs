//! Independent receipt/statistic review. Does not include campaign or parser code.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::{BTreeMap, BTreeSet}, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/crt-euler-native176";
fn load(p: impl AsRef<Path>) -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() }
fn sha(bytes: &[u8]) -> String { format!("{:x}", Sha256::digest(bytes)) }
fn hash(p: impl AsRef<Path>) -> String { sha(&fs::read(p).unwrap()) }
fn close(a: f64, b: f64) { assert!((a-b).abs() <= 1e-12*a.abs().max(b.abs()).max(1.), "{a} != {b}"); }
fn mid(a: &[u64]) -> u64 { let mut b=a.to_vec(); b.sort_unstable(); b[b.len()/2] }
fn rows(name: &str) -> Value {
    let text=fs::read_to_string(format!("{R}/capture/{name}/stdout.log")).unwrap();
    let lines:Vec<_>=text.lines().filter_map(|l| l.strip_prefix("RESULT ")).collect();
    assert_eq!(lines.len(),1); serde_json::from_str(lines[0]).unwrap()
}
fn dist(mut x: Vec<f64>) -> Value {
    assert_eq!(x.len(),15); let wins=x.iter().filter(|v| **v<1.).count();
    x.sort_by(f64::total_cmp); json!({"median":x[7],"low":x[3],"high":x[11],"wins":wins,"values":x})
}
fn same_dist(a: &Value,b: &Value) {
    for f in ["median","low","high"] {close(a[f].as_f64().unwrap(),b[f].as_f64().unwrap());}
    assert_eq!(a["wins"],b["wins"]);
    for (x,y) in a["values"].as_array().unwrap().iter().zip(b["values"].as_array().unwrap()) {close(x.as_f64().unwrap(),y.as_f64().unwrap());}
    assert_eq!(b["values"].as_array().unwrap().len(),15);
}
fn main() {
    assert!(!Path::new(&format!("{R}/review.json")).exists(), "review is first-pass only");
    let mut pin_union=BTreeMap::<String,String>::new(); let mut commands=0;
    for (dir,plan_file,expected) in [("runs","plan.json",6),("build","build-plan.json",2),("capture","capture-plan.json",310)] {
        let plan=load(format!("{R}/{plan_file}")); let result=load(format!("{R}/{dir}/result.json"));
        assert_eq!(result["status"],"complete"); let steps=plan["steps"].as_array().unwrap(); assert_eq!(steps.len(),expected);
        let names:Vec<_>=steps.iter().map(|s|s["name"].clone()).collect();
        assert_eq!(result["steps"],json!(names)); assert_eq!(load(format!("{R}/{dir}/completed.json")),json!(names));
        let mut normalized=plan.clone();
        normalized.as_object_mut().unwrap().entry("environment").or_insert(json!({}));
        for step in normalized["steps"].as_array_mut().unwrap() {
            step.as_object_mut().unwrap().entry("environment").or_insert(json!({}));
            step.as_object_mut().unwrap().entry("required_tests").or_insert(json!([]));
        }
        assert_eq!(load(format!("{R}/{dir}/protocol.json")),normalized);
        assert_eq!(fs::read_dir(format!("{R}/{dir}")).unwrap().filter(|e| e.as_ref().unwrap().file_name().to_string_lossy().starts_with("attempt-")).count(),1);
        assert_eq!(load(format!("{R}/{dir}/attempt-0.json"))["resume"],false);
        let pins=load(plan["source_pins"].as_str().unwrap()); assert_eq!(load(format!("{R}/{dir}/source-pins.json")),pins);
        for (p,h) in pins.as_object().unwrap() { if let Some(old)=pin_union.insert(p.clone(),h.as_str().unwrap().into()) {assert_eq!(old,h.as_str().unwrap());} }
        for step in steps {
            let path=format!("{R}/{dir}/{}",step["name"].as_str().unwrap()); let c=load(format!("{path}/command.json")); let e=load(format!("{path}/exit.json"));
            assert_eq!(e["success"],true); assert_eq!(e["returncode"],0);
            for stream in ["stdout","stderr"] {assert_eq!(hash(format!("{path}/{stream}.log")),e[format!("{stream}_sha256")].as_str().unwrap());}
            let actual=c["argv"].as_array().unwrap(); let declared=step["argv"].as_array().unwrap(); assert!(actual.ends_with(declared));
            assert_eq!(actual.len(),declared.len()+4); assert_eq!(actual[0],"timeout"); assert_eq!(actual[1],"-k"); assert_eq!(actual[2],"5s");
            assert_eq!(actual[3],format!("{}s",step["timeout_seconds"].as_u64().unwrap()));
            let env=step.get("environment").cloned().unwrap_or(json!({})); assert_eq!(c["environment"],env);
            if step["name"].as_str().unwrap().starts_with("compile-") || dir=="build" {
                for required in ["-O","target-cpu=native","lto=fat","codegen-units=1"] {assert!(actual.contains(&json!(required)));}
                for (label,prefix) in [("zopro_bn254","libzopro_bn254-1200e635"),("zopro_core","libzopro_core-9dc62928"),("rayon","librayon-bf7ab8bf")] {
                    assert!(actual.iter().any(|v|v.as_str().is_some_and(|s|s.starts_with(&format!("{label}="))&&s.contains(prefix))));
                }
            }
            commands+=1;
        }
    }
    for (p,h) in &pin_union { assert_eq!(hash(p),*h,"pin {p}"); }
    assert_eq!(commands,318);
    let parent=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/crt-euler176/src/lib.rs").unwrap();
    assert_eq!(fs::read_to_string(format!("{R}/base.rs")).unwrap(),format!("{parent}\n#[path=\"direct.rs\"]pub mod direct;\n"));
    let direct=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/crt-euler-direct176/direct-fixed.rs").unwrap();
    assert_eq!(fs::read_to_string(format!("{R}/direct.rs")).unwrap(),format!("{direct}\n#[path=\"native.rs\"]pub mod native;\n"));
    let source_manifest=load("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json");
    let source=source_manifest["source_files_sha256"].as_object().unwrap(); assert_eq!(source.len(),176);
    for (p,h) in source {assert_eq!(hash(format!("/home/ubuntu/src/zk-tools/zopro/{p}")),h.as_str().unwrap());}
    for name in ["native-checked-tests","native-release-tests"] {
        let t=fs::read_to_string(format!("{R}/runs/{name}/stdout.log")).unwrap();
        assert!(t.contains("3 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out"));
        assert_eq!(t.lines().filter(|l|l.starts_with("test direct::native::tests::")&&l.ends_with(" ... ok")).count(),3);
    }
    let q=load(format!("{R}/qualification.json")); let printed:Value=serde_json::from_str(fs::read_to_string(format!("{R}/runs/complete-functional-qualification/stdout.log")).unwrap().trim()).unwrap();
    assert_eq!(q,printed); assert_eq!(q["coordinate_comparisons"],131072); assert_eq!(q["native_output_coordinates"],65536); assert_eq!(q["worker_counts"],json!([16,32]));
    let ids=load(format!("{R}/identities.json")); for arm in ["FROZEN","OFF","ON"] {assert_eq!(hash(ids[arm]["binary"].as_str().unwrap()),ids[arm]["sha256"].as_str().unwrap());}
    assert_eq!(ids["OFF"],ids["ON"]); assert_ne!(ids["FROZEN"]["sha256"],ids["OFF"]["sha256"]);
    let jobs=load(format!("{R}/jobs.json")); let cases=load(format!("{R}/cases.json")); let analysis=load(format!("{R}/analysis.json"));
    let mut times=BTreeMap::<(String,String,usize),u64>::new(); let mut bindings=BTreeMap::new(); let mut preparations=BTreeMap::<String,Vec<u64>>::new(); let mut rss=BTreeMap::<String,Vec<u64>>::new(); let mut cpu=BTreeMap::<String,Vec<f64>>::new();
    let mut engine_calls=0; let mut measured=0; let mut panel_counts=BTreeMap::<String,usize>::new();
    for j in jobs.as_array().unwrap() {
        let name=j["name"].as_str().unwrap(); let v=rows(name); let panel=j["panel"].as_str().unwrap(); let case=j["case"]["name"].as_str().unwrap(); let arm=j["arm"].as_str().unwrap(); let rounds=j["rounds"].as_u64().unwrap();
        *panel_counts.entry(panel.into()).or_default()+=1;
        for f in ["domain","workers"] {assert_eq!(v[f],j["case"][f]);} assert_eq!(v["mode"],arm); assert_eq!(v["rounds"],rounds); assert_eq!(v["inputs_unchanged"],true); assert_eq!(v["primes"],6); assert_eq!(v["new_proofs"],0);
        let workers=j["case"]["workers"].as_u64().unwrap(); let allowed=if workers==1 {vec![2]} else {(0..workers).collect::<Vec<_>>()}; assert_eq!(v["allowed_cpus"],json!(allowed));
        assert_eq!(v["profile"],panel=="stat"||panel=="cycles");
        let checked=if panel=="stat"||panel=="cycles"{2}else{rounds}; assert_eq!(v["outputs_checked"],checked+6);
        let binding=json!([v["input_hashes"],v["output_hashes"]]); if let Some(old)=bindings.insert(case.to_string(),binding.clone()) {assert_eq!(old,binding);}
        let observation=analysis["observations"].as_array().unwrap().iter().find(|o|o["job"]["name"]==name).unwrap(); assert_eq!(observation["result"],v); assert_eq!(observation["job"],*j);
        engine_calls+=rounds+6;
        if panel=="latency" {
            let raw=v["times_ns"].as_array().unwrap().iter().map(|x|x.as_u64().unwrap()).collect::<Vec<_>>(); assert_eq!(raw.len(),rounds as usize); assert!(raw.iter().all(|t|*t>0)); let m=mid(&raw); assert_eq!(v["median_ns"],m);
            let block=j["block"].as_u64().unwrap() as usize; assert!(times.insert((case.into(),arm.into(),block),m).is_none()); measured+=rounds;
            let key=format!("{case}/{arm}"); preparations.entry(key.clone()).or_default().push(v["preparation_ns"].as_u64().unwrap()); rss.entry(key.clone()).or_default().push(v["usage"]["whole_process_maxrss_kib"].as_u64().unwrap());
            cpu.entry(key).or_default().push((v["usage"]["user_cpu_ms"].as_f64().unwrap()+v["usage"]["system_cpu_ms"].as_f64().unwrap())/rounds as f64);
        }
    }
    assert_eq!(jobs.as_array().unwrap().len(),294); assert_eq!(analysis["observations"].as_array().unwrap().len(),294); assert_eq!(times.len(),270); assert_eq!(measured,61020); assert_eq!(engine_calls,75582);
    assert_eq!(panel_counts,BTreeMap::from([("latency".into(),270),("preflight".into(),18),("stat".into(),3),("cycles".into(),3)]));
    let mut failures=BTreeSet::new(); let mut comparisons=BTreeMap::new(); let mut memory=BTreeMap::new();
    for c in cases.as_array().unwrap() {
        let name=c["name"].as_str().unwrap(); let get=|arm:&str,b:usize|times[&(name.into(),arm.into(),b)];
        for arm in ["FROZEN","OFF","ON"] {
            let key=format!("{name}/{arm}"); let values:Vec<_>=(0..15).map(|b|get(arm,b)).collect(); close(mid(&values) as f64/1e6,analysis["medians_ms"][&key].as_f64().unwrap());
            assert_eq!(preparations[&key].len(),15); assert_eq!(rss[&key].len(),15); let mut cp=cpu[&key].clone();cp.sort_by(f64::total_cmp);
            memory.insert(key.clone(),json!({"blocks":15,"preparation_median_ms":mid(&preparations[&key])as f64/1e6,"preparation_max_ms":preparations[&key].iter().max().unwrap().to_owned()as f64/1e6,"process_rss_max_mib":rss[&key].iter().max().unwrap().to_owned()as f64/1024.,"rusage_cpu_median_ms_per_call_including_outside_clock_checks":cp[7]}));
        }
        for (numer,denom) in [("OFF","FROZEN"),("ON","FROZEN"),("ON","OFF")] {
            let key=format!("{name}/{numer}/{denom}"); let d=dist((0..15).map(|b|get(numer,b)as f64/get(denom,b)as f64).collect()); same_dist(&d,&analysis["comparisons"][&key]["paired"]);
            let low=d["low"].as_f64().unwrap(); let high=d["high"].as_f64().unwrap(); let good=high<=1.03&&(numer!="OFF"||low>=1./1.03);
            assert_eq!(analysis["comparisons"][&key]["nonregression_or_calibration_passed"],good); if !good{failures.insert(key.clone());}
            if name=="n32768-w32"&&numer=="ON"&&!(high<1.&&d["wins"].as_u64().unwrap()>=10){failures.insert(format!("{key}/gain"));}
            comparisons.insert(key,d);
        }
    }
    for den in ["FROZEN","OFF"] {
        let key=format!("n32768-w16-32/ON/{den}"); let d=dist((0..15).map(|b|{
            let a=times[&("n32768-w16".into(),"ON".into(),b)]as f64/times[&("n32768-w16".into(),den.into(),b)]as f64;
            let c=times[&("n32768-w32".into(),"ON".into(),b)]as f64/times[&("n32768-w32".into(),den.into(),b)]as f64;(a*c).sqrt()
        }).collect());same_dist(&d,&analysis["comparisons"][&key]["paired"]);
        let good=d["high"].as_f64().unwrap()<1.&&d["wins"].as_u64().unwrap()>=10;assert_eq!(analysis["comparisons"][&key]["gain_passed"],good);if !good{failures.insert(format!("{key}/gain"));}comparisons.insert(key,d);
    }
    let declared:BTreeSet<_>=analysis["failed_checks"].as_array().unwrap().iter().map(|x|x.as_str().unwrap().to_owned()).collect();assert_eq!(declared,failures);assert_eq!(failures.len(),16);assert_eq!(analysis["gate_passed"],false);
    let mut counters=Vec::new(); let mut profiles=Vec::new();
    for j in jobs.as_array().unwrap().iter().filter(|j|j["panel"]=="stat") {
        let name=j["name"].as_str().unwrap();let text=fs::read_to_string(j["data"].as_str().unwrap()).unwrap();let reference=analysis["counters"].as_array().unwrap().iter().find(|c|c["job"]==name).unwrap();let mut events=BTreeMap::new();
        for l in text.lines().filter(|l|!l.trim().is_empty()&&!l.trim_start().starts_with('#')) {let f:Vec<_>=l.split(';').collect(); assert!(f.len()>=5); let count:f64=f[0].trim().parse().unwrap();let runtime:f64=f[3].trim().parse().unwrap();let coverage:f64=f[4].trim().parse().unwrap();assert!(runtime>0.&&coverage>=99.9);close(count,reference["events"][f[2]]["count"].as_f64().unwrap()); events.insert(f[2].to_string(),json!({"count":count,"per_operator":count/151.,"running_percent":coverage,"runtime_ns":runtime}));}assert_eq!(events.len(),4);assert_eq!(reference["qualified"],true);counters.push(json!({"job":name,"events":events}));
    }
    for j in jobs.as_array().unwrap().iter().filter(|j|j["panel"]=="cycles") {
        let name=j["name"].as_str().unwrap();let pid=rows(name)["pid"].as_u64().unwrap();let meta=fs::read_to_string(format!("{R}/capture/metadata-{name}/stdout.log")).unwrap();let binary=j["binary"].as_str().unwrap();let mut owners=BTreeSet::new();let mut tids=BTreeSet::from([pid]);
        for l in meta.lines() {
            if l.contains("PERF_RECORD_MMAP")&&l.ends_with(binary) {owners.insert(l.split_whitespace().next().unwrap().split('/').next().unwrap().parse::<u64>().unwrap());}
            if let Some(pos)=l.find("PERF_RECORD_FORK(") {let value=&l[pos+17..];let end=value.find(')').unwrap();let pieces:Vec<_>=value[..end].split(':').collect();assert_eq!(pieces.len(),2);if pieces[0].parse::<u64>().unwrap()==pid{tids.insert(pieces[1].parse().unwrap());}}
        }
        assert_eq!(owners,BTreeSet::from([pid]));assert_eq!(tids.len(),33);
        let text=fs::read_to_string(format!("{R}/capture/decode-{name}/stdout.log")).unwrap();let mut samples=0u64;let mut unknown=0u64;let mut period_sum=0u64;
        for line in text.lines().filter(|l|!l.trim().is_empty()&&!l.trim_start().starts_with('#')) {
            let f:Vec<_>=line.split_whitespace().collect();assert!(f.len()>=6);let pt:Vec<_>=f[0].split('/').collect();assert_eq!(pt[0].parse::<u64>().unwrap(),pid);assert!(tids.contains(&pt[1].parse::<u64>().unwrap()));let cpu:u32=f[1].trim_matches(['[',']']).parse().unwrap();assert!(cpu<32);let time:f64=f[2].trim_end_matches(':').parse().unwrap();assert!(time.is_finite()&&time>0.);let period:u64=f[3].parse().unwrap();assert_eq!(period,16000000);u64::from_str_radix(f[4].trim_start_matches("0x"),16).unwrap();samples+=1;period_sum+=period;if f[5]=="[unknown]"{unknown+=period;}
        }
        let loss=load(format!("{R}/capture/loss-{name}/stdout.log"));assert_eq!(loss["records"],json!([]));assert_eq!(loss["zero_lost"],true);
        let event=fs::read_to_string(format!("{R}/capture/events-{name}/stdout.log")).unwrap();let cycle=event.lines().find(|l|l.starts_with("cycles:u:")).unwrap();assert!(cycle.contains("sample_freq }: 16000000")&&cycle.contains("exclude_kernel: 1"));
        let reference=analysis["profiles"].as_array().unwrap().iter().find(|p|p["job"]==name).unwrap();assert_eq!(reference["samples"],samples);assert_eq!(reference["total_period"],period_sum);assert_eq!(reference["unknown_period"],unknown);assert_eq!(reference["registered_tids"],json!(tids));let fraction=unknown as f64/period_sum as f64;close(fraction,reference["unknown_fraction"].as_f64().unwrap());let qualified=samples>=1000&&fraction<=0.05;assert_eq!(reference["qualified"],qualified);
        assert_eq!(reference["categories"].as_object().unwrap().values().map(|v|v.as_u64().unwrap()).sum::<u64>(),period_sum);
        profiles.push(json!({"job":name,"samples":samples,"unknown_fraction":fraction,"period_sum":period_sum,"owned_pid":pid,"registered_tids":tids.len(),"zero_lost":true,"qualified":qualified}));
    }
    assert_eq!(profiles.iter().filter(|p|p["qualified"]==true).count(),2);assert_eq!(profiles[2]["qualified"],false);
    let ledger="/home/ubuntu/src/zopro-perf-work/finalize-drain-bundle176/proof-sha256-after.json";let proofs=load(ledger);assert_eq!(proofs.as_array().unwrap().len(),29953);let unique:BTreeSet<_>=proofs.as_array().unwrap().iter().map(|p|p.as_str().unwrap()).collect();assert_eq!(unique.len(),29953);
    let dot=fs::read_to_string(format!("{R}/crt-dot.s")).unwrap();let static_ifma=dot.lines().filter(|l|l.contains("vpmadd52")).count();assert_eq!(static_ifma,28);
    let out=json!({"review_passed":true,"independent_of_campaign_code":true,"source_pin_checks":pin_union.len(),"successful_workflow_commands":commands,"no_attempt_resumes":true,"production_files_unchanged":176,"new_native_tests_per_mode":3,"inherited_parent_tests_filtered":6,"unique_projection_cases_per_mode":74120,"bounded_CRT_outputs_per_mode":4104,"arbitrary_residue_CRT_outputs_per_mode":8192,"small_native_operator_calls_per_mode":24,"full_native_output_coordinates":65536,"full_original_output_coordinates":65536,"caller_processes":294,"latency_calls":measured,"engine_calls_including_primes_diagnostics":engine_calls,"additional_reference_calls":588,"gate_passed":false,"failed_checks":failures,"paired_results":comparisons,"memory_and_setup":memory,"counters":counters,"profiles":profiles,"linked_static_IFMA_sites":static_ifma,"logical_IFMA_per_eight_coefficients":201,"new_proofs":0,"proof_ledger_count":29953,"proof_ledger_sha256":hash(ledger),"adopted":false,"goal_achieved":false});
    fs::write(format!("{R}/review.json"),serde_json::to_vec_pretty(&out).unwrap()).unwrap();
    println!("Independent review PASS: {} pins, {commands} commands; 270 latency rows; 16 failed candidate gates retained; ON attribution failure retained; zero new proofs",pin_union.len());
}
