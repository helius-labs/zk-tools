# Owned bucket roots: complete SDK results

**No confirmed full-proof improvement; the candidate is not adopted.** Primary 1×2 at 32 workers is **7.832193 ms**, versus fresh OFF **7.842525 ms** and accepted REF **7.821799 ms**. Paired ON/OFF is **0.997068 [0.989476,1.003009]**,18/31 wins; ON/REF **0.999470 [0.995821,1.003602]**,16/31 wins. Both intervals cross parity. Positive point estimates remain recorded; there is no minimum percentage-gain floor.

**73/79 declared checks pass.** All 11 control calibrations,22 nonregression bounds,22 RSS guards and 18 diagnostic-quality checks pass. The six failures are primary gain/win requirements against both controls and both ALL8 gain checks. ALL8 equal-log ON/OFF is 0.999407 [0.997016,1.001723]; ON/REF 1.000426 [0.998425,1.002417]. Neither aggregate establishes a gain or slowdown. The exact recipe closes without exclusions, extra samples, relaxed thresholds, subset promotion or unchanged recapture.

The [qualified owned-entry implementation](../2026-11-10-weighted-owned-roots 176/README.md) consumes temporary bucket vectors across nine terminal callers, writes nine roots into each 256-bucket group's existing prefix, then folds up to eight groups after a parallel barrier. Original roots/kernel arithmetic, both weighted/unweighted partials and offset corrections are preserved. Guards run before mutation; short/nonfull/G2/disabled cases retain the original borrowed reducer. It adds no separate heap root array, per-call memory-admission operation, key table, retained plan, pool or RAM cap. The existing parent storage and 6176 logical fixed G1-leaf bytes remain forecast; this is not an emitted stack/RSS bound. Native 131-test/13-build-link qualification is inherited without replay.

Ryzen 9950X,16 physical/32 logical CPUs, shared host. ALL8 plus merge 54 at 32 workers, primary also 1/16, worker i pinned to CPUi, soloCPU2, SDK default worker stacks. Timers include solve, randomized proving, internal verification and serialization. Loading, pool startup, three primes and final export are excluded. Original field 1200, accepted cfgs including dense busy reset and exact qualified SDKs are reused.

| Context | REF ms | OFF ms | ON ms | Paired ON/OFF [95% bounds], wins |
|---|---:|---:|---:|---|
|1x2-w32|7.821799|7.842525|7.832193|0.997068 [0.989476,1.003009], 18/31|
|2x2-w32|10.944242|10.963765|10.946027|0.999221 [0.993703,1.005478], 8/15|
|4x2-w32|15.772085|15.725042|15.831752|1.005422 [1.000847,1.010001], 5/15|
|8x2-w32|23.978159|23.978925|24.048435|1.001454 [0.993617,1.009437], 5/15|
|12x2-w32|33.963264|33.779121|33.896805|1.000034 [0.993352,1.005931], 7/15|
|16x2-w32|39.896846|39.911212|39.870816|0.993072 [0.984310,1.000538], 8/15|
|24x2-w32|66.553347|66.230451|66.404179|1.001447 [0.997392,1.005716], 8/15|
|merge-24x1-w32|65.349634|65.597393|65.365760|0.997583 [0.989868,1.005602], 9/15|
|merge-54x1-w32|163.133911|163.311600|163.404904|1.004425 [0.997116,1.012410], 7/15|
|1x2-w1|94.887026|94.644843|94.661267|0.999349 [0.995892,1.002998], 8/15|
|1x2-w16|9.214387|9.215165|9.188609|0.994016 [0.982596,1.001305], 24/31|

Times are medians of arithmetic process means of integer-rounded SDK nanoseconds; paired ratios need not equal quotients of displayed medians. Primary 16 retains ON/OFF 0.994016 [0.982596,1.001305], 24/31 wins, and ON/REF 0.999638 [0.996788,1.002975], 20/31. Solo intervals also cross parity. These estimates do not qualify a worker-specific adoption.

