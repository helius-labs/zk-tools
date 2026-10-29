# One pass compact representatives for scalar grouping

The component gate **fails**. Production remains unchanged. This experiment retains compact representative-index/count records during one hashing pass and materializes full scalar values only after exact admission. It removes the previous deferred builder's second input hash pass. It targets work discarded when grouping is rejected, while preserving each query's readiness and independence.

## Algorithm

The constructor uses pinned hashbrown 0.16.1 HashTable<u32> class IDs, eight-byte records containing an original representative row and a count, and the original four-byte row classes. Full four-word equality and table-growth rehashing read immutable input words through the representative. First occurrences define class order. A new distinct value after floor(n×percent/100) rejects exactly; occupied values at that limit remain valid. Supported input length below u32::MAX bounds every index, count and sentinel.

The input is hashed in one pass, with normal table-growth rehashing. After dropping the dictionary, successful inputs materialize scalar values, usize counts and prefix starts from the records. Records are released before positions/order allocation and the original stable packing pass. Rejected inputs allocate no full scalar-value vector. No row/class tags, extra hash pass, persistent key table, plan, point copy, worker pool, unsafe code or cross-query dependency is added. Sampling and acceptance policy remain unchanged.

The [two-pass deferred constructor](../2026-10-29-deferred-groups175/README.md) improved rejected primary inputs about 44% but regressed the all-input aggregate by 20.87%. That gate remains failed. This new design accepts row-class and compact-record work on rejected inputs to avoid hashing admitted inputs twice. The [one-pass Swiss builder](../2026-10-29-swiss-groups175/README.md) remains a separate reference; its earlier SDK calibration failure is unchanged.

First-pass dictionary/record/class payload fits below the source-extracted original map/value/count/class forecast. During admitted materialization, original positions/order workspace covers the temporary records; records are freed before that workspace is allocated. Checked source-extracted forecasters and independent 32/64-bit payload inequalities verify these bounds. They do not establish allocator overhead, whole-prover RSS, minimum RAM or pressure behavior.

## Measurement method

Ryzen 9 9950X, main thread pinned to CPU 2. For all nine fixtures, the frozen accepted raw-key solver and freshly serialized solver-image solutions agree. A/B use canonical words for the first two shapes and Montgomery words for the remaining shapes, matching accepted entry geometry. All 18 sampling/admission outcomes remain recorded. No scalar or witness arrays are persisted.

Thirty-one fixed blocks rotate/reverse context and algorithm order. Every cell averages eight separately timed constructors after three checked primes per context/algorithm. All three algorithms share deterministic AHash seeds by block/call. Timers include hashing, allocation, growth, admitted scalar materialization, map destruction and stable output construction. Sampling/reservation and returned output checking/destruction are outside timing. Every prime and timed output equals an independent ordered-map oracle.

The three algorithms are the source-extracted original HashMap, the previous one-pass Swiss builder and the new deferred builder, linked in one native/fat-LTO executable using unchanged frozen accepted libraries. This is a constructor comparison in one compiled context, not independent-build calibration or proof latency. Earlier full-SDK calibration failures remain unchanged; a component pass would still require accepted/fresh-OFF/ON SDK qualification.

## Constructor results

Times are milliseconds, medians of block means. Paired ratios need not equal ratios of displayed medians. Ratios below one mean faster.

|Input|Original ms|Swiss ms|Deferred ms|Deferred/original|Bounds|Wins / 31|Deferred/Swiss|Bounds|Wins / 31|
|---|---:|---:|---:|---:|---|---:|---:|---|---:|
|1x2/A|0.134329|0.123706|0.105921|0.784977|[0.693034, 0.790494]|31|0.855281|[0.852320, 0.858115]|31|
|1x2/B|0.209229|0.196079|0.160174|0.767285|[0.764575, 0.770043]|31|0.817801|[0.815906, 0.820278]|31|
|2x2/A|0.233524|0.230828|0.231289|0.990957|[0.989253, 0.994469]|30|1.003538|[0.999612, 1.007917]|10|
|2x2/B|0.347002|0.339428|0.343060|0.988653|[0.986532, 0.993372]|30|1.010159|[1.006423, 1.016544]|3|
|4x2/A|0.385960|0.397088|0.395561|1.024557|[1.021363, 1.026696]|0|0.994335|[0.989862, 1.000016]|21|
|4x2/B|0.588013|0.590724|0.585261|0.994837|[0.992282, 0.997525]|23|0.991092|[0.983343, 0.996533]|25|
|8x2/A|0.721680|0.763432|0.750120|1.041275|[1.036019, 1.047147]|0|0.987566|[0.981101, 0.992288]|27|
|8x2/B|1.091044|1.103993|1.105886|1.014353|[1.011962, 1.022461]|0|1.001460|[0.998565, 1.013511]|12|
|12x2/A|1.208160|1.268057|1.281098|1.062608|[1.057862, 1.070905]|0|1.013509|[1.008781, 1.022556]|4|
|12x2/B|1.829163|1.850953|1.883051|1.030126|[1.027981, 1.032063]|0|1.020168|[1.015607, 1.032296]|0|
|16x2/A|1.561092|1.617027|1.644866|1.052241|[1.050297, 1.059039]|0|1.020119|[1.006735, 1.026054]|7|
|16x2/B|2.187891|2.214568|2.211627|1.010776|[1.007996, 1.021441]|1|1.003255|[0.995466, 1.014327]|15|
|24x2/A|2.317240|2.409281|2.426836|1.046848|[1.045642, 1.061652]|0|1.008219|[0.996981, 1.023521]|10|
|24x2/B|3.733238|3.724355|3.779409|1.014411|[1.011562, 1.017139]|1|1.014985|[1.011192, 1.017536]|0|
|merge-24x1/A|2.235746|2.312790|2.354306|1.054483|[1.050190, 1.074300]|0|1.018498|[1.003365, 1.041200]|6|
|merge-24x1/B|3.496228|3.507178|3.566601|1.023187|[1.016414, 1.029111]|0|1.017967|[1.010808, 1.028537]|2|
|merge-54x1/A|5.455408|5.587937|5.705901|1.043710|[1.042180, 1.047300]|0|1.022373|[1.019622, 1.024453]|0|
|merge-54x1/B|9.339551|9.362550|9.718081|1.043021|[1.038627, 1.048148]|0|1.038522|[1.033205, 1.041423]|0|

