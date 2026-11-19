# Folded proof timelines on C7a

Proof-local clocks share a common origin from entry into the solver. They measure the solver, gather, quotient FFT, G1 A, K, H, G2 B2, and public correction. Each clocked process has a matched unclocked process with the same binary and schedule. The released build provides an additional unclocked control. One c7a.24xlarge uses 64 main and 32 frontend physical cores. 4×2 uses four matched blocks per input and 8×2 uses six. The temporary instance and storage were removed.

Each process primes three proofs and times thirteen. All 140 exported proofs independently verified. The spans include scheduling and work stolen by their workers, and concurrent spans can overlap. The last column measures how long each query continued past the last of the other recorded phases. This describes the observed schedule and excludes potential changes from moving or shortening a query. CPU and Metal scorecard entries use verified full-proof timings. [M5 comparison](../2026-11-19-folded-query-overlap-c7a820/README.md#eight-supplied-m5-reference-timings)

On 8×2's original request, B2 finished last in 50 of 78 clocked ordinary proofs. The early-B2 arm overlapped B2 with the quotient for 3.810 ms and its quotient span was 3.810 ms versus 2.721 ms in the ordinary schedule.

| Shape | Input | Schedule | Full proof ms | Paired clock effect | Solve | Gather | Quotient | A | K | H | B2 | B2 overlap with FFT | Sole A / K / H / B2 |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 4×2 | catalog | ordinary | 15.414 | +1.18% | 3.542 | 0.958 | 2.038 | 5.400 | 7.109 | 5.782 | 7.144 | 0.000 | 0.065 / 0.150 / 0.002 / 0.147 |
| 4×2 | catalog | early-b2 | 15.164 | +0.35% | 3.566 | 0.973 | 2.114 | 5.947 | 6.193 | 6.189 | 8.483 | 2.114 | 0.022 / 0.012 / 0.022 / 0.072 |
| 4×2 | catalog | early-ab2 | 15.960 | +0.15% | 3.551 | 0.978 | 2.489 | 8.787 | 6.582 | 5.568 | 8.764 | 2.482 | 0.333 / 0.083 / 0.011 / 0.165 |
| 4×2 | heldout | ordinary | 15.459 | +0.15% | 3.529 | 0.985 | 2.042 | 4.847 | 7.121 | 5.370 | 7.086 | 0.000 | 0.018 / 0.176 / 0.002 / 0.164 |
| 4×2 | heldout | early-b2 | 15.169 | +0.50% | 3.566 | 0.988 | 2.135 | 6.323 | 6.080 | 6.377 | 8.370 | 2.135 | 0.065 / 0.023 / 0.045 / 0.092 |
| 4×2 | heldout | early-ab2 | 15.665 | -0.02% | 3.570 | 0.981 | 2.394 | 8.347 | 6.591 | 5.866 | 8.410 | 2.388 | 0.217 / 0.124 / 0.002 / 0.011 |
| 8×2 | catalog | ordinary | 20.173 | +0.50% | 4.638 | 0.893 | 2.721 | 8.071 | 9.308 | 9.244 | 9.757 | 0.000 | 0.001 / 0.018 / 0.020 / 0.186 |
| 8×2 | catalog | early-b2 | 22.129 | -0.98% | 4.607 | 0.901 | 3.810 | 9.034 | 9.126 | 9.951 | 11.649 | 3.810 | 0.327 / 0.000 / 0.253 / 0.079 |
| 8×2 | catalog | early-ab2 | 22.243 | -0.07% | 4.589 | 0.901 | 4.285 | 10.915 | 9.172 | 9.538 | 12.634 | 4.281 | 0.224 / 0.024 / 0.308 / 0.159 |
| 8×2 | heldout | ordinary | 20.435 | +0.17% | 4.639 | 0.894 | 2.754 | 8.599 | 9.446 | 9.528 | 9.799 | 0.000 | 0.153 / 0.000 / 0.019 / 0.154 |
| 8×2 | heldout | early-b2 | 22.289 | -0.30% | 4.632 | 0.938 | 3.733 | 9.478 | 9.039 | 9.956 | 13.269 | 3.733 | 0.205 / 0.016 / 0.226 / 0.140 |
| 8×2 | heldout | early-ab2 | 22.529 | +2.11% | 4.642 | 0.925 | 4.248 | 10.695 | 9.309 | 9.454 | 12.545 | 4.244 | 0.144 / 0.111 / 0.181 / 0.126 |

All durations are milliseconds. [Samples and clocks](artifacts/rows.json.gz) · [Summary](artifacts/summary.json) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)
