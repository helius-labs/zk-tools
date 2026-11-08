# Branchless affine offsets: complete G1/G2 caller screen

**Decision: rejected.** Production is unchanged. The field primitives now run inside complete changing signed borrowed callers, including packing, inversions, scalar tails and dispatch. This screen measures component batches; it adds no proof-latency measurement. Calibrated G2 medians remain 1.8–3.0% slower at one and sixteen workers, with zero winning blocks for both sizes. G1 improves about 0.6–1.3% in those contexts, but that subset does not qualify the full implementation.

The [field qualification](../2026-11-08-affine-redc-offset176/README.md) checks deferred normalization of canonical affine-coordinate offsets. It preserves the original CIOS multiplication and IFMA count, folding coordinate subtraction into the product's last carry normalization. This candidate removes the biased raw-correction early return and always executes the same subtraction/borrow/mask blend. Both coherent cores use its newly qualified field library; only ON enables the endpoint formulas. The parent exact model is inherited; 619,520 lane outputs are freshly checked against Arkworks and the original sequence. See the new [field qualification](artifacts/field-qualification.json).

|Caller|Frozen µs/wave|Fresh OFF µs/wave|ON µs/wave|Paired ON/OFF [rank interval]|Wins / 7|Control calibration|
|---|---:|---:|---:|---:|---:|---|
|G1-200-w1|4.211|4.207|4.165|0.989964 [0.988060, 0.990143]|7|pass|
|G1-200-w16|4.769|4.760|4.722|0.991848 [0.990808, 0.993050]|6|pass|
|G1-200-w32|8.090|6.874|6.897|1.006860 [1.003268, 1.050206]|0|fail|
|G1-352-w1|6.915|6.924|6.838|0.987302 [0.986954, 0.988113]|7|pass|
|G1-352-w16|7.925|7.903|7.843|0.993719 [0.986865, 0.994641]|7|pass|
|G1-352-w32|11.743|11.529|11.771|1.001928 [0.995792, 1.022603]|3|fail|
|G2-200-w1|7.662|7.665|7.888|1.029727 [1.027616, 1.030873]|0|pass|
|G2-200-w16|8.916|8.911|9.149|1.025287 [1.022746, 1.030785]|0|pass|
|G2-200-w32|14.931|15.079|14.418|0.965843 [0.940375, 0.994091]|6|fail|
|G2-352-w1|12.967|12.977|13.196|1.017611 [1.016758, 1.018815]|0|pass|
|G2-352-w16|15.199|15.172|15.612|1.029015 [1.023997, 1.029666]|0|pass|
|G2-352-w32|24.854|25.129|24.940|0.995794 [0.981846, 1.007477]|4|pass|

A wave applies one independent batch per worker. Seven paired process ratios use sorted zero-based ranks 1/3/5; these descriptive small-screen intervals are not 95% or familywise confidence intervals. All observations and unfavorable calibration results are retained.

|Equal-log 32-worker aggregate|ON/OFF [rank interval]|Wins / 7|ON/frozen [rank interval]|Wins / 7|
|---|---:|---:|---:|---:|
|G1-w32|1.012422 [1.002920, 1.086830]|1|0.924020 [0.908581, 1.115822]|4|
|G2-w32|0.979237 [0.962431, 1.000762]|5|1.001710 [0.972927, 1.018147]|3|
|ALL-w32|0.992381 [0.984143, 1.031632]|4|0.962081 [0.953660, 1.054642]|4|

The prospectively declared checks produce 20 failed promotion checks and 6 clear-regression checks. Promotion requires every control calibration interval within factor 1.05, every ON/control upper bound at most 1.02, and G1/G2/ALL 32-worker aggregate upper bounds below one with at least five winning blocks against both controls. There is no minimum positive percentage floor. No extra blocks, exclusions, threshold changes or favorable-subset rescue are applied.

The complete prospectively declared promotion rule does not pass. All positive estimates remain visible, but this fixed candidate closes without an SDK trial, extra blocks, exclusions, threshold changes or an unchanged reroll.

## Automatic counters and profiles

Each diagnostic uses a separate checked process with a warm FIFO measurement gate. These single counter panels are descriptive observations.

|32-worker caller|Instructions ON/OFF|Cycles ON/OFF|
|---|---:|---:|
|G1/200|0.985727|1.019018|
|G1/352|0.984495|0.994841|
|G2/200|0.977968|1.028567|
|G2/352|0.977115|1.012459|

|32-worker caller|Branch misses OFF/wave|Branch misses ON/wave|ON/OFF|
|---|---:|---:|---:|
|G1/200|444.731|440.517|0.990525|
|G1/352|449.985|446.344|0.991907|
|G2/200|469.974|452.506|0.962831|
|G2/352|466.797|462.193|0.990137|

These diagnostic panels put branch misses near fresh OFF, while G2 cycles remain higher. Removing the shortcut alone does not qualify the fused path. These branch-miss observations are from the new masked candidate. Its raw-correction shortcut is absent. Differences versus the previously rejected branchful trial are cross-campaign observations, not a paired causal ablation or proof-latency gain.

Automatic diagnostic quality passed: true. Across 12 profiles there are 94284 samples, maximum unknown 0.077469% and minimum assigned-process/thread share 100.000000%. Each profile's samples, loss result, exact PID/TID/CPU mapping, event attributes, MMAP identity and symbol distribution are retained in the diagnostics. PMU running percentages are retained per event. Flat samples and generic cache counters do not isolate removable proof latency, spills or DRAM capacity.

