# Semantic expression and inverse-hint census

The read-only census finds small, independently confirmed opportunities to reuse witness expressions. On 1x2, 110 A terms and 185 B terms belong to redundant proportional query classes; on 2x2, the counts are 195 and 303. All 49/72 repeated inverse hints can also be identified directly from their original linear inputs, without retaining the semantic analysis DAG. No production change or proof speedup is established.

## Structural results

|Program|Query|Original terms|Exact or zero terms removable|Proportional or zero terms removable|Proportional reduction|
|---|---|---:|---:|---:|---:|
|1x2|A|21,006|38|110|0.524%|
|1x2|B|30,089|98|185|0.615%|
|2x2|A|36,637|51|195|0.532%|
|2x2|B|52,274|138|303|0.580%|

The proportional column includes exact aliases. No zero query rows were found. Counts describe structural scalar equivalence over arbitrary inputs, rather than coincidences in a particular witness. They do not measure point additions saved: existing per-proof exact-scalar grouping can already recover some exact aliases. Static proportional folding would require a verified key transformation and full-proof measurement.

|Program|Product requests / reused products|Inverse requests / reused hints|Direct exact inverse aliases|Direct scaled inverse aliases|
|---|---:|---:|---:|---:|
|1x2|28,496 / 136|122 / 49|34|15|
|2x2|49,472 / 231|182 / 72|51|21|

Repeated bit outputs and guarded divisions are zero. Boolean-square, complement, Boolean inverse/bit, inverse-involution and constant-high-bit rules have zero hits in both real programs. Those rules have synthetic test coverage; the separate reader certifies their preconditions are absent in these programs.

## Method and verification

The earlier [affine closure census](../2026-10-30-affine-closure176/README.md) treated nonlinear outputs as fresh opaque atoms. This extension interns commutative products of normalized affine forms and deterministic inverse/bit functions. Wire zero is constant one; other inputs remain arbitrary. It preserves zero-inverse and guarded-division semantics, including the zero case that prevents cancelling an unknown x/x to one. It does not infer facts from constraint-only rows or expand general polynomial ideals.

The main five-step xtask workflow links and runs eight unfiltered structural/semantic tests, links the census, and reads two cached solver images. All eight tests pass on the first execution. Coverage includes independent field valuations, scaled and commutative products, inverse zero/involution, bit prefixes and boundaries, Boolean identities, guarded division, complete synthetic programs with known aliases, and parser corruption. The real readers account for 77,980 assignments, 84,796 constraints, 85,144 operations and 677,274 sum records.

A separate two-step workflow links and executes an independently written reader using sorted coefficient vectors. It imports neither primary model module and agrees on every parsed dimension, semantic counter, and full A/B class summary. Both readers use accepted Fr arithmetic and the same image format; this is not an independent field-arithmetic oracle. All seven workflow commands complete successfully, with no successful test or census rerun.

The separate reader additionally compares inverse-hint inputs over original wire IDs, without the semantic DAG. For each alias it verifies that the source operation precedes the target and certifies the exact coefficient relation `scale * target_input = source_input`. This establishes `inv0(target_input) = scale * inv0(source_input)`, including zero. These local signatures recover all 49/72 observed inverse aliases. This supports investigating a small hint-only discovery pass; it does not establish a runtime memory bound, schedule correctness, or net latency gain.

The cached version-2 programs are reused from wire-readiness175, without export or solve. SHA-256 hashes are `0f6851718a9faa3c0d6f2107696f1ef7d4862bec100f7501b4f55f071165a6a4` (1x2, 6,699,124 bytes) and `8693f80ae587e73f51aa99f7dffcef81e7e82cdb9890359a7b140cde70df1288` (2x2, 11,564,476 bytes). Private solver images, alias records, full expression DAGs, executables and libraries are excluded from publication.

The analysis DAG is a disposable offline object. No retained runtime DAG, lookup table, copied key, second solver plan, arbitrary RAM cap or new worker pool is implemented. A future inverse alias opcode must preserve schedule dependencies, validation, serialization and resource accounting. A future static query fold must preserve proof equations and verification. The counts are positive but provide no evidence of a twofold proof improvement.

## Proof timings remain unchanged

These are inherited local 32-worker accepted SDK observations on Ryzen 9950X and supplied M5 references. This census adds no timing, profile, page-fault, RAM, AWS or Aeglos measurement. Accepted source remains 176 files, SDK SHA-256 `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`. The distinct-proof ledger remains 28,847, with zero new solves, proofs or Gnark checks. No production field/core/prover suite is rerun for this read-only analysis.

|Shape|Accepted ms|M5 CPU ms|M5 Metal ms|CPU 10x target ms|
|---|---:|---:|---:|---:|
|1x2|7.928660|37|24|3.7|
|2x2|11.151932|59|30|5.9|
|4x2|16.086386|109|48|10.9|
|8x2|24.179453|189|74|18.9|
|12x2|33.884455|284|101|28.4|
|16x2|40.077381|335|124|33.5|
|24x2|65.850777|528|176|52.8|
|merge-24x1|65.060937|507|175|50.7|

All eight CPU 10x targets remain unmet. Primary is 4.667x faster than the supplied M5 CPU time and 3.027x faster than Metal; reaching 3.7 ms requires another 2.143x reduction. Accepted merge-54x1 is 161.883899 ms without a supplied M5 reference. Accepted primary RSS counter is 132.75 MiB, a diagnostic rather than a precise peak or minimum RAM. The [accepted-build report](../2026-10-30-g2-vector-sign-adoption176/README.md) retains its physical-16 calibration exception.

The latest retained AWS comparison is [historical source175 on c7a.24xlarge](../2026-10-29-proof-pgo-c7a175/README.md): Zopro 11.915591 ms at 32 workers and Aeglos 7.959184 ms at 96 workers, each campaign's best tested configuration. It does not measure current source176. This report adds no cloud, phone, ARM/TEE, large 250/4999, concurrent-throughput or memory-pressure qualification.

## Artifacts

[Design](artifacts/design.md), [hint coverage protocol](artifacts/hint-coverage.md), [all counts](artifacts/analysis.json), [independent certificate](artifacts/review.json), [primary model](artifacts/model.rs), [semantic engine](artifacts/semantic.rs), [semantic tests](artifacts/semantic-tests.rs), [independent reader](artifacts/review.rs), [input provenance](artifacts/inputs.json), [main plan](artifacts/plan.json), [main pins](artifacts/pins.json), [review plan](artifacts/review-plan.json), [review pins](artifacts/review-pins.json), [raw command receipts](artifacts/receipts.json.gz), [accepted source manifest](artifacts/accepted-manifest.json), [publisher](artifacts/publish.rs), and [checksums](SHA256SUMS).

The exact source helpers, setup logs and workflow outputs accompany these artifacts. Receipt archive decompression is checked byte for byte. All input/source/library/executable pins and command stream hashes are checked before publication. The legacy affine/opaque counters in the parser are retained for conservation accounting; `semantic_engine` and `semantic_query_classes` contain the new opportunity measures.

The first publication attempt checked the checksum-file link before creating that file and exited during bookkeeping. The [repair note](artifacts/publication-repair.md) and [original publisher](artifacts/publish-attempt-1.rs) preserve that failure. The corrected publisher writes checksums first. No successful test or census was repeated.
