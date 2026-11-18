use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

fn sample(pid: u32) -> Result<BTreeMap<u32, Value>, Box<dyn Error>> {
    let mut samples = BTreeMap::new();
    for entry in fs::read_dir(format!("/proc/{pid}/task"))? {
        let entry = entry?;
        let tid: u32 = entry.file_name().into_string().unwrap().parse()?;
        let status = fs::read_to_string(entry.path().join("status"))?;
        let field = |key: &str| -> Result<&str, Box<dyn Error>> {
            Ok(status
                .lines()
                .find_map(|line| line.strip_prefix(key))
                .ok_or("missing task field")?
                .trim())
        };
        let sched = fs::read_to_string(entry.path().join("schedstat"))?;
        let v = sched
            .split_whitespace()
            .map(str::parse::<u64>)
            .collect::<Result<Vec<_>, _>>()?;
        if v.len() != 3 {
            return Err("unexpected schedstat".into());
        }
        samples.insert(
            tid,
            json!({
                "cpu_ns": v[0], "runqueue_ns": v[1], "slices": v[2],
                "voluntary": field("voluntary_ctxt_switches:")?.parse::<u64>()?,
                "involuntary": field("nonvoluntary_ctxt_switches:")?.parse::<u64>()?,
                "allowed": field("Cpus_allowed_list:")?
            }),
        );
    }
    Ok(samples)
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    let pid = args.get(1).ok_or("missing PID")?.parse()?;
    let dest = Path::new(args.get(2).ok_or("missing directory")?);
    let ctl = OpenOptions::new()
        .read(true)
        .write(true)
        .open(dest.join("ctl"))?;
    let mut ack = OpenOptions::new()
        .read(true)
        .write(true)
        .open(dest.join("ack"))?;
    let mut input = BufReader::new(ctl);
    let mut result = Vec::new();
    for expected in ["enable", "disable"] {
        let mut line = String::new();
        input.read_line(&mut line)?;
        if line.trim() != expected {
            return Err("unexpected SDK timing marker".into());
        }
        result.push(sample(pid)?);
        ack.write_all(b"ack\n")?;
        ack.flush()?;
    }
    if result[0].keys().ne(result[1].keys()) {
        return Err("task set changed".into());
    }
    fs::write(
        dest.join("tasks.json"),
        serde_json::to_vec(&json!({"caller":pid, "snapshots":result}))?,
    )?;
    Ok(())
}
