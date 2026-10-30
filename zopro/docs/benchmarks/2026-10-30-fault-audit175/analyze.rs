use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, path::Path, process::Command};
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const R: &str = "/home/ubuntu/src/zopro-perf-work/fault-audit175";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-30-fault-audit175";
const AWS: &str = "docs/benchmarks/2026-10-29-proof-pgo-c7a175/artifacts/analysis.json";
const CALLER: &str = "docs/benchmarks/2026-10-30-r256-truncated-caller175/artifacts/analysis.json";
const SNAPSHOT: &str = "docs/benchmarks/2026-10-30-current-bench-snapshot175/data.json";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn median(v: &[f64]) -> f64 {
    let mut v = v.to_vec();
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}
fn num(v: &Value) -> f64 {
    v.as_f64().unwrap()
}
fn approx(x: f64, y: f64) {
    assert!((x - y).abs() <= x.abs().max(y.abs()).max(1.) * 1e-12);
}
fn save(p: impl AsRef<Path>, v: Value) {
    fs::write(p, serde_json::to_vec_pretty(&v).unwrap()).unwrap();
}
fn main() {
    let z = Path::new(Z);
    let r = Path::new(R);
    let d = Path::new(D);
    fs::create_dir_all(d).unwrap();
    let aws = read(z.join(AWS));
    let caller = read(z.join(CALLER));
    let snapshot = read(z.join(SNAPSHOT));
    assert_eq!(aws["complete"], true);
    assert_eq!(caller["complete"], true);
    assert_eq!(caller["component_gate"], false);
    assert_eq!(snapshot["historical_distinct_Gnark_ledger"], 25231);
    let manifest = z.join(
        "docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json",
    );
    let m = read(&manifest);
    for (p, h) in m["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(z.join(p)), h.as_str().unwrap());
    }
    let mut awscells = BTreeMap::new();
    let mut awscount = 0;
    for (key, cell) in aws["cells"].as_object().unwrap() {
        if !key.ends_with("/plain") && !key.ends_with("/aeglos") {
            continue;
        }
        let blocks = cell["blocks"].as_array().unwrap();
        assert_eq!(blocks.len(), 15);
        let mut normalized = vec![];
        let mut latencies = vec![];
        let mut cpus = vec![];
        for (b, row) in blocks.iter().enumerate() {
            assert_eq!(row["block"], b);
            let s = &row["summary"];
            assert_eq!(s["priming"], 3);
            let rounds = s["rounds"].as_u64().unwrap();
            assert_eq!(rounds, if key.starts_with("1x2-") { 101 } else { 17 });
            assert!(s["minor_faults"].as_u64().is_some());
            assert!(s.get("major_faults").is_none());
            let user = num(&s["user_cpu_ms"]) / rounds as f64;
            let sys = num(&s["system_cpu_ms"]) / rounds as f64;
            approx(user + sys, num(&row["CPU_ms"]));
            normalized.push(json!({"block":b,"rounds":rounds,"minor_faults_per_request":num(&s["minor_faults"])/rounds as f64,"system_CPU_ms_per_request":sys,"user_CPU_ms_per_request":user,"voluntary_switches_per_request":num(&s["voluntary_switches"])/rounds as f64,"involuntary_switches_per_request":num(&s["involuntary_switches"])/rounds as f64}));
            latencies.push(num(&row["ms"]));
            cpus.push(user + sys);
            awscount += 1;
        }
        approx(median(&latencies), num(&cell["ms"]));
        approx(median(&cpus), num(&cell["CPU_ms"]));
        let mut result = json!({"request_ms":cell["ms"],"CPU_ms_per_request":cell["CPU_ms"],"max_RSS_MiB":cell["max_RSS_MiB"],"normalized_blocks":normalized,"major_faults":"not retained"});
        for field in [
            "minor_faults_per_request",
            "system_CPU_ms_per_request",
            "user_CPU_ms_per_request",
            "voluntary_switches_per_request",
            "involuntary_switches_per_request",
        ] {
            let values: Vec<_> = normalized.iter().map(|v| num(&v[field])).collect();
            result[format!("median_{field}")] = json!(median(&values));
        }
        awscells.insert(key.clone(), result);
    }
    assert_eq!(awscells.len(), 54);
    assert_eq!(awscount, 810);
    let mut callercells = BTreeMap::new();
    let mut callercount = 0;
    for (key, cell) in caller["cells"].as_object().unwrap() {
        let blocks = cell["blocks"].as_array().unwrap();
        assert_eq!(blocks.len(), 31);
        let mut faults = vec![];
        let mut sys = vec![];
        let mut calls = vec![];
        for (b, row) in blocks.iter().enumerate() {
            assert_eq!(row["block"], b);
            let v = &row["result"];
            assert_eq!(v["timed_allocations"], 0);
            assert_eq!(v["iterations_per_worker"], 8192);
            assert!(v["usage"].get("major_faults").is_none());
            let n = v["worker_count"].as_u64().unwrap() * 8192;
            assert_eq!(v["total_kernel_calls"], n);
            calls.push(n);
            faults.push(num(&v["usage"]["minor_faults"]));
            sys.push(num(&v["usage"]["system_cpu_ms"]));
            callercount += 1;
        }
        assert!(calls.iter().all(|&c| c == calls[0]));
        callercells.insert(key.clone(),json!({"kernel_calls_per_timed_wave":calls[0],"minor_faults_per_timed_wave":faults,"median_minor_faults_per_timed_wave":median(&faults),"maximum_minor_faults_per_timed_wave":faults.iter().cloned().fold(0.,f64::max),"median_system_CPU_ms_per_timed_wave":median(&sys),"major_faults":"not retained"}));
    }
    assert_eq!(callercount, 1674);
    assert_eq!(callercells.len(), 54);
    let historical = json!({"source":"../2026-10-23-aws-local-crosshost-aeglos/README.md","scope":"rounded values reported in a different-build historical study, not a new controlled comparison","Zopro_merge54_w96_minor_faults_per_request":296715,"Zopro_merge54_w96_kernel_CPU_ms_per_request":16604,"Zopro_merge54_w96_touch_diagnostic_unchanged_request_ms":421.539});
    let analysis = json!({"as_of":"2026-10-30","repository_evidence_commit":"72eaef9a0c3ca265357addfb1423b20a6f32ce19","AWS_cells":awscells,"caller_cells":callercells,"AWS_process_records_reviewed":awscount,"caller_process_records_reviewed":callercount,"historical_context":historical,"cgroup_events_whole_AWS_workflow":aws["cgroup_events_whole_workflow_deltas"],"minor_faults_are_not_disk_IO":true,"major_faults_retained":false,"timings_added":0,"proofs_added":0,"production_changed":false,"proof_ledger":25231,"goal_achieved":false});
    save(r.join("analysis.json"), analysis.clone());
    save(d.join("data.json"), analysis);
    let mut md=String::from("# Page-fault audit of saved threaded prover runs\n\n30 October 2026. Reanalysis of completed measurements through commit `72eaef9a0c3ca265357addfb1423b20a6f32ce19`; no new benchmark, proof or AWS execution.\n\n**The former 54→1 page-table fault problem is substantially reduced. The recent 32-worker arithmetic regression occurs with almost no faults in either variant.** Full-proof Zopro still records more minor faults than pinned Aeglos, so some VM work remains. Neither fault counts nor total kernel CPU alone identify its cost on the proof critical path.\n\n## Definitions and scope\n\nA minor fault is serviced without I/O; a major fault requires I/O. Minor faults are not evidence by themselves that RAM is exhausted. `RUSAGE_SELF` sums resource usage over the process's threads. [Linux getrusage documentation](https://man7.org/linux/man-pages/man2/getrusage.2.html).\n\nThe existing runners record `ru_minflt` but omit `ru_majflt`; major faults therefore remain **unreported**, not assumed zero. The SDK usage interval starts after three primes and excludes key loading. We divide each process's count by its timed request count, then take the median of those per-request averages across 15 processes. This is not a median of individually instrumented proof fault counts. CPU time is summed across threads; it is not wall latency. All comparisons below retain the pinned engine configurations and original measurement limits.\n\n## AWS primary 1×2\n\nc7a.24xlarge, 96 physical cores, no SMT; 101 timed requests per process after three primes. Accepted-source Zopro is labelled plain; the unadopted PGO candidate is excluded from this accepted-engine comparison. Aeglos uses the pinned Sorted engine.\n\n|Workers|Zopro faults/request|Aeglos faults/request|Zopro kernel CPU ms/request|Aeglos kernel CPU ms/request|Zopro request ms|Aeglos request ms|\n|---:|---:|---:|---:|---:|---:|---:|\n");
    for w in [32, 64, 96] {
        let a = &awscells[&format!("1x2-w{w}/plain")];
        let b = &awscells[&format!("1x2-w{w}/aeglos")];
        writeln!(
            md,
            "|{w}|{:.2}|{:.2}|{:.3}|{:.3}|{:.6}|{:.6}|",
            num(&a["median_minor_faults_per_request"]),
            num(&b["median_minor_faults_per_request"]),
            num(&a["median_system_CPU_ms_per_request"]),
            num(&b["median_system_CPU_ms_per_request"]),
            num(&a["request_ms"]),
            num(&b["request_ms"])
        )
        .unwrap();
    }
    md.push_str("\nZopro's primary minor-fault count rises modestly with workers while kernel CPU grows much more. Aeglos's faults also rise as its request latency falls. These observations do not make faults costless, but they show that fault counts alone cannot explain the different scaling. Kernel CPU includes work besides fault handling; this audit does not attribute its remainder to allocation, futexes or scheduling.\n\n## AWS merge 54→1\n\nSeventeen timed requests per process after three primes; the same normalization applies.\n\n|Workers|Zopro faults/request|Aeglos faults/request|Zopro kernel CPU ms/request|Aeglos kernel CPU ms/request|Zopro request ms|Aeglos request ms|\n|---:|---:|---:|---:|---:|---:|---:|\n");
    for w in [32, 64, 96] {
        let a = &awscells[&format!("merge-54x1-w{w}/plain")];
        let b = &awscells[&format!("merge-54x1-w{w}/aeglos")];
        writeln!(
            md,
            "|{w}|{:.2}|{:.2}|{:.3}|{:.3}|{:.6}|{:.6}|",
            num(&a["median_minor_faults_per_request"]),
            num(&b["median_minor_faults_per_request"]),
            num(&a["median_system_CPU_ms_per_request"]),
            num(&b["median_system_CPU_ms_per_request"]),
            num(&a["request_ms"]),
            num(&b["request_ms"])
        )
        .unwrap();
    }
    md.push_str("\nThe [October 23 study](../2026-10-23-aws-local-crosshost-aeglos/README.md) reported roughly 296,715 minor faults and 16,604 ms summed kernel CPU per 54→1 proof at 96 workers. Its kernel call-chain profile placed about 91% under anonymous-page handling and page-table locking. In the newer build the corresponding descriptive counts are roughly 23,952 faults and 350.5 ms kernel CPU, with 126.994 ms request latency. The old separate unchanged diagnostic measured 421.539 ms. These are different builds/runs; the full latency change cannot be attributed to fault reduction alone. No new kernel call-chain profile establishes the composition of the remaining 350.5 ms.\n\nAt 96 workers, Zopro still has about seven times Aeglos's minor-fault count, while proving 54→1 faster. The whole-workflow memory/OOM/swap-event deltas in the latest AWS record are zero. Those are workflow-wide observations, not per-request major-fault counts or a general memory-pressure qualification.\n\n## Preallocated batch callers\n\nThe latest local R256-only arithmetic trial reuses scratch after primes, reports zero timed allocations and changes no heap capacities. Its OFF and ON medians both show **two minor faults per complete timed wave** for every 32-worker caller. Each wave contains 8,192 calls per worker, or 262,144 batch calls in total. These are not proof requests.\n\n|32-worker caller|OFF median faults/wave|ON median faults/wave|OFF maximum|ON maximum|\n|---|---:|---:|---:|---:|\n");
    for kind in ["g1-owned", "g1-borrowed-signed", "g2-borrowed-signed"] {
        for n in [200, 352] {
            let a = &callercells[&format!("{kind}-{n}-logical32/off")];
            let b = &callercells[&format!("{kind}-{n}-logical32/on")];
            writeln!(
                md,
                "|{kind}/{n}|{:.0}|{:.0}|{:.0}|{:.0}|",
                num(&a["median_minor_faults_per_timed_wave"]),
                num(&b["median_minor_faults_per_timed_wave"]),
                num(&a["maximum_minor_faults_per_timed_wave"]),
                num(&b["maximum_minor_faults_per_timed_wave"])
            )
            .unwrap();
            assert_eq!(a["median_minor_faults_per_timed_wave"], 2.);
            assert_eq!(b["median_minor_faults_per_timed_wave"], 2.);
        }
    }
    md.push_str("\nAll six 32-worker timing estimates regressed in that arithmetic experiment despite these low fault counts. This provides no evidence that page faults explain that regression. It also does not establish allocation-free end-to-end proving: the full SDK has different allocation and synchronization lifetimes. [Complete caller experiment](../2026-10-30-r256-truncated-caller175/README.md).\n\n## Consequences for further optimization\n\nThe historical large-proof page-table contention should not be carried forward as the sole current bottleneck. The shared arithmetic still needs work that helps complete threaded callers. End-to-end VM and coordination costs remain separate targets; a new diagnostic should retain major faults and use phase/kernel evidence before attributing system CPU to fault handling. Any proposed workspace reuse or page-touch policy must preserve admission, release and peak-memory behavior, rather than retain unbounded state.\n\nThis audit reviews all 810 accepted-Zopro/Aeglos AWS process summaries across nine shapes and three worker counts, plus all 1,674 local caller timing records across 18 contexts and three variants. Count cardinality, priming/rounds, normalized CPU totals, process medians, zero timed allocations and all 175 production source hashes are checked. The machine-readable file includes every selected cell and normalized block; no successful test/capture/proof is repeated. Major counts cannot be reconstructed from these retained summaries.\n\nAccepted single-proof results remain 1×2 **8.022165 ms / 134 MiB** locally and 54→1 **162.250464 ms / 1,831.87 MiB**. The original eight CPU 10× targets remain unmet. All nine local/AWS rows and eight supplied M5 CPU/Metal comparisons are preserved in [accepted rows](accepted-rows.json) and the [complete benchmark snapshot](../2026-10-30-current-bench-snapshot175/README.md). No new performance, minimum-RAM, pressure/swap, mobile/ARM/TEE, huge-circuit or concurrency result is claimed. Distinct-proof ledger remains 25,231.\n\n[Data and normalization](data.json), [analysis source](analyze.rs), [input hashes](source-hashes.json), [checksums](SHA256SUMS), [original AWS benchmark](../2026-10-29-proof-pgo-c7a175/README.md).\n");
    fs::write(d.join("README.md"), md).unwrap();
    fs::copy(r.join("analyze.rs"), d.join("analyze.rs")).unwrap();
    let rows = Command::new("jq")
        .arg(".rows")
        .arg(z.join(SNAPSHOT))
        .output()
        .unwrap();
    assert!(rows.status.success());
    fs::write(d.join("accepted-rows.json"), rows.stdout).unwrap();
    let mut pins = BTreeMap::new();
    for p in [
        z.join(AWS),
        z.join(CALLER),
        z.join(SNAPSHOT),
        manifest,
        z.join("docs/benchmarks/2026-10-23-aws-local-crosshost-aeglos/README.md"),
        z.join("crates/zopro-bench/examples/support/perf.rs"),
        z.join("crates/zopro-bench/examples/cpu-sdk.rs"),
        r.join("analyze.rs"),
    ] {
        pins.insert(p.display().to_string(), hash(p));
    }
    save(d.join("source-hashes.json"), json!(pins));
    let sums: String = [
        "README.md",
        "data.json",
        "accepted-rows.json",
        "analyze.rs",
        "source-hashes.json",
    ]
    .iter()
    .map(|p| format!("{}  {p}\n", hash(d.join(p))))
    .collect();
    fs::write(d.join("SHA256SUMS"), sums).unwrap();
    println!("Reviewed {awscount} AWS and {callercount} caller summaries, 54 cells each, 175 production hashes; no new runtime execution");
}
