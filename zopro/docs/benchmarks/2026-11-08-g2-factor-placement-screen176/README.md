# G2 real-factor placement: complete caller screen

**Rejected: the candidate is slower in all four measured complete-caller contexts.** Moving the shared real factor onto the denominator components reduces some static stack-operand sites but does not reduce field multiplication count. The saved screen and its reader agree on rejection. Production remains unchanged.

The [native qualification](../2026-11-08-g2-factor-placement176/README.md) records the exact algebra, source recovery, 128 passing core tests and generic compilation. This report completes its prospectively fixed first performance screen; it does not repeat native tests or rebuild either executable.

|Complete caller|Frozen median µs/wave|Fresh OFF median µs/wave|Candidate median µs/wave|Paired ON/OFF median change|Paired ON/OFF rank interval|ON wins / 7|
|---|---:|---:|---:|---:|---:|---:|
|G2-200-w1|7.636|7.633|7.885|+3.31%|[1.03192, 1.03577]|0|
|G2-200-w32|14.343|14.341|15.023|+4.47%|[1.03259, 1.05426]|1|
|G2-352-w1|12.945|12.937|13.384|+3.59%|[1.03512, 1.03640]|0|
|G2-352-w32|24.703|24.680|25.280|+1.64%|[1.01511, 1.02855]|0|

Each wave applies one changing signed batch per worker. These are component caller times, not proof times. Seven paired process ratios use the frozen zero-based order ranks 1/3/5; these descriptive small-screen intervals are not 95% or familywise confidence intervals. All four OFF/frozen calibration intervals pass the declared factor-1.05 range. The candidate has clear regressions in both single-worker contexts and the 200-pair 32-worker context, triggering the declared early rejection. All observations remain included; there is no extension, subset rescue or altered threshold.

The 139-command Rust/xtask workflow completed on its first capture: 12 checked preflights, 84 latency processes, 12 PMU/profile processes, one preflight review and 30 offline commands. The 108 caller processes execute 22,174,080 worker batches including three 128-round primes per process. Complete generator outputs and scratch capacities match across arms; all timed allocation counts are zero. Solo runs use CPU 2; 32-worker runs pin individual workers to CPUs 0–31 with 512 KiB configured stacks. No owned build, hash, analyzer, report or push ran during recordings.

The OFF and ON selectors share exact executable SHA-256 `4e8fa03288fcde9823f0027f00d811c36fe29a21ac66e31b876f1c0ca04a2831`. The original-only frozen executable is `f07229ce665cf61c972037a27752bb53cce8c5dd58be0c71d8776434c961064b`. Exact paths, compiler arguments and pre-recording pins are retained in the evidence.

All six profiles pass quality checks: zero LOST records, 100% of samples assigned to the actual process and declared threads, and under 0.052% unknown symbols. All four PMU counters report 100% running time. Diagnostic processes are separate observations and do not replace the latency blocks.

|32-worker caller|ON/OFF user-cycle change|ON/OFF retired-instruction change|
|---|---:|---:|
|200 pairs|+1.73%|+1.14%|
|352 pairs|+2.21%|+1.19%|

These counters support the measured regression despite the lower static stack-operand count. They do not isolate a unique microarchitectural cause. Static site counts alone were insufficient to predict the result. The single diagnostic run per arm is descriptive rather than a repeated counter-confidence study.

Maximum observed process RSS across latency runs is 3 MiB for the single-worker contexts and 15.5 MiB for 32 workers. These fixture peaks exclude proving keys and are neither a whole-prover memory requirement nor a minimum-RAM result. Scratch capacities are unchanged and no lookup table or retained key plan is introduced.

The final reader reconstructs all 108 outputs, process means, paired ranks, wins, RSS and selection checks from saved evidence. Detailed PMU/profile parsing is shared with the analyzer, rather than supplied by a second independent decoder. Existing scalar/curve correctness references are also shared. No successful recording, decoder, native test or prior cryptographic export is replayed.

The fixed factor-placement recipe is closed and not selected for SDK integration. A different experiment must reduce useful work or demonstrate a different cost advantage. The canonical proof inventory remains 36,477; this screen adds zero SDK requests and zero independent Gnark exports.

## Full-proof context, inherited

Sub-8 ms full-proof medians were already recorded: approximately 7.86–7.87 ms for earlier private candidates and 7.92 ms for the accepted build in an earlier campaign. The latest completed matched campaign measured 8.018438 ms for that same accepted build and 7.897108 ms for its unadopted default-carry candidate. These cross-campaign absolute values do not establish a regression or a new record. The default-carry candidate remains unadopted because its full selection gate failed three 16-worker checks.

|Shape|Accepted local CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|8.018438|37 / 4.614×|24 / 2.993×|3.7|
|2x2|11.199751|59 / 5.268×|30 / 2.679×|5.9|
|4x2|15.957517|109 / 6.831×|48 / 3.008×|10.9|
|8x2|24.250900|189 / 7.794×|74 / 3.051×|18.9|
|12x2|33.735233|284 / 8.418×|101 / 2.994×|28.4|
|16x2|40.165709|335 / 8.340×|124 / 3.087×|33.5|
|24x2|65.802777|528 / 8.024×|176 / 2.675×|52.8|
|merge-24x1|64.926690|507 / 7.809×|175 / 2.695×|50.7|

This table is inherited from the [latest full-proof campaign](../2026-11-08-uniform-carry-default-performance183/README.md), not a new measurement of this component candidate. Supplied M5 CPU/Metal references use different hardware. Accepted merge54 is 162.442299 ms, without a supplied M5 reference. The verified CPU AVX 10× goal remains active and unmet, 0/8 shapes. No new AWS/Aeglos, mobile, huge-circuit, pressure, swap, minimum-RAM or concurrent-proof-throughput result is added.

[Results](artifacts/capture-analysis.json), [diagnostics](artifacts/diagnostic-analysis.json), [reader](artifacts/capture-review.json), [frozen protocol](artifacts/capture-protocol.json), [evidence](artifacts/evidence.json.gz), [checksums](SHA256SUMS).
