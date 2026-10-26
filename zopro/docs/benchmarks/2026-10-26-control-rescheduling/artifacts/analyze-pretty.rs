use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Clone, Debug)]
struct Event {
    ns: u64,
    pid: u32,
    tid: u32,
    cpu: usize,
    kind: String,
    fields: String,
}
fn timestamp(s: &str) -> u64 {
    let (a, b) = s.split_once('.').expect("decimal timestamp");
    assert!(b.len() <= 9);
    a.parse::<u64>().unwrap() * 1_000_000_000
        + b.parse::<u64>().unwrap() * 10u64.pow(9 - b.len() as u32)
}
fn parse(line: &str) -> Option<Event> {
    let (head, rest) = line.trim_start().split_once('[')?;
    let (cpu, rest) = rest.split_once(']')?;
    let (time, rest) = rest.trim_start().split_once(": ")?;
    let (kind, fields) = rest
        .trim_start()
        .split_once(": ")
        .or_else(|| rest.trim().strip_suffix(':').map(|k| (k, "")))?;
    let ids: Vec<_> = head.split_whitespace().collect();
    let (pid, tid) = if let Some((p, t)) = ids.last()?.split_once('/') {
        (p.parse().ok()?, t.parse().ok()?)
    } else {
        if ids.len() < 2 {
            return None;
        }
        (
            ids[ids.len() - 2].parse().ok()?,
            ids[ids.len() - 1].parse().ok()?,
        )
    };
    Some(Event {
        ns: timestamp(time),
        pid,
        tid,
        cpu: cpu.parse().ok()?,
        kind: kind.trim().into(),
        fields: fields.into(),
    })
}
fn field<'a>(s: &'a str, key: &str) -> &'a str {
    s.split_whitespace()
        .find_map(|v| v.strip_prefix(key))
        .expect(key)
}
fn switch(s: &str) -> (u32, u32, String) {
    if s.contains("prev_pid=") {
        return (
            field(s, "prev_pid=").parse().unwrap(),
            field(s, "next_pid=").parse().unwrap(),
            field(s, "prev_state=").into(),
        );
    }
    let (left, right) = s.split_once(" ==> ").expect("perf pretty switch arrow");
    let id = |s: &str| -> u32 {
        s.rsplit_once(" [")
            .unwrap()
            .0
            .rsplit_once(':')
            .unwrap()
            .1
            .parse()
            .unwrap()
    };
    (
        id(left),
        id(right),
        left.split_whitespace().last().unwrap().into(),
    )
}
fn overlap(a: u64, b: u64, windows: &[(u64, u64)]) -> u64 {
    if a >= b {
        return 0;
    }
    windows
        .iter()
        .map(|&(l, r)| b.min(r).saturating_sub(a.max(l)))
        .sum()
}
fn add(map: &mut BTreeMap<String, u64>, k: &str, n: u64) {
    *map.entry(k.into()).or_default() += n;
}
fn class(tid: u32, own: &BTreeSet<u32>) -> &'static str {
    if tid == 0 {
        "idle"
    } else if own.contains(&tid) {
        "sdk"
    } else {
        "other"
    }
}
fn median(v: &mut [f64]) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}
fn inspect(events: &mut [Event], windows: &[(u64, u64)], pid: u32, cpus: usize) -> Value {
    assert!(!windows.is_empty());
    assert!(windows.iter().all(|(a, b)| a < b));
    assert!(windows.windows(2).all(|v| v[0].1 <= v[1].0));
    events.sort_by_key(|e| e.ns);
    let own: BTreeSet<u32> = events
        .iter()
        .filter(|e| e.pid == pid)
        .map(|e| e.tid)
        .chain([pid])
        .collect();
    let first = windows[0].0;
    let last = windows.last().unwrap().1;
    let mut occupancy = BTreeMap::<String, u64>::new();
    let mut current = vec![None::<(u64, u32)>; cpus];
    let mut discontinuities = 0;
    let mut calls = BTreeMap::<u32, (String, u64)>::new();
    let mut off = BTreeMap::<u32, (String, u64)>::new();
    let mut syscall_ns = BTreeMap::new();
    let mut syscall_counts = BTreeMap::new();
    let mut censored = BTreeMap::new();
    let mut off_ns = BTreeMap::new();
    let mut switches = BTreeMap::new();
    let mut unknown_syscalls = 0;
    let mut pid_inconsistencies = 0;
    for e in events.iter() {
        assert!(e.cpu < cpus);
        if own.contains(&e.tid) && e.pid != pid && e.tid != 0 {
            pid_inconsistencies += 1;
        }
        if e.kind == "sched:sched_switch" {
            let (prev, next, state) = switch(&e.fields);
            if let Some((start, tid)) = current[e.cpu] {
                if tid != prev {
                    discontinuities += 1;
                }
                add(
                    &mut occupancy,
                    class(tid, &own),
                    overlap(start, e.ns, windows),
                );
            } else {
                add(
                    &mut occupancy,
                    class(prev, &own),
                    overlap(first, e.ns, windows),
                );
            }
            current[e.cpu] = Some((e.ns, next));
            if own.contains(&prev) {
                let category = if let Some((k, _)) = calls.get(&prev) {
                    format!("inside_{k}")
                } else if state == "R+" {
                    "preempted_R+".into()
                } else if state == "R" {
                    "other_runnable".into()
                } else {
                    "blocked".into()
                };
                if off.insert(prev, (category, e.ns)).is_some() {
                    add(&mut censored, "off_replaced", 1);
                }
                if overlap(e.ns, e.ns + 1, windows) > 0 {
                    add(&mut switches, class(next, &own), 1);
                }
            }
            if own.contains(&next) {
                if let Some((k, start)) = off.remove(&next) {
                    add(&mut off_ns, &k, overlap(start, e.ns, windows));
                } else if e.ns > first {
                    add(&mut censored, "off_left_boundary_or_missing", 1);
                }
            }
        } else if own.contains(&e.tid) && e.kind.starts_with("syscalls:") {
            let is_enter = e.kind.contains("sys_enter_");
            let name = if e.kind.ends_with("sched_yield") {
                "yield"
            } else if e.kind.ends_with("futex") {
                "futex"
            } else {
                unknown_syscalls += 1;
                continue;
            };
            if is_enter {
                if overlap(e.ns, e.ns + 1, windows) > 0 {
                    add(&mut syscall_counts, name, 1);
                }
                if calls.insert(e.tid, (name.into(), e.ns)).is_some() {
                    add(&mut censored, "syscall_replaced", 1);
                }
            } else if let Some((k, start)) = calls.remove(&e.tid) {
                assert_eq!(k, name);
                add(&mut syscall_ns, name, overlap(start, e.ns, windows));
            } else {
                add(&mut censored, "syscall_left_boundary_or_missing", 1);
            }
        }
    }
    for value in current {
        if let Some((start, tid)) = value {
            add(
                &mut occupancy,
                class(tid, &own),
                overlap(start, last, windows),
            );
        } else {
            add(
                &mut occupancy,
                "unknown",
                windows.iter().map(|(a, b)| b - a).sum(),
            );
        }
    }
    for (_, (k, start)) in calls {
        add(&mut syscall_ns, &k, overlap(start, last, windows));
        add(&mut censored, "syscall_right_boundary", 1);
    }
    for (_, (k, start)) in off {
        add(&mut off_ns, &k, overlap(start, last, windows));
        add(&mut censored, "off_right_boundary", 1);
    }
    let capacity: u64 = windows.iter().map(|(a, b)| b - a).sum::<u64>() * cpus as u64;
    let observed: u64 = occupancy.values().sum();
    assert_eq!(capacity, observed, "capacity conservation");
    json!({"diagnostic_only":true,"own_tid_count":own.len(),"event_count":events.len(),"proof_window_ns":windows.iter().map(|(a,b)|b-a).sum::<u64>(),"cpu_capacity_ns":capacity,"occupancy_ns":occupancy,"capacity_conserved":capacity==observed,"switch_continuity_errors":discontinuities,"pid_inconsistencies":pid_inconsistencies,"syscall_entries":syscall_counts,"syscall_elapsed_ns_summed_over_threads":syscall_ns,"sdk_off_cpu_ns_summed_over_threads":off_ns,"sdk_switch_destinations":switches,"censored":censored,"unknown_syscalls":unknown_syscalls,"arithmetic_or_latency_qualification":false})
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert_eq!(a.len(), 5, "analyze SCRIPT SDK_STDOUT CPUS OUTPUT");
    let log = fs::read_to_string(&a[2]).unwrap();
    let mut windows = Vec::new();
    let mut timings = Vec::new();
    let mut summary = Value::Null;
    let mut bracket_max_excess_ns = 0.;
    for line in log.lines() {
        if let Some(s) = line.strip_prefix("SAMPLE ") {
            let s: Value = serde_json::from_str(s).unwrap();
            let b = s["begin_ns"].as_u64().unwrap();
            let e = s["end_ns"].as_u64().unwrap();
            let ms = s["ms"].as_f64().unwrap();
            assert!(e > b && (e - b) as f64 + 2. >= ms * 1e6);
            bracket_max_excess_ns = f64::max(bracket_max_excess_ns, (e - b) as f64 - ms * 1e6);
            windows.push((b, e));
            timings.push(ms);
        } else if let Some(s) = line.strip_prefix("SUMMARY ") {
            assert!(summary.is_null());
            summary = serde_json::from_str(s).unwrap();
        }
    }
    assert_eq!(summary["rounds"].as_u64().unwrap() as usize, windows.len());
    let pid = summary["pid"].as_u64().unwrap() as u32;
    let mut events = Vec::new();
    let mut lost_lines = 0;
    let mut unparsed = 0;
    for l in BufReader::new(fs::File::open(&a[1]).unwrap()).lines() {
        let l = l.unwrap();
        if l.contains("LOST") || l.contains("lost events") {
            lost_lines += 1;
            continue;
        }
        if let Some(e) = parse(&l) {
            events.push(e);
        } else if !l.trim().is_empty() && !l.starts_with('#') {
            unparsed += 1;
        }
    }
    let mut out = inspect(&mut events, &windows, pid, a[3].parse().unwrap());
    out["lost_record_lines"] = json!(lost_lines);
    out["unparsed_noncomment_lines"] = json!(unparsed);
    out["sdk_summary"] = summary;
    out["median_instrumented_ms"] = json!(median(&mut timings));
    out["bracket_max_excess_ns"] = json!(bracket_max_excess_ns);
    out["trace_usable"] = json!(
        lost_lines == 0
            && unparsed == 0
            && out["switch_continuity_errors"] == 0
            && out["pid_inconsistencies"] == 0
            && out["own_tid_count"].as_u64().unwrap() > 1
    );
    fs::write(Path::new(&a[4]), serde_json::to_vec_pretty(&out).unwrap()).unwrap();
    println!("{} events; usable={}", events.len(), out["trace_usable"]);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nanoseconds_and_fields() {
        let empty = parse("42 43 [001] 1.000000001: syscalls:sys_enter_sched_yield: ").unwrap();
        assert_eq!(empty.kind, "syscalls:sys_enter_sched_yield");
        assert_eq!(empty.fields, "");
        assert_eq!(timestamp("123.000000019"), 123000000019);
        assert_eq!(timestamp("1.25"), 1250000000);
        let e=parse("  42/43 [001] 1.000000001: sched:sched_switch: prev_pid=43 prev_state=R+ ==> next_pid=0").unwrap();
        assert_eq!((e.pid, e.tid, e.cpu, e.ns), (42, 43, 1, 1000000001));
        assert_eq!(field(&e.fields, "prev_state="), "R+");
        assert!(parse("header").is_none());
    }
    #[test]
    fn perf_pretty_switch_has_numeric_ids_and_preemption() {
        assert_eq!(
            switch("a task:name:43 [120] R+ ==> swapper/0:0 [120]"),
            (43, 0, "R+".into())
        );
        assert_eq!(switch("a:0 [120] S ==> task:99 [100]"), (0, 99, "S".into()));
    }
    #[test]
    fn disjoint_clipping() {
        assert_eq!(overlap(8, 32, &[(10, 20), (30, 40)]), 12);
        assert_eq!(overlap(15, 15, &[(10, 20)]), 0);
        assert_eq!(overlap(0, 100, &[(10, 20), (30, 40)]), 20);
    }
    fn e(t: u64, k: &str, f: &str) -> Event {
        Event {
            ns: t,
            pid: 42,
            tid: 43,
            cpu: 0,
            kind: k.into(),
            fields: f.into(),
        }
    }
    #[test]
    fn yield_off_cpu_and_capacity() {
        let mut es = vec![
            e(11, "syscalls:sys_enter_sched_yield", ""),
            e(
                12,
                "sched:sched_switch",
                "prev_pid=43 prev_state=R ==> next_pid=0",
            ),
            e(
                17,
                "sched:sched_switch",
                "prev_pid=0 prev_state=R ==> next_pid=43",
            ),
            e(18, "syscalls:sys_exit_sched_yield", ""),
        ];
        let v = inspect(&mut es, &[(10, 20)], 42, 1);
        assert_eq!(v["occupancy_ns"]["sdk"], 5);
        assert_eq!(v["occupancy_ns"]["idle"], 5);
        assert_eq!(v["sdk_off_cpu_ns_summed_over_threads"]["inside_yield"], 5);
        assert_eq!(v["syscall_elapsed_ns_summed_over_threads"]["yield"], 7);
        assert_eq!(v["capacity_conserved"], true);
    }
    #[test]
    fn missing_cpu_and_right_censor() {
        let mut es = vec![
            e(11, "syscalls:sys_enter_futex", ""),
            e(
                12,
                "sched:sched_switch",
                "prev_pid=43 prev_state=S ==> next_pid=99",
            ),
        ];
        let v = inspect(&mut es, &[(10, 20)], 42, 2);
        assert_eq!(v["occupancy_ns"]["unknown"], 10);
        assert_eq!(v["occupancy_ns"]["other"], 8);
        assert_eq!(v["censored"]["syscall_right_boundary"], 1);
        assert_eq!(v["sdk_off_cpu_ns_summed_over_threads"]["inside_futex"], 8);
    }
    #[test]
    fn continuity_is_detected() {
        let mut es = vec![
            e(
                11,
                "sched:sched_switch",
                "prev_pid=43 prev_state=R+ ==> next_pid=99",
            ),
            e(
                13,
                "sched:sched_switch",
                "prev_pid=100 prev_state=R ==> next_pid=43",
            ),
        ];
        let v = inspect(&mut es, &[(10, 20)], 42, 1);
        assert_eq!(v["switch_continuity_errors"], 1);
        assert_eq!(v["sdk_off_cpu_ns_summed_over_threads"]["preempted_R+"], 2);
    }
}
