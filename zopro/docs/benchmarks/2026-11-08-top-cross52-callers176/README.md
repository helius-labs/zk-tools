# Shared top-limb arithmetic: complete G1/G2 caller screen

**Rejected: the fixed candidate is slower overall and substantially slower for G2 at 32 workers.** The paired 32-worker aggregate ON/OFF ratio is 1.033872, with zero winning blocks. G2's ratio is 1.164029, also zero wins. The apparent favorable G1 aggregate is not a qualified gain: its interval includes parity and both 32-worker control calibrations fail. Production remains unchanged.

The [native field qualification](../2026-11-08-top-cross52-native176/README.md) established 92/134 IFMAs versus the original recipe's 100/150, with 554,496 independently checked field lane outputs and canonical exports. This new coherent core integration measures complete changing signed borrowed G1/G2 batches, including the existing inversions, packing, tails and dispatch. No field library is rebuilt.

## Results

|Caller|Frozen median µs/wave|Fresh OFF median µs/wave|ON median µs/wave|Paired ON/OFF ratio [rank interval]|Wins / 7|OFF/frozen calibration|
|---|---:|---:|---:|---:|---:|---|
|G1-200-w1|4.210|4.212|4.216|1.001181 [0.999161, 1.002250]|2|pass|
|G1-200-w16|4.754|4.789|4.803|1.005975 [0.998871, 1.014157]|2|pass|
|G1-200-w32|6.884|7.069|6.976|0.997431 [0.872838, 1.002444]|4|fail|
|G1-352-w1|6.927|6.937|6.938|1.000526 [0.999668, 1.000724]|2|pass|
|G1-352-w16|7.917|7.918|8.019|1.013611 [1.006406, 1.015815]|0|pass|
|G1-352-w32|14.328|14.517|12.274|1.016853 [0.830711, 1.029551]|3|fail|
|G2-200-w1|7.662|7.669|7.764|1.013180 [1.011874, 1.014881]|0|pass|
|G2-200-w16|8.936|8.915|9.307|1.045143 [1.040018, 1.050074]|0|pass|
|G2-200-w32|14.451|14.401|16.492|1.154352 [1.138338, 1.179271]|0|pass|
|G2-352-w1|12.966|12.969|13.023|1.003053 [1.002564, 1.004305]|0|pass|
|G2-352-w16|15.194|15.244|15.844|1.040017 [1.038666, 1.051186]|0|pass|
|G2-352-w32|25.028|24.701|28.763|1.157945 [1.152616, 1.173787]|0|pass|

One wave applies one independent batch per worker; these are component times, not proof latency or proof throughput. The seven paired process ratios use sorted zero-based ranks 1/3/5. These descriptive small-screen intervals are not 95% or familywise confidence intervals. All observations, including the first block and unfavorable control results, are retained.

|Equal-log 32-worker aggregate|ON/OFF ratio [rank interval]|Wins / 7|ON/frozen ratio [rank interval]|Wins / 7|
|---|---:|---:|---:|---:|
|G1-w32|0.929382 [0.910262, 1.010716]|5|0.932424 [0.914603, 1.011837]|5|
|G2-w32|1.164029 [1.143925, 1.168559]|0|1.153217 [1.144213, 1.161947]|0|
|ALL-w32|1.033872 [1.028277, 1.074595]|0|1.040568 [1.030412, 1.078013]|0|

All G2 calibration checks pass, and its 16- and 32-worker regressions trigger the declared early rejection. G2's paired 32-worker medians regress 15.44% and 15.79% for 200 and 352 pairs; the equal-log aggregate is 16.40%, since aggregating paired block ratios differs from combining marginal medians. The G1 estimate of 7.06% lower aggregate time remains visible but is inconclusive: ON/OFF interval [0.910262, 1.010716], and G1 control calibration intervals extend to 1.157841 for 200 pairs and [0.814000, 1.190586] for 352. No faster G1 subset is promoted or used to rescue this candidate.

Promotion required every calibration interval within factor 1.05, every ON/control upper bound at most 1.02, and gains with upper bound below one and at least five wins for each G1/G2/ALL 32-worker aggregate against both controls. There is no minimum positive percentage floor. The result has 19 failed promotion checks. The fixed implementation is closed without an SDK trial, extra blocks, exclusions, threshold changes or unchanged reroll.

## Executed work and profiles

Separate diagnostic processes retain the same checked fixtures and warm FIFO gate. Counter ratios below compare ON/OFF per wave; each is one panel, not a repeated counter uncertainty estimate.

|32-worker caller|Retired instructions ON/OFF|User cycles ON/OFF|
|---|---:|---:|
|G1/200|1.125434|1.009368|
|G1/352|1.133385|1.005246|
|G2/200|1.269668|1.078951|
|G2/352|1.278731|1.063267|

