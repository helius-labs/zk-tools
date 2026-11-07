# Uniform scaled prefixes: first complete-SDK pilot

The candidate's paired estimate is **0.90% faster than fresh OFF** and **1.49% faster than accepted**, with calibration passing. The interval against OFF narrowly crosses parity: **0.983682590–1.000011672**. The declared pilot gate fails, so there is **no adoption**. This is positive but insufficiently qualified latency evidence, not a measured regression or a zero improvement.

## Why this measurement followed the static rejection

The [native stage](../2026-11-07-uniform-prefix-native176/README.md) passed correctness and failed its whole-function instruction heuristic. The later [exhaustive control-flow audit](../2026-11-07-uniform-prefix-cfg176/README.md) found that every matched vector loop shrinks in both curves, while G1's total increase lies outside those loops. That new location evidence justified reconsidering the heuristic before this candidate's first clock. The original failed screen remains unchanged; it was not a proof of slower execution.

This study uses the **same qualified accepted/OFF/ON executables**. There is no native rebuild, curve-only subset, changed worker count, layout, compiler recipe or replay of successful field/core tests, models, symbol listings or disassemblies. The new complete-proof protocol was frozen before measurement. Candidate field/point encodings and proof-local storage remain as documented in the native report; no lookup table, key copy, solver plan, additional scratch buffer or pool is added.

The inherited qualification includes 500,256 independent Arkworks-checked SIMD outputs, 8,206 scalar-root cases, 127 OFF/127 ON core tests and eight ON prover tests. The new preflight covers all nine shapes and all three executables with **27 fresh independent Gnark verifications**.

## Complete proof latency

Ryzen 9 9950X, 32 logical workers with process affinity CPUs 0–31. Fifteen fixed rotated/reversed blocks retain every observation. Each process performs three primes and 151 measured complete SDK requests, including verification and serialization. All latency finishes before its independent verification and before counters/profiles. No other owned CPU workload overlaps recording; exclusive use of the host is not claimed.

|Arm|Median process mean, ms|Median process maximum RSS, MiB|Largest process maximum RSS, MiB|
|---|---:|---:|---:|
|Accepted|7.985302|130.25|135.00|
|Fresh OFF|7.934800|130.00|133.50|
|ON|7.859756|130.00|132.25|

|Paired comparison|Median ratio|Interval|Wins / 15|Check|
|---|---:|---:|---:|---|
|OFF / accepted|0.994576656|0.986383838–1.003571113|9|Calibration passes|
|ON / OFF|0.991000557|0.983682590–1.000011672|11|Gain fails|
|ON / accepted|0.985103027|0.980060431–0.989470318|12|Gain passes|

Ratios pair process means by block and need not equal ratios of displayed medians. Bounds use sorted ranks 4–12, nominal 96.484375% marginal median coverage conditional on independent, exchangeable blocks, without a familywise guarantee. Gain requires upper ratio below one and at least 10 wins against **both** controls. OFF/accepted must remain within factor 1.03. There is no percentage gain floor.

The upper ON/OFF bound exceeds parity by approximately **0.0012%**. That small margin does not make the failed check pass, and the positive point estimates are retained. No extra blocks, exclusions, threshold change or unchanged timing reroll follows. This exact pilot closes without adoption or an all-shape performance extension. It neither establishes a universal speedup nor rules out a small real benefit.

## Automatic diagnostics

After latency, each arm runs one 31-request user-counter panel and one 251-request user-cycle profile, each after three primes and with FIFO warm-region gates. Profile period is 16 million cycles with 512 mmap pages. All four counter events report 100% running time.

|Counter per complete SDK request|OFF|ON|ON / OFF|
|---|---:|---:|---:|
|Cycles|816,026,431|810,465,720|0.993186|
|Instructions|1,390,055,082|1,366,325,955|0.982929|
|Generic cache misses|2,708,288|2,938,260|1.084914|
|Branch misses|911,057|886,054|0.972557|

The single panels support a small whole-proof instruction reduction, with more generic cache misses. They do not have repeated-panel confidence intervals or identify a causal cache level, DRAM bottleneck, spill cost or removable wall-time fraction. They do not override the latency gate.

Accepted/OFF/ON profiles contain **12,416 / 12,299 / 12,111 samples**, with **1.72% / 1.71% / 1.86% unknown symbols** and zero recorded LOST events. Metadata, period, process/thread/CPU ownership and totals pass review. Saved native decodes are reused; no new exact IP/ELF attribution is claimed. Flat function samples include inlined work and are not exclusive wall time.

Process maximum RSS includes loading/primes and is not minimum RAM, incremental scratch cost, or qualification for phones, ARM, concurrent proofs, giant circuits or pressure/swap. No new fixed RAM cap or memory-pressure policy is introduced.

## Verification and provenance

All phases complete on first execution. Preflight adds 108 SDK requests including primes and 27 fresh Gnark exports; latency adds 6,930 requests/45 exports; diagnostics add 864 requests/six exports. **Total: 7,902 requests and 78 fresh independently verified exports.** Canonical unique-proof ledger advances **31,699 → 31,726 → 31,771 → 31,777**. Proof/public/image/VK bindings and freshness pass.

The outcome-neutral reader reconstructs paired checks, raw samples, counter/profile totals and all 51 post-preflight export bindings. It checks 406 input pins and 117 capture/verification/offline receipts. Its final workflow additionally binds its own executable/source and finished analyses, with 411 pins total. It is not a second cryptographic verifier, disassembler, LOST decoder or exact IP/ELF analyzer. No observation or successful qualification was repeated for bookkeeping.

Accepted source176 and SDK SHA-256 `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661` remain unchanged. OFF/ON SHA-256 values are `af7a3b6f6533848ef562996dbdf2c91f91558fc22b2c44a32ea7046faaf3c191` and `23b08c329c83fc6c83918a70b4033eb0839f3a70f7ddd1633c2e15ea3d13d9fd`. The candidate has no production adoption or new AWS/Aeglos comparison.

## Accepted CPU and Metal comparisons, unchanged

Inherited local 32-worker full-SDK observations and supplied M5 references follow. They are not an all-shape measurement of this candidate. All eight CPU 10× requirements remain unmet.

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.915095|37 / 4.675×|24 / 3.032×|3.7|
|2x2|11.169812|59 / 5.282×|30 / 2.686×|5.9|
|4x2|16.085882|109 / 6.776×|48 / 2.984×|10.9|
|8x2|24.517520|189 / 7.709×|74 / 3.018×|18.9|
|12x2|33.838626|284 / 8.393×|101 / 2.985×|28.4|
|16x2|40.083672|335 / 8.358×|124 / 3.094×|33.5|
|24x2|65.506198|528 / 8.060×|176 / 2.687×|52.8|
|merge-24x1|65.200982|507 / 7.776×|175 / 2.684×|50.7|

Merge54 remains 161.718956 ms without a supplied M5 reference. The goal remains active and unmet, 0/8. The loop audit and this small positive work/latency estimate remain useful evidence for a distinct future change; they do not supply the requested twofold primary jump.

[Artifacts](artifacts/) retain frozen sources/helpers, designs, plans, pins, all latency rows, counter/profile summaries, command receipts and independent reviews. Archives are compared with originals. Actual keys/images, requests, witnesses, proof/public-input payloads, executables/libraries and raw perf data remain private.
