# Declared-ready arithmetic units

The current small proof has 60 long streaming-fill units: 21 for A, 19 for fused K/H and 20 for G2. They account for a median 75.3% of the **recorded leaf elapsed time** locally. These counts are identical across all 51 observations at each of 16 and 32 workers. Hundreds of short indexed-tree and weighted-reduction units provide additional work, some dependent on completed fills. Total task count alone therefore overstates the amount of sustained parallel arithmetic available to a 96-core machine.

This is a local diagnostic, not a speed improvement or a measured explanation of C7a. The full declared timing qualification fails. Production remains unchanged; no candidate is adopted. The [previous C7a review](../2026-10-26-c7a-architecture-review/README.md) motivates the measurement.

## Recorder and validation

A private copy of the accepted 170-file source adds instrumentation in five existing files and one module. Batch declarations timestamp independent units before the existing parallel iterator. Each nonforking arithmetic body records its start and finish. Thin top-window substitutions cancel their unused streaming units and record the actual indexed-tree work instead. There is no new arithmetic, key table, algorithm selection, task subdivision or executor policy.

The recorder preallocates 32,768 records, totaling 1,572,864 bytes, before priming in both OFF and ON. It uses atomic fields with one owner per record, no unsafe storage writes, bounded reservation, serialized sessions, overflow/incomplete-record errors and thread-local checks against nested leaves. There is no per-event allocation, output or environment lookup. Snapshot allocation occurs after each request timer; loop CPU counters include that diagnostic work and are not proof-only arithmetic measurements.

All 113 native core tests pass, including four new recorder tests; two raw and six cubic tests pass. Three separate analysis tests check known sweep integrals, cancelled-unit exclusion and overlap rejection. Field arithmetic is unchanged; its accepted qualification is inherited without claiming a fresh field-suite execution. Six SDK preflights pass, with six fresh exported proofs independently verified by Gnark. Every recorded snapshot validates identities, states, timestamp ordering, per-batch ordinal consistency and absence of overlapping leaves on one worker.

## Fixed local comparison

Ryzen 9950X, three rotated/reversed blocks of accepted, diagnostic OFF and ON. Primary32, physical16 pinned to CPUs0–15, and8x2-32 each use17 timed SDK requests after3 primes. Loading is excluded; witness solving, fresh proof randomness, proving, SDK verification and serialization are included. All latency precedes three separate primary32 FIFO-controlled999Hz cycles:u profiles,51 requests plus3 primes each. No owned build, archive or profile overlaps latency.

| Case | Accepted ms | OFF ms | ON ms | Paired ON/OFF |
|---|---:|---:|---:|---:|
| 1x2,32 workers | 8.588341 | 8.423921 | 8.544789 | 1.015196 |
| 1x2,16 physical cores | 15.633843 | 10.235559 | 10.124579 | 0.989157 |
| 8x2,32 workers | 25.218353 | 24.672123 | 24.839258 | 1.017558 |

Times are medians of process medians; ratios are medians of block-paired ratios. The declared gate requires OFF/accepted within a factor1.03 in every block, and ON/OFF within a factor1.03 by paired median and1.05 in every block. **The full gate fails.** All observations remain; none is excluded or repeated.

Primary16 OFF/accepted is0.656040 in block1 and0.625922 in block2; block2 ON/OFF is1.562024. The slow accepted processes record47,735 and67,830 involuntary switches across their17-request warm loops; slow ON/block2 records63,096. OFF/block2 records3,553. These counters accompany the anomaly; they do not establish its external cause or a recorder-induced change. For8x2, block2 OFF/accepted is0.956520 and also fails calibration. Primary32 passes its individual checks, but this does not overturn the failed full-matrix qualification or justify replacing accepted timings.

Maximum whole-process RSS in MiB, accepted/OFF/ON: primary32 101.25/100.50/102.50; primary16 76.25/73.00/76.75;8x2 474.25/480.00/483.75. These include loading and priming, are not minimum RAM, and do not establish a universal memory benefit.

## Observed arithmetic geometry

