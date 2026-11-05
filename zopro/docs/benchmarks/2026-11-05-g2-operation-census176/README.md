# G2 operation census for the late multiplier trial

The rejected late-multiplier geometry changes the work mix substantially: primary ordinary-window projective spills increase **1,178→36,623 (31.09×)** and normalized projective buckets **429→20,820 (48.53×)**, despite fewer mathematical additions. Equal-x exceptions remain zero in both ordinary paths. Reduction scalar tails also increase **9,838→40,290 (4.10×)**. These are exact branch/work counts on retained real inputs, not sampled attribution or a new latency result.

This explains a limitation of the earlier [necessary count screen](../2026-11-05-lfg-multiplier-planes176/README.md), which saved4.22%of generic additions but did not price their execution type. The [complete-query trial](../2026-11-05-late-m3-query176/README.md) remains without adoption; its corrected same-binary ON/OFF observations were2.754×at16workers/2.594×at32, with reference calibration also failing. This census does not repair that calibration, establish accepted-baseline timing, reopen the fixed candidate or change production.

## Exact observations

One complete B query per process,1×2/2×2, original dispatcher OFF/late-M3 requested ON, one workerCPU2/32workersCPUs0–31:8processes/8complete queries. Every output matches its retained independent Arkworks complete-MSM hash, key points remain unchanged, and actual route313 occurs only for primaryON. Both2×2 modes retain original admitted global grouping. There are no priming or repeated timing passes.

All ordinary per-window/component counts and aggregate reduction counts match exactly between1and32workers. Grouped2×2 OFF/ON counts also match. Original20ordinary window-part records become36in the candidate (two sequential multiplier planes); the indexed leading window is excluded from Window records and included in global reducer observations. All192Window records across8processes satisfy independent conservation checks.

|Primary ordinary-window work|Original|Late multiplier|
|---|---:|---:|
|Nonzero nonidentity input terms|558614|502887|
|Initial affine bucket writes|40960|73619|
|Batched affine updates|516476|392645|
|Nonempty borrowed affine batches|2605|1148|
|Full batches|2565|1076|
|Borrowed affine scalar-tail pairs|148|277|
|Queued arrivals|30907|58930|
|Queued arrivals drained into affine path|29729|22307|
|Queue points sent to projective path|1178|36623|
|Queue-overflow flushes|2|69|
|Adds to existing projective accumulators|749|15803|
|New projective accumulators|429|20820|
|Equal-x same-sign cases|0|0|
|Equal-x opposite cases|0|0|

The original200-point batch/199queue limit becomes352/351under the existing width13heuristic; this census keeps both policies unchanged. Overflow flushes rise2→69. The candidate's two multiplier planes have similar spill counts:18,320for multiplier1and18,303for multiplier3. A favorable plane is not selected. All projective arrivals in these ordinary windows come from queue flushing, with no measured equal-x contribution. The observer does not change queue order or arithmetic.

|Primary reduction workload, aggregate|Original|Late multiplier|
|---|---:|---:|
|Weighted reductions|11|19|
|Weighted bucket slots|20544|74752|
|Nonidentity weighted buckets|20536|37310|
|Indexed leading-component reductions|4|4|
|Indexed input point references|55277|55749|
|Total indexed sign bins|512|8192|
|Affine reduction-layer invocations|3935|17884|
|Reduction generic pairs|95094|137730|
|Reduction vector pairs|85256|97440|
|Reduction scalar-tail pairs|9838|40290|
|Reduction identity/equal-x pair slots|15|61038|
|Layers with zero generic pairs|0|16|

Global reduction-layer counters aggregate weighted, indexed and grouping work; they do not label every stage independently. Identity/equal-x pair slots combine those conditions and are not all exceptional additions. Scalar tails are pairs outside complete SIMD groups, not a count of base-field multiplications. Layer and batch counts do not measure inverse service time. The candidate has less borrowed affine work (516,476→392,645,−23.98%), but more scalar projective work, normalization, sparse tree slots and scalar tails. This is a change in work performed, beyond a RAM-capacity explanation; no exclusive causal decomposition of the prior wall time is claimed.

