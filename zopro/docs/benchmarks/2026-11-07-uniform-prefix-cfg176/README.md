# Uniform-prefix control-flow audit

The previous whole-function instruction screen was too conservative to settle this candidate. **Every matched vector loop is smaller in both curves**, despite G1's larger total function. This new evidence selects the first complete-SDK measurement of the same qualified binaries. It establishes no latency gain or adoption. The [original static gate failure](../2026-11-07-uniform-prefix-native176/README.md) remains recorded unchanged.

## New evidence

One fresh Rust audit reads all four saved consumer disassemblies, without another symbol listing, decode, native build, test, model or proof. It partitions every instruction into basic blocks, resolves direct branches, computes reachability/dominators, enumerates all natural backedges and merges loops sharing a header. The method and selection condition were frozen before execution.

All four graphs resolve completely, with no indirect/unresolved branches or external branch exits. They contain 659 blocks, 631 reachable blocks and 18 natural loops. All loops are nested or disjoint. The IFMA-bearing outer loops form the same three G1 and two G2 roles in each arm. The table retains every such role, including the forward path containing a buffer-growth call.

|Vector-loop role|Instructions OFF → ON|IFMA sites, unchanged|Left-4 / right-48 shifts OFF → ON|ZMM stack operand rows OFF → ON|
|---|---:|---:|---:|---:|
|G1 forward, no growth call|670 → 630|200|12 / 10 → 0 / 0|10 → 10|
|G1 forward, with growth call|789 → 740|200|12 / 10 → 0 / 0|82 → 80|
|G1 reverse|1,053 → 1,017|400|24 / 20 → 12 / 10|20 → 20|
|G2 forward|714 → 662|250|12 / 10 → 0 / 0|30 → 52|
|G2 reverse|3,590 → 3,450|1,200|60 / 50 → 24 / 20|265 → 261|

G1's union of vector-loop sites shrinks 2,512→2,387, while its complement grows 1,916→2,071. The net total is still 4,428→4,458. G2's union shrinks 4,304→4,112 and complement grows 4,287→4,429, producing the original 8,591→8,541 total. Neither complement contains IFMA sites. The new root-inverse implementation is an outlined call and remains outside these selected functions' instruction counts.

Complete vector batches call the corrected scalar-root inverse. Fallback inversion and ordinary scalar-tail reversal apply the four vector doublings afterwards. Consequently, charging total function size as repeated per-group work can reject a useful loop change. The new location evidence supports reconsideration; it does not prove those outside paths are never executed or price their cost.

Both old and new evidence retain the G2 frame increase 5,952→6,720 bytes and increased forward-loop stack operands. IFMA counts stay unchanged. Calls remain inventoried, not priced as zero work. Natural-loop counts are static site counts, not measured iteration frequencies, dynamic instructions, spills, critical-path costs or speed estimates. Unwinding is not modeled. No twofold gain is inferred.

## Independent review and decision

A separate reader reconstructs every block's successors and counts from retained text, checks instruction conservation, and independently verifies all dominators through **631 node-deletion reachability searches**. It checks every natural-loop latch/body and all five paired roles, 11 input pins, 176 accepted production hashes and both successful command receipts. Both the audit and review pass on first execution. This is not a second disassembler or native arithmetic oracle.

The prior native correctness evidence is inherited: 500,256 Arkworks-checked SIMD outputs, 8,206 scalar-root cases, 127 OFF/127 ON core tests and eight ON prover tests. No successful qualification is replayed. Source, layout, workers, compiler flags and binaries remain fixed.

This corrects an overly aggressive screening heuristic rather than converting its failed gate to a pass. A prospectively frozen, first whole-proof pilot will compare accepted/OFF/ON after fresh independent Gnark preflight for all nine shapes. It includes both curves, complete inverse/fallback work and verification/serialization. Gain must beat both controls; unchanged-control calibration must pass. No gain floor, exclusions, extra blocks or favorable curve/worker subset applies. Positive pilot results still require all-eight plus merge54 and resource qualification before adoption.

## Research and unchanged baseline

Primary-source research rechecked [truncated Montgomery multiplication](https://arxiv.org/html/2410.18129v1), [parallel LogJumps](https://eprint.iacr.org/2026/1324), [multi-limb matrix multiplication](https://www.texmacs.org/joris/amx/amx-v2.html) and [vectorized SQIsign](https://arxiv.org/html/2608.13948v1). Truncated REDC, LogJumps and pairwise radix51 already have recorded local trials and are not repeated. SQIsign's reduction uses its small-coefficient prime form; its Montgomery differential ladders require input relationships absent from arbitrary BN254 bucket addition. No external benchmark speedup is transferred to this prover.

Accepted production176/SDK6d83 remain unchanged. Local 32-worker **1×2 remains 7.915095 ms**, versus supplied M5 CPU 37 ms and Metal 24 ms; the CPU 10× limit is 3.7 ms. [All-eight CPU/Metal comparisons](../2026-11-07-uniform-prefix-native176/README.md#accepted-comparisons-unchanged) remain inherited and all eight targets unmet. Merge54 remains 161.718956 ms without an M5 reference. Canonical independently verified proof ledger stays 31,699. No new AWS/Aeglos, RAM, mobile, pressure/swap or large-circuit result is added by this audit.

[Artifacts](artifacts/) preserve source, frozen design, full graphs/loops, analysis, independent review, plans, pins and raw text/JSON receipts. Existing binaries and decodes remain private with hashes; no key, witness, proof or public-input payload is published.
