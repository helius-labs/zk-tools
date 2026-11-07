# Native-feature SPR scheduling: full SDK comparison

The declared full-proof screen fails. **1×2 at 32 workers measures 8.008090 ms accepted, 8.005689 ms control and 8.833788 ms candidate.** Paired candidate/control is 1.103317 [1.099749, 1.107026], with 0/15 wins; candidate/accepted is 1.102573 [1.097505, 1.107363]. The candidate is about 10.33% slower than control. The original-eight aggregate is 1.023066 [1.020742, 1.024985] versus control. Production remains unchanged; this exact compiler tuning is closed.

## Change and measurement

This isolated Rust 1.98.1 experiment adds `RUSTC_BOOTSTRAP=1` and `-Z tune-cpu=sapphirerapids` while retaining `target-cpu=native`, the accepted cfgs, arithmetic and all 176 source files. The final SDK links eight coherent libraries with the same driver, optimization, native CPU, fat LTO and one codegen unit. There is no new lookup table, key copy, solver plan, production pool or source-level allocation policy. Compiler code layout and spills can change. The unstable tuning flag is experimental, not a production build requirement.

The [earlier scheduling audit](../2026-11-06-llvm-schedule-audit176/README.md) and [hardware calibration](../2026-11-06-instruction-calibration176/README.md) motivated one alternative model. All three probe IR functions are byte-identical after removing two tuning attributes; native CPU/features remain exact, and emitted instruction schedules change. The LLVM 20 SPR model reports one register IFMA/cycle, still different from the measured Ryzen throughput. Its coverage check is not Rust LLVM 22 performance validation or a calibrated Ryzen model. No target/model sweep or compiler defect is claimed. Eight final candidate SDK functions are decoded once; static counts are not dynamic cost or stack bounds.

Ryzen 9 9950X, 16 physical cores/32 logical CPUs, shared host. Fifteen fixed rotated/reversed blocks compare accepted SDK `6d83`, retained accepted-source control `64bfa` and candidate `4861687d`. All nine circuits use 32 workers; primary also uses 16 workers on CPUs 0–15. Each process primes three requests, then measures 151 primary or 31 other requests. Requests include fresh solving, randomized proving, internal verification and serialization. Key loading is excluded from latency and included in lifetime process RSS. The controls are reused binaries with new process observations; no unchanged rebuild is presented as a fresh compiler control.

All 450 latency processes precede 18 diagnostic captures; all captures precede 468 Gnark checks and 45 offline commands. No owned build, analysis, hash, archive or other workload overlaps capture. Results are medians of process means. Ratios use paired geometric means and 20,000 fixed-seed paired bootstrap resamples. These are marginal 95% intervals conditional on the resampling assumptions, without a familywise guarantee. Calibration must fit factor 1.03; all nonregression upper bounds must be ≤1.02; primary at 32 workers and equal-log all-eight gains require an upper bound below 1 against both controls. No minimum gain floor, exclusions, extension, reroll or favorable subset is used.

## Complete results

Smaller candidate/control ratios are faster. Every original context remains, including primary at 16 workers, whose control calibration fails.

|Context|Accepted ms|Control ms|Candidate ms|Candidate/control [interval]|Wins/15|
|---|---:|---:|---:|---|---:|
|1x2-w32|8.008090|8.005689|8.833788|1.103317 [1.099749, 1.107026]|0|
|2x2-w32|11.251552|11.170289|12.015639|1.072409 [1.067828, 1.076803]|0|
|4x2-w32|16.143542|16.078570|16.252814|1.008336 [1.002002, 1.014337]|4|
|8x2-w32|24.174653|24.279320|24.310828|0.998365 [0.989288, 1.008150]|7|
|12x2-w32|34.040730|33.968133|34.046583|1.001989 [0.993925, 1.010136]|5|
|16x2-w32|40.208904|40.118618|40.356034|1.003984 [0.996711, 1.010443]|5|
|24x2-w32|66.267049|66.336446|66.424034|0.999924 [0.992529, 1.007697]|7|
|merge-24x1-w32|65.343182|65.254085|65.371363|1.001651 [0.998124, 1.005258]|8|
|merge-54x1-w32|161.958416|161.904163|161.491113|0.995143 [0.990443, 0.999630]|9|
|1x2-w16|9.726916|9.897300|11.910677|1.192598 [1.126329, 1.260928]|1|

There are 11 failed latency checks. Primary at 32 workers and 2×2 regress against both controls; primary at 16 workers also fails nonregression and control calibration. Larger cases are mostly near parity. Merge54 improves against control, while its accepted comparison spans parity; this does not rescue the failed global configuration. Exact failures and all accepted/control intervals remain in the artifacts.

## Profiles and memory

All nine grouped PMU panels and nine cycle profiles pass their declared quality guards: at least 99.9% PMU running coverage, at least 1,000 samples, at most 5% unknown, zero LOST records and checked FIFO/CPU/process ownership. The saved profiles contain 92,285 samples. The [demangled attribution](artifacts/profile-attribution.json) preserves every profile; flat sample categories include inlined work and do not establish exclusive CPU costs, causal stalls or a removable wall-time fraction.

