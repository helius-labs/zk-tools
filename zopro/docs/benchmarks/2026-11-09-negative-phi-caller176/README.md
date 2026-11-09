# Complete forward transform with a short negative factor

At 2,048 points and 32 workers, the new forward transform is **12.39% faster than fresh ordinary SIMD**: paired ratio **0.876102 [0.871919, 0.880644]**, 31/31 wins. It improves against the original-field SIMD control as well, ratio 0.874087 [0.870538, 0.878074], 31/31 wins. Every candidate/control comparison wins all its blocks across the six contexts.

**The declared overall gate is false.** Its sole failure is the 64-point/32-worker original/fresh-control calibration. The other 36 checks pass. Production is unchanged, and no full-query or proof gain is established. A separately documented **engineering exception to the declared closure rule** selects a distinct original-order G2 query experiment; it does not relabel this screen passed. The exception was not preauthorized in the protocol. [Decision](artifacts/decision.json).

## Same-work construction and controls

All three arms compute the original forward cubic map, multiplying point X by β. The [preceding qualified arithmetic](../2026-11-09-negative-phi-short176/README.md) uses `β = −(β²+1) mod p`: a short 191-bit factor, signed low accumulators, and a zero-input correction. It has 72 IFMA sites versus 100 for ordinary SIMD. No scalar component swap, changed bucket stream or final query correction is needed. Its exact native library, tests and already-linked fresh caller are reused without rebuilding or retesting.

REF is the original-field frozen executable from the [inverse-map study](../2026-11-09-inverse-phi-small176/README.md), requested only in its **forward ordinary-SIMD** mode. Its unavailable fixed mode is never requested. The original field, wrapper source, saved code decode and successful link receipt remain bound. OFF and ON share the new coherent executable: OFF uses original forward SIMD, ON uses the short negative factor. Thus this comparison does not equate an inverse map with a forward map. The original scalar production path is a weaker historical baseline and is not newly timed here.

The native model and three focused tests are inherited: 6,920 exact BN254 outputs, 1,048,575 toy-radix inputs, 128 Arkworks curve identities; 36,928 native Arkworks lane outputs, 14,610 complete point outputs and 53 small-stack workers. No unchanged model, field/core/prover suite, oracle, key or witness is rerun. Actual linked counts remain those in the preceding report: short factor 72 IFMA/282 instruction/6 RSP-operand sites versus SIMD 100/286/12, including alignment padding. Static counts do not measure executed work, spills or stack peaks.

The implementation adds 64 logical constant bytes and no per-key table, retained key/witness/point/scalar copy, plan, input-sized array, production pool or artificial RAM cap. Compiler constants/stack and the benchmark's worker-owned fixtures are additional; benchmark fixture copies are not a proposed prover cache.

## Fixed complete-caller protocol

Ryzen 9950X, 16 physical/32 logical CPUs. The existing executables use Rust 1.98.1, native O3, fat LTO, one codegen unit and debug information. Workers are standard threads with 512 KiB requested stacks, individually pinned to CPU 2 solo or their index in CPUs 0–15/0–31. No new Rayon or concurrent-proof qualification is inferred from these thread waves.

Fifteen fixed rotated/reversed matched process blocks cover 64/2,048 points and 1/16/32 workers; the primary 2,048/32 case has 31 blocks. Each worker performs three checked primes of 16 cycles, then 192 measured cycles, with three transforms per cycle. Packing, multiplication, export, stores, scalar tails, loop and black-box costs are included. Fixture generation, thread startup, affinity, barriers and final checks are outside each worker clock. The statistic is maximum integer worker elapsed divided by 576 transforms, with medians of process means displayed.

Inputs are public generator multiples with deterministic identity slots, not captured witness-bucket arrays. Every prime phase and final timed worker result restores the initial coordinates after cubing; capacities remain unchanged. Intermediate timed states are not individually checked. Correct forward orientation is inherited from the bound native qualification. This measures transform waves, not proof latency, proof scheduling, exclusive critical-path time or concurrent-proof throughput.

All 18 preflights pass before 318 latency processes. Six four-event PMU panels and six profiles follow, each using 32,768 cycles per worker. All recording finishes before 30 offline steps. The **378-command workflow covers 348 caller processes and 32,677,362 complete transforms**, including primes. Actual handle 72593 is polled through terminal exit 0; no owned build/helper/hash/analysis/decoder/verifier/report/archive/push overlaps recording. Exclusive use of the shared host is not claimed.

