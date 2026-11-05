# Native auxiliary-prime Euler operator

The complete AVX512 auxiliary-prime operator is correct on the qualified cases, but slower than the original FFT in every measured context. It is not adopted. At domain 32768 with 32 workers, median operator latency is **1.543040 ms**, versus **0.670646 ms** for the fresh original control. The paired ratio is **2.291323 [2.249450, 2.322071]**, with zero wins in 15 blocks.

This is an operator experiment, not a proof benchmark. Accepted production source176 and the accepted SDK remain unchanged. The all-eight CPU10× goal remains unmet, **0/8**. No SDK proof, Gnark verification, AWS, mobile or memory-pressure run was added by this experiment.

## Implementation and qualification

The [parent convolution qualification](../2026-11-05-crt-euler176/README.md) expresses Euler differentiation as a twisted negacyclic convolution over 17 auxiliary 31-bit primes. Their product exceeds sixteen times the signed coefficient bound at domain 32768. The [integer-boundary qualification](../2026-11-05-crt-euler-direct176/README.md) supplies exact integer projection and fixed-correction CRT formulas. Those algebra results remain valid.

This experiment implements eight-lane integer projection and a fixed-coefficient CRT dot product with a shared modular reduction in Rust AVX512 intrinsics. It uses the existing field SIMD operations for twist, restore and final product combination. All projections, small-prime transforms, reconstruction, allocation and boundary work remain in the measured complete operator. The existing Rayon pool schedules the work; there is no new production pool or retained proving-key table. Restore compensation is applied in place.

The dot requires 201 logical IFMA operations per eight coefficients before linear/carry/packing work. The linked, partly rolled routine has 28 static IFMA instruction sites; these are different counts. Extracted linked disassembly is included. Small-prime transforms use unmodified `tfhe-ntt 0.7.1`; its license is included.

Three new tests pass in both checked-optimized and release-optimized modes. Each mode covers 74,120 padded projection outputs across all 17 primes, 4,104 bounded signed CRT outputs, 8,192 arbitrary-residue dot/selector outputs, and 24 complete small-operator calls at 1/4 workers. Arbitrary residue tests do not establish the bounded signed interpretation. The two build modes repeat the same vectors; they are not twice as many distinct cases. Six parent tests are filtered, not rerun.

A separate domain32768 qualification compares both native and original accepted FFT outputs with an independent low-degree polynomial derivative at 16 and 32 workers: 65,536 native plus 65,536 original coordinate comparisons. It checks all outputs. These are component checks, not proofs.

This stage uses the **original** coherent accepted field1200e635/core9dc62928/Rayonbf7ab8bf libraries. The prior scalar stage used a fresh library from accepted field source; its binary-provenance wording was corrected in commit `d788cb63e8e9eb60ab1967c666556cc7d60f8f72`. No earlier algebra result is retracted. Actual native link receipts contain `-O`, `target-cpu=native`, `lto=fat`, and `codegen-units=1`.

## Complete-operator measurement

Local host: AMD Ryzen 9 9950X, 16 physical / 32 logical CPUs, Linux 6.8.0-147, Rust 1.98.1. Solo uses CPU2, 16 workers use CPUs0–15, and 32 use CPUs0–31. Workers may migrate within their allowed mask; there is no per-worker pinning claim.

FROZEN is an optimized original-only driver. OFF and ON share the same fresh optimized executable; OFF calls the original FFT, ON the native CRT implementation. Binary identities and exact commands are included. Two deterministic full-width synthetic fixture pairs alternate, with scattered zeros and negative units. All inputs/outputs agree across arms and blocks; these are not captured proof witnesses.

Each process prepares its engine, computes independent original-FFT references before timing, and primes each fixture three times. The timer includes input clones and the complete operator; output comparison/drop are after the timer. Every timed output is checked. Preparation is reported separately. Process CPU usage for latency includes the outside-clock checks and cannot be treated as exclusive operator CPU time.

