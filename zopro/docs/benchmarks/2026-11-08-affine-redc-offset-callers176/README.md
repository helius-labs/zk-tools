# Deferred affine offsets: complete G1/G2 caller screen

**Decision: rejected.** Production is unchanged. The field primitives now run inside complete changing signed borrowed callers, including packing, inversions, scalar tails and dispatch. This screen measures component batches; it adds no proof-latency measurement.

The [field qualification](../2026-11-08-affine-redc-offset176/README.md) checks deferred normalization of canonical affine-coordinate offsets. It preserves the original CIOS multiplication and IFMA count, folding coordinate subtraction into the product's last carry normalization. Both coherent cores reuse that exact qualified field library; only ON enables the new endpoint formulas.

|Caller|Frozen µs/wave|Fresh OFF µs/wave|ON µs/wave|Paired ON/OFF [rank interval]|Wins / 7|Control calibration|
|---|---:|---:|---:|---:|---:|---|
|G1-200-w1|4.209|4.211|4.158|0.988222 [0.987292, 0.988592]|7|pass|
|G1-200-w16|4.755|4.765|4.713|0.992587 [0.987256, 0.997435]|6|pass|
|G1-200-w32|6.866|6.904|6.830|0.988414 [0.977911, 1.144076]|5|fail|
|G1-352-w1|6.921|6.929|6.849|0.988731 [0.986846, 0.989493]|7|pass|
|G1-352-w16|7.906|7.927|7.811|0.984754 [0.981744, 0.988229]|7|pass|
|G1-352-w32|13.920|11.780|11.598|0.999626 [0.818919, 1.011014]|4|fail|
|G2-200-w1|7.653|7.654|8.043|1.050775 [1.049048, 1.052137]|0|pass|
|G2-200-w16|8.935|8.921|9.326|1.046378 [1.043560, 1.047440]|0|pass|
|G2-200-w32|14.775|14.308|14.684|1.007444 [0.976199, 1.029333]|3|fail|
|G2-352-w1|12.951|12.975|13.636|1.052050 [1.050590, 1.052182]|0|pass|
|G2-352-w16|15.186|15.152|15.894|1.048682 [1.042675, 1.058041]|0|pass|
|G2-352-w32|25.165|25.138|25.285|1.005833 [0.998338, 1.010932]|2|pass|

A wave applies one independent batch per worker. Seven paired process ratios use sorted zero-based ranks 1/3/5; these descriptive small-screen intervals are not 95% or familywise confidence intervals. All observations and unfavorable calibration results are retained.

|Equal-log 32-worker aggregate|ON/OFF [rank interval]|Wins / 7|ON/frozen [rank interval]|Wins / 7|
|---|---:|---:|---:|---:|
|G1-w32|0.993435 [0.894891, 1.070931]|5|0.974275 [0.904212, 1.123643]|4|
|G2-w32|1.004906 [0.993414, 1.016073]|3|0.997496 [0.986069, 1.014280]|4|
|ALL-w32|0.999215 [0.952453, 1.019117]|4|0.994076 [0.944254, 1.062131]|4|

The prospectively declared checks produce 22 failed promotion checks and 8 clear-regression checks. Promotion requires every control calibration interval within factor 1.05, every ON/control upper bound at most 1.02, and G1/G2/ALL 32-worker aggregate upper bounds below one with at least five winning blocks against both controls. There is no minimum positive percentage floor. No extra blocks, exclusions, threshold changes or favorable-subset rescue are applied.

The fixed candidate is closed at this caller screen. Positive estimates remain in the tables, but the complete declared promotion rule does not pass. No SDK trial or unchanged reroll follows this result.

The rejection is supported by calibrated 1- and 16-worker G2 measurements, which regress roughly 4.6–5.2% against fresh OFF with zero winning blocks for both sizes. G1's calibrated 1/16-worker medians improve about 0.7–1.5%; these are retained observations, not a qualified overall gain. Three 32-worker control calibrations fail. The 32-worker ALL ON/OFF estimate is near parity, with four wins and an interval crossing one; it does not establish either an overall speedup or a clear overall regression.

## Automatic counters and profiles

Each diagnostic uses a separate checked process with a warm FIFO measurement gate. These single counter panels are descriptive observations.

|32-worker caller|Instructions ON/OFF|Cycles ON/OFF|
|---|---:|---:|
|G1/200|0.983933|0.981002|
|G1/352|0.982525|0.978182|
|G2/200|0.963581|1.009353|
|G2/352|0.962200|1.007730|

