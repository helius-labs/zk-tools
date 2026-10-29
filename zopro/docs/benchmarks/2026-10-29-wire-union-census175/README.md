# Shared witness wire classification opportunity

The necessary count/metadata screen passes. Primary A/B together require 47,762 nonzero scalar hash visits in separate maps versus 28,638 for their wire union, a **40.04% reduction**. All nine fixtures show approximately 40% fewer modeled visits. Reconstructed query groups exactly match the original constructor in every case. This is an operation-count result, not a faster prover; production remains unchanged.

## Work removal

A and B query scalars come from overlapping indices into the same solved witness. The proposed mechanism classifies their active wire union once per proof, then reconstructs each query's original stable local groups from global class labels. This removes repeated scalar hashing across queries, instead of changing dictionary layout again. It requires no persistent key table, copied plan, new point array or witness reuse across proofs.

The screen reads A/B indices from the accepted solver image and checks strictly increasing order, bounds, scalar-vector lengths and every query value against its original witness wire. Frozen accepted raw-key and serialized-plan scalar solutions agree for all nine fixtures, 18 explicit solves. The first two shapes use canonical words and the remaining shapes Montgomery words, matching their production entry geometry. Private wire/scalar arrays are not persisted.

Existing per-query sampling remains. All 18 actual samples admit construction; the two primary full populations still decline at their original cutoffs. Sampling-rejected queries do not need shared classification. The diagnostic model uses ordered maps/sets to establish semantics, not a proposed native data structure or runtime comparison. A native prototype would use the existing wire owner and compact class indices; this screen does not measure such an implementation.

## Counts on real inputs

A hash visit is one nonzero scalar presented to the grouping dictionary. Equal values on different wires still require visits; wire overlap removes repeated visits to the same wire. Counts exclude the unchanged sampling scans, dictionary probes/growth, output construction, remapping, allocation, group-point formation, FFT and MSM work. They are not total group operations or latency predictions.

|Shape|A wires|B wires|Overlapping wires|Separate nonzero visits|Shared nonzero visits|Reduction|
|---|---:|---:|---:|---:|---:|---:|
|1x2|21006|30089|20601|47762|28638|40.040%|
|2x2|36637|52274|35878|83081|49784|40.078%|
|4x2|67154|95537|65684|151804|90932|40.099%|
|8x2|129064|183347|126112|291332|174489|40.106%|
|12x2|190734|270757|186220|430002|257550|40.105%|
|16x2|252548|358343|246392|568876|340775|40.097%|
|24x2|376992|534619|367312|848170|508192|40.084%|
|merge-24x1|359220|507895|351018|809622|484463|40.162%|
|merge-54x1|804765|1137190|785508|1811935|1084537|40.145%|

The global pass still contains almost the full B query, plus A-only wires and new remapping work. Lower summed hashing work therefore does not establish a shorter B critical path. Front-loading the pass could delay previously independent queries. A useful caller prototype must include that dependency change and account for overlap with quotient preparation, rather than timing only a faster dictionary.

## Metadata and correctness

The model reconstructs original scalar values, first-occurrence class order, per-class counts, prefix starts, stable input-index lists and accept/decline outcomes. All 18 real query results match the source-extracted original builder exactly. One new test executes 75 synthetic cases across empty/overlapping/disjoint query sets, zeros, repeated values on different wires and full-width words, comparing 150 local outputs. All four compile/test/real-census workflow steps pass on their first execution. No full core/field/prover suite is repeated or claimed.

For primary, one dense global label vector and two local remap vectors require 366,252 bytes. This is below the existing conservative two-map bound of 6,046,016 bytes. All other fixtures also pass this necessary dense-metadata check. The count excludes the future global dictionary, unchanged outputs, allocator/stack overhead, reservation lifetimes and concurrency with FFT scratch; it is not a complete forecast, RSS saving or RAM minimum. The diagnostic ordered-map/set allocations are not the proposed native payload.

The prospective necessary gate requires exact metadata agreement everywhere, fewer primary nonzero hash visits, and primary dense metadata within the old map bound. It imposes no minimum gain floor. These checks pass, selecting a separate actual-caller prototype only. Native workspace accounting and a full SDK accepted-relative comparison remain required before any source adoption or proof-speed claim.

## Prover state and supplied references

No SDK proof, independent export or performance capture is produced. The verified proof ledger remains 21,007. Production source 175 and SDK 20fde remain unchanged. The original all-eight verified CPU 10× objective remains active and unmet 0/8. Accepted full-proof timings and supplied historical M5 references follow.

|Shape|Accepted local ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|37|4.612×|24|2.992×|3.7|
|2x2|11.335969|59|5.205×|30|2.646×|5.9|
|4x2|15.945544|109|6.836×|48|3.010×|10.9|
|8x2|24.090485|189|7.845×|74|3.072×|18.9|
|12x2|33.844855|284|8.391×|101|2.984×|28.4|
|16x2|39.931557|335|8.389×|124|3.105×|33.5|
|24x2|66.233309|528|7.972×|176|2.657×|52.8|
|merge-24x1|65.273224|507|7.767×|175|2.681×|50.7|

Accepted 54→1 remains 162.250464 ms without a supplied M5 reference; accepted primary peak RSS remains 134 MiB from its full qualification. The [current local/AWS summary](../2026-10-29-current-cpu-summary/README.md) retains the accepted physical 16 calibration exception and separately true final-artifact gate. This screen supplies no new AWS/Aeglos, mobile/ARM/TEE, forester 250/4999, swap, pressure, concurrency or minimum-RAM result.

[Protocol](artifacts/design.md), [full counts](artifacts/analysis.json), [implementation](artifacts/probe.rs), [source-extracted original](artifacts/legacy.rs), [workflow](artifacts/plan.json), [receipts](artifacts/receipts.json.gz), [closing pins](artifacts/pins.json).
