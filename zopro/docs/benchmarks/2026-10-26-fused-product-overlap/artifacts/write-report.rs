use serde_json::{json, Value};
use std::{fmt::Write, fs, path::Path};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn main() {
    let r = Path::new("/tmp/zopro-fused-product-overlap");
    let report = Path::new(
        "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-26-fused-product-overlap",
    );
    fs::create_dir_all(report).unwrap();
    let result = read(r.join("pilot-result.json"));
    let profile = read(r.join("profile-analysis.json"));
    let code = read(r.join("codegen.json"));
    let mut s=String::from("# Overlapping the product with fused K/H proving queries\n\nThe fixed speed gate fails; the candidate is not adopted. Primary paired latency regresses 0.267% against accepted and 1.066% against fresh OFF, with 3/7 and 1/7 wins. The original-eight aggregate regresses 0.481% and 0.258%, winning one block against each control. All OFF calibration checks pass, including the physical-16 case. Production remains unchanged.\n\n## Scheduling change\n\nThe current small-proof path finishes the Euler product before starting A, fused K+H and G2. The prototype starts A and G2 while a scoped branch computes the product and then fused K+H. Arithmetic, scalar grouping, bucket widths, worker pool and field representations remain. Earlier four-query overlap trials were marginal and one failed a warm memory-pressure test; their results remain unchanged. This trial uses the current combined K/H stream, Euler product and G2 prefix implementation, with one fixed H-first schedule.\n\nThe corrected path requires the existing fused-query eligibility, an Euler query, sufficient query points and explicit `ZOPRO_CPU_FUSED_PRODUCT_OVERLAP=1`. A process OnceLock samples the flag once; omission and zero are OFF. An active allocator reclamation guard selects the sequential path. This decision is advisory at branch entry and does not reschedule a proof if pressure arrives later. Three existing core files gain additions; removing them recovers the originals byte for byte in both source versions. The other 167 accepted files, including field arithmetic and the workspace forecast, match.\n\nThe code moves existing input vectors and borrows existing key/scalar slices. It adds no key table, point copy, auxiliary heap buffer, worker or Key field. The unchanged forecast already sums FFT fields, all MSM parents and per-worker leaf scratch. More live buffers can coexist despite an unchanged upper forecast. Actual pressure behavior would require qualification before adoption.\n\n## Complete SDK comparison\n\nRyzen 9950X, 16 physical cores and 32 logical CPUs. Seven rotated/reversed process blocks compare accepted, fresh OFF and ON. Every 32-worker case measures 31 requests after three primes; physical-16 primary measures 101 after three primes on CPUs 0–15. Requests include fresh solving, randomness, proving, SDK verification and serialization. Loading and primes are excluded. All latency finishes before profiles; no other owned build, profile, archive or benchmark overlaps latency.\n\n| Case | Accepted ms | OFF ms | ON ms | Paired ON/accepted | Paired ON/OFF |\n| --- | ---: | ---: | ---: | ---: | ---: |\n");
    for name in [
        "1x2-w32",
        "2x2-w32",
        "4x2-w32",
        "8x2-w32",
        "12x2-w32",
        "16x2-w32",
        "24x2-w32",
        "merge-24x1-w32",
        "merge-54x1-w32",
        "1x2-w16",
    ] {
        let c = &result["cases"][name];
        writeln!(
            s,
            "| {name} | {:.6} | {:.6} | {:.6} | {:.9} | {:.9} |",
            c["metrics"]["accepted"]["median_ms"].as_f64().unwrap(),
            c["metrics"]["off"]["median_ms"].as_f64().unwrap(),
            c["metrics"]["on"]["median_ms"].as_f64().unwrap(),
            c["on_over_control"]["accepted"]["paired_median"]
                .as_f64()
                .unwrap(),
            c["on_over_control"]["off"]["paired_median"]
                .as_f64()
                .unwrap()
        )
        .unwrap();
    }
    s.push_str("\nTimes are medians of process medians; paired ratios can rank differently. The gate requires primary and original-eight improvement against both controls with at least 4/7 winning blocks, each case within 2% paired median and 5% in every block, and OFF calibration within a factor of 1.03. No minimum positive-gain floor, observation exclusion or unchanged retry applies. Thirteen checks fail: two primary-improvement checks, two aggregate checks, two physical-16 median checks, six physical-16 block checks and one 8x2 block check. The primary and aggregate failures independently reject the change. The branch is inactive on larger shapes; their variation does not demonstrate a causal effect of overlap.\n\n| Unprofiled resource observation | Accepted | OFF | ON |\n| --- | ---: | ---: | ---: |\n");
    for (name, label) in [("1x2-w32", "Primary"), ("merge-54x1-w32", "Merge54")] {
        let c = &result["cases"][name]["metrics"];
        for (key, unit) in [
            ("cpu_ms_per_request", "CPU ms/request"),
            ("maxrss_mib", "maximum process RSS MiB"),
        ] {
            writeln!(
                s,
                "| {label} {unit} | {:.3} | {:.3} | {:.3} |",
                c["accepted"][key].as_f64().unwrap(),
                c["off"][key].as_f64().unwrap(),
                c["on"][key].as_f64().unwrap()
            )
            .unwrap();
        }
    }
    s.push_str("\nCPU entries are medians of process means. RSS includes loading and primes; it is not minimum RAM or a live-scratch bound. Primary ON maximum RSS is higher in this finite observation.\n\n## Profiles and generated code\n\nAfter latency, nine warm user-cycle captures cover all variants at primary/101, 12x2/51 and merge54/9 requests. The audit verifies all tuples, round counts, FIFO warm control, monotonic clock, `--no-buildid-cache`, one sampled PID per capture, 297,412 samples and zero lost records. One capture per tuple does not establish a causal phase-cost decomposition.\n\n| Primary profile | CPU ms/request | G1 + G2 sample % | Epoch + scheduler sample % | FFT sample % |\n| --- | ---: | ---: | ---: | ---: |\n");
    for p in profile["profiles"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|p| p["case"] == "1x2-w32")
    {
        let c = &p["category_percent"];
        writeln!(
            s,
            "| {} | {:.3} | {:.2} | {:.2} | {:.2} |",
            p["variant"].as_str().unwrap(),
            p["cpu_ms_per_request"].as_f64().unwrap(),
            c["G1_arithmetic"].as_f64().unwrap() + c["G2_arithmetic"].as_f64().unwrap(),
            c["epoch"].as_f64().unwrap() + c["scheduler"].as_f64().unwrap(),
            c["fft"].as_f64().unwrap()
        )
        .unwrap();
    }
    s.push_str("\nBoth actual G2 prefix specializations retain identical static counts between accepted and candidate: the true specialization has 50,288 bytes, 8,326 instructions, 1,450 IFMA sites and 5,696 bytes of literal stack subtractions. The false specialization also matches its accepted counts. ON and OFF share one executable.\n\n| Actual canonical-wire caller | Bytes | Instructions | Literal stack subtraction bytes |\n| --- | ---: | ---: | ---: |\n");
    for c in code["symbols"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["symbol"] == "<zopro_core::Key>::solve_canonical_wires")
    {
        writeln!(
            s,
            "| {} | {} | {} | {} |",
            c["variant"].as_str().unwrap(),
            c["size_bytes"],
            c["instructions"],
            c["literal_rsp_subtraction_sum"]
        )
        .unwrap();
    }
    s.push_str("\nThe candidate additionally emits a separate fused helper. Static code/stack counts exclude dynamic execution, alignment, pushes and callees; they do not isolate a cause of the timing result. Overlapping these stages did not produce a reliable reduction in latency or CPU work. This experiment does not establish a general memory-bandwidth bottleneck.\n\n## Validation and retained failures\n\nThe corrected candidate passes 110 unfiltered core tests in both modes, two raw-prover tests and six cubic API tests. A nested reclamation-guard test and existing independent combined-query generator tests pass. All nine shapes pass corrected OFF/ON preflight. Trace checks confirm four primary ON calls and no OFF or ineligible calls. Field validation is inherited from unchanged source and pinned reference artifacts; no new field-suite run is claimed.\n\nThe complete campaign contains 9,228 completed benchmark SDK requests, 246 distinct exports independently verified by Gnark and 492 proof/public files. This includes 36 successful initial OFF preflight requests and nine exports, followed by 9,192 corrected-candidate/control requests and 237 exports. The failed ON attempt completed no SDK request or export. Proofs have the 164-byte compressed format, distinct hashes and expected fixture public inputs. Freshness is checked against 4,321 earlier identities, then 4,330 before corrected preflight and 4,348 before performance. Two fixed source fixture members are separate. All 361 performance pins pass the closing audit.\n\nThe first enabled preflight failed an assertion added by this prototype: the Euler product returns 30,922 actual rows rather than the padded 32,768-point domain. The repair restricts overlap to Euler products and uses max(a.len(), b.len()), matching the existing product routine. Source, binary and receipts from the failure are retained; both full native modes and preflight were repeated for the changed source. No comparative latency block ran before the repair. Initial preflight SAMPLE observations remain archived and are used only as correctness evidence.\n\nThe summary command initially omitted the frozen reader's two required arguments after all measurements had completed. The corrected invocation reads a byte-identical configuration alias; no gate, source or timing rerun follows. Rebuilding also reused one scheduler-metadata pathname after the first preflight terminated. Its original JSON is preserved separately, reconstructed from retained Cargo receipts and required to match its original pinned SHA exactly; original source, libraries, executables and proofs remain unchanged. The current performance pins are unchanged in place.\n\nFurther portable, public/generic/admission-denied, concurrent SDK and cold/warm pressure qualification was not run after the failed speed gate. No mobile, huge-circuit, AWS, Aeglos or minimum-RAM result is claimed. The tested schedule is rejected without adoption or an unchanged retry.\n\n## Supplied M5 references\n\nCandidate diagnostics below do not replace accepted production results. CPU and Metal references are the supplied historical measurements, not fresh Apple measurements.\n\n| Shape | ON ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |\n| --- | ---: | ---: | ---: | ---: | ---: |\n");
    for c in result["m5_supplied_references"].as_array().unwrap() {
        writeln!(
            s,
            "| {} | {:.6} | {:.0} | {:.2}× | {:.0} | {:.2}× |",
            c["shape"].as_str().unwrap(),
            c["on_ms"].as_f64().unwrap(),
            c["cpu_reference_ms"].as_f64().unwrap(),
            c["on_cpu_speedup"].as_f64().unwrap(),
            c["metal_reference_ms"].as_f64().unwrap(),
            c["on_metal_speedup"].as_f64().unwrap()
        )
        .unwrap();
    }
    s.push_str("\nNo merge54 M5 reference was supplied. The all-eight 10× CPU objective remains unmet. Accepted historical primary remains 8.594267 ms and merge54 178.891799 ms in the [production checkpoint](../2026-10-25-grouping-module/README.md).\n\nResearch review: [SQIsign AVX-512](https://arxiv.org/html/2608.13948v1) uses a special-prime reduction; [MORPH](https://arxiv.org/html/2604.17808v1) targets TPU arithmetic/dataflow. Neither supplies a measured BN254 CPU speedup for this prover. The schedule tested here is derived from the local dependency graph.\n\n[Protocol](artifacts/experiment.json), [repair](artifacts/repair.json), [all samples](artifacts/perf/rows.json), [decision](artifacts/pilot-result.json), [activation](artifacts/activation-audit-v2.json), [profiles](artifacts/profile-analysis.json), [actual code](artifacts/codegen.json), [proof inventory](artifacts/proof-inventory.json), [closing audit](artifacts/closeout.json), [metadata relocation](artifacts/pin-relocation.json), [hashes](artifacts/SHA256SUMS). Nine raw captures and nine expanded symbol streams remain hash-indexed locally and omitted from Git.\n");
    fs::write(report.join("README.md"), s).unwrap();
    fs::write(r.join("validation-notes.json"),serde_json::to_vec_pretty(&json!({"initial_timed_requests_before_repair_label":"The0inrepair.json refers to zero comparative latency-campaign requests. The nine correctness preflight SAMPLE observations from successful initialOFF runs are retained, not discarded or treated as comparative speed evidence.","all_initial_preflight_completed_requests":36,"all_successful_proof_exports_included":246,"no_comparative_measurement_repeated":true})).unwrap()).unwrap();
    println!(
        "Wrote complete factual rejection report, all-shape CPU/Metal context and provenance notes"
    );
}
