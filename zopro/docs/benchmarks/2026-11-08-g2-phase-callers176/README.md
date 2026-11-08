# Whole-loop G2 phase boundaries: complete-caller result

**Rejected before full-proof timing.** Splitting the G2 prefix and reverse loops into separate helpers is **5.27% slower than fresh OFF** in the four-size 32-worker aggregate, with 0/15 winning blocks. The all-24 aggregate is 1.93% slower than OFF, also with 0/15 wins. All 1,188 complete callers pass output and allocation checks, and the independent results reader passes. Production 176 and accepted SDK 6d83 remain unchanged.

The fixed gate fails **34 of 76 timing checks**: five calibrations, 25 nonregression checks and all four aggregate gain checks. Some 32-worker controls have wide intervals and fail calibration; those observations remain in the results. Separately calibrated solo and 16-worker G2 regressions support closing the candidate. No extra blocks, exclusions, threshold changes, helper attributes, compiler choices, favorable subsets or unchanged rerolls follow.

## Change and qualification

The [native-qualified candidate](../2026-11-08-g2-phase-boundary176/README.md) places the complete forward prefix loop and complete reverse update loop in two never-inline functions, retaining the scalar-tail/inversion bridge in the dispatcher. Original arithmetic, prefix recurrence, signs, destinations, records and scheduling remain unchanged. There is no new production heap buffer, point/key copy, table, retained plan or pool. Call ABI and SIMD-state transfer can add stack traffic or overhead; unchanged heap layouts do not imply unchanged stack use.

The exact qualified 177-file snapshot gains only a native diagnostic adapter and two append-only exports, giving 178 files. G1 and driver bytes match the original reference. Removing the prior experimental branch and renaming the unused selector recovers the original G2 fixture. Both arms call the actual production entry, with only ON compiled using `zopro_g2_phase_boundary`. The previous descriptor, tree-policy and affine-offset candidates are absent. Original native field SHA `1200e63547baa41c299a48e3221f1cef85990b0369f17452dc12eb17045f9713`, matching core/Rayon dependencies and native optimized fat-LTO/one-CGU links are bound by four new successful build/link receipts.

Native correctness is inherited from the exact parent: 127 unfiltered tests per arm, generator/original comparisons, actual small-stack nested workers and the generic selector guard. Those successful populations are not replayed for diagnostic-only exports. The frozen reference `f07229ce…` is an immutable source 176 caller, not the accepted proof SDK. Its original pins, all 176 accepted originals and matching fixture/driver identities are checked before capture.

Two fresh symbol listings and nine function decodes establish the intended linked boundaries. ON's actual G2 loop transfers to both helpers. G1 run/stream byte sizes and mnemonic counts match across fresh arms. G2 retains 1,450 IFMA instruction sites in the selected bodies.

|Selected G2 code|OFF bytes|ON bytes|
|---|---:|---:|
|Original monolith|51,830|—|
|Caller/bridge|859|23,928|
|Prefix helper|—|5,077|
|Reverse helper|—|24,154|
|Selected total|52,689|53,159|

Each new phase is smaller than the original monolith, but total selected footprint grows by 470 bytes. Static instruction sites rise 8,762→8,796 and RSP-operand sites fall 1,509→1,471. These are neither executed instruction/spill counts nor stack bounds. Smaller individual functions did not produce a faster complete caller.

## Fixed complete-caller matrix

Ryzen 9 9950X, shared host, CPU 2 for solo and CPUs 0–15/0–31 for 16/32 workers, requesting 512 KiB worker stacks. G1/G2 × 200/201/352/353 points × 1/16/32 workers gives 24 contexts. Three checked 128-update primes precede a reset, then 8,192 changing batches per worker. Reference-array assembly, dispatch, arithmetic and waiting are timed; setup, reset and full generator checks are outside timing. Every final bucket is checked against independent generator coefficients. The fixture preserves changing updates and signs, but does not reproduce whole-proof key locality or task scheduling.

The prospective plan and reader are frozen before any preflight. Seventy-two preflights and their checker precede 15 rotated/reversed blocks (1,080 latency processes), then 18 PMU panels and 18 profiles, then 90 ownership/decode/event/metadata/loss commands. All 1,279 commands and 1,188 checked caller processes succeed on their first capture. This executes 199,292,928 worker batches including primes, **not proofs**. No owned helper, build, hash, analyzer, decoder, verifier, report, archive or push overlaps any recording; only the confirmed live handle is polled.

