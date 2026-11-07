# Borrowed G2 bucket trees: exact work screen

The necessary operation-count screen passes on both retained fixtures. Balanced reduction requests **43.22% fewer ordinary inversion batches on 1×2 and 48.18% fewer on 2×2**, with unchanged logical generic point-join counts. This selects a native prototype. It establishes no timing, native correctness, observed RAM improvement or adoption.

## Mechanism and prior evidence

Route the ten ordinary Phi2 windows into signed u32 lists, partitioned by the existing 256-bucket groups. Reduce high-component leaves as balanced trees, apply the original endomorphism to completed roots, then reduce low leaves seeded by those roots. Each level batches the group's independent pairs. Two point partitions, 12-bit digits, grouping, narrow indexed top, part merge and weighted reduction remain.

The proposed implementation borrows input leaves and alternates compact intermediate point buffers. It adds temporary routing and workspace; it adds no persistent table, key copy or solver plan. Its memory admission needs a new conservative forecast before allocation.

This differs from the [routed rounds](../2026-10-24-g2-routed-rounds/README.md), which retained 200-entry streaming batches, [GLS4 indexed top](../2026-10-29-g2-gls4-indexed-top177/README.md), changed-width trials and scalar coalescence. Those outcomes remain closed. The [Aeglos source audit](../2026-11-06-aeglos-msm-mechanism176/README.md) already documents sorted block accumulation; this is a local bounded-workspace hypothesis without a novelty claim or transferred speed estimate.

## Exact counts

Immutable B2 query fixtures contain 30,089 and 52,274 original rows. The unchanged grouping policy declines primary and admits 28,852 first-seen nonzero classes for 2×2. Original fixture points are nonidentity; identities from actual grouped-point cancellation are not modeled.

|Ordinary fill quantity|1×2|2×2|
|---|---:|---:|
|Original inversion requests|2,605|2,688|
|Balanced-tree inversion requests|1,479|1,393|
|Original and candidate logical generic joins|517,654|535,600|
|Original scalar-tail pairs|148|152|
|Candidate scalar-tail pairs without padding|4,902|4,696|
|Alternative vector-padding dummy pairs|5,530|5,144|
|Index payload, bytes|2,234,456|2,306,240|
|Count/start/cursor payload across ten windows, bytes|1,966,240|1,966,240|
|Largest modeled group payload, bytes|479,952|423,904|

The inversion comparison excludes unchanged indexed-top work. Grouping, top, merge and weighted operations do not disappear. The real-join identity is independently checked by summing population minus one for each occupied bucket. Original projective overflow additions and finish merges are included in that comparison.

Reduced inversion requests are not a speedup estimate. In particular, many small tree levels increase scalar tails. The selected [native design](artifacts/native-design.md), fixed before implementation or timing, pads an eligible nonempty batch to eight lanes using distinct inactive output slots and a valid pre-update pair. This avoids those scalar tails while executing about 1.07%/0.96% extra vector dummy joins. Empty batches and real identity/doubling/opposite cases need explicit handling; the count model does not qualify them.

The group payload formula charges two conservative retained compact buffers, original prefix/tail scratch, borrowed descriptors and roots. It is layout arithmetic, excluding allocator effects and complete concurrent lifetimes. Routing, parent forks, output arrays, headers and rounded dummy slots must all be included in native admission. These numbers are not process peaks, minimum RAM, mobile or pressure qualification. Skewed inputs require checked worst-case bounds and fallback before allocation.

## Checks and decision

The new model checks 1,157 structural cases, including empty/singleton/odd populations and token cancellation without occupancy removal. Two distinct full-width Fr token streams check every ordinary output bucket, unchanged top contribution and the original scalar dot product for both fixtures. The existing Phi/Booth extraction, baseline queue model and grouping helper are reused byte-exact; accepted production's 176 source hashes and both fixture hashes match. Token checks are not independent curve or native SIMD checks.

A separate reader reconstructs all **40 ordinary rows, 320 bucket groups and 2,872 tree levels** from joint population histograms using ceil-division formulas. It checks logical joins, inversion requests, tail/padding counts, payload calculations, baseline histogram flow, 253 source-pin occurrences and three compile/preparation/model receipts. Its scope excludes an independent token, field, curve or grouping implementation.

Compilation, preparation, model execution, review compilation and review each succeed on their first execution. Existing native suites, fixture oracles, solver exports, proof verification, profiles and decodes are inherited and not replayed. No new proof, timing or performance result is added; canonical proof count remains **31,621**. Production and accepted SDK remain unchanged.

Next is the isolated native implementation: complete exception/sign/tail/reuse oracles, coherent libraries and source recovery, conservative admission, then actual complete-proof measurements with exact accepted and fresh OFF controls. A positive primary pilot would select all-shape/resource/portability qualification, not immediate adoption.

## Accepted CPU and Metal comparisons, unchanged

These are inherited local 32-worker full-SDK observations from the [window-bound campaign](../2026-11-06-window-batch-proof-capture176/README.md), using the accepted SDK. M5 CPU and Metal figures are supplied references. They are not measurements of this count model.

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.915095|37 / 4.675×|24 / 3.032×|3.7|
|2x2|11.169812|59 / 5.282×|30 / 2.686×|5.9|
|4x2|16.085882|109 / 6.776×|48 / 2.984×|10.9|
|8x2|24.517520|189 / 7.709×|74 / 3.018×|18.9|
|12x2|33.838626|284 / 8.393×|101 / 2.985×|28.4|
|16x2|40.083672|335 / 8.358×|124 / 3.094×|33.5|
|24x2|65.506198|528 / 8.060×|176 / 2.687×|52.8|
|merge-24x1|65.200982|507 / 7.776×|175 / 2.684×|50.7|

Accepted merge54 is 161.718956 ms without a supplied M5 reference. All eight CPU 10× targets remain unmet. There is no fresh AWS/Aeglos, phone, ARM, TEE, giant-circuit, concurrent-throughput or swap measurement.

[Design](artifacts/design.md), [native design](artifacts/native-design.md), [counts](artifacts/analysis.json), [review](artifacts/review.json) and remaining [artifacts](artifacts/) retain helpers, plans, pins and exact raw receipts. Checksums cover published files. Fixture/key/witness/proof/public-input payloads and executable/library blobs remain private.
