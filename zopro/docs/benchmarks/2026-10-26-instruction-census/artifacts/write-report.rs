use serde_json::Value;
use std::{fmt::Write, fs, path::Path};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn main() {
    let root = Path::new("/tmp/zopro-instruction-census");
    let summary = read(root.join("instruction-summary.json"));
    let close = read(root.join("closeout.json"));
    let mut s=String::from("# Instruction census of saved CPU profiles\n\nThe offline diagnostic accounts for all 297,412 samples from nine existing warm SDK captures. It adds a Rust `xtask instructions` reader and changes no prover code. No new proofs, timings or profile captures were collected; the preceding overlap candidate remains rejected.\n\n## Input and mapping audit\n\nThe captures are the accepted/OFF/ON primary32, 12x2 and merge54 runs from the [completed product-overlap experiment](../2026-10-26-fused-product-overlap/README.md). All nine raw and compact-stream digests match that experiment's archive index. The extraction uses those user-owned raw copies, the two exact executables, `perf script --show-mmap-events --show-task-events`, `objdump -d -w --insn-width=16`, `nm -S -C --defined-only` and `readelf -lW`. Commands and exit/output receipts are retained. These are reads of saved captures.\n\nThe reader selects each sample PID's executable MMAP2 interval and derives relocation from ELF PT_LOAD file and virtual offsets. The accepted executable's executable segment has file offset 0xe1a00 and virtual address 0xe2a00; guessing relocation from the file offset alone would shift every instruction by 0x1000. Every disassembled instruction is checked against executable bytes, and every sampled executable IP must equal an exact decoded boundary. Addresses are never rounded to nearby instructions. Other addresses remain explicitly outside the executable.\n\n| Capture | Samples | Period weight | Outside executable % | Sampled executable sites | TIDs |\n| --- | ---: | ---: | ---: | ---: | ---: |\n");
    for run in summary["runs"].as_array().unwrap() {
        let r = &run["summary"];
        let total = r["total"]["period"].as_u64().unwrap() as f64;
        writeln!(
            s,
            "| {} | {} | {} | {:.3} | {} | {} |",
            run["run"].as_str().unwrap(),
            r["total"]["samples"],
            r["total"]["period"],
            r["outside_executable"]["period"].as_u64().unwrap() as f64 * 100. / total,
            r["sampled_instruction_sites"],
            r["sampled_tids"]
        )
        .unwrap();
    }
    write!(s,"\nAn independent audit reparses the original compact streams and all per-IP JSONL: {} unique runtime-IP observations across runs conserve {} samples and {} total period weight. All 147 analysis pins pass again. Thirty-six named target-function totals independently match the original symbol-weight records. No sampled executable IP is missing; no unscoped or cross-scope alias range carries sample weight in these inputs. Aliases remain supported explicitly.\n\n",close["unique_runtime_ips_across_runs"],close["all_samples"],close["all_period"]).unwrap();
    s.push_str("## Observed instruction families\n\nEntries below are percentages of period weight, not raw event counts. Scope share uses all samples in that capture. IFMA and RSP entries each use that scope as their denominator; these orthogonal views overlap and must not be added.\n\n| Capture | Named scope | Scope share % | IFMA within scope % | RSP operand within scope % |\n| --- | --- | ---: | ---: | ---: |\n");
    for r in summary["scopes"].as_array().unwrap() {
        let run = r["run"].as_str().unwrap();
        let scope = r["scope"].as_str().unwrap();
        if (run.starts_with("1x2-")
            && matches!(scope, "g1_stream" | "g2_prefix" | "g1_reduce" | "g2_reduce"))
            || (run.ends_with("-accepted") && scope == "g2_prefix")
        {
            writeln!(
                s,
                "| {run} | {scope} | {:.3} | {:.3} | {:.3} |",
                r["share_of_all_percent"].as_f64().unwrap(),
                r["within_scope_percent"]["ifma"].as_f64().unwrap(),
                r["within_scope_percent"]["rsp_operand"].as_f64().unwrap()
            )
            .unwrap();
        }
    }
    s.push_str("\nFor accepted primary G1 streaming, IFMA carries 41.025% of scope weight, vector add/subtract 12.806%, logic 16.541%, shifts 10.250%, moves 9.512% and permutes/broadcasts 4.600%. Accepted primary G2 prefix has 41.380% IFMA, 12.749% vector add/subtract, 12.414% logic, 8.572% shifts, 12.406% moves and 5.842% permutes/broadcasts. Full bins and top functions for every capture are retained.\n\nG2 prefix retains roughly 41–44% IFMA weight in all nine captures and roughly 9–14% on instructions with explicit RSP operands. In primary, G1 streaming has about 4.6–4.8% RSP-operand weight and G2 prefix about 12.9–14.3%. This is a reason to inspect the live values around actual G2 arithmetic. It does not establish that the difference is spills, stalled loads, or avoidable latency. The G2-prefix function also contains packing, record handling and batch bookkeeping.\n\nNamed scope bins are literal symbol-range subsets. Inlined closures and callees outside those symbol ranges stay under `other_executable`, even when their work belongs semantically to MSM or FFT. For example, merge54's leading symbols include G1 BatchAdd, two stored FFT column-leaf specializations, G2 reduction and an unsigned-G1 closure. Absence of a G1-stream range for that capture does not mean absence of G1 work. RBP is reported separately because it can be a general register. LEA address calculation and NOP addressing syntax do not count as memory operands.\n\n## Interpretation and next experiment\n\nThese are user-cycle IP samples, not precise retired-instruction or load-stall events. Sampling may skid. A sample at an instruction is not its exclusive execution cost. Memory-address tags do not count bytes, actual cache/DRAM traffic or register spills. One inherited capture per tuple does not establish a causal comparison between candidates or a removable wall-time budget. The data does not diagnose a general RAM-bandwidth bottleneck.\n\nThe next arithmetic hypothesis is to stream the exact b0+b1 sum one limb at a time in the earlier three-product, two-CIOS-accumulator Fq2 method, while preserving its reduction order and IFMA count. That would remove four long-lived summed-B vectors without a key table, buffer or new representation. The earlier shared implementation won the isolated component and lost in the actual SDK; the phased-Comba follow-up lost even in isolation. Those results remain rejected. This new hypothesis requires checking what LLVM already emits, independent bounds/oracles and a frozen component test, then actual linked G2 callers and full SDK qualification if it passes. A compiler live-range change alone cannot be presented as a 2× prediction. No source for that candidate is part of this diagnostic.\n\n## Tool validation and retained artifacts\n\nThe new command lives in separate ELF, instruction, sample and orchestration modules, with no new dependencies. Thirty unfiltered xtask tests pass, including PIE mapping with unequal file/virtual offsets, malformed and non-x86 ELF rejection, byte checks, prefixes, LEA/NOP/comment handling, RSP/RBP separation, alias overlaps, PID separation, external samples and conservation. Formatting, release build and Clippy with `-D warnings` pass. A real-input negative check with a wrong executable digest fails before creating the output directory.\n\nThe tool was built under an isolated target directory. The old qualified workflow executable, accepted SDK, all 170 accepted prover sources and accepted manifest remain unchanged. No new field, proof, pressure, portable, mobile, large-circuit, AWS or Aeglos qualification is claimed. The complete-SDK results and resource measurements in the parent report retain their original scope.\n\n[Configuration](artifacts/analysis-config.json), [independent audit](artifacts/closeout.json), [all-run bins](artifacts/instruction-summary.json), [primary G2 instruction sites](artifacts/g2-primary-sites.json) and [artifact index](artifacts/artifact-index.json) preserve the evidence. Each run has all per-IP JSONL and function/alias summaries. Metadata streams and full disassembly are local-only, hash-indexed; their extraction commands and receipts are committed. Raw captures remain at the parent archive paths with original digests. Executables and compiler caches are represented by their source and input identities.\n\n## Inherited accepted CPU/Metal comparison\n\nThese are unchanged accepted-production measurements against supplied historical M5 references, not new timing results from this diagnostic.\n\n| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |\n| --- | ---: | ---: | ---: | ---: | ---: |\n");
    let manifest = read("/tmp/zopro-grouping-module/production-source-manifest.json");
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
        let r = &manifest["m5"][shape];
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
    s.push_str("\nAccepted merge54 remains 178.891799 ms; no M5 merge54 reference was supplied. The all-eight CPU 10× goal remains active and unmet (0/8). This diagnostic adopts no prover optimization.\n");
    let dst =
        Path::new("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-26-instruction-census");
    fs::create_dir(dst).unwrap();
    fs::write(dst.join("README.md"), s).unwrap();
}