ON's measured whole-process RSS peaks range 2.50–15.50 MiB across these fixtures. Timed allocation counts are zero and scratch capacities match all controls. Peaks include fixture/runtime/code and exclude proving keys; they are not whole-prover RAM, minimum RAM or pressure measurements. The implementation adds no lookup table, key copy, retained plan, new production pool or larger group record.

## Implementation and validation

G1 replaces stored xsum with canonical bx, retaining five fields, 1600-byte records and 64-byte alignment. Both reserved writes and push constructors use the replacement. Reverse X folds both coordinate offsets into slope² normalization; reverse Y folds ay into its product. G2's owned and signed borrowed prefix endpoints use the corresponding fused X/Y formulas. Offsets are canonical packed original affine coordinates. Generic reduction lane helpers can receive lazy coordinates and retain their original arithmetic. Numerator, denominator, inversion, prefix, scalar tails, G2 nonprefix groups, Fr FFT and portable paths remain original.

Both native cores explicitly link qualified field SHA-256 `6ce73715b5a3c6acb5748bb06d5080b2c3796112a63ad1f92025dd41ee1ba04b`; the field library was compiled and checked in the separate four-command native workflow. Frozen original caller `389307…` provides the third control. Source reconstruction checks the qualified parent: 170 of 176 original files are byte-exact accepted copies, the field parent is the qualified append, and five core parents have reversible edits; two private children produce 178 files. The fixture prefix and perf helper are byte-exact, and the driver changes only its selector label. Private field module declaration/generator integration remains required before any production adoption.

OFF passes 128 unfiltered core tests, including 146016 coupled point comparisons and 37920 owned/unsigned point comparisons, plus 53 actual workers across 1/4/16/32-worker pools requesting 512 KiB stacks. New tests cover signs, edge sizes, full groups, scalar tails, FAST modes, both G1 stores, changing state, input immutability and scratch reuse. The generator-coefficient oracle uses unchanged library arithmetic; it is not independent Gnark proof verification.

ON passes 128 unfiltered core tests, including 146016 coupled point comparisons and 37920 owned/unsigned point comparisons, plus 53 actual workers across 1/4/16/32-worker pools requesting 512 KiB stacks. New tests cover signs, edge sizes, full groups, scalar tails, FAST modes, both G1 stores, changing state, input immutability and scratch reuse. The generator-coefficient oracle uses unchanged library arithmetic; it is not independent Gnark proof verification.

The generic no-assembly/no-IFMA selector guard compiles against the previously qualified generic field; this is not ARM execution. The nine-command build completes once. The prospectively checked 373-command Rust/xtask capture contains 36 fresh preflights, one checker, 252 latency processes, 24 diagnostic processes and 60 offline ownership/decoder/metadata/event/loss commands after all recording. Its 312 checked caller processes execute 50,455,296 worker batches including primes, not proofs. Solo CPU2, 16 workers CPU0–15 and 32 workers CPU0–31 are fixed, with 512 KiB worker stacks. No owned build/helper/hash/analyzer/decoder/verifier/report/push overlaps recording, including preflight.

The analyzer and final reader reconstruct process means, paired ratios, ranks, wins, resource peaks, output/capacity agreement and the complete decision. Detailed PMU/profile parsing is shared rather than independently decoded twice. The parent exact mathematical model is inherited without replay; the changed native code is newly checked. Four complete caller bodies are decoded; comparison with saved parent decodes confirms removal of exactly two G1 and five G2 raw-correction mask-test branches, with all remaining helper-origin histograms unchanged. This is static evidence, not a global constant-time or speed claim. The initial reader failed an arbitrary source-line instruction-count assumption before timing; inlined intrinsic attribution makes that assumption invalid. The corrected reader uses actual mask-test branch pairs and retains the G2 source-attribution limitation. Original source, visible failure details and corrected output are archived; no decoder or runtime test is repeated. Helper compilation commands were issued directly; they do not have complete xtask build receipts. Runtime builds, captures and final analysis have workflow receipts.

## Full-proof comparison, inherited

This caller screen adds zero SDK requests and zero independent Gnark exports. Canonical inventory remains 36,477; production176/accepted SDK6d83 are unchanged. Historical accepted full-proof timing was already 7.915095 ms, with private candidates at 7.859756/7.869400 ms. The latest matched accepted observation is 8.018438 ms. Different campaigns do not establish an adopted regression or a new record.

The following full-proof observations are inherited from the [latest full-proof campaign](../2026-11-08-uniform-carry-default-performance183/README.md). Supplied M5 references use different hardware.

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

Accepted merge54 remains 162.442299 ms without a supplied M5 reference. The verified CPU AVX 10× goal remains active and unmet, 0/8 shapes. No new AWS/Aeglos, huge-circuit, mobile/ARM/TEE, pressure/swap, minimum-RAM or concurrent-proof result is added.

[Analysis](artifacts/analysis.json), [diagnostics](artifacts/diagnostic-analysis.json), [final reader](artifacts/capture-review.json), [protocol](artifacts/early-protocol.md), [qualification](artifacts/qualification.json), [reproducibility archive](artifacts/evidence.json.gz), [checksums](SHA256SUMS).
