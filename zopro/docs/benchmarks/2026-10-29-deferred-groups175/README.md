# Deferred output allocation for scalar grouping

The component gate **fails**. Production remains unchanged. This experiment postpones scalar grouping output arrays until exact admission succeeds, then converts the same compact dictionary from original-row indices to class indices in place. It targets work discarded when grouping is rejected, while preserving each query's readiness and independence.

## Algorithm

The first pass indexes nonzero scalar values with pinned hashbrown 0.16.1 HashTable<u32>, comparing all four words in the borrowed immutable input. It rejects on the next new value after floor(n×percent/100), since the distinct count cannot decrease. Values, row classes, counts and stable output lists are not allocated before successful admission.

On success, a second pass introduces class IDs in first-occurrence order. Untagged dictionary slots refer to input rows; tagged slots refer to the existing output value vector. The represented scalar and hash stay identical. No insertion or rehash occurs after tagging. Stable output packing follows the source-extracted original. Inputs of at least 2^31 rows retain the original builder to avoid tag ambiguity. Sampling, zero handling and acceptance thresholds remain unchanged.

This differs from the earlier [one-pass Swiss dictionary](../2026-10-29-swiss-groups175/README.md), which constructs all outputs while hashing, and [shared witness classification](../2026-10-29-shared-wire-sdk175/README.md), which adds a dependency between queries. It is a new algorithm/lifetime arrangement, not a repeat of either measurement. No persistent key table, execution plan, point copy, worker pool or new unsafe code is added. The dictionary uses four bytes per slot plus control/group padding. Original conservative map/output forecasts dominate its payload, but this component does not qualify whole-prover RSS, minimum RAM or pressure behavior.

## Measurement method

Ryzen 9 9950X, main thread pinned to CPU 2. For all nine fixtures, the frozen accepted raw-key solver and freshly serialized solver-image solutions agree. A/B use canonical words for the first two shapes and Montgomery words for the remaining shapes, matching accepted entry geometry. All 18 sampling/admission outcomes remain recorded. No scalar or witness arrays are persisted.

Thirty-one fixed blocks rotate/reverse context and algorithm order. Every cell averages eight separately timed constructors after three checked primes per context/algorithm. All three algorithms share deterministic AHash seeds by block/call. Timers include hashing, allocation, growth, the candidate's second pass, map destruction and stable output construction. Sampling/reservation and returned output checking/destruction are outside timing. Every prime and timed output equals an independent ordered-map oracle.

The three algorithms are the source-extracted original HashMap, the previous one-pass Swiss builder and the new deferred builder, linked in one native/fat-LTO executable using unchanged frozen accepted libraries. This is a constructor comparison in one compiled context, not independent-build calibration or proof latency. Earlier full-SDK calibration failures remain unchanged; a component pass would still require accepted/fresh-OFF/ON SDK qualification.

## Constructor results

Times are milliseconds, medians of block means. Paired ratios need not equal ratios of displayed medians. Ratios below one mean faster.

