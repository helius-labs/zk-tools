use serde_json::Value;
use std::{fmt::Write, fs, path::Path};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn main() {
    let r = Path::new("/tmp/zopro-phased-complex-products");
    let p = Path::new(
        "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-26-phased-complex-products",
    );
    fs::create_dir_all(p).unwrap();
    let result = read(r.join("component-result.json"));
    let profile = read(r.join("profile-analysis.json"));
    let code = read(r.join("codegen.json"));
    let mut s=String::from("# Phased quadratic-extension multiplication\n\nThe component fails its fixed speed gate and is not adopted. Its paired aggregate is 12.47% slower than accepted ordinary arithmetic, 12.40% slower than fresh ordinary arithmetic, and 31.28% slower than the earlier shared-CIOS component. None of the 31 block aggregates wins against any control. The smaller standalone stack frame does not translate into a speed improvement.\n\n## Arithmetic and scope\n\nFor exact integer products A=a0*b0, B=a1*b1 and C=(a0+a1)*(b0+b1), the prototype first forms D=A−B and S=−(A+B). It completes the real Montgomery reduction before forming the exact input sums and accumulating C into S for the imaginary reduction. This uses three raw products and two reductions, as the earlier [shared-product experiment](../2026-10-25-shared-complex-products/README.md) did, but replaces simultaneous row CIOS with phased Comba convolutions. Logical phase boundaries do not force LLVM register lifetimes.\n\nInputs have normalized radix-52 limbs and values at most 2p, with p < 2^254. Exact sums fit below 2^256. Signed carries preserve the real difference; biasing its reduced value by p and conditionally subtracting 2p gives an output below 2p. The imaginary cross-product is nonnegative and its reduced result also needs at most one 2p correction. The [range argument](artifacts/range-argument.md) covers signed coefficients and mixed-radix packing.\n\nOnly two native field files gain methods. Removing the additions recovers both original files byte for byte; the other 168 accepted source files match. There is no core dispatch, scheduler, key table, transformed key, persistent cache or heap buffer. This experiment does not measure actual G2 callers, complete curve operations, proof latency or minimum RAM.\n\n## Fixed component comparison\n\nRyzen 9950X, one thread pinned to CPU 2, 31 rotated/reversed blocks, four patterns and four variants. Every process runs three 2,000-iteration primes and 100,000 measured iterations. All 496 observations and 49.6 million measured iterations are retained; no other owned build, profile or archive overlaps latency. Packing and canonical conversion occur outside timing, and an optimizer barrier consumes state before the end timestamp. Numbers below are ns per eight-lane complex element per iteration, additionally divided by four for independent streams.\n\n| Pattern | Accepted | Fresh ordinary | Shared CIOS | Phased Comba | Phased/fresh | Phased/shared |\n| --- | ---: | ---: | ---: | ---: | ---: | ---: |\n");
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
            ns("phased"),
            ratio("fresh"),
            ratio("shared")
        )
        .unwrap();
    }
    s.push_str("\nRatios are medians of block-paired ratios; times are medians of process observations. The gate requires aggregate improvement with at least 21/31 winning blocks against each of three controls, each pattern within 3% regression, and fresh/accepted calibration within a factor of 1.03. All 12 pattern checks and three aggregate checks fail. Calibration passes for every pattern, with maximum factor 1.002394263. Every pattern loses all 31 blocks against all three controls. All 41,664 final canonical word comparisons agree. No samples are excluded and no unchanged trial is repeated.\n\n## Generated code and profiles\n\nThese are exact linked standalone wrappers, not inlined prover callers. Literal stack subtractions exclude alignment, pushes and callees; RSP operand lines are not measured spill counts.\n\n| Wrapper | Bytes | Instructions | Static IFMA sites | Literal stack subtraction bytes | RSP operand lines |\n| --- | ---: | ---: | ---: | ---: | ---: |\n");
    for v in ["accepted", "fresh", "shared", "phased"] {
        let c = code["symbols"]
            .as_array()
            .unwrap()
            .iter()
            .find(|x| x["variant"] == v && x["symbol"].as_str().unwrap().starts_with("complex_"))
            .unwrap();
        writeln!(
            s,
            "| {v} | {} | {} | {} | {} | {} |",
            c["size_bytes"],
            c["instructions"],
            c["ifma_sites"],
            c["literal_rsp_subtraction_sum"],
            c["rsp_operand_lines"]
        )
        .unwrap();
    }
    s.push_str("\nThe phased implementation uses full low-digit cancellation, accounting for ten more static IFMA sites than the earlier shared-CIOS wrapper. Its stack allocation is much smaller, but it still loses every timing comparison. This does not isolate a causal cost for cancellation, dependency chains, loads or spills. The actual repeat functions and disassemblies are archived too; the phased repeat function has 16,320 bytes of literal stack subtractions versus 21,696 for shared CIOS.\n\nAfter all latency, four declared 999 Hz user-cycle captures run the affine expression for 20 million iterations each. These include setup, primes and output, rather than SDK warm intervals. All profile output words agree.\n\n| Profile | ns/complex element | Samples | Lost records |\n| --- | ---: | ---: | ---: |\n");
    for c in profile["profiles"].as_array().unwrap() {
        writeln!(
            s,
            "| {} | {:.6} | {} | {} |",
            c["variant"].as_str().unwrap(),
            c["profile_ns_per_lane"].as_f64().unwrap(),
            c["samples"],
            c["lost_event_records"]
        )
        .unwrap();
    }
    writeln!(s,"\nThe {} samples match raw sample-event counts, with one PID per capture and zero lost records. Symbol-period attribution is retained without treating it as removable wall time or a causal model.\n",profile["samples"]).unwrap();
    s.push_str("## Validation and decision\n\nAll 33 existing native field tests and the generator consistency check pass. Four new tests cover Fq/Fr, R256/R260 and four/eight SIMD lanes, including zero, p, 2p, limb boundaries, lazy inputs and eight-step chains. They check 729,088 result lanes against Arkworks and 91,136 raw representatives against independent exact-integer REDC and signed-coefficient models. Outputs have normalized limbs and remain strictly below 2p. The maximum modeled prepacking coefficient is 51,068,970,446,924,509, below the conservative 144,115,188,075,855,872 bound. Test-only aligned representation access is documented in the source.\n\nThe first oracle build failed on an ambiguous BigUint shift type. One explicit test-variable type annotation repaired it before any oracle execution or timing. The failed receipt and initial source remain; successful native field tests were reused after verifying exit and stream hashes. Arithmetic, timing harness and gates were unchanged. All 213 final pins and the accepted 170 source files plus qualified SDK pass the closing hash audit.\n\nThere are no new benchmark SDK requests, proof exports or Gnark verifications in this component experiment. Two fixed source proof/public fixtures are separate. No actual G2 caller, portable device, concurrent SDK, memory-pressure, large-circuit, mobile, AWS or Aeglos qualification is claimed. The component is rejected before SDK integration. A new experiment needs a distinct arithmetic argument supported by generated code and profiles; reducing stack use alone is insufficient evidence.\n\n## Accepted prover context\n\nThe following accepted results are inherited from the [production checkpoint](../2026-10-25-grouping-module/README.md), not measurements made by this component experiment. The CPU and Metal references were supplied historical M5 measurements.\n\n| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |\n| --- | ---: | ---: | ---: | ---: | ---: |\n");
    for (shape, ms, cpu, metal) in [
        ("1x2", 8.594267, 37., 24.),
        ("2x2", 11.697247, 59., 30.),
        ("4x2", 16.158262, 109., 48.),
        ("8x2", 24.477005, 189., 74.),
        ("12x2", 34.386319, 284., 101.),
        ("16x2", 40.867383, 335., 124.),
        ("24x2", 69.265160, 528., 176.),
        ("merge-24x1", 68.150148, 507., 175.),
    ] {
        writeln!(
            s,
            "| {shape} | {ms:.6} | {cpu:.0} | {:.2}× | {metal:.0} | {:.2}× |",
            cpu / ms,
            metal / ms
        )
        .unwrap();
    }
    s.push_str("\nAccepted merge54 remains 178.891799 ms; no M5 reference was supplied for it. The all-eight 10× CPU goal remains unmet.\n\n[Protocol](artifacts/experiment.json), [decision and all block ratios](artifacts/component-result.json), [order](artifacts/timing-order.json), [source audit](artifacts/source-audit.json), [generated code](artifacts/codegen.json), [profiles](artifacts/profile-analysis.json), [closing audit](artifacts/closeout.json), [artifact hashes](artifacts/SHA256SUMS). Four raw captures and eight expanded profile streams remain hash-indexed locally and omitted from Git.\n");
    fs::write(p.join("README.md"), s).unwrap();
    println!("Wrote factual component rejection report");
}
