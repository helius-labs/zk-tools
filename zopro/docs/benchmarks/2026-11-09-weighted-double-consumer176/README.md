# Complete G1 weighted double-add integration

The actual G1 weighted-reduction entry now has a qualified private implementation using the [previously checked double-add kernel](../2026-11-09-weighted-double-add176/README.md). It preserves both weighted and unweighted group sums, supports partial group tiles, and reuses one original tree/scratch pair per job. **Speed is unmeasured. Production is unchanged and no optimization is adopted.**

Only native G1 with the existing resource policy and at least 2,048 buckets takes the new path. Short, disabled, generic and G2 calls retain the original body. One outer job reduces at most eight 256-bucket groups sequentially using the unchanged affine-tree layer routine, collects nine affine roots per group, then executes eight SIMD double-add steps. Original offset corrections consume both outputs. Partial long groups are padded internally with identity points; inactive SIMD lanes never become outputs. The exact-sized partial-result vector is retained, with safe initialized entries.

The native snapshot has 179 files: 174 accepted originals remain exact, two parent files recover through the recorded edits, and three children contain the reducer, generic-adapted kernel and tests. The original scalar/IFMA field library is reused unchanged. The kernel recovers byte-exactly to its previously qualified source after reversing the type adaptation and constructor/indexing spelling changes. No solver, FFT, scalar split, window geometry, key, preparation table, allocator or production pool changes.

Fixed roots and call objects add **6,176 logical bytes per G1 leaf** to the existing checked required-workspace forecast. The implementation reuses one 511-slot point vector and one original scratch object, rather than retaining eight tree workspaces. The partial-result allocation still has exactly ceil(n/256) entries. This accounting does not establish emitted stack peak, whole-process RSS, minimum RAM or full-proof pressure/admission ownership. The generic/G2 forecast increment is zero. Full SDK resource qualification remains necessary.

Three focused native tests pass, with 125 old tests filtered and inherited. Coverage includes 175 complete weighted-input cases: 90 eligible long cases and 85 original short cases. Independent Arkworks weighted dot products check the entry result, and separate group-level checks verify both intermediate outputs. Cases include full-width generator coefficients, sparse/identity/repeated/opposite/cancellation patterns, 256-boundary tails and all eight group-tail populations. Inputs remain immutable. These fixtures reuse a fixed public point population and are not actual proving-key bucket captures.

All 53 actual workers across 1/4/16/32-worker pools execute the complete consumer with requested 512 KiB stacks. Twelve further calls run through four concurrent OS callers sharing an eight-worker pool. Six workspace edges, checked overflow, resource-disabled original fallback, explicit-zero optional-admission rejection and unchanged G2 bypass pass. The isolated admission check is not a complete proof under memory pressure. Generic no-ASM/no-IFMA compilation passes using the existing qualified generic field; no ARM execution or unchanged field-suite replay is claimed.

Review found a redundant zero-fill of every full input group. Before any benchmark, it was narrowed to only the missing rows in a partial group. The initial source and successful five-command workflow remain available. Four dependent commands then rebuild the tests, rerun the two affected correctness tests, and rebuild final ON. The unchanged generic/OFF/resource/field qualifications are inherited. Coverage is counted once rather than treating the repeated 175 cases or 53 workers as new populations. The final timing candidate is `libs/ON-tail/libzopro_core.rlib`; initial `libs/ON` is history.

All nine workflow commands pass. Before the first workflow, a generic constructor's turbofish spelling was synchronized in generated source and its helper; no scientific command had run. Compiler warnings remain in the original receipts without warning-only rebuilding. OFF and final ON expose the same private read-only complete-consumer API. Release symbol checks find the candidate namespace only in ON and no test route counter in either. The three `nm` invocations are recorded in the tool transcript, not claimed as extra workflow receipts.

The separate same-agent reader checks 379 initial pins using the retained initial source at the two later-edited paths, 384 final pins, all nine command/stream receipts, all 176 recovered originals, the generic kernel adaptation, both-source initialization delta and release identities. It shares cryptographic libraries and does not independently implement field arithmetic or replay tests. [Review](artifacts/review-result.json), [source recovery](artifacts/source-audit.json), [final library identities](artifacts/identities.json), [initialization change](artifacts/tail-change.json).

Next is a prospectively frozen complete-consumer comparison with original accepted-source REF, fresh OFF and final ON, including allocation/drop, root reductions, packing, both inversions, exceptional fallbacks, offset corrections and output normalization. The old full-SDK reference cannot substitute for a component reference. Task parallelism and additional inversion work may offset the vector arithmetic. No latency, instruction reduction, memory peak or full-proof gain follows from these correctness checks. A successful caller result still needs coherent prover/SDK rebuilding and all-eight plus merge54 SDK/Gnark/resource/portable/concurrent qualification before adoption.

This stage adds zero SDK requests or independent proof exports. The canonical private ledger remains 41,104 and accepted production's 176 files remain unchanged. The inherited CPU/Metal comparison uses the latest unchanged accepted fixed-placement REF, with medians of process means; M5 values are supplied references on different hardware. All eight CPU10x targets remain unmet.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU10x target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.832708|37|24|4.724x|3.064x|3.7|
|2x2|11.065989|59|30|5.332x|2.711x|5.9|
|4x2|15.913588|109|48|6.849x|3.016x|10.9|
|8x2|24.254933|189|74|7.792x|3.051x|18.9|
|12x2|34.007120|284|101|8.351x|2.970x|28.4|
|16x2|40.086324|335|124|8.357x|3.093x|33.5|
|24x2|66.467587|528|176|7.944x|2.648x|52.8|
|merge-24x1|65.569872|507|175|7.732x|2.669x|50.7|

[Design](artifacts/design.md), [complete reducer](artifacts/weighted_double.rs), [kernel](artifacts/kernel.rs), [independent tests](artifacts/tests.rs), [continuation](artifacts/next.md), [sources and receipts](evidence.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Private root: `/home/ubuntu/src/zopro-perf-work/weighted-double-consumer176`. Binary libraries/executables, private proving inputs/keys/scalars/proofs, raw perf and full ledgers are excluded. No new AWS/Aeglos, mobile/ARM, huge-circuit, pressure/swap, minimum-RAM or proof-throughput result is added.
