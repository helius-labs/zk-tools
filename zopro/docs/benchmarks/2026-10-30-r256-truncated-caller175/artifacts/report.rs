use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fmt::Write,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/r256-truncated-caller175";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const D: &str =
    "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-30-r256-truncated-caller175";
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn num(v: &Value) -> f64 {
    v.as_f64().unwrap()
}
fn walk(p: &Path, v: &mut Vec<PathBuf>) {
    let mut paths: Vec<_> = fs::read_dir(p)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    paths.sort();
    for p in paths {
        if p.is_dir() {
            walk(&p, v)
        } else {
            v.push(p)
        }
    }
}
fn archive(files: &[PathBuf], base: &Path, path: &Path) -> usize {
    let mut map = BTreeMap::new();
    for p in files {
        let b = fs::read(p).unwrap();
        map.insert(
            p.strip_prefix(base).unwrap().display().to_string(),
            json!({"sha256":hash(p),"bytes":b}),
        );
    }
    let raw = serde_json::to_vec(&map).unwrap();
    let local = Path::new(R).join(format!(
        "archive-{}.json",
        path.file_stem().unwrap().to_str().unwrap()
    ));
    fs::write(&local, &raw).unwrap();
    let gz = Command::new("gzip")
        .arg("-n")
        .arg("-c")
        .arg(&local)
        .output()
        .unwrap();
    assert!(gz.status.success());
    fs::write(path, gz.stdout).unwrap();
    let dec = Command::new("gzip")
        .arg("-d")
        .arg("-c")
        .arg(path)
        .output()
        .unwrap();
    assert!(dec.status.success());
    assert_eq!(dec.stdout, raw);
    map.len()
}
fn main() {
    let r = Path::new(R);
    let z = Path::new(Z);
    let d = Path::new(D);
    let out = d.join("artifacts");
    fs::create_dir_all(&out).unwrap();
    let a = read(r.join("analysis.json"));
    assert_eq!(a["complete"], true);
    assert_eq!(a["production_changed"], false);
    assert_eq!(a["proof_ledger"], 25231);
    let pins = read(r.join("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    for f in [
        "analysis.json",
        "source-audit.json",
        "field.rs",
        "field-tests.rs",
        "inherited-helper.txt",
        "native.rs",
        "setup.rs",
        "bootstrap.rs",
        "probe.rs",
        "caller_probe.rs",
        "perf.rs",
        "make-capture.rs",
        "capture.rs",
        "report.rs",
        "design.md",
        "capture-design.md",
        "history-audit.md",
        "codegen.json",
        "contexts.json",
        "jobs.json",
        "pins.json",
        "build-pins.json",
        "build-plan.json",
        "run-plan.json",
    ] {
        fs::copy(r.join(f), out.join(f)).unwrap();
    }
    let mut receipts = vec![];
    for folder in ["native", "runs"] {
        walk(&r.join(folder), &mut receipts);
    }
    for f in [
        "native-attempt0.log",
        "native.log",
        "capture.log",
        "analysis.log",
    ] {
        if r.join(f).exists() {
            receipts.push(r.join(f));
        }
    }
    let receipt_count = archive(&receipts, r, &out.join("receipts.json.gz"));
    let mut source = vec![];
    walk(&r.join("source"), &mut source);
    let source_count = archive(&source, &r.join("source"), &out.join("source.json.gz"));
    assert_eq!(source_count, 177);
    let rows = Command::new("jq")
        .arg(".rows")
        .arg(z.join("docs/benchmarks/2026-10-30-current-bench-snapshot175/data.json"))
        .output()
        .unwrap();
    assert!(rows.status.success());
    fs::write(d.join("accepted-rows.json"), rows.stdout).unwrap();
    let mut md=format!("# R256-only truncated reduction in MSM batch callers\n\n30 October 2026. Complete actual-caller gate: **{}**; production is unchanged. This is a new representation-specific integration of the earlier checked arithmetic, not a new mathematical reduction or a reinterpretation of its failed generic gate. No full-proof speedup or adoption is established by these caller timings.\n\n",a["component_gate"]);
    md.push_str("The candidate dispatches only native eight-lane R256 products to truncated full-radix reduction. R260, four-lane, scalar and portable arithmetic retain accepted routines. The negative-inverse constant and product body match the [earlier checked implementation](../2026-10-24-truncated-redc-component/README.md) exactly. That generic trial improved R256 components but regressed R260 curve expressions and remains rejected. This independent trial measures complete G1 and G2 batch callers, including surrounding inversion, field packing and point finishing. There is no new heap buffer, point-key copy, persistent table or execution plan; the inverse constant is 40 bytes per field/radix. Fixed vector temporaries can change stack/register pressure.\n\n## Measurements\n\nRyzen9950X,16physical/32logicalCPUs. SoloCPU2,physical16CPUs0–15,logical32CPUs0–31. Thirty-one fixed rotated/reversed blocks compare exact frozen caller, fresh OFF and ON in all18contexts. Each worker executes8192changing batches after3checked1024-call primes. Common wave wall time includes the slowest worker/barrier tail. Loading/fixture construction/output checks are outside timers. Full generator scalar-sum checks, digests, capacities and zero timed allocations are required for every process. These synthetic valid-curve caller fixtures are not real SDK proof distributions. No owned build/helper/hash/archive/analyzer/other workload overlaps performance captures.\n\nTimes are microseconds per complete batch wave; paired ratios need not equal ratios of displayed marginal medians.\n\n|Caller/context|Frozen µs|OFF µs|ON µs|Paired ON/OFF|Bounds|Wins/31|\n|---|---:|---:|---:|---:|---|---:|\n");
    let contexts = read(r.join("contexts.json"));
    for c in contexts.as_array().unwrap() {
        let name = c["name"].as_str().unwrap();
        let p = &a["pairs"][format!("{name}/on/off")];
        writeln!(
            md,
            "|{name}|{:.6}|{:.6}|{:.6}|{:.9}|[{:.9}, {:.9}]|{}|",
            num(&a["cells"][format!("{name}/frozen")]["median_wave_ns"]) / 1000.,
            num(&a["cells"][format!("{name}/off")]["median_wave_ns"]) / 1000.,
            num(&a["cells"][format!("{name}/on")]["median_wave_ns"]) / 1000.,
            num(&p["median"]),
            num(&p["low"]),
            num(&p["high"]),
            p["wins"]
        )
        .unwrap();
    }
    md.push_str("\nAll18 fresh/frozen calibration intervals must fit factor1.03; every ON/control nonregression upper bound must be≤1.03. All18 and logical32-six equal-log aggregates must each have upper<1 and≥21/31wins against both controls. Bounds use ranks10–22/31, nominal97.055% individual coverage conditional on independent/exchangeable blocks, without a familywise guarantee. There is no gain floor, exclusion, sample extension, changed threshold or subset promotion.\n\n|Aggregate|Ratio|Bounds|Wins/31|\n|---|---:|---|---:|\n");
    for (k, v) in a["aggregates"].as_object().unwrap() {
        writeln!(
            md,
            "|{k}|{:.9}|[{:.9}, {:.9}]|{}|",
            num(&v["median"]),
            num(&v["low"]),
            num(&v["high"]),
            v["wins"]
        )
        .unwrap();
    }
    writeln!(md,"\nFailed checks: `{}`. All observations, positive/negative estimates and control comparisons are retained in [analysis](artifacts/analysis.json). A pass selects a separate full-SDK trial; a failure closes this integration without adoption.\n",a["failed_checks"]).unwrap();
    md.push_str("## Correctness, profiles and memory\n\nFresh OFF passes33field and128core tests; ON passes35field and128core tests, unfiltered. The two new tests cover both primes and inclusive2p/lazy/radix boundaries in eight-step ordinary/fused/difference chains, comparing normalized lazy digits exactly with accepted CIOS and checking canonical exports/ranges. Test-only dispatch counters require8192R256 selections and zero R260 selections; the instrumentation is absent from library builds. Existing independent Arkworks field tests run in both variants. The old108800-case integer model and coefficient argument are inherited for the unchanged helper, not newly executed cases. No full-proof, portable or pressure qualification follows yet.\n\nFrozen caller source reverses exactly to all175 accepted files after removing diagnostic exports; its exact coherent libraries and executable are reused without rebuilding or rerunning its tests. Fresh snapshot has177files:172originals exact,three reversible parent edits andtwo children. This prototype uses an appended private include; template/generator integration remains necessary before adoption.\n\nFour fixed16Mcycles:u profiles follow all latency: borrowedG1/200 and borrowedG2/200,logical32,OFF/ON,262144batches/worker. FIFO acknowledgments exclude primes and output checks. Captures precede ownership/decoding. At least1000samples,at most5%unknownperiod,one sampledPID andzeroLOST are required. The period/time/CPU fields are recorded; sampled symbol shares and static code counts do not establish intrinsic instruction latency, exclusive costs or removable proof wall time. Full symbol periods remain in analysis, and raw profiles stay local with hashes.\n\nThe candidate keeps heap capacities; observed lifetime RSS/capacities in raw rows are not minimum RAM, compiler-stack bounds or memory-pressure guarantees. Frame/register effects remain part of measured caller cost. No per-key lookup expansion is introduced.\n\nThe native workflow initially stopped after detecting a temporary edit to its pinned design.md while a separate profile requirement was being drafted. The original protocol bytes were restored, additions moved to capture-design.md before capture, and the unchanged native plan resumed with successful field/core/library steps reused. Both attempts and the failure are retained. No successful suite, link or timing is rerun to replace an outcome.\n\n## Accepted CPU and Metal comparison\n\nThe inherited accepted table is unchanged. M5 numbers are supplied historical references, not new matched-hardware measurements. Accepted local1x2RSS remains134MiB; its physical16 calibration engineering exception remains explicit in the [complete snapshot](../2026-10-30-current-bench-snapshot175/README.md).\n\n|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× target ms|\n|---|---:|---:|---:|---:|---:|---:|\n");
    let accepted = read(d.join("accepted-rows.json"));
    for row in accepted.as_array().unwrap() {
        let v = &row["M5_reference_and_local_ratios"];
        let shape = row["shape"].as_str().unwrap();
        if v.is_null() {
            writeln!(
                md,
                "|{shape}|{:.6}|—|—|—|—|—|",
                num(&row["local_accepted"]["ms"])
            )
            .unwrap();
        } else {
            assert_eq!(v["goal10x"], false);
            writeln!(
                md,
                "|{shape}|{:.6}|{:.0}|{:.3}×|{:.0}|{:.3}×|{:.1}|",
                num(&row["local_accepted"]["ms"]),
                num(&v["cpu_reference_ms"]),
                num(&v["cpu_speedup"]),
                num(&v["metal_reference_ms"]),
                num(&v["metal_speedup"]),
                num(&v["required_ms_for_CPU10x"])
            )
            .unwrap();
        }
    }
    md.push_str("\nAll eight CPU10× targets remain unmet; no M5 reference exists for54→1. This caller trial adds zero SDK requests or Gnark exports; distinct-proof ledger25231unchanged. LatestAWS primary remains11.915591ms/w32 Zopro versus7.959184ms/w96 AeglosSorted; no new AWS/Aeglos/mobile/ARM/TEE/huge-circuit/swap/pressure/concurrent-throughput result is claimed.\n\n[Protocol](artifacts/design.md), [capture declaration](artifacts/capture-design.md), [history audit](artifacts/history-audit.md), [source audit](artifacts/source-audit.json), [all observations/profiles](artifacts/analysis.json), [source archive](artifacts/source.json.gz), [workflow receipts](artifacts/receipts.json.gz), [pins](artifacts/pins.json), [review](artifacts/publication-review.json), [checksums](SHA256SUMS).\n");
    fs::write(d.join("README.md"), md).unwrap();
    fs::write(out.join("publication-review.json"),serde_json::to_vec_pretty(&json!({"review_passed":true,"pins_verified":pins.as_object().unwrap().len(),"source_members":source_count,"receipt_members":receipt_count,"archive_roundtrips_exact":true,"production_source_files_unchanged":175,"SDK_requests":0,"new_Gnark_exports":0,"proof_ledger":25231})).unwrap()).unwrap();
    let mut files = vec![d.join("README.md"), d.join("accepted-rows.json")];
    walk(&out, &mut files);
    files.sort();
    let sums: String = files
        .iter()
        .map(|p| format!("{}  {}\n", hash(p), p.strip_prefix(d).unwrap().display()))
        .collect();
    fs::write(d.join("SHA256SUMS"), sums).unwrap();
    println!(
        "Published caller study: gate {},{}pins,{}source/{}receipt members; no proof requests",
        a["component_gate"],
        pins.as_object().unwrap().len(),
        source_count,
        receipt_count
    );
}
