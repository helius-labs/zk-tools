# Causal-profiler qualification on a Rust parallel workload

**This configuration fails the declared qualification; no prover profiling or optimization is selected.** Removing 25% of the critical worker's arithmetic improves joined latency by 24.866%, while the fixed Coz fit predicts 3.391%. The discrepancy is 21.475 percentage points. Removing 25% of the noncritical worker's work has effectively no measured effect. Production and accepted proving times remain unchanged.

This investigation tests whether causal profiling can guide the next prover change. It follows the [complete-caller arithmetic rejection](../2026-11-07-mixed-pair52-callers176/README.md), where fewer multiply instructions coincided with slower execution. Coz uses inserted delays and progress counters to estimate optimization potential ([paper](https://arxiv.org/pdf/1608.03676), sections 2–3; [upstream source](https://github.com/plasma-umass/coz/tree/dfc22c6c6f54cfbf085199440089cb29552f67d6)). Virtual results are predictions, not actual speedups. This result concerns this pinned tool, workload and host; it does not establish a cause for the mismatch or invalidate causal profiling generally.

Two persistent Rust workers on physical CPUs 2 and 3 execute changing affine recurrences, with 400,000 and 200,000 steps per batch. A coordinator on CPU 4 joins them at barriers. Actual controls remove exactly 25% of either worker's steps. Each final state is independently checked by logarithmic affine composition; worker identity, step counts, 128 warm-up batches and the separate counter-init unit are checked. No prover, key, retained plan or production pool is changed.

Fifteen fixed rotated/reversed blocks compare the three modes for one second per process. All 45 observations finish before one 180-second randomized Coz run. The [protocol](artifacts/capture-protocol.md), both initial analyzers, tool/input identities and actual code mappings were frozen before clocks. No owned build, helper, hash, analysis, report or push overlaps recording. There is no pilot, extension, selected rerun or threshold amendment.

|25% work reduction|Actual latency ratio [marginal interval]|Actual latency reduction|Predicted reduction|
|---|---:|---:|---:|
|critical|0.751340882 [0.750777988, 0.752073069]|24.866%|3.391%|
|slack|0.999833667 [0.999390018, 1.000231375]|0.017%|-2.653%|

Intervals are sorted zero-based ranks 3/7/11 of 15, without simultaneous family-wise coverage. The critical upper ratio must be <0.90; the noncritical interval must lie within [0.97, 1.03]. Both checks pass. Raw nanoseconds per joined batch, all paired ratios and changing-state outputs remain in the evidence; these are not proof times.

|Worker line|Zero-speedup experiments|Nonzero experiments|Experiments in fixed fit region|Baseline ns/batch|Predicted 25% effect|
|---|---:|---:|---:|---:|---:|
|critical|81|191|68|319469.976|3.391%|
|slack|22|53|25|306351.535|-2.653%|

All 348 emitted experiments and all speedup bins are retained. The weighted fit uses the predeclared region 0<x≤0.40, before the half-sized slack workload can become the critical path. Zero-speedup period is total effective duration divided by total progress visits. Fit relative period reduction against virtual speedup through the origin, weighted by visits; the 25% prediction is one quarter of that slope. Higher-speedup rows are reported but do not change the fixed fit. No confidence interval or precise predictive uncertainty is claimed for these fits.

The direction/scale rule requires a critical prediction ≥10%, absolute noncritical prediction ≤10%, a critical/noncritical separation ≥10 percentage points, and critical agreement with actual reduction within 10 percentage points. The critical direction, separation and agreement checks fail. Both lines have enough baseline, nonzero and fitting-region observations.

Two additional failures remain visible. One experiment selects a line with zero selected samples. The original analyzer also expected one runtime frame, but the stream contains 11 periodic cumulative frames. That was a parser assumption error: upstream log_samples writes snapshots periodically and at shutdown. Separate offline readers parse the frames and retain the original failed frame-count check. They use final cumulative sample values rather than summing snapshots. No recording is repeated. The 21.475-point prediction discrepancy independently rejects this configuration regardless of those two failures.

All five retained failures:

- `causal-runtime-record-count`
- `causal-zero-selected-samples`
- `critical-predicted-direction`
- `critical-minus-slack`
- `critical-predicted-vs-actual`

The independent reader reconstructs all 47 command receipts, 46 final workload states, paired controls, 348 raw experiments, every bin, baselines, weighted fits and decisions. An initial reader repair applies xtask's managed-variable whitelist when comparing environment receipts: LD_PRELOAD is passed but omitted from that receipt whitelist; the profiled environment is explicit in checked argv. Original failed readers and stderr remain available. No acceptance threshold changes.

Tool provenance is pinned: Coz `dfc22c6c6f54cfbf085199440089cb29552f67d6`, LIEF `038b60671f12dbd86bf84d9f8a38395bd2a8176e`, libelfin `5a032d5896f98aa57200e6e58e93f1dac0527a90`, Rust API 0.1.3 and offline-locked libc 0.2.191. Coz's path-filter and DWARF-scope tests pass. The Rust toy passes two meaningful arithmetic/control tests. Its original identical worker functions merged under LLVM; before any clock, distinct multipliers with the same instruction pattern and new oracle checks restore separate 76-byte functions. Actual instruction/inline-line mappings are retained. Earlier passed tests on the merged version are not additional distinct coverage.

The initial configure lacked zlib development files; a private extracted package supplies headers. A non-PIC static-link failure was repaired by linking the existing shared runtime, reusing compiled objects. Failed stages and sources are preserved. No system package installation, global kernel setting, Python runner, LLM viewer or source upload occurs. root access is confined to sampling and the exact owned profile path. This external C++ profiler is a diagnostic tool, not a C/ASM prover rewrite.

The accepted full-proof comparison below is inherited from the [latest full SDK trial](../2026-11-07-fused-epoch-sdk176/README.md), not measured by this toy. Supplied M5 references use different hardware. The same accepted SDK previously measured 7.915095/7.928660 ms locally for 1×2; its latest matched observation is 8.018344 ms. The inter-run cause remains unconfirmed; no slower replacement was adopted.

|Shape|Accepted local ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU 10× limit ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.018344|37|24|4.614×|2.993×|3.7|
|2x2|11.241463|59|30|5.248×|2.669×|5.9|
|4x2|15.933816|109|48|6.841×|3.012×|10.9|
|8x2|24.234311|189|74|7.799×|3.054×|18.9|
|12x2|33.855565|284|101|8.389×|2.983×|28.4|
|16x2|40.071563|335|124|8.360×|3.094×|33.5|
|24x2|65.917462|528|176|8.010×|2.670×|52.8|
|merge-24x1|65.305023|507|175|7.764×|2.680×|50.7|

Accepted merge54 remains 162.240109 ms without a supplied M5 reference. The verified CPU 10× goal remains active and unmet on all eight shapes. This tool check adds zero SDK requests or proofs; canonical 34,929 remains unchanged. No new prover peak/minimum RAM, AWS/Aeglos, mobile/ARM, huge-circuit, pressure/swap or concurrent-proof result is claimed.

This fixed configuration is closed without a debug/progress SDK run. No claim about an achievable prover speedup follows. Next work returns to a distinct reduction of complete prover work or dependencies, verified by actual complete callers and proofs.

[Results](artifacts/results.json), [independent review](artifacts/review.json), [protocol](artifacts/capture-protocol.md), [decision](artifacts/decision.md), [raw Coz stream](artifacts/profile.coz), [source/tool identities](artifacts/identities.json), and the [complete UTF8 evidence archive](artifacts/evidence.json.gz) preserve the experiment. The archive includes authored helpers, frozen and repaired readers, plans, pins, command streams, toy source and metadata. External source checkouts, package/archive binaries and compiled artifacts stay local with identities; upstream URLs identify their sources. No key, witness, proof payload or private ledger is published.
