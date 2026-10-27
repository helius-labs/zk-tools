# IBS loads in accepted and disabled runtime builds

The disabled experimental runtime remains slower than the accepted SDK in this diagnostic. Its sampled loads do not show a general increase in cache misses or RAM service. No optimization is adopted. Accepted local 1×2 latency remains **8.324853 ms**; the all-eight CPU 10× objective remains unmet, 0/8.

This compares the exact accepted SDK `14da8231` with experimental SDK `fde4038c` using `ZOPRO_CPU_BATCH_LOCAL_STEAL=0`. The [batch-transfer screen](../2026-10-27-batch-local-steal/README.md) remains failed. The [preceding static audit](../2026-10-27-batch-off-code-audit/README.md) found matching instructions and literal payloads in 16 selected arithmetic functions. These observations do not establish whole-program equivalence.

## Findings

Three fixed paired blocks on a Ryzen 9950X, with 32 workers on CPUs 0–31. Values summarize the three process observations per mode. SDK times include profiling and are not acceptance measurements.

| Mode | Instrumented SDK median, ms | CPU ms/request | Observed process peak RSS, MiB |
|---|---:|---:|---:|
| Accepted control | 8.462729 | 193.120 | 116.75–122.75 |
| Experimental OFF | 9.170174 | 206.742 | 120.75–125.75 |

RSS here describes these instrumented processes; it does not replace the accepted run's 134.50 MiB observation or establish minimum RAM.

| Mode | Group | Demand-load samples | L1 miss % | L2 miss % | RAM-served % | Positive miss-weight mean, cycles |
|---|---|---:|---:|---:|---:|---:|
| Control | All user | 30,461 | 7.218 | 0.837 | 0.0328 | 34.02 |
| OFF | All user | 31,572 | 6.955 | 0.755 | 0.0284 | 33.35 |
| Control | G2 prefix | 4,032 | 11.356 | 0.050 | 0.0000 | 17.97 |
| OFF | G2 prefix | 4,146 | 11.420 | 0.072 | 0.0000 | 19.97 |
| Control | G1 stream | 3,745 | 18.531 | 0.350 | 0.0000 | 14.58 |
| OFF | G1 stream | 3,789 | 17.260 | 0.264 | 0.0000 | 14.04 |
| Control | Affine tree | 3,906 | 4.193 | 0.000 | 0.0000 | 13.80 |
| OFF | Affine tree | 3,937 | 4.151 | 0.051 | 0.0000 | 13.68 |
| Control | Epoch | 940 | 4.681 | 2.191 | 0.1095 | 271.25 |
| OFF | Epoch | 929 | 3.970 | 1.395 | 0.1073 | 202.67 |
| Control | Scheduler | 1,593 | 2.134 | 0.517 | 0.0000 | 129.59 |
| OFF | Scheduler | 2,579 | 1.485 | 0.543 | 0.0000 | 107.76 |

Counts and rates are separate medians, so table entries need not reconstruct one process. RAM counts are small: control processes have 13/10/4 RAM-served user demand loads, and OFF has 9/6/9. No G2-prefix load is RAM-served in any capture. All six have zero uncached demand loads.

The G2-prefix positive miss-weight mean is higher in OFF in each pair, but L1 miss fractions are similar and almost all loads hit L1 or L2. Across all user code, OFF has lower L1 and L2 miss fractions and a lower positive miss-weight mean in each pair. The results therefore provide no support for a general increase in demand-load misses or DRAM service as the explanation for the OFF regression. They do not exclude contention, dependencies, instruction-cache effects, frequency differences or costs outside sampled loads.

Scheduler-attributed demand loads increase in all three pairs: control 1,593/1,548/1,623 versus OFF 2,621/2,579/2,492. OFF contains an outlined victim-scan `filter_try_fold` closure with 1,311/1,304/1,245 demand-load samples. Symbol attribution can move when the compiler outlines or inlines code: these counts do **not** establish that the difference is extra scheduler work, or a removable wall-time fraction. The next targeted audit is this victim-scan path and its call boundaries, including which work is inlined into the accepted caller. No further latency run or production change is justified by these symbol counts alone.

These are three descriptive pairs. Miss weights and sample fractions do not measure exclusive time, bandwidth, cache conflicts, cache capacity or minimum RAM. This comparison contains no new Aeglos measurement and does not explain the C7a gap by itself. The [older Zopro/Aeglos IBS study](../2026-10-23-local-ibs-memory/README.md) used different binaries.

## Capture and verification

