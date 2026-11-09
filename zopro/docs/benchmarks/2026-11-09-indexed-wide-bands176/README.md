# Indexed wide-band G2: structural and payload screen

**The fixed construction passes its necessary structural screen on both retained populations.** Modeled elementary products fall about 5%, and root inverse calls fall about one third. There is no new native implementation, timing, proof or adoption. Accepted local full-proof 1×2 remains **7.857170 ms**; the all-eight 10× goal remains unmet.

The next investigation is full checked ownership and native correctness. A lower count proxy did not predict the [previous ring trial's measured performance](../2026-11-09-ring-affine-query176/README.md), so this result does not select a timing trial automatically. [Design](artifacts/design.md), [counts](artifacts/analysis.json), [separate replay](artifacts/review.json).

## Fixed construction

Keep qualified Phi2 decomposition, two original source partitions and original field arithmetic. Use width14: nine ordinary positions plus one signed carry. Each ordinary position builds one band-major index for thirty-two 256-bucket cells. Each cell contains 1,024 independent groups: high/low components across both source partitions. A proposed nonforking leaf reduces those groups together through the original complete tree kernel, sharing each inversion round. It merges high partitions, applies φ to their sums, merges low partitions, combines high/low, then runs the original 256-bucket weighted recurrence.

Each leaf returns its weighted result and ordinary total. The window combines these pairs with the existing weighted-affine outer identity:

    W = sum(local weighted sums) + 256 * weighted offsets of leaf totals.

It retains no full 8,192-bucket coordinate vector and does not scan the original input separately for each cell. The signed leading carry keeps the original indexed-top algorithm with one magnitude slot. Root Horner uses fourteen doublings. Encoded point indices carry signs; no input point receives a separate φ transform. Initial signed point copies, routing and all tree work still have costs.

The rejected [four-pass band model](../2026-11-08-iterative-band-cost176/README.md) uses repeated filtered streaming and projective queue fallback. This proposal uses indexed joint-class trees and releases each cell's coordinates after local weighting. Earlier ring, GLS4, shared-point, compact-tree and indexed-top-bundle results remain unchanged; no isolated gain transfers. This is one fixed geometry, without a parameter sweep.

Selected sections of the [SQIsign SIMD paper](https://arxiv.org/html/2608.13948v2) motivate examining arithmetic scheduling beyond isolated field multiplication. Its special-prime reduction and speedups do not transfer to BN254. This proposal is derived from the local grouped/indexed/weighted and Phi2 code; it does not implement that cryptosystem or claim a new published algorithm.

## Complete source-shaped work counts

Primary has 30,089 terms. The retained 2×2 population has 52,274 raw rows and 28,852 grouped terms, matching inherited scalar-cardinality and point-nonidentity evidence. Grouping and arithmetic qualification are inherited, not replayed. Counts assume nonidentity, noncancelling generic sums; actual equal/opposite/doubling trajectories are unqualified.

The model charges every tree/top/merge/weighted/projective/Horner operation, φ map, scalar tail and inverse boundary. V is an eight-lane Fq product, F an eight-lane fused two-product operation; M/S/Fs/I are scalar products/squares/fused operations/inverses. The nominal proxy is 100V+150F. Elementary products are 8V+16F+M+S+2Fs. Neither prices instructions, cycles, routing, packing, memory or scheduling.

|Population / recipe|Noncancelling point additions|Active projective doublings|Vector proxy|Elementary products|Scalar root inverses|
|---|---:|---:|---:|---:|---:|
|1x2 / Accepted|634486|846|116322150|11223641|6607|
|1x2 / Indexed wide|600594|2502|108992550|10665922|4490|
|2x2 / Accepted|654290|846|120232150|11548580|6746|
|2x2 / Indexed wide|617506|2502|112118700|10950425|4335|

|Population|Vector-proxy reduction|Elementary-product reduction|Inverse reduction|
|---|---:|---:|---:|
|1x2|6.301%|4.969%|32.042%|
|2x2|6.748%|5.179%|35.740%|

More doublings and scalar work remain real tradeoffs. Primary scalar M/S/Fs counts rise **185,133/336/88,130→265,450/6,048/102,400**. Generic projective spill and Window normalization are absent from this indexed model, but native exceptional handling has not been implemented. No work type is assigned a synthetic cycle exchange rate.

The necessary screen, declared before execution, requires the abstract identities/count tests, strictly fewer vector-proxy and elementary products, no increase in inverse calls on both populations, and retained ordinary index payload no larger than accepted ordinary coordinate payload. It passes. This permits resource/native-correctness investigation only; it is not an end-to-end performance or memory gate.

## Payload and ownership limits

The index uses u32 starts for 32×1,024 classes and signed u32 point ordinals for at most twice the root's input length. For nine ordinary windows its retained payload bound is:

    9 * (32,769 * 4 + 2 * n * 4) bytes.

|Population|Retained ordinary index bound B|Accepted ordinary coordinates B|Largest cell rows|Largest class rows|Maximum leaf input-point payload B|
|---|---:|---:|---:|---:|---:|
|1x2|3346092|5242880|2359|514|301952|
|2x2|3257028|5242880|2177|11|278656|

These compare different subsets of ownership, not total RAM. Per-window transient count/position payloads are another **65,536+131,072 bytes**, which must drop before child work. A leaf also needs group offsets/counts, original complete tree records and capacity growth, up to four 256-bucket outputs and 511 weighted point slots. Largest input-point payload is about 295 KiB primary and 272 KiB grouped. Ordinary returned weighted/total pairs add 92,160 payload bytes across windows.

All suspended parent indices, construction overlap, leading parents, scalar storage, leaf scratch, headers and worker stacks still need a conservative typed forecast and checked admission. Input multiplicity can be skewed; a guessed 2,048-row leaf cutoff would already reject primary's observed 2,359-row cell. No artificial cap is introduced. Neither this model nor its coordinate formulas establish RSS, minimum RAM, mobile/pressure or concurrent-process safety.

Digit extraction, stable indexing, signed point copies, identity checks and memory traffic are unpriced in the proxies. The proposed implementation needs two ordinary routing passes and the leading reducer's existing routing. Those source visits are not cache/DRAM traffic and are not interchangeable with the older model's simplified visit counter.

## Qualification and ledger correction

Four Rust/xtask commands pass on their first executions. Two new abstract tests cover φ after part merging, signs/zeros/cancellations, weighted sums plus total offsets, lengths through 4,099, and 64 joint-class histograms checked by independent ceil-division. The group is Z1019[ω]; these are finite formal-group evaluations, not native elliptic-curve proofs or arbitrary-device tests.

The separate same-agent reader checks **198 pins**, all four command/stream receipts, **117,882 scalar-component reconstructions**, all **576 leaf diagnostics**, tree/merge/weighted histograms and the complete field ledger. It groups scalars by first occurrence, recodes through direct division/carry and reconstructs reduction layers using ceil-division and occupied support sets. The Phi split and the independent reader's previously qualified support-set helpers are inherited. The accepted baseline is read, not regenerated; original tests and native arithmetic are not replayed. Reader compilation/execution are separate tool calls, not additional model-workflow receipts. No successful stage is repeated.

The source audit also finds an earlier bookkeeping overcount: accepted ordinary Window normalization occurs **10 windows×2 parts=20 times**, once after both component feeds. The earlier covered accepted ledger counted 40. The [prior report](../2026-11-09-ring-affine-completion176/README.md) now includes corrected accepted rows and an explicit erratum; its original frozen review, histograms, measurements and selection decisions remain intact. This model's full baseline already used 20. The partial covered ledger and this full field ledger have different scopes. [Source audit](artifacts/source-audit.json), [correction record](artifacts/prior-ledger-correction.json).

There is no new native field/core/prover build, curve suite, SDK request, proof, Gnark export, timing or profile. The canonical verified proof ledger remains 40,154. The next step is a checked ownership forecast and native implementation preserving complete exceptions, grouping and resource fallback; only then can a frozen complete-consumer comparison be selected. No AWS, Aeglos, phone, ARM, huge-circuit, pressure, swap, minimum-RAM or throughput result follows.

## Accepted CPU and Metal comparison, inherited

The following local Ryzen 9950X full-proof timings with 32 workers remain the accepted REF measurements from the completed singleton-elision SDK campaign. They are not results for this model or new M5 measurements.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.857170|37|24|4.709×|3.055×|3.7|
|2x2|11.052363|59|30|5.338×|2.714×|5.9|
|4x2|15.952018|109|48|6.833×|3.009×|10.9|
|8x2|24.080322|189|74|7.849×|3.073×|18.9|
|12x2|34.012417|284|101|8.350×|2.970×|28.4|
|16x2|40.128912|335|124|8.348×|3.090×|33.5|
|24x2|66.348981|528|176|7.958×|2.653×|52.8|
|merge-24x1|65.829291|507|175|7.702×|2.658×|50.7|

Production remains accepted source176/SDK6d83. The verified all-eight CPU10× goal remains active and unmet, **0/8**.

[Continuation](artifacts/next.md), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS).
