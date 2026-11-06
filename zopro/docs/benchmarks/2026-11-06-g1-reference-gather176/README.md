# G1 reference gathers in complete update batches

**The candidate fails the declared caller gate and is not adopted.** At one and 16 workers it is consistently about 0.3–0.5% slower than fresh OFF. The 32-worker results fail calibration and aggregate gain checks. Accepted local 1×2 proof latency remains 7.928660 ms.

The experiment changes four forward gather expressions in the G1 stream. Two eight-point arrays for bucket operands and two for incoming operands hold references instead of copied affine values. Coordinate packing, arithmetic, signs, inverse and tail paths, reverse evaluation and scratch records retain their original operations. Original and changed kernels coexist in one diagnostic executable. There is no production dispatch change, field change, lookup table, additional point buffer or new unsafe operation.

The apparent source copies were a hypothesis about work to remove. In the signed borrowed functions, original/reference code has 4,425/4,427 instruction sites, 800/800 IFMA sites, 586/590 stack-memory operand sites and 286/270 vector stack-memory operand sites. Explicit stack subtractions sum to 4,288/4,160 bytes, excluding pushes, alignment and callees. Fewer vector stack sites do not establish fewer executed copies, a lower stack peak or faster code. Earlier record-storage, late-coordinate, bucket52 and unit-prefix experiments remain closed.

## Complete caller comparison

Ryzen 9950X, 16 physical and 32 logical CPUs. Six contexts cover signed borrowed G1, 200/352 pairs and 1/16/32 workers. Fifteen fixed rotated/reversed process blocks time 8,192 complete updates per fixture after three checked 128-update primes. Wave time includes dispatch/join, kernel calls, packing, inversion and scatter. Fixture construction and generator-scalar output checks are outside the timer.

Workers pin individually to CPU 2 or CPUs 0–15/0–31 and use 512 KiB stacks. Fixtures are Rayon tasks; each fixture is not permanently bound to one worker. This is a component comparison, not full-MSM or proof latency.

FROZEN is the retained `unit-prefix176/control-bin`, containing the original kernel. It is an original diagnostic executable, not the accepted SDK. OFF and ON are identical new executable copies with a selector. Both final compiler commands use optimization level 3, native CPU targeting, fat LTO and one codegen unit. The new Cargo build links coherent native libraries.

| Pairs / workers | FROZEN µs/wave | OFF µs/wave | ON µs/wave | Paired ON/OFF [interval] | Wins/15 |
|---|---:|---:|---:|---|---:|
| 200 / 1 | 4.289600 | 4.221931 | 4.236415 | 1.003660 [1.001646, 1.004590] | 2 |
| 200 / 16 | 4.867144 | 4.809913 | 4.824119 | 1.003754 [1.001904, 1.004487] | 2 |
| 200 / 32 | 6.964273 | 8.094686 | 6.942498 | 0.997783 [0.854673, 1.009465] | 8 |
| 352 / 1 | 7.013319 | 6.947720 | 6.966929 | 1.002993 [1.000563, 1.003954] | 2 |
| 352 / 16 | 8.033252 | 7.951756 | 7.993105 | 1.005066 [1.002941, 1.007249] | 0 |
| 352 / 32 | 11.758805 | 11.665014 | 11.754310 | 1.003093 [0.806252, 1.019858] | 6 |

Times are medians of 15 wave observations; paired ratios are not ratios of those medians. The two-size 32-worker ON/OFF aggregate is **0.981831 [0.893608, 1.013538], 9/15 wins**. Against FROZEN it is **0.996749 [0.919310, 1.084764], 8/15 wins**. Neither establishes the required gain. Stable one/16-worker observations favor OFF; they do not identify the cause of 32-worker variability.

Every OFF/FROZEN calibration interval must fit [1/1.03, 1.03], and every ON/control upper bound must be ≤1.03. Both 32-worker aggregate upper bounds must be <1 with at least 10/15 wins. Ordered ranks 4–12 of 15 give nominal 96.484375% marginal coverage conditional on exchangeable blocks, without familywise coverage.

Five checks fail: both 32-worker calibrations, 200/32 ON/FROZEN nonregression, and both aggregate gains. There is no gain floor, exclusion, extension, parameter sweep, changed gate or unchanged retry.

## CPU and memory diagnostics

All 270 latency callers finish before six warm PMU panels and six separate fixed-period 16M `cycles:u` profiles. No owned build, hashing, archive, analysis, verifier or other workload overlaps recording; exclusive host use is not established. The first capture completes all 330 commands.

| Pairs at 32 workers | OFF / ON instructions per wave | OFF / ON cycles per wave | OFF / ON generic cache misses per wave |
|---|---:|---:|---:|
| 200 | 1,678,120 / 1,681,469 | 949,294 / 953,607 | 18.448 / 43.033 |
| 352 | 2,725,731 / 2,731,557 | 1,563,587 / 1,572,390 | 49.560 / 67.673 |

