# Accepted SDK executable backing diagnostic

**The large same-byte component effect does not reproduce as a large full-proof difference in this run.** Primary 1×2 takes 8.385567 ms through the accepted executable's original inode and 8.452564 / 8.415173 ms through two fixed, byte-identical fresh files. All twenty paired copy/original intervals span parity. All 28,044 SDK requests and 531 independent Gnark exports pass. Production remains unchanged.

All copies and outcomes are retained. This is a diagnostic of the preceding component-level backing/frontend observation; it introduces no algorithm or executable-copy selection policy. The observations do not establish exact equivalence, a universal absence of backing effects, or a physical-page/cache-color/kernel cause.

## Verified request latency

Ryzen 9950X, 32 logical CPUs / 16 physical cores. Fifteen fixed rotated/reversed process blocks, three executables per cell. Primary has 151 timed requests per process; other shapes have 31. Three primes precede each process. Witness solving, fresh randomized proving, SDK verification, and serialization are timed; loading is excluded. CPU time includes coordination. RSS includes loading and priming. Values below are medians of block medians; they are warm single-request latency, not parallel proof throughput.

| Shape/workers | Executable | Median ms | CPU ms/request | Median peak MiB | Max peak MiB |
|---|---|---:|---:|---:|---:|
| 12x2-w32 | copy-001-x | 34.512299 | 943.254 | 693.00 | 710.25 |
| 12x2-w32 | copy-002-x | 34.417323 | 941.647 | 696.25 | 712.00 |
| 12x2-w32 | original-x | 34.435066 | 939.643 | 685.25 | 705.00 |
| 16x2-w32 | copy-001-x | 40.832529 | 1125.878 | 796.50 | 820.50 |
| 16x2-w32 | copy-002-x | 41.074344 | 1129.481 | 791.00 | 831.50 |
| 16x2-w32 | original-x | 40.662153 | 1118.125 | 798.00 | 817.00 |
| 1x2-w16 | copy-001-x | 10.463186 | 126.070 | 86.50 | 90.75 |
| 1x2-w16 | copy-002-x | 10.318409 | 125.222 | 85.00 | 91.00 |
| 1x2-w16 | original-x | 10.743815 | 124.768 | 86.75 | 90.00 |
| 1x2-w32 | copy-001-x | 8.452564 | 192.024 | 130.50 | 134.25 |
| 1x2-w32 | copy-002-x | 8.415173 | 191.723 | 131.00 | 135.75 |
| 1x2-w32 | original-x | 8.385567 | 191.858 | 130.25 | 132.50 |
| 24x2-w32 | copy-001-x | 67.094938 | 1892.526 | 1237.33 | 1294.83 |
| 24x2-w32 | copy-002-x | 67.512809 | 1903.405 | 1250.63 | 1312.25 |
| 24x2-w32 | original-x | 66.729473 | 1880.946 | 1246.55 | 1321.81 |
| 2x2-w32 | copy-001-x | 11.622474 | 266.886 | 260.25 | 280.50 |
| 2x2-w32 | copy-002-x | 11.545287 | 265.720 | 261.25 | 270.00 |
| 2x2-w32 | original-x | 11.603096 | 266.781 | 259.50 | 273.00 |
| 4x2-w32 | copy-001-x | 16.381059 | 422.959 | 299.00 | 315.50 |
| 4x2-w32 | copy-002-x | 16.447609 | 423.511 | 301.00 | 312.25 |
| 4x2-w32 | original-x | 16.565043 | 425.146 | 300.00 | 312.00 |
| 8x2-w32 | copy-001-x | 24.867072 | 672.163 | 490.25 | 515.50 |
| 8x2-w32 | copy-002-x | 24.578090 | 669.318 | 494.25 | 511.25 |
| 8x2-w32 | original-x | 24.689731 | 668.191 | 491.00 | 502.50 |
| merge-24x1-w32 | copy-001-x | 66.679899 | 1822.350 | 1129.77 | 1169.00 |
| merge-24x1-w32 | copy-002-x | 66.068862 | 1811.791 | 1122.25 | 1194.71 |
| merge-24x1-w32 | original-x | 66.301464 | 1815.342 | 1122.29 | 1208.55 |
| merge-54x1-w32 | copy-001-x | 164.817753 | 4417.695 | 1781.26 | 1901.02 |
| merge-54x1-w32 | copy-002-x | 163.746601 | 4423.626 | 1785.20 | 1848.66 |
| merge-54x1-w32 | original-x | 163.928034 | 4428.667 | 1749.00 | 1855.06 |

