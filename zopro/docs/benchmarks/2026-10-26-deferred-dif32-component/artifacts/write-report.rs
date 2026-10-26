use serde_json::Value;
use std::{fs, path::Path};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn main() {
    let root = Path::new("/tmp/zopro-deferred-dif32");
    let report = Path::new(
        "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-26-deferred-dif32-component",
    );
    fs::create_dir_all(report).unwrap();
    let r = read(root.join("component-result.json"));
    let profiles = read(root.join("profile-analysis.json"));
    let code = read(root.join("codegen.json"));
    let mut s=String::from("# Deferred correction in five-stage FFT DIF tiles\n\n**Component gate passed; full-proof integration is not yet measured.** The new tile takes 32.15% less time than the accepted-library reference and 32.14% less than the freshly compiled reference. All three cases improve in all 31 paired blocks against both controls. Accepted prover source and timings remain unchanged.\n\nThe routine defers modular correction inside the existing 32-vector, five-stage decimation-in-frequency tile. It keeps normalized radix-52 limbs, biases differences by known multiples of p, and uses the unchanged ordinary Montgomery multiplication. It restores the ordinary below-2p representation before returning. Final correction requires 31 conditional passes across the tile, rather than correcting each of its 80 low butterfly outputs. Positive biased differences also avoid the original subtraction's conditional borrow repair.\n\nNo lookup table, root preparation, retained cache, key data, heap allocation, field representation or scheduler change is added. Only a new experimental block in the four-lane field source and its generated eight-lane version changes. Removing each block recovers the complete original file byte for byte; the other 168 accepted source files are identical. This experiment does not dispatch the routine from the prover.\n\n## Bounds and independent checks\n\nThe uniform radix is R=2^260. For either BN254 prime, 64p<R. Tile inputs are normalized below2p and every root is canonical belowp. At stage s, sum and biased difference are below2^(s+2)p, hence below64p. Multiplying the difference by its root has T<64p²<pR, so ordinary REDC returns below2p. The number of trailing low branches determines each final output bound; descending power-of-two modulus corrections restore below2p. [The complete local range argument](artifacts/range-argument.md) also bounds signed bias coefficients below2^58 and ordinary CIOS coefficients below32·2^52.\n\n[Scott's Slothful reduction](https://eprint.iacr.org/2017/437) motivates delaying correction when intermediate ranges permit it. Its primary abstract and indexed excerpts were consulted; the full PDF request returned403. The BN254/R260 tile schedule, range proof and implementation here are independently derived, with no imported performance estimate. This differs from the prior R260 bucket, bounded-subtraction, prepared-root and FFT scheduling trials.\n\nAll 33 existing native field tests and generator consistency pass. Six new tests check four/eight-lane implementations:\n\n- 49,152 raw output residues match an independent arbitrary-precision Montgomery/schedule model exactly, before host conversion. All output digits are normalized and values below2p; compact/expand roundtrips agree.\n- 122,880 modeled Montgomery products match an independently computed full-radix REDC identity. The model also checks ordinary CIOS coefficients; its largest observed coefficient is 47,239,727,476,272,382, below the conservative 144,115,188,075,855,872 limit. The existing four-lane implementation uses diagonal multiplication; its raw tile outputs are checked by the same exact-value oracle.\n- 5,760 field outputs match direct 32-point Fourier sums computed in Arkworks, with nonzero column twists, zeros, ones, impulses and signed/full-width values.\n- Twenty repeated eight-lane tiles check 5,120 outputs against the existing field implementation. Mixed R256 invocation fails with the expected assertion.\n\nThe exact model reaches 260-bit intermediates and covers inputs near2p, canonical roots nearp, zero/one roots and random normalized values. These are deterministic tests plus the range argument, not exhaustive enumeration of all field values. The twiddle-belowp precondition must be discharged for actual FFT call sites before integration.\n\n## Fixed component comparison\n\nRyzen 9950X, one OS thread pinned to CPU2. Thirty-one rotating/reversing fresh-process blocks compare the exact accepted BN254 library with fresh-reference and deferred modes from one new executable. The reference implements the current five-stage column arithmetic. All modes perform three2,000-tile primes and100,000 measured tiles per process, with changing state, then an optimizer barrier. Each tile processes32 vectors/eight lanes. Initial packing, root construction, output conversion and reporting are outside timing. No owned build, profile, archive or other benchmark overlaps unprofiled timing.\n\nThe q/column cases supply synthetic canonical roots for independent32-point transforms and twists. They are arithmetic component geometries, not complete packed prover FFTs or real-key SDK requests.\n\n| q / column | Accepted ns/output | Fresh reference ns/output | Deferred ns/output | Paired / accepted | Paired / fresh | Wins against each |\n| --- | ---: | ---: | ---: | ---: | ---: | ---: |\n");
    for (q, col) in [(1, 0), (256, 173), (16384, 8191)] {
        let row = &r["cases"][q.to_string()];
        let v = &row["variants"];
        let checks = row["checks"].as_array().unwrap();
        s.push_str(&format!(
            "| {q} / {col} | {:.6} | {:.6} | {:.6} | {:.9} | {:.9} | 31/31 |\n",
            v["accepted"]["median_ns_per_element"].as_f64().unwrap(),
            v["fresh"]["median_ns_per_element"].as_f64().unwrap(),
            v["deferred"]["median_ns_per_element"].as_f64().unwrap(),
            checks[0]["paired_median_ratio"].as_f64().unwrap(),
            checks[1]["paired_median_ratio"].as_f64().unwrap()
        ));
    }
    s.push_str("\nThe geometric means of paired case medians are 0.678504088 against accepted and 0.678561858 against fresh, both with31/31 winning block aggregates. Every case passes the3% nonregression and fresh-calibration conditions. The primary q256 case and aggregate must improve against both controls with at least21/31 wins. There is no minimum positive-gain floor, omitted sample, gate change or unchanged timing retry.\n\nThe279 measured processes complete27,900,000 timed tiles and1,674,000 prime tiles. Every final canonical word agrees across modes:190,464 64-bit word comparisons against the two controls, including all256 outputs per process. This is arithmetic verification, not proof verification.\n\n## Linked code and profiles\n\n| Linked repeated-tile function | Bytes | Instructions | Static IFMA sites | RSP operand lines |\n| --- | ---: | ---: | ---: | ---: |\n");
    for row in code["symbols"].as_array().unwrap() {
        s.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            row["variant"].as_str().unwrap(),
            row["size_bytes"],
            row["instructions"],
            row["ifma_sites"],
            row["rsp_operand_lines"]
        ));
    }
    s.push_str("\nThe new function is slightly larger; both loops retain100 static IFMA sites. Source-level multiplication count is unchanged at80 vector products per tile. Fewer static stack references are not a measurement of spill cost or memory traffic. Full disassemblies and call sites are retained.\n\nAfter all latency measurements, one q256/column173 capture per variant records5,000,000 tiles at999Hz user-cycle sampling with build-ID cache updates disabled. The complete component process is captured, including setup, primes and output.\n\n| Capture | Samples | Profiled ns/output | Period share in repeated-tile routine |\n| --- | ---: | ---: | ---: |\n");
    for p in profiles["profiles"].as_array().unwrap() {
        s.push_str(&format!(
            "| {} | {} | {:.6} | {:.4}% |\n",
            p["variant"].as_str().unwrap(),
            p["samples"],
            p["profile_ns_per_element"].as_f64().unwrap(),
            100. * p["symbols"][0]["share"].as_f64().unwrap()
        ));
    }
    s.push_str("\nAll24,971 decoded samples match raw sample-event counts, with one process ID per capture and zero lost-event records. All2,048 final profiled word comparisons against both controls agree. The sample shares locate the work; they are not removable SDK latency fractions or a causal decomposition.\n\n## Scope and retained harness failures\n\nThe first oracle compilation failed on test-only BigUint/reference/index type inference. Those annotations were repaired without changing field arithmetic. The six oracle tests then passed, but the workflow's required-name reader included libtest's `- should panic` suffix while the requested name omitted it. The successful exit, six cases, unfiltered summary and exact output hashes were independently audited and reused; no test or timing was repeated. Both failed workflow records, original source and repairs remain archived.\n\nAll214 final input pins and the170 accepted production hashes pass the closing audit. This stage generates zero SDK proofs and zero Gnark exports. No full FFT, SDK, pressure, stack-peak, minimum-RAM, concurrent-caller, portable-device, AWS, Aeglos or mobile result is established. The routine uses fixed-size temporaries and adds no heap storage; that is not a whole-process memory bound.\n\nThe next experiment must use this byte-pinned arithmetic in the real FFT, verify cached/uncached roots and storage boundaries, and measure all nine shapes plus the physical-core control with fresh SDK proofs and independent Gnark verification. The component pass is insufficient for adoption or for the original10× goal.\n\n## Accepted full-proof context\n\nHistorical accepted local timings below are unchanged. M5 CPU/Metal figures are supplied references, not fresh Apple measurements. None of the eight CPU cases reaches10×.\n\n| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |\n| --- | ---: | ---: | ---: | ---: | ---: |\n");
    for (shape, t, c, m) in [
        ("1x2", 8.594267, 37., 24.),
        ("2x2", 11.697247, 59., 30.),
        ("4x2", 16.158262, 109., 48.),
        ("8x2", 24.477005, 189., 74.),
        ("12x2", 34.386319, 284., 101.),
        ("16x2", 40.867383, 335., 124.),
        ("24x2", 69.265160, 528., 176.),
        ("merge-24x1", 68.150148, 507., 175.),
    ] {
        s.push_str(&format!(
            "| {shape} | {t:.6} | {c:.0} | {:.2}× | {m:.0} | {:.2}× |\n",
            c / t,
            m / t
        ));
    }
    s.push_str("\nAccepted merge-54 is178.891799ms; no M5 reference was supplied for it. The [accepted grouping-module report](../2026-10-25-grouping-module/README.md) remains the production checkpoint.\n\n[Protocol](artifacts/experiment.json), [results](artifacts/component-result.json), [all timing receipts](artifacts/timing), [range derivation](artifacts/range-argument.md), [new arithmetic](artifacts/field-block.rs.txt), [oracles and driver](artifacts/probe.rs), [oracle qualification](artifacts/oracle-qualification.json), [source recovery audit](artifacts/source-audit.json), [linked code](artifacts/codegen.json), [profiles](artifacts/profile-analysis.json), [closing audit](artifacts/closeout.json), [artifact hashes](artifacts/SHA256SUMS). Raw captures and six expanded profile streams remain hash-indexed in the local archive and are omitted from Git.\n");
    fs::write(report.join("README.md"), s).unwrap();
    println!("Wrote component-pass report without full-proof/adoption claim");
}