The primary 32-worker sample increase is concentrated in the signed borrowed G2 consumer. In the fixed 251-request profiles, its samples rise from **1,941 control → 3,306 candidate**, while borrowed G1 changes from **2,472 → 2,525**. Whole-loop counters show cycles per request rising from 809.49 million to 891.13 million, with instructions rising from 1.3868 billion to 1.4005 billion. These observations narrow the next code inspection to the G2 caller's emitted schedule; they do not identify a particular dependency, spill, cache level or compiler defect. No new recording or decode was needed for this attribution.

|Context|Accepted peak RSS MiB|Control peak RSS MiB|Candidate peak RSS MiB|Accepted / candidate minor faults per request|
|---|---:|---:|---:|---:|
|1x2-w32|135.25|133.75|135.25|129.510 / 128.695|
|1x2-w16|90.25|89.25|89.75|58.099 / 56.430|
|24x2-w32|1279.00|1329.29|1289.75|6899.419 / 5822.903|
|merge-54x1-w32|1871.26|1847.62|1852.17|21492.000 / 21280.258|

RSS includes loading, allocator state, stacks and all process memory. Fault values are per-request medians from the measurement loops. Neither is minimum RAM, a memory-pressure feasibility result or evidence that faults caused the timing difference.

## Correctness and review

The changed compiler configuration passes 162 distinct library tests: 35 field, 125 core and 2 prover tests. The initial concurrent command passes all 35 field and 121 core tests, then fails four allocator assertions involving process-wide policy/mapping state. Each of those four passes in isolation using the same compiled binary; the failed command remains archived. The successful 35/121 are not repeated, and the previously unrun two prover library tests run once. These counts exclude integration suites. Isolated success does not establish concurrent test isolation or identify a compiler defect. A pre-execution metadata draft incorrectly assumed eight prover library tests; inspecting actual receipts corrected that count before the qualification reviewer ran.

Twenty-two preflight exports and 468 campaign exports pass independent Gnark checks with fresh proof hashes and exact image/VK/public bindings. In total there are 28,006 SDK requests and 490 fresh exports; canonical ledger 33,949→34,439. Proof/public payloads are 164/44 bytes. No successful test, model, recording, Gnark verifier or perf decoder is replayed for bookkeeping.

The independent Rust reader checks source and binary pins, 1,035 successful workflow receipts, the retained failed test receipt, nine new SDK codegen receipts, all 176 exact production sources, 490 fresh proof bindings and 26,514 raw samples. It reconstructs all 32 paired contrasts and bootstrap intervals using independently generated multinomial weights, reconciles every failed gate, reparses the saved profile streams without a new decode, and derives all eight goal comparisons. This review is not a second cryptographic verifier.

## M5 comparisons

The M5 CPU and Metal values are supplied references on different hardware. New accepted-build observations below replace the previous campaign's displayed accepted observations; the accepted binary is unchanged. The candidate remains rejected.

|Shape|Accepted CPU ms|Candidate ms|M5 CPU ms|M5 Metal ms|CPU speedup accepted / candidate|Metal speedup accepted / candidate|10× CPU limit ms|
|---|---:|---:|---:|---:|---:|---:|---:|
|1x2|8.008090|8.833788|37|24|4.620× / 4.188×|2.997× / 2.717×|3.7|
|2x2|11.251552|12.015639|59|30|5.244× / 4.910×|2.666× / 2.497×|5.9|
|4x2|16.143542|16.252814|109|48|6.752× / 6.707×|2.973× / 2.953×|10.9|
|8x2|24.174653|24.310828|189|74|7.818× / 7.774×|3.061× / 3.044×|18.9|
|12x2|34.040730|34.046583|284|101|8.343× / 8.342×|2.967× / 2.967×|28.4|
|16x2|40.208904|40.356034|335|124|8.331× / 8.301×|3.084× / 3.073×|33.5|
|24x2|66.267049|66.424034|528|176|7.968× / 7.949×|2.656× / 2.650×|52.8|
|merge-24x1|65.343182|65.371363|507|175|7.759× / 7.756×|2.678× / 2.677×|50.7|

Accepted and candidate meet 0/8 and 0/8 CPU targets respectively; the original verified all-eight 10× goal remains active and unmet. Earlier sub-8 ms primary observations used the same accepted binary; inter-run variation has no established cause. No new AWS/Aeglos, mobile/ARM, 250/4999, pressure/swap, minimum-RAM or concurrent-proof measurement is claimed.

[Full latency results](artifacts/latency-analysis.json), [diagnostics](artifacts/diagnostic-analysis.json), [goal comparisons](artifacts/goal-comparison.json), [review](artifacts/review.json), [decision](artifacts/decision.json), [UTF-8 source/plans/pins/receipts](artifacts/evidence.tar.gz), [checksums](SHA256SUMS). The archive excludes executable/library files, keys, requests, witnesses, proof/public payloads, unmodified production source and binary perf recordings.