## Paired fresh/original ratios

Ratios below one favor the fresh file. Bounds use ranks 4..12 of 15 paired process blocks, nominal 96.484375% under independent exchangeable blocks; there is no familywise guarantee. No block is excluded, extended, or repeated after success. These are descriptive comparisons between copies of one algorithm, not a passed source-optimization gate.

| Cell/copy/original | Paired median | Lower | Upper | Wins |
|---|---:|---:|---:|---:|
| 12x2-w32/copy-001-x/original-x | 1.003294 | 0.992019 | 1.011260 | 6/15 |
| 12x2-w32/copy-002-x/original-x | 1.002426 | 0.986930 | 1.011474 | 6/15 |
| 16x2-w32/copy-001-x/original-x | 1.005322 | 0.996949 | 1.019191 | 6/15 |
| 16x2-w32/copy-002-x/original-x | 1.004607 | 0.993694 | 1.016848 | 5/15 |
| 1x2-w16/copy-001-x/original-x | 0.989023 | 0.954787 | 1.057788 | 8/15 |
| 1x2-w16/copy-002-x/original-x | 0.980827 | 0.944021 | 1.000629 | 11/15 |
| 1x2-w32/copy-001-x/original-x | 1.006908 | 0.998739 | 1.015449 | 4/15 |
| 1x2-w32/copy-002-x/original-x | 1.006182 | 0.994925 | 1.011635 | 4/15 |
| 24x2-w32/copy-001-x/original-x | 1.009595 | 0.995411 | 1.018383 | 5/15 |
| 24x2-w32/copy-002-x/original-x | 1.007930 | 0.998977 | 1.028929 | 4/15 |
| 2x2-w32/copy-001-x/original-x | 1.004089 | 0.996499 | 1.009840 | 5/15 |
| 2x2-w32/copy-002-x/original-x | 0.998443 | 0.993813 | 1.006256 | 9/15 |
| 4x2-w32/copy-001-x/original-x | 0.989633 | 0.973097 | 1.010109 | 9/15 |
| 4x2-w32/copy-002-x/original-x | 1.003425 | 0.984267 | 1.011442 | 6/15 |
| 8x2-w32/copy-001-x/original-x | 1.008456 | 0.992090 | 1.022847 | 5/15 |
| 8x2-w32/copy-002-x/original-x | 0.988211 | 0.979329 | 1.010675 | 9/15 |
| merge-24x1-w32/copy-001-x/original-x | 1.000667 | 0.998753 | 1.012865 | 6/15 |
| merge-24x1-w32/copy-002-x/original-x | 1.002019 | 0.988791 | 1.004936 | 7/15 |
| merge-54x1-w32/copy-001-x/original-x | 0.999669 | 0.992750 | 1.011128 | 8/15 |
| merge-54x1-w32/copy-002-x/original-x | 0.996730 | 0.989699 | 1.010416 | 9/15 |

## Frontend counters

Separate counter processes follow all latency captures: three blocks, primary at 16/32 workers and merge-54x1 at 32, all three executables, two four-event grouped userland panels, 31 requests after three primes. One shared FIFO pair gates the measured request loop. All counted events must run at least 99.9%; complete percentages and all observations are retained in analysis.json. The table reports median per-request counts. Cycles/instructions come from the fetch panel; frontend/L1D from the stall panel. Panel observations are separate runs, so no synthetic combined cycle decomposition is implied. Counter-run wall times are not pooled into latency results.

| Cell/executable | Cycles M | Instructions M | L1I misses M | iTLB misses | Frontend stall M | L1D misses M |
|---|---:|---:|---:|---:|---:|---:|
| 1x2-w16/copy-001-x | 610.410 | 1397.737 | 1.232044 | 155.4 | 31.684 | 29.981 |
| 1x2-w16/copy-002-x | 594.499 | 1395.831 | 0.683690 | 148.9 | 28.658 | 29.787 |
| 1x2-w16/original-x | 602.492 | 1397.141 | 0.609398 | 174.4 | 28.053 | 29.885 |
| 1x2-w32/copy-001-x | 869.921 | 1440.422 | 1.299447 | 1101.7 | 63.891 | 38.310 |
| 1x2-w32/copy-002-x | 865.332 | 1439.899 | 0.808164 | 1108.0 | 61.270 | 38.387 |
| 1x2-w32/original-x | 869.042 | 1438.784 | 0.609593 | 996.7 | 61.140 | 38.164 |
| merge-54x1-w32/copy-001-x | 20873.813 | 18020.309 | 8.001993 | 14333.0 | 553.461 | 795.584 |
| merge-54x1-w32/copy-002-x | 20641.661 | 18019.135 | 3.226987 | 13146.4 | 533.791 | 782.404 |
| merge-54x1-w32/original-x | 20918.261 | 18017.129 | 3.821242 | 13647.5 | 530.285 | 798.671 |

