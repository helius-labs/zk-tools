# Signed radix-52 primitive measurements

The fixed signed-difference recipe improves every measured R256 context and both radices' fused operations, but regresses ordinary R260 multiplication. **The declared overall gate fails**, so this global replacement is not adopted and no full-proof trial is selected. These primitive gains are retained; they are not full-proof speedups.

The implementation and its focused Arkworks qualification are in the [native report](../2026-11-06-signed-pair52-native176/README.md). Accepted, fresh OFF and candidate binaries were measured on CPU2 of the Ryzen9 9950X. All sixteen OFF/accepted calibration intervals passed. The protocol used fifteen paired, rotating/reversing blocks, 2^20 rounds per sample, with 48 warmups and 720 timing observations. Intervals are paired order statistics 4/8/12 of15: 96.484375% coverage for a median under independent stationary blocks, not family-wise inference. No minimum gain floor, optional extension or favorable subset was used.

## All contexts

Times are median nanoseconds per lane operation, including fixed initialization, packing and export amortized across the long loop. Ratios are candidate/accepted from paired blocks; smaller is faster. Fresh OFF comparisons and every raw observation are in the linked artifacts.

| Context | Accepted ns | Fresh OFF ns | Candidate ns | Paired ratio [interval] | Wins/15 |
|---|---:|---:|---:|---:|---:|
| fq256_mul1 | 2.3231 | 2.3223 | 2.2451 | 0.96638 [0.96399, 0.96856] | 15 |
| fq256_mul4 | 1.7549 | 1.7552 | 1.7290 | 0.98424 [0.98207, 0.98592] | 15 |
| fq256_sum1 | 3.1881 | 3.1914 | 2.8072 | 0.87966 [0.87862, 0.88144] | 15 |
| fq256_diff1 | 3.1980 | 3.1921 | 3.0278 | 0.94735 [0.94515, 0.94915] | 15 |
| fr256_mul1 | 2.3228 | 2.3210 | 2.2459 | 0.96700 [0.96434, 0.96851] | 15 |
| fr256_mul4 | 1.7503 | 1.7541 | 1.7280 | 0.98573 [0.98228, 0.98937] | 15 |
| fr256_sum1 | 3.1928 | 3.1896 | 2.8082 | 0.88032 [0.87738, 0.88167] | 15 |
| fr256_diff1 | 3.1956 | 3.1931 | 3.0282 | 0.94843 [0.94599, 0.95019] | 15 |
| fq260_mul1 | 2.0855 | 2.0809 | 2.1599 | 1.03551 [1.03458, 1.03733] | 0 |
| fq260_mul4 | 1.5054 | 1.5060 | 1.5746 | 1.04572 [1.04434, 1.04874] | 0 |
| fq260_sum1 | 2.9245 | 2.9246 | 2.7142 | 0.92960 [0.92752, 0.93084] | 15 |
| fq260_diff1 | 2.9564 | 2.9581 | 2.8897 | 0.97648 [0.97500, 0.97925] | 15 |
| fr260_mul1 | 2.0852 | 2.0814 | 2.1585 | 1.03553 [1.03338, 1.03743] | 0 |
| fr260_mul4 | 1.4766 | 1.4771 | 1.6754 | 1.13371 [1.13270, 1.13546] | 0 |
| fr260_sum1 | 2.9218 | 2.9222 | 2.7082 | 0.92763 [0.92503, 0.92884] | 15 |
| fr260_diff1 | 2.9665 | 2.9643 | 2.8490 | 0.96015 [0.95799, 0.96191] | 15 |

Fq/R256 ordinary dependent multiplication improves about **3.4%**, the four-state loop **1.6%**, fused sum **12.0%**, and difference **5.3%**, winning all15 paired blocks against both controls in each context. These are genuine measured primitive gains. Ordinary uniform-R260 regressions are about3.6–13.4%, with zero wins in those four contexts. Eight of58 checks fail: both-control nonregression checks for Fq/Fr R260 ordinary one-/four-state multiplication. The primary gains and aggregate checks pass, but they do not override the prospectively declared all-context requirement.

