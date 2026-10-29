use serde_json::Value;
use std::{fmt::Write, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/balanced-ordinary175";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-balanced-ordinary175";
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
    assert_eq!(a["independent_proof_count"], 17083);
    let p = &a["pairs"]["1x2-w32/ON/accepted"];
    let q = &a["pairs"]["1x2-w32/ON/OFF"];
    let mut s = String::from("# Balanced ordinary G1 windows: primary SDK pilot\n\n");
    writeln!(s,"The declared pilot gate is **{}**. Primary at 32 workers measures **{:.6} ms ON versus {:.6} ms accepted and {:.6} ms fresh OFF**. Paired ON/accepted is {:.9}, bounds [{:.9}, {:.9}], {}/15 wins; ON/OFF is {:.9}, bounds [{:.9}, {:.9}], {}/15 wins. Production remains source175/SDK20fde; no candidate is adopted.\n",a["gate_passed"],n(&a["cells"]["1x2-w32/ON"]["median_ms"]),n(&a["cells"]["1x2-w32/accepted"]["median_ms"]),n(&a["cells"]["1x2-w32/OFF"]["median_ms"]),n(&p["median"]),n(&p["low"]),n(&p["high"]),p["wins"],n(&q["median"]),n(&q["low"]),n(&q["high"]),q["wins"]).unwrap();
    writeln!(s,"Failed checks: `{}`. Paired change versus accepted is {:+.3}% and versus OFF {:+.3}%; positive values here mean slower. All observations remain. A passing pilot permits separate full-scope qualification; a failed pilot closes this exact mechanism without an unchanged rerun.\n",a["failed_checks"],100.*(n(&p["median"])-1.),100.*(n(&q["median"])-1.)).unwrap();
    s.push_str("## Change and scope\n\nThe real fused K/H caller hardwires width13. Extending its unused widths would not affect these proofs, so that initially suggested scope was rejected before building. Instead, this candidate changes only eligible ordinary borrowed G1 queries at width12. The accepted environment explicitly selects width12/parts1; supported runtime bounds remain 1024..65535 points, parts1..2, native IFMA, packed/direct and deferred-sign guards. The accepted fused13 child, field/G2/solver/key/scheduler code and other widths remain byte-exact.\n\nTwenty-two balanced signed spans cover255bits:9width11 and13width12, totaling35840 bucket slots versus21×2048 ordinary uniform buckets plus the indexed top. The signed reader uses the preceding bit as its carry, without a recoded digit array. Input points remain borrowed; pending/collision/exception kernels and equal-weight merging are unchanged. The last indexed digit becomes an ordinary balanced window. Existing batch sizes follow each span width.\n\nLower bucket capacity does not establish fewer total operations, faster proofs or lower process RSS: digit counts, collisions, batch inversions and top reduction differ. The existing conservative forecast charges22full2048 parents plus top scratch; new spans fit those parent/batch bounds. Span metadata is352bytes on64bit, with compiler frames additional. No new input-sized buffer, retained key table, pool or plan is introduced.\n\nThe private176-file snapshot copies175 accepted originals:174 remain byte-exact; reversing the same-line parent cfg hook/suffix and removing the child recovers175. Both native configurations pass unfiltered core suites (124OFF/126ON) and8prover/raw-cubic tests each. The two new tests check exact320bit reconstruction over4861valid scalars/106942digits, and independent generator/owned-control sums across workers1/4, parts1/2, lengths0/1/17/1024/21006/35973/65535 and six scalar families, with identities, repeated/opposite points and limb/sign boundaries. The unchanged17field source/manifest files inherit33field tests; no new field suite is claimed.\n\n## Complete verified requests\n\nRyzen9950X,32logical workers on CPUs0–31. Fifteen fixed rotated/reversed process blocks compare accepted, freshOFF and ON; each measures151 SDK requests after3primes. Witness solving, fresh randomized proving, SDK verification and serialization are timed; loading is excluded. All9shapes×3variants preflight and independently verify before latency. All45latency captures precede their45Gnark checks. No owned build/helper/hash/archive/second workload overlaps capture; no exclusive-host claim is made.\n\n| Variant | Median block ms | Aggregate CPU ms/request | Process peak RSS median/max MiB |\n|---|---:|---:|---:|\n");
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
    s.push_str("\n| Profile variant | Samples | Lost | Ordinary-balanced matched samples | Matched cycle-period share |\n|---|---:|---:|---:|---:|\n");
    for (k, v) in a["diagnostics"]["profiles"].as_object().unwrap() {
        let matched: Vec<_> = v["symbols"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|s| s["symbol"].as_str().unwrap().contains("ordinary_balanced"))
            .collect();
        let count: u64 = matched.iter().map(|s| s["samples"].as_u64().unwrap()).sum();
        let period: u64 = matched.iter().map(|s| s["period"].as_u64().unwrap()).sum();
        writeln!(
            s,
            "| {k} | {} | {} | {count} | {:.3}% |",
            v["samples"],
            v["lost"],
            100. * period as f64 / n(&v["period"])
        )
        .unwrap();
    }
    s.push_str("\nAll stat events run at least99.9%; each profile contains one SDK PID and zero lost records. Name matches can miss inlined work; their period is not removable wall time or an isolated cost of the changed reader. These observations do not establish an exclusive cache/DRAM/scheduler cause.\n\n## CPU and Metal scope\n\nThe fresh candidate primary comparison follows; M5 values are supplied historical references, not new M5 captures.\n\n| Primary variant | Measured ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |\n|---|---:|---:|---:|---:|---:|\n");
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
    s.push_str("\nThe all-eight CPU10× goal remains **active and unmet**. Accepted175 reaches0/8; this candidate has only one fresh latency row and no all-eight qualification. Primary requires3.7ms. Accepted54→1 remains162.250464ms without a supplied M5 reference. The accepted physical16 baseline-calibration exception remains recorded; this pilot does not rerun or relabel it.\n\n## Verification and provenance\n\nAll165workflow steps complete. All **7,902 SDK requests** verify internally; all **78fresh164-byte exports** independently pass Gnark, exact image/public binding and freshness against17,005 prior identities. The ledger is **17,083**, with156proof/public members representing78proofs. Native libraries, shared proc macros, helper/driver, source176 and all image/cache pins close. Full source including non-Rust files, commands, observations, proofs and decoded profiles are archived. Three owned raw perf recordings and binary/rlib payloads stay local with hashes. No successful capture is repeated.\n\nNo new portable/ARM/mobile/TEE, physical16 latency, giant250/4999, concurrent-proof, swap, active-pressure, minimum-RAM or AWS/Aeglos result is claimed. [Accepted production](../2026-10-29-balanced-kh-adoption/README.md) remains unchanged. [Design](artifacts/design.md), [history audit](artifacts/history-audit.md), [analysis](artifacts/analysis.json), [build/tests](artifacts/native/result.json), [source audit](artifacts/source-audit.json), [proof inventory](artifacts/proof-inventory.json), [capture completion](artifacts/runs/result.json).\n");
    for (a, b) in [
        ("source175", "source 175"),
        ("source176", "source 176"),
        ("SDK20fde", "SDK 20fde"),
        ("width13", "width 13"),
        ("width12", "width 12"),
        (
            "cover255bits:9width11 and13width12",
            "cover 255 bits: 9 width 11 and 13 width 12",
        ),
        ("totaling35840", "totaling 35840"),
        ("charges22full2048", "charges 22 full 2048"),
        ("is352bytes on64bit", "is 352 bytes on 64-bit"),
        ("private176-file", "private 176-file"),
        ("copies175", "copies 175"),
        ("originals:174", "originals: 174"),
        ("recovers175", "recovers 175"),
        ("124OFF/126ON", "124 OFF/126 ON"),
        ("and8prover", "and 8 prover"),
        ("exact320bit", "exact 320-bit"),
        (
            "over4861valid scalars/106942digits",
            "over 4861 valid scalars/106942 digits",
        ),
        ("workers1/4", "workers 1/4"),
        ("parts1/2", "parts 1/2"),
        ("lengths0/1/17", "lengths 0/1/17"),
        ("unchanged17field", "unchanged 17 field"),
        ("inherit33field", "inherit 33 field"),
        ("Ryzen9950X,32logical", "Ryzen 9950X, 32 logical"),
        ("CPUs0–31", "CPUs 0–31"),
        ("freshOFF", "fresh OFF"),
        ("measures151", "measures 151"),
        ("after3primes", "after 3 primes"),
        ("All9shapes×3variants", "All 9 shapes × 3 variants"),
        ("All45latency", "All 45 latency"),
        ("their45Gnark", "their 45 Gnark"),
        ("below1", "below 1"),
        ("least10/15wins", "least 10/15 wins"),
        ("ranks4–12 of15", "ranks 4–12 of 15"),
        ("nominal96.484375%", "nominal 96.484375%"),
        ("31requests", "31 requests"),
        ("separate199Hz", "separate 199 Hz"),
        ("251requests", "251 requests"),
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
        ("against17,005", "against 17,005"),
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