Linux 6.8.0-147-generic, perf 6.8.12. Order: control/OFF, OFF/control, control/OFF. Each process makes three priming proofs and 61 measured proofs from the same pinned image and request. FIFO acknowledgments enable sampling after load/priming and disable it before export. Event `ibs_op/cnt_ctl=1/`, period 524,288, `-m 1024 -d -W --raw-samples`; no user/kernel hardware filter is requested.

All six captures finish before independent verification or decoding. No owned helper, build, hashing, archive or second CPU workload overlaps capture. There are **384 internally verified SDK requests**, **six fresh 164-byte exported proofs independently verified with pinned Gnark**, and 12 proof/public members. Original public-input binding and freshness against all prior/new proof identities pass. The proof ledger advances **12,518 → 12,524**.

The captures contain **1,064,185 operation samples**, zero lost records and one user PID each. Every raw record joins exactly one script record by PID, TID, nanosecond timestamp and actual perf IP. Raw operation flags and valid miss weights agree with perf conversion; script and raw-dump data sources and weights match exactly. Individual software-prefetch records are excluded from demand-load groups. Uncached weights are excluded from cache-latency aggregates. Page-line histograms remain descriptive virtual page offsets.

The original workflow stopped at the first analysis because perf omits `IbsOpRip` when `RipInvalid` is set. A separate decoder accepts omission only with that flag, counts it separately and still joins actual perf IP. All 82 omitted pointers across the six captures have that flag. Original source, pins, failure and completed steps are retained. An offline-only continuation performs the remaining decoding, repeating no capture or proof verification.

The continuation also retained an ownership-check failure: root perf rejected a raw file whose ownership had been transferred to the user for closure. Restoring root ownership for the five undecoded files allowed the same plan to resume. The successful analysis checkpoint was reused. After decoding, raw files were transferred to the user for closure hashing; their bytes and mode were unchanged. The original failed workflow is not relabeled successful; the separate continuation has its own completion receipt.

Nine new Rust observer tests pass across three invocations: the initial seven, one uncached-weight refinement and one omitted-invalid-RIP regression. Successful tests are not repeated. A capability-only SHA256 workload supplied 239 samples with exact raw/script matching and zero losses; it contains no SDK requests. The inherited arithmetic, runtime and independent-verifier suites are unchanged and were not rerun.

The existing SDKs and actual linked libraries, 172 production sources, image/request, independent verifier and tools are pinned and checked before/after. There is no prover rebuild. Raw perf captures, scripts and register dumps remain local with hashes; analyses, commands, source, proof exports and verification receipts are archived. A copy of the ownership-error log is retained separately because bulk raw/script directories are excluded from Git.

The [Linux v6.8 IBS implementation](https://github.com/torvalds/linux/blob/v6.8/arch/x86/events/amd/ibs.c) and [perf IBS manual](https://man7.org/linux/man-pages/man1/perf-amd-ibs.1.html) provide field/conversion references. Installed kernel headers and actual sysfs capabilities are archived; upstream source is not asserted byte-identical to Ubuntu's running kernel.

No AWS resources or global perf settings change. This diagnostic does not qualify phone, TEE, memory-pressure, concurrency or large-circuit behavior.

## Inherited CPU and Metal comparison

Accepted local timings and supplied historical M5 references, inherited from the [canonical report](../2026-10-27-clean-fft-adoption/README.md). Its full-performance-gate failure and engineering exception remain recorded. This diagnostic measures none of these reference timings.

| Shape | Accepted ms | CPU reference ms | CPU/accepted | Metal reference ms | Metal/accepted |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37 | 4.445× | 24 | 2.883× |
| 2x2 | 11.460675 | 59 | 5.148× | 30 | 2.618× |
| 4x2 | 15.854365 | 109 | 6.875× | 48 | 3.028× |
| 8x2 | 24.084626 | 189 | 7.847× | 74 | 3.072× |
| 12x2 | 33.693397 | 284 | 8.429× | 101 | 2.998× |
| 16x2 | 39.981636 | 335 | 8.379× | 124 | 3.101× |
| 24x2 | 66.010670 | 528 | 7.999× | 176 | 2.666× |
| Merge 24x1 | 65.086624 | 507 | 7.790× | 175 | 2.689× |

Accepted merge 54 remains 161.341835 ms with no supplied M5 reference. The latest unprofiled primary control observation, 8.358001 ms, remains separate.

[Design](artifacts/design.md), [declaration](artifacts/declaration.json), [observations](artifacts/analysis.json), [proof inventory](artifacts/proof-inventory.json), [source pins](artifacts/pins.json), [offline continuation](artifacts/decode-plan.json), [artifact hashes](artifact-index.json).
