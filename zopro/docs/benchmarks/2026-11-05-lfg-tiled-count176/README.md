# Bounded binary-radix LFG Horner count screen

The fixed bounded interpretation passes its **necessary count screen**, with 6,371 fewer primary A generic additions (1.42%) and 22,733 fewer B2 additions (3.59%) after one formation addition per needed triple. This is an optimistic structural result: **18,626 A and 27,919 B2 formation doublings remain uncharged**, and projective combinations/doublings increase. It establishes no CPU speedup, full-query eligibility or adoption. Production remains unchanged.

The earlier [literal LFG screen](../2026-11-05-lfg-power-two176/README.md) rejected large shifted-point tables. This new fixed construction derives width13 ordinary digits from the validated M={1,3} converter, keeps the leading digit direct, preserves the original Phi2 components/two G2 point partitions, and would generate only one tile's needed3P values. No shifted-point table, key-sized derived array, new field backend or native MSM is implemented. It is separate from earlier failed fixed2P/3P/ring variants.

## Fixed geometry and counted work

Original ordinary width12 uses2,048 dense magnitude buckets. Width13 has2,050 compact allowed magnitudes including zero. Their integer weights are nonconsecutive. The model therefore weights the equivalent4,096-position dense vector with holes, using the original256-position weighted-tree recurrence; it does **not** incorrectly apply consecutive compact ordinal weights. A future layout would have to supply those holes into bounded local trees and charge routing/ownership. Top magnitudes are padded to256 for A and1,024 for B2, covering incoming carries without an extra full scalar window.

A transient3P is needed once per nonidentity input point if any ordinary digit, including either G2 component, uses multiplier3. The simple formation account is2P=P+P followed by3P=2P+P: one addition and one doubling. The table below charges the addition only. Formation normalization, input classification/recoding, compact mapping, point copies, exception handling, batching/queue changes and layout/scheduler effects remain excluded. Generic counts assume no point cancellations; token cancellation never changes formal occupancy.

|Primary query|Baseline fill+weight+merges|Candidate before formation|Formation additions|Net generic additions saved|Formation doublings, uncharged|
|---|---:|---:|---:|---:|---:|
|"1x2-A"|448422|423425|18626|6371|18626|
|"1x2-B"|633620|582968|27919|22733|27919|

|Query / route|Baseline fill|Candidate fill|Baseline weight|Candidate weight|Baseline merges|Candidate merges|
|---|---:|---:|---:|---:|---:|---:|
|"1x2-A"|364096|333859|84326|89565|0|1|
|"1x2-B"|572483|518190|40265|43219|20872|21559|

A projective combines increase1,808→3,314 and counted nonidentity doublings1,766→2,839, before formation doublings. B2 increases866→1,585 and846→1,365. These operation types are retained separately; they are not interchangeable CPU units.

Both2×2 populations admit original global scalar grouping and bypass this mechanism. The model does not discard those savings or promote primary's grouping behavior into a property of all circuits/witnesses. Other shapes are not recounted; the preceding all-eight grouping census supplies their grouping status. The fixed necessary gate requires positive net generic-addition savings for both primary queries after one formation addition, with no minimum gain floor. It passes without exclusions, geometry changes or a repeated study. It does not charge the explicit missing work.

## Formation budget and next decision

Under a simplified equal-cost generic-addition model, total3P formation would need to cost less than about1.342 original additions per A point or1.814 per B2 point, even before the extra projective/implementation costs. Equivalently, after the one charged addition, the uncharged doubling budget is about34%/81% of one addition. These are algebraic budgets, not measured timings or a claim about the cost of doubling. The margin is small, particularly for A.

A native follow-up is not yet justified by these counts alone. First qualify a complete formation method and its actual caller cost, or establish that it cannot fit the budget. Borrowed window queues must not outlive a temporary3P tile; flushing or copying to solve that ownership issue adds work absent here. Complete memory admission must include tile points, compact/dense-hole routing, every live window/part and normalization scratch. No arbitrary RAM cap or retained key table is proposed. The current result neither proves a twofold path nor rules out every low-memory use of the binary bucket sets.

## Validation and provenance

Three new model tests pass on first execution: weighted dense/hole trees, direct leading-digit exact reconstruction, and complete synthetic query sums through2,051points with zeros/identities/small/full-width scalars. Original parent algebra tests are inherited and were not repeated. Real retained1×2/2×2 A scalar/identity and B point/word fixtures keep their original hashes and oracle provenance. Current accepted Phi2 source is checked byte-exact against the extracted helper. Scalar decomposition correctness is inherited from the earlier field/curve qualification; this is not a fresh independent BN254 oracle.

Every baseline and candidate real query result equals independent scalar dot products in two small integer additive groups. These are linear representation oracles, not elliptic-curve MSM or proof verification. Baseline fill counts364,096/572,483 agree with the prior complete count screen. A separate reviewer sorts bucket indices instead of using streaming symbolic buckets, derives weighted-tree pair counts from selected block occupancy instead of replaying the recurrence, and independently derives width13 digit decompositions. It reproduces all primary fill/weight/merge/triple counts and the declared gate. All4workflow receipts/stream hashes and pins match;176 production source files remain exact. No successful model/test run is repeated, no helper failure occurs, and no performance timing/profile is added.

## Accepted CPU and Metal comparisons, inherited

Accepted SDK remains `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`. No latency is remeasured here. All eight CPU10× targets remain unmet.

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

Merge54 remains161.883899ms without a supplied M5 reference. Latest full-SDK latency evidence remains epoch-clock176, ledger29,461 unchanged. No new SDK/Gnark proof, AWS/Aeglos, process peak/minimum RAM, pressure/swap, phone/ARM/TEE, 250/4999 or concurrency result is added. Goal all-eight CPU10× remains active.

## Artifacts

[Design](artifacts/design.md), [all counts](artifacts/analysis.json), [study](artifacts/study.rs), [validated converter](artifacts/binary.rs), [accepted Phi2 helper](artifacts/split.rs), [source audit](artifacts/source-audit.json), [input provenance](artifacts/inputs.json), [reviewer](artifacts/review.rs), [review](artifacts/review.json), [protocol](artifacts/plan.json), [pins](artifacts/pins.json), [raw receipts](artifacts/receipts.json.gz), [publisher](artifacts/publish.rs), [checksums](SHA256SUMS). Receipt JSON gzip roundtrips exactly; private scalar/point/key/witness arrays and executable/library binaries are excluded.