The candidate executes roughly 27–28% more retired instructions in G2 despite fewer IFMAs in its individual field primitives. Its sign/recombination and compiled work are material costs; these observations do not isolate a unique register, cache, SMT or scheduling cause. Generic cache-miss counters are not DRAM-capacity measurements. All 12 profiles qualify: 95041 samples, zero LOST, 100% assigned to the actual process and declared execution threads, and maximum unknown 0.121080%. All four PMU events report 100% running time. Flat sample shares are not removable proof-wall-time fractions.

ON's observed whole-process RSS peaks range 2.75–18.50 MiB across these fixtures. All timed allocation counts are zero and every scratch capacity matches across arms. These peaks include fixture/runtime/code and exclude proving keys; they are not whole-prover RAM requirements, minimum RAM or pressure tests. No key table/copy, retained plan, new production pool or input-sized runtime buffer is introduced.

## Qualification and capture scope

Two new core builds explicitly link the same exact qualified native field library, SHA-256 `62e1dc6432d6840f59771060436bbf446cee074f3d5973b577b248a26c1a8661`. Only the core selector enables the three new FqBatch R256 multiply/sum2/difference2 wrappers. The specialized signed-difference-scaled field primitive remains original, as do Fr FFT arithmetic, scalar paths and four-lane fallbacks. Original methods remain available in OFF. The frozen original is the qualified `389307…` complete-caller binary, not an SDK prover. Exact binary/core hashes and compile recipes are retained.

Each build passes all 127 unfiltered core tests, including two new coupled tests: 73,008 point comparisons per arm, immutable inputs, signs, tails and scratch reuse; and all 53 actual worker indices across 1/4/16/32-worker pools requesting 512 KiB stacks. Generic core selector guards pass using the already qualified no-assembly/no-IFMA field. All 176 original sources recover from the 178-file snapshot: 172 exact copies, four reversible parents and two children. The caller fixture prefix and perf helper are unchanged; the driver changes only its compile-selector name. Existing parent field/model/decoder qualifications are inherited without replay.

The generator-point references share the unchanged scalar field and curve implementation. Timed runs check the final state and every priming result, rather than each intermediate timed state. This adds no independent cryptographic proof verifier. The public README includes these scope clarifications; the archive preserves the generated report bytes.

The nine-command build and 373-command Rust/xtask capture each complete once. Capture contains 36 fresh 64-round preflights, one checker, 252 latency processes with 4,096 rounds, 24 diagnostic processes and 60 offline ownership/decoder/metadata/event/loss commands after all recordings. Its 312 checked caller processes execute 50,455,296 worker batches including three 128-round primes per process. Outputs, capacities and zero timed allocations pass throughout. Solo runs use CPU 2; 16 workers use CPUs 0–15 and 32 use 0–31, with 512 KiB stacks. No owned build, helper, hash, analyzer, decoder, verifier, report or push overlaps any recording, including preflights.

Two offline reader assumptions were repaired before recording. The first qualification reader interpreted an unchanged binary manifest member as UTF-8; the corrected reader hashes unchanged members as bytes and reverses only four textual parents. The first prospective reader expected the old variable name in a source-text assertion; the new aggregate uses `summary` with the same minimum-wins rule. Sources, binary identities, visible failure messages and correction notes are preserved; separate raw stderr files were not captured for those initial attempts. No runtime source, native test population, command order, statistic, gate or successful test/capture was repeated or changed. The corrected prospective review passes before the first preflight.

The analyzer and corrected final reader each succeed once on saved evidence. The latter independently reconstructs process means, paired ratios/ranks/wins, RSS, all aggregate calculations and the final rejection. It shares the detailed PMU/profile parser rather than supplying a second independent decoder. The public analysis view removes only the large `caller_rows` map; complete raw analysis and every receipt remain in the checked UTF-8 archive, with separate public/raw hashes.

## Full-proof comparisons, inherited

This screen adds zero SDK requests and zero independent Gnark exports. Canonical inventory remains 36,477; production176 and accepted SDK6d83 are unchanged. Historical full-proof medians already include 7.86–7.87 ms private candidates and 7.92 ms for that same accepted build. Its latest matched full-proof observation is 8.018438 ms, with unadopted default carry at 7.897108 ms. Different campaigns do not establish an adopted regression or a new record.

The following observations come from the [latest full-proof campaign](../2026-11-08-uniform-carry-default-performance183/README.md), not this rejected caller. Supplied M5 references use different hardware.

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

Accepted merge54 remains 162.442299 ms without a supplied M5 reference. The verified CPU AVX 10× goal remains active and unmet, 0/8 shapes. No new AWS/Aeglos, huge-circuit, phone/ARM/TEE, pressure/swap, minimum-RAM or concurrent-proof-throughput result is added.

[Analysis view](artifacts/analysis.json), [diagnostics](artifacts/diagnostic-analysis.json), [final reader](artifacts/capture-review.json), [prospective protocol](artifacts/capture-protocol.json), [qualification](artifacts/qualification.json), [evidence](artifacts/evidence.json.gz), [checksums](SHA256SUMS).
