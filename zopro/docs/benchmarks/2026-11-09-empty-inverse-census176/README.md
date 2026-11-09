# Empty ordinary-layer inversion census

**Empty layers exist, but are sparse in this matrix. No optimization or speed result is selected.** Primary 1×2 has 27 empty G1 layers among 3,688 G1 tree-layer calls, with no empty G2 layers among 3,935 calls. Larger cases have no empty G1 layers and generally one empty G2 layer. The accepted build remains unchanged at its last measured 7.803404 ms.

The current `reduce_layer` collects ordinary distinct-X nonidentity pairs, then prepares the unchanged inversion path even when that set is empty. Such a layer has unit initial chains, no full group and no scalar tail. Its ordinary inverse results are unused; exceptional compaction still needs to run. This observation tests the frequency of that condition, without adding the proposed branch or changing any arithmetic.

A private cfg-only module records metadata at that exact source boundary. All 176 accepted originals recover through two reversible core edits; 174 remain byte-exact, and a diagnostic child is added. Field library 1200, point formulas, queues, scratch, geometry, allocator and forecasts remain original. No proving-key table, point copy, retained plan, pool or architectural RAM cap is added.

The counter state is 264 fixed bytes. Each row tracks calls, zero/short/full classification, pair/tail/group totals, maximum pairs and eight residue bins. Overflow is visible and remains false. Global atomics can perturb scheduling; the data are counts, not service times or causal latency fractions. Reset and snapshot occur between joined single-proof calls, after loading. This diagnostic state is not safe for overlapping proof epochs and is not a production architecture proposal. No field values, scalars, coordinates or keys are logged.

All nine shapes run at 32 workers, with primary also at 16. Each process records three warm epochs and one final epoch; all ten observation processes finish before ten independent Gnark checks. The table gives final-epoch counts. Raw metadata for all 40 epochs is retained.

|Shape|Workers|G1 calls|G1 empty|G2 calls|G2 empty|Combined empty / tree calls|
|---|---:|---:|---:|---:|---:|---:|
|1x2|32|3688|27|3935|0|27 /7623 (0.3542%)|
|2x2|32|4786|0|4134|0|0 /8920 (0.0000%)|
|4x2|32|6389|0|1817|1|1 /8206 (0.0122%)|
|8x2|32|6945|0|1925|1|1 /8870 (0.0113%)|
|12x2|32|7432|0|2030|1|1 /9462 (0.0106%)|
|16x2|32|7841|0|2080|1|1 /9921 (0.0101%)|
|24x2|32|10054|0|2223|1|1 /12277 (0.0081%)|
|merge-24x1|32|8572|0|2165|1|1 /10737 (0.0093%)|
|merge-54x1|32|14102|0|2505|1|1 /16607 (0.0060%)|
|1x2|16|3688|27|3935|0|27 /7623 (0.3542%)|

All four observed epochs have identical counter rows within each case: **true**. This is an observed count invariant, not a general claim about randomized proofs.

The zero-opportunity closure criterion is therefore false. Primary has 27/7,623 combined empty tree calls (0.3542%); that fraction is neither the fraction of all field inversions nor a removable wall-time bound. The earlier G2 volume census's zero-empty result for primary inputs is consistent with this run. New coverage adds G1 and the larger shapes.

These sparse counts do not demonstrate the large gain required by the full objective. No empty-layer optimization is implemented, adopted or selected by this census. It is not a failed native performance trial. If a future branch is selected, it still needs complete exception/generic/resource/concurrency checks and a frozen matched all-eight plus merge-54 performance comparison. Neither a favorable shape nor a new short-count threshold is selected. The closed active-chain, root-export/lazy-codec/broker and scalar-state trials remain unchanged.

Seven build/qualification commands complete successfully on first execution: four coherent core/prover/zolana/SDK artifacts, counter test build/run and a generic observer guard. Two new counter tests cover 32 exact metadata cases, reset/visible overflow, 53 actual broadcast workers across 1/4/16/32-worker pools with requested 512 KiB stacks, and four callers sharing an eight-worker pool; all 7,296 concurrent record updates match a separate integer oracle. The generic build is not physical ARM/mobile execution. Original field and arithmetic suites are inherited unchanged, not replayed.

The source/receipt checks bind the one observation site and reversible driver reset/export edits to the exact compiled artifacts. The frozen separate same-agent reader reconstructs 40 epochs, all 20 capture receipts, metadata conservation and the ten fresh export set differences without importing the analysis helper. Both readers pass. Manual preparation/reader/publication helper invocations do not all have xtask receipts. One unused helper import warning is retained without a cleanup rebuild.

The 40 SDK requests are internally verified; ten final 164-byte proofs independently verify with fresh proof hashes and expected image/public bindings. Canonical inventory advances **43,073 → 43,083**. Requests are not distinct independently exported proofs. All worker placement checks and counter conservation checks pass.

Saved `SAMPLE`/rusage lines are instrumented functional output and are not analyzed as performance. No PMU/profile capture or new speed, RAM-pressure, minimum-RAM, phone, huge-circuit, AWS/Aeglos or concurrent-throughput result is added. No successful old model, test suite, proof, capture or decoder is replayed.

The table below carries forward accepted results from the [last complete performance comparison](../2026-11-09-initial-unit-sdk176/README.md), with supplied historical M5 CPU/Metal references on different hardware. The observer SDK is never treated as a performance candidate or production baseline.

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.803404|37.0 /4.742x|24.0 /3.076x|3.7|
|2x2|10.940319|59.0 /5.393x|30.0 /2.742x|5.9|
|4x2|15.809849|109.0 /6.894x|48.0 /3.036x|10.9|
|8x2|24.043743|189.0 /7.861x|74.0 /3.078x|18.9|
|12x2|33.720355|284.0 /8.422x|101.0 /2.995x|28.4|
|16x2|39.753010|335.0 /8.427x|124.0 /3.119x|33.5|
|24x2|65.983137|528.0 /8.002x|176.0 /2.667x|52.8|
|merge-24x1|65.378103|507.0 /7.755x|175.0 /2.677x|50.7|

The full verified CPU 10× goal remains active and unmet, **0/8**. Accepted primary peak RSS remains 132.50 MiB; merge 54 remains 162.189200 ms without a supplied M5 reference.

[Counts and epochs](artifacts/analysis.json), [separate reader](artifacts/review-result.json), [source recovery](artifacts/source-audit.json), [prospective review](artifacts/prospective-review.json), [identities](artifacts/identities.json), [counter](artifacts/census.rs), [design](artifacts/design.md), [decision](artifacts/decision.json), [continuation](artifacts/next.md), [source and receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS).