|Points/workers|REF µs|OFF µs|Short factor µs|Paired ON/OFF [95% interval]|Wins|
|---|---:|---:|---:|---:|---:|
|64 / 1|0.372073|0.369847|0.267953|0.732727 [0.705412, 0.769291]|15/15|
|64 / 16|0.397972|0.397486|0.286026|0.721762 [0.714778, 0.726822]|15/15|
|64 / 32|0.550205|0.552882|0.488543|0.845510 [0.785257, 0.892066]|15/15|
|2048 / 1|11.680377|11.750179|8.474521|0.723314 [0.721527, 0.724981]|15/15|
|2048 / 16|12.851351|12.769965|9.392536|0.735374 [0.731135, 0.739951]|15/15|
|2048 / 32|19.178561|19.172924|16.725844|0.876102 [0.871919, 0.880644]|31/31|

Paired geometric ratios need not divide the marginal medians. The paired-log bootstrap uses 20,000 resamples, seed `0x551c36a48ee2d971`, and sorted ranks 499/19,499. It assumes exchangeable independent blocks and gives no familywise guarantee. The 2,048-point gains against OFF are about 27.67%/26.46%/12.39% at 1/16/32 workers. No arbitrary positive-gain floor is imposed.

The equal-log aggregate across both sizes at 32 workers is 0.860670 [0.829399, 0.884296] against OFF and 0.878206 [0.869464, 0.886271] against REF. Sizes are resampled independently within their 31/15 blocks; cross-size correlation is not assumed. Both aggregate checks pass, without an all-eight inference.

## Failed calibration and selection exception

The prospective 37 checks require all six OFF/REF intervals inside factor 1.03, all twelve candidate/control nonregression bounds ≤1.02 and maximum RSS ratios ≤1.05, primary gains with ≥21/31 wins against both controls, both size aggregates improving, and all diagnostics qualifying. Every check passes except:

|Failed check|Paired ratio [95% interval]|Required|
|---|---:|---|
|64 / 32 OFF/REF calibration|1.043564 [0.987906, 1.120194]|Entire interval within [1/1.03, 1.03]|

The primary control calibrates at 0.997700 [0.995003, 1.000542]. All other five calibration contexts pass. Candidate/REF at 64/32 is 0.882344 [0.865188, 0.897698], 15/15 wins; candidate/OFF also wins 15/15 there. The broad control interval remains unexplained. No observation is excluded, extended or rerun, and the failed check is not waived in the gate calculation.

The declared failure rule would stop before query integration, with no exception preauthorized. The explicit engineering decision selects a **new endpoint and implementation** because both primary comparisons and its control calibrate, every candidate block wins against both controls, and all nonregression/resource/diagnostic checks pass. It preserves the failed overall gate and changes neither thresholds nor sample membership. This is investigative selection, not production adoption or a claim that the small-array controls were reliable. The new query must freeze and pass its own controls/gates before SDK selection. The earlier inverse-query failures remain closed.

## Process resources and automatic CPU profiles

|Points/workers|Maximum process RSS, all arms MiB|Median CPU ms/wave OFF / ON|Median minor faults/wave OFF / ON|
|---|---:|---:|---:|
|64 / 1|2.25|0.000389 / 0.000288|0.005208 / 0.005208|
|64 / 16|2.00|0.006314 / 0.004550|0.003472 / 0.003472|
|64 / 32|2.25|0.018009 / 0.015497|0.003472 / 0.010417|
|2048 / 1|2.75|0.011773 / 0.008488|0.005208 / 0.005208|
|2048 / 16|6.25|0.197436 / 0.145441|0.003472 / 0.005208|
|2048 / 32|10.50|0.592000 / 0.512773|0.003472 / 0.003472|

Lifetime RSS includes fixtures, startup, primes and allocator retention. Equal observed peaks do not establish allocation-free execution, pressure/swap behavior, full-prover RAM or minimum RAM. Minor faults are mapping events without disk reads, not a DRAM-bandwidth measurement. Allocation counts and major faults are not instrumented by this driver.

All six profiles qualify: **65,069 samples**, zero LOST, at most 0.081% unknown, assigned PID/TID/CPU, executable MMAP/FORK and fixed 16M user-cycle period checked. Target transform symbols account for at least 99.919% of each profile. All 24 grouped PMU event values run 100%. Absolute cycle-period coverage is not independently validated, and single counter panels are descriptive.

