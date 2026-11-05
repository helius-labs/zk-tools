# Late multiplier-plane G2 count screen

The fixed G2 construction passes its necessary count screen: **633,620 → 606,872 generic additions**, saving 26,748 (4.22%). No native implementation, measured latency improvement or adoption is established. Production and accepted performance remain unchanged.

The [preceding tripling trial](../2026-11-05-tripling-formation176/README.md) found per-input tripling too expensive for its declared budgets. This distinct organization uses linearity: split each window by digit multiplier, reduce the multiplier-one plane, release its bucket owners retaining one projective result, then reduce the multiplier-three plane and compute `S1 + 3*S3`. It forms no per-input triples or key-derived point table. It preserves original high→phi→low component order and the two point partitions within each multiplier plane. This differs from the earlier rejected phi-component planes, which merged buckets before one weighting.

## Geometry and scope

Width 13/M13, direct leading digit and dense 4,096 weight positions with holes are fixed before the study. The original 256-position weighted tree supplies integer weights; compact ordinal weighting would be incorrect. The top has multiplier one only and capacities A256/B1024. The two multiplier planes are sequential within each window; different windows may overlap phases and their complete workspace remains unqualified.

G2 was selected prospectively because original weighted additions are 40,265/633,620 (6.4%), versus 84,326/448,422 (18.8%) for G1. G1 is retained as a diagnostic and worsens by 24,623 additions (5.49%). This is not a favorable post-hoc subset of a joint gate. Both 2×2 queries retain original admitted global grouping and bypass the candidate. The all-eight CPU10× goal is unchanged.

|Primary query|Baseline fill|New fill|Baseline weight|New weight|Baseline merges|New merges|Total, old → new|
|---|---:|---:|---:|---:|---:|---:|---:|
|G2, declared candidate|572,483|481,452|40,265|85,546|20,872|39,874|633,620 → 606,872|
|G1, diagnostic only|364,096|296,025|84,326|177,019|0|1|448,422 → 473,045|

G2 projective combinations increase **866 → 3,160**, and doublings **846 → 2,598**. G1 diagnostic combinations increase 1,808 → 6,639 and doublings 1,766 → 5,442. These are different operation types, not interchangeable CPU units. Both routes form zero input triples. The count gate requires strictly fewer G2 generic additions, with no minimum gain floor. It passes; the extra projective, recoding, routing, batching and allocation costs could still exceed the savings.

A prospective `[u16;20]` digit buffer uses **1,203,560 bytes** for the primary G2 query (40 bytes per input), or 840,240 bytes for diagnostic G1. These are hypothetical payloads, not measured allocations or complete live memory. Existing input words remain live; all buckets, queues, part outputs, normalization buffers, scheduler-held state and headers also need admission. No fixed global RAM cap or second retained key/plan is proposed.

## Validation and next action

The first four-step workflow compiles and runs three new unfiltered tests, then compiles and runs the real-fixture screen. All four steps succeed on first execution. Tests cover empty/identity planes, zero and signed/full-width scalars, leading carries, ragged point partitions and complete synthetic queries through 2,051 points. Parent algebra tests and the previous real baseline census are inherited and are not rerun.

Every new real query agrees with independent scalar dot products in two small integer additive groups. These are linear representation checks, **not elliptic-curve or proof oracles**. The original fixture hashes and prior curve/proof provenance remain pinned. An independent reviewer uses sorted bucket keys rather than streaming symbolic points, a separately derived digit converter and closed weighted-tree occupancy formulas. It reproduces every primary fill, weighting, merge, projective-combination and doubling count, verifies baseline reuse, all pins and successful receipt stream hashes, and checks all 176 production files against the accepted manifest. No new proof, latency capture or CPU profile is recorded.

Next: a separate isolated native G2 complete-query prototype, complete curve oracles and conservative resource admission. Preserve current global grouping and fallback before opt-in dispatch; keep borrowed point lifetimes valid. Compare the actual complete caller against the frozen control and fresh OFF arm, including input recoding and output normalization. Full SDK qualification is justified only by that evidence. This necessary count result neither predicts a proof speedup nor reopens per-input tripling or the old phi-plane design.

## CPU and Metal comparison, inherited

Accepted SDK SHA-256 remains `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`. No timings are remeasured. **0/8** shapes meet CPU10×.

|Shape|Accepted ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.928660|37 / 4.667x|24 / 3.027x|3.7|
|2x2|11.151932|59 / 5.291x|30 / 2.690x|5.9|
|4x2|16.086386|109 / 6.776x|48 / 2.984x|10.9|
|8x2|24.179453|189 / 7.817x|74 / 3.060x|18.9|
|12x2|33.884455|284 / 8.381x|101 / 2.981x|28.4|
|16x2|40.077381|335 / 8.359x|124 / 3.094x|33.5|
|24x2|65.850777|528 / 8.018x|176 / 2.673x|52.8|
|merge-24x1|65.060937|507 / 7.793x|175 / 2.690x|50.7|

Merge54 remains 161.883899 ms without an M5 reference. Latest full-SDK latency evidence remains epoch-clock176. The canonical proof ledger stays 29,461; no fresh SDK/Gnark, AWS/Aeglos, 250/4999, pressure/swap, minimum-RAM or concurrency result is added.

## Artifacts

[Design](artifacts/design.md), [counts](artifacts/analysis.json), [model](artifacts/study.rs), [converter](artifacts/binary.rs), [Phi2 helper](artifacts/split.rs), [audit](artifacts/source-audit.json), [inputs](artifacts/inputs.json), [reviewer](artifacts/review.rs), [review](artifacts/review.json), [protocol](artifacts/plan.json), [pins](artifacts/pins.json), [raw receipts](artifacts/receipts.json.gz), [publisher](artifacts/publish.rs), [checksums](SHA256SUMS). Receipt JSON gzip roundtrips exactly; private scalar/point/key/witness arrays and executable/library binaries are excluded.