All 24 contexts require OFF/frozen calibration within reciprocal 1.03–1.03 and ON/control nonregression upper ≤ 1.02 against both controls. The G2 four-size 32-worker and all 24 equal-log aggregates each require upper < 1 and at least 10/15 wins against both controls. G1 is an unchanged negative control, without a separate gain condition. Fixed paired-ratio ranks are 3/7/11 of 15 (zero-based); marginal intervals have no familywise guarantee. There is no minimum positive gain floor.

|Aggregate|ON/OFF ratio [rank interval]|Wins/15|ON/frozen ratio [rank interval]|Wins/15|
|---|---:|---:|---:|---:|
|G2|1.052664178 [1.044407782, 1.060912661]|0|1.051007385 [1.038427037, 1.054940613]|0|
|ALL|1.019299951 [1.007537795, 1.031268569]|0|1.010609684 [1.007103379, 1.029919837]|2|

Displayed times below are medians of process means in microseconds. Paired ratios use matched blocks and need not equal ratios of the displayed medians. Every G2 ON/OFF case loses all 15 blocks; its paired median slowdowns range 1.26–6.06%.

|Context|Frozen / OFF / ON µs|ON/OFF paired ratio [rank interval]|Wins/15|
|---|---:|---:|---:|
|G1-200-w1|4.232219 / 4.231225 / 4.234702|1.000660200 [0.998063991, 1.001827136]|5|
|G1-200-w16|4.809629 / 4.809151 / 4.835424|1.004355007 [0.997610314, 1.007473981]|6|
|G1-200-w32|8.214626 / 6.905694 / 6.926001|1.000362622 [0.839300677, 1.007267519]|7|
|G1-201-w1|4.354355 / 4.349043 / 4.352509|1.000552557 [0.998782701, 1.001517480]|6|
|G1-201-w16|4.934393 / 4.937240 / 4.943156|1.001152720 [0.998215286, 1.003660478]|7|
|G1-201-w32|8.134511 / 7.082605 / 8.323232|0.999658374 [0.992543516, 1.185128633]|8|
|G1-352-w1|6.961780 / 6.960096 / 6.962540|1.000136583 [0.999110530, 1.001194504]|6|
|G1-352-w16|7.951086 / 7.961796 / 7.961925|1.000276037 [0.996497441, 1.004588257]|6|
|G1-352-w32|11.521345 / 11.608425 / 11.898890|1.008765957 [0.995187054, 1.254137131]|4|
|G1-353-w1|7.067764 / 7.074605 / 7.071335|1.000332514 [0.998988120, 1.001011257]|6|
|G1-353-w16|8.085568 / 8.087260 / 8.093047|0.999953212 [0.997108371, 1.002833989]|8|
|G1-353-w32|11.708009 / 11.825896 / 11.779833|0.999909955 [0.803482114, 1.243989669]|8|
|G2-200-w1|7.694261 / 7.697691 / 7.937600|1.031939296 [1.030485146, 1.032549695]|0|
|G2-200-w16|8.937432 / 8.942245 / 9.167201|1.025128987 [1.023405514, 1.026226960]|0|
|G2-200-w32|14.450858 / 14.369652 / 15.223711|1.053549768 [1.044350456, 1.071630628]|0|
|G2-201-w1|8.160478 / 8.043591 / 8.175387|1.016437642 [1.015387379, 1.018187561]|0|
|G2-201-w16|9.472499 / 9.345490 / 9.461092|1.012610552 [1.010779968, 1.014508432]|0|
|G2-201-w32|15.007305 / 14.866203 / 15.504602|1.040590189 [1.024898949, 1.054342934]|0|
|G2-352-w1|13.030359 / 13.025133 / 13.403510|1.029313748 [1.028034771, 1.029946908]|0|
|G2-352-w16|15.212103 / 15.206317 / 15.823845|1.039451433 [1.035557193, 1.041851817]|0|
|G2-352-w32|25.017714 / 24.975499 / 26.363267|1.057865792 [1.046919335, 1.068991175]|0|
|G2-353-w1|13.450429 / 13.370962 / 13.669674|1.022076073 [1.021203217, 1.022341208]|0|
|G2-353-w16|15.713564 / 15.624637 / 16.167108|1.035417651 [1.032451494, 1.036983270]|0|
|G2-353-w32|25.153649 / 25.424647 / 26.839016|1.060626855 [1.052694352, 1.066029044]|0|

The five failed calibrations are G1-200/201/353-w32 and G2-200/201-w32. All raw blocks and their wide intervals remain. Apparent positive G1 estimates are retained in the table and data; unchanged G1 code and failed controls do not establish a G1 optimization. The calibrated G2-200 solo/16 cases slow 3.19%/2.51%; G2-352 solo/16 slow 2.93%/3.95%, with zero wins. The calibrated G2-352/353-w32 cases also lose. Rejection therefore does not rely only on uncalibrated 32-worker comparisons.

