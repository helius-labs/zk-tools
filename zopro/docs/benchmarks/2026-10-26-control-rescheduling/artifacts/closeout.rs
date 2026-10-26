use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};
const R: &str = "/tmp/zopro-control-rescheduling";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
mod trace {
    include!("analyze-raw.rs");
    pub fn reasons(path: &Path, pid: u32, windows: &[(u64, u64)]) -> Value {
        let mut es: Vec<Event> = BufReader::new(fs::File::open(path).unwrap())
            .lines()
            .map(|l| parse(&l.unwrap()).unwrap())
            .collect();
        es.sort_by_key(|e| e.ns);
        let own: BTreeSet<_> = es
            .iter()
            .filter(|e| e.pid == pid)
            .map(|e| e.tid)
            .chain([pid])
            .collect();
        let mut calls = BTreeMap::<u32, &str>::new();
        let mut counts = BTreeMap::new();
        for e in &es {
            if e.kind == "sched:sched_switch" {
                let (prev, _, state) = switch(&e.fields);
                if own.contains(&prev) && overlap(e.ns, e.ns + 1, windows) > 0 {
                    let reason = calls
                        .get(&prev)
                        .copied()
                        .unwrap_or("outside_traced_syscall");
                    add(&mut counts, &format!("{state}/{reason}"), 1);
                }
            } else if own.contains(&e.tid) {
                if e.kind.starts_with("syscalls:sys_enter_") {
                    calls.insert(
                        e.tid,
                        if e.kind.ends_with("sched_yield") {
                            "yield"
                        } else {
                            "futex"
                        },
                    );
                } else if e.kind.starts_with("syscalls:sys_exit_") {
                    calls.remove(&e.tid);
                }
            }
        }
        json!(counts)
    }
}
fn main() {
    let r = Path::new(R);
    let base = read("/tmp/zopro-grouping-module/production-source-manifest.json");
    for (p, h) in base["source_files_sha256"].as_object().unwrap() {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)),
            h.as_str().unwrap()
        );
    }
    for (p, h) in read(r.join("raw-padded-run-pins.json"))
        .as_object()
        .unwrap()
    {
        assert_eq!(hash(p), h.as_str().unwrap(), "closing input pin {p}");
    }
    let mut inventory = BTreeMap::new();
    let mut distinct = BTreeSet::new();
    for f in ["preflight-inventory.json", "proof-inventory.json"] {
        for (p, h) in read(r.join(f)).as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
            assert!(inventory.insert(p.clone(), h.clone()).is_none());
            if p.ends_with(".proof") {
                assert!(distinct.insert(h.as_str().unwrap().to_string()));
                assert_eq!(fs::metadata(p).unwrap().len(), 164);
            }
        }
    }
    assert_eq!(inventory.len(), 42);
    assert_eq!(distinct.len(), 21);
    save(r.join("all-proof-inventory.json"), &json!(inventory));
    let mut observations = read(r.join("observations-raw-padded.json"));
    let mut all_reasons = BTreeMap::<String, u64>::new();
    let mut total_events = 0;
    let mut sum_switches = 0;
    let mut preemptions = 0;
    let mut all_states_exact = true;
    for row in observations.as_array_mut().unwrap() {
        assert_eq!(row["trace_usable"], true);
        let d = r.join(row["case"]["name"].as_str().unwrap());
        let pid = row["sdk_summary"]["pid"].as_u64().unwrap() as u32;
        let windows: Vec<_> = fs::read_to_string(d.join("stdout.log"))
            .unwrap()
            .lines()
            .filter_map(|l| l.strip_prefix("SAMPLE "))
            .map(|s| {
                let v: Value = serde_json::from_str(s).unwrap();
                (
                    v["begin_ns"].as_u64().unwrap(),
                    v["end_ns"].as_u64().unwrap(),
                )
            })
            .collect();
        let reasons = trace::reasons(&d.join("raw-events-padded.log"), pid, &windows);
        let count: u64 = reasons
            .as_object()
            .unwrap()
            .values()
            .map(|v| v.as_u64().unwrap())
            .sum();
        let recorded: u64 = row["sdk_switch_states"]
            .as_object()
            .unwrap()
            .values()
            .map(|v| v.as_u64().unwrap())
            .sum();
        assert_eq!(count, recorded);
        for (k, v) in reasons.as_object().unwrap() {
            *all_reasons.entry(k.clone()).or_default() += v.as_u64().unwrap();
        }
        let states = &row["sdk_switch_states"];
        let rcount = states["R"].as_u64().unwrap_or(0) + states["R+"].as_u64().unwrap_or(0);
        let scount = states["S"].as_u64().unwrap_or(0);
        row["switch_reason_counts"] = reasons;
        row["runnable_switches_minus_rusage_nivcsw"] = json!(
            rcount as i64 - row["sdk_summary"]["involuntary_switches"].as_u64().unwrap() as i64
        );
        row["sleep_switches_minus_rusage_nvcsw"] = json!(
            scount as i64 - row["sdk_summary"]["voluntary_switches"].as_u64().unwrap() as i64
        );
        all_states_exact &= row["runnable_switches_minus_rusage_nivcsw"] == 0
            && row["sleep_switches_minus_rusage_nvcsw"] == 0;
        total_events += row["event_count"].as_u64().unwrap();
        sum_switches += count;
        preemptions += row["sdk_switch_states"]["R+"].as_u64().unwrap_or(0);
    }
    save(r.join("observations-final.json"), &observations);
    let decision = json!({"status":"diagnostic complete; prior qualification failure remains","goal_achieved":false,"adopted":false,"small_gain_discarded":false,"production_files_checked_unchanged":170,"sdk_requests":624,"independent_gnark_exports":21,"proof_public_members":42,"prior_count":6793,"next_prior_count":6814,"trace_captures":18,"trace_usable_in_proof_windows":18,"raw_event_count":total_events,"raw_pretty_event_counts_match":true,"lost_samples":0,"sdk_switches_in_windows":sum_switches,"preempted_R_plus_switches":preemptions,"switch_reason_counts":all_reasons,"rusage_switch_counts_exact_all_processes":all_states_exact,"new_untraced_latency_measurement":false,"new_aws_or_aeglos_measurement":false,"memory_minimum_qualified":false,"raw_payloads_local_only":true,"next_action":"Keep fixed-chain candidate; use a separately declared clean Rust integration screen to assess whole-proof behavior without runtime experiment selection. Do not repeat failed idle-wait variants or infer external-load cause from nivcsw."});
    save(r.join("decision.json"), &decision);
    println!("{}", serde_json::to_string_pretty(&decision).unwrap());
    let dest = Path::new(
        "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-26-control-rescheduling",
    );
    let artifacts = dest.join("artifacts");
    fs::create_dir_all(&artifacts).unwrap();
    archive(r, r, &artifacts);
    let mut hashes = BTreeMap::new();
    fn inventory_files(root: &Path, here: &Path, out: &mut BTreeMap<String, String>) {
        let mut files: Vec<_> = fs::read_dir(here)
            .unwrap()
            .map(|e| e.unwrap().path())
            .collect();
        files.sort();
        for p in files {
            if p.is_dir() {
                inventory_files(root, &p, out);
            } else {
                out.insert(
                    p.strip_prefix(root).unwrap().display().to_string(),
                    hash(&p),
                );
            }
        }
    }
    inventory_files(&artifacts, &artifacts, &mut hashes);
    save(dest.join("artifact-index.json"), &json!(hashes));
    println!("Archived {} hashed files", hashes.len());
}
fn archive(root: &Path, here: &Path, out: &Path) {
    let mut files: Vec<_> = fs::read_dir(here)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    files.sort();
    for p in files {
        let rel = p.strip_prefix(root).unwrap();
        if p.is_dir() {
            archive(root, &p, out);
        } else {
            let name = p.file_name().unwrap().to_str().unwrap();
            let rels = rel.display().to_string();
            if name == "perf.data"
                || name.starts_with("raw-events")
                || rels.contains("/decoded/stdout.log")
                || name == "closeout"
                || !["json", "jsonl", "rs", "md", "txt", "log"]
                    .iter()
                    .any(|ext| p.extension().is_some_and(|e| e == *ext))
            {
                continue;
            }
            let dest = out.join(rel);
            fs::create_dir_all(dest.parent().unwrap()).unwrap();
            fs::copy(&p, dest).unwrap();
        }
    }
}