## Fidelity and validation

The isolated snapshot adds local counters to the exact original Window branches and its candidate feed, emits bounded diagnostic rows at finish, and adds low-frequency weighted/indexed/reduction-layer observations. Counter locking, row allocation and additional reads perturb scheduling and memory. **No instrumented latency, RSS, minimum RAM, PMU or speed result is reported.** The128-row observer buffer is temporary diagnostic storage, not an architectural memory cap or production allocation policy. Point formulas, queue order, grouping, widths, normalization, weighted/reduction algorithms and resource admission remain unchanged from the parent trial.

One new focused native test passes on first execution. It verifies complete generator-sum bucket outputs under repeated/opposite/identity/collision-overflow inputs, every counter identity and a weighted-tree oracle. The128previously passing parent tests are inherited and filtered, not rerun or claimed as fresh. The step's inherited name `full-core` does not change that scope. Coherent native libraries are frozen; the driver link's actual argv is checked for-O,native CPU targeting,fatLTO andoneCGU **before** all eight processes. Extra SHA2flags use `additional_rustc_flags`, preserving optimization defaults.

The reviewer checks264pins,12successful command receipts and stream hashes, all177parent source bytes recovered by reversing observer edits,176production files unchanged, raw per-component histograms/conservation, aggregate sums, cross-thread equality and the original grouped fallback. Ordinary baseline counts exactly reproduce the older independently checked width12/parts2/batch200model (516,476affine updates,1,178projective arrivals,749accumulator adds,429normalizations,2,605batches). The new complete curve census was not available for the width13/M13geometry in those earlier reports. Existing queue scan/drain variants remain closed.

The conservation identities distinguish arrival processing from equivalent mathematical additions: terms = first writes + affine updates + flushed points + equal-x same/opposite cases; queued = drained + flushed; projective arrivals = initializations + accumulator adds; each batch histogram reconstructs affine work, full batches and scalar/vector lanes; final normalizations equal projective initializations. The simple generic-addition reduction remains true, but it is insufficient as a native cost predictor.

## Next decision

The missing cost categories are now observed directly. Another undifferentiated-addition screen should not trigger a native candidate without counting spills, normalization, scalar/vector work and reduction geometry. These observations support closing this fixed late-M3implementation; they do not justify an unchanged queue-scan/drain rerun, a post-hoc batch/width sweep, or a twofold proof-speed claim. A future distinct design needs a credible reduction of expensive work on the complete critical path. No runtime optimization or full-SDK follow-up is adopted here.

## Accepted CPU and Metal comparisons, inherited

Accepted SDK remains `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`. No proof latency is remeasured. All8CPU10×targets remain unmet.

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

Merge54 remains161.883899ms without an M5 reference. Latest full-SDK latency stays epoch-clock176; canonical proof ledger29,461unchanged. No fresh SDK/Gnark,AWS/Aeglos,250/4999,device/pressure or concurrent proof result is added. Goal all-eight CPU10× remains active.

## Artifacts

[Design](artifacts/design.md), [counters/test](artifacts/census.rs), [observer edits](artifacts/observer-edits.json), [source audit](artifacts/source-audit.json), [driver](artifacts/driver.rs), [workflow helper](artifacts/campaign.rs), [native plan](artifacts/native-plan.json), [link plan](artifacts/link-plan.json), [capture plan](artifacts/capture-plan.json), [all counts](artifacts/analysis.json), [reviewer](artifacts/review.rs), [review](artifacts/review.json), [raw command receipts](artifacts/receipts.json.gz), [source](artifacts/source.tar.gz), [source member hashes](artifacts/source-members.json), [publisher](artifacts/publish.rs), [checksums](SHA256SUMS). Source178regular members match extracted hashes and receipt JSONgzip roundtrips exactly. Executables,libraries,private actual key/image/witness/scalar/point arrays are excluded.