Fifteen fixed rotated/reversed blocks cover six contexts and three arms: 270 latency processes, 151 calls/process at domain32768 and 301 at domain1024. All latency precedes diagnostic stats/profiles. There are 61,020 measured latency calls. Including 18 preflights and six diagnostic callers, there are 294 caller processes, 75,582 engine calls including primes, and 588 additional original-reference calls. None is counted as a proof.

| Domain | Workers | FROZEN ms | OFF ms | ON ms |
|---|---:|---:|---:|---:|
| 1024 | 1 | 0.078017 | 0.078017 | 0.139753 |
| 1024 | 16 | 0.094448 | 0.093938 | 0.110598 |
| 1024 | 32 | 0.124475 | 0.124725 | 0.143971 |
| 32768 | 1 | 3.034184 | 3.032400 | 4.109382 |
| 32768 | 16 | 0.604100 | 0.599080 | 1.234399 |
| 32768 | 32 | 0.673741 | 0.670646 | 1.543040 |

Values are medians of 15 process medians. Ratios use paired block observations, not ratios of the displayed medians. Primary ON/FROZEN is **2.292324 [2.267591, 2.312762]**, zero wins. The required domain32768 16/32-worker geometric aggregate ON/OFF is **2.180871 [2.150773, 2.186359]**, zero wins.

The predeclared full gate fails exactly 16 checks: all twelve candidate nonregression checks, both primary gain checks, and both aggregate gain checks. All six OFF/FROZEN calibrations pass. Nonregression requires upper ratio ≤1.03; calibration must remain within [1/1.03, 1.03]; gain requires upper <1 and at least 10/15 wins. Bounds are ordered block ratios 4–12 of 15, nominal 96.484375% marginal under the declared independent/exchangeable-block assumptions, without a familywise guarantee. No exclusions, extensions, threshold changes or favorable-subset promotion were used.

## Work, setup and memory observations

Grouped PMU panels each cover 151 profile-mode calls after priming, with 100% reported running coverage. Counts per operator are diagnostic, without repeated-panel confidence intervals:

| Counter | FROZEN | OFF | ON |
|---|---:|---:|---:|
| Instructions, millions | 57.199 | 56.964 | 105.743 |
| Cycles, millions | 88.977 | 88.668 | 128.526 |
| Cache misses | 500,969 | 502,752 | 780,822 |
| Branch misses | 42,114 | 42,734 | 73,104 |

The candidate executes about 85.6% more instructions in this panel. This supports rejecting this implementation on total work as well as latency. It is not an exclusive conversion/scheduling/DRAM cost decomposition.

All three cycles:u profiles use period16M and 4097 calls, with the measured region enabled after priming and disabled before final validation. Profile mode consumes every output but checks the last two fixture states after disabling; its boundary differs from the per-call latency timer and it does not enter latency distributions. The initial design said 501 profile calls; the final design increased only that diagnostic length **before any preflight or timing**. Both declarations are retained.

| Arm | Samples | Unknown period | Attribution qualifies |
|---|---:|---:|---|
| FROZEN | 21,314 | 1.426% | yes |
| OFF | 21,107 | 1.374% | yes |
| ON | 32,506 | 10.807% | **no** |

Executable/PID/thread/CPU ownership, period and zero-loss checks pass for all profiles. ON fails the predeclared ≤5% unknown-period threshold; no recapture was performed. Keep that limitation when reading symbol/category totals. Crossbeam/Rayon symbols can contain inlined payload, and sampled symbols are not exclusive wall-time costs. Latency rejection remains supported by the independent latency measurements.

Setup medians and maximum whole-process RSS across **all 15 latency processes** at domain32768:

| Workers | FROZEN prep ms | OFF prep ms | ON prep ms | FROZEN RSS MiB | OFF RSS MiB | ON RSS MiB |
|---|---:|---:|---:|---:|---:|---:|
| 1 | 1.365 | 1.353 | 14.460 | 17.00 | 17.25 | 30.25 |
| 16 | 1.064 | 1.021 | 15.258 | 19.25 | 19.25 | 34.00 |
| 32 | 1.485 | 1.452 | 15.667 | 22.00 | 22.25 | 32.00 |