## CPU profiles and memory

All 18 profiles pass: 119,847 samples, zero reported LOST, 100% assigned samples, minimum target share 99.902%, maximum unknown 0.098%, and at least 3,016 samples per capture. All grouped PMU events run 100%. Counters and profiles are collected separately after latency, with acknowledged FIFO gating after primes and before output checks. Each context/arm has one panel and one profile, so counter uncertainty and causal stall attribution are not measured.

|Diagnostic context|Retired instructions ON/OFF|Cycles ON/OFF|
|---|---:|---:|
|G2-200-w1|0.993373|1.032753|
|G2-200-w32|0.993353|1.031183|
|G2-352-w32|0.992840|1.020011|

The G2 panels retire about 0.66–0.72% fewer instructions while taking 2.00–3.28% more cycles. Cache-miss changes differ by context. These observations do not isolate calling convention, dependency, register allocation, cache or DRAM as the cause. CPU-sample shares are not removable proof-wall fractions.

All timed allocation counts are zero and all primed scratch capacities match across arms. G2 full-batch prefix payload is 16,000 bytes at 200 points and 28,160 bytes at 352; one-point tails retain 1,280 additional bytes. Maximum candidate diagnostic process RSS is 19 MiB, including setup and worker stacks but excluding proving keys. This is not whole-prover peak, minimum RAM, pressure, phone or giant-circuit evidence.

Both post-capture analysis commands pass on first execution. The independent reader reconstructs all commands, outputs, ratios, 76 timing checks, profile identities/ownership/CPU/period/loss and counter qualifications. Native and exact copied-loop qualification is inherited, not a second independent field implementation. Shared parser/statistical assumptions and sampling limitations remain.

Before clocks, symbol-target preparation needed a Rust impl-name selector correction; it had stopped before any decode plan. The existing two NM results were reused and all nine decodes ran once. Campaign-source adaptation also needed a formatting-pattern guard before either generated source was written. Initial helper sources/executables and tool-observed failures are retained; those attempts have no separate failed workflow receipts. Runtime code, inputs, populations and rules were unchanged. Helper compilation/plan-reader invocations do not all have workflow receipts. No successful native test, oracle, decoder or capture was replayed.

## Unchanged full-proof CPU and Metal comparison

This caller screen creates zero SDK requests or independent Gnark exports. The canonical proof inventory remains 37,023. Full-proof times below are inherited from the [latest standard-SDK campaign](../2026-11-08-tree-policy-sdk176/README.md); supplied M5 CPU and Metal values are historical measurements on different hardware. No new AWS/Aeglos or full-proof performance is claimed.

|Shape|Accepted local ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× limit ms|
|---|---:|---:|---:|---:|
|1x2|8.040649|37 / 4.602×|24 / 2.985×|3.7|
|2x2|11.149097|59 / 5.292×|30 / 2.691×|5.9|
|4x2|16.069415|109 / 6.783×|48 / 2.987×|10.9|
|8x2|24.131178|189 / 7.832×|74 / 3.067×|18.9|
|12x2|33.940628|284 / 8.368×|101 / 2.976×|28.4|
|16x2|40.164083|335 / 8.341×|124 / 3.087×|33.5|
|24x2|66.187704|528 / 7.977×|176 / 2.659×|52.8|
|merge-24x1|65.361571|507 / 7.757×|175 / 2.677×|50.7|

Merge54 remains 160.890372 ms without a supplied M5 reference. Historical same-accepted 7.915095 ms and private 7.859756/7.869400 ms remain valid sub-8 ms observations; later variation has no confirmed cause and does not establish an adopted regression or a new record.

This whole-phase candidate is closed without adoption or a standard-SDK follow-up. No layout/attribute/phase-boundary rescue is selected. A new experiment needs a distinct useful-work or data-dependency reduction supported by the main-MSM evidence and prior negatives. The full verified single-proof CPU AVX 10× objective remains active and unmet, 0/8.

[All context results](artifacts/analysis.json), [independent reader](artifacts/review.json), [actual linked boundaries](artifacts/codegen.json), [source audit](artifacts/source-audit.json), [prospective protocol](artifacts/protocol.md), [source/raw observations/complete receipts](artifacts/evidence.json.gz), [checksums](SHA256SUMS). Public analysis omits only duplicated full caller/command receipt maps; the exact original analysis and all receipts remain archived and hash-bound.