|Equal log aggregate versus|Ratio|Bounds|Wins / 31|
|---|---:|---|---:|
|original|0.996738665|[0.989779084, 0.997448289]|31|
|swiss|0.989996984|[0.988998289, 0.990332830]|31|

Primary A improves 21.50% versus original and 14.47% versus Swiss; primary B improves 23.27% and 18.22%, respectively. All four comparisons win 31/31 blocks. The all-18 aggregate improves 0.326% versus original and 1.000% versus Swiss, also with 31 wins. Eight larger constructor contexts fail the nonregression check; for example, 12×2 A is 6.26% slower versus original. All gains and regressions remain in the table.

The prospective gate requires both primary A/B upper ratios <1 with at least 21 wins against the original and against one-pass Swiss. Every candidate/original upper ratio must be ≤1.03, and the all-18 candidate/original aggregate must have upper <1 with at least 21 wins. Other Swiss comparisons remain descriptive because Swiss is not the adopted baseline. Sorted paired ranks 10–22 give nominal 97.055% individual median coverage under independent/exchangeable blocks, not a familywise guarantee. There is no gain floor, exclusion, sample extension, seed/layout selection, subset promotion, threshold change or unchanged rerun.

Failed checks:

- `12x2/A/candidate/original/nonregression`
- `12x2/B/candidate/original/nonregression`
- `16x2/A/candidate/original/nonregression`
- `24x2/A/candidate/original/nonregression`
- `8x2/A/candidate/original/nonregression`
- `merge-24x1/A/candidate/original/nonregression`
- `merge-54x1/A/candidate/original/nonregression`
- `merge-54x1/B/candidate/original/nonregression`

## Correctness and CPU profiles

Eight unfiltered native component tests pass: three representative-dictionary tests, the three existing algorithm/output/resource oracles and two inherited FIFO tests. New tests exercise deliberately identical hashes, all four value words, representative row/count tracking, empty/zero/cutoff boundaries, occupied entries at the limit, a later rejecting distinct value, row/count integer boundaries and original forecast bounds and immutable inputs. The full-output oracle compares every value, class count and stable row list. Original field/core/SDK libraries and their qualification are inherited unchanged; no new full-prover or portable suite is claimed.

All 13,392 timed constructors and 162 latency primes finish before three separate FIFO-gated 199 Hz cycles:u primary-B profiles, original/Swiss/representative, 4,096 calls each after checked primes and followed by an output check. Individual profiled outputs are dropped. CPU IDs and timestamps are explicitly recorded. Every recording finishes before ownership transfer, owner-user decoding and raw lost-record audits. No owned compiler/helper/hash/archive or second workload overlaps capture.

|Profile|Samples|Lost records|
|---|---:|---:|
|candidate|128|0|
|original|157|0|
|swiss|153|0|

Each profile has one sampled process and zero lost records. Linked common-build disassembly, observations, source/dependency pins and all outputs are retained in the archives. Raw recordings and binaries remain local with hashes. Sampled instruction residence is not an isolated cache, probe or allocator cause or a removable wall-time budget. All 16 capture/offline steps complete.

## Decision and accepted proof comparisons

The generic constructor gate fails. Positive contexts and all failed checks remain in the record; no general constructor nonregression or production adoption is claimed.

The strong primary gains, improving aggregate and bounded temporary representation justify a separate complete-SDK qualification across all nine circuits. This is an explicit decision to evaluate whole-request latency and memory under a new prospective protocol, rather than treating a phase's 3% bound as a whole-proof result. The original constructor gate remains false. The new test must include all nine latency rows, fresh accepted/OFF/ON controls and independent proof verification; it cannot promote a favorable subset of these constructor timings.

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