The fixed protocol uses 31 matched primary 16/32 blocks×151 requests,15 other blocks×31 except solo 11, three primes and fixed rotating/reversing context/arm order. The 20,000 fixed-seed paired-log bootstrap and stratified equal-shape aggregate are unchanged. There is no familywise guarantee; stationarity and block exchangeability are not guaranteed. All 114 predetermined independently written complete-ELF backings are retained, with equal-length names, modes, hashes, sizes, distinct dev/inodes, single links and source immutability checked before/after. No backing is selected after capture; this does not establish physical-page coloring or eliminate host drift.

| Context | REF / OFF / ON peak RSS MiB | REF / OFF / ON CPU ms/proof | REF / OFF / ON minor faults/proof |
|---|---:|---:|---:|
|1x2-w32|135.250 / 135.750 / 134.250|178.482 / 178.254 / 178.033|128.364 / 130.311 / 128.940|
|2x2-w32|275.250 / 282.000 / 275.500|258.910 / 257.701 / 257.152|1291.194 / 1300.806 / 1280.032|
|4x2-w32|310.500 / 319.250 / 311.250|421.501 / 420.838 / 420.960|1081.484 / 1115.903 / 1099.226|
|8x2-w32|503.250 / 502.000 / 507.500|674.184 / 673.453 / 673.389|1621.871 / 1676.548 / 1647.290|
|12x2-w32|728.000 / 715.750 / 723.016|950.804 / 950.834 / 950.446|2220.484 / 2358.645 / 2217.484|
|16x2-w32|820.250 / 838.000 / 812.000|1132.155 / 1133.298 / 1133.696|2404.903 / 2537.484 / 2473.839|
|24x2-w32|1277.945 / 1306.453 / 1326.441|1937.260 / 1935.619 / 1937.349|6399.000 / 5326.903 / 4725.323|
|merge-24x1-w32|1157.789 / 1170.137 / 1182.141|1846.791 / 1853.141 / 1849.724|3407.516 / 3682.710 / 3313.000|
|merge-54x1-w32|1974.488 / 1985.199 / 2000.898|4537.648 / 4536.945 / 4545.125|22139.290 / 22003.871 / 22518.194|
|1x2-w1|34.250 / 34.250 / 34.250|94.884 / 94.642 / 94.658|0.000 / 0.000 / 0.000|
|1x2-w16|88.750 / 88.500 / 87.500|117.605 / 117.627 / 117.210|56.702 / 55.497 / 56.126|

RSS is lifetime peak including loading/primes/retained buffers, not minimum RAM. All ON/control RSS guards pass at factor 1.05. CPU/fault counts cover the timed batch, scaled per request; every batch reports zero major faults. Minor faults are not a service-time or pressure/swap measurement.

| Primary32 / arm | Median user CPU ms/proof | Median system CPU ms/proof | Median voluntary switches/proof |
|---|---:|---:|---:|
|REF|173.683199|4.710358|357.582781|
|OFF|173.623550|4.609861|360.463576|
|ON|173.217868|4.719424|366.768212|

These are marginal medians and need not sum to median total CPU. System CPU and voluntary switches are near the current controls. The preceding heap-root campaign's ON had 8.411 ms system CPU and 532.24 switches/proof; here ON has 4.719 ms and 366.77. That is a descriptive cross-campaign comparison, not a controlled causal interaction. Root storage, admission and gathering/scheduling changed together. The large earlier regression is absent, but useful proof latency has not improved.

| Single counter panel | ON/OFF instructions | ON/OFF cycles | ON/OFF branch misses | ON/OFF generic cache misses |
|---|---:|---:|---:|---:|
|1x2-w32|0.996653|1.001607|1.008862|1.053662|
|1x2-w16|0.996558|1.019186|1.016895|0.981154|
|2x2-w32|0.996613|0.988090|1.005470|0.924053|

