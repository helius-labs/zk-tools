use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{BufRead, BufReader},
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-queue-index-composition176";
const OLD: &str = "/home/ubuntu/src/zopro-perf-work/query-wrapper176";
const ARMS: [&str; 4] = ["ORIGINAL", "DRAIN", "PACK", "BOTH"];
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn walk(p: &Path, pins: &mut BTreeMap<String, String>) {
    for e in fs::read_dir(p).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            walk(&p, pins)
        } else if p.is_file() {
            pins.insert(p.display().to_string(), hash(p));
        }
    }
}
fn command(name: &str, argv: Vec<String>) -> Value {
    json!({"kind":"command","name":name,"argv":argv,"timeout_seconds":900})
}
fn caller(j: &Value) -> Value {
    let base = j["reused_from"]
        .as_str()
        .map(PathBuf::from)
        .unwrap_or_else(|| r("runs").join(j["name"].as_str().unwrap()));
    let exit = read(base.join("exit.json"));
    assert_eq!(exit["success"], true);
    assert_eq!(exit["returncode"], 0);
    for k in ["stdout", "stderr"] {
        assert_eq!(
            hash(base.join(format!("{k}.log"))),
            exit[format!("{k}_sha256")]
        );
    }
    let text = fs::read_to_string(base.join("stdout.log")).unwrap();
    let lines = text
        .lines()
        .filter_map(|l| l.strip_prefix("CALLER "))
        .collect::<Vec<_>>();
    assert_eq!(lines.len(), 1);
    let v: Value = serde_json::from_str(lines[0]).unwrap();
    assert_eq!(v["query"], j["actual_query"]);
    assert_eq!(v["composition"]["pack_enabled"], ["PACK","BOTH"].contains(&j["variant"].as_str().unwrap()));
    assert_eq!(v["composition"]["drain_enabled"], ["DRAIN","BOTH"].contains(&j["variant"].as_str().unwrap()));
    assert_eq!(v["workers"], j["case"]["workers"]);
    assert_eq!(v["rounds"], j["rounds"]);
    for k in [
        "all_timed_outputs_checked",
        "every_output_matches_independent_full_MSM",
        "point_inputs_unchanged",
        "normalization_inside_timer",
    ] {
        assert_eq!(v[k], true);
    }
    assert_eq!(v["B_word_reset_inside_timer"], j["case"]["query"] == "B");
    assert_eq!(
        v["metadata"]["route"],
        if ["PACK","BOTH"].contains(&j["variant"].as_str().unwrap()) && j["case"]["query"] == "B" {
            336
        } else {
            0
        }
    );
    assert_eq!(
        v["samples_ns"].as_array().unwrap().len(),
        j["rounds"].as_u64().unwrap() as usize
    );
    v
}
fn add(
    case: &Value,
    arm: &str,
    block: usize,
    panel: &str,
    env: &Value,
    ids: &Value,
    jobs: &mut Vec<Value>,
    steps: &mut Vec<Value>,
    reuse: Option<&Value>,
) {
    let name = if panel == "latency" {
        format!(
            "latency-b{block:02}-{}-{arm}",
            case["name"].as_str().unwrap()
        )
    } else {
        format!("{panel}-{}-{arm}", case["name"].as_str().unwrap())
    };
    let rounds = match panel {
        "preflight" => 1,
        "latency" => if case["query"] == "A" { 47 } else { 17 },
        "stat" => 200,
        "cycles" => 1024,
        _ => panic!(),
    };
    let w = case["workers"].as_u64().unwrap();
    let mut env = env.clone();
    env["RAYON_NUM_THREADS"] = json!(w.to_string());
    env["ZOPRO_CPU_QUERY_DRAIN"] = json!(if ["DRAIN","BOTH"].contains(&arm){"1"}else{"0"});
    env["ZOPRO_CPU_QUERY_PACK"] = json!(if ["PACK","BOTH"].contains(&arm){"1"}else{"0"});
    let query = if ["PACK","BOTH"].contains(&arm) && case["query"] == "B" {
        "M"
    } else {
        case["query"].as_str().unwrap()
    };
    let mut argv = vec![];
    let mut data = Value::Null;
    if panel == "stat" || panel == "cycles" {
        env["ZOPRO_BENCH_PERF_CTL"] = json!(r("ctl"));
        env["ZOPRO_BENCH_PERF_ACK"] = json!(r("ack"));
        argv.extend(["sudo".into(), "-n".into(), "env".into()]);
        for (k, v) in env.as_object().unwrap() {
            argv.push(format!("{k}={}", v.as_str().unwrap()));
        }
        argv.push("perf".into());
        if panel == "stat" {
            let path = r("stat").join(format!("{name}.csv"));
            data = json!(path);
            argv.extend([
                "stat".into(),
                "--no-big-num".into(),
                "-x".into(),
                ";".into(),
                "-e".into(),
                "{cycles:u,instructions:u,cache-misses:u,branch-misses:u}".into(),
                "-o".into(),
                path.display().to_string(),
            ]);
        } else {
            let path = r("perf").join(format!("{name}.data"));
            data = json!(path);
            argv.extend([
                "record".into(),
                "--no-buildid-cache".into(),
                "--sample-cpu".into(),
                "--timestamp".into(),
                "-P".into(),
                "-e".into(),
                "cycles:u".into(),
                "-c".into(),
                "16000000".into(),
                "-m".into(),
                "512".into(),
                "-o".into(),
                path.display().to_string(),
            ]);
        }
        argv.extend([
            "--delay=-1".into(),
            format!("--control=fifo:{R}/ctl,{R}/ack"),
            "--".into(),
        ]);
    }
    argv.extend([
        "taskset".into(),
        "-c".into(),
        if w == 1 {
            "2".into()
        } else {
            format!("0-{}", w - 1)
        },
        ids[arm]["binary"].as_str().unwrap().into(),
        query.into(),
        case["fixture"].as_str().unwrap().into(),
        case["oracle"].as_str().unwrap().into(),
        w.to_string(),
        rounds.to_string(),
    ]);
    let mut step = command(&name, argv);
    if panel == "latency" || panel == "preflight" {
        step["environment"] = env;
    }
    let mut job = json!({"name":name,"case":case,"variant":arm,"panel":panel,"block":block,"rounds":rounds,"actual_query":query,"binary":ids[arm]["binary"],"data":data,"command":step});
    if let Some(old) = reuse {
        let p = old["reused_from"]
            .as_str()
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                Path::new(OLD)
                    .join("runs")
                    .join(old["name"].as_str().unwrap())
            });
        job["reused_from"] = json!(p);
    } else {
        steps.push(step);
    }
    jobs.push(job);
}

