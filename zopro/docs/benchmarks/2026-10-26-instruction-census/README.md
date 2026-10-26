# Instruction census of saved CPU profiles

The offline diagnostic accounts for all 297,412 samples from nine existing warm SDK captures. It adds a Rust `xtask instructions` reader and changes no prover code. No new proofs, timings or profile captures were collected; the preceding overlap candidate remains rejected.

## Input and mapping audit

The captures are the accepted/OFF/ON primary32, 12x2 and merge54 runs from the [completed product-overlap experiment](../2026-10-26-fused-product-overlap/README.md). All nine raw and compact-stream digests match that experiment's archive index. The extraction uses those user-owned raw copies, the two exact executables, `perf script --show-mmap-events --show-task-events`, `objdump -d -w --insn-width=16`, `nm -S -C --defined-only` and `readelf -lW`. Commands and exit/output receipts are retained. These are reads of saved captures.

The reader selects each sample PID's executable MMAP2 interval and derives relocation from ELF PT_LOAD file and virtual offsets. The accepted executable's executable segment has file offset 0xe1a00 and virtual address 0xe2a00; guessing relocation from the file offset alone would shift every instruction by 0x1000. Every disassembled instruction is checked against executable bytes, and every sampled executable IP must equal an exact decoded boundary. Addresses are never rounded to nearby instructions. Other addresses remain explicitly outside the executable.

| Capture | Samples | Period weight | Outside executable % | Sampled executable sites | TIDs |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1x2-w32-accepted | 16215 | 84650858310 | 1.988 | 5635 | 33 |
| 1x2-w32-off | 16094 | 85849195249 | 2.097 | 5664 | 33 |
| 1x2-w32-on | 16634 | 87227331392 | 2.069 | 5800 | 33 |
| 12x2-w32-off | 42094 | 219085752562 | 1.138 | 9982 | 33 |
| 12x2-w32-on | 41941 | 219236230142 | 0.931 | 10050 | 33 |
| 12x2-w32-accepted | 43049 | 220795373627 | 0.956 | 10208 | 33 |
| merge-54x1-w32-on | 39973 | 200788623113 | 0.916 | 7817 | 33 |
| merge-54x1-w32-accepted | 41366 | 206401061828 | 0.755 | 7940 | 33 |
| merge-54x1-w32-off | 40046 | 203594466962 | 0.760 | 7754 | 33 |

An independent audit reparses the original compact streams and all per-IP JSONL: 72302 unique runtime-IP observations across runs conserve 297412 samples and 1527628893185 total period weight. All 147 analysis pins pass again. Thirty-six named target-function totals independently match the original symbol-weight records. No sampled executable IP is missing; no unscoped or cross-scope alias range carries sample weight in these inputs. Aliases remain supported explicitly.

## Observed instruction families

Entries below are percentages of period weight, not raw event counts. Scope share uses all samples in that capture. IFMA and RSP entries each use that scope as their denominator; these orthogonal views overlap and must not be added.

| Capture | Named scope | Scope share % | IFMA within scope % | RSP operand within scope % |
| --- | --- | ---: | ---: | ---: |
| 1x2-w32-accepted | g1_reduce | 5.642 | 34.657 | 12.070 |
| 1x2-w32-accepted | g1_stream | 15.772 | 41.025 | 4.773 |
| 1x2-w32-accepted | g2_prefix | 14.911 | 41.380 | 12.927 |
| 1x2-w32-accepted | g2_reduce | 5.261 | 18.223 | 18.196 |
| 1x2-w32-off | g1_reduce | 5.169 | 29.165 | 8.660 |
| 1x2-w32-off | g1_stream | 16.463 | 40.614 | 4.691 |
| 1x2-w32-off | g2_prefix | 15.118 | 43.964 | 14.348 |
| 1x2-w32-off | g2_reduce | 5.227 | 18.633 | 23.551 |
| 1x2-w32-on | g1_reduce | 5.540 | 32.023 | 14.130 |
| 1x2-w32-on | g1_stream | 16.314 | 40.116 | 4.609 |
| 1x2-w32-on | g2_prefix | 14.514 | 40.750 | 13.582 |
| 1x2-w32-on | g2_reduce | 5.374 | 15.062 | 22.131 |
| 12x2-w32-accepted | g2_prefix | 5.569 | 43.410 | 12.106 |
| merge-54x1-w32-accepted | g2_prefix | 2.798 | 43.174 | 11.244 |

For accepted primary G1 streaming, IFMA carries 41.025% of scope weight, vector add/subtract 12.806%, logic 16.541%, shifts 10.250%, moves 9.512% and permutes/broadcasts 4.600%. Accepted primary G2 prefix has 41.380% IFMA, 12.749% vector add/subtract, 12.414% logic, 8.572% shifts, 12.406% moves and 5.842% permutes/broadcasts. Full bins and top functions for every capture are retained.

