# Locating the G2 window tail at 64 and 96 cores

The experimental Zopro SDK and retained Aeglos ran on the same C7a host. Each main process timed 151 sequential warm 1×2 proofs after three primes. Three rotated processes contributed to each median. Zopro limited witness and FFT work to 32 workers.

At 64 cores, the last fill through last window weighting averaged **0.422 ms on fast proofs and 1.400 ms on slow proofs**. After every window had finished weighting, collection took 0.043 vs 0.264 ms, and the final serial fold took 0.069 vs 0.067 ms. Of 133 slow proofs, the largest delay was before the first fill on 45, between the last fill and last weighting on 43, and after G2 returned on 32. The fill span itself averaged 3.265 vs 3.262 ms. This points to three distinct places to investigate and rules out faster final folding as a material fix for these outliers.

Median trace/plain was 1.031 at both widths. Fast and slow fractions stayed within ten percentage points of plain in all three 64-core blocks and two of three at 96 cores.

| Physical cores | Arm | Median ms | CPU ms/proof | Peak MiB | M5 CPU / Metal |
|---:|---|---:|---:|---:|---:|
| 64 | plain | 9.309 | 310 | 77 | 3.97× / 2.58× |
| 64 | trace | 9.608 | 315 | 76 | 3.85× / 2.50× |
| 64 | aeglos | 8.664 | 428 | 442 | 4.27× / 2.77× |
| 96 | plain | 10.063 | 452 | 67 | 3.68× / 2.38× |
| 96 | trace | 10.376 | 461 | 69 | 3.57× / 2.31× |
| 96 | aeglos | 7.787 | 515 | 481 | 4.75× / 3.08× |

Each of the 906 timed Zopro traces resolved 22 partition fills, eleven markers immediately after per-window weighting, and one marker on the G2 coordinator immediately after the parallel window collection. The intervals telescope to the full measured request. The last column counts slow requests where that interval has the largest excess over the same block’s median fast duration. Fast means under 10 ms and slow means at least 11 ms.

| Cores | Interval | Fast mean ms | Slow mean ms | Difference ms | Largest excess count |
|---:|---|---:|---:|---:|---:|
| 64 | to_solver | 0.635 | 0.638 | +0.003 | 0 |
| 64 | solve | 1.986 | 1.959 | -0.027 | 0 |
| 64 | prepare | 0.245 | 0.245 | +0.000 | 0 |
| 64 | fft_check | 0.881 | 0.883 | +0.002 | 0 |
| 64 | to_g2 | 0.099 | 0.099 | +0.001 | 0 |
| 64 | to_fill | 0.114 | 1.167 | +1.054 | 45 |
| 64 | fill_span | 3.265 | 3.262 | -0.003 | 3 |
| 64 | fill_to_weight | 0.422 | 1.400 | +0.977 | 43 |
| 64 | weight_to_collect | 0.043 | 0.264 | +0.221 | 10 |
| 64 | collect_to_raw | 0.069 | 0.067 | -0.002 | 0 |
| 64 | after_g2 | 0.042 | 0.802 | +0.760 | 32 |
| 64 | sdk_tail | 0.882 | 0.871 | -0.012 | 0 |
| 64 | outside | 0.004 | 0.004 | +0.000 | 0 |
| 96 | to_solver | 0.818 | 0.829 | +0.011 | 0 |
| 96 | solve | 2.069 | 2.090 | +0.021 | 0 |
| 96 | prepare | 0.247 | 0.242 | -0.005 | 0 |
| 96 | fft_check | 0.942 | 0.937 | -0.005 | 0 |
| 96 | to_g2 | 0.192 | 0.212 | +0.021 | 0 |
| 96 | to_fill | 0.177 | 1.578 | +1.401 | 57 |
| 96 | fill_span | 3.418 | 3.566 | +0.148 | 4 |
| 96 | fill_to_weight | 0.437 | 0.956 | +0.518 | 21 |
| 96 | weight_to_collect | 0.038 | 0.238 | +0.199 | 8 |
| 96 | collect_to_raw | 0.073 | 0.068 | -0.004 | 0 |
| 96 | after_g2 | 0.031 | 0.997 | +0.966 | 39 |
| 96 | sdk_tail | 0.920 | 0.906 | -0.014 | 1 |
| 96 | outside | 0.004 | 0.004 | +0.000 | 0 |

The interval from last fill through last weighting includes finishing the partition joins, merging buckets and weighting the remaining windows. From last weighting to collection return, the window results are already computed. From collection return to G2 return, the coordinator folds the window sums and releases their storage. Clock differences combine execution, waiting and descheduling.

[Block-level calibration](artifacts/blocks.json) · [Request series](artifacts/requests.json.gz) · [Protocol](artifacts/protocol.md) · [Probe offsets](artifacts/targets.tsv) · [Decoder](artifacts/trace.rs)

## How many windows remain on the long tail?

This post hoc check selects slow requests whose last-fill-to-last-weight interval exceeded the same block’s fast median by more than 1 ms. The isolated interval starts when all 22 fills and ten of eleven window weightings have returned. It ends when the eleventh weighting returns.

| Cores | Selected proofs | Entire interval, mean ms | Only last weighting outstanding, mean ms | Isolated interval >1 ms |
|---:|---:|---:|---:|---:|
| 64 | 43 | 3.393 | 1.724 | 24 |
| 96 | 21 | 3.476 | 1.777 | 12 |

A window can still be waiting for a partition join or bucket merge before entering its weighted reduction. The marker alone cannot distinguish that wait from time inside the reduction. [Per-request window completions](artifacts/window-completions.json.gz)

All 24 AWS exports passed Gnark verification on the instance and after retrieval. One additional local probe smoke proof also verified. The instance terminated, its volume query was empty, and the temporary bucket was removed. [Verification](artifacts/verification.json)

## Why the extra cores help Aeglos on this input

Aeglos reports times for its own witness, FFT, MSM and assembly phases. Each timing below is the median of three process means. The Zopro trace resolves intervals on the request timeline. Stage definitions differ across engines, so the change from 64 to 96 cores is shown within each engine.

| Engine | Interval | 64 cores, ms | 96 cores, ms | 96 minus 64, ms |
|---|---|---:|---:|---:|
| aeglos | witness_ms | 1.438 | 1.559 | +0.121 |
| aeglos | fft_ms | 0.803 | 0.879 | +0.075 |
| aeglos | msm_ms | 5.041 | 3.863 | -1.178 |
| aeglos | assembly_ms | 0.276 | 0.297 | +0.021 |
| aeglos | full | 8.664 | 7.787 | -0.878 |
| trace | solve | 1.978 | 2.075 | +0.097 |
| trace | fft_check | 0.877 | 0.942 | +0.065 |
| trace | to_fill | 0.424 | 0.617 | +0.193 |
| trace | fill_span | 3.269 | 3.478 | +0.208 |
| trace | fill_to_weight | 0.737 | 0.586 | -0.151 |
| trace | weight_to_collect | 0.119 | 0.083 | -0.036 |
| trace | collect_to_raw | 0.068 | 0.071 | +0.003 |
| trace | after_g2 | 0.284 | 0.318 | +0.034 |
| trace | full | 9.608 | 10.376 | +0.768 |
