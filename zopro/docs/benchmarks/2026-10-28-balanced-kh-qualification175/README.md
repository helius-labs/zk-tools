# Balanced K/H windows: full SDK performance qualification

The 32-worker primary gain repeats, but the full qualification gate is **false** because the 16-worker baseline calibration fails. No production switch is made. Candidate 1x2 takes **8.039940 ms** versus accepted **8.105814 ms**; paired ON/accepted improves **0.813%**, with bounds [0.984037,0.996694] and 13/15 wins. Every candidate gain/nonregression check passes; the sole failed check is `1x2-w16/OFF/accepted`. The passing pilot remains separate evidence.

## Scope and method

This reuses the exact [passing pilot](../2026-10-28-balanced-kh-windows174/README.md) source 175 and executables at their original paths. Balanced 12/13-bit signed spans replace the fused K/H width 13 top-window topology, retaining field kernels, borrowed queues, complete exceptions and the conservative original memory forecast. No code, key, driver, flag or resource policy changes in this stage. Native 122/124 core and eight prover tests per build, plus inherited 33 field tests, are reused with checked source/library pins; no new native tests/builds/links execute. The native receipts in this archive are explicitly inherited evidence.

All nine shapes run at 32 workers / CPUs 0–31; primary also runs at 16 physical workers / CPUs 0–15 on Ryzen 9950X. Fifteen fixed rotated/reversed blocks each compare accepted/OFF/ON in independent processes. Primary processes execute 151 complete requests after three primes; others 31. Solving, fresh blinding/proving, SDK verification and serialization are timed; loading is excluded. No exclusive-host claim is made.

All 30 preflight exports verify before 450 latency processes. The ON primary preflight at 32 workers alone traces four route records, checked in its Gnark step before latency. All 450 latency exports verify after all latency. Twelve diagnostic processes then run before their independent verification/decode. No owned build/helper/hash/archive/second benchmark overlaps captures.

## Complete requests

Values are medians of the fifteen process medians, in milliseconds. They are not paired-ratio estimates.

| Case | Accepted ms | Reused OFF ms | ON ms |
|---|---:|---:|---:|
| 1x2-w32 | 8.105814 | 8.206804 | 8.039940 |
| 2x2-w32 | 11.291047 | 11.386969 | 11.313311 |
| 4x2-w32 | 16.020905 | 16.047824 | 16.021489 |
| 8x2-w32 | 24.082256 | 24.235737 | 24.042427 |
| 12x2-w32 | 33.840266 | 33.759778 | 33.734127 |
| 16x2-w32 | 40.034518 | 40.188599 | 40.096854 |
| 24x2-w32 | 66.310989 | 66.137073 | 66.143755 |
| merge-24x1-w32 | 65.345371 | 65.677673 | 65.132194 |
| merge-54x1-w32 | 161.306709 | 161.116998 | 161.575926 |
| 1x2-w16 | 9.775631 | 9.954227 | 9.623225 |

| Case | ON/accepted paired [bounds] | Wins | ON/OFF paired [bounds] | Wins |
|---|---|---:|---|---:|
| 1x2-w32 | 0.991867345 [0.984036789,0.996693778] | 13 | 0.980204933 [0.974546364,0.982931584] | 15 |
| 2x2-w32 | 1.004779185 [0.991079812,1.009016782] | 7 | 0.994486018 [0.983513976,1.003238875] | 10 |
| 4x2-w32 | 1.006363005 [0.993622950,1.009943651] | 5 | 0.990964943 [0.986416565,1.007703339] | 9 |
| 8x2-w32 | 0.994702220 [0.974585731,1.008409936] | 11 | 0.992229394 [0.985019165,0.999851481] | 13 |
| 12x2-w32 | 0.997466539 [0.987836986,1.006395835] | 8 | 0.997782895 [0.996343338,1.004528102] | 10 |
| 16x2-w32 | 0.999867948 [0.991801895,1.008522737] | 8 | 0.998883272 [0.982872692,1.004350240] | 8 |
| 24x2-w32 | 0.995492920 [0.988369928,1.013864645] | 10 | 0.998537296 [0.990386081,1.006651978] | 8 |
| merge-24x1-w32 | 0.995449301 [0.990224797,1.005203777] | 10 | 0.990964369 [0.983440439,0.997637514] | 12 |
| merge-54x1-w32 | 1.001830197 [0.992424661,1.006448548] | 6 | 1.003295169 [0.990609172,1.015024211] | 7 |
| 1x2-w16 | 0.983535283 [0.958098812,1.013823994] | 10 | 0.960612441 [0.916989649,0.977394510] | 13 |

The primary at 32 workers gate requires both candidate upper ratios below 1 and at least10/15 wins. Physical 16 upper ratios must be at most 1.02, other cases at 32 workers shape ratios at most 1.03, against both baselines. Every calibration interval must lie within [1/1.03,1.03]. All candidate conditions pass. Calibration results follow.

