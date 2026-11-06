# Four-word Fq export caller screen

The alternative export is correct and modestly faster in solo callers, but **fails the declared complete-caller performance gate and is not adopted**. No full-query or SDK trial follows this screen. Accepted proof timings remain unchanged.

Against fresh OFF, solo G2 callers improve by 0.64%/0.68% and G1 by 0.56%/0.58% at sizes 200/352; all four comparisons win 15/15 blocks. At 16 workers, small G2 improves 1.37% while larger G2 regresses 0.50%. At 32 workers, both curves fail their aggregate gain tests. Two control calibrations and four nonregression bounds also fail. The positive estimates, wide G1 intervals and control drift are retained.

| 32-worker two-size aggregate | Median ON/control ratio | Interval | Wins |
|---|---:|---:|---:|
| G1 / fresh OFF | 1.007495 | [0.910151, 1.114053] | 5/15 |
| G1 / frozen original | 0.926894 | [0.892624, 1.102705] | 10/15 |
| G2 / fresh OFF | 0.996233 | [0.980508, 1.003587] | 9/15 |
| G2 / frozen original | 1.007354 | [0.999324, 1.013090] | 4/15 |

## Change and correctness

Original mixed-R256 Fq export subtracts p across five radix-52 digits, then packs four 64-bit word vectors. The candidate packs first, subtracts p with exact unsigned word borrows, selects canonical words and uses the original transpose/store body. Normalized Fq values strictly below 2p fit losslessly because 2p < 2^255; one subtraction suffices. Each modulus word is below u64::MAX, so adding incoming borrow cannot overflow. Inclusive 2p, unnormalized digits, other moduli and uniform R260 are outside this alternative's contract.

The field snapshot has 177 files with 175 originals exact. The caller snapshot has 178 files with 172 originals exact; inverse edits recover all 176 accepted files. Changes comprise an append-only field include, one cfg-only native eight-lane FqBatch::get dispatch, two diagnostic exports and two children. OFF arithmetic, generated bodies/generator, Fr and other radices remain exact. No key table, format, heap buffer, pool or memory cap is added. Earlier delta-input packing and early-X canonical reuse remain separate, closed experiments.

Two focused field tests pass: **19,216 boundary/random lanes and 131,072 changing arithmetic-chain lanes**, compared with scalar word subtraction, original export and Arkworks. Coverage includes 0/p/2p−1, bit/word/radix boundaries, all 256 per-lane lazy selections and all eight admissible low-word borrow patterns. Original 35 field tests are filtered and inherited. ON passes all **127 unfiltered core tests**, including two reapplied complete-generator/tails/reuse/small-stack threaded test populations. No new mathematical populations or prover/Gnark qualification are claimed.

The qualified field library is reused without rebuilding. New OFF/ON core libraries use that field and immutable accepted support dependencies coherently. Final callers use actual O3/native/fatLTO/oneCGU flags. Original G1/G2 adapter bodies remain exact. The pre-existing frozen control's source, original arithmetic, driver/adapters, pinned binary and optimized final command are independently checked; it is a kernel control, not the accepted SDK.

## Protocol and validation

Ryzen 9950X, sizes 200/352, both curves, 1/16/32 workers. Main/solo CPU 2; parallel workers CPUs 0–15 or 0–31 with 512 KiB stacks. Fifteen fixed rotated/reversed blocks compare frozen original, fresh OFF and ON. Each process performs three reset 128-round checked primes, resets, then 4,096 changing timed batches per worker. Ref assembly, dispatch, arithmetic and waiting are timed. Each worker owns a complete fixture: wave time is not proof latency or concurrent-proof throughput. Every prime and final state matches independent generator-scalar sums; intermediate timed states are not individually checked.

All 36 preflights match cross-arm hashes/capacities. All 600 caller processes have zero timed allocations and stable primed capacities. They execute **67,621,632 measured batches plus 3,907,584 priming batches**, not proofs. All 540 latency processes precede 12 PMU panels and 12 profiles; 60 offline ownership/decode/event/loss steps follow. No owned build, hashing, archive, analyzer or second workload overlaps capture.

