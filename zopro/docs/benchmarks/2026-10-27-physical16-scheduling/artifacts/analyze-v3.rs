use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{BufRead, BufReader},
    path::Path,
    process::Command,
};
const R: &str = "/tmp/zopro-physical16-scheduling";
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn ns(s: &str) -> Result<u64> {
    let (a, b) = s
        .trim_end_matches(':')
        .split_once('.')
        .ok_or("missing fractional timestamp")?;
    if b.len() != 9 {
        return Err("expected nanosecond perf timestamps".into());
    }
    Ok(a.parse::<u64>()?
        .checked_mul(1_000_000_000)
        .ok_or("timestamp overflow")?
        .checked_add(b.parse()?)
        .ok_or("timestamp overflow")?)
}
struct Header<'a> {
    pid: u32,
    tid: u32,
    time: u64,
    words: Vec<&'a str>,
    after_time: usize,
}
fn header(line: &str) -> Result<Option<Header<'_>>> {
    if line.contains("PERF_RECORD_LOST") || line.contains("LOST_SAMPLES") {
        return Err("lost perf records".into());
    }
    let w: Vec<_> = line.split_whitespace().collect();
    let sampled = w.first().and_then(|v| v.split_once('/'));
    if sampled.is_none() && !line.contains("PERF_RECORD_SWITCH") {
        return Ok(None);
    }
    let i = w
        .iter()
        .position(|s| s.ends_with(':') && s.as_bytes().first().is_some_and(u8::is_ascii_digit))
        .ok_or("missing perf timestamp")?;
    let (pid, tid) = if let Some((p, t)) = sampled {
        (p.parse()?, t.parse()?)
    } else {
        // perf's synthetic SWITCH records use COMM TID [CPU], irrespective
        // of the requested sample fields. Their TIDs must be joined to the
        // sampled process's own thread set; COMM is never used as identity.
        (
            0,
            w.get(i.checked_sub(2).ok_or("missing switch TID")?)
                .ok_or("missing switch TID")?
                .parse()?,
        )
    };
    Ok(Some(Header {
        pid,
        tid,
        time: ns(w[i])?,
        words: w,
        after_time: i + 1,
    }))
}
#[derive(Default)]
struct Thread {
    samples: u64,
    period: u64,
    yields: u64,
    futex: u64,
    outs: u64,
    ins: u64,
    preempt: u64,
    perf_flagged_runnable: u64,
    next_own_runnable: u64,
    next_other_runnable: u64,
    yield_associated_outs: u64,
    runnable: u64,
    sleeping: u64,
    unknown: u64,
    paired: u64,
    off_ns: u64,
    preempt_off_ns: u64,
    yield_off_ns: u64,
    runnable_off_ns: u64,
    sleep_off_ns: u64,
    unknown_off_ns: u64,
    unmatched_in: u64,
    unmatched_out: u64,
    first_in: u64,
    states: BTreeMap<String, u64>,
    last_yield: Option<u64>,
    last_state: Option<(u64, String)>,
    pending: Option<(u64, bool, bool, String)>,
}
fn threads_value(threads: &BTreeMap<u32, Thread>) -> Value {
    json!(threads.iter().map(|(tid,t)|json!({"tid":tid,"cycle_samples":t.samples,"cycle_period":t.period,"sched_yield_entries":t.yields,"futex_entries":t.futex,"switch_outs":t.outs,"switch_ins":t.ins,"preempt_outs":t.preempt,"runnable_outs":t.runnable,"sleeping_outs":t.sleeping,"unknown_state_outs":t.unknown,"paired_off_cpu_intervals":t.paired,"off_cpu_ms":t.off_ns as f64/1e6,"preempt_off_cpu_ms":t.preempt_off_ns as f64/1e6,"yield_associated_off_cpu_ms":t.yield_off_ns as f64/1e6,"other_runnable_off_cpu_ms":t.runnable_off_ns as f64/1e6,"sleep_off_cpu_ms":t.sleep_off_ns as f64/1e6,"unknown_off_cpu_ms":t.unknown_off_ns as f64/1e6,"unmatched_in":t.unmatched_in,"unmatched_out":t.unmatched_out,"first_boundary_in":t.first_in,"trace_switch_states":t.states})).collect::<Vec<_>>())
}
fn trace(path: &Path) -> Result<Value> {
    let mut pids = BTreeSet::new();
    let mut own_tids = BTreeSet::new();
    let mut first = u64::MAX;
    let mut last = 0;
    let mut records = 0u64;
    let mut events = BTreeMap::<String, u64>::new();
    let mut periods = BTreeMap::<String, BTreeSet<u64>>::new();
    for line in BufReader::new(fs::File::open(path)?).lines() {
        let line = line?;
        if let Some(h) = header(&line)? {
            if !line.contains("PERF_RECORD_") {
                pids.insert(h.pid);
                own_tids.insert(h.tid);
                first = first.min(h.time);
                last = last.max(h.time);
                records += 1;
                let event = if let Some(i) = h.words.iter().position(|v| v.starts_with("cycles/")) {
                    periods
                        .entry("cycles".into())
                        .or_default()
                        .insert(h.words[i - 1].parse()?);
                    "cycles"
                } else if line.contains("syscalls:sys_enter_sched_yield:") {
                    "sched_yield"
                } else if line.contains("syscalls:sys_enter_futex:") {
                    "futex"
                } else if line.contains("sched:sched_switch:") {
                    "sched_switch"
                } else {
                    return Err(format!("unexpected sample: {line}").into());
                };
                *events.entry(event.into()).or_default() += 1;
            }
        }
    }
    if pids.len() != 1 || last <= first {
        return Err("expected one sampled PID and nonempty span".into());
    }
    let pid = *pids.first().unwrap();
    let mut threads = BTreeMap::<u32, Thread>::new();
    let mut symbols = BTreeMap::<String, u64>::new();
    let mut futex_ops = BTreeMap::<String, u64>::new();
    for line in BufReader::new(fs::File::open(path)?).lines() {
        let line = line?;
        let Some(h) = header(&line)? else { continue };
        if (h.pid != pid && !(h.pid == 0 && own_tids.contains(&h.tid)))
            || h.time < first
            || h.time > last
        {
            continue;
        }
        let t = threads.entry(h.tid).or_default();
        if let Some(i) = h.words.iter().position(|v| v.starts_with("cycles/")) {
            let period: u64 = h.words[i - 1].parse()?;
            t.samples += 1;
            t.period += period;
            let name = h
                .words
                .get(i + 2..)
                .ok_or("missing cycles symbol")?
                .join(" ");
            *symbols.entry(name).or_default() += period;
        } else if line.contains("syscalls:sys_enter_sched_yield:") {
            t.yields += 1;
            t.last_yield = Some(h.time);
        } else if line.contains("syscalls:sys_enter_futex:") {
            t.futex += 1;
            let op = h
                .words
                .iter()
                .find_map(|s| s.strip_prefix("op="))
                .unwrap_or("unparsed");
            *futex_ops.entry(op.to_owned()).or_default() += 1;
        } else if line.contains("sched:sched_switch:") {
            let i = h
                .words
                .iter()
                .position(|s| *s == "==>")
                .ok_or("unrecognized sched_switch trace")?;
            let state = h.words[i - 1].to_owned();
            if state.starts_with('R') {
                let next: u32 = h
                    .words
                    .get(i + 1)
                    .and_then(|s| s.rsplit_once(':'))
                    .ok_or("missing next task ID")?
                    .1
                    .parse()?;
                if own_tids.contains(&next) {
                    t.next_own_runnable += 1;
                } else {
                    t.next_other_runnable += 1;
                }
            }
            *t.states.entry(state.clone()).or_default() += 1;
            t.last_state = Some((h.time, state));
        } else if h.words[h.after_time] == "PERF_RECORD_SWITCH" {
            match h.words.get(h.after_time + 1).copied() {
                Some("OUT") => {
                    t.outs += 1;
                    if t.pending.is_some() {
                        t.unmatched_out += 1
                    }
                    if h.words.contains(&"preempt") {
                        t.perf_flagged_runnable += 1;
                    }
                    let state = t
                        .last_state
                        .take()
                        .filter(|(at, _)| h.time.saturating_sub(*at) <= 100_000)
                        .map(|(_, s)| s)
                        .unwrap_or_else(|| "unknown".into());
                    // Linux6.8 perf sets its SWITCH preempt bit for task->on_rq,
                    // including sched_yield. The sched_switch tracepoint '+'
                    // comes from the actual scheduler preempt argument.
                    let preempt = state.ends_with('+');
                    if preempt {
                        t.preempt += 1;
                    }
                    let runnable = state.starts_with('R');
                    if runnable {
                        t.runnable += 1
                    } else if state == "unknown" {
                        t.unknown += 1
                    } else {
                        t.sleeping += 1
                    }
                    let yielded = !preempt
                        && runnable
                        && t.last_yield
                            .take()
                            .is_some_and(|at| h.time.saturating_sub(at) <= 100_000);
                    if yielded {
                        t.yield_associated_outs += 1;
                    }
                    t.pending = Some((h.time, preempt, yielded, state));
                }
                Some("IN") => {
                    t.ins += 1;
                    if let Some((at, preempt, yielded, state)) = t.pending.take() {
                        let d = h
                            .time
                            .checked_sub(at)
                            .ok_or("out-of-order own thread switches")?;
                        t.paired += 1;
                        t.off_ns += d;
                        if preempt {
                            t.preempt_off_ns += d
                        } else if yielded {
                            t.yield_off_ns += d
                        } else if state.starts_with('R') {
                            t.runnable_off_ns += d
                        } else if state == "unknown" {
                            t.unknown_off_ns += d
                        } else {
                            t.sleep_off_ns += d
                        }
                    } else if t.outs == 0 {
                        t.first_in += 1
                    } else {
                        t.unmatched_in += 1
                    }
                }
                _ => return Err("unknown SWITCH direction".into()),
            }
        }
    }
    for t in threads.values_mut() {
        if t.pending.take().is_some() {
            t.unmatched_out += 1
        }
    }
    let sum = |f: fn(&Thread) -> u64| threads.values().map(f).sum::<u64>();
    let samples = sum(|t| t.samples);
    let period = sum(|t| t.period);
    let off = sum(|t| t.off_ns);
    assert_eq!(samples, events.get("cycles").copied().unwrap_or(0));
    assert_eq!(period, symbols.values().sum::<u64>());
    assert_eq!(
        off,
        sum(|t| t.preempt_off_ns
            + t.yield_off_ns
            + t.runnable_off_ns
            + t.sleep_off_ns
            + t.unknown_off_ns)
    );
    if samples == 0 {
        return Err("empty cycles profile".into());
    }
    if events.get("sched_switch").copied().unwrap_or(0) > 0 && sum(|t| t.outs) == 0 {
        return Err("sched_switch samples present but no own-TID SWITCH metadata parsed".into());
    }
    let mut top: Vec<_> = symbols.into_iter().collect();
    top.sort_by(|a, b| b.1.cmp(&a.1));
    let mut result = std::convert::identity(
        json!({"pid":pid,"sampled_threads":threads.len(),"observed_sample_span_ms":(last-first)as f64/1e6,"first_sample_ns":first,"last_sample_ns":last,"sample_records":records,"event_counts":events,"cycles_samples":samples,"cycles_period":period,"cycle_period_distinct_values":periods.get("cycles").map(BTreeSet::len),"sched_yield_entries":sum(|t|t.yields),"futex_entries":sum(|t|t.futex),"futex_operations":futex_ops,"switch_outs":sum(|t|t.outs),"switch_ins":sum(|t|t.ins),"preempt_outs":sum(|t|t.preempt),"runnable_outs":sum(|t|t.runnable),"sleeping_outs":sum(|t|t.sleeping),"unknown_state_outs":sum(|t|t.unknown),"paired_off_cpu_intervals":sum(|t|t.paired),"off_cpu_ms":off as f64/1e6,"preempt_off_cpu_ms":sum(|t|t.preempt_off_ns)as f64/1e6,"yield_associated_off_cpu_ms":sum(|t|t.yield_off_ns)as f64/1e6,"other_runnable_off_cpu_ms":sum(|t|t.runnable_off_ns)as f64/1e6,"sleep_off_cpu_ms":sum(|t|t.sleep_off_ns)as f64/1e6,"unknown_off_cpu_ms":sum(|t|t.unknown_off_ns)as f64/1e6,"unmatched_in":sum(|t|t.unmatched_in),"unmatched_out":sum(|t|t.unmatched_out),"first_boundary_in":sum(|t|t.first_in),"threads":threads_value(&threads),"top_cycle_symbols":top.into_iter().take(30).map(|(s,p)|json!({"symbol":s,"period":p,"percent":100.*p as f64/period as f64})).collect::<Vec<_>>(),"lost_records":0,"scope":"own task events; paired off-CPU time is summed across threads, not exclusive wall time or measured runqueue delay"}),
    );
    result["perf_flagged_runnable_outs"] = json!(sum(|t| t.perf_flagged_runnable));
    result["next_own_runnable_outs"] = json!(sum(|t| t.next_own_runnable));
    result["next_other_runnable_outs"] = json!(sum(|t| t.next_other_runnable));
    result["yield_associated_outs"] = json!(sum(|t| t.yield_associated_outs));
    result["preemption_classification"]=json!("sched_switch trace '+' uses actual scheduler preempt argument; perf SWITCH preempt bit is only task->on_rq in Linux6.8");
    Ok(result)
}
fn main() -> Result<()> {
    let r = Path::new(R);
    let runs = read(r.join("runs.json"));
    assert_eq!(read(r.join("workflow/result.json"))["status"], "complete");
    let inputs = read(r.join("inputs.json"));
    let mut seen: BTreeSet<String> = serde_json::from_value(inputs["prior_proof_sha256"].clone())?;
    assert_eq!(seen.len(), 9060);
    let mut inventory = BTreeMap::new();
    let mut hashes = BTreeMap::new();
    let mut rows = Vec::new();
    for run in runs.as_array().unwrap() {
        let d = Path::new(run["directory"].as_str().unwrap());
        let tracefile = d.join("decoded.trace");
        let original_hashes = read(r.join("raw-local-only-sha256.json"));
        assert_eq!(
            hash(&tracefile),
            original_hashes[tracefile.to_str().unwrap()]
                .as_str()
                .unwrap()
        );
        let decoded = trace(&tracefile)?;
        let original = read(d.join("scheduling-v1.json"));
        assert_eq!(decoded["event_counts"], original["event_counts"]);
        assert_eq!(decoded["cycles_period"], original["cycles_period"]);
        assert_eq!(decoded["pid"], original["pid"]);
        save(d.join("scheduling.json"), &decoded);
        hashes.insert(tracefile.display().to_string(), hash(&tracefile));
        let h = Command::new("sudo")
            .args(["-n", "sha256sum"])
            .arg(d.join("perf.data"))
            .output()?;
        assert!(h.status.success());
        let h = String::from_utf8(h.stdout)?;
        hashes.insert(
            d.join("perf.data").display().to_string(),
            h.split_whitespace().next().unwrap().to_owned(),
        );
        let log = fs::read_to_string(d.join("sdk.log"))?;
        let mut samples: Vec<f64> = log
            .lines()
            .filter_map(|s| s.strip_prefix("SAMPLE "))
            .map(|s| {
                serde_json::from_str::<Value>(s).unwrap()["ms"]
                    .as_f64()
                    .unwrap()
            })
            .collect();
        assert_eq!(samples.len(), 501);
        samples.sort_by(f64::total_cmp);
        let sdk: Value = serde_json::from_str(
            log.lines()
                .find_map(|s| s.strip_prefix("SUMMARY "))
                .ok_or("missing SDK summary")?,
        )?;
        assert_eq!(sdk["rounds"], 501);
        assert_eq!(sdk["priming"], 3);
        assert_eq!(sdk["workers"], 16);
        let proof = d.join("proofs/transfer_confidential_1_2/0.proof");
        assert_eq!(fs::metadata(&proof)?.len(), 164);
        assert_eq!(
            hash(proof.with_extension("public")),
            inputs["case"]["public_sha256"].as_str().unwrap()
        );
        assert!(seen.insert(hash(&proof)), "duplicate proof");
        for p in [proof.clone(), proof.with_extension("public")] {
            inventory.insert(p.display().to_string(), hash(&p));
        }
        rows.push(json!({"run":run,"sdk":sdk,"median_ms":samples[250],"p95_ms":samples[475],"p99_ms":samples[495],"scheduling":decoded}));
    }
    assert_eq!(seen.len(), 9069);
    assert_eq!(inventory.len(), 18);
    for (p, h) in read(r.join("pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap())
    }
    save(r.join("proof-inventory.json"), &json!(inventory));
    save(r.join("raw-local-only-sha256.json"), &json!(hashes));
    save(
        r.join("analysis.json"),
        &json!({"status":"complete diagnostic; not adoption latency", "offline_decoder_revision":3,"original_analysis_not_used_for_switch_attribution":true,"perf_preempt_flag_not_used_as_scheduler_preempt_argument":true,"rows":rows,"SDK_requests":4536,"independent_exports":9,"proof_public_members":18,"prior_count":9060,"next_prior_count":9069,"production_changed":false,"earlier_composed_full_gate_passed":false,"adopted":false,"goal_achieved":false,"original8_CPU10x_rows":0,"settings_changed":false,"lost_records":0}),
    );
    println!("Complete physical16 scheduling diagnostic;4536SDK9Gnarkexports;proofledger9069;no acceptance verdict changed");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn fixture(text: &str) -> std::path::PathBuf {
        let p = std::env::temp_dir().join(format!(
            "zopro-sched-parser-{}-{}",
            std::process::id(),
            text.len()
        ));
        fs::write(&p, text).unwrap();
        p
    }
    #[test]
    fn rejects_loss_and_mixed_processes() {
        assert!(header("PERF_RECORD_LOST 12").is_err());
        let p=fixture("1/1 [000] 1.000000000: 5 cycles/freq=199/u: 10 first\n2/2 [000] 1.000001000: 5 cycles/freq=199/u: 10 second\n");
        assert!(trace(&p).is_err());
        fs::remove_file(p).unwrap();
    }
    #[test]
    fn distinguishes_preempt_yield_and_sleep_without_assigning_wall_cost() {
        let p=fixture("1/1 [000] 1.000000000: 5 cycles/freq=199/u: 10 first\n1/1 [000] 1.000001000: syscalls:sys_enter_sched_yield:\n1/1 [000] 1.000002000: sched:sched_switch: own:1 [120] R ==> other:2 [120]\n1/1 [000] 1.000002100: PERF_RECORD_SWITCH OUT\n1/1 [000] 1.000003100: PERF_RECORD_SWITCH IN\n1/1 [000] 1.000004000: sched:sched_switch: own:1 [120] R+ ==> other:2 [120]\n1/1 [000] 1.000004100: PERF_RECORD_SWITCH OUT preempt\n1/1 [000] 1.000006100: PERF_RECORD_SWITCH IN\n1/1 [000] 1.000007000: syscalls:sys_enter_futex: op=FUTEX_WAIT|FUTEX_PRIVATE_FLAG uaddr=0x1 val=0\n1/1 [000] 1.000008000: sched:sched_switch: own:1 [120] S ==> other:2 [120]\n1/1 [000] 1.000008100: PERF_RECORD_SWITCH OUT\n1/1 [000] 1.000011100: PERF_RECORD_SWITCH IN\n1/1 [000] 1.000012000: 10 cycles/freq=199/u: 10 second\n");
        let v = trace(&p).unwrap();
        assert_eq!(v["cycles_period"], 15);
        assert_eq!(v["paired_off_cpu_intervals"], 3);
        assert_eq!(v["preempt_outs"], 1);
        assert_eq!(v["sched_yield_entries"], 1);
        assert_eq!(v["yield_associated_off_cpu_ms"], 0.001);
        assert_eq!(v["preempt_off_cpu_ms"], 0.002);
        assert_eq!(v["sleep_off_cpu_ms"], 0.003);
        assert_eq!(v["unmatched_out"], 0);
        fs::remove_file(p).unwrap();
    }
    #[test]
    fn reports_unmatched_boundaries() {
        let p=fixture("1/1 [000] 1.000000000: 5 cycles/freq=199/u: 10 first\n1/1 [000] 1.000001000: PERF_RECORD_SWITCH IN\n1/1 [000] 1.000002000: PERF_RECORD_SWITCH OUT\n1/1 [000] 1.000003000: 10 cycles/freq=199/u: 10 second\n");
        let v = trace(&p).unwrap();
        assert_eq!(v["first_boundary_in"], 1);
        assert_eq!(v["unmatched_out"], 1);
        assert_eq!(v["off_cpu_ms"], 0.0);
        fs::remove_file(p).unwrap();
    }
    #[test]
    fn joins_real_perf_switch_format_only_to_sampled_own_tids() {
        let p=fixture("sudo 99 [000] 1.000000001: PERF_RECORD_SWITCH OUT\n1/1 [000] 1.000000000: 5 cycles/freq=199/u: 10 first\n1/2 [000] 1.000001000: syscalls:sys_enter_sched_yield:\n1/2 [000] 1.000002000: sched:sched_switch: own:2 [120] R ==> other:3 [120]\nworker 2 [000] 1.000002100: PERF_RECORD_SWITCH OUT preempt\nworker 2 [000] 1.000003100: PERF_RECORD_SWITCH IN\n1/1 [000] 1.000004000: 10 cycles/freq=199/u: 10 second\n");
        let v = trace(&p).unwrap();
        assert_eq!(v["pid"], 1);
        assert_eq!(v["sampled_threads"], 2);
        assert_eq!(v["switch_outs"], 1);
        assert_eq!(v["paired_off_cpu_intervals"], 1);
        assert_eq!(v["yield_associated_off_cpu_ms"], 0.001);
        assert_eq!(v["preempt_outs"], 0);
        assert_eq!(v["perf_flagged_runnable_outs"], 1);
        assert_eq!(v["yield_associated_outs"], 1);
        assert_eq!(v["next_other_runnable_outs"], 1);
        fs::remove_file(p).unwrap();
    }
}