| Case | OFF/accepted paired [bounds] | Check |
|---|---|---|
| 1x2-w32 | 1.010878829 [1.006063748,1.013758397] | pass |
| 2x2-w32 | 1.008729879 [0.999247314,1.019596400] | pass |
| 4x2-w32 | 1.006919238 [0.988469631,1.019921750] | pass |
| 8x2-w32 | 1.000485937 [0.985387897,1.012981542] | pass |
| 12x2-w32 | 0.993828684 [0.987244698,1.006223651] | pass |
| 16x2-w32 | 1.005211026 [0.992910706,1.016641040] | pass |
| 24x2-w32 | 0.998902667 [0.992455125,1.008825932] | pass |
| merge-24x1-w32 | 1.002798858 [0.998523135,1.015798311] | pass |
| merge-54x1-w32 | 0.995336735 [0.990954722,1.007448010] | pass |
| 1x2-w16 | 1.015797252 [1.004840026,1.061353625] | FAIL |

Bounds are sorted paired-ratio ranks 4–12 of 15, nominal 96.484375% coverage per comparison under independent exchangeable blocks, without a familywise guarantee. There is no gain floor, exclusion, sample extension, changed threshold or successful-capture repeat. The full failed gate stays failed; neither the positive pilot nor passing candidate checks override it. Primary at 32 workers paired savings are66.286 µs against accepted and162.266 µs against OFF. Physical 16 ON/accepted has a positive 1.646% estimate but bounds cross parity; it establishes no verified latency gain. Its nonregression condition passes, while its baseline calibration does not. Small positive/negative estimates in the larger cases remain in the tables without being rounded to zero or attributed to the balanced mechanism, whose existing eligibility is narrow.

## Identical baselines and calibration audit

Accepted and OFF are byte-identical SHA25681033d7786e14821cb76494de7faa2ebcc9e40c3222708372b21861fdeb56c20, at the same original executable paths used in the pilot. They are separate process streams, not different implementations. Primary at 32 workers OFF is1.088% slower paired than accepted; the candidate's smaller0.813% accepted comparison is the relevant headline. Physical 16 OFF/accepted has upper 1.061354, exceeding 1.03.

An offline audit retains all 15 blocks and all 30 physical 16 baseline processes. The standard driver's getrusage SELF deltas cover the measured loop after primes; sample printing follows the loop. Slow baseline processes have substantially more involuntary switches and lower average CPU concurrency, without greater measured CPU work. Three examples below are descriptive; no row is excluded or used to modify an estimator. The complete table is in the audit artifact.

| Block | Stream | Median ms | CPU ms/request | Involuntary switches/process | Average active CPU cores |
|---|---|---:|---:|---:|---:|
| 1 | accepted | 9.674339 | 122.292 | 24264 | 12.547 |
| 1 | OFF | 11.258045 | 118.807 | 280632 | 9.677 |
| 9 | accepted | 11.712373 | 118.243 | 303293 | 9.606 |
| 9 | OFF | 9.903584 | 123.013 | 51271 | 12.236 |
| 13 | accepted | 9.765262 | 122.567 | 26711 | 12.489 |
| 13 | OFF | 12.402194 | 119.176 | 337749 | 9.387 |

Across all30 baseline processes, involuntary-switches/request versus mean loop wall/request has exploratory Pearson correlation 0.993544. Average active cores is aggregate CPU/loop wall, not a runqueue-wait measurement. This association is consistent with scheduling effects, but does not identify external interference, internal yielding, CPU affinity, frequency or another cause. No scheduler trace or new capture is claimed. [Audit source/data](artifacts/calibration-audit.json). Further compatibility/resource investigation may proceed, but this report grants no adoption or fully calibrated physical 16 performance qualification.

## Memory and diagnostics

Process high-water RSS includes loading and primes. Each cell gives median/maximum MiB across processes. These are not minimum-RAM or pressure results.

| Case | Accepted MiB | OFF MiB | ON MiB |
|---|---:|---:|---:|
| 1x2-w32 | 130.75/133.50 | 131.25/134.75 | 130.75/135.50 |
| 2x2-w32 | 263.75/272.75 | 259.00/271.75 | 260.00/269.50 |
| 4x2-w32 | 298.75/308.00 | 303.75/313.00 | 297.25/314.50 |
| 8x2-w32 | 488.50/508.25 | 489.00/529.00 | 491.50/508.00 |
| 12x2-w32 | 687.75/717.00 | 698.00/720.50 | 694.25/720.50 |
| 16x2-w32 | 806.50/822.75 | 792.50/821.00 | 800.50/838.00 |
| 24x2-w32 | 1246.72/1313.07 | 1226.47/1289.00 | 1216.50/1311.36 |
| merge-24x1-w32 | 1126.71/1165.84 | 1122.64/1154.32 | 1144.18/1173.88 |
| merge-54x1-w32 | 1762.32/1900.80 | 1803.89/1872.69 | 1790.89/1895.94 |
| 1x2-w16 | 86.50/90.25 | 85.75/89.25 | 87.00/89.25 |