|Input|Original ms|Swiss ms|Deferred ms|Deferred/original|Bounds|Wins / 31|Deferred/Swiss|Bounds|Wins / 31|
|---|---:|---:|---:|---:|---|---:|---:|---|---:|
|1x2/A|0.130072|0.122046|0.072841|0.555913|[0.553341, 0.563457]|31|0.596214|[0.592308, 0.600253]|31|
|1x2/B|0.204876|0.194555|0.114389|0.558627|[0.557495, 0.560756]|31|0.587754|[0.586427, 0.590905]|31|
|2x2/A|0.227865|0.225544|0.301349|1.323254|[1.319201, 1.325597]|0|1.337138|[1.330353, 1.341905]|0|
|2x2/B|0.338799|0.334818|0.446095|1.316582|[1.310533, 1.320289]|0|1.329316|[1.327437, 1.338173]|0|
|4x2/A|0.382569|0.392114|0.516237|1.347099|[1.344447, 1.353017]|0|1.315607|[1.310759, 1.320438]|0|
|4x2/B|0.591191|0.590181|0.765613|1.295614|[1.292541, 1.299118]|0|1.299825|[1.288461, 1.305866]|0|
|8x2/A|0.724797|0.761879|1.003510|1.384409|[1.380403, 1.391104]|0|1.317640|[1.314193, 1.325864]|0|
|8x2/B|1.105854|1.101093|1.445018|1.307139|[1.303524, 1.314497]|0|1.312618|[1.306622, 1.321891]|0|
|12x2/A|1.207135|1.262863|1.614857|1.340334|[1.335037, 1.347112]|0|1.277125|[1.275267, 1.293737]|0|
|12x2/B|1.801484|1.815806|2.314141|1.282560|[1.281670, 1.291878]|0|1.276951|[1.263325, 1.284098]|0|
|16x2/A|1.546290|1.600826|2.052205|1.326853|[1.320152, 1.341054]|0|1.278358|[1.265518, 1.293754]|0|
|16x2/B|2.159108|2.190942|2.811389|1.300508|[1.298029, 1.318406]|0|1.284058|[1.266340, 1.301502]|0|
|24x2/A|2.323196|2.418636|3.051941|1.313978|[1.305976, 1.337079]|0|1.261358|[1.237149, 1.283627]|0|
|24x2/B|3.722791|3.689589|4.922730|1.322895|[1.317731, 1.327371]|0|1.333690|[1.326424, 1.339348]|0|
|merge-24x1/A|2.204018|2.283549|2.905365|1.319671|[1.309345, 1.355866]|0|1.272074|[1.247030, 1.308325]|0|
|merge-24x1/B|3.487490|3.483767|4.565648|1.311584|[1.306522, 1.314785]|0|1.310548|[1.298889, 1.321534]|0|
|merge-54x1/A|5.458121|5.549248|7.668279|1.405636|[1.401600, 1.411478]|0|1.381697|[1.379751, 1.382889]|0|
|merge-54x1/B|9.480635|9.596695|12.857442|1.356926|[1.353686, 1.361891]|0|1.340273|[1.337437, 1.345137]|0|

|Equal log aggregate versus|Ratio|Bounds|Wins / 31|
|---|---:|---|---:|
|original|1.208738792|[1.206586580, 1.209804937]|0|
|swiss|1.198512259|[1.197358081, 1.199062747]|0|

Primary A improves 44.41% versus original and 40.38% versus Swiss; primary B improves 44.14% and 41.22%, respectively. Every primary comparison wins 31/31 blocks. Both primary populations decline grouping, so the new path discards only its compact dictionary. Those gains remain valid within this constructor capture.

Every admitted context fails nonregression versus the original. The all-18 aggregate is 20.87% slower versus original and 19.85% slower versus Swiss, with zero aggregate wins. Successful inputs pay the complete first pass and then hash their values again while materializing outputs; the source establishes that extra traversal, while these captures do not isolate its exclusive cost from allocation, probing or cache effects.

The prospective gate requires both primary A/B upper ratios <1 with at least 21 wins against the original and against one-pass Swiss. Every candidate/original upper ratio must be ≤1.03, and the all-18 candidate/original aggregate must have upper <1 with at least 21 wins. Other Swiss comparisons remain descriptive because Swiss is not the adopted baseline. Sorted paired ranks 10–22 give nominal 97.055% individual median coverage under independent/exchangeable blocks, not a familywise guarantee. There is no gain floor, exclusion, sample extension, seed/layout selection, subset promotion, threshold change or unchanged rerun.

Failed checks:

- `12x2/A/candidate/original/nonregression`
- `12x2/B/candidate/original/nonregression`
- `16x2/A/candidate/original/nonregression`
- `16x2/B/candidate/original/nonregression`
- `24x2/A/candidate/original/nonregression`
- `24x2/B/candidate/original/nonregression`
- `2x2/A/candidate/original/nonregression`
- `2x2/B/candidate/original/nonregression`
- `4x2/A/candidate/original/nonregression`
- `4x2/B/candidate/original/nonregression`
- `8x2/A/candidate/original/nonregression`
- `8x2/B/candidate/original/nonregression`
- `all18/candidate/original/gain`
- `merge-24x1/A/candidate/original/nonregression`
- `merge-24x1/B/candidate/original/nonregression`
- `merge-54x1/A/candidate/original/nonregression`
- `merge-54x1/B/candidate/original/nonregression`

