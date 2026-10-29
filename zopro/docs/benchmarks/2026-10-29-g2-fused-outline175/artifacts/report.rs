use serde_json::Value;
use std::{fmt::Write, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-fused-outline175";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-g2-fused-outline175";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn n(v: &Value) -> f64 {
    v.as_f64().unwrap()
}
fn main() {
    let r = Path::new(R);
    let a = read(r.join("analysis.json"));
    let ids = read(r.join("identities.json"));
    assert_eq!(a["SDK_requests"], 7902);
    assert_eq!(a["new_independent_exports"], 78);
    assert_eq!(a["independent_proof_count"], 17161);
    let p = &a["pairs"]["1x2-w32/ON/accepted"];
    let q = &a["pairs"]["1x2-w32/ON/OFF"];
    let mut s = String::from("# Selective fused-Fq outlining: primary SDK pilot\n\n");
    writeln!(s,"The declared pilot gate is **{}**. Primary at 32 workers measures **{:.6} ms ON versus {:.6} ms accepted and {:.6} ms fresh OFF**. Paired ON/accepted is {:.9}, bounds [{:.9}, {:.9}], {}/15 wins; ON/OFF is {:.9}, bounds [{:.9}, {:.9}], {}/15 wins. Production remains source175/SDK20fde; no candidate is adopted.\n",a["gate_passed"],n(&a["cells"]["1x2-w32/ON"]["median_ms"]),n(&a["cells"]["1x2-w32/accepted"]["median_ms"]),n(&a["cells"]["1x2-w32/OFF"]["median_ms"]),n(&p["median"]),n(&p["low"]),n(&p["high"]),p["wins"],n(&q["median"]),n(&q["low"]),n(&q["high"]),q["wins"]).unwrap();
    writeln!(s,"Failed checks: `{}`. Paired change versus accepted is {:+.3}% and versus OFF {:+.3}%; positive values here mean slower. All observations remain. A passing pilot permits separate full-scope qualification; a failed pilot closes this exact mechanism without an unchanged rerun.\n",a["failed_checks"],100.*(n(&p["median"])-1.),100.*(n(&q["median"])-1.)).unwrap();
    s.push_str("## Change and scope\n\nOnly native eight-lane Fq R256 fused products use a new non-inlined helper. The helper calls the exact original CIOS arithmetic body; other field modes retain the original path. The source template and its officially generated eight-lane file are the only changes among 175 files. All 173 other files remain exact. Reversing the template edit and running the unchanged official generator recovers all 175 accepted files. Core, G2 consumers, solver, key, scheduler and memory forecasting are unchanged.\n\nThis tests code footprint and call boundaries, without adding heap allocations, lookup tables, point copies, plans, pools, unsafe code or handwritten assembly. Rust calls can add stack traffic, copies and overhead. Smaller linked functions alone establish neither faster proofs nor lower process memory.\n\nBoth configurations freshly pass all 33 existing field tests, 124 core tests and 8 prover tests. Field tests include independent Arkworks comparisons and fused reduction boundaries; no new arithmetic cases were added. The official generator check also passes. These results do not qualify portable or giant-circuit behavior.\n\n## Linked code before timing\n\nThe linked audit verifies the candidate helper is the native eight-lane Fq/MIXED=true/FUSED=true specialization, and the G2 borrowed caller invokes it. The table describes selected linked functions, not whole-binary size or dynamic instruction counts.\n\n| Variant | G2 caller bytes | Helper bytes | Caller outline sites |\n|---|---:|---:|---:|\n");
    let code = read(r.join("code-audit.json"));
    for v in code["variants"].as_array().unwrap() {
        let caller = &v["G2_borrowed_callers"][0];
        let helper_bytes: u64 = v["outlined_helpers"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x["bytes"].as_u64().unwrap())
            .sum();
        writeln!(
            s,
            "| {} | {} | {} | {} |",
            v["variant"].as_str().unwrap(),
            caller["bytes"],
            helper_bytes,
            caller["outline_call_or_jump_sites"]
        )
        .unwrap();
    }
    s.push_str("\nThe candidate caller plus helper occupies 46,790 bytes versus 51,950 bytes for the original caller. This selected footprint difference does not establish instruction-cache pressure or explain a latency result.\n\n## Complete verified requests\n\nOn this Ryzen 9950X, every latency process uses 32 workers on CPUs 0–31, measures 151 requests after 3 primes, and exports a fresh proof for independent verification. Fifteen fixed rotated/reversed blocks compare accepted, fresh OFF and ON. All 9 shapes × 3 variants pass preflight Gnark checks before latency; all 45 latency captures precede their 45 Gnark checks. No owned build, helper compilation, hashing, archiving or second workload overlaps captures. The host is shared; exclusive access is not claimed.\n\n| Variant | Median latency ms | CPU ms/request | Median/max process RSS MiB |\n|---|---:|---:|---:|\n");
    for v in ["accepted", "OFF", "ON"] {
        let c = &a["cells"][format!("1x2-w32/{v}")];
        writeln!(
            s,
            "| {v} | {:.6} | {:.3} | {:.2}/{:.2} |",
            n(&c["median_ms"]),
            n(&c["CPU_ms"]),
            n(&c["median_RSS_MiB"]),
            n(&c["max_RSS_MiB"])
        )
        .unwrap();
    }
    s.push_str("\nProcess high-water RSS includes loading/primes; it is not minimum RAM. Displayed medians are not paired-ratio estimates.\n\n| Comparison | Paired ratio [bounds] | Wins | Paired saving µs | Check |\n|---|---|---:|---:|---|\n");
    for key in ["ON/accepted", "ON/OFF", "OFF/accepted"] {
        let full = format!("1x2-w32/{key}");
        let p = &a["pairs"][&full];
        writeln!(
            s,
            "| {key} | {:.9} [{:.9}, {:.9}] | {} | {:.3} | {} |",
            n(&p["median"]),
            n(&p["low"]),
            n(&p["high"]),
            p["wins"],
            n(&p["paired_saving_ms"]) * 1000.,
            if a["checks"][&full] == true {
                "pass"
            } else {
                "FAIL"
            }
        )
        .unwrap();
    }
    writeln!(s,"\nAccepted SHA256 `{}` and fresh OFF `{}` are {}. Pilot primary gains require upper paired ratios below1 and at least10/15wins against both controls; OFF/accepted calibration bounds must lie within [1/1.03,1.03]. Sorted paired-ratio ranks4–12 of15 give nominal96.484375% per-comparison coverage under independent exchangeable blocks, without a familywise guarantee. No minimum gain floor, exclusion, sample extension, changed threshold or fastest-layout selection is used.\n",ids["accepted"]["sha256"].as_str().unwrap(),ids["OFF"]["sha256"].as_str().unwrap(),if ids["accepted"]["sha256"]==ids["OFF"]["sha256"]{"byte-identical executables at different paths"}else{"distinct compiled executables"}).unwrap();
    s.push_str("## CPU profiles\n\nAfter latency verification, three FIFO-gated four-event stat runs (31requests) and three separate199Hz cycles:u profiles (251requests), after3primes each, precede their six Gnark checks and decoding. FIFO gates exclude loading/primes. Diagnostic times do not enter the latency gate.\n\n| Stat variant | Cycles/request, million | Instructions/request, million | Generic cache misses/request, million | Branch misses/request, million |\n|---|---:|---:|---:|---:|\n");
    for (k, v) in a["diagnostics"]["counters"].as_object().unwrap() {
        let e = &v["events"];
        writeln!(
            s,
            "| {k} | {:.3} | {:.3} | {:.3} | {:.3} |",
            n(&e["cycles:u"]["per_request"]) / 1e6,
            n(&e["instructions:u"]["per_request"]) / 1e6,
            n(&e["cache-misses:u"]["per_request"]) / 1e6,
            n(&e["branch-misses:u"]["per_request"]) / 1e6
        )
        .unwrap();
    }
    s.push_str("\n| Profile variant | Samples | Lost | Outlined helper matched samples | Matched cycle-period share |\n|---|---:|---:|---:|---:|\n");
    for (k, v) in a["diagnostics"]["profiles"].as_object().unwrap() {
        let matched: Vec<_> = v["symbols"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|s| {
                s["symbol"]
                    .as_str()
                    .unwrap()
                    .contains("product_cios_outlined")
            })
            .collect();
        let count: u64 = matched.iter().map(|s| s["samples"].as_u64().unwrap()).sum();
        let period: u64 = matched.iter().map(|s| s["period"].as_u64().unwrap()).sum();
        writeln!(
            s,
            "| {k} | {} | {} | {count} | {:.3}% |",
            v["samples"],
            v["lost"]["lost_records"].as_array().unwrap().len(),
            100. * period as f64 / n(&v["period"])
        )
        .unwrap();
    }
    s.push_str("\nAll stat events run at least99.9%; each profile contains one SDK PID and zero lost records. Name matches can miss inlined work; their period is not removable wall time or an isolated cost of the changed call boundary. These observations do not establish an exclusive cache/DRAM/scheduler cause.\n\n## CPU and Metal scope\n\nThe fresh candidate primary comparison follows; M5 values are supplied historical references, not new M5 captures.\n\n| Primary variant | Measured ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |\n|---|---:|---:|---:|---:|---:|\n");
    let refs = &a["new_reference_comparisons"]["1x2-w32"];
    for v in ["accepted", "OFF", "ON"] {
        let p = &refs["variants"][v];
        writeln!(
            s,
            "| {v} | {:.6} | {:.0} | {:.3}× | {:.0} | {:.3}× |",
            n(&p["ms"]),
            n(&refs["reference"]["cpu_ms"]),
            n(&p["CPU_speedup"]),
            n(&refs["reference"]["metal_ms"]),
            n(&p["Metal_speedup"])
        )
        .unwrap();
    }
    s.push_str("\nThe complete all-eight accepted source175 comparison is inherited below; it is not a candidate all-eight performance qualification or a new measurement of the other cases. All-nine preflights establish correctness, not their latency.\n\n| Shape | Accepted ms | M5 CPU ms | CPU/accepted | M5 Metal ms | Metal/accepted | CPU10× required ms |\n|---|---:|---:|---:|---:|---:|---:|\n");
    for shape in [
        "1x2",
        "2x2",
        "4x2",
        "8x2",
        "12x2",
        "16x2",
        "24x2",
        "merge-24x1",
    ] {
        let p = &a["inherited_accepted_M5"][shape];
        writeln!(
            s,
            "| {shape} | {:.6} | {:.0} | {:.3}× | {:.0} | {:.3}× | {:.1} |",
            n(&p["accepted_ms"]),
            n(&p["cpu_reference_ms"]),
            n(&p["cpu_speedup"]),
            n(&p["metal_reference_ms"]),
            n(&p["metal_speedup"]),
            n(&p["required_ms_for_CPU10x"])
        )
        .unwrap();
    }
    s.push_str("\nThe all-eight CPU10× goal remains **active and unmet**. Accepted175 reaches0/8; this candidate has only one fresh latency row and no all-eight qualification. Primary requires3.7ms. Accepted54→1 remains162.250464ms without a supplied M5 reference. The accepted physical16 baseline-calibration exception remains recorded; this pilot does not rerun or relabel it.\n\n## Verification and provenance\n\nAll165workflow steps complete. All **7,902 SDK requests** verify internally; all **78fresh164-byte exports** independently pass Gnark, exact image/public binding and freshness against17,083 prior identities. The ledger is **17,161**, with156proof/public members representing78proofs. Native libraries, shared proc macros, helper/driver, source175 and all image/cache pins close. Full source including non-Rust files, commands, observations, proofs and decoded profiles are archived. Three owned raw perf recordings and binary/rlib payloads stay local with hashes. No successful capture is repeated.\n\nNo new portable/ARM/mobile/TEE, physical16 latency, giant250/4999, concurrent-proof, swap, active-pressure, minimum-RAM or AWS/Aeglos result is claimed. [Accepted production](../2026-10-29-balanced-kh-adoption/README.md) remains unchanged. [Design](artifacts/design.md), [history audit](artifacts/history-audit.md), [analysis](artifacts/analysis.json), [build/tests](artifacts/native/result.json), [source audit](artifacts/source-audit.json), [proof inventory](artifacts/proof-inventory.json), [capture completion](artifacts/runs/result.json).\n");
    for (a, b) in [
        ("source175", "source 175"),
        ("SDK20fde", "SDK 20fde"),
        ("below1", "below 1"),
        ("least10/15wins", "least 10/15 wins"),
        ("ranks4–12 of15", "ranks 4–12 of 15"),
        ("nominal96.484375%", "nominal 96.484375%"),
        ("31requests", "31 requests"),
        ("separate199Hz", "separate 199 Hz"),
        ("251requests", "251 requests"),
        ("after3primes", "after 3 primes"),
        ("least99.9%", "least 99.9%"),
        ("CPU10×", "CPU 10×"),
        ("reaches0/8", "reaches 0/8"),
        ("requires3.7ms", "requires 3.7 ms"),
        (
            "Accepted54→1 remains162.250464ms",
            "Accepted 54→1 remains 162.250464 ms",
        ),
        ("physical16", "physical 16"),
        ("All165workflow", "All 165 workflow"),
        ("78fresh164-byte", "78 fresh 164-byte"),
        ("against17,083", "against 17,083"),
        (
            "with156proof/public members representing78proofs",
            "with 156 proof/public members representing 78 proofs",
        ),
        ("giant250/4999", "giant 250/4999"),
    ] {
        s = s.replace(a, b);
    }
    fs::write(Path::new(D).join("README.md"), s).unwrap();
    println!("Reported primarypilot with all8inheritedCPU/Metal;78Gnark;gate retained");
}
