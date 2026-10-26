use serde_json::Value;
use std::{fmt::Write, fs, path::Path};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn main() {
    let root = Path::new("/tmp/zopro-ifma-zero-mask");
    let code = read(root.join("codegen.json"));
    let mut s=String::from("# IFMA zero-accumulator mask audit\n\nThe proposed cleanup removes no AND instructions from the 24 tested field kernels. All 12 eight-lane kernels match the accepted instruction streams after relocation/constant normalization. The compiler already removes the 52-bit cancellation mask in these arithmetic contexts. The audit closes without timing, proofs or adoption; production remains unchanged.\n\n## Instruction contract and proposed change\n\nWith a zero accumulator, IFMA low-half multiplication produces a value below 2^52, so AND with 2^52−1 leaves it unchanged. Both operands contribute only their low 52 bits. This follows from the primary Rust documentation for the [512-bit intrinsic](https://doc.rust-lang.org/core/arch/x86_64/fn._mm512_madd52lo_epu64.html) and [256-bit intrinsic](https://doc.rust-lang.org/core/arch/x86_64/fn._mm256_madd52lo_epu64.html). The inference applies specifically to zero accumulation; it is not permission to remove arbitrary limb or mixed-radix masks.\n\nThe private candidate changes two cancellation sites in the four-lane template and regenerates its eight-lane counterpart. Compile-time `zopro_unmask_m52` selects the unmasked zero-accumulator result only for radix-52 cancellation. The final radix-48 correction, every limb normalization, arithmetic recurrence and field layout remain. Both whole original files recover through documented inverse edits; the other 168 accepted sources match. No key, heap buffer, table, core policy or representation changes.\n\nInitial inspection of the actual accepted G2 caller found two IFMA/AND pairs using the required 48-bit constant. Its ELF virtual address is 0xab98, with little-endian bytes `ff ff ff ff ff ff 00 00`. Matching nearby instructions alone does not establish dataflow or a redundant mask. The full preceding [SDK report](../2026-10-26-streamed-complex-sdk/README.md) retains those caller disassemblies.\n\n## Independent ISA checks\n\nThe raw probe checks 100,032 lane results against independent u128 multiplication: 33,344 four-lane and 66,688 eight-lane results. Boundary and random full-width u64 operands cover ignored high operand bits. Every result has its upper 12 bits clear and equals its masked form. A further 200,064 high-half product checks validate both masked and unmasked feeds into IFMA. Twelve counterexamples demonstrate that a 48-bit result mask changes the value and must remain where the algorithm requires it. These are intrinsic conformance checks, not an independent complete field proof.\n\n| ISA context | Four lanes | Eight lanes |\n| --- | --- | --- |\n");
    for (field, label) in [
        (
            "feed_mask52_vs_raw_equal",
            "Masked/unmasked result fed into IFMA",
        ),
        (
            "return_mask52_vs_raw_equal",
            "Masked/unmasked result returned directly",
        ),
    ] {
        let vals = code["isa_pairs"].as_array().unwrap();
        writeln!(
            s,
            "| {label} | {} | {} |",
            if vals[0][field].as_bool().unwrap() {
                "same normalized instructions"
            } else {
                "different instructions"
            },
            if vals[1][field].as_bool().unwrap() {
                "same normalized instructions"
            } else {
                "different instructions"
            }
        )
        .unwrap();
    }
    s.push_str("\nThe isolated eight-lane return still has an AND52, while both feed contexts eliminate it. That return context is not the prover's cancellation chain. The four-lane 48-bit mask lowers to a blend rather than an AND; an AND count alone does not identify every mask implementation.\n\n## Actual field code comparison\n\nTwo probes link the same 24 exported wrapper signatures against the qualified accepted field library and the freshly built candidate. The matrix covers Fq/Fr, R256/R260, four/eight lanes and multiplication/sum-of-two-products/difference-of-two-products. Field probes are compiled for inspection, not run as field tests or timed.\n\nEvery disassembled instruction is bound to its exact ELF bytes and contiguous function extent. Normalization retains opcodes, registers and immediates while replacing RIP-relative displacements with the referenced constant bytes. There are no calls in these wrappers. Instruction, AND, IFMA, RSP-reference and literal stack-subtraction counts match in all 24 pairs. Sixteen complete normalized streams match; eight four-lane streams differ, so full machine-code equivalence is not claimed for those eight.\n\nEach row applies separately to both Fq and Fr; their static counts agree. Counts below are the same in accepted and candidate.\n\n| Radix | Lanes | Operation | Instructions | AND sites | IFMA sites | Matching normalized pairs |\n| --- | ---: | --- | ---: | ---: | ---: | ---: |\n");
    for radix in [256, 260] {
        for lanes in [4, 8] {
            for op in ["mul", "sum2", "difference2"] {
                let symbol = format!("audit_fq_r{radix}_{op}{lanes}");
                let row = code["functions"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|v| v["label"] == "accepted" && v["symbol"] == symbol)
                    .unwrap();
                let matched = code["pairs"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|v| {
                        let n = v["symbol"].as_str().unwrap();
                        (n == symbol || n == symbol.replace("audit_fq_", "audit_fr_"))
                            && v["normalized_equal"] == true
                    })
                    .count();
                writeln!(
                    s,
                    "| {radix} | {lanes} | {op} | {} | {} | {} | {matched}/2 |",
                    row["instructions"], row["and_sites"], row["ifma_sites"]
                )
                .unwrap();
            }
        }
    }
    s.push_str("\nThe four-lane scheduling/register/encoding differences do not remove the targeted mask operations. For example, R256 multiplication changes from 1113 to 1106 bytes but remains 191 instructions with five AND sites. The fixed screen requires a remaining mask-elimination mechanism before performance work; none is observed. Incidental layout changes are not presented as an optimization or a speed result. The next step needs a material arithmetic/dataflow change rather than this source cleanup.\n\n## Verification and limits\n\nThe native build, generator consistency check, ISA run and both exact-library links pass. All 183 input pins and both linked libraries/probes pass the closing hash audit. Both modified field files recover their complete original digests; accepted production's 170 sources and qualified SDK remain unchanged. The inverse-edit audit initially expected an unformatted one-line template in the generated eight-lane file; its formatter had expanded the conditional. The check was repaired to match that generated formatting while still requiring the original whole-file SHA. The initial audit source and repair note remain; field/build/probe/codegen sources and all 183 pins did not change.\n\nNo timing benchmark, new proof, Gnark export, profile, complete field oracle, SDK trial, portable device, pressure, large-circuit, AWS, Aeglos or minimum-RAM qualification follows from this screen. It describes this native compiler/artifact set and does not claim that every compiler or target removes every redundant mask. No adoption is made.\n\n## Inherited accepted prover context\n\nThese are unchanged accepted timings against supplied historical M5 CPU/Metal references, not measurements from this audit.\n\n| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |\n| --- | ---: | ---: | ---: | ---: | ---: |\n");
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
    s.push_str("\nAccepted merge54 remains 178.891799 ms; no M5 merge54 reference was supplied. The all-eight CPU 10× goal remains active and unmet (0/8).\n\n[Declared screen](artifacts/experiment.json), [candidate edits](artifacts/edits.json), [all code comparisons](artifacts/codegen.json), [disassembly](artifacts/codegen.txt), [ISA source](artifacts/isa.rs), [ISA results](artifacts/build/isa-check/stdout.log), [closing audit](artifacts/closeout.json), [inverse-check repair](artifacts/closeout-repair.json) and [artifact index](artifacts/artifact-index.json). Executables/compiler caches are represented by source, commands and pinned hashes.\n");
    let report =
        Path::new("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-26-ifma-zero-mask");
    fs::create_dir(report).unwrap();
    fs::write(report.join("README.md"), s).unwrap();
    println!("Wrote no-runtime-work mask audit; no new speed claim");
}