## Correctness and CPU profiles

Eight unfiltered native component tests pass: three new deferred-dictionary tests, the three existing algorithm/output/resource oracles and two inherited FIFO tests. New tests exercise deliberately identical hashes, all four value words, row-to-class tag transitions, empty/zero/cutoff boundaries, occupied entries at the limit, a later rejecting distinct value, integer/tag boundaries and immutable inputs. The full-output oracle compares every value, class count and stable row list. Original field/core/SDK libraries and their qualification are inherited unchanged; no new full-prover or portable suite is claimed.

All 13,392 timed constructors and 162 latency primes finish before three separate FIFO-gated 199 Hz cycles:u primary-B profiles, original/Swiss/deferred, 4,096 calls each after checked primes and followed by an output check. Individual profiled outputs are dropped. CPU IDs and timestamps are explicitly recorded. Every recording finishes before ownership transfer, owner-user decoding and raw lost-record audits. No owned compiler/helper/hash/archive or second workload overlaps capture.

|Profile|Samples|Lost records|
|---|---:|---:|
|candidate|89|0|
|original|161|0|
|swiss|264|0|

Each profile has one sampled process and zero lost records. Linked common-build disassembly, observations, source/dependency pins and all outputs are retained in the archives. Raw recordings and binaries remain local with hashes. Sampled instruction residence is not an isolated cache, probe or allocator cause or a removable wall-time budget. All 16 capture/offline steps complete.

## Decision and accepted proof comparisons

This exact universal constructor does not advance under its declared gate. Positive contexts and all failed checks remain in the record. It is not promoted through a post-result subset or unchanged rerun.

A distinct next implementation could retain row classes and compact representative-index/count records during one hashing pass, materializing full scalar values only on successful admission. It would pay some metadata work on rejected inputs but avoid the second hash pass on admitted inputs. This is unimplemented and unmeasured; the current two-pass gate remains failed. It needs a new prospective protocol and whole-output oracle checks before any performance claim.

No SDK request, proof export or Gnark verification is added. The distinct-proof ledger remains **21,259**. The all-eight verified CPU 10× objective remains active and unmet, 0/8. The accepted local results below use supplied historical M5 references, not fresh Apple measurements or candidate proof timings.

|Shape|Accepted local ms|Peak RSS MiB|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|134.00|37|4.612×|24|2.992×|3.7|
|2x2|11.335969|275.00|59|5.205×|30|2.646×|5.9|
|4x2|15.945544|315.00|109|6.836×|48|3.010×|10.9|
|8x2|24.090485|512.50|189|7.845×|74|3.072×|18.9|
|12x2|33.844855|707.25|284|8.391×|101|2.984×|28.4|
|16x2|39.931557|826.25|335|8.389×|124|3.105×|33.5|
|24x2|66.233309|1328.75|528|7.972×|176|2.657×|52.8|
|merge-24x1|65.273224|1175.50|507|7.767×|175|2.681×|50.7|
|54→1|162.250464|1831.87|—|—|—|—|—|

No M5 reference was supplied for 54→1. The accepted build retains its documented failed full performance gate and physical-16 calibration engineering exception; its separate final-artifact gate passes. Latest AWS 1×2 remains Zopro 11.915591 ms at 32 workers versus Aeglos 7.959184 ms at 96. This component adds no AWS, phone/ARM/TEE, forester 250/4999, minimum-RAM, swap/pressure or concurrent-proof result. [Full local/AWS summary](../2026-10-29-current-cpu-summary/README.md).

[Protocol](artifacts/design.md), [all observations](artifacts/analysis.json), [deferred implementation](artifacts/deferred.rs), [extraction audit](artifacts/extraction-audit.json), [source archive](artifacts/source.json.gz), [receipts](artifacts/receipts.json.gz), [publication review](artifacts/publication-review.json).