## Supplied M5 CPU and Metal references

These are historical cross-machine references, not a same-machine or newly remeasured M5 comparison. All eight reference shapes are measured anew here. Every executable is shown to avoid selecting a favorable copy. Merge-54x1 has no supplied reference and is included in the latency table only.

| Shape | Executable | Local ms | M5 CPU ms | M5 Metal ms | CPU/local | Metal/local |
|---|---|---:|---:|---:|---:|---:|
| 1x2 | original-x | 8.385567 | 37 | 24 | 4.412× | 2.862× |
| 1x2 | copy-001-x | 8.452564 | 37 | 24 | 4.377× | 2.839× |
| 1x2 | copy-002-x | 8.415173 | 37 | 24 | 4.397× | 2.852× |
| 2x2 | original-x | 11.603096 | 59 | 30 | 5.085× | 2.586× |
| 2x2 | copy-001-x | 11.622474 | 59 | 30 | 5.076× | 2.581× |
| 2x2 | copy-002-x | 11.545287 | 59 | 30 | 5.110× | 2.598× |
| 4x2 | original-x | 16.565043 | 109 | 48 | 6.580× | 2.898× |
| 4x2 | copy-001-x | 16.381059 | 109 | 48 | 6.654× | 2.930× |
| 4x2 | copy-002-x | 16.447609 | 109 | 48 | 6.627× | 2.918× |
| 8x2 | original-x | 24.689731 | 189 | 74 | 7.655× | 2.997× |
| 8x2 | copy-001-x | 24.867072 | 189 | 74 | 7.600× | 2.976× |
| 8x2 | copy-002-x | 24.578090 | 189 | 74 | 7.690× | 3.011× |
| 12x2 | original-x | 34.435066 | 284 | 101 | 8.247× | 2.933× |
| 12x2 | copy-001-x | 34.512299 | 284 | 101 | 8.229× | 2.926× |
| 12x2 | copy-002-x | 34.417323 | 284 | 101 | 8.252× | 2.935× |
| 16x2 | original-x | 40.662153 | 335 | 124 | 8.239× | 3.050× |
| 16x2 | copy-001-x | 40.832529 | 335 | 124 | 8.204× | 3.037× |
| 16x2 | copy-002-x | 41.074344 | 335 | 124 | 8.156× | 3.019× |
| 24x2 | original-x | 66.729473 | 528 | 176 | 7.913× | 2.638× |
| 24x2 | copy-001-x | 67.094938 | 528 | 176 | 7.869× | 2.623× |
| 24x2 | copy-002-x | 67.512809 | 528 | 176 | 7.821× | 2.607× |
| merge-24x1 | original-x | 66.301464 | 507 | 175 | 7.647× | 2.639× |
| merge-24x1 | copy-001-x | 66.679899 | 507 | 175 | 7.603× | 2.624× |
| merge-24x1 | copy-002-x | 66.068862 | 507 | 175 | 7.674× | 2.649× |

The original CPU10x objective remains **active and unmet**: 0/8 rows reach the target using the original accepted executable in this capture. Primary requires at most 3.7 ms. No component-only or favorable-copy result changes completion requirements. The prior accepted FFT engineering exception and its false full-performance/qualified-scope gates remain unchanged.

## Provenance and verification

The accepted executable SHA256 is `14da8231f5797db25d3f45e3570a73c38141bfb957295d2548b0e457676e612a`. `original-x` is a hardlink to its original inode. `copy-001-x` and `copy-002-x` are explicit read/write new files, not reflinks or hardlinks. All names are ten bytes in one directory. Identities and full contents are frozen before any proof capture. Complete native library receipts are retained; no native library or SDK is rebuilt. The offline raw/Euler converters link the accepted native libraries; the study/report helpers are Rust.

