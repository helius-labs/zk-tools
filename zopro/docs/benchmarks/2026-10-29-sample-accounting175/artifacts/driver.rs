use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::{BTreeMap, BTreeSet}, fs, io::{BufRead, BufReader, Read, Write}, path::{Path, PathBuf}, process::{Command, Stdio}};
const R: &str = "/home/ubuntu/src/zopro-perf-work/sample-accounting175";
const P: &str = "/home/ubuntu/src/zopro-perf-work/instruction-profile175";
const SDK: &str = "/home/ubuntu/src/zopro-perf-work/balanced-kh-production/native/candidate-sdk";
const VERIFY: &str = "/home/ubuntu/src/zopro-perf-work/batch-local-steal/gnark/verify";
fn r(p: impl AsRef<Path>) -> PathBuf { Path::new(R).join(p) }
fn read(p: impl AsRef<Path>) -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() }
fn save(p: impl AsRef<Path>, v: &Value) { fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap() }
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap(); let mut h = Sha256::new(); let mut b = [0;65536];
    loop { let n=f.read(&mut b).unwrap(); if n==0 { break; } h.update(&b[..n]); }
    format!("{:x}",h.finalize())
}
fn pin(pins: &mut BTreeMap<String,String>, p: impl AsRef<Path>) { pins.insert(p.as_ref().display().to_string(), hash(p)); }
fn command(name: String, argv: Vec<String>) -> Value { json!({"kind":"command", "name":name, "argv":argv, "timeout_seconds":900}) }
fn checked_line(reader: &mut impl BufRead, expected: &str) {
    let mut line=String::new(); assert!(reader.read_line(&mut line).unwrap()>0);
    assert_eq!(line.trim(), expected);
}
fn relay<T: Read+Write>(mut input: impl BufRead, mut output: impl Write, mut controls: Vec<(T,BufReader<T>)>) {
    let mut receipts=Vec::new();
    for expected in ["enable", "disable"] {
        checked_line(&mut input, expected);
        for (index,(ctl,ack)) in controls.iter_mut().enumerate() {
            writeln!(ctl,"{expected}").unwrap(); ctl.flush().unwrap(); checked_line(ack,"ack");
            receipts.push(json!({"command":expected,"counter":index,"ack":true}));
        }
        writeln!(output,"ack").unwrap(); output.flush().unwrap();
    }
    println!("CONTROL {}",json!({"both_acks_before_SDK":true,"order":receipts}));
}
fn fifo(name: &str) -> fs::File { fs::OpenOptions::new().read(true).write(true).open(r(name)).unwrap() }
fn run(name: &str) {
    let input=BufReader::new(fifo("sdkctl")); let output=fifo("sdkack");
    let controls=vec![(fifo("recordctl"),BufReader::new(fifo("recordack"))), (fifo("statctl"),BufReader::new(fifo("statack")))];
    let thread=std::thread::spawn(move||relay(input,output,controls));
    let jobs=read(r("jobs.json")); let job=jobs.as_array().unwrap().iter().find(|j|j["name"]==name).unwrap();
    let a=job["argv"].as_array().unwrap().iter().map(|v|v.as_str().unwrap()).collect::<Vec<_>>();
    let status=Command::new(a[0]).args(&a[1..]).status().unwrap(); assert!(status.success(),"capture {name}: {status}");
    thread.join().unwrap();
}
fn prepare() {
    assert!(!r("plan.json").exists()); let mut pins=BTreeMap::new();
    for(p,h) in read(Path::new(P).join("pins.json")).as_object().unwrap() { assert_eq!(hash(p),h.as_str().unwrap()); pins.insert(p.clone(),h.as_str().unwrap().into()); }
    assert_eq!(hash(SDK),"20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f");
    let c=read(Path::new(P).join("case.json")); save(r("case.json"),&c);
    let prior=read(Path::new(P).join("proof-sha256-after.json")); assert_eq!(prior.as_array().unwrap().len(),22210);
    save(r("prior-proof-sha256.json"),&prior); pin(&mut pins,Path::new(P).join("proof-sha256-after.json"));
    fs::copy(Path::new(P).join("inherited-preflights.json"),r("inherited-preflights.json")).unwrap();
    let state=Command::new("sudo").args(["-n","cat","/sys/kernel/tracing/uprobe_events"]).output().unwrap();
    assert!(state.status.success()); assert!(!String::from_utf8_lossy(&state.stdout).contains(SDK)); save(r("no-sdk-probes.json"),&json!({"none":true,"raw":String::from_utf8_lossy(&state.stdout)}));
    for d in ["perf","stat","proofs"] { fs::create_dir_all(r(d)).unwrap(); }
    for f in ["sdkctl","sdkack","recordctl","recordack","statctl","statack"] { assert!(Command::new("mkfifo").arg(r(f)).status().unwrap().success()); }
    let manifest=read("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json");
    let mut jobs=Vec::new(); let mut steps=Vec::new();
    for block in 0..3 {
        let mut order=Vec::new(); for w in [16,32] { for event in ["cycles","instructions"] { for mode in ["freq","fixed"] { order.push((w,event,mode)); } } }
        order.rotate_left(block); if block%2==1 {order.reverse();}
        for(w,event,mode) in order {
            let name=format!("b{block}-w{w}-{event}-{mode}"); let cpu=format!("0-{}",w-1);
            let mut env=manifest["environment"].clone(); env["RAYON_NUM_THREADS"]=json!(w.to_string()); env["ZOPRO_CPU_DEDUP"]=json!("1");
            env["ZOPRO_BENCH_PERF_CTL"]=json!(r("sdkctl")); env["ZOPRO_BENCH_PERF_ACK"]=json!(r("sdkack"));
            let mut a=vec!["sudo".to_string(),"-n".into(),"env".into()];
            for(k,v) in env.as_object().unwrap() {a.push(format!("{k}={}",v.as_str().unwrap()));}
            a.extend(["perf".into(),"record".into(),"--no-buildid-cache".into(),"--sample-cpu".into(),"--timestamp".into(),"-e".into(),format!("{event}:u"),if mode=="freq" {"-F".into()}else{"-c".into()},if mode=="freq" {"199".into()}else{"16000000".into()},"-m".into(),"512".into(),"-o".into(),r(format!("perf/{name}.data")).display().to_string(),"--delay=-1".into(),format!("--control=fifo:{},{}",r("recordctl").display(),r("recordack").display()),"--".into(),"perf".into(),"stat".into(),"--no-big-num".into(),"-x".into(),";".into(),"-e".into(),format!("{event}:u"),"-o".into(),r(format!("stat/{name}.csv")).display().to_string(),"--delay=-1".into(),format!("--control=fifo:{},{}",r("statctl").display(),r("statack").display()),"--".into(),"taskset".into(),"-c".into(),cpu.clone(),SDK.into(),c["image"].as_str().unwrap().into(),c["request"].as_str().unwrap().into(),"151".into(),r(format!("proofs/{name}")).display().to_string()]);
            // Store the period explicitly in fixed-count records too.
            let record_index=a.iter().position(|s|s=="record").unwrap(); a.insert(record_index+1,"-P".into());
            jobs.push(json!({"name":name,"block":block,"workers":w,"event":event,"mode":mode,"cpu_affinity":cpu,"argv":a}));
            steps.push(command(name.clone(),vec![r("driver").display().to_string(),"run".into(),name]));
        }
    }
    assert_eq!(steps.len(),24); let mut offline=Vec::new();
    for j in &jobs { let n=j["name"].as_str().unwrap(); offline.push(command(format!("own-{n}"),vec!["sudo".into(),"-n".into(),"chown".into(),"ubuntu:ubuntu".into(),r(format!("perf/{n}.data")).display().to_string()])); }
    for j in &jobs { let n=j["name"].as_str().unwrap(); offline.push(command(format!("verify-{n}"),vec![VERIFY.into(),c["image"].as_str().unwrap().into(),c["image_sha256"].as_str().unwrap().into(),r(format!("proofs/{n}/0.proof")).display().to_string(),r(format!("proofs/{n}/0.public")).display().to_string(),c["public_sha256"].as_str().unwrap().into()])); }
    for j in &jobs {
        let n=j["name"].as_str().unwrap(); let data=r(format!("perf/{n}.data")).display().to_string();
        for(kind,a) in [
            ("decode",vec!["perf".into(),"script".into(),"--header".into(),"--ns".into(),"-i".into(),data.clone(),"-F".into(),"pid,tid,cpu,time,period,ip,sym".into()]),
            ("events",vec!["perf".into(),"evlist".into(),"-v".into(),"-i".into(),data.clone()]),
            ("loss",vec![r("driver").display().to_string(),"loss".into(),data.clone()]),
            ("metadata",vec!["perf".into(),"script".into(),"--header".into(),"--show-mmap-events".into(),"--show-task-events".into(),"--ns".into(),"-i".into(),data,"-F".into(),"pid,tid,cpu,time,period,ip,sym".into()])
        ] {offline.push(command(format!("{kind}-{n}"),a));}
    }
    assert_eq!(offline.len(),144); save(r("jobs.json"),&json!(jobs));
    for f in ["driver.rs","driver","sampling.rs","counter.rs","analysis.rs","publish.rs","publish","publication.rs","design.md","case.json","jobs.json","prior-proof-sha256.json","inherited-preflights.json","no-sdk-probes.json","unit-tests.log"] { pin(&mut pins,r(f)); }
    pin(&mut pins,"/usr/bin/perf"); save(r("pins.json"),&json!(pins));
    save(r("plan.json"),&json!({"version":1,"source_pins":r("pins.json"),"steps":steps}));
    save(r("offline-plan.json"),&json!({"version":1,"source_pins":r("pins.json"),"steps":offline}));
    println!("Prepared24 simultaneous stat/sample captures +144offline; no source rebuild");
}
fn loss(path:&str) {
    let mut child=Command::new("perf").args(["script","-D","-i",path]).stdout(Stdio::piped()).spawn().unwrap();
    let mut bad=Vec::new(); let mut lines=0;
    for line in BufReader::new(child.stdout.take().unwrap()).lines() {let line=line.unwrap(); lines+=1; if ["PERF_RECORD_LOST","PERF_RECORD_THROTTLE","PERF_RECORD_UNTHROTTLE"].iter().any(|needle|line.contains(needle)){bad.push(line);}}
    assert!(child.wait().unwrap().success()); println!("{}",json!({"lines":lines,"zero_lost_or_throttle":bad.is_empty(),"records":bad}));
}
include!("sampling.rs");
include!("counter.rs");
include!("analysis.rs");
fn main(){let a=std::env::args().collect::<Vec<_>>();match a.get(1).map(String::as_str){Some("prepare")=>prepare(),Some("run")=>run(&a[2]),Some("loss")=>loss(&a[2]),Some("finish")=>finish(),_=>panic!("mode")}}
#[cfg(test)] mod relay_tests {
    use super::*;
    #[test] fn acknowledgments_checked() { checked_line(&mut std::io::Cursor::new("ack\n"),"ack"); }
    #[test] #[should_panic] fn malformed_ack_rejected() { checked_line(&mut std::io::Cursor::new("no\n"),"ack"); }
    #[test] fn commands_roundtrip() {
        use std::os::unix::net::UnixStream;
        let(mut a,mut b)=UnixStream::pair().unwrap();
        std::thread::scope(|scope| {
            scope.spawn(move||{let mut r=BufReader::new(a.try_clone().unwrap()); for cmd in ["enable","disable"] { checked_line(&mut r,cmd); writeln!(a,"ack").unwrap(); }});
            let mut r=BufReader::new(b.try_clone().unwrap()); for cmd in ["enable","disable"] {writeln!(b,"{cmd}").unwrap();checked_line(&mut r,"ack");}
        });
    }
    #[test] fn sdk_ack_waits_for_both_tools() {
        use std::{os::unix::net::UnixStream, sync::{Arc,atomic::{AtomicUsize,Ordering}}};
        struct Audit { completed:Arc<AtomicUsize>, responses:usize }
        impl Write for Audit {
            fn write(&mut self,b:&[u8])->std::io::Result<usize>{
                if b.contains(&b'\n') {self.responses+=1;assert_eq!(self.completed.load(Ordering::SeqCst),self.responses*2);}
                Ok(b.len())
            }
            fn flush(&mut self)->std::io::Result<()> {Ok(())}
        }
        let completed=Arc::new(AtomicUsize::new(0));let mut controls=Vec::new();let mut handles=Vec::new();
        for _ in 0..2 {
            let(client,mut server)=UnixStream::pair().unwrap(); controls.push((client.try_clone().unwrap(),BufReader::new(client)));
            let count=completed.clone();handles.push(std::thread::spawn(move||{let mut reader=BufReader::new(server.try_clone().unwrap());for cmd in ["enable","disable"]{checked_line(&mut reader,cmd);count.fetch_add(1,Ordering::SeqCst);writeln!(server,"ack").unwrap();}}));
        }
        let mut audit=Audit {completed,responses:0};relay(std::io::Cursor::new("enable\ndisable\n"),&mut audit,controls);assert_eq!(audit.responses,2);
        for handle in handles {handle.join().unwrap();}
    }
}