All 661 capture commands succeed first time. Independent review reproduces 820 pins, 675 successful workflow commands across all stages, source recovery, library/link identities, outputs/capacities, paired statistics, all ten failed checks and 93,849 profile samples. All profiles meet fixed-period/PID/TID/worker-CPU/count/unknown qualifications with zero lost records; unknown share is at most 0.105%. Absolute period coverage, exclusive CPU service and causal critical-path cost are not established. A supplementary raw-CSV reader checks all 12 counter panels and running coverage.

Intervals use paired order-statistic ranks 4–12 of 15: nominal 96.484375% marginal coverage under independent exchangeable blocks, without a simultaneous guarantee. All 12 OFF/frozen intervals must remain within factor 1.03. All 24 ON/control nonregression upper bounds must be ≤1.03. Both curves' 32-worker aggregates must have upper ratio <1 and at least ten wins against both controls. There is no minimum gain floor or individual gain requirement. Failed gates are preserved; no subset promotion, sample extension or unchanged reroll follows.

Preparation failures and repairs remain archived: ambiguous getrandom versions, a repair-generator raw-string delimiter, missing Cargo metadata for wasm-bindgen macros, and a campaign spacing typo. The main review succeeds first time. The supplementary counter reader initially rejected a one-step floating-point JSON round-trip difference; raw counts remain exact, with a 1e−14 relative tolerance only for derived per-wave values. No successful field test/build/codegen/core test/link/capture was repeated for these repairs.

## CPU work and decision

Standalone export wrappers shrink from **62 instruction sites / 417 bytes to 54 sites / 375 bytes**. This is a static count, not an isolated timing result. Complete 32-worker G2 panels count about **1.06%/1.10% fewer instructions** and **0.68%/0.76% fewer cycles**. G1 instructions are essentially unchanged (+0.013%/+0.054%); cycles are mixed. Standalone savings therefore do not establish the same instruction reduction across complete callers. These are separate single PMU panels without confidence intervals. Cache-miss counts identify neither stall cost nor a cause of latency variation.

Fixed vector state adds no heap payload in this scope; compiler stack/register effects remain real. Process RSS counters are diagnostic, not minimum RAM or pressure qualification. No new full-proof, Aeglos/AWS, phone/ARM/TEE, large 250/4999 circuit, swap, pressure or concurrent-throughput result is added. This screen establishes no new 2× path.

Close this implementation at the failed caller gate. Retain the small solo gains and counted G2 work reduction. A further experiment must remove distinct useful work or a dependency; do not rerun this export configuration or tune its layout/attributes.

## All complete-caller contexts

Times are median microseconds per batch wave. Ratios are medians of paired block ratios and need not equal ratios of the displayed medians. All raw rows, CPU/RSS counters and failed gate names are archived.

| Context | Frozen µs | OFF µs | ON µs | ON/OFF [interval] | Wins vs OFF | ON/frozen [interval] | Control calibration |
|---|---:|---:|---:|---|---:|---|---|
| G2-200-w1 | 7.669 | 7.669 | 7.623 | 0.993564 [0.992388, 0.994557] | 15/15 | 0.994023 [0.993725, 0.994603] | pass |
| G2-200-w16 | 8.912 | 8.983 | 8.852 | 0.986308 [0.984313, 0.986955] | 15/15 | 0.992913 [0.987837, 0.995237] | pass |
| G2-200-w32 | 14.301 | 14.613 | 14.519 | 0.998823 [0.969470, 1.016807] | 8/15 | 1.011900 [1.004573, 1.030856] | fail |
| G2-352-w1 | 12.987 | 12.989 | 12.899 | 0.993185 [0.992570, 0.993642] | 15/15 | 0.993640 [0.992976, 0.994002] | pass |
| G2-352-w16 | 15.194 | 15.245 | 15.327 | 1.004968 [1.001543, 1.006778] | 2/15 | 1.008396 [1.005804, 1.009931] | pass |
| G2-352-w32 | 25.112 | 25.217 | 24.944 | 0.991735 [0.979477, 1.008107] | 9/15 | 0.995933 [0.989992, 1.002471] | pass |
| G1-200-w1 | 4.212 | 4.216 | 4.197 | 0.994421 [0.993832, 0.997043] | 15/15 | 0.997376 [0.995688, 0.998368] | pass |
| G1-200-w16 | 4.770 | 4.784 | 4.757 | 0.992971 [0.990762, 0.996724] | 14/15 | 0.993924 [0.989296, 1.003290] | pass |
| G1-200-w32 | 6.899 | 6.897 | 6.841 | 0.997304 [0.846277, 1.225461] | 10/15 | 0.992471 [0.881523, 1.222652] | pass |
| G1-352-w1 | 6.931 | 6.941 | 6.909 | 0.994185 [0.993879, 0.995455] | 15/15 | 0.995754 [0.995007, 0.997012] | pass |
| G1-352-w16 | 7.925 | 7.952 | 7.902 | 0.993616 [0.989821, 0.996784] | 14/15 | 0.997948 [0.995233, 0.999915] | pass |
| G1-352-w32 | 14.475 | 11.596 | 11.604 | 1.005019 [0.974770, 1.205382] | 5/15 | 0.811962 [0.789681, 1.023419] | fail |

