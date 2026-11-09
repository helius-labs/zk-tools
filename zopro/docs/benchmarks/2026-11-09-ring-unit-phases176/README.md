# One-array ring unit phases: work and space screen

**The necessary count screen passes, but this fixed schedule is not selected for a native timing trial.** Reusing one ordinary bucket array removes two of three unit planes, yet moves enough work into scalar projective fallbacks to increase every covered field-operation category versus the old ring. Production and accepted proof timings remain unchanged. No measured slowdown or speedup is established.

For primary, projective mixed additions plus finish joins rise **676→12,523**, and points needing normalization rise **83→5,813**. Independent queue replay identifies **10,746 points spilled on queue overflow** and **1,777 at phase boundaries**. Changing only the phase-end flush would leave most of the observed spill population. These are generic structural counts, not actual elliptic-curve exception or CPU observations. [Counts](artifacts/analysis.json), [independent replay and field ledger](artifacts/review.json).

## Distinct schedule

The rejected [three-plane G2 ring](../2026-11-06-g2-joint-ring-native176/README.md) uses 8,193 buckets per part. This proposal keeps one 2,731-bucket ordinary Window, processes unit2, applies φ to its affine and exceptional projective accumulators, processes unit1, applies φ again, then processes unit0. Linearity gives `φ²(S2)+φ(S1)+S0`. The original queue is flushed at every phase boundary. Incoming point coordinates need no unit transform. The older G1 single-plane experiment (`2026-10-24-ring-single-plane-component`) instead transformed each incoming point and remains rejected.

The exact recoder, radix128, nineteen windows, two original point partitions, batch200/queue199 LIFO policy, signed indexed leading window, representative axes and weighted trees are retained. The leading window keeps its original three-plane algorithm and cost. Ordinary collapsed parts merge directly, then reuse axis weighting. Scalar-code rows are inspected three times instead of once; unit filtering precedes any incoming coordinate/identity read. These are semantic access counts, not physical cache/DRAM traffic. Existing circuit-independent residue constants are reused, with no new point/key table, persistent plan, pool or native implementation. [Frozen scope](artifacts/design.md).

## Necessary screen and limitations

The prospectively declared checks require the phase identity and slot geometry, a vector-product proxy strictly below accepted, and no increase in inverse-root calls versus accepted on both retained populations. All pass. That screen permits only further resource/native-correctness investigation. It does not price scalar fallback, normalization, maps or scheduling as free and does not automatically select performance capture. No after-result threshold or width/layout/worker sweep is introduced.

V is an eight-lane Fq product, F an eight-lane fused two-product operation. The proxy 100V+150F counts nominal IFMA source sites and excludes scalar work and all non-product costs. Full signed scalar recoding/top/tree qualification is inherited; old baseline counts are read from their saved census. Raw2×2 is diagnostic because production admits whole-scalar grouping there.

|Population / recipe|Generic affine pairs|Inverse roots|Vector-product proxy|Projective mixed + finish|Normalized points|Unit-map Fq products|
|---|---:|---:|---:|---:|---:|---:|
|1x2-B / Accepted|632442|6587|116322150|1178|429|82486|
|1x2-B / Old ring|572574|2608|104454800|676|83|188084|
|1x2-B / Unit phases|560727|2862|102643150|12523|5813|359428|
|2x2-B / Accepted|1082397|8911|198460450|2236|851|83338|
|2x2-B / Old ring|955437|4394|174701450|997|46|190324|
|2x2-B / Unit phases|953918|4884|174917450|2516|952|380648|

Primary's vector proxy is 11.76% below accepted and 1.73% below the old ring. That partial result hides the scalar work. The total number of generic additions plus projective mixed/finish additions is conserved versus the old ring in the noncancelling model; they have moved to different formulas and execution types.

## Scalar-work closure

The unchanged generic mixed G2 addition uses 7 Fq2 multiplies and 4 Fq2 squares. Native x86 Fq2 multiplication uses two scalar fused products; Fq2 squaring uses two ordinary Fq products. Batch normalization uses 6 Fq2 multiplies plus one Fq2 square per nonidentity point and one Fq2 inverse per finish call. The source-bound ledger charges these operations, scalar tails, unit maps, the old/new 19/1 β² source expressions and the common 19 axis-result maps. All 36 ordinary finish calls remain.

The following logical events expand each full eight-lane operation to eight field events. Vector and scalar execution have different throughput: these are neither instructions nor runtime estimates. Common weighted/projective postprocessing, packing, recoding, additions/subtractions, memory and scheduling remain unpriced. The reviewer checks identical weighted-projective call geometry between old ring and unit phases, not arbitrary native cancellation behavior.