No general RSS reduction is established: primary at 32 workers candidate maximum exceeds accepted, and54candidate median rises despite a slightly lower maximum. The unchanged maximum bucket/leaf forecast and absence of a new heap buffer do not imply identical process peaks.

Six FIFO-gated four-event stat runs and six separate 199 Hz cycles:u profiles follow all latency, covering primary16/32 and all variants. Counters use31 requests, profiles 251, after three primes. Loading/primes are outside the FIFO gate. Diagnostic timings do not enter latency decisions.

| Stat case/variant | cycles/request, million | instructions/request, million | Generic cache misses/request, million | Branch misses/request, million |
|---|---:|---:|---:|---:|
| arith-1x2-w16-OFF | 569.968 | 1407.559 | 2.012 | 1.118 |
| arith-1x2-w16-ON | 562.682 | 1376.911 | 2.022 | 1.071 |
| arith-1x2-w16-accepted | 561.041 | 1406.883 | 2.034 | 1.119 |
| arith-1x2-w32-OFF | 858.062 | 1452.918 | 3.008 | 1.235 |
| arith-1x2-w32-ON | 827.670 | 1420.056 | 2.973 | 1.197 |
| arith-1x2-w32-accepted | 852.570 | 1453.934 | 3.156 | 1.256 |

All counter events run 100%. The six profiles contain 43527 samples, one PID each and zero lost records; physical 16 profiles contain 17 sampled TIDs, logical32profiles 33. Primary at 32 workers candidate retires fewer instructions and cycles than accepted in the separate stat captures. Physical 16 instructions also fall, while cycles are slightly higher than accepted; all observations remain. Profiles and generic cache counters establish neither exclusive wall-time costs nor a particular cache/DRAM/scheduler cause.

## Fresh CPU and Metal comparisons

M5 figures are supplied historical references, not new M5 captures. Ratios use the fresh 32-worker medians above.

| Shape | Accepted ms | ON ms | M5 CPU ms | CPU/accepted | CPU/ON | M5 Metal ms | Metal/accepted | Metal/ON |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 8.105814 | 8.039940 | 37 | 4.565× | 4.602× | 24 | 2.961× | 2.985× |
| 2x2 | 11.291047 | 11.313311 | 59 | 5.225× | 5.215× | 30 | 2.657× | 2.652× |
| 4x2 | 16.020905 | 16.021489 | 109 | 6.804× | 6.803× | 48 | 2.996× | 2.996× |
| 8x2 | 24.082256 | 24.042427 | 189 | 7.848× | 7.861× | 74 | 3.073× | 3.078× |
| 12x2 | 33.840266 | 33.734127 | 284 | 8.392× | 8.419× | 101 | 2.985× | 2.994× |
| 16x2 | 40.034518 | 40.096854 | 335 | 8.368× | 8.355× | 124 | 3.097× | 3.093× |
| 24x2 | 66.310989 | 66.143755 | 528 | 7.962× | 7.983× | 176 | 2.654× | 2.661× |
| merge-24x1 | 65.345371 | 65.132194 | 507 | 7.759× | 7.784× | 175 | 2.678× | 2.687× |

No M5 reference was supplied for 54→1. It measures 161.575926 ms ON versus 161.306709 accepted and 161.116998 OFF; paired ratios are 1.001830/1.003295, with bounds crossing parity. The all-eight CPU 10× objective remains **active and unmet, 0/8 for both accepted and candidate**. Primary CPU target is 3.7 ms. Fresh accepted measurements do not replace the previous production source 174 qualification or its recorded engineering exception. Production stays unchanged; candidate is not adopted.

## Verification and provenance

All 1002 workflow steps complete. All **27,948 SDK requests** verify internally. All **492 fresh 164-byte exports** independently pass Gnark, have exact public bindings and are disjoint from all prior/new identities. All 984 proof/public members are archived; ledger 15,919→16,411. Every closing source/library/helper/executable/image/fixture/cache pin matches. The exact 175-file snapshot matches the compiled pilot source; 173 originals remain exact and reversing the parent change recovers source 174. Copied native receipts are marked reused, with zero new native tests/builds in this stage. Their original test-only syntax repair remains documented in the pilot.

Complete source including non-Rust files, inherited receipts, commands, raw observations, exported proofs, decoded profiles and the offline calibration audit are retained. Six raw perf recordings remain local with checked SHA256 receipts; ownership changes only for those owned recordings after captures. No owned build/helper/hash/archive/second benchmark overlaps capture. No threshold, sample count, executable, source or flags change during the run. No capture is rerun. There is no new AWS/Aeglos, mobile/ARM/TEE, giant-circuit, portable, concurrent-proof, swap or forced-pressure result. Those compatibility/resource/production checks remain outstanding.

[Protocol](artifacts/design.md), [all observations/paired checks](artifacts/analysis.json), [identities](artifacts/identities.json), [native reuse](artifacts/native-reuse.json), [proof inventory](artifacts/proof-inventory.json), [closing pins](artifacts/pins.json), [capture completion](artifacts/runs/result.json), [source audit](artifacts/source-audit.json).
