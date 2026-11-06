# Typed G2 descriptor groups

**Not adopted.** Typed fixed-size descriptor groups remove bounds-check paths and reduce retired instructions, but do not establish a kernel latency gain. The two-size 32-worker ON/OFF estimate is 0.999659 [0.992119,1.011413],8/15 wins. Both solo cases lose all 15 blocks; 16-worker cases are 1.18%/2.50% slower. No complete-query or full-SDK follow-up is selected.

All six G2 control calibrations pass. The unchanged G1 controls fail calibration at all three worker counts, and remain in the result. Neither those failures nor the small positive 32-worker aggregate point estimate changes the failed declared gate. Accepted source 176 / SDK 6d83 and proof timings remain unchanged.

## Change and actual linked code

The original borrowed G2 kernel indexes a flat descriptor slice by k+i and reverse prefix records by j. The candidate traverses batch.as_chunks::<LANES>() and zips the reverse fixed groups with their corresponding records. It retains bucket destination bounds checks, the original single prefix chain, scalar-tail/inversion body, signed-y handling, field/point formulas and record layout. There is no unsafe operation, new allocation, key table, point array, plan, runtime pool or field change. Production dispatch remains exact; only a diagnostic child calls the clone.

This differs from the separately rejected [two prefix chains](../2026-11-06-g2-two-chain-stream176/README.md), [cold-tail outlining](../2026-10-30-g2-cold-tail175/README.md) and [fused-field outlining](../2026-10-29-g2-fused-outline175/README.md). It adds no field operations. Reversing two loop-section edits recovers the original apply exactly, including scalar-tail and point-finishing statements.

|Linked G2 function|OFF|ON|
|---|---:|---:|
|Bytes|51,830|50,614|
|Instruction sites|8,590|8,391|
|Bounds-panic call sites|36|19|
|Conditional jump sites|96|69|
|IFMA sites|1,450|1,450|
|RSP operand lines|1,450|1,396|

The functions occupy distinct addresses in the same native O3/fat-LTO/one-CGU executable, SHA-256 740708e5… . Static sites are not dynamic executed checks, spills, complete stack bounds or causal stall costs. Original FROZEN is the exact pre-existing source176 caller f07229ce…, not the accepted full SDK. The reference audit rechecks 385 pins, matching driver/G1, inverse G2fixture edits, source recovery and optimized consumer commands. All new registry entries match old versions/checksums; the old lock has extra unused entries.

## Fixed kernel screen

Ryzen 9950X, 1/16/32 workers pinned to CPU2/CPUs0–15/0–31 with 512 KiB stacks. Fifteen fixed rotated/reversed blocks compare FROZEN/OFF/ON. Three checked 128-batch primes precede reset and 4,096 timed changing batches per worker. Ref assembly, dispatch, arithmetic and waiting are timed; construction/reset/final generator checks are outside. These are whole-wave means in microseconds, **not proof times**.

|Context|FROZEN/OFF/ON µs|Paired ON/OFF [bounds]|Wins/15|
|---|---:|---|---:|
|G2-200-w1|7.674205/7.678419/7.810898|1.017175 [1.016554,1.018467]|0|
|G2-200-w16|8.917648/8.936164/9.050584|1.011803 [1.007469,1.017129]|1|
|G2-200-w32|14.388367/14.353739/14.349316|1.008850 [0.987101,1.027982]|7|
|G2-352-w1|12.986677/12.980796/13.246492|1.020096 [1.019839,1.020362]|0|
|G2-352-w16|15.212135/15.180806/15.556118|1.024950 [1.020186,1.027463]|1|
|G2-352-w32|25.189385/25.086911/25.100240|0.997161 [0.988355,1.015206]|9|
|G1-200-w1, unchanged|4.212282/4.396133/4.390220|0.998655 [0.996021,1.000277]|11|
|G1-200-w16, unchanged|4.764309/4.946851/4.943588|0.997952 [0.995149,1.000634]|11|
|G1-200-w32, unchanged|6.847690/7.070099/7.039299|0.998197 [0.991505,1.006281]|9|

Two-size 32-worker ON/FROZEN is 0.997741 [0.988480,1.008548],9/15 wins. Marginal displayed medians and paired ratios need not divide into one another. Both small positive aggregate estimates remain reported, with intervals spanning parity.

The prospective gate requires all OFF/FROZEN intervals within factor 1.03; all ON/control upper bounds≤1.03; and both 32-worker G2 sizes plus their equal-log aggregate upper<1 and≥10 wins against BOTHcontrols. It fails 13 checks: six gain checks, G2-200-w32 ON/FROZEN nonregression, three G1 calibrations and three G1 ON/FROZEN nonregressions. All nine contexts and raw blocks remain. Rank 4–12/15 bounds have nominal 96.484375% marginal median coverage under independent/exchangeable blocks, without familywise coverage. No minimum gain floor, exclusion, extension, attribute/compiler rescue, favorable subset or unchanged reroll is used.

