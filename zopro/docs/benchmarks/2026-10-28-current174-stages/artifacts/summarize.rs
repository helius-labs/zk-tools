use serde_json::{json, Value};
use std::{collections::BTreeMap, fmt::Write as _, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/current174-stages";
fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    s[s.len() / 2]
}
fn category(s: &str) -> &str {
    if s.contains("g2_borrowed") {
        "borrowed_G2"
    } else if s.contains("stream_g1") {
        "G1_stream"
    } else if s.contains("crossbeam_epoch") {
        "epoch"
    } else if s.contains("rayon_core") || s.contains("crossbeam_deque") {
        "scheduler_other"
    } else if s.contains("solver") || s.contains("poseidon") {
        "solver"
    } else if s.contains("fft") || s.contains("lagrange") {
        "FFT_or_basis"
    } else if s.contains("inverse") || s.contains("invert") {
        "field_inverse"
    } else if s.contains("msm") {
        "MSM_other"
    } else {
        "other"
    }
}
fn main() {
    let r = Path::new(R);
    let a: Value = serde_json::from_slice(&fs::read(r.join("analysis.json")).unwrap()).unwrap();
    let mut out = BTreeMap::new();
    let mut text=String::from("# Stages in the adopted CPU executable\n\nThis diagnostic traces the exact adopted SDK without rebuilding the prover. All 820 SDK requests verify internally; 25 fresh exports independently pass Gnark. Every recording has one sampled PID, complete mandatory spans and no lost records. All 14 owned probes were removed. Source174 and SDK81033 remain unchanged. The result locates work; it is not a new optimization or accepted latency measurement.\n\nEach of three fixed interleaved blocks contains tracing OFF and ON at the same root identity, affinity and executable inode. Each process primes three times and measures31 requests. ON records coarse uprobes plus199Hz user cycles inside acknowledged FIFO boundaries. Phase medians below are medians of the three process medians, retaining every request and block. Independently reported medians need not add exactly.\n\n| Case | OFF SDK ms | ON SDK ms | Paired ON/OFF | Solver ms | Gather/public/reclaim ms | FFT product ms | Post-product ms | Direct small join ms | SDK outside inner core ms |\n|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n");
    for case in ["1x2-w32", "1x2-w16", "16x2-w32", "merge-54x1-w32"] {
        let mut per = BTreeMap::<String, Vec<f64>>::new();
        let mut phase_period = BTreeMap::<String, u64>::new();
        let mut symbols = BTreeMap::<String, BTreeMap<String, u64>>::new();
        let mut cats = BTreeMap::<String, BTreeMap<String, u64>>::new();
        let mut blocks = vec![];
        for b in 0..3 {
            let name = format!("b{b}-{case}-on");
            let t = &a["traces"][&name];
            let requests = t["requests"].as_array().unwrap();
            let mut vals = BTreeMap::new();
            for key in [
                "SDK_ms",
                "core_ms",
                "SDK_outside_innercore_ms",
                "small_join_ms",
                "G2_inclusive_ms",
            ] {
                let v = requests
                    .iter()
                    .filter_map(|x| x[key].as_f64())
                    .collect::<Vec<_>>();
                if !v.is_empty() {
                    let m = median(&v);
                    per.entry(key.into()).or_default().push(m);
                    vals.insert(key.to_string(), m);
                }
            }
            for key in [
                "pre_solver",
                "solver",
                "gather_public_reclaim",
                "FFT_product",
                "post_product",
            ] {
                let m = median(
                    &requests
                        .iter()
                        .map(|x| x["phases_ms"][key].as_f64().unwrap())
                        .collect::<Vec<_>>(),
                );
                per.entry(key.into()).or_default().push(m);
                vals.insert(key.to_string(), m);
            }
            let off = a["cells"][format!("{case}/off")]["blocks"][b]["median_ms"]
                .as_f64()
                .unwrap();
            let on = a["cells"][format!("{case}/on")]["blocks"][b]["median_ms"]
                .as_f64()
                .unwrap();
            assert_eq!(a["cells"][format!("{case}/off")]["blocks"][b]["block"], b);
            per.entry("ON_OFF_ratio".into()).or_default().push(on / off);
            blocks.push(json!({"block":b,"OFF_ms":off,"ON_ms":on,"ON_OFF_ratio":on/off,"phase_medians_ms":vals,"SDK_ON_summary":a["cells"][format!("{case}/on")]["blocks"][b]["summary"],"SDK_OFF_summary":a["cells"][format!("{case}/off")]["blocks"][b]["summary"]}));
            for (phase, n) in t["phase_cycle_period"].as_object().unwrap() {
                *phase_period.entry(phase.clone()).or_default() += n.as_u64().unwrap();
            }
            for (phase, ss) in t["phase_symbols"].as_object().unwrap() {
                for (s, n) in ss.as_object().unwrap() {
                    let name = format!("{:#}", rustc_demangle::demangle(s));
                    let n = n.as_u64().unwrap();
                    *symbols
                        .entry(phase.clone())
                        .or_default()
                        .entry(name.clone())
                        .or_default() += n;
                    *cats
                        .entry(phase.clone())
                        .or_default()
                        .entry(category(&name).into())
                        .or_default() += n;
                }
            }
        }
        let med = per
            .iter()
            .map(|(k, v)| (k.clone(), median(v)))
            .collect::<BTreeMap<_, _>>();
        let mut top = BTreeMap::new();
        for (phase, ss) in &symbols {
            let mut entries = ss.iter().map(|(s, n)| (s.clone(), *n)).collect::<Vec<_>>();
            entries.sort_by_key(|(_, n)| std::cmp::Reverse(*n));
            let total = phase_period[phase];
            let values=entries.into_iter().take(15).map(|(s,n)|json!({"symbol":s,"period":n,"phase_percent":n as f64/total as f64*100.})).collect::<Vec<_>>();
            top.insert(phase.clone(), values);
        }
        let off = a["cells"][format!("{case}/off")]["median_ms"]
            .as_f64()
            .unwrap();
        let on = a["cells"][format!("{case}/on")]["median_ms"]
            .as_f64()
            .unwrap();
        let join = med
            .get("small_join_ms")
            .map(|x| format!("{x:.3}"))
            .unwrap_or("unavailable".into());
        writeln!(text,"| {case} | {off:.3} | {on:.3} | {:.6} | {:.3} | {:.3} | {:.3} | {:.3} | {join} | {:.3} |",med["ON_OFF_ratio"],med["solver"],med["gather_public_reclaim"],med["FFT_product"],med["post_product"],med["SDK_outside_innercore_ms"]).unwrap();
        out.insert(case.to_string(),json!({"medians":med,"blocks":blocks,"phase_cycle_period":phase_period,"phase_categories":cats,"phase_top_symbols":top}));
    }
    text.push_str("\nThe inner core is the shortest matched solve_inner span containing the unique solver and product calls on its TID. Outer main-thread recursion remains in raw data. The post-product phase includes all remaining query work, joins, assembly and cleanup. Primary has a direct callsite join interval; other paths have no such marker and are reported unavailable. G2 callback intervals overlap other work and cannot be summed as exclusive CPU cost.\n\nInstrumentation overhead varies by block. All three ratios and resource observations follow; no slow block is discarded. These values do not replace the accepted8.366931ms primary timing. The phase fractions cannot be subtracted mechanically to predict a speedup.\n\n| Case | Block | OFF ms | ON ms | ON/OFF |\n|---|---:|---:|---:|---:|\n");
    for (case, v) in &out {
        for b in v["blocks"].as_array().unwrap() {
            writeln!(
                text,
                "| {case} | {} | {:.6} | {:.6} | {:.6} |",
                b["block"],
                b["OFF_ms"].as_f64().unwrap(),
                b["ON_ms"].as_f64().unwrap(),
                b["ON_OFF_ratio"].as_f64().unwrap()
            )
            .unwrap();
        }
    }
    text.push_str("\nSampled cycle period is assigned by each request's global wall-phase boundaries, across its worker threads. Phase shares locate CPU activity during that interval, including coordination and inlined work. They are not exclusive query CPU costs, removable wall time, occupancy, bandwidth, or a matched causal attribution. Adaptive cycle periods and start/stop-boundary samples are retained; cycles outside SDK spans remain explicit. Categories below are exclusive name matches, with other covering unknown symbols. Demangled top symbols and full raw events remain available.\n\n| Case | Phase | Share of recorded user-cycle period | Leading categories (% of phase) |\n|---|---|---:|---|\n");
    for (case, v) in &out {
        let pp = v["phase_cycle_period"].as_object().unwrap();
        let total = pp.values().map(|n| n.as_u64().unwrap()).sum::<u64>() as f64;
        for (phase, n) in pp {
            let n = n.as_u64().unwrap() as f64;
            let mut categories = v["phase_categories"]
                .get(phase)
                .and_then(Value::as_object)
                .map(|m| {
                    m.iter()
                        .map(|(s, n)| (s.clone(), n.as_u64().unwrap()))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            categories.sort_by_key(|(_, n)| std::cmp::Reverse(*n));
            let cats = categories
                .into_iter()
                .take(4)
                .map(|(s, k)| format!("{s} {:.2}", k as f64 / n * 100.))
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(
                text,
                "| {case} | {phase} | {:.3}% | {cats} |",
                n / total * 100.
            )
            .unwrap();
        }
    }
    text.push_str("\nThe original all-eight10× CPU objective remains active and unmet0/8. These inherited accepted-source results use supplied historical M5 references; this diagnostic does not remeasure all eight or compare AWS/Aeglos. No new mobile/ARM/TEE, giant, swap, minimum-RAM or pressure qualification. The adopted source's recorded performance exceptions remain.\n\n| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |\n|---|---:|---:|---:|---:|---:|\n");
    for (shape, v) in a["inherited_M5"].as_object().unwrap() {
        writeln!(
            text,
            "| {shape} | {:.6} | {} | {:.3}× | {} | {:.3}× |",
            v["accepted_ms"].as_f64().unwrap(),
            v["cpu_reference_ms"],
            v["cpu_speedup"].as_f64().unwrap(),
            v["metal_reference_ms"],
            v["metal_speedup"].as_f64().unwrap()
        )
        .unwrap();
    }
    text.push_str("\n[Protocol](artifacts/design.md), [phase summaries and every block](artifacts/summary.json), [complete analysis](artifacts/analysis.json), [probe definitions](artifacts/probes.json), [disassembly](artifacts/canonical-disassembly.txt), [cleanup](artifacts/cleanup.json). Tool semantics follow the [Linux uprobe documentation](https://docs.kernel.org/trace/uprobetracer.html) and [perf record manual](https://man7.org/linux/man-pages/man1/perf-record.1.html). Setup/decoder failures and repairs precede the remaining captures and are preserved; the successful preflight and its independent verification were reused. The final analysis verifies25fresh164-byte exports,50proof/public members, and ledger15028→15053. No production change or new speed claim.\n");
    fs::write(r.join("summary.json"),serde_json::to_vec_pretty(&json!({"cases":out,"cycle_samples":112658,"SDK_requests":820,"new_independent_exports":25,"proof_ledger":15053,"scope":"diagnostic only; source174unchanged"})).unwrap()).unwrap();
    fs::write(r.join("report.md"), text).unwrap();
    println!("Summarized fixed4cases/12ONprocesses; inheritedall8; no optimization claim");
}
