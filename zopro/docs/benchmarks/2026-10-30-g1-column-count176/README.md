# SIMD bucket columns: structural screen

A segmented, lane-stable G1 bucket layout passes its metadata/algebra opportunity screen on two raw A-query populations. It preserves the modeled point-addition count and reduces logical host field conversions by about49%, with at least99.45% lane utilization. **No native curve implementation or speedup is established.** Production source176 and all accepted proof results remain unchanged.

The initial unsplit design fails: a heavily populated low-digit bucket forces a long dependent chain. The follow-up divides each bucket into segments of at most eight points, puts similarly sized segments into SIMD cohorts, and merges their partial sums afterwards. Segment size is fixed to the native lane count; no size sweep is performed. All original failures remain in the archive.

## Mechanism and scope

Keep each cohort's eight bucket coordinates in SIMD lane form through its updates, while borrowing original key points. This aims to avoid repeated bucket-coordinate import/export. It differs from the earlier native radix52 trial, which gathered/scattered random destinations, and from grouped multiwindow queues. Incoming points still require packing; lazy coordinates still require correct range repair. Existing canonical arithmetic shortcuts cannot blindly consume lazy bucket state.

The exact accepted native libraries extract raw pre-group A-query scalars for1x2 and2x2. These raw populations are not automatically the final grouped execution populations. The ordinary signed width12, one-part geometry matches the accepted deferred-sign route; the narrow indexed top stays unchanged. This is a filler feasibility screen, not a census of actual current proof operations. No key/witness values are published.

Cohorts are sorted by population. If cohort g needs h_g updates, T=sum(h_g), M=25cohorts per200-entry batch, and W=max(max(h_g),ceil(T/M)), put each cohort's occurrences consecutively around W wave indices with one circular cursor. Each wave then has at most25 distinct cohorts. This removes a priority heap from the proposed schedule. Addition order within a bucket is commutative; original input indices are consumed exactly once. Compact wave metadata is proof-local, not a retained key plan.

Partial sums merge through the existing256-original-bucket group tree topology. For N insertions, S partial segments and B occupied original buckets, (N−S) linear additions plus (S−B) merges equals the original N−B additions. Padding and merge layers are charged separately. Real curve cancellation, doubling and exceptional fallback remain unmodeled.

## Corrected rawA observations

| Shape | Raw rows | Ordinary windows | Original additions | Linear + merge additions | Minimum lane utilization | Logical field conversions, reference → segmented | Minimum conversion reduction |
|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 21006 | 21 | 348545 | 324341 + 24204 | 99.460% | 2091270 → 1062718 | 49.001% |
| 2x2 | 36637 | 21 | 636516 | 575698 + 60818 | 99.549% | 3819096 → 1931588 | 49.325% |

The predeclared necessary gate requires every ordinary window to have utilization≥0.98, fewer reviewed logical host field conversions, and all scalar/token/schedule checks passing. All42 windows pass for the segmented design. This permits native work only; it is not an adoption or latency gate.

An ideal dense affine filler converts6(N−B) logical scalar fields. The segmented model charges2N+2S before merging and6(S−B) for merging, so modeled savings are4N−8S. These are logical field counts, **not emitted instructions, traffic, cycles or removable proof latency**. Dummy lanes, in-lane canonicalization, sorting/indexing, gathers, shuffles, exceptions, allocation/cache behavior, final weighted reduction and scheduling still need a complete native measurement.

| Lowest window | Unsplit utilization | Unsplit waves | Ideal dense batches | Segmented linear waves | Segmented merge layer calls |
|---|---:|---:|---:|---:|---:|
| 1x2 | 79.403% | 652 | 87 | 81 | 21 |
| 2x2 | 78.523% | 1251 | 158 | 143 | 26 |

Across21ordinary windows,1x2 uses1641planned linear waves and341merge layer calls versus1747ideal dense batches;2x2 uses2884 and442 versus3198. Merge layer calls have varying sizes and are not measured inversions. Neither table establishes a reduction in inversion cost. The unchanged unsplit gate remains false; higher windows are not promoted separately.

## Correctness and corrections

73 original column-model self-checks cover empty/odd/skewed/signed populations, holes, identities, SIMD tails, circular wrap, wave uniqueness/capacity and exact input use. The corrected entry inherits those unchanged model bodies instead of repeating them.71new segmented checks cover segment remainders, long buckets and partial merges. Both real inputs run two deterministic full-width Fr-token streams, checking each segment, each original bucket and the weighted sum. The corrected ordinary/top Booth reconstruction also matches every original scalar. These are Fr-token/algebra checks, not BN254 point-kernel qualification.

