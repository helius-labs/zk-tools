use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path, process::Command};
const R: &str = "/tmp/zopro-g1-reverse-overlap";
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
        111
    );
    let artifact = read(r.join("native/probe/artifacts.json"));
    let binary = artifact["binary"].as_str().unwrap();
    assert_eq!(hash(binary), artifact["binary_sha256"]);
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r.join("source-pins.json"))).unwrap();
    for (p, h) in artifact["input_pins"].as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
        add(p, &mut pins);
    }
    add(binary, &mut pins);
    let nm = Command::new("nm")
        .args(["-C", "-S", "--defined-only", binary])
        .output()
        .unwrap();
    assert!(nm.status.success());
    let nm = String::from_utf8(nm.stdout).unwrap();
    let selected = nm
        .lines()
        .filter(|s| s.contains("stream_g1") || s.contains("G1StreamProbe::run"))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(!selected.is_empty());
    fs::write(r.join("codegen-symbols.txt"), selected).unwrap();
    for name in [
        "prepare.rs",
        "prepare",
        "analyze.rs",
        "analyze",
        "close.rs",
        "close",
        "adapt.rs",
        "native-plan.json",
        "native/result.json",
        "native/probe/artifacts.json",
        "source-audit.json",
        "codegen-symbols.txt",
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
        for signed in [0, 1] {
            for pairs in [8, 9, 16, 17, 24, 25, 200, 203, 352, 355] {
                let name = format!("g1-b{pairs}-signed{signed}-s{seed}");
                let case = json!({"name":name,"curve":"g1","pairs":pairs,"signed":signed==1,"seed":seed,"iterations":1024});
                steps.push(json!({"kind":"command","name":name,"argv":["taskset","-c","2",binary,pairs.to_string(),signed.to_string(),seed.to_string(),"paired"],"timeout_seconds":600}));
                jobs.push(json!({"phase":"latency","name":name,"case":case}));
                cases.push(case);
            }
        }
    }
    assert_eq!(cases.len(), 40);
    for pairs in [200, 352] {
        for mode in ["control-a", "overlap"] {
            let name = format!("profile-g1-b{pairs}-{mode}");
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
            steps.push(json!({"kind":"command","name":name,"argv":["sudo","-n","/usr/bin/perf","record","--no-buildid-cache","--clockid","mono","--delay=-1",control,"-e","cycles:u","-F","999","-o",dir.join("perf.data"),"--","sudo","-n","-u","ubuntu","env",format!("ZOPRO_BENCH_PERF_CTL={}",dir.join("ctl").display()),format!("ZOPRO_BENCH_PERF_ACK={}",dir.join("ack").display()),"taskset","-c","2",binary,pairs.to_string(),"1","1",mode],"timeout_seconds":600}));
            jobs.push(json!({"phase":"profile","name":name,"pairs":pairs,"signed":true,"seed":1,"mode":mode}));
        }
    }
    for pairs in [200, 352] {
        for mode in ["control-a", "overlap"] {
            let name = format!("profile-g1-b{pairs}-{mode}");
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
    println!("Frozen40G1streamcases840stages then4profiles;111corepassed;0SDK/Gnark;prior8310.");
}
