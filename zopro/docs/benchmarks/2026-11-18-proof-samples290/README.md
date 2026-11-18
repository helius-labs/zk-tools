# Whole-proof CPU samples at 32 Ryzen threads

The profiler marked only SDK entry and return. Each process used the exact 257 executable. One warm sequential proof was in flight at a time. A second unprofiled arm (sham) measures drift between repeated processes.

| Arm | Median ms | CPU ms | Peak MiB | Under 10 ms | At least 11 ms | M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|---:|
| plain | 7.535 | 148.7 | 65.5 | 356 | 0 | 4.91× / 3.18× |
| sham | 7.516 | 148.3 | 69.5 | 353 | 1 | 4.92× / 3.19× |
| trace | 7.601 | 149.4 | 65.5 | 355 | 0 | 4.87× / 3.16× |

Median traced/plain wall ratio was 1.0054. The fast and slow fractions were each within 10 percentage points of plain in 7/7 blocks. Sham/plain was 1.0019 with 7/7 matching blocks. The observer passed its prespecified calibration.

## Mean cycle samples per proof and share within each class

| Containing symbol | all (n=357) | fast (n=355) |
|---|---:|---:|
| Other symbols | 20.74 (49.1%) | 20.74 (49.1%) |
| G2 borrowed batch arithmetic | 8.02 (19.0%) | 8.02 (19.0%) |
| K/H named code | 3.86 (9.1%) | 3.87 (9.2%) |
| Crossbeam epoch | 3.35 (7.9%) | 3.34 (7.9%) |
| Field inversion | 2.52 (6.0%) | 2.52 (6.0%) |
| G2 feed | 1.51 (3.6%) | 1.52 (3.6%) |
| FFT | 1.17 (2.8%) | 1.17 (2.8%) |
| Solver | 0.64 (1.5%) | 0.64 (1.5%) |
| Other Rayon or deque symbols | 0.40 (0.9%) | 0.40 (0.9%) |

One sample represents 16,000,019 user-mode cycles. Counts are normalized by the number of proofs in the class. Kernel and descheduled time are excluded. Categories identify the sampled instruction’s containing symbol and include inlined code. The residual category retains unknown addresses. Samples from all threads during a request are combined. These observations cannot attribute a sample to an open G2 callback, establish critical-path time, or predict the effect of suppressing epoch bookkeeping.

All 21 process exports passed pinned Gnark verification locally.

[Protocol](artifacts/protocol.md) · [Sample counts and receipts](artifacts/analysis.json.gz) · [Per-proof classes](artifacts/requests.json.gz)
