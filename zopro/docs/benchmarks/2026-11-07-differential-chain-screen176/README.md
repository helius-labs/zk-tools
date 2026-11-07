# Regular differential ladders: necessary-work screen

The fixed regular d-MUL recurrence is **not selected for native implementation**. On the retained primary and grouped 2×2 scalar populations, an optimistic bound already requires **10.90× / 10.92×** the accepted model's complete point-add calls. This is an integer work comparison, not a measured slowdown or a rejection of all differential-chain algorithms.

## Algorithm and scope

The [2017 construction](https://eprint.iacr.org/2017/311) schedules one doubling and d additions per scalar bit. Its differential-formula option stores `(3^d−1)/2` difference points; its traditional-addition option avoids that table. The [2020 encoding follow-up](https://eprint.iacr.org/2020/503) simplifies scalar encoding while preserving the regular ladder. These methods address useful small-dimensional settings; their operation schedule needs a separate comparison with a large windowed MSM.

Only accessible author abstracts, the indexed 2017 contribution excerpt and the previously readable 2020 introduction were used for those statements. Full ePrint PDF requests returned 403; an alternate fetch timed out. No full-paper implementation, complete storage audit or source-code reproduction is claimed.

For BN254, cheap Montgomery-curve formulas cannot be substituted without checking the curve model. The [short-Weierstrass XZ formula database](https://hyperelliptic.org/EFD/g1p/auto-shortw-xz.html) lists differential addition with an affine known difference at 6M+2S, and general difference at 7M+2S, before parameter operations. No field-cycle weights are assigned here. A known difference is additional required input, not a free arbitrary bucket-add primitive.

The [separate 2024/038 paper](https://eprint.iacr.org/2024/038) was revised in February 2026. Its current abstract reports 11.34% improvement for its large-dimension method, whereas an indexed original abstract reported 50%. Neither headline is transferred to this prover or treated as a comparison with our Pippenger implementation. Its complete revised recurrence remains unaudited and is outside this screen.

## Actual inputs and bound

The [qualified fixed-window audit](../2026-11-06-g2-fixed-wnaf176/README.md) supplies retained scalar identities, original Phi2 decomposition/grouping code and complete accepted point-add counts. No old model, curve oracle or successful test is replayed. This new input pass measures component bitlengths, a different quantity.

For any partition of nontrivial components into groups G, let L_G be the largest bitlength in G. Granting a free leading step gives at least `|G|·(L_G−1)` scheduled additions. This is at least the sum of `max(bitlength_i−1,0)`. The bound therefore permits arbitrary partitions and drops zero/unit/sign handling for free. It also ignores all doublings, encoding, table setup/storage, exceptions, coordinate recovery, group merging and packing.

|Population|Scalar terms|Signed Phi2 components|Optimistic regular-ladder add bound|Original complete point-add calls|Bound / original|
|---|---:|---:|---:|---:|---:|
|1×2|30,089|60,178|6,912,902|634,486|10.895279|
|Grouped 2×2|28,852|57,704|7,142,554|654,290|10.916496|

The second population comes from 52,274 raw rows with inherited mathematical grouping and zero-identity evidence. This does not replay runtime resource or prepared-key admission. Original counts include fill, partition/component/sign merges, indexed leading reduction, weighted trees and Horner additions. The comparator's doublings are also omitted from this additions-only ratio. Both algorithms use their existing signed Phi2 components; no dimension, width, lattice or subset sweep is selected.

The necessary screen requires the optimistic candidate add bound to be below the original complete add count on both populations. Both fail. Coordinate formulas have different costs, and invocation counts are not cycles. This result closes this fixed regular recurrence without a native query, SDK trial or claim of universal algorithmic impossibility.

## Table growth, descriptive

These sizes illustrate the published differential-table formula. They are not allocations, RAM peaks, resident-set measurements or architectural limits. Minimum x-only payload assumes one 32-byte Fq coordinate for G1 or one 64-byte Fq2 coordinate for G2, omitting denominators and all metadata.

|Dimension|Stored points|G1 minimum x payload bytes|G2 minimum x payload bytes|
|---|---:|---:|---:|
|8|3,280|104,960|209,920|
|16|21,523,360|688,747,520|1,377,495,040|
|20|1,743,392,200|55,788,550,400|111,577,100,800|

Bounding dimension controls table growth but does not evade the partition-independent scheduled-add bound above. No table is built or added to production.

## Validation and decision

One new integer-model test passes on first execution. It exhaustively checks more than 400,000 small set/partition cases and independently enumerates ternary sign-equivalence classes through dimension 6. The producer then scans both pinned populations once. Original Phi2/grouping tests remain inherited.

The independent reader reconstructs all 117,882 component bitlengths using byte scanning rather than leading-zero arithmetic, the histogram dot-product bound, first-occurrence grouping, scalar/component digests, table sizes using a geometric sum, four successful command receipts and all 176 unchanged production hashes. It reuses the previously qualified Phi2 routine; it is not a new decomposition oracle or cryptographic verifier.

The first reader compiled, then failed on exact floating-point equality for a descriptive ratio differing in its last binary bit after JSON serialization/parsing. The original source and failure remain. The repaired reader checks exact numerator/denominator/selection and allows two relative machine epsilons only for that displayed ratio. Only the failed review stage is repeated with its changed source; no successful model/test/capture/proof is rerun. Review passes. No performance or statistical threshold changes.

Production176 and accepted SDK6d83 stay unchanged. No native arithmetic candidate, SDK/Gnark proof, profile, timing, RSS, AWS/Aeglos, pressure/mobile or 250/4999 measurement is added. Canonical proof ledger 31,913 is unchanged. The all-eight CPU 10× goal remains active and unmet, 0/8.

## Accepted CPU and Metal comparison, inherited

These local 32-worker complete-SDK observations and supplied M5 references are unchanged; they are not timings of this model.

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.915095|37 /4.675×|24 /3.032×|3.7|
|2×2|11.169812|59 /5.282×|30 /2.686×|5.9|
|4×2|16.085882|109 /6.776×|48 /2.984×|10.9|
|8×2|24.517520|189 /7.709×|74 /3.018×|18.9|
|12×2|33.838626|284 /8.393×|101 /2.985×|28.4|
|16×2|40.083672|335 /8.358×|124 /3.094×|33.5|
|24×2|65.506198|528 /8.060×|176 /2.687×|52.8|
|merge24|65.200982|507 /7.776×|175 /2.684×|50.7|

Merge54 remains 161.718956 ms without a supplied M5 reference. [Artifacts](artifacts/) preserve design, model/test and reviewer sources, both reviewer versions, hashes, raw histograms, plans and exact receipts. Private scalar/point payloads, proof/key/witness inputs and executable/library blobs are excluded.