Four-state callers use distinct states in a compiler-rolled stream loop, not four simultaneously register-resident products. Fused callers keep the second product's operands constant across iterations, allowing some preprocessing hoisting. Timed setup/pack/export and these caller properties limit extrapolation to real curve arithmetic. No G1/G2 complete caller or full SDK proof ran.

## Counters and profiles

All twelve whole-process PMU panels meet the90% coverage requirement, and all twelve cycle profiles qualify. No lost records were detected. Profile ownership is bound using the actual ELF executable segment offset/vaddr, MMAP2, PID/TID, CPU2, symbol and static IP range, accounting for PIE load addresses.

| Fq/R256 context | Accepted cycles/field | Candidate cycles/field | Accepted instructions/field | Candidate instructions/field |
|---|---:|---:|---:|---:|
| fq256_mul1 | 12.789 | 12.290 | 21.129 | 26.879 |
| fq256_mul4 | 9.571 | 9.322 | 23.282 | 29.657 |
| fq256_sum1 | 17.539 | 15.307 | 28.754 | 35.879 |
| fq256_diff1 | 17.531 | 16.419 | 31.254 | 46.129 |

Profiles contain **68116 samples** in total; the minimum target-kernel fraction is 99.9152% and maximum unknown fraction 0.0848%. The candidate executes more instructions yet uses fewer measured cycles in these R256 contexts. This supports keeping the IFMA-versus-additional-arithmetic tradeoff as a useful observation; it does not isolate the causal cost of IFMA dependencies, spills or a cache level. Whole-process PMU numbers include setup/export and are not exclusive loop cycles. Static code/stack findings remain those in the native report; no RSS peak or minimum-RAM measurement is added.

## Capture and decision

The initial workflow completed all792 warmup/timing/stat/record commands, then the first offline decode failed: `sudo perf record` had created root-owned mode0600 files. Only ownership of this campaign's twelve data files was changed to the workspace user. The failed receipt is retained. The same frozen plan/pins resumed, reusing all792 successful receipts and executing only the24 failed/unstarted offline commands. No timing, counter or recording was repeated. The frozen analyzer ran once; a separate review reconstructed all32 paired context/control summaries from raw timings, checked all58 gates, verified all816 successful receipts and the profile/counter guards.

This closes the **global fixed signed-difference replacement** under its declared gate. It does not erase its R256 gains or prove all pairwise algorithms unsuitable. No subset integration, parameter/layout/attribute rescue, unchanged reroll or full-proof speedup claim is selected. The next research question is whether the R256-versus-R260 difference identifies an independent reduction/carry dependency change in the retained arithmetic; existing reduction history must be checked before treating that as a new hypothesis.

Production source176/SDK6d83 remain unchanged. Accepted local32 1×2 stays **7.928660 ms**, merge54 **161.883899 ms**; inherited132.75MiB RSS is not a new peak or minimum. Proof ledger remains **30,846**. The inherited [all-eight M5 CPU and Metal comparison](../2026-11-06-signed-pair52-model176/README.md#accepted-full-proof-baseline-inherited) is unchanged, and the10× CPU goal remains unmet on all eight shapes. No new AWS/Aeglos, large-circuit, mobile, pressure, swap or concurrent-throughput result exists.

[Analysis and all gates](artifacts/analysis.json), [protocol](artifacts/protocol.json), [review](artifacts/review.json), [raw observations](artifacts/raw-observations.json), [pins](artifacts/pins.json) and [source/receipts](artifacts/source-and-receipts.tar.gz) are preserved. Raw perf files and full decoded trace stdout stay local with a [hash/size index](artifacts/local-trace-index.json). Public artifacts contain no executable, library, key, witness or proof blobs.