|Workers|Instructions OFF / ON per transform|Cycles OFF / ON per transform|
|---|---:|---:|
|16|127,015.698 / 125,480.693|63,308.141 / 45,810.316|
|32|127,015.710 / 125,480.705|86,083.492 / 73,028.917|

The 32-worker panel has about 1.21% fewer instructions and 15.17% fewer cycles. At 16 workers cache misses stay almost equal while cycles decrease; there is no uniform cache explanation. Profiles and code establish that the intended arithmetic executes. They do not isolate a particular execution port, clock, spill or cache cause, nor give removable proof time.

## Review and next query

The frozen reader passes reconstruction of all raw worker maxima, calls, 318 latency rows, paired statistics, both size aggregates, all 37 checks and the single failure. It checks all 418 pins, 378 command/exit/stream receipts and all 176 unchanged production hashes. It independently implements the size aggregate but shares the paired bootstrap and detailed profile/PMU parsers with the producer. This is same-agent review, not external audit. Float comparisons use 1e−12 relative tolerance; integers/hashes are exact. The original model, native tests, library, consumers, decoder and captures are not replayed.

One controller compile failed on a duplicate median helper before any recording. Removing that local duplicate reuses the already-included median; original source, pins and compiler receipt remain. Three corrected helper builds pass once. Unused helper warnings remain without cleanup rebuilds. Preparation, prospective review, analysis and reader execution have tool-transcript evidence; capture commands have full workflow receipts.

The selected query replaces only high-affine forward-map loops with the qualified four-point short-factor map. Original scalar split/component order, β projective spills, width/partition policy, indexed top, grouping, queues, admission, tails, weighting and final result remain. It adds no final correction or input-sized point/scalar array. This query is not implemented or timed in this report. Fresh original-field/fresh-OFF/ON controls, native full-query correctness and automatic profiles are required. Full SDK, ALL8 plus Merge54, independent Gnark, resource/admission, portable and concurrent-proof qualification remain required before adoption.

## Accepted full-proof CPU and Metal comparison, inherited

These preceding accepted-library timings use fixed worker placement; it is not an adopted production default. Supplied M5 references use different hardware. Historical standard 1×2 **7.915095 ms** remains valid; latest standard **8.075143 ms** and fixed **7.869302 ms** retain their separate conditions. No accepted code regression or new proof record is established by this transform experiment.

|Shape|Accepted fixed ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.869302|37 /4.702×|24 /3.050×|3.7|
|2×2|10.984868|59 /5.371×|30 /2.731×|5.9|
|4×2|15.734200|109 /6.928×|48 /3.051×|10.9|
|8×2|23.929023|189 /7.898×|74 /3.092×|18.9|
|12×2|33.722646|284 /8.422×|101 /2.995×|28.4|
|16×2|39.776845|335 /8.422×|124 /3.117×|33.5|
|24×2|66.183856|528 /7.978×|176 /2.659×|52.8|
|Merge24×1|65.448733|507 /7.747×|175 /2.674×|50.7|

Source: [preceding full campaign](../2026-11-09-weighted-prefix-fixed-sdk176/README.md). Merge54 remains 162.489760 ms without an M5 reference. Accepted176/SDK6d83 and canonical **38,668** verified exports stay unchanged. There are zero new SDK requests/Gnark exports, AWS resources, Aeglos runs, mobile/huge-circuit/pressure/swap/minimum-RAM or concurrent-proof results. The full verified all-eight CPU10× goal remains active and unmet, **0/8**.

[Prospective design](artifacts/design.md), [all observations](artifacts/analysis.json.gz), [prospective review](artifacts/prospective-review.json), [result review](artifacts/review-result.json), [explicit selection exception](artifacts/decision.json), [continuation loop](artifacts/next.md), [source/receipts/decoded profiles](artifacts/evidence.json.gz), [publication checks](artifacts/publication.json), [checksums](SHA256SUMS). The archive retains exact UTF8 member bytes/hashes and gzip roundtrip. Binaries/libraries, raw perf.data, unchanged full source, keys/witnesses/proofs, the private proof inventory and FIFO contents are excluded. Its prelaunch note describes its original time, not current liveness.