The old temporary all-shape images were absent. Nine CDN key artifacts are checked against the repository lockfile before parsing; regenerated raw and Euler images all match the historical production hashes, including the surviving primary. Preparation completes before proof capture. Existing accepted cache files are frozen after preflight. There is no new persistent key table, point recoding cache, expanded execution plan, or runtime memory-budget policy. The 172 accepted source files remain byte-exact, and prior unchanged-executable native/portable qualification is inherited rather than counted as new tests.

27 preflights, 450 latency processes, and 54 counter processes complete. All **28,044 complete SDK requests verify internally**. All **531 fresh 164-byte proof exports independently pass Gnark**, with historical expected image/public binding, member hashes, and uniqueness against 13,181 prior proof identities. The independent ledger is **13,712**. The 1,062 proof/public members are not counted as 1,062 proofs; three copied binaries are not three algorithm implementations. All independent verification runs after capture.

No owned build, helper, hash/archive job, or second benchmark overlaps proof capture. Unrelated host processes are left untouched; this is not an exclusive-host claim. Host snapshots and run receipts are retained. There is no new AWS/Aeglos, mobile/TEE, memory-pressure/swap, minimum-RAM, or 250/4999 qualification. Measured RSS is a process observation, not a minimum RAM bound. No global ASLR, kernel, or cache-policy setting changes.

[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [file identities](artifacts/identities.json), [image restoration](artifacts/restore/result.json), [all capture and verifier receipts](artifacts/runs/result.json), [proof inventory](artifacts/proof-inventory.json). The binary/key/image payloads remain local with pinned hashes.

## Findings and next action

**The large component-level difference does not reproduce as a large SDK proof difference in this experiment.** Primary32 original is 8.385567 ms; the two fixed fresh files are 8.452564 and 8.415173 ms. Their paired ratios are 1.006908 [0.998739, 1.015449] and 1.006182 [0.994925, 1.011635]. Original median peak RSS is 130.25 MiB, observed maximum 132.50 MiB. Merge54 original is 163.928034 ms and copies 164.817753/163.746601 ms; paired ratios 0.999669 [0.992750, 1.011128] and 0.996730 [0.989699, 1.010416]. This is a completed new all-shape measurement, not the historical 8.324853/161.341835 ms observation.

All twenty paired intervals span parity. Positive point estimates are retained: primary16 copy002 is about 1.92% lower in paired latency, bounds about 5.60% lower to 0.063% higher; its interval remains inconclusive. The observations neither establish exact equivalence nor prove that executable backing can never matter. They do not support transferring the earlier roughly 32% solo-component wall-time reduction to this accepted prover or selecting a favorable copy as a performance fix.

The counter association also differs from the pathological component control. Original primary32 records about 0.610 M L1I misses/request, versus 1.299 M and 0.808 M for fresh copies; fetch cycles are 869.0/869.9/865.3 M and instructions 1438.8/1440.4/1439.9 M. Separate stall-panel frontend counts are 61.14/63.89/61.27 M. Merge54 copy001 has about 8.00 M L1I misses versus original 3.82 M, yet the full-proof interval spans parity. These three-block aggregate counters are descriptive: a larger L1I miss count alone does not establish a large proof-latency penalty. No physical-page or exclusive causal explanation is claimed.

Original primary16 retires about 1397.1 M instructions and 602.5 M fetch cycles, versus primary32 1438.8 M and 869.0 M. The 32-worker latency remains lower (8.386 versus 10.744 ms) while aggregate CPU work is higher. This preserves the distinction between reducing CPU work and reducing elapsed single-proof latency; it does not identify a sole cause of the scaling cost.

The next concrete implementation experiment is the first **full-SDK integration of the borrowed G2 pending queue**, not another unchanged component or executable-copy rerun. Its smaller proof-local queue and omitted unused buffers have a measured resource benefit; the component's positive16-worker estimate and failed performance gate remain intact and do not establish proof-level improvement. Link the exact private candidate into the real SDK, compare its complete verified requests against accepted controls with retained executable identities, and measure memory and all-shape nonregression before any adoption. Preserve fallback behavior and avoid persistent key/point tables. This diagnostic closes the large full-proof backing-speed hypothesis as a current optimization direction; it does not retroactively promote or discard earlier arithmetic candidates.

All eight CPU10x targets remain unmet in this capture. Production is unchanged, with no new AWS/Aeglos or minimum-RAM claim.
