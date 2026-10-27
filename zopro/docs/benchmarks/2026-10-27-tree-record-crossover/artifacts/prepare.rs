use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path, process::Command};
const R: &str = "/tmp/zopro-tree-record-crossover";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn add(p: impl AsRef<Path>, pins: &mut BTreeMap<String, String>) {
    let p = p.as_ref();
    pins.insert(p.display().to_string(), hash(p));
}
fn main() {
    let r = Path::new(R);
    assert_eq!(read(r.join("native/result.json"))["status"], "complete");
    assert_eq!(
        read(r.join("native/core/tests.json"))["passed_tests"]
            .as_array()
            .unwrap()
            .len(),
        112
    );
    let artifact = read(r.join("native/probe/artifacts.json"));
    let binary = artifact["binary"].as_str().unwrap();
    assert_eq!(hash(binary), artifact["binary_sha256"].as_str().unwrap());
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r.join("source-pins.json"))).unwrap();
    for (p, h) in artifact["input_pins"].as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
        add(p, &mut pins);
    }
    add(binary, &mut pins);
    for name in [
        "prepare.rs",
        "prepare",
        "analyze.rs",
        "analyze",
        "close.rs",
        "close",
        "native-plan.json",
        "native/result.json",
        "native/probe/artifacts.json",
        "source-audit.json",
    ] {
        add(r.join(name), &mut pins);
    }
    add("/usr/bin/perf", &mut pins);
    add(
        "/tmp/zopro-xtask-fast-screen-target/release/xtask",
        &mut pins,
    );
    let mut cases = Vec::new();
    let mut jobs = Vec::new();
    let mut steps = Vec::new();
    for seed in [0, 1] {
        for pairs in [1usize, 4, 7, 8, 9, 15, 16, 17, 32, 64, 128, 256, 512, 1024] {
            for mixed in [0, 1] {
                let name = format!("p{pairs}-m{mixed}-s{seed}");
                let case = json!({"name":name,"requested_pairs":pairs,"mixed":mixed==1,"seed":seed,"split":if seed==0{"training"}else{"heldout"},"iterations":(16384/pairs).max(64)});
                steps.push(json!({"kind":"command","name":name,"argv":["taskset","-c","2",binary,pairs.to_string(),mixed.to_string(),seed.to_string(),"paired"],"timeout_seconds":600}));
                jobs.push(json!({"phase":"latency","name":name,"case":case}));
                cases.push(case);
            }
        }
    }
    assert_eq!(cases.len(), 56);
    for pairs in [16, 1024] {
        for mode in ["control-a", "prefix"] {
            let name = format!("profile-p{pairs}-{mode}");
            let dir = r.join("profiles").join(&name);
            fs::create_dir_all(&dir).unwrap();
            for pipe in ["ctl", "ack"] {
                assert!(Command::new("mkfifo")
                    .arg(dir.join(pipe))
                    .status()
                    .unwrap()
                    .success());
            }
            let control = format!(
                "--control=fifo:{},{}",
                dir.join("ctl").display(),
                dir.join("ack").display()
            );
            steps.push(json!({"kind":"command","name":name,"argv":["sudo","-n","/usr/bin/perf","record","--no-buildid-cache","--clockid","mono","--delay=-1",control,"-e","cycles:u","-F","999","-o",dir.join("perf.data"),"--","sudo","-n","-u","ubuntu","env",format!("ZOPRO_BENCH_PERF_CTL={}",dir.join("ctl").display()),format!("ZOPRO_BENCH_PERF_ACK={}",dir.join("ack").display()),"taskset","-c","2",binary,pairs.to_string(),"0","1",mode],"timeout_seconds":600}));
            jobs.push(json!({"phase":"profile","name":name,"requested_pairs":pairs,"mixed":false,"seed":1,"mode":mode}));
        }
    }
    for pairs in [16, 1024] {
        for mode in ["control-a", "prefix"] {
            let name = format!("profile-p{pairs}-{mode}");
            let data = r.join("profiles").join(&name).join("perf.data");
            steps.push(json!({"kind":"command","name":format!("samples-{name}"),"argv":["sudo","-n","/usr/bin/perf","script","--show-lost-events","-i",data,"-F","comm,pid,tid,time,event,ip,sym,period"],"timeout_seconds":180}));
            steps.push(json!({"kind":"command","name":format!("report-{name}"),"argv":["sudo","-n","/usr/bin/perf","report","--stdio","--header","--percent-limit","0","-i",data],"timeout_seconds":180}));
        }
    }
    save(r.join("cases.json"), &json!(cases));
    save(r.join("jobs.json"), &json!(jobs));
    add(r.join("cases.json"), &mut pins);
    add(r.join("jobs.json"), &mut pins);
    for (p, h) in &pins {
        assert_eq!(&hash(p), h);
    }
    save(r.join("measurement-pins.json"), &json!(pins));
    save(
        r.join("measurement-plan.json"),
        &json!({"version":1,"source_pins":r.join("measurement-pins.json"),"environment":{"RAYON_NUM_THREADS":"1","MALLOC_ARENA_MAX":"32","MALLOC_MMAP_THRESHOLD_":"1073741824","MALLOC_TRIM_THRESHOLD_":"-1"},"steps":steps}),
    );
    println!("Frozen56pairedprocesses1176stages then4profiles;allhelperspinned;0SDK/Gnark;prior7560unchanged");
}