In these panels G2 retires roughly 3.6–3.8% fewer instructions, but uses slightly more cycles. Its branch misses increase from about 457 to 1147 per wave at 200 pairs and 473 to 1656 at 352. The new raw correction helper has a data-dependent high-limb early return. Whether that branch explains the latency requires instruction-address attribution; these counters alone do not establish causality. This changes the next investigation from instruction volume to the actual correction/control path. It does not reopen this fixed candidate or qualify a G1-only subset.

Automatic diagnostic quality passed: true. Across 12 profiles there are 93918 samples, maximum unknown 0.104689% and minimum assigned-process/thread share 100.000000%. Each profile's samples, loss result, exact PID/TID/CPU mapping, event attributes, MMAP identity and symbol distribution are retained in the diagnostics. PMU running percentages are retained per event. Flat samples and generic cache counters do not isolate removable proof latency, spills or DRAM capacity.

ON's measured whole-process RSS peaks range 2.75–15.00 MiB across these fixtures. Timed allocation counts are zero and scratch capacities match all controls. Peaks include fixture/runtime/code and exclude proving keys; they are not whole-prover RAM, minimum RAM or pressure measurements. The implementation adds no lookup table, key copy, retained plan, new production pool or larger group record.

## Implementation and validation

G1 replaces stored xsum with canonical bx, retaining five fields, 1600-byte records and 64-byte alignment. Both reserved writes and push constructors use the replacement. Reverse X folds both coordinate offsets into slope² normalization; reverse Y folds ay into its product. G2's owned and signed borrowed prefix endpoints use the corresponding fused X/Y formulas. Offsets are canonical packed original affine coordinates. Generic reduction lane helpers can receive lazy coordinates and retain their original arithmetic. Numerator, denominator, inversion, prefix, scalar tails, G2 nonprefix groups, Fr FFT and portable paths remain original.

Both native cores explicitly link qualified field SHA-256 `8696e96f98eaec64328d5f5441d5fdc46ba3eae1e163e4c800c1dc84f8b1e701`; the field library is not rebuilt. Frozen original caller `389307…` provides the third control. Source reconstruction checks the qualified parent: 170 of 176 original files are byte-exact accepted copies, the field parent is the qualified append, and five core parents have reversible edits; two private children produce 178 files. The fixture prefix and perf helper are byte-exact, and the driver changes only its selector label. Private field module declaration/generator integration remains required before any production adoption.

OFF passes 128 unfiltered core tests, including 146016 coupled point comparisons and 37920 owned/unsigned point comparisons, plus 53 actual workers across 1/4/16/32-worker pools requesting 512 KiB stacks. New tests cover signs, edge sizes, full groups, scalar tails, FAST modes, both G1 stores, changing state, input immutability and scratch reuse. The generator-coefficient oracle uses unchanged library arithmetic; it is not independent Gnark proof verification.

ON passes 128 unfiltered core tests, including 146016 coupled point comparisons and 37920 owned/unsigned point comparisons, plus 53 actual workers across 1/4/16/32-worker pools requesting 512 KiB stacks. New tests cover signs, edge sizes, full groups, scalar tails, FAST modes, both G1 stores, changing state, input immutability and scratch reuse. The generator-coefficient oracle uses unchanged library arithmetic; it is not independent Gnark proof verification.

The generic no-assembly/no-IFMA selector guard compiles against the previously qualified generic field; this is not ARM execution. The nine-command build completes once. The prospectively checked 373-command Rust/xtask capture contains 36 fresh preflights, one checker, 252 latency processes, 24 diagnostic processes and 60 offline ownership/decoder/metadata/event/loss commands after all recording. Its 312 checked caller processes execute 50,455,296 worker batches including primes, not proofs. Solo CPU2, 16 workers CPU0–15 and 32 workers CPU0–31 are fixed, with 512 KiB worker stacks. No owned build/helper/hash/analyzer/decoder/verifier/report/push overlaps recording, including preflight.

The analyzer and final reader reconstruct process means, paired ratios, ranks, wins, resource peaks, output/capacity agreement and the complete decision. Detailed PMU/profile parsing is shared rather than independently decoded twice. Parent field math/native/consumer-codegen checks are inherited without replay. Helper compilation commands were issued directly; they do not have complete xtask build receipts. Runtime builds, captures and final analysis have workflow receipts.

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