G2 prefix retains roughly 41–44% IFMA weight in all nine captures and roughly 9–14% on instructions with explicit RSP operands. In primary, G1 streaming has about 4.6–4.8% RSP-operand weight and G2 prefix about 12.9–14.3%. This is a reason to inspect the live values around actual G2 arithmetic. It does not establish that the difference is spills, stalled loads, or avoidable latency. The G2-prefix function also contains packing, record handling and batch bookkeeping.

Named scope bins are literal symbol-range subsets. Inlined closures and callees outside those symbol ranges stay under `other_executable`, even when their work belongs semantically to MSM or FFT. For example, merge54's leading symbols include G1 BatchAdd, two stored FFT column-leaf specializations, G2 reduction and an unsigned-G1 closure. Absence of a G1-stream range for that capture does not mean absence of G1 work. RBP is reported separately because it can be a general register. LEA address calculation and NOP addressing syntax do not count as memory operands.

## Interpretation and next experiment

These are user-cycle IP samples, not precise retired-instruction or load-stall events. Sampling may skid. A sample at an instruction is not its exclusive execution cost. Memory-address tags do not count bytes, actual cache/DRAM traffic or register spills. One inherited capture per tuple does not establish a causal comparison between candidates or a removable wall-time budget. The data does not diagnose a general RAM-bandwidth bottleneck.

The next arithmetic hypothesis is to stream the exact b0+b1 sum one limb at a time in the earlier three-product, two-CIOS-accumulator Fq2 method, while preserving its reduction order and IFMA count. That would remove four long-lived summed-B vectors without a key table, buffer or new representation. The earlier shared implementation won the isolated component and lost in the actual SDK; the phased-Comba follow-up lost even in isolation. Those results remain rejected. This new hypothesis requires checking what LLVM already emits, independent bounds/oracles and a frozen component test, then actual linked G2 callers and full SDK qualification if it passes. A compiler live-range change alone cannot be presented as a 2× prediction. No source for that candidate is part of this diagnostic.

## Tool validation and retained artifacts

The new command lives in separate ELF, instruction, sample and orchestration modules, with no new dependencies. Thirty unfiltered xtask tests pass, including PIE mapping with unequal file/virtual offsets, malformed and non-x86 ELF rejection, byte checks, prefixes, LEA/NOP/comment handling, RSP/RBP separation, alias overlaps, PID separation, external samples and conservation. Formatting, release build and Clippy with `-D warnings` pass. A real-input negative check with a wrong executable digest fails before creating the output directory.

The tool was built under an isolated target directory. The old qualified workflow executable, accepted SDK, all 170 accepted prover sources and accepted manifest remain unchanged. No new field, proof, pressure, portable, mobile, large-circuit, AWS or Aeglos qualification is claimed. The complete-SDK results and resource measurements in the parent report retain their original scope.

[Configuration](artifacts/analysis-config.json), [independent audit](artifacts/closeout.json), [all-run bins](artifacts/instruction-summary.json), [primary G2 instruction sites](artifacts/g2-primary-sites.json) and [artifact index](artifacts/artifact-index.json) preserve the evidence. Each run has all per-IP JSONL and function/alias summaries. Metadata streams and full disassembly are local-only, hash-indexed; their extraction commands and receipts are committed. Raw captures remain at the parent archive paths with original digests. Executables and compiler caches are represented by their source and input identities.

## Inherited accepted CPU/Metal comparison

These are unchanged accepted-production measurements against supplied historical M5 references, not new timing results from this diagnostic.

| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1x2 | 8.594267 | 37.0 | 4.305× | 24.0 | 2.793× |
| 2x2 | 11.697247 | 59.0 | 5.044× | 30.0 | 2.565× |
| 4x2 | 16.158262 | 109.0 | 6.746× | 48.0 | 2.971× |
| 8x2 | 24.477005 | 189.0 | 7.722× | 74.0 | 3.023× |
| 12x2 | 34.386319 | 284.0 | 8.259× | 101.0 | 2.937× |
| 16x2 | 40.867383 | 335.0 | 8.197× | 124.0 | 3.034× |
| 24x2 | 69.265160 | 528.0 | 7.623× | 176.0 | 2.541× |
| merge-24x1 | 68.150148 | 507.0 | 7.439× | 175.0 | 2.568× |

Accepted merge54 remains 178.891799 ms; no M5 merge54 reference was supplied. The all-eight CPU 10× goal remains active and unmet (0/8). This diagnostic adopts no prover optimization.
