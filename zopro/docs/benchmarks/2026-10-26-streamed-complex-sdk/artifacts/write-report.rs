use serde_json::Value;
use std::{fmt::Write, fs, path::Path};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn main() {
    let root = Path::new("/tmp/zopro-streamed-complex-sdk");
    let result = read(root.join("pilot-result.json"));
    let profiles = read(root.join("profile-analysis.json"));
    let instructions = read(root.join("instruction-summary.json"));
    let code = read(root.join("codegen.json"));
    let mut s=String::from("# Streamed quadratic-extension multiplication in the CPU prover\n\nThe full SDK candidate fails its fixed speed gate and is rejected. Primary paired latency regresses 4.753% against accepted and 4.327% against fresh OFF, with 0/7 and 1/7 wins. The original-eight aggregate regresses 2.921% and 2.560%, losing every block against both controls. All OFF calibration checks pass. Production remains unchanged.\n\n## Integration and scope\n\nThe [preceding component](../2026-10-26-streamed-complex-b/README.md) passed its field benchmark: roughly 15% faster than ordinary arithmetic and 0.656% faster than the earlier shared-CIOS method, with a 3.202% gain in its affine-expression pattern. This trial integrates that exact streamed method into the actual native IFMA8 G2 lane product and conjugate-numerator product. It retains three raw products, two simultaneous signed CIOS accumulators and two reductions, precomputes summed A and streams exact summed B one radix-52 digit per row.\n\nTwo field files gain the byte-identical tested methods; a core FqBatch bridge and two G2 methods select them at compile time. All four original files recover through documented inverse edits; the other 166 accepted files match. OFF uses accepted native flags; a separate ON executable adds only `--cfg zopro_streamed_complex_b`. There is no per-call environment check, runtime selector, table, cache, heap buffer, representation, worker, scheduler, solver, query-order, Key/Plan or workspace-forecast change. The ordinary four-lane/portable path remains.\n\n## Complete SDK comparison\n\nRyzen 9950X, 16 physical cores and 32 logical CPUs. Seven rotated/reversed process blocks compare accepted, freshly compiled OFF and ON. Every 32-worker shape measures 31 requests after three primes; primary with 16 workers measures 101 on CPUs 0–15. Requests include solving, fresh randomness, proving, SDK verification and serialization. Loading and primes are excluded. All latency finishes before profiles; no other owned build, profile or archive overlaps latency.\n\n| Case | Accepted ms | OFF ms | ON ms | Paired ON/accepted | Paired ON/OFF |\n| --- | ---: | ---: | ---: | ---: | ---: |\n");
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
    s.push_str("\nTimes are medians of process medians; ratios are medians of paired block ratios and can rank differently. The gate requires primary and original-eight improvement against both controls, primary winning at least 4/7 blocks, every case within 2% paired median and 5% in each block, and OFF calibration within a factor of 1.03. Forty-one checks fail. Primary and aggregate failures independently reject the candidate. Maximum OFF calibration factor is 1.013990000. No minimum positive-gain floor, observation exclusion or unchanged retry applies.\n\n| Unprofiled resource observation | Accepted | OFF | ON |\n| --- | ---: | ---: | ---: |\n");
    for (name, label) in [("1x2-w32", "Primary"), ("merge-54x1-w32", "Merge54")] {
        for (key, unit) in [
            ("cpu_ms_per_request", "CPU ms/request"),
            ("maxrss_mib", "maximum process RSS MiB"),
        ] {
            let c = &result["cases"][name]["metrics"];
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
    s.push_str("\nCPU entries are medians of process means. RSS includes loading and primes; it is not minimum RAM or a live-workspace bound. Primary ON and accepted reach the same observed maximum RSS, while ON uses more CPU time and takes longer. Fault and context-switch counters remain in every raw row. These observations do not diagnose a general RAM-bandwidth limit.\n\n## Actual generated code and profiles\n\nThe standalone component's smaller frame did not carry into the prover. All three SDKs emit the same canonical-wire caller counts. Both G2 prefix specializations and G2 reduction use fewer static IFMA sites in ON, while the functions and literal stack allocations grow.\n\n| Variant and actual caller | Bytes | Instructions | Static IFMA sites | Literal stack subtraction bytes | RSP operand lines |\n| --- | ---: | ---: | ---: | ---: | ---: |\n");
    for c in code["symbols"].as_array().unwrap() {
        let name = c["symbol"].as_str().unwrap();
        let label = if name.contains("reduce_layer") {
            "G2 reduction"
        } else if name.contains(", true>") {
            "G2 prefix true"
        } else if name.contains("g2_prefix") {
            "G2 prefix false"
        } else {
            "canonical wires"
        };
        writeln!(
            s,
            "| {} {label} | {} | {} | {} | {} | {} |",
            c["variant"].as_str().unwrap(),
            c["size_bytes"],
            c["instructions"],
            c["ifma_sites"],
            c["literal_rsp_subtraction_sum"],
            c["rsp_operand_lines"]
        )
        .unwrap();
    }
    s.push_str("\nStatic sites are not executed instruction counts. Literal stack subtractions omit alignment, pushes and callees; RSP operand lines do not count dynamic spills or bytes transferred. The larger caller and extra arithmetic/operand work are consistent with the SDK regression, but these data do not isolate a causal cost. Fewer multiplications alone did not deliver a proof-speed gain.\n\nAfter latency, nine declared warm FIFO-controlled captures use monotonic user-cycle sampling at primary/101, 12x2/51 and merge54/9 requests. All 301,177 samples are retained with zero lost records and one PID per capture.\n\n| Primary warm profile | CPU ms/request | G1 sample % | G2 sample % | Epoch + scheduler sample % | FFT sample % |\n| --- | ---: | ---: | ---: | ---: | ---: |\n");
    for c in profiles["profiles"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["case"] == "1x2-w32")
    {
        let p = &c["category_percent"];
        writeln!(
            s,
            "| {} | {:.3} | {:.3} | {:.3} | {:.3} | {:.3} |",
            c["variant"].as_str().unwrap(),
            c["cpu_ms_per_request"].as_f64().unwrap(),
            p["G1_arithmetic"].as_f64().unwrap(),
            p["G2_arithmetic"].as_f64().unwrap(),
            p["epoch"].as_f64().unwrap() + p["scheduler"].as_f64().unwrap(),
            p["fft"].as_f64().unwrap()
        )
        .unwrap();
    }
    s.push_str("\nAfter rejection, the qualified Rust instruction reader analyzes all three retained primary captures: 49,666 samples, with no new timings or captures. Recorded PID-specific MMAP2 and ELF PT_LOAD offsets determine relocation; every instruction's bytes match the executable and sampled executable IPs match exact boundaries. Outside-executable, aliases and unclassified code remain explicit. An independent JSONL check conserves each run's sample and period totals.\n\n| Primary G2-prefix instruction observations | Scope share of capture % | IFMA within scope % | Vector add/sub within scope % | RSP operands within scope % |\n| --- | ---: | ---: | ---: | ---: |\n");
    for c in instructions["bins"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["scope"] == "g2_prefix")
    {
        let p = &c["within_scope_percent"];
        writeln!(
            s,
            "| {} | {:.3} | {:.3} | {:.3} | {:.3} |",
            c["variant"].as_str().unwrap(),
            c["share_of_capture_percent"].as_f64().unwrap(),
            p["ifma"].as_f64().unwrap(),
            p["vector_add_sub"].as_f64().unwrap(),
            p["rsp_operand"].as_f64().unwrap()
        )
        .unwrap();
    }
    s.push_str("\nInstruction families and address-register tags are orthogonal and overlap. These are weighted cycle-IP associations, subject to sampling skid; they are not exclusive opcode costs, retired counts or removable latency. Even the unchanged accepted/OFF instruction streams differ in their finite sampled IFMA shares. RSP/RBP tags do not prove spills or traffic; RBP can be a general register. Literal named scopes omit inlined work attributed to other enclosing symbols. The ON data supports investigating total arithmetic and intermediate lifetimes rather than using raw IFMA count as the performance model.\n\n## Validation and decision\n\nBoth separate native builds pass all 109 core tests, two raw-prover tests and six cubic tests. Required tests include independent G2-prefix/generator, odd-count/tail, reused-layer and combined-query oracles. The generator consistency check passes. The exact field methods inherit the preceding component's 91,136 BigInt/raw-row-model results and 729,088 Arkworks chained result lanes after byte matching; no new independent field-suite run is claimed.\n\nAll nine shapes pass OFF/ON four-worker preflight: 72 fresh SDK requests and 18 independent Gnark exports. Actual FFT Plan size/alignment is 1728/32 in accepted, OFF and ON. The full experiment completes 9,192 benchmark SDK requests and 237 distinct Gnark-verified exports, retaining 474 compressed proof/public members. Every proof is 164 bytes; public inputs match the established fixtures. Preflight freshness checks against 4,567 prior proof identities; performance adds the 18 preflight identities and checks against 4,585. Source proof/public digests from the preceding campaign are verified before building that set. Two fixed source fixture members are separate.\n\nAll 394 performance pins pass the closing audit, as do original build/preflight and instruction-analysis inputs. All four source edits recover whole original files, both field blocks still match the passed component, and the accepted 170 sources and qualified SDK remain unchanged. No source, compiler, gate or timing repair occurred during this SDK trial. The postmeasurement instruction analysis uses verified user-owned copies of three root-owned raw captures, with original/copy digests recorded. Original nine captures are retained and archived; those read copies need no duplicate archive bytes.\n\nNo further portable, small-stack, concurrent SDK, real-pressure, huge-circuit, mobile, AWS, Aeglos or minimum-RAM qualification was run after the failed speed gate. The component result remains valid in its scope; its integration is rejected. Next work must reduce the extra arithmetic or change the dataflow materially, then independently validate and measure the actual prover.\n\n## Supplied M5 references\n\nThese candidate diagnostics do not replace accepted production measurements. CPU and Metal references are supplied historical M5 timings, not fresh measurements of those devices.\n\n| Shape | ON ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |\n| --- | ---: | ---: | ---: | ---: | ---: |\n");
    for r in result["m5_supplied_references"].as_array().unwrap() {
        writeln!(
            s,
            "| {} | {:.6} | {} | {:.3}× | {} | {:.3}× |",
            r["shape"].as_str().unwrap(),
            r["on_ms"].as_f64().unwrap(),
            r["cpu_reference_ms"],
            r["on_cpu_speedup"].as_f64().unwrap(),
            r["metal_reference_ms"],
            r["on_metal_speedup"].as_f64().unwrap()
        )
        .unwrap();
    }
    s.push_str("\nNo M5 merge54 reference was supplied. Accepted production remains the [qualified checkpoint](../2026-10-25-grouping-module/README.md), historically 8.594267 ms for primary and 178.891799 ms for merge54. The all-eight CPU 10× goal remains active and unmet (0/8); this candidate is not adopted.\n\n[Protocol](artifacts/experiment.json), [all paired results](artifacts/pilot-result.json), [all raw rows](artifacts/perf/rows.json), [source audit](artifacts/source-audit.json), [caller code](artifacts/codegen.json), [warm profiles](artifacts/profile-analysis.json), [instruction bins](artifacts/instruction-summary.json), [proof inventory](artifacts/proof-inventory.json), [closing audit](artifacts/closeout.json) and [artifact index](artifacts/artifact-index.json). Nine raw captures, nine expanded sample streams, and three full disassembly/metadata pairs remain local with hashes and are omitted from Git. Executables/compiler caches are represented by pinned identities and sources.\n");
    let report = Path::new(
        "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-26-streamed-complex-sdk",
    );
    fs::create_dir(report).unwrap();
    fs::write(report.join("README.md"), s).unwrap();
    println!("Wrote complete SDK rejection report, verified9192requests237exports, no adoption");
}
