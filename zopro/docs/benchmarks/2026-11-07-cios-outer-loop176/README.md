# Compact CIOS outer-loop codegen screen

The fixed Rust loop recipe does not retain a compact Montgomery row loop after optimization. Its two selected Fq wrappers occupy **2,946 bytes**, versus **2,944 bytes** for both frozen accepted and fresh OFF controls. The declared structural screen fails, so the experiment closes before arithmetic execution or timing. Production is unchanged.

## Change and decision

The private field snapshot replaces the explicitly expanded first four radix52 CIOS rows with `for i in 0..4 { row!(i) }` only for eight-lane Fq mixed-R256 products. The row body, final radix48 cancellation, carry correction and representation stay the same. The macro index accepts an expression rather than only a literal. Fr, four-lane and portable paths retain their original selection. No field annotation, function-call boundary, compiler option, key table/copy, heap scratch, retained plan or production pool is added.

This differs from the rejected [fused-Fq outlining trial](../2026-10-29-g2-fused-outline175/README.md), which introduced per-product calls. The [hardware calibration](../2026-11-06-instruction-calibration176/README.md) and prior arithmetic/layout/chain experiments are inherited, not rerun. Code footprint is a hypothesis here; no frontend bottleneck or removable CPU time is established.

The prospective screen requires both selected ON Fq8 wrappers to retain an IFMA-containing backward row loop, reduce their combined size against both controls and add no product-helper calls. LLVM fully expands the proposed row loop. Ordinary/fused wrappers still have 100/150 static IFMA sites and 191/278 decoded instructions. Their retained correction branch has no IFMA sites in its backward address range. All selected wrappers have zero calls.

|Selected mixed-R256 function|Accepted bytes|Fresh OFF bytes|ON bytes|Static IFMA sites, all arms|
|---|---:|---:|---:|---:|
|Fq8 product|1,193|1,193|1,194|100|
|Fq8 fused two-product sum|1,751|1,751|1,752|150|
|Fr8 product control|1,193|1,193|1,193|100|
|Fq4 product control|1,113|1,113|1,113|105|

The Fr8 and Fq4 control function bytes also match across all three binaries. Function sizes and instruction sites describe these standalone wrappers, not the complete G2 consumer, dynamic instruction counts or proof latency. No unroll flag, opaque loop bound, attribute, iteration-count change, favorable subset or benchmark rescue is selected.

## Evidence and limits

Two new field libraries and three coherent probe consumers were compiled. Frozen accepted uses the existing field1200 library; OFF and ON share the isolated source. Actual consumer commands include O3, native CPU, fat LTO and one codegen unit. Twelve selected disassemblies are each captured once and reused by analysis. No probe or changed field arithmetic was executed.

The analyzer verifies all 176 current accepted source hashes. Sixteen of the 18 copied field-package files are exact; reversing the edits recovers both remaining files byte-for-byte. The unchanged official IFMA generator produced the eight-lane source. No Python runner was introduced. The analyzer reads ELF64 symbol/section tables for function sizes and function-byte hashes, binds each decoded instruction to its function address range, inspects backward branches and checks command argv, exit status and stream hashes. A separate [readelf comparison](artifacts/metadata-review.json) agrees on all twelve function sizes; it is not a second instruction decoder.

Seventeen field-build, consumer-build and decode commands succeed. The initial probe compile failed because its display code requested a nonexistent little-endian serialization method. A separate corrected source uses the public big-endian method and explicitly instantiates mixed-R256 types; the original source and failure remain. Both successful field libraries are reused, and only failed/unexecuted consumer/decode stages continue. The offline analyzer's compile and execution pass on their first attempts. There is no arithmetic correctness qualification, native field/core/prover test, profile or performance acceptance claim for this candidate.

Latest accepted local 32-worker **1×2 remains 7.915095 ms**, versus supplied M5 CPU **37 ms** and Metal **24 ms**. The all-eight verified CPU10× goal stays active and unmet, **0/8**; the full inherited comparison table is in the [real-norm report](../2026-11-07-g2-real-norm-z176/README.md). Merge54 remains 161.718956 ms, without a supplied M5 reference. The distinct-proof ledger is unchanged at 31,387. This screen adds zero proofs, timing observations, AWS/Aeglos comparisons or RAM/pressure/mobile results.

Continue with a distinct whole-path work or dependency change. The [field-volume ledger](../2026-11-07-g2-field-volume176/README.md) still prioritizes ordinary accumulation in the counted vector work; this failed codegen recipe supplies no new runtime candidate.

[Analysis](artifacts/analysis.json), [analyzer](artifacts/analyze.rs), [declared screen](artifacts/design.md), [source patch](artifacts/source.patch), [corrected probe](artifacts/probe-fixed.rs), [original failed probe](artifacts/probe.rs), plans/pins and exact workflow streams retain the experiment. Executables and libraries remain local, identified by hashes; no key, witness, proof or binary payload is published.