Known retained parent arrays total 12.625MiB, excluding metadata, BigUint constants, query scratch, fixtures, stacks and other live allocations. RSS includes setup, references and primes. It is not minimum RAM, a rigorous workspace peak, or proof of pressure/mobile suitability. No large-circuit, swap or concurrent-proof memory qualification was attempted.

## Independent review and decision

The independent Rust reviewer imports no campaign analysis or parser code. It validates 1,538 unique source/tool/library pins, all 318 successful qualification/link/capture command receipts and stream hashes, normalized protocol defaults, coherent original libraries/native flags, all 270 raw latency rows, all declared paired gates/aggregates, fixture bindings, setup/RSS summaries, and profile PID/FORK membership, CPU, period, unknown/loss accounting. All 176 production source files and the 29,953-entry proof ledger remain unchanged.

The initial reviewer stopped because xtask serializes absent protocol fields as explicit empty defaults. Only the reviewer was repaired to normalize `environment` and `required_tests`; both reviewer sources and the failure log are retained. No benchmark source, raw data, declared gate or successful test/capture was changed or repeated. The corrected reviewer passes. The runtime qualification, two driver links and 310-command capture each succeeded on their first attempt.

Close this fixed native implementation. Do not adopt it, repeat it unchanged, or replace its complete-operator comparison with an NTT-only result. The negative timing does not invalidate its algebra qualifications or prove all CRT algorithms inferior. The next experiment needs a distinct reduction of dominant full-proof work.

## Unchanged full-proof goal comparison

These are **inherited accepted** 32-worker local measurements and the supplied M5 references, not new measurements from this experiment. Different machines do not establish equal hardware efficiency. Source: [accepted source manifest](../2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json). The latest separate full-SDK experiment remains [finalization/drain composition](../2026-11-05-finalize-drain-bundle176/README.md), whose real primary gain and failed full gate remain unchanged.

| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | CPU/local | Metal/local | CPU10× target ms |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 7.928660 | 37 | 24 | 4.667× | 3.027× | 3.7 |
| 2x2 | 11.151932 | 59 | 30 | 5.291× | 2.690× | 5.9 |
| 4x2 | 16.086386 | 109 | 48 | 6.776× | 2.984× | 10.9 |
| 8x2 | 24.179453 | 189 | 74 | 7.817× | 3.060× | 18.9 |
| 12x2 | 33.884455 | 284 | 101 | 8.381× | 2.981× | 28.4 |
| 16x2 | 40.077381 | 335 | 124 | 8.359× | 3.094× | 33.5 |
| 24x2 | 65.850777 | 528 | 176 | 8.018× | 2.673× | 52.8 |
| merge24x1 | 65.060937 | 507 | 175 | 7.793× | 2.690× | 50.7 |

Merge54x1 inherited local latency is 161.883899ms, without a supplied M5 reference. No new Aeglos/Gnark timing or AWS result is claimed here. Goal remains active and unmet.

## Artifacts and reproduction

See [artifact index](artifact-index.json), [review](artifacts/review.json), [complete analysis](artifacts/analysis.json), [final protocol](artifacts/capture-design-final.md), [reproduction notes](artifacts/reproduce.md) and [checksums](SHA256SUMS).

Sources, declared plans, pins, identities, corrected/initial reviewer, linked disassembly and decoded receipts are published. Executables, libraries, raw perf.data, private keys/images/witnesses and input arrays are excluded. Synthetic input generation is in `bench.rs`. The gzip JSON receipt archive contains relative member paths, exact UTF-8 contents, byte lengths and SHA256s; extraction was checked byte-for-byte. Absolute historical command paths remain as provenance; use the reproduction notes to relocate a new experiment without overwriting this one.