| Primary32 unit | Count | Median body us | Median declared-ready delay us |
|---|---:|---:|---:|
| A streaming fill | 21 | 802.7 | 24.2 |
| Fused K/H streaming fill | 19 | 2450.0 | 2.4 |
| G2 streaming fill | 20 | 2112.8 | 1085.6 |
| G1 weighted group | 475 | 28.9 | 147.4 |
| G2 weighted group | 81 | 53.6 | 6.1 |
| G1 indexed-tree tile | 204 | 24.3 | 221.7 |
| G2 indexed-tree tile | 448 | 25.4 | 229.5 |

Medians summarize each proof, then the51 proofs. Seven additional scalar-class groups bring the recorded total to1,275. These are units of the existing decomposition, not individual curve additions. A body's elapsed duration includes OS descheduling and memory stalls; it is not CPU self-time. Declared-ready delay begins at the batch declaration, not when a concrete Rayon queue entry is published.

Across the recorded primary32 arithmetic interval, the median number of simultaneously executing observed leaves is25.13. A declared unit is pending while fewer than16 leaves are observed running for1.53% of that interval. This establishes substantial local arithmetic overlap; it does not prove physical CPU occupancy, quantify idle time, or predict96-core utilization. The final recorded leaf ends a median0.169ms after the final recorded streaming fill. This is an observation within the instrumented arithmetic interval, not a full proof critical path.

The8x2 path uses a different large-G2 streaming implementation that this recorder does **not** instrument. Its lower observed leaf concurrency must not be interpreted as lower overall arithmetic utilization. Solver, FFT, scalar preprocessing, some normalization and final combination are also outside the recorder. This coverage limitation and the calibration failures prevent treating the traces as a complete critical-path analysis.

The three cycle profiles contain25,496 samples, zero lost records and one sampled process each. Accepted/OFF/ON G1+G2 category shares are68.32%/69.53%/67.54%. Aggregate cycle categories do not resolve dependency depth or memory-stall causality. The detailed symbol/category records are retained.

## Consequence and next measurement

The useful new hypothesis concerns the width and duration of the dominant arithmetic units. Fusion reduces repeated bucket work while making K/H a family of19 long streams. That can be a good trade on16 physical cores plus SMT and a different trade on96 physical cores. Additional short tasks cannot guarantee that every core has useful work throughout the long streams. This is consistent with the historical C7a scaling curve, but has not yet been measured there.

The user has requested AWS measurement. The next run should compare current Zopro and pinned Aeglos on C7a at16/32/64/96 workers, separate uninstrumented latency from recorder observations, and retain tracing calibration failures. Any later fusion ablation must hold worker entry and arithmetic primitives fixed and charge the alternative workspace; disabling an entire compile feature would also change entry behavior and confound the comparison. Prior negative indexed-tree, extra-partition and executor experiments remain evidence against transferring those mechanisms unchanged.

The [latest complete nine-shape comparison and supplied M5 CPU/Metal references](../2026-10-26-g2-owned-finish/README.md) remain separate. This diagnostic measures two shapes and establishes no new all-eight speedup. The tenfold objective remains unmet.

## Artifacts and corrections

Totals:726 SDK requests,36 distinct fresh Gnark exports,72 proof/public members,153 latency snapshots,51 profile snapshots and3 preflight snapshots. [Analysis](artifacts/analysis.json), [post-capture geometry](artifacts/geometry.json), [declared design](artifacts/design.md), [profile addendum](artifacts/profile-addendum.md), [source edits](artifacts/edits.json), [closeout](artifacts/closeout.json) and [artifact hashes](artifacts/artifact-index.json) are retained. Thirteen task JSON files are losslessly compressed; the index records compressed hashes and uncompressed hashes/sizes. Raw perf.data and executables remain local with hashes; decoded records and build metadata are archived. Reproduction requires restoring the pinned source, building with the recorded flags, and rebasing absolute local paths.

The first preflight plan was rejected for a missing schema version before any SDK call. The first performance adapter was rejected because its command did not begin with the pinned executable, before any latency/profile process. The corrected ON adapter sets its unique trace path and execs the unchanged SDK. Both initial plans/logs and repairs are retained. No successful test, preflight, timing or profile was repeated after either correction. All five whole original source files recover through inverse edits; the other165 source files and accepted SDK remain unchanged. No AWS, Aeglos, phone, pressure, concurrency or giant-circuit measurement is included in this local report.
