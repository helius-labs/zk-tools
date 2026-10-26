use serde_json::Value;
use std::{fmt::Write, fs, path::Path};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn main() {
    let root = Path::new("/tmp/zopro-streamed-complex-b");
    let result = read(root.join("component-result.json"));
    let profile = read(root.join("profile-analysis.json"));
    let code = read(root.join("codegen.json"));
    let mut s=String::from("# Streaming one summed input in quadratic-extension multiplication\n\nThe component passes its fixed gate and remains unadopted pending prover integration. Its paired aggregate is 14.970% faster than accepted ordinary arithmetic, 14.915% faster than fresh ordinary arithmetic and 0.656% faster than the earlier shared-CIOS method. All 31 aggregate block comparisons win against each control. These are field-component measurements, not proof latency.\n\n## Arithmetic and scope\n\nThe [saved instruction census](../2026-10-26-instruction-census/README.md) pointed to arithmetic and live intermediates in current G2 kernels, without establishing a spill or memory-bandwidth cause. The [earlier shared-product method](../2026-10-25-shared-complex-products/README.md) won its component and lost in the actual prover; the [phased-Comba follow-up](../2026-10-26-phased-complex-products/README.md) lost its component. Those results remain unchanged.\n\nThis prototype retains the original three exact products, two simultaneous signed CIOS accumulators and two reduction chains. It precomputes the exact five-limb a0+a1 sum and forms one b0+b1 digit, with its carry, inside each CIOS row. Product and cancellation expressions remain unchanged. It therefore removes four logically long-lived summed-B vectors without changing the arithmetic count. Logical source lifetimes do not force LLVM scheduling. No barrier or assembly is added.\n\nFor normalized radix-52 inputs at most 2p and p<2^254, each sum digit and carry matches the former precomputation by induction; exact sums fit below 2^256 and the final carry is zero. The real result is REDC(a0*b0-a1*b1)+p, and imaginary is REDC((a0+a1)*(b0+b1)-a0*b0-a1*b1). Each output requires at most one 2p correction and remains below 2p. See the [range argument](artifacts/range-argument.md).\n\nOnly two field files gain methods; removing the added blocks recovers both originals byte for byte. The other 168 accepted source files match. There is no prover dispatch, scheduler change, key table, persistent cache, heap buffer or new representation. This component does not measure actual G2 callers, verified proof latency or minimum RAM.\n\n## Fixed component comparison\n\nRyzen 9950X, one thread pinned to CPU 2, 31 rotated/reversed blocks, four patterns and four variants. Each process runs three 2,000-iteration primes and 100,000 measured iterations. All 496 observations and 49.6 million measured iterations are retained. No other owned build, profile or archive overlaps latency. Packing and canonical conversion are outside timing; an optimizer barrier consumes state before the end timestamp. Numbers are ns per eight-lane complex element per iteration, additionally divided by four for independent streams.\n\n| Pattern | Accepted | Fresh ordinary | Shared CIOS | Streamed B | Streamed/fresh | Streamed/shared |\n| --- | ---: | ---: | ---: | ---: | ---: | ---: |\n");
    for pattern in ["chain", "streams4", "scaled", "affine-expression"] {
        let c = &result["cases"][pattern];
        let ns = |v: &str| c["variants"][v]["median_ns_per_lane"].as_f64().unwrap();
        let ratio = |v: &str| {
            c["checks"]
                .as_array()
                .unwrap()
                .iter()
                .find(|x| x["control"] == v)
                .unwrap()["paired_median_ratio"]
                .as_f64()
                .unwrap()
        };
        writeln!(
            s,
            "| {pattern} | {:.6} | {:.6} | {:.6} | {:.6} | {:.6} | {:.6} |",
            ns("accepted"),
            ns("fresh"),
            ns("shared"),
            ns("streamed"),
            ratio("fresh"),
            ratio("shared")
        )
        .unwrap();
    }
    s.push_str("\nRatios are medians of block-paired ratios; times are medians of process observations. The gate requires aggregate improvement and at least 21/31 aggregate wins against all three controls, every pattern within 3% regression and fresh/accepted calibration within a factor of 1.03. Every check passes; maximum calibration factor is 1.004655195. All 41,664 final canonical word comparisons agree. There is no minimum positive-gain floor, sample exclusion or unchanged retry.\n\nThe gain over shared CIOS is concentrated in the affine expression: 3.202% paired improvement, winning all 31 blocks. Chain regresses 0.430% and independent streams regress 0.269%, with 13/31 and 8/31 wins; scaled is nearly flat at 0.076% improvement and 17/31 wins. The aggregate gain is not an improvement in every context.\n\n## Generated code and profiles\n\nThese are exact linked component wrappers and repeat functions, not prover G2 callers. Literal stack subtractions exclude alignment, pushes and callees. RSP operand lines do not measure spills or traffic.\n\n| Variant and function | Bytes | Instructions | Static IFMA sites | Literal stack subtraction bytes | RSP operand lines |\n| --- | ---: | ---: | ---: | ---: | ---: |\n");
    for c in code["symbols"].as_array().unwrap() {
        writeln!(
            s,
            "| {} {} | {} | {} | {} | {} | {} |",
            c["variant"].as_str().unwrap(),
            if c["symbol"].as_str().unwrap().starts_with("complex_") {
                "wrapper"
            } else {
                "repeat"
            },
            c["size_bytes"],
            c["instructions"],
            c["ifma_sites"],
            c["literal_rsp_subtraction_sum"],
            c["rsp_operand_lines"]
        )
        .unwrap();
    }
    s.push_str("\nThe streamed wrapper preserves 250 IFMA sites and has a smaller stack frame, while the full repeat has a slightly larger frame than shared CIOS. Both emitted routines differ after removing instruction addresses, RIP-relative displacements and comments; the declared codegen screen passes. These observations do not establish why the component is faster. Accepted and fresh ordinary wrappers match static counts, while their repeat contexts differ; the fixed fresh/accepted calibration passes.\n\nAfter every latency process finished, four declared 999 Hz user-cycle captures run 20 million affine-expression iterations each. They include setup, primes and output, rather than SDK warm intervals. All 192 profile word comparisons against three controls agree.\n\n| Profile | ns/complex element | Samples | Repeat-symbol period % | Lost records |\n| --- | ---: | ---: | ---: | ---: |\n");
    for c in profile["profiles"].as_array().unwrap() {
        writeln!(
            s,
            "| {} | {:.6} | {} | {:.3} | {} |",
            c["variant"].as_str().unwrap(),
            c["profile_ns_per_lane"].as_f64().unwrap(),
            c["samples"],
            c["symbols"][0]["share"].as_f64().unwrap() * 100.,
            c["lost_event_records"]
        )
        .unwrap();
    }
    s.push_str("\nAll 13,442 samples match raw sample-event counts, with one PID/TID per capture and zero lost records. At least 99.85% of each capture's period weight falls in its actual repeat function. Symbol attribution does not isolate opcode costs, stalls or removable wall time.\n\n## Validation and next stage\n\nAll 33 existing native field tests and the generator check pass. Four corrected, unfiltered oracle tests cover Fq/Fr, R256/R260 and four/eight SIMD lanes, including zero, p, 2p, limb boundaries, lazy inputs and eight-step chains. They verify 91,136 exact raw representatives against independent BigInt REDC and a signed scalar row-CIOS model, compare raw digits directly with old shared CIOS, and check 729,088 chained result lanes against Arkworks. Streamed sum carries/digits and final-zero carry are checked independently. Outputs have normalized limbs and lie strictly below 2p. The largest modeled prepacking coefficient is 41,058,117,838,107,261, below the conservative 144,115,188,075,855,872 bound; final mixed packing is checked below 2^62.\n\nThe first copied probe still contained the previous inline coefficient tests, so the newly prepared row-model file was initially unused. Those tests exercised the new streamed method and passed algebra/exact-REDC checks, but did not validate the new row model or direct old-shared raw-digit comparison. Before any timing, only the test section was replaced by a relative include of the pinned new tests. All four intended oracle tests and both probes were rebuilt. Field arithmetic and the runtime source prefix did not change. Initial source, binaries' identities and receipts remain; the unchanged 33 field tests and generator check are reused with verified output hashes. The original 185 pins match with one exact initial-probe-source relocation; corrected 197 and final 218 pin maps pass the closing audit.\n\nThe component pass authorizes a separately frozen actual-G2-caller and complete SDK experiment. Production remains unchanged. In particular, a smaller standalone frame cannot justify adopting the earlier shared method that lost in the prover. This candidate still requires full linked-caller, proof, whole-SDK speed and memory qualification. No actual G2 caller, portable device, concurrent SDK, memory-pressure, large-circuit, mobile, AWS, Aeglos or minimum-RAM result is claimed here. There are zero new SDK requests or Gnark exports; two fixed source fixture members are separate.\n\n## Inherited accepted prover comparison\n\nThese are unchanged accepted-production measurements against supplied historical M5 references, not new timings from this component.\n\n| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |\n| --- | ---: | ---: | ---: | ---: | ---: |\n");
    let base = read(root.join("accepted-manifest.json"));
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
        let r = &base["m5"][shape];
        writeln!(
            s,
            "| {shape} | {:.6} | {} | {:.3}× | {} | {:.3}× |",
            r["candidate_ms"].as_f64().unwrap(),
            r["cpu_reference_ms"],
            r["cpu_speedup"].as_f64().unwrap(),
            r["metal_reference_ms"],
            r["metal_speedup"].as_f64().unwrap()
        )
        .unwrap();
    }
    s.push_str("\nAccepted merge54 remains 178.891799 ms; no M5 merge54 reference was supplied. The all-eight CPU 10× goal remains active and unmet (0/8).\n\n[Protocol](artifacts/experiment.json), [all paired results](artifacts/component-result.json), [timing order](artifacts/timing-order.json), [source audit](artifacts/source-audit.json), [codegen screen](artifacts/codegen-screen.json), [generated code](artifacts/codegen.json), [profiles](artifacts/profile-analysis.json), [harness repair](artifacts/harness-repair.json), [closing audit](artifacts/closeout.json) and [artifact index](artifacts/artifact-index.json). Four raw captures and eight expanded profile streams are retained locally with hashes and omitted from Git. Executables/compiler caches are represented by pinned identities and sources.\n");
    let report =
        Path::new("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-26-streamed-complex-b");
    fs::create_dir(report).unwrap();
    fs::write(report.join("README.md"), s).unwrap();
    println!("Wrote factual component pass report; no proof-speed or adoption claim");
}
