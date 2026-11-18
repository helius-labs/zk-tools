# Whole-proof CPU samples at 96 C7a cores

The profiler marked only SDK entry and return. Each process used the exact 257 executable. One warm sequential proof was in flight at a time. A second unprofiled arm (sham) measures drift between repeated processes.

| Arm | Median ms | CPU ms | Peak MiB | Under 10 ms | At least 11 ms | M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|---:|
| plain | 10.144 | 457.0 | 76.8 | 763 | 288 | 3.65× / 2.37× |
| sham | 10.086 | 454.6 | 69.4 | 783 | 267 | 3.67× / 2.38× |
| trace | 10.539 | 466.1 | 71.0 | 694 | 341 | 3.51× / 2.28× |

Median traced/plain wall ratio was 1.0394. The fast and slow fractions were each within 10 percentage points of plain in 5/7 blocks. Sham/plain was 0.9928 with 7/7 matching blocks. The observer passed its prespecified calibration.

## Mean cycle samples per proof and share within each class

| Containing symbol | all (n=1057) | fast (n=694) | slow (n=341) |
|---|---:|---:|---:|
| Crossbeam epoch | 37.82 (41.8%) | 33.05 (38.7%) | 47.99 (47.2%) |
| Other symbols | 26.55 (29.3%) | 26.59 (31.1%) | 26.73 (26.3%) |
| G2 borrowed batch arithmetic | 9.29 (10.3%) | 9.37 (11.0%) | 9.18 (9.0%) |
| Other Rayon or deque symbols | 5.19 (5.7%) | 4.84 (5.7%) | 5.95 (5.8%) |
| K/H named code | 3.56 (3.9%) | 3.56 (4.2%) | 3.63 (3.6%) |
| FFT | 2.98 (3.3%) | 3.02 (3.5%) | 2.94 (2.9%) |
| Field inversion | 2.06 (2.3%) | 2.05 (2.4%) | 2.10 (2.1%) |
| G2 feed | 2.00 (2.2%) | 1.93 (2.3%) | 2.15 (2.1%) |
| Solver | 1.03 (1.1%) | 1.03 (1.2%) | 1.04 (1.0%) |

The slow group averaged 101.72 samples per proof versus 85.44 for the fast group. Epoch bookkeeping rose from 33.05 to 47.99 samples per proof, accounting for 14.94 of the 16.28 extra samples across all threads. It increased in every one of the seven blocks, by 12.33 to 17.36 samples per proof. The G2 batch arithmetic symbol was stable at 9.37 versus 9.18. This distinguishes the additional aggregate CPU work from the base arithmetic cost. It leaves open whether extended proof time lets workers search longer or the search itself delays completion.

One sample represents 16,000,019 user-mode cycles. Counts are normalized by the number of proofs in the class. Kernel and descheduled time are excluded. Categories identify the sampled instruction’s containing symbol and include inlined code. The residual category retains unknown addresses. Samples from all threads during a request are combined. These observations cannot attribute a sample to an open G2 callback, establish critical-path time, or predict the effect of suppressing epoch bookkeeping.

## Small-shape core scaling against Aeglos

These separate unprofiled processes used the same C7a instance and fixtures, 51 timed requests after three primes, seven rotated blocks per row. Zopro used the first contiguous physical cores and a frontend cap of 32. Aeglos used its retained 96-core setup. Times are medians of process means. Width selection from these five rows is exploratory.

| Shape | Engine | Cores | Median ms | Paired vs Zopro 96 | CPU ms | Peak MiB | M5 CPU / Metal | Blocks ≤9 ms |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | Zopro | 32 | 10.736 | +4.70% | 231 | 64 | 3.45× / 2.24× | 0/7 |
| 1x2 | Zopro | 48 | 9.754 | -4.81% | 271 | 77 | 3.79× / 2.46× | 0/7 |
| 1x2 | Zopro | 64 | 9.471 | -7.47% | 312 | 68 | 3.91× / 2.53× | 0/7 |
| 1x2 | Zopro | 80 | 9.801 | -3.57% | 376 | 65 | 3.78× / 2.45× | 0/7 |
| 1x2 | Zopro | 96 | 10.225 | +0.00% | 459 | 61 | 3.62× / 2.35× | 0/7 |
| 1x2 | Aeglos | 96 | 8.026 | -21.47% | 513 | 470 | 4.61× / 2.99× | 7/7 |
| 2x2 | Zopro | 32 | 14.445 | -5.72% | 314 | 276 | 4.08× / 2.08× | — |
| 2x2 | Zopro | 48 | 14.073 | -8.47% | 380 | 279 | 4.19× / 2.13× | — |
| 2x2 | Zopro | 64 | 14.213 | -7.46% | 465 | 288 | 4.15× / 2.11× | — |
| 2x2 | Zopro | 80 | 14.808 | -3.09% | 573 | 284 | 3.98× / 2.03× | — |
| 2x2 | Zopro | 96 | 15.359 | +0.00% | 697 | 278 | 3.84× / 1.95× | — |
| 2x2 | Aeglos | 96 | 11.564 | -24.56% | 826 | 816 | 5.10× / 2.59× | — |

[Width comparisons](artifacts/widths.json)

All 105 process exports passed pinned Gnark verification on the instance and after retrieval. The instance terminated, the EBS check found no matching volume, and the temporary S3 bucket was removed.

[Protocol](artifacts/protocol.md) · [Sample counts and receipts](artifacts/analysis.json.gz) · [Per-proof classes](artifacts/requests.json.gz)

## When the epoch samples accumulate

Every traced SDK interval lasted more than 8 ms, so the first four two-millisecond bins have the same exposure per proof in both classes. The last bin ends at each proof’s own return. The sample counts below are pooled class means from the same 1,057 requests.

| Time from SDK entry, ms | Fast epoch samples / proof | Slow epoch samples / proof | Slow minus fast |
|---|---:|---:|---:|
| 0–2 | 2.92 | 3.03 | +0.11 |
| 2–4 | 3.57 | 3.45 | -0.12 |
| 4–6 | 11.12 | 11.93 | +0.81 |
| 6–8 | 8.62 | 12.00 | +3.38 |
| 8 to return | 6.81 | 17.57 | +10.76 |

By 8 ms the mean difference was +4.18 samples per proof. From 8 ms to return it was +10.76. Fast proofs averaged 9.51 ms and slow proofs 12.71 ms. The later bin includes time after fast proofs have returned. The counts do not establish that sampling an epoch instruction delayed the proof. [Block and interval counts](artifacts/sample-time.json)