fn prepare(){
 assert!(!r("capture-plan.json").exists());assert_eq!(read(r("native/result.json"))["status"],"complete");
 for tag in["OFF","ON"]{let t=fs::read_to_string(r(format!("native/core-drain-{tag}/stdout.log"))).unwrap();assert!(t.contains("test result: ok. 130 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out"));}
 let link=fs::read_to_string(r("native/driver-build/stderr.log")).unwrap();let line=link.lines().find(|l|l.contains("Running")&&l.contains("--crate-name zopro_queue_index_composition")).unwrap();
 for f in["-C opt-level=3","-C lto=fat","-C codegen-units=1","-C target-cpu=native","--cfg zopro_contained_index","--cfg zopro_queue_drain","--cfg zopro_queue_index_composition"]{assert!(line.contains(f),"{f}");}
 assert!(!r("candidate-bin").exists());let new=r("target/release/zopro-queue-index-composition");fs::copy(&new,r("candidate-bin")).unwrap();let mut ids=serde_json::Map::new();for arm in ARMS{ids.insert(arm.into(),json!({"binary":r("candidate-bin"),"binary_sha256":hash(&new),"actual_rustc":line}));}let ids=Value::Object(ids);save(r("identities.json"),&ids);
 let fixtures=read(r("fixtures.json"));let mut cases=vec![];for shape in["1x2","2x2"]{for w in[1,16,32]{let mut c=fixtures[format!("{shape}-B")].clone();c["name"]=json!(format!("{shape}-B-w{w}"));c["workers"]=json!(w);cases.push(c);}}let mut a=fixtures["1x2-A"].clone();a["name"]=json!("1x2-A-w32");a["workers"]=json!(32);cases.push(a);save(r("cases.json"),&json!(cases));
 let mut env=read(r("environment.json"));for k in["RUSTFLAGS","RUSTC","CARGO_BUILD_JOBS","CARGO_TARGET_DIR"]{env.as_object_mut().unwrap().remove(k);}
 let mut jobs=vec![];let mut steps=vec![];for c in &cases{for arm in ARMS{add(c,arm,0,"preflight",&env,&ids,&mut jobs,&mut steps,None);}}steps.push(command("preflight-check",vec![r("campaign").display().to_string(),"preflight".into()]));
 for b in 0..15{let mut order=(0..7).collect::<Vec<_>>();order.rotate_left(b%7);if b%2==1{order.reverse();}let mut arms=ARMS;arms.rotate_left(b%4);if b%2==1{arms.reverse();}for i in order{for arm in arms{add(&cases[i],arm,b,"latency",&env,&ids,&mut jobs,&mut steps,None);}}}
 for d in["stat","perf"]{fs::create_dir(r(d)).unwrap();}for n in["ctl","ack"]{assert!(Command::new("mkfifo").arg(r(n)).status().unwrap().success());}
 let c=cases.iter().find(|c|c["name"]=="1x2-B-w32").unwrap();for panel in["stat","cycles"]{for arm in ARMS{add(c,arm,0,panel,&env,&ids,&mut jobs,&mut steps,None);}}
 for j in jobs.iter().filter(|j|j["panel"]=="cycles"){let n=j["name"].as_str().unwrap();let d=j["data"].as_str().unwrap();steps.push(command(&format!("own-{n}"),vec!["sudo".into(),"-n".into(),r("campaign").display().to_string(),"own".into(),d.into()]));for meta in[false,true]{let mut argv=vec!["perf".into(),"script".into(),"--header".into(),"--ns".into(),"-i".into(),d.into(),"-F".into(),"pid,tid,cpu,time,period,ip,sym".into()];if meta{argv.extend(["--show-mmap-events".into(),"--show-task-events".into(),"--show-lost-events".into()]);}steps.push(command(&format!("{}-{n}",if meta{"metadata"}else{"decode"}),argv));}steps.push(command(&format!("events-{n}"),vec!["perf".into(),"evlist".into(),"-v".into(),"-i".into(),d.into()]));steps.push(command(&format!("loss-{n}"),vec![r("campaign").display().to_string(),"loss".into(),d.into()]));}
 assert_eq!(jobs.len(),456);assert_eq!(steps.len(),477);save(r("jobs.json"),&json!(jobs));
 let mut pins=BTreeMap::new();for d in["source","driver","native"]{walk(&r(d),&mut pins);}for f in["setup.rs","finish-setup.rs","setup-first-failure.txt","startup-command-correction.txt","design.md","history-audit.md","protocol.md","source-audit.json","driver-audit.json","build-plan.json","build-pins.json","prepare-campaign.rs","campaign.rs","campaign","environment.json","fixtures.json","cases.json","jobs.json","identities.json","candidate-bin"]{pins.insert(r(f).display().to_string(),hash(r(f)));}for c in &cases{for k in["fixture","oracle"]{let p=c[k].as_str().unwrap();pins.insert(p.into(),hash(p));}}
 save(r("capture-pins.json"),&json!(pins));save(r("capture-plan.json"),&json!({"version":1,"source_pins":r("capture-pins.json"),"steps":steps}));println!("Prepared477capturecommands/456freshcallers/420latencies then8PMU/profile;onebinary/fourarms. Native130tests perdrainmode and optimizedargv checked. No capture started.");
}
fn preflight(){let jobs=read(r("jobs.json"));let mut count=0;for j in jobs.as_array().unwrap().iter().filter(|j|j["panel"]=="preflight"){caller(j);count+=1;}assert_eq!(count,28);println!("All28freshpreflights match independentwholeMSMoracles,actualtoproutes and bothprocess selectors; pointsunchanged.");}
fn own(data: &str) {
    assert!(Path::new(data).starts_with(r("perf")) && data.ends_with(".data"));
    let before = hash(data);
    let m = fs::metadata(r("campaign")).unwrap();
    assert!(Command::new("chown")
        .arg(format!("{}:{}", m.uid(), m.gid()))
        .arg(data)
        .status()
        .unwrap()
        .success());
    assert_eq!(hash(data), before);
    println!(
        "{}",
        json!({"data":data,"sha256_before":before,"sha256_after":hash(data)})
    );
}
fn loss(data: &str) {
    let mut child = Command::new("perf")
        .args(["script", "-D", "-i", data])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut lost = vec![];
    let mut lines = 0;
    for line in BufReader::new(child.stdout.take().unwrap()).lines() {
        let line = line.unwrap();
        lines += 1;
        if line.contains("PERF_RECORD_LOST") {
            lost.push(line);
        }
    }
    assert!(child.wait().unwrap().success());
    println!(
        "{}",
        json!({"lines":lines,"zero_lost":lost.is_empty(),"lost":lost})
    );
}
fn main() {
    let a = std::env::args().collect::<Vec<_>>();
    match a.get(1).map(String::as_str) {
        Some("prepare") => prepare(),
        Some("preflight") => preflight(),
        Some("own") => own(&a[2]),
        Some("loss") => loss(&a[2]),
        _ => panic!("prepare/preflight/own/loss"),
    }
}
