use serde_json::Value;
use std::{fmt::Write, fs};
const R: &str = "/home/ubuntu/src/zopro-perf-work/sdk-control-copies";
fn read(p: &str) -> Value {
    serde_json::from_slice(&fs::read(format!("{R}/{p}")).unwrap()).unwrap()
}
fn n(v: &Value, k: &str) -> f64 {
    v[k].as_f64().unwrap()
}
fn main() {
    let a = read("analysis.json");
    let cases = read("cases.json");
    let mut out = String::from("# Accepted SDK executable backing diagnostic\n\n");
    out.push_str("**The large same-byte component effect does not reproduce as a large full-proof difference in this run.** Primary 1×2 takes 8.385567 ms through the accepted executable's original inode and 8.452564 / 8.415173 ms through two fixed, byte-identical fresh files. All twenty paired copy/original intervals span parity. All 28,044 SDK requests and 531 independent Gnark exports pass. Production remains unchanged.\n\n");
    out.push_str("All copies and outcomes are retained. This is a diagnostic of the preceding component-level backing/frontend observation; it introduces no algorithm or executable-copy selection policy. The observations do not establish exact equivalence, a universal absence of backing effects, or a physical-page/cache-color/kernel cause.\n\n");
    out.push_str("## Verified request latency\n\nRyzen 9950X, 32 logical CPUs / 16 physical cores. Fifteen fixed rotated/reversed process blocks, three executables per cell. Primary has 151 timed requests per process; other shapes have 31. Three primes precede each process. Witness solving, fresh randomized proving, SDK verification, and serialization are timed; loading is excluded. CPU time includes coordination. RSS includes loading and priming. Values below are medians of block medians; they are warm single-request latency, not parallel proof throughput.\n\n");
    out.push_str("| Shape/workers | Executable | Median ms | CPU ms/request | Median peak MiB | Max peak MiB |\n|---|---|---:|---:|---:|---:|\n");
    for (k, v) in a["cells"].as_object().unwrap() {
        let (case, var) = k.rsplit_once('/').unwrap();
        writeln!(
            out,
            "| {case} | {var} | {:.6} | {:.3} | {:.2} | {:.2} |",
            n(v, "median_ms"),
            n(v, "cpu_ms"),
            n(v, "median_RSS_MiB"),
            n(v, "max_RSS_MiB")
        )
        .unwrap();
    }
    out.push_str("\n## Paired fresh/original ratios\n\nRatios below one favor the fresh file. Bounds use ranks 4..12 of 15 paired process blocks, nominal 96.484375% under independent exchangeable blocks; there is no familywise guarantee. No block is excluded, extended, or repeated after success. These are descriptive comparisons between copies of one algorithm, not a passed source-optimization gate.\n\n| Cell/copy/original | Paired median | Lower | Upper | Wins |\n|---|---:|---:|---:|---:|\n");
    for (k, v) in a["pairs"].as_object().unwrap() {
        writeln!(
            out,
            "| {k} | {:.6} | {:.6} | {:.6} | {}/15 |",
            n(v, "median"),
            n(v, "low"),
            n(v, "high"),
            v["wins"]
        )
        .unwrap();
    }
    out.push_str("\n## Frontend counters\n\nSeparate counter processes follow all latency captures: three blocks, primary at 16/32 workers and merge-54x1 at 32, all three executables, two four-event grouped userland panels, 31 requests after three primes. One shared FIFO pair gates the measured request loop. All counted events must run at least 99.9%; complete percentages and all observations are retained in analysis.json. The table reports median per-request counts. Cycles/instructions come from the fetch panel; frontend/L1D from the stall panel. Panel observations are separate runs, so no synthetic combined cycle decomposition is implied. Counter-run wall times are not pooled into latency results.\n\n| Cell/executable | Cycles M | Instructions M | L1I misses M | iTLB misses | Frontend stall M | L1D misses M |\n|---|---:|---:|---:|---:|---:|---:|\n");
    for (k, v) in a["counters"]
        .as_object()
        .unwrap()
        .iter()
        .filter(|(k, _)| k.ends_with("/fetch"))
    {
        let key = k.strip_suffix("/fetch").unwrap();
        let f = &v["median_events_per_request"];
        let s = &a["counters"][format!("{key}/stall")]["median_events_per_request"];
        writeln!(
            out,
            "| {key} | {:.3} | {:.3} | {:.6} | {:.1} | {:.3} | {:.3} |",
            n(f, "cycles") / 1e6,
            n(f, "instructions") / 1e6,
            n(f, "L1-icache-load-misses") / 1e6,
            n(f, "iTLB-load-misses"),
            n(s, "stalled-cycles-frontend") / 1e6,
            n(s, "L1-dcache-load-misses") / 1e6
        )
        .unwrap();
    }
    out.push_str("\n## Supplied M5 CPU and Metal references\n\nThese are historical cross-machine references, not a same-machine or newly remeasured M5 comparison. All eight reference shapes are measured anew here. Every executable is shown to avoid selecting a favorable copy. Merge-54x1 has no supplied reference and is included in the latency table only.\n\n| Shape | Executable | Local ms | M5 CPU ms | M5 Metal ms | CPU/local | Metal/local |\n|---|---|---:|---:|---:|---:|---:|\n");
    for c in cases.as_array().unwrap() {
        if c["reference"].is_null() {
            continue;
        }
        let shape = c["shape"].as_str().unwrap();
        let cpu = n(&c["reference"], "cpu_ms");
        let metal = n(&c["reference"], "metal_ms");
        for v in ["original-x", "copy-001-x", "copy-002-x"] {
            let ms = n(&a["cells"][format!("{shape}-w32/{v}")], "median_ms");
            writeln!(
                out,
                "| {shape} | {v} | {ms:.6} | {cpu:.0} | {metal:.0} | {:.3}× | {:.3}× |",
                cpu / ms,
                metal / ms
            )
            .unwrap();
        }
    }
    writeln!(out,"\nThe original CPU10x objective remains **active and unmet**: {}/8 rows reach the target using the original accepted executable in this capture. Primary requires at most 3.7 ms. No component-only or favorable-copy result changes completion requirements. The prior accepted FFT engineering exception and its false full-performance/qualified-scope gates remain unchanged.",a["original8_CPU10x_rows"]).unwrap();
    out.push_str("\n## Provenance and verification\n\nThe accepted executable SHA256 is `14da8231f5797db25d3f45e3570a73c38141bfb957295d2548b0e457676e612a`. `original-x` is a hardlink to its original inode. `copy-001-x` and `copy-002-x` are explicit read/write new files, not reflinks or hardlinks. All names are ten bytes in one directory. Identities and full contents are frozen before any proof capture. Complete native library receipts are retained; no native library or SDK is rebuilt. The offline raw/Euler converters link the accepted native libraries; the study/report helpers are Rust.\n\nThe old temporary all-shape images were absent. Nine CDN key artifacts are checked against the repository lockfile before parsing; regenerated raw and Euler images all match the historical production hashes, including the surviving primary. Preparation completes before proof capture. Existing accepted cache files are frozen after preflight. There is no new persistent key table, point recoding cache, expanded execution plan, or runtime memory-budget policy. The 172 accepted source files remain byte-exact, and prior unchanged-executable native/portable qualification is inherited rather than counted as new tests.\n\n27 preflights, 450 latency processes, and 54 counter processes complete. All **28,044 complete SDK requests verify internally**. All **531 fresh 164-byte proof exports independently pass Gnark**, with historical expected image/public binding, member hashes, and uniqueness against 13,181 prior proof identities. The independent ledger is **13,712**. The 1,062 proof/public members are not counted as 1,062 proofs; three copied binaries are not three algorithm implementations. All independent verification runs after capture.\n\nNo owned build, helper, hash/archive job, or second benchmark overlaps proof capture. Unrelated host processes are left untouched; this is not an exclusive-host claim. Host snapshots and run receipts are retained. There is no new AWS/Aeglos, mobile/TEE, memory-pressure/swap, minimum-RAM, or 250/4999 qualification. Measured RSS is a process observation, not a minimum RAM bound. No global ASLR, kernel, or cache-policy setting changes.\n\n[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [file identities](artifacts/identities.json), [image restoration](artifacts/restore/result.json), [all capture and verifier receipts](artifacts/runs/result.json), [proof inventory](artifacts/proof-inventory.json). The binary/key/image payloads remain local with pinned hashes.\n");
    if let Ok(s) = fs::read_to_string(format!("{R}/interpretation.md")) {
        out.push_str("\n");
        out.push_str(&s);
    }
    fs::write(format!("{R}/report.md"), out).unwrap();
}
