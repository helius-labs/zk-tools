# Folded proof timelines on Ryzen

Proof-local clocks share a common origin from entry into the solver. They measure the solver, gather, quotient FFT, G1 A, K, H, G2 B2, and public correction. Each clocked process has a matched unclocked process with the same binary and schedule. The released build provides an additional unclocked control. Ryzen uses 24 main and eight frontend logical CPUs. Both shapes use four matched blocks per input.

Each process primes three proofs and times thirteen. All 112 exported proofs independently verified. The spans include scheduling and work stolen by their workers, and concurrent spans can overlap. The last column measures how long each query continued past the last of the other recorded phases. This describes the observed schedule and excludes potential changes from moving or shortening a query. CPU and Metal scorecard entries use verified full-proof timings. [M5 comparison](../2026-11-19-folded-query-overlap-c7a820/README.md#eight-supplied-m5-reference-timings)

On 8×2's original request, B2 finished last in 29 of 52 clocked ordinary proofs. The early-B2 arm overlapped B2 with the quotient for 8.223 ms and its quotient span was 8.341 ms versus 3.941 ms in the ordinary schedule.

| Shape | Input | Schedule | Full proof ms | Paired clock effect | Solve | Gather | Quotient | A | K | H | B2 | B2 overlap with FFT | Sole A / K / H / B2 |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 4×2 | catalog | ordinary | 14.523 | +0.98% | 2.341 | 0.352 | 2.179 | 7.449 | 7.991 | 8.268 | 8.439 | 0.000 | 0.000 / 0.000 / 0.025 / 0.059 |
| 4×2 | catalog | early-b2 | 14.700 | +0.33% | 2.359 | 0.337 | 4.548 | 5.192 | 6.276 | 5.281 | 9.350 | 4.548 | 0.000 / 0.538 / 0.000 / 0.072 |
| 4×2 | catalog | early-ab2 | 15.169 | +0.53% | 2.351 | 0.339 | 5.189 | 5.114 | 6.125 | 4.516 | 9.074 | 5.188 | 0.000 / 1.563 / 0.000 / 0.000 |
| 4×2 | heldout | ordinary | 14.367 | +0.25% | 2.345 | 0.347 | 2.152 | 7.489 | 8.066 | 8.263 | 8.429 | 0.000 | 0.000 / 0.004 / 0.017 / 0.049 |
| 4×2 | heldout | early-b2 | 14.761 | +0.31% | 2.348 | 0.338 | 4.629 | 5.178 | 6.291 | 5.333 | 9.998 | 4.629 | 0.000 / 0.539 / 0.000 / 0.055 |
| 4×2 | heldout | early-ab2 | 15.088 | -0.44% | 2.340 | 0.336 | 5.249 | 4.769 | 6.076 | 4.475 | 8.947 | 5.247 | 0.000 / 1.557 / 0.000 / 0.000 |
| 8×2 | catalog | ordinary | 22.674 | +0.91% | 2.683 | 0.623 | 3.941 | 13.375 | 12.860 | 14.055 | 14.035 | 0.000 | 0.042 / 0.000 / 0.023 / 0.037 |
| 8×2 | catalog | early-b2 | 22.726 | +0.54% | 2.669 | 0.640 | 8.341 | 9.098 | 9.603 | 9.098 | 11.213 | 8.223 | 0.000 / 0.329 / 0.000 / 0.023 |
| 8×2 | catalog | early-ab2 | 22.896 | -0.10% | 2.661 | 0.622 | 9.131 | 8.026 | 9.048 | 7.859 | 15.235 | 9.116 | 0.000 / 1.040 / 0.000 / 0.000 |
| 8×2 | heldout | ordinary | 22.710 | +1.03% | 2.660 | 0.616 | 3.893 | 13.083 | 12.738 | 14.018 | 14.194 | 0.000 | 0.035 / 0.000 / 0.018 / 0.154 |
| 8×2 | heldout | early-b2 | 22.816 | +0.70% | 2.634 | 0.624 | 8.274 | 9.099 | 9.802 | 9.137 | 10.196 | 8.208 | 0.000 / 0.610 / 0.000 / 0.000 |
| 8×2 | heldout | early-ab2 | 23.227 | +0.71% | 2.623 | 0.638 | 9.350 | 11.457 | 9.148 | 8.028 | 14.584 | 9.174 | 0.000 / 0.989 / 0.000 / 0.001 |

All durations are milliseconds. [Samples and clocks](artifacts/rows.json.gz) · [Summary](artifacts/summary.json) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)
