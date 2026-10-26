use serde_json::Value;
use std::{fs, path::Path};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn main() {
    let root = Path::new("/tmp/zopro-fq2-component-lanes");
    let report =
        Path::new("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-26-fq2-component-lanes");
    fs::create_dir_all(report).unwrap();
    let result = read(root.join("component-result.json"));
    let profile = read(root.join("profile-analysis.json"));
    let mut text=String::from("# Real and imaginary components in one SIMD vector\n\nRejected by the fixed arithmetic-component test. Packing four Fq2 elements into one eight-lane vector is **42.4% slower across the four workloads** and **2.14× slower in the affine expression**. Every workload loses all 31 paired blocks. No SDK trial or production change follows.\n\nThe candidate packs four real components in the lower half and their four imaginary components in the upper half. Two such vectors produce the same eight complex results as the reference's separate real/imaginary vectors. Existing half extraction/join, negation and fused Montgomery operations compute both outputs together. The field representation and kernels are unchanged. This tests register layout, without a new table, heap buffer, assembly, key copy or retained preparation.\n\nBoth tests and measurements explicitly link the qualified BN254 library, SHA-256 `680805fb88bce25c54240d20ca88dd455b80aea1c4c2c496e11ecf66a32200b9`. The standalone probe SHA-256 is `f70de4c244e374162ec06fee09f0d6ff5ec368f4b5a94d64c120e97524a81ed9`. The 170 accepted source files and qualified SDK remain unchanged.\n\n## Component measurements\n\nAMD Ryzen 9 9950X, one OS thread pinned to CPU 2. The fixed protocol rotates four workloads and alternates mode order over 31 fresh-process blocks. Each process performs three 2,000-iteration primes and 1,000,000 measured iterations. Both modes calculate eight Fq2 elements per iteration; the independent-stream workload calculates four such states. Initial input preparation/packing and final canonical conversion are outside timing. Arithmetic and a final optimizer barrier are inside. No other owned compilation, profile or benchmark overlaps latency.\n\n| Workload | Reference ns/Fq2 | Interleaved ns/Fq2 | Paired ratio | Wins |\n| --- | ---: | ---: | ---: | ---: |\n");
    for (p, label) in [
        ("chain", "Dependent chain"),
        ("four-streams", "Four independent streams"),
        ("scaled-product", "Product and real scaling"),
        ("affine-expression", "Norm and affine expression"),
    ] {
        let row = &result["patterns"][p];
        text.push_str(&format!(
            "| {label} | {:.6} | {:.6} | {:.6} | {}/31 |\n",
            row["reference_ns_per_element"].as_f64().unwrap(),
            row["interleaved_ns_per_element"].as_f64().unwrap(),
            row["paired_median_ratio"].as_f64().unwrap(),
            row["wins"]
        ));
    }
    text.push_str("\nThe aggregate is the geometric mean of the four paired median ratios: **1.424075201**, with **0/31** winning block aggregates. The prospective gate requires an improving affine expression and aggregate, each with at least 21/31 wins, plus every workload within 3%. It fails all four nonregression checks and both improvement checks. There is no positive-gain floor, discarded sample, threshold change or unchanged reroll.\n\nAll 248 measured processes complete 248 million iterations and 1,488,000 priming iterations. Every corresponding final canonical word agrees: 16,864 64-bit word comparisons across all states, including updated real scales. This is deterministic arithmetic, not fresh proof generation.\n\nThe affine workload evaluates denominator norms and the ordinary G2 reverse-pass coordinate formulas with a changing state and supplied real factor. It excludes scalar inversion, prefix-chain construction, streaming allocation, bucket collisions and full proving. Its state need not remain a curve point between iterations. Separate oracle tests below use valid curve points and actual inverse norms.\n\n## Linked code and profiles\n\nThe exact outer functions and their arithmetic callees are retained. LLVM outlines array construction in both layouts. For example, the reference affine outer routine has 21,025 bytes and 1,050 static IFMA sites; the candidate has 18,124 bytes and 705 sites. Those counts exclude callees. The reference scaling helper has 100 IFMA sites and the candidate multiplication helper has 150, each traversing two outputs. Static outer counts therefore do not establish fewer executed operations or lower register traffic.\n\nThe reference copies the accepted field formulas but uses an array-map wrapper for real scaling; the production helper spells out two products. This is a standalone expression comparator, not the actual linked SDK kernel. The candidate uses half-width helpers for norms and some carries, as documented in the source. These differences, extraction/join operations, stack traffic and call boundaries remain combined in the timing result; no causal share is isolated.\n\nAfter all latency measurements, one 20-million-iteration affine-expression capture per mode records user cycles at 999 Hz. The complete process is captured, including small setup, primes and output; there is no SDK warm-interval claim.\n\n| Profile | Samples | Profiled ns/Fq2 | Outer-function period share | Outlined arithmetic helper share |\n| --- | ---: | ---: | ---: | ---: |\n");
    for p in profile["profiles"].as_array().unwrap() {
        let sym = p["symbols"].as_array().unwrap();
        text.push_str(&format!(
            "| {} | {} | {:.6} | {:.3}% | {:.3}% |\n",
            p["mode"].as_str().unwrap(),
            p["samples"],
            p["ns_per_element"].as_f64().unwrap(),
            sym[0]["share"].as_f64().unwrap() * 100.0,
            sym[1]["share"].as_f64().unwrap() * 100.0
        ));
    }
    text.push_str("\nAll 14,977 decoded samples match raw sample-event counts, each capture has one process ID, and both have zero lost-event records. All 128 final profiled output words also agree. Period shares are sampled user-cycle attribution, not wall-time fractions or removable costs. The profiles support keeping the negative result, without attributing the slowdown solely to a call, spill or permutation.\n\n## Validation and retained failures\n\nFour new tests pass against the frozen accepted field library: 196,608 complex result-lane checks and 16,384 norm checks against independent Arkworks arithmetic, 864 modulus-corner/half-order checks, 384 ordinary G2 additions against Arkworks, and 20 short changing-state comparisons. Inputs include normalized lazy values on both sides of p and below 2p, signs, zeros and eight-step chains. The ordinary curve formula requires distinct x coordinates; this component does not implement bucket exception handling for identity, doubling or cancellation.\n\nThe initial oracle compilation omitted Arkworks' `AdditiveGroup` trait for its zero constant. It failed before any test or measurement. The import-only repair and original source/logs are archived. Cargo builds the oracle dependencies, while explicit Rust links select the accepted BN254 library for the tests and timed probe.\n\nThe first profile command exits successfully, then the workflow rejects changed executable inode metadata. Inspection finds the actual hard link created by perf's build-ID cache; contents and mtime are unchanged. All 200 content pins match. The successful reference capture is retained, then decoded without recapture; the candidate capture disables build-ID cache updates. Both the rejection and repaired workflow are archived. All 206 final content pins and the accepted source hashes pass the closing audit.\n\nThe arithmetic uses fixed-size vector states. This experiment adds no key table or persistent data, but supplies no process-memory, stack-bound, pressure, concurrent-caller, portable-device or minimum-RAM qualification. There are **zero new SDK proofs and zero Gnark exports**. No AWS, Aeglos, phone or large-circuit measurement is added.\n\n## Accepted prover context\n\nThese historical accepted local full-SDK timings are unchanged by this component. CPU and Metal references are the supplied M5 figures, not fresh Apple measurements. The all-eight 10× CPU goal remains unmet.\n\n| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |\n| --- | ---: | ---: | ---: | ---: | ---: |\n");
    for (s, t, c, m) in [
        ("1x2", 8.594267, 37., 24.),
        ("2x2", 11.697247, 59., 30.),
        ("4x2", 16.158262, 109., 48.),
        ("8x2", 24.477005, 189., 74.),
        ("12x2", 34.386319, 284., 101.),
        ("16x2", 40.867383, 335., 124.),
        ("24x2", 69.265160, 528., 176.),
        ("merge-24x1", 68.150148, 507., 175.),
    ] {
        text.push_str(&format!(
            "| {s} | {t:.6} | {c:.0} | {:.2}× | {m:.0} | {:.2}× |\n",
            c / t,
            m / t
        ));
    }
    text.push_str("\nAccepted merge-54 is 178.891799 ms; no corresponding M5 reference was supplied. The [accepted grouping-module qualification](../2026-10-25-grouping-module/README.md) and [prior shared-products integration](../2026-10-25-shared-complex-products/README.md) provide the surrounding context. This new result rejects the tested layout, without claiming that every possible interleaved representation is slower.\n\n[Protocol](artifacts/experiment.json), [component results](artifacts/component-result.json), [all process receipts](artifacts/timing), [probe source](artifacts/probe.rs), [native tests](artifacts/build-v2/oracle-tests/stdout.log), [linked outer routines](artifacts/codegen.json), [outlined helpers](artifacts/helper-codegen.json), [profile analysis](artifacts/profile-analysis.json), [closing audit](artifacts/closeout.json), [artifact hashes](artifacts/SHA256SUMS). Raw captures and expanded decode/dump streams remain in the local archive with hashes and are omitted from Git.\n");
    fs::write(report.join("README.md"), text).unwrap();
    println!("Wrote factual component report and historical all-eight CPU/Metal context");
}
