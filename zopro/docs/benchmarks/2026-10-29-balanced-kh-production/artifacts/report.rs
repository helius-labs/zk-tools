use serde_json::Value;
use std::{fmt::Write, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/balanced-kh-production";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-balanced-kh-production";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn n(v: &Value) -> f64 {
    v.as_f64().unwrap()
}
fn main() {
    let r = Path::new(R);
    let a = read(r.join("analysis.json"));
    let cases = read(r.join("cases.json"));
    let ids = read(r.join("identities.json"));
    assert_eq!(a["SDK_requests"], 28000);
    assert_eq!(a["new_independent_exports"], 505);
    assert_eq!(a["independent_proof_count"], 17005);
    assert_eq!(a["previous_full_performance_gate_passed"], false);
    let primary = &a["cells"]["1x2-w32/ON"];
    let accepted = &a["cells"]["1x2-w32/accepted"];
    let pair = &a["pairs"]["1x2-w32/ON/accepted"];
    let mut s = String::from("# Clean balanced K/H prover: final-artifact qualification\n\n");
    writeln!(s,"Primary at 32 workers measures **{:.6} ms candidate versus {:.6} ms accepted**. Paired ON/accepted is {:.9}, bounds [{:.9}, {:.9}], with {}/15 wins. The complete current performance gate is **{}**; the separately declared engineering-assessment gate is **{}**. Production remains unchanged and the candidate is not adopted by this report.\n",n(&primary["median_ms"]),n(&accepted["median_ms"]),n(&pair["median"]),n(&pair["low"]),n(&pair["high"]),pair["wins"],a["gate_passed"],a["final_artifact_candidate_gate_passed"]).unwrap();
    writeln!(s,"Current failed checks: `{}`. The [preceding private-candidate qualification](../2026-10-28-balanced-kh-qualification175/README.md) remains **failed** at `1x2-w16/OFF/accepted`; this new artifact does not relabel that result. [Portable/resource checks](../2026-10-29-balanced-kh-compatibility175/README.md) also remain separate evidence.\n",a["failed_checks"]).unwrap();
    s.push_str("## Source, builds and method\n\nThe clean candidate removes the private balanced cfg and the optional trace statement. Its child module is byte-exact to rustfmt of the tested child after removing that statement. The parent adds one private module declaration and a width-13 fused K/H dispatch after existing input assertions. Of 174 accepted files, 173 remain byte-exact; reversing the parent edits and removing the child recovers all 174. There are 175 candidate files. The balanced 12/13-bit windows retain existing field arithmetic, borrowed queues, complete exceptions and conservative allocation forecasts. No key table, duplicate plan, scheduler, pool, image or public API change is made. Production source is unchanged during assessment.\n\nFresh native control and candidate use the same 17 accepted flags, without the private balanced flag. Fresh candidate native core/prover suites pass 124/8 tests; portable x86-64/no-IFMA/no-ASM suites pass 101/8, including both balanced scalar/point oracles. Control 122/8 and field 33-native/11-portable evidence are inherited only where source is unchanged. All transitive libraries and shared proc macros are frozen before standard eight-library SDK links. This is qualification of the actual clean artifact, not a repeat of the previous private binary.\n\nRyzen 9950X: all nine shapes use 32 workers on CPUs 0–31; primary also uses 16 distinct physical cores on CPUs 0–15. Fifteen fixed rotated/reversed blocks compare accepted, fresh OFF and clean ON in separate processes. Primary processes measure 151 requests, others 31, after three primes. Witness solving, fresh randomized proving, SDK verification and serialization are timed; image loading is excluded. No exclusive-host claim is made.\n\nBefore latency, 13 portable processes cover all nine basis shapes and primary/54 raw/denied preparation, followed by 30 native preflights. Every preflight export verifies independently before latency. All 450 latency captures precede their 450 Gnark checks. Six FIFO-gated stat runs and six separate cycle profiles then precede their 12 verifications and decoding. No owned build, helper, hash, archive or second workload overlaps captures. There is no diagnostic trace in the clean prover; route evidence is the checked source/control flow.\n\n## Complete verified requests\n\nTimes are medians of fifteen process medians, not paired-ratio estimates. CPU is aggregate measured-loop CPU per request; RSS includes loading/primes and is not minimum RAM.\n\n| Case | Accepted ms | Fresh OFF ms | Clean ON ms | ON CPU ms | ON RSS median/max MiB |\n|---|---:|---:|---:|---:|---:|\n");
    for c in cases.as_array().unwrap() {
        let k = c["name"].as_str().unwrap();
        let p = &a["cells"][format!("{k}/ON")];
        writeln!(
            s,
            "| {k} | {:.6} | {:.6} | {:.6} | {:.3} | {:.2}/{:.2} |",
            n(&a["cells"][format!("{k}/accepted")]["median_ms"]),
            n(&a["cells"][format!("{k}/OFF")]["median_ms"]),
            n(&p["median_ms"]),
            n(&p["CPU_ms"]),
            n(&p["median_RSS_MiB"]),
            n(&p["max_RSS_MiB"])
        )
        .unwrap();
    }
    s.push_str("\n| Case | ON/accepted paired [bounds] | Wins | ON/OFF paired [bounds] | Wins |\n|---|---|---:|---|---:|\n");
    for c in cases.as_array().unwrap() {
        let k = c["name"].as_str().unwrap();
        let p = &a["pairs"][format!("{k}/ON/accepted")];
        let q = &a["pairs"][format!("{k}/ON/OFF")];
        writeln!(
            s,
            "| {k} | {:.9} [{:.9}, {:.9}] | {} | {:.9} [{:.9}, {:.9}] | {} |",
            n(&p["median"]),
            n(&p["low"]),
            n(&p["high"]),
            p["wins"],
            n(&q["median"]),
            n(&q["low"]),
            n(&q["high"]),
            q["wins"]
        )
        .unwrap();
    }
    s.push_str("\nPrimary32 requires upper candidate ratios below 1 and at least 10/15 wins against both controls. Physical16 requires both upper ratios at most 1.02; other32 cases at most 1.03. Every full-screen baseline calibration interval must fit [1/1.03, 1.03]. The separate engineering-assessment gate excludes only physical16 OFF/accepted calibration, as declared before capture; it does not turn a failed full gate into a pass. Passing it alone does not adopt the candidate or achieve the original goal.\n\n| Case | OFF/accepted paired [bounds] | Calibration |\n|---|---|---|\n");
    for c in cases.as_array().unwrap() {
        let k = c["name"].as_str().unwrap();
        let key = format!("{k}/OFF/accepted");
        let p = &a["pairs"][&key];
        writeln!(
            s,
            "| {k} | {:.9} [{:.9}, {:.9}] | {} |",
            n(&p["median"]),
            n(&p["low"]),
            n(&p["high"]),
            if a["calibration_checks"][&key] == true {
                "pass"
            } else {
                "FAIL"
            }
        )
        .unwrap();
    }
    writeln!(s,"\nAccepted SHA256 is `{}`; fresh OFF is `{}`; they are {}. Independent process streams and all fifteen observations remain. Bounds are paired-ratio ranks 4–12 of 15, nominal 96.484375% coverage under independent exchangeable blocks, without a familywise guarantee. No minimum gain floor, exclusion, sample extension, threshold change, fastest-layout selection or successful-capture repeat is applied. Small positive and negative estimates remain in the tables without being rounded to zero.\n",ids["accepted"]["sha256"].as_str().unwrap(),ids["OFF"]["sha256"].as_str().unwrap(),if ids["accepted"]["sha256"]==ids["OFF"]["sha256"]{"byte-identical executables at separate paths"}else{"distinct compiled executables"}).unwrap();
    s.push_str("## Memory and diagnostics\n\n| Case | Accepted RSS median/max MiB | OFF RSS median/max MiB | ON RSS median/max MiB |\n|---|---:|---:|---:|\n");
    for c in cases.as_array().unwrap() {
        let k = c["name"].as_str().unwrap();
        write!(s, "| {k} |").unwrap();
        for v in ["accepted", "OFF", "ON"] {
            let p = &a["cells"][format!("{k}/{v}")];
            write!(
                s,
                " {:.2}/{:.2} |",
                n(&p["median_RSS_MiB"]),
                n(&p["max_RSS_MiB"])
            )
            .unwrap();
        }
        s.push('\n');
    }
    s.push_str("\nNo input-sized buffer or retained key table is added; that is not a claim of identical process peaks or a general memory reduction. The earlier eight configured memory-limit cases had zero high/max/OOM events and did not establish behavior under active throttling. Those cases are not new measurements of this clean binary.\n\nStat runs measure 31 requests and cycle profiles 251, after three primes. FIFO gates exclude loading and primes; diagnostic times do not enter latency decisions.\n\n| Diagnostic | Cycles/request, million | Instructions/request, million | Generic cache misses/request, million | Branch misses/request, million |\n|---|---:|---:|---:|---:|\n");
    for (k, p) in a["diagnostics"]["counters"].as_object().unwrap() {
        let e = &p["events"];
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
    let samples: u64 = a["diagnostics"]["profiles"]
        .as_object()
        .unwrap()
        .values()
        .map(|p| p["samples"].as_u64().unwrap())
        .sum();
    writeln!(s,"\nAll stat events run at least 99.9%. The six profiles contain {samples} samples, one SDK PID each and zero lost records. Complete per-profile PID/TID sets and symbol weights are retained. These observations do not isolate removable wall time or establish a cache, DRAM, scheduler or external-preemption cause.\n").unwrap();
    s.push_str("## Fresh CPU and Metal comparisons\n\nM5 values are supplied historical references, not new M5 captures. Ratios use the fresh 32-worker medians.\n\n| Shape | Accepted ms | ON ms | M5 CPU ms | CPU/accepted | CPU/ON | M5 Metal ms | Metal/accepted | Metal/ON |\n|---|---:|---:|---:|---:|---:|---:|---:|---:|\n");
    for c in cases.as_array().unwrap() {
        if c["workers"] != 32 || c["reference"].is_null() {
            continue;
        }
        let k = c["name"].as_str().unwrap();
        let p = &a["new_reference_comparisons"][k];
        let v = &p["variants"];
        writeln!(
            s,
            "| {} | {:.6} | {:.6} | {:.0} | {:.3}× | {:.3}× | {:.0} | {:.3}× | {:.3}× |",
            c["shape"].as_str().unwrap(),
            n(&v["accepted"]["ms"]),
            n(&v["ON"]["ms"]),
            n(&p["reference"]["cpu_ms"]),
            n(&v["accepted"]["CPU_speedup"]),
            n(&v["ON"]["CPU_speedup"]),
            n(&p["reference"]["metal_ms"]),
            n(&v["accepted"]["Metal_speedup"]),
            n(&v["ON"]["Metal_speedup"])
        )
        .unwrap();
    }
    writeln!(s,"\nCandidate reaches {}/8 CPU10× thresholds; accepted reaches {}/8. The original all-eight goal remains active pending its full completion audit. Primary requires 3.7 ms. No M5 reference was supplied for 54→1, whose complete observations remain above. No current AWS/Aeglos comparison follows from local data.\n",a["candidate_original8_CPU10x_rows"],a["accepted_original8_CPU10x_rows"]).unwrap();
    s.push_str("## Verification and provenance\n\nAll 1,028 workflow steps complete. All **28,000 SDK requests** verify internally. All **505 fresh 164-byte exports** independently pass Gnark, exact public/key binding, member hashing and uniqueness against 16,500 previous identities. The ledger reaches **17,005**; 1,010 archived proof/public members represent 505 proofs. Raw portable images are verified with their corresponding Euler verifying image; both identities are retained. Every closing source, library, helper, executable, image and cache pin matches.\n\nAll source files, including non-Rust manifests/locks/licenses/shaders, fresh build/test/link receipts, command logs, observations, exported proofs and decoded profiles are retained. Preparation failures before any successful build/test/capture are preserved; their fixes concern a field-receipt schema assertion and formatting of old test imports, with no runtime algorithm variant. Six raw perf files remain local with hashes; ownership changes only for those owned files after capture. No successful capture is repeated. No new phone/ARM/TEE, giant250/4999, concurrency, swap, forced-pressure or AWS result is claimed. Production is unchanged and any engineering adoption decision remains separate.\n\n[Protocol](artifacts/final-protocol.md), [analysis](artifacts/analysis.json), [identities](artifacts/identities.json), [build completion](artifacts/native/result.json), [source audit](artifacts/source-audit.json), [proof inventory](artifacts/proof-inventory.json), [capture completion](artifacts/runs/result.json), [closing pins](artifacts/pins.json).\n");
    fs::write(Path::new(D).join("README.md"), s).unwrap();
    println!("Reported all10cases/all8CPU+Metal;full and engineering gates distinct;505Gnark");
}