These panels are descriptive, not replicated attribution. All 9 grouped PMU panels have≥99.9% scheduled coverage. Nine fixed 16M-user-cycle profiles contain **120368 samples**, maximum unknown **1.924%**, zero LOST and checked executable MMAP/PID/TID/CPU. Private owned namespaces appear only in ON:52 primary 16,75 primary 32 and 110 at 2×2-32, confirming execution. Flat samples can contain inlined payload; they are not exclusive critical-path/removable wall time. Absolute period coverage is unvalidated. All 9 raw profiles/decodes are archived.

All **1329 serialized commands succeed once**:33 SDK functional preflights plus 33 Gnark checks,591 latency processes,18 diagnostics,45 offline operations and 609 capture-proof verifications. There are **44,238 internally checked SDK requests including primes**, and **642 fresh unique 164-byte Gnark-verified exports** with checked image/public bindings. Canonical freshness advances **49,363→49,396→50,005**. No successful native, model, crypto or recording work is replayed for bookkeeping.

Both actual readers,79 gates,400 pins, inputs/order/copies are frozen before any SDK functional preflight. The separate same-agent reader reconstructs 591 raw means,79 decisions,609 capture exports plus preflight bindings,176-source recovery and diagnostics. Statistical/diagnostic parsers are shared; this is not an external audit. No owned helper/build/hash/analysis/report overlaps preflight or recording. All recordings precede offline work. New-stage preparation/capture/analysis/review pass first execution; inherited qualification tooling repairs are disclosed in its linked report.

Fresh accepted REF and unadopted ON compared with supplied historical M5 CPU/Metal follow. M5 was not remeasured; these are different-hardware ratios, not equal-efficiency claims.

| Shape | REF / ON ms | M5 CPU ms / REF speedup / ON speedup | M5 Metal ms / REF speedup / ON speedup | CPU10× limit ms |
|---|---:|---:|---:|---:|
|1x2|7.821799 / 7.832193|37 / 4.730× / 4.724×|24 / 3.068× / 3.064×|3.7|
|2x2|10.944242 / 10.946027|59 / 5.391× / 5.390×|30 / 2.741× / 2.741×|5.9|
|4x2|15.772085 / 15.831752|109 / 6.911× / 6.885×|48 / 3.043× / 3.032×|10.9|
|8x2|23.978159 / 24.048435|189 / 7.882× / 7.859×|74 / 3.086× / 3.077×|18.9|
|12x2|33.963264 / 33.896805|284 / 8.362× / 8.378×|101 / 2.974× / 2.980×|28.4|
|16x2|39.896846 / 39.870816|335 / 8.397× / 8.402×|124 / 3.108× / 3.110×|33.5|
|24x2|66.553347 / 66.404179|528 / 7.933× / 7.951×|176 / 2.644× / 2.650×|52.8|
|merge-24x1|65.349634 / 65.365760|507 / 7.758× / 7.756×|175 / 2.678× / 2.677×|50.7|

Production remains unchanged; the CPU AVX10× ALL8 goal is active and unmet, **0/8**. Merge 54 has no supplied M5 reference. No new AWS/Aeglos, ARM/mobile/TEE,250/4999, minimum-RAM, pressure/swap or concurrent-proof-throughput claim follows; no cloud resource is created. Further sibling weighted-fold variants are not selected. The next investigation returns to the documented AWS32→96-worker scaling gap, using existing evidence before another experiment.

[Analysis](artifacts/analysis.json), [raw-reader review](artifacts/review-result.json), [decision](artifacts/decision.json), [diagnostics](artifacts/diagnostic-summary.json), [SDK identities](artifacts/identities.json), [copy identities](artifacts/copies.json), [complete evidence](artifacts/evidence.tar.gz), [member hashes](artifacts/public-members.json), [checksums](SHA256SUMS). The archive includes all native/link/capture receipts, raw profiles/decodes, plans/pins, authored source and the next-agent autoresearch prompt. Native executables/libraries, keys, requests, witnesses, proofs and full ledgers stay private and hash-bound.