These PMU panels show about 0.20–0.21% more instructions and 0.45–0.56% more cycles for ON versus OFF. Generic cache misses are not measured DRAM traffic or a causal memory diagnosis. All four events run at least 99.9% of their enabled interval.

All six profiles qualify: 30,881 samples in total, zero loss and at most 0.048% unknown symbols. Actual PID/TID, worker CPU, MMAP/FORK and period bindings are checked. Flat samples and static operands are not exclusive critical-path time; absolute sample-period coverage is not validated.

All arms retain 40,000/70,400 record bytes per 200/352-point fixture, with zero tail capacity for these complete SIMD sizes. Timed allocations are zero and capacities agree. Maximum process RSS across latency cells is 8.5 MiB. These synthetic-process counters are not whole-prover peak RAM, minimum RAM or pressure qualification. Retained heap scratch does not decrease.

## Correctness and review

All 128 unfiltered core tests pass, including three new cases covering signed/owned inputs, 21 boundary/tail sizes, original scratch store/inversion modes, reuse/immutability and 1/4/16/32-worker pools with 512 KiB stacks. Field sources are exact and inherit their prior qualification; no fresh field or full-prover suite is claimed.

The snapshot has 178 files: 174 original files remain exact and two diagnostic parents have append-only changes. Reversing those additions recovers all 176 originals. Reversing the four gather substitutions and private signature also recovers the original kernel. Production source is independently checked unchanged.

The 300 checked caller processes execute 36,126,720 latency kernel calls; 51,874,176 measured calls including preflights, PMU and profiles; and 1,953,792 priming calls. Total capture calls are 53,827,968. Every prime and final output is checked against generator-scalar arithmetic; intermediate timed outputs are not independently checked one by one. These are kernel calls, not proofs. No SDK/Gnark export is produced, and the canonical proof ledger stays at 30,846.

Independent review reconstructs 417 pin occurrences, 333 successful build/capture commands and stream hashes, caller output/capacity signatures, timings, ratios, gates, CPU/RSS summaries, raw PMU counts and all 30,881 profile samples. It reproduces the same five failed checks.

Preserved helper failures do not repeat any successful test, build or capture. The initial workflow plan omits its required version field and stops before compilation. The first static analyzer expects an original owned specialization as a separate symbol, which this build does not expose. Two reviewer versions reject one-ULP JSON decimal roundtrip differences at separate timing and PMU sites; the final reviewer uses relative tolerance 1e-12 for reconstructed floating summaries. Raw integers, commands, hashes, gates and observations are unchanged. Initial sources, logs, partial disassemblies and final helpers are archived.

The candidate is closed without SDK integration, portable/device, pressure, swap, large-circuit or concurrent-proof qualification. No AWS/Aeglos measurement is added. This result does not rule out every possible gather or layout change.

## Accepted CPU and Metal comparison, inherited

These accepted local measurements and supplied historical M5 references are unchanged, not remeasured here. All-eight CPU10× remains active and unmet, 0/8. Cross-machine timings do not establish equal hardware efficiency.

| Shape | Accepted CPU ms | M5 CPU ms | M5 Metal ms | CPU10× target ms |
|---|---:|---:|---:|---:|
| 1×2 | 7.928660 | 37 | 24 | 3.7 |
| 2×2 | 11.151932 | 59 | 30 | 5.9 |
| 4×2 | 16.086386 | 109 | 48 | 10.9 |
| 8×2 | 24.179453 | 189 | 74 | 18.9 |
| 12×2 | 33.884455 | 284 | 101 | 28.4 |
| 16×2 | 40.077381 | 335 | 124 | 33.5 |
| 24×2 | 65.850777 | 528 | 176 | 52.8 |
| merge24 | 65.060937 | 507 | 175 | 50.7 |

Merge54 remains 161.883899 ms without a supplied M5 reference. Accepted SDK SHA-256 is `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`.

[Analysis](artifacts/analysis.json), [review](artifacts/review.json), [source audit](artifacts/source-audit.json), [static code](artifacts/codegen.json), [identities and compiler commands](artifacts/identities.json), [symbols](artifacts/profile-symbols.json), [design](artifacts/design.md), [caller protocol](artifacts/caller-protocol.md), [source snapshot](artifacts/source-snapshot.hex.json.gz), [receipts, helpers and disassembly](artifacts/receipts.json.gz), [raw profile inventory](artifacts/raw-profile-inventory.json), [checksums](SHA256SUMS).

Six raw profiles are losslessly compressed beside the inventory. All archives round-trip exact bytes; source and receipt members retain lengths and hashes. Executables, libraries and private proving inputs are excluded. Report prose was edited after archive generation. Reproduction requires the pinned toolchain, dependencies and retained original diagnostic executable. Private root: `/home/ubuntu/src/zopro-perf-work/g1-reference-gather176`.