Failed checks:

```text
G2-200-w32/OFF/frozen/calibration
G2-200-w32/ON/frozen/nonregression
G1-200-w32/ON/frozen/nonregression
G1-200-w32/ON/OFF/nonregression
G1-352-w32/OFF/frozen/calibration
G1-352-w32/ON/OFF/nonregression
G1-32aggregate/ON/frozen/gain
G1-32aggregate/ON/OFF/gain
G2-32aggregate/ON/frozen/gain
G2-32aggregate/ON/OFF/gain
```

## Accepted CPU and Metal comparisons, unchanged

Inherited local 32-worker full-proof observations and supplied historical M5 references follow. They are not new measurements.

|Shape|Zopro ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× limit ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37 / 4.667×|24 / 3.027×|3.7|
|2×2|11.151932|59 / 5.291×|30 / 2.690×|5.9|
|4×2|16.086386|109 / 6.776×|48 / 2.984×|10.9|
|8×2|24.179453|189 / 7.817×|74 / 3.060×|18.9|
|12×2|33.884455|284 / 8.381×|101 / 2.981×|28.4|
|16×2|40.077381|335 / 8.359×|124 / 3.094×|33.5|
|24×2|65.850777|528 / 8.018×|176 / 2.673×|52.8|
|merge24|65.060937|507 / 7.793×|175 / 2.690×|50.7|

Merge54 remains 161.883899 ms without an M5 reference. Primary RSS132.75 MiB is inherited. Source176/SDK6d83 and the 30,846-proof ledger are unchanged. The all-eight CPU10× objective remains active/unmet, **0/8**. No new AWS/Aeglos/Gnark, phone, huge-circuit, pressure, swap, minimum-RAM or concurrent-throughput result is added.


[Analysis](artifacts/analysis.json.gz), [main review](artifacts/review.json), [counter review](artifacts/counter-review.json), [caller design](artifacts/caller-design.md), [source audit](artifacts/caller-source-audit.json), [library and caller identities](artifacts/caller-identities.json), [capture plan](artifacts/plan.json), [pins](artifacts/pins.json), [exact candidate source archive](artifacts/candidate-source.json.gz), [exact helpers/receipts archive](artifacts/helpers-and-receipts.json.gz), [publication](artifacts/publication.json), [checksums](SHA256SUMS). Twelve lossless raw cycle profiles are included under artifacts. Executables, libraries, private keys/images/witnesses/proofs and external copyrighted source are excluded.

The candidate source archive uses lossless hex for all178files, preserving checked-in binary test vectors. The initial publisher treated source as UTF-8 and stopped at a binary test-vector file; its source/log remain archived. Publication resumed only at source/receipt/profile archives and checksums, reusing successful report/artifact/analysis writes. No native test, build, measurement or review was repeated. See [publication repair](artifacts/publication-repair.md).
