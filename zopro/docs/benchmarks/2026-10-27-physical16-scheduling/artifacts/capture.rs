use serde_json::{json, Value};
use std::{
    fs,
    path::Path,
    process::{Command, Stdio},
};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 3);
    let c = read("/tmp/zopro-physical16-scheduling/inputs.json");
    let variant = c["variants"]
        .as_array()
        .unwrap()
        .iter()
        .find(|v| v["name"] == args[1])
        .unwrap();
    let d = Path::new(&args[2]);
    fs::create_dir(d).unwrap();
    for name in ["ctl", "ack"] {
        assert!(Command::new("mkfifo")
            .arg("-m")
            .arg("600")
            .arg(d.join(name))
            .status()
            .unwrap()
            .success());
    }
    let mut cmd: Vec<String> = [
        "sudo",
        "-n",
        "/usr/bin/perf",
        "record",
        "--clockid",
        "mono",
        "--no-buildid-cache",
        "--sample-cpu",
        "-e",
        "cycles/freq=199/u",
        "-e",
        "sched:sched_switch",
        "-e",
        "syscalls:sys_enter_sched_yield",
        "-e",
        "syscalls:sys_enter_futex",
        "--switch-events",
        "-m",
        "2048",
        "--delay=-1",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    cmd.push(format!(
        "--control=fifo:{},{}",
        d.join("ctl").display(),
        d.join("ack").display()
    ));
    cmd.extend([
        "-o".to_owned(),
        d.join("perf.data").display().to_string(),
        "--".to_owned(),
        "sudo".to_owned(),
        "-n".to_owned(),
        "-u".to_owned(),
        "ubuntu".to_owned(),
        "env".to_owned(),
    ]);
    let mut env = c["environment"].as_object().unwrap().clone();
    env.insert("RAYON_NUM_THREADS".into(), json!("16"));
    env.insert("ZOPRO_BENCH_PERF_CTL".into(), json!(d.join("ctl")));
    env.insert("ZOPRO_BENCH_PERF_ACK".into(), json!(d.join("ack")));
    for (k, v) in &env {
        cmd.push(format!("{k}={}", v.as_str().unwrap()));
    }
    cmd.extend([
        "taskset".into(),
        "-c".into(),
        "0-15".into(),
        variant["binary"].as_str().unwrap().into(),
        c["case"]["image"].as_str().unwrap().into(),
        c["case"]["request"].as_str().unwrap().into(),
        "501".into(),
        d.join("proofs/transfer_confidential_1_2")
            .display()
            .to_string(),
    ]);
    fs::write(
        d.join("command.json"),
        serde_json::to_vec_pretty(&cmd).unwrap(),
    )
    .unwrap();
    let mut child = Command::new(&cmd[0])
        .args(&cmd[1..])
        .stdout(Stdio::from(fs::File::create(d.join("sdk.log")).unwrap()))
        .stderr(Stdio::from(fs::File::create(d.join("perf.log")).unwrap()))
        .spawn()
        .unwrap();
    fs::write(
        d.join("perf-process.json"),
        serde_json::to_vec_pretty(
            &json!({"wrapper_pid":std::process::id(), "child_pid":child.id()}),
        )
        .unwrap(),
    )
    .unwrap();
    let status = child.wait().unwrap();
    fs::write(
        d.join("exit.json"),
        serde_json::to_vec_pretty(&json!({"success":status.success(),"code":status.code()}))
            .unwrap(),
    )
    .unwrap();
    assert!(
        status.success(),
        "capture failed; retain artifacts and do not rerun successful processes"
    );
}
