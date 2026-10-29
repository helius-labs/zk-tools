use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    time::{Duration, Instant},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/balanced-kh-compatibility175";
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn read(p: impl AsRef<Path>) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn save(p: impl AsRef<Path>, v: &Value) -> Result<()> {
    fs::write(p, serde_json::to_vec_pretty(v)?)?;
    Ok(())
}
fn hash(p: impl AsRef<Path>) -> Result<String> {
    let mut f = fs::File::open(p)?;
    let mut h = Sha256::new();
    let mut b = [0u8; 65536];
    loop {
        let n = f.read(&mut b)?;
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    Ok(format!("{:x}", h.finalize()))
}
fn properties(unit: &str) -> Result<BTreeMap<String, String>> {
    let o=Command::new("systemctl").args(["--user","show",unit,"--property=LoadState,ControlGroup,Result,ExecMainCode,ExecMainStatus,ActiveState,SubState,MemoryPeak,MemorySwapPeak,CPUUsageNSec"]).output()?;
    let text = String::from_utf8(o.stdout)?;
    Ok(text
        .lines()
        .filter_map(|s| s.split_once('='))
        .map(|(k, v)| (k.into(), v.into()))
        .collect())
}
struct Unit(String);
impl Drop for Unit {
    fn drop(&mut self) {
        let _ = Command::new("systemctl")
            .args(["--user", "stop", &self.0])
            .output();
        let _ = Command::new("systemctl")
            .args(["--user", "reset-failed", &self.0])
            .output();
        let path = Path::new("/run/user")
            .join(unsafe { libc::getuid() }.to_string())
            .join("systemd/user.control")
            .join(format!("{}.d", self.0));
        if self.0.starts_with("zopro-bal-kh-q-") && path.exists() {
            let _ = fs::remove_dir_all(path);
        }
    }
}
fn cgroup_metrics(cg: &Path) -> Value {
    let mut m = serde_json::Map::new();
    for n in [
        "memory.current",
        "memory.peak",
        "memory.high",
        "memory.max",
        "memory.swap.max",
        "memory.swap.peak",
        "memory.events",
        "memory.pressure",
        "memory.stat",
        "cpu.stat",
    ] {
        if let Ok(s) = fs::read_to_string(cg.join(n)) {
            m.insert(n.into(), json!(s.trim()));
        }
    }
    Value::Object(m)
}
fn numbers(s: &str) -> BTreeMap<String, u64> {
    s.lines()
        .filter_map(|l| {
            let mut w = l.split_whitespace();
            Some((w.next()?.to_owned(), w.next()?.parse().ok()?))
        })
        .collect()
}
fn cache_pages(file: &fs::File) -> Result<Value> {
    use std::os::fd::AsRawFd;
    let len = file.metadata()?.len() as usize;
    let page = unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as usize;
    let pages = len.div_ceil(page);
    let mut v = vec![0u8; pages];
    // PROT_NONE observes residency without touching proving-key pages.
    let p = unsafe {
        libc::mmap(
            std::ptr::null_mut(),
            len,
            libc::PROT_NONE,
            libc::MAP_SHARED,
            file.as_raw_fd(),
            0,
        )
    };
    if p == libc::MAP_FAILED {
        return Err(std::io::Error::last_os_error().into());
    }
    let code = unsafe { libc::mincore(p, len, v.as_mut_ptr()) };
    let err = std::io::Error::last_os_error();
    let unmapped = unsafe { libc::munmap(p, len) };
    if code != 0 || unmapped != 0 {
        return Err(err.into());
    }
    Ok(
        json!({"pages":pages,"resident_pages":v.iter().filter(|b|**b&1!=0).count(),"page_size":page}),
    )
}
fn cold_cache(case: &Value, out: &Path) -> Result<()> {
    use std::os::fd::AsRawFd;
    let p = case["image"].as_str().ok_or("missing image")?;
    let digest = hash(p)?;
    if digest != case["image_sha256"].as_str().unwrap() {
        return Err("large image hash mismatch".into());
    }
    let f = fs::File::open(p)?;
    let before = cache_pages(&f)?;
    f.sync_all()?;
    let result = unsafe { libc::posix_fadvise(f.as_raw_fd(), 0, 0, libc::POSIX_FADV_DONTNEED) };
    if result != 0 {
        return Err(std::io::Error::from_raw_os_error(result).into());
    }
    let after = cache_pages(&f)?;
    save(
        out.join("cold-cache.json"),
        &json!({"image":p,"sha256":digest,"before":before,"after":after,"method":"fsync and POSIX_FADV_DONTNEED on the owned image only; PROT_NONE mincore"}),
    )?;
    if after["resident_pages"] != 0 {
        return Err("owned large image still resident; preserve failure without retry".into());
    }
    Ok(())
}
fn copy_charged(source: &Path, dest: &Path) -> Result<()> {
    let mut a = fs::File::open(source)?;
    let mut b = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(dest)?;
    let mut buf = [0u8; 65536];
    loop {
        let n = a.read(&mut buf)?;
        if n == 0 {
            break;
        }
        b.write_all(&buf[..n])?;
    }
    b.sync_all()?;
    Ok(())
}
fn wait_child(child: &mut Child, timeout: Duration) -> Result<std::process::ExitStatus> {
    let start = Instant::now();
    loop {
        if let Some(s) = child.try_wait()? {
            return Ok(s);
        }
        if start.elapsed() > timeout {
            child.kill()?;
            let _ = child.wait();
            return Err("owned child timeout".into());
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}
fn child_mode(mode: &str, out: &Path, case: &Value) -> Result<()> {
    let input = out.join("input");
    fs::create_dir(&input)?;
    let src = Path::new(case["image"].as_str().unwrap());
    let copy = input.join(src.file_name().unwrap());
    copy_charged(src, &copy)?;
    let stem = out.join("proofs/transfer_confidential_1_2");
    let binary = case["binary"].as_str().unwrap();
    let request = case["request"].as_str().unwrap();
    let mut cmd = Command::new(binary);
    if mode == "warm-child" {
        cmd.arg(&copy)
            .arg(request)
            .arg(&stem)
            .env("ZOPRO_PRESSURE_PAUSE", out);
    } else {
        cmd.arg(&copy).arg(request).arg("1").arg(&stem);
    }
    cmd.stdout(Stdio::from(fs::File::create(out.join("sdk.log"))?))
        .stderr(Stdio::from(fs::File::create(out.join("sdk-stderr.log"))?));
    let mut p = cmd.spawn()?;
    save(
        out.join("sdk-process.json"),
        &json!({"pid":p.id(),"controller_pid":std::process::id()}),
    )?;
    let mut ballast = Vec::new();
    if mode == "warm-child" {
        let start = Instant::now();
        while !out.join("ready").exists() {
            if p.try_wait()?.is_some() {
                return Err("SDK ended before warm pressure barrier".into());
            }
            if start.elapsed() > Duration::from_secs(60) {
                p.kill()?;
                let _ = p.wait();
                return Err("warm priming timeout".into());
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        ballast.resize(32 << 20, 0u8);
        for i in (0..ballast.len()).step_by(4096) {
            ballast[i] = 1;
        }
        std::hint::black_box(&ballast);
        fs::write(out.join("ballast-ready"), b"ready")?;
    }
    let status = wait_child(&mut p, Duration::from_secs(100))?;
    std::hint::black_box(&ballast);
    let cg = fs::read_to_string("/proc/self/cgroup")?;
    let rel = cg.trim().split_once("::").ok_or("missing cgroup2 scope")?.1;
    let cg = Path::new("/sys/fs/cgroup").join(rel.trim_start_matches('/'));
    save(
        out.join("child-receipt.json"),
        &json!({"code":status.code(),"success":status.success(),"ballast_bytes":ballast.len(),"cgroup":cg,"metrics":cgroup_metrics(&cg)}),
    )?;
    fs::remove_file(copy)?;
    fs::remove_dir(input)?;
    if !status.success() {
        return Err("SDK resource child failed".into());
    }
    Ok(())
}
fn run_unit(case: &Value, root: &Path) -> Result<Value> {
    let name = case["name"].as_str().unwrap();
    assert!(name.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'-'));
    let out = root.join("runs").join(name);
    fs::create_dir(&out)?;
    save(out.join("case.json"), case)?;
    let large = case["kind"] == "large";
    let warm = case["kind"] == "warm";
    if large {
        cold_cache(case, &out)?;
    }
    let unit = format!("zopro-bal-kh-q-{}-{name}.service", std::process::id());
    let before = properties(&unit)?;
    if before.get("LoadState").is_some_and(|s| s != "not-found") {
        return Err("refusing to reuse an existing systemd unit".into());
    }
    let guard = Unit(unit.clone());
    let initial = if large {
        95u64 << 30
    } else if warm {
        512u64 << 20
    } else {
        192u64 << 20
    };
    let inputs = read(root.join("inputs.json"))?;
    let mut env = inputs["environment"].as_object().unwrap().clone();
    env.insert("RAYON_NUM_THREADS".into(), json!("32"));
    if case["allocator"] == "default" {
        for n in [
            "MALLOC_TRIM_THRESHOLD_",
            "MALLOC_MMAP_THRESHOLD_",
            "MALLOC_ARENA_MAX",
        ] {
            env.remove(n);
        }
    }
    let mut args: Vec<String> = vec![
        "--user".into(),
        format!("--unit={unit}"),
        "--service-type=exec".into(),
        "--wait".into(),
        "--pipe".into(),
        "--property=RemainAfterExit=yes".into(),
        "--property=MemoryAccounting=yes".into(),
        format!("--property=MemoryHigh={initial}"),
        format!("--property=MemoryMax={initial}"),
        "--property=MemorySwapMax=0".into(),
        "--property=OOMPolicy=stop".into(),
        "--property=RuntimeMaxSec=1200".into(),
    ];
    for (k, v) in &env {
        args.push(format!("--setenv={k}={}", v.as_str().unwrap()));
    }
    args.push("/usr/bin/env".into());
    if case["allocator"] == "default" {
        for n in [
            "MALLOC_TRIM_THRESHOLD_",
            "MALLOC_MMAP_THRESHOLD_",
            "MALLOC_ARENA_MAX",
        ] {
            args.extend(["-u".into(), n.into()]);
        }
    }
    if large {
        args.extend([
            case["binary"].as_str().unwrap().into(),
            case["image"].as_str().unwrap().into(),
            case["witnesses"].as_str().unwrap().into(),
            out.join("proofs").display().to_string(),
        ]);
    } else {
        args.extend([
            root.join("resources").display().to_string(),
            if warm {
                "warm-child".into()
            } else {
                "cold-child".into()
            },
            out.display().to_string(),
        ]);
    }
    save(
        out.join("command.json"),
        &json!({"program":"systemd-run","args":args,"unit":unit,"environment":env}),
    )?;
    let mut launcher = Command::new("systemd-run")
        .args(&args)
        .stdout(Stdio::from(fs::File::create(out.join(if large {
            "sdk.log"
        } else {
            "unit.log"
        }))?))
        .stderr(Stdio::from(fs::File::create(out.join("unit-stderr.log"))?))
        .spawn()?;
    save(
        out.join("launcher-process.json"),
        &json!({"pid":launcher.id(),"controller_pid":std::process::id(),"unit":unit}),
    )?;
    let start = Instant::now();
    let mut cg = None;
    let mut last = json!({});
    let mut lowered = false;
    let mut snapshots = fs::File::create(out.join("memory.jsonl"))?;
    let final_properties = loop {
        let props = properties(&unit)?;
        if cg.is_none() {
            if let Some(p) = props.get("ControlGroup").filter(|s| !s.is_empty()) {
                if !p.starts_with("/user.slice/") || !p.ends_with(&unit) {
                    return Err("unexpected cgroup path for owned unit".into());
                }
                cg = Some(Path::new("/sys/fs/cgroup").join(p.trim_start_matches('/')));
            }
        }
        if let Some(cg) = &cg {
            let m = cgroup_metrics(cg);
            if m.get("memory.peak").is_some() {
                last = m;
                writeln!(
                    snapshots,
                    "{}",
                    json!({"seconds":start.elapsed().as_secs_f64(),"metrics":last})
                )?;
                snapshots.flush()?;
            }
        }
        if warm && !lowered && out.join("ballast-ready").exists() {
            let s = Command::new("systemctl")
                .args([
                    "--user",
                    "set-property",
                    "--runtime",
                    &unit,
                    "MemoryHigh=134217728",
                    "MemoryMax=201326592",
                ])
                .output()?;
            if !s.status.success() {
                return Err("failed to set declared warm limits".into());
            }
            let m = cgroup_metrics(cg.as_ref().ok_or("no cgroup at barrier")?);
            if m["memory.high"] != "134217728"
                || m["memory.max"] != "201326592"
                || m["memory.swap.max"] != "0"
            {
                return Err("warm limits were not applied exactly".into());
            }
            save(out.join("lowered-limits.json"), &m)?;
            lowered = true;
            fs::write(out.join("go"), b"go")?;
        }
        if props
            .get("SubState")
            .is_some_and(|s| s == "exited" || s == "failed")
        {
            break props;
        }
        if let Some(status) = launcher.try_wait()? {
            save(
                out.join("unexpected-launcher-exit.json"),
                &json!({"code":status.code(),"properties":props}),
            )?;
            return Err("launcher ended before an observed terminal service state".into());
        }
        if start.elapsed() > Duration::from_secs(1250) {
            return Err("owned systemd service timeout".into());
        }
        std::thread::sleep(Duration::from_millis(250));
    };
    let normal = final_properties
        .get("Result")
        .is_some_and(|s| s == "success")
        && final_properties
            .get("ExecMainCode")
            .is_some_and(|s| s == "1")
        && final_properties
            .get("ExecMainStatus")
            .is_some_and(|s| s == "0");
    let metrics = if !large && out.join("child-receipt.json").exists() {
        read(out.join("child-receipt.json"))?["metrics"].clone()
    } else {
        last.clone()
    };
    let events = numbers(
        metrics["memory.events"]
            .as_str()
            .ok_or("missing final cgroup memory events")?,
    );
    let desired = if large { 95u64 << 30 } else { 192u64 << 20 };
    let limits = metrics["memory.max"].as_str() == Some(&desired.to_string())
        && metrics["memory.swap.max"] == "0";
    let success = normal
        && limits
        && (!warm || lowered)
        && events.get("oom").copied().unwrap_or(0) == 0
        && events.get("oom_kill").copied().unwrap_or(0) == 0
        && (large || events.get("max").copied().unwrap_or(0) == 0);
    let result = json!({"case":case,"unit":unit,"normal_exit":normal,"passed":success,"seconds":start.elapsed().as_secs_f64(),"final_properties":final_properties,"metrics":metrics,"sampled_final_metrics":last,"lowered":lowered,"memory_events":events,"SDK_requests":if large{0}else if warm{5}else{4},"core_requests":if large{2}else{0},"expected_exports":if large||warm{2}else{1}});
    save(out.join("result.json"), &result)?;
    drop(guard);
    let status = wait_child(&mut launcher, Duration::from_secs(20))?;
    save(
        out.join("launcher-exit.json"),
        &json!({"code":status.code()}),
    )?;
    if !success || !status.success() {
        return Err(format!("resource case {name} failed; no larger retry").into());
    }
    let verifier: Vec<String> = case["verifier"]
        .as_array()
        .ok_or("missing independent verifier")?
        .iter()
        .map(|v| {
            v.as_str()
                .unwrap()
                .replace("{proofs}", &out.join("proofs").display().to_string())
        })
        .collect();
    let verified = Command::new(&verifier[0])
        .args(&verifier[1..])
        .stdout(Stdio::from(fs::File::create(out.join("verifier.log"))?))
        .stderr(Stdio::from(fs::File::create(
            out.join("verifier-stderr.log"),
        )?))
        .status()?;
    save(
        out.join("verifier-exit.json"),
        &json!({"code":verified.code(),"success":verified.success(),"argv":verifier}),
    )?;
    if !verified.success() {
        return Err(format!("independent verifier failed for {name}").into());
    }
    Ok(result)
}
fn run() -> Result<()> {
    let root = Path::new(R);
    let cfg = read(root.join("resource-cases.json"))?;
    let limits = read(root.join("resource-limit-derivation.json"))?;
    if limits["test_memory_max_MiB"] != 192
        || limits["test_warm_high_MiB"] != 128
        || limits["warm_priming_max_MiB"] != 512
        || limits["ballast_MiB"] != 32
        || limits["swap_max"] != 0
        || cfg.as_array().map(Vec::len) != Some(8)
        || cfg
            .as_array()
            .unwrap()
            .iter()
            .any(|c| c["kind"] != "cold" && c["kind"] != "warm")
    {
        return Err("resource limits or declared eight-case scope changed".into());
    }
    fs::create_dir(root.join("runs"))?;
    let mut rows = Vec::new();
    for case in cfg.as_array().unwrap() {
        println!("RUN_RESOURCE {}", case["name"]);
        std::io::stdout().flush()?;
        let result = run_unit(case, root)?;
        println!(
            "DONE_RESOURCE {} peak {}",
            case["name"], result["metrics"]["memory.peak"]
        );
        rows.push(result);
        save(root.join("resource-progress.json"), &json!(rows))?;
    }
    save(
        root.join("resource-result.json"),
        &json!({"passed":true,"rows":rows,"SDK_requests":36,"core_requests":0,"independent_exports_planned":12,"adopted":false}),
    )?;
    Ok(())
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    let result = match a.get(1).map(String::as_str) {
        Some("run") => run(),
        Some(mode @ ("cold-child" | "warm-child")) => {
            let out = PathBuf::from(a.get(2).expect("resource child output"));
            let case = read(out.join("case.json")).unwrap();
            child_mode(mode, &out, &case)
        }
        _ => Err("usage: resources run | cold-child OUT | warm-child OUT".into()),
    };
    if let Err(e) = result {
        let _ = save(
            Path::new(R).join(format!("resource-failure-{}.json", std::process::id())),
            &json!({"error":e.to_string(),"args":a,"pid":std::process::id()}),
        );
        eprintln!("resources: {e}");
        std::process::exit(1)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn memory_events_parse_counts() {
        let v = numbers("low 0\nhigh 15\nmax 0\noom 0\noom_kill 0\n");
        assert_eq!(v["high"], 15);
        assert_eq!(v["max"], 0);
    }
    #[test]
    fn cache_probe_does_not_fault_file_pages() {
        use std::os::fd::AsRawFd;
        let p = std::env::temp_dir().join(format!("zopro-mincore-q-{}", std::process::id()));
        let f = fs::OpenOptions::new()
            .create_new(true)
            .read(true)
            .write(true)
            .open(&p)
            .unwrap();
        f.set_len(8192).unwrap();
        f.sync_all().unwrap();
        assert_eq!(
            unsafe { libc::posix_fadvise(f.as_raw_fd(), 0, 0, libc::POSIX_FADV_DONTNEED) },
            0
        );
        let a = cache_pages(&f).unwrap();
        let b = cache_pages(&f).unwrap();
        assert_eq!(a["resident_pages"], 0);
        assert_eq!(a, b);
        fs::remove_file(p).unwrap();
    }
}