The first metadata run mistakenly used the generic width selector (10/11), overlooking the accepted deferred-sign width12 route. Its source, executable identity,48window observations and receipts remain labeled as initial nonproduction geometry. A separate corrected workflow uses width12/parts1 with identical model bodies and gate. The initial run did not cache its extracted scalars, so correction required two additional solves: **four solver calls in this study**, zero proof requests. Corrected scalar/identity caches then serve the segmented follow-up, which adds no solves. All90initial/corrected raw window counts and42segmented windows are independently recomputed from those cached inputs using a different bit reader.

The initial/corrected raw observer also included seed copies in its reference-import field count. Those immutable raw derived fields are superseded by the reviewed codec equations above; a seed copy is not a SIMD conversion. No count or timing is rerun for that accounting correction.

All three source-pinned Rust xtask link/count workflows complete with248/256/267pins. They link exact accepted libraries, without rebuilding the prover or repeating its suites. The independent review checks raw input/production/library pins, model-body identity, schedule bounds, histogram/merge/codec arithmetic and both failed/passing gates. No successful model execution is repeated. No latency, PMU recording, CPU profile, Gnark export or new proof occurs. Ledger remains27,353.

## Memory and next qualification

The lowest1x2window models262,400bytes of native column coordinates,209,664bytes of exported partial affine points and77,076bytes of input indices. The2x2window models408,960/327,104/134,256bytes respectively. Maps, offsets, wave IDs,24,000bytes of three-field records and generic merge scratch are additional. Native coordinate and exported partial arrays overlap during export, so neither can be omitted from a future admission bound. These are requested payloads, not allocator/process peaks or minimum RAM. Original key/input ownership, compiler frames and complete weighted-output storage are not included.

A native implementation needs explicit canonical/lazy contracts, complete exceptional-point handling, checked workspace admission/fallback and actual allocation/lifetime accounting. It then needs matched whole-filler/query correctness, threaded latency and profiles before any fullSDK/all-shape/resource qualification. The count model does not establish that conversion savings outweigh input gathers and extra scratch, nor that this alone can supply a twofold proof improvement. No AWS/Aeglos, phone/ARM/TEE, huge-circuit, pressure/swap, concurrency or minimum-RAM claim follows.

History: [native radix52 buckets](../2026-10-22-universal-native-bucket52/README.md), [shared-point queues](../2026-10-25-shared-point-window-queue/README.md), [closed G2 output stores](../2026-10-30-g2-tree-store176/README.md). Primary[DoT](https://arxiv.org/html/2604.21566v1) and[Noël–Plantard](https://link.springer.com/article/10.1007/s13389-026-00393-z) pages were rechecked; both mechanisms were already represented in earlier studies. This layout is a local hypothesis, with no imported paper speedup.

## Inherited accepted CPU and Metal comparison

| Shape | Zopro ms | Peak RSS MiB | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup | 10× CPU target ms |
|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 7.928660 | 132.75 | 37 | 4.667× | 24 | 3.027× | 3.7 |
| 2x2 | 11.151932 | 275.00 | 59 | 5.291× | 30 | 2.690× | 5.9 |
| 4x2 | 16.086386 | 312.25 | 109 | 6.776× | 48 | 2.984× | 10.9 |
| 8x2 | 24.179453 | 517.25 | 189 | 7.817× | 74 | 3.060× | 18.9 |
| 12x2 | 33.884455 | 706.50 | 284 | 8.381× | 101 | 2.981× | 28.4 |
| 16x2 | 40.077381 | 842.00 | 335 | 8.359× | 124 | 3.094× | 33.5 |
| 24x2 | 65.850777 | 1300.50 | 528 | 8.018× | 176 | 2.673× | 52.8 |
| merge24 | 65.060937 | 1168.19 | 507 | 7.793× | 175 | 2.690× | 50.7 |

The all-eight 10× CPU objective remains unmet: **0/8** targets achieved. The primary accepted result is 7.928660 ms against a 3.7 ms target. The inherited 54-1 merge result is 161.883899 ms / 1,876.96 MiB peak RSS, without a supplied M5 reference. Latest AWS comparisons predate accepted source176; this study changes none of them.


[Initial protocol](artifacts/design.md), [geometry correction](artifacts/correction.md), [segmented follow-up protocol](artifacts/segmented-design.md), [initial counts](artifacts/analysis.json), [corrected unsplit counts](artifacts/analysis-corrected.json), [segmented counts](artifacts/analysis-segmented.json), [independent review](artifacts/review.json), [workflow/source archive](artifacts/workflow.tar.gz), [all command receipts](artifacts/receipts.tar.gz), [checksums](SHA256SUMS). Raw scalar caches, key images, libraries and executables remain local with recorded hashes.