|Population / recipe|Covered Fq products|Covered fused2 events|Scalar Fq squares|Scalar Fq inverses|
|---|---:|---:|---:|---:|
|1x2-B / Old ring|4310769|2873206|320|2644|
|1x2-B / Unit phases|4516080|3048325|848|2898|
|2x2-B / Old ring|7069126|4791575|312|4430|
|2x2-B / Unit phases|7283111|4815860|828|4920|

Primary adds **205,311 ordinary Fq products**, **175,119 fused2 events**, **528 scalar squares** and **254 inverses** versus the old ring in this covered ledger. Raw2×2 also increases all four categories. The footprint reduction therefore does not supply a convincing useful-work gain over that rejected recipe. A cache benefit could offset work in a real implementation, but none is measured or assumed. Close this fixed schedule without native timing; do not relabel its necessary screen as failed or claim every one-array strategy is slower.

## Space and validation

Ordinary bucket coordinates fall from 8,193 to 2,731 slots per part. If every two-part window output coexists, coordinates alone fall **39,850,752→14,681,856 bytes** (about 38.005→14.002 MiB), including the unchanged leading window. This is not a complete ownership forecast, measured liveness/RSS/stack or minimum RAM. Codes, metadata, queue/batch, projective fallback, top indices/results, axis scratch, suspended parents, fallback and other proof state still need charging before native use. No architectural memory cap or pressure claim follows.

Four corrected Rust/xtask commands pass once. Two new formal tests check unit-phase linearity over Z1019[ω], signs/zeros/cancellations, small unit masks, lengths through 4,099 and payload geometry. Three inherited recipe tests are filtered, not rerun. These are formal-group tests, not native EC exception-dispatch or device qualification. An initial compile rejects Value==&String before any test/model; a single comparison spelling repair to name.as_str() preserves the initial sources, pins and failed receipt. No successful scientific stage is repeated. [Source binding](artifacts/source-audit.json), [repair](artifacts/case-match-repair.json).

The separate same-agent reviewer checks 215 model pins, all four command/stream receipts, phase-map counts, unchanged leading/tree accounting and the source field ledger. It independently decodes using the existing native residue MAP and qualified Phi split, with direct Euclidean subtraction/division, then replays the phase queues using VecDeque. Every new stream count matches. Baseline tree helpers, MAP and Phi arithmetic qualification are inherited rather than independent field implementations. Its compile/execution are separate tool calls, not additional model-workflow receipts. There is no external audit, native kernel/proof test, benchmark or new performance profile.

A related [Noël–Plantard paper abstract](https://eprint.iacr.org/2026/1316) describes bucket counts adapted to limited memory and reports BLS12-381 gains concentrated under tight RAM. Only its official abstract/metadata were accessible; the official PDF fetch returned 403 and the mirror open failed. Its algorithm is not implemented here, and its claims are not transferred to BN254. Existing radix 3072/6561 and fused-scaling complete-caller failures were found during the history audit and not reopened.

## Accepted CPU and Metal comparison, inherited

These accepted fixed-placement measurements come from the [preceding full SDK comparison](../2026-11-09-singleton-elision-sdk176/README.md), not this model. M5 references use different hardware.

|Shape|Accepted local ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.857170|37 / 4.709×|24 / 3.055×|3.7|
|2x2|11.052363|59 / 5.338×|30 / 2.714×|5.9|
|4x2|15.952018|109 / 6.833×|48 / 3.009×|10.9|
|8x2|24.080322|189 / 7.849×|74 / 3.073×|18.9|
|12x2|34.012417|284 / 8.350×|101 / 2.970×|28.4|
|16x2|40.128912|335 / 8.348×|124 / 3.090×|33.5|
|24x2|66.348981|528 / 7.958×|176 / 2.653×|52.8|
|merge-24x1|65.829291|507 / 7.702×|175 / 2.658×|50.7|

All eight CPU 10× requirements remain active/unmet, **0/8**. The accepted 1×2 remains 7.857170 ms, with historical accepted standard 7.915095 ms still valid; the rejected copy candidate 7.928927 ms and prior unadopted 7.753327 ms are not production records. This study adds zero proofs; canonical 40,154 is unchanged. No new AWS/Aeglos, phone/ARM, huge circuit, pressure/swap, minimum-RAM or throughput result follows.

Next work needs a distinct batching/accumulation method that avoids the overflow explosion without restoring three dense arrays or adding per-input point transforms, or another ordinary G1/G2 work reduction after the history check. No replacement is selected. Full resource/correctness/native-query and all-eight/merge54 SDK/Gnark qualification precede any adoption. [Continuation loop](artifacts/next.md).

[Exact text evidence](artifacts/evidence.json.gz) retains helpers, fixed/failed sources, immutable pins and model receipts. Executables/libraries, private point/scalar/key/witness/proof inputs and third-party PDF text are excluded. [Publication checks](artifacts/publication.json), [checksums](SHA256SUMS). Private root: `/home/ubuntu/src/zopro-perf-work/ring-unit-phases176`.