## CPU, memory and validation

All 405 latency processes finish before six single PMU panels and six separate fixed 16M cycles:u profiles. FIFO excludes primes/final checks and the complete runtime environment is forwarded under sudo. Every event runs 100%; these single panels are descriptive, not confidence intervals.

|32-worker panel|Instructions/wave OFF→ON|Cycles/wave OFF→ON|
|---|---:|---:|
|G2-200|3,557,188→3,497,356|1,885,076→1,901,586|
|G2-352|6,033,159→5,927,702|3,251,366→3,267,120|

Instructions fall 1.68%/1.75%, while cycles rise 0.88%/0.48%. The structural work removal did not supply a qualified latency win. Generic cache observations and code size do not establish an instruction-cache, bandwidth, frequency or scheduling cause.

Scratch payloads per worker remain [16,000,0]/[28,160,0] bytes at 200/352, with zero timed allocations and stable primed capacities across all arms. For G2-200/32, maximum latency-process RSS counters are 7.75/7.50/7.75 MiB (FROZEN/OFF/ON). RSS is diagnostic, can undercount and does not establish precise peaks, minimum RAM or pressure/mobile feasibility. No new heap buffer is introduced; compiler stack behavior remains additional.

The unfiltered native core suite passes 127 tests: 125 originals plus two existing signed-prefix/vector-sign oracle populations reapplied to the changed function. These cover independent generator sums, signs, tails, repeated use and 1/4/16/32 workers with 512 KiB stacks; they are not new mathematical populations. Field/prover/image qualification is inherited unchanged. The ordinary nonzero-denominator/distinct-destination contract remains. All 27 preflights pass independent generator checks and cross-arm digest/capacity equality before latency. The 444 caller processes cover 45,708,864 kernel batches including primes, not proofs; intermediate timed states are not individually checked.

All 3 native + 475 capture commands succeed first time. The separate reviewer verifies 427 current pin occurrences + 385 inherited reference pins, 176-source recovery, exact caller inverses, actual argv/environment/stream hashes, every raw latency/CPU/RSS/gate and profile bin/ownership/CPU/period. It passes once. Its inherited output count field 128 is corrected to 127 against the same core summary, which the reviewer already checked as 127; before/after hashes and correction source are retained. No successful review, test or capture is rerun for this metadata correction. Static code counts are separate helper observations, not an independently recounted performance model.

All 63,027 profile samples qualify with zero lost records and unknown shares below 0.091%. Inlining and sampling prevent interpreting symbol periods as removable wall time or a critical-path decomposition. No owned compiler/helper/hash/archive/analyzer/second benchmark overlaps capture.

## Accepted proof comparisons, unchanged

Inherited local 32-worker full-proof observations and supplied M5 references; no new Apple or proof timing follows.

|Shape|Accepted ms|M5 CPU ms /speedup|M5 Metal ms /speedup|CPU 10×limit ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37 /4.667×|24 /3.027×|3.7|
|2×2|11.151932|59 /5.291×|30 /2.690×|5.9|
|4×2|16.086386|109 /6.776×|48 /2.984×|10.9|
|8×2|24.179453|189 /7.817×|74 /3.060×|18.9|
|12×2|33.884455|284 /8.381×|101 /2.981×|28.4|
|16×2|40.077381|335 /8.359×|124 /3.094×|33.5|
|24×2|65.850777|528 /8.018×|176 /2.673×|52.8|
|merge24|65.060937|507 /7.793×|175 /2.690×|50.7|

Merge54 remains 161.883899 ms without an M5 reference; accepted primary RSS 132.75 MiB is inherited. The all-eight CPU 10× goal remains active/unmet 0/8, ledger 30,846 unchanged. No SDK/Gnark export, new AWS/Aeglos comparison, phone, huge-circuit, swap, pressure, minimum-RAM or concurrent-throughput result is added.

[Analysis](artifacts/analysis.json), [review](artifacts/review.json), [count correction](artifacts/review-count-correction.json), [source recovery](artifacts/source-audit.json), [protocol](artifacts/protocol.md), [linked code](artifacts/codegen.json), lossless sources/receipts/raw profiles and [checksums](SHA256SUMS) retain the experiment. Full original analysis is archived; public JSON drops only duplicated caller/command receipts. Runtime point/scalar/key/witness arrays and executable/library blobs are excluded.
