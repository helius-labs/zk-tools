# G2 partition geometry after scalar grouping

When a B2 key has at least 65,536 points but the witness groups to 1,024–65,535 distinct nonzero coefficients, the optional grouped prover owns those coefficients. The integrated implementation splits their scalars while combining their point groups, then always uses two 12-bit partitions. This experiment changes that last step to four 11-bit or eight 10-bit partitions. The preparation policy reserves any additional workspace before the reduction. The established route handles insufficient headroom. Both new settings reuse the grouped coefficients and point sums.

All arms use the same small public coefficient file and packed FFT. Release is the earlier executable, integrated selects grouped G2 in the integrated executable, and off uses two partitions in the new executable. Four balanced blocks per input used 32 logical Ryzen workers, three warmups and eleven timed proofs. Request 67 is separately generated and differs from the catalog transaction. A separate 2×16 check set optional preparation to zero. All 323 exports verified. The dispatch artifact records which catalog keys entered the changed path.

Both settings reached the grouped kernel on all 11 catalog shapes. The four-way setting reduced median paired latency on 2/15 inputs against the same executable with the new geometry disabled. The eight-way setting did so on 0/15. These Ryzen results retain the existing default. The dedicated zero-budget check took the fallback.

| Shape | Request | Geometry | Release / integrated / off / enabled ms | vs off (wins) | vs integrated (wins) | Peak MiB release / enabled |
|---|---|---|---:|---:|---:|---:|
| 2x16 | catalog | 11x4 | 17.519 / 16.175 / 16.283 / 16.330 | +0.88% (1/4) | +1.10% (1/4) | 254.9 / 247.1 |
| 2x16 | catalog | 10x8 | 17.519 / 16.175 / 16.283 / 16.730 | +2.84% (0/4) | +2.60% (1/4) | 254.9 / 244.3 |
| 2x16 | 67 | 11x4 | 16.708 / 16.274 / 16.270 / 16.464 | +1.09% (0/4) | +1.44% (1/4) | 249.9 / 248.1 |
| 2x16 | 67 | 10x8 | 16.708 / 16.274 / 16.270 / 16.908 | +3.98% (0/4) | +4.19% (0/4) | 249.9 / 248.6 |
| 3x2 | catalog | 11x4 | 13.158 / 13.218 / 12.932 / 13.008 | +0.04% (2/4) | -1.67% (4/4) | 265.5 / 250.8 |
| 3x2 | catalog | 10x8 | 13.158 / 13.218 / 12.932 / 13.736 | +5.69% (1/4) | +3.92% (1/4) | 265.5 / 264.5 |
| 3x4 | catalog | 11x4 | 13.704 / 13.682 / 13.581 / 13.661 | -0.13% (2/4) | -0.08% (2/4) | 215.2 / 220.2 |
| 3x4 | catalog | 10x8 | 13.704 / 13.682 / 13.581 / 14.234 | +5.11% (0/4) | +4.01% (0/4) | 215.2 / 203.8 |
| 3x4 | 67 | 11x4 | 13.768 / 13.696 / 13.430 / 13.884 | +2.36% (1/4) | +1.14% (1/4) | 197.0 / 216.0 |
| 3x4 | 67 | 10x8 | 13.768 / 13.696 / 13.430 / 14.063 | +4.71% (0/4) | +2.42% (0/4) | 197.0 / 207.0 |
| 3x8 | catalog | 11x4 | 15.465 / 15.157 / 14.900 / 15.474 | +2.95% (1/4) | +2.09% (0/4) | 271.8 / 315.2 |
| 3x8 | catalog | 10x8 | 15.465 / 15.157 / 14.900 / 15.564 | +4.26% (1/4) | +3.27% (0/4) | 271.8 / 257.7 |
| 3x8 | 67 | 11x4 | 15.421 / 15.318 / 15.256 / 15.310 | +1.64% (1/4) | +0.56% (1/4) | 248.9 / 229.8 |
| 3x8 | 67 | 10x8 | 15.421 / 15.318 / 15.256 / 15.769 | +5.61% (0/4) | +4.22% (0/4) | 248.9 / 277.5 |
| 4x16 | catalog | 11x4 | 19.758 / 19.441 / 19.462 / 19.700 | +0.98% (2/4) | +0.42% (2/4) | 307.4 / 316.8 |
| 4x16 | catalog | 10x8 | 19.758 / 19.441 / 19.462 / 20.160 | +3.83% (0/4) | +3.47% (1/4) | 307.4 / 306.6 |
| 4x16 | 67 | 11x4 | 19.759 / 19.582 / 19.405 / 19.604 | +1.03% (0/4) | +0.02% (2/4) | 299.0 / 306.0 |
| 4x16 | 67 | 10x8 | 19.759 / 19.582 / 19.405 / 19.706 | +1.55% (0/4) | +0.58% (2/4) | 299.0 / 312.5 |
| 4x2 | catalog | 11x4 | 15.020 / 14.677 / 14.622 / 14.586 | +1.56% (1/4) | +0.30% (2/4) | 296.0 / 276.8 |
| 4x2 | catalog | 10x8 | 15.020 / 14.677 / 14.622 / 15.142 | +3.99% (1/4) | +3.40% (1/4) | 296.0 / 219.5 |
| 4x4 | catalog | 11x4 | 15.300 / 15.139 / 15.097 / 15.004 | -0.61% (3/4) | -0.71% (3/4) | 234.0 / 233.2 |
| 4x4 | catalog | 10x8 | 15.300 / 15.139 / 15.097 / 15.534 | +2.79% (1/4) | +3.72% (1/4) | 234.0 / 227.8 |
| 4x8 | catalog | 11x4 | 16.813 / 16.673 / 16.506 / 16.757 | +1.15% (0/4) | +0.58% (2/4) | 308.0 / 340.1 |
| 4x8 | catalog | 10x8 | 16.813 / 16.673 / 16.506 / 17.191 | +3.48% (0/4) | +2.41% (1/4) | 308.0 / 285.9 |
| 5x16 | catalog | 11x4 | 22.991 / 22.643 / 22.503 / 22.913 | +2.40% (1/4) | +0.93% (1/4) | 344.6 / 363.1 |
| 5x16 | catalog | 10x8 | 22.991 / 22.643 / 22.503 / 23.199 | +3.36% (0/4) | +2.66% (0/4) | 344.6 / 352.3 |
| 5x2 | catalog | 11x4 | 16.767 / 16.592 / 16.282 / 16.722 | +2.66% (0/4) | +1.30% (1/4) | 263.9 / 246.8 |
| 5x2 | catalog | 10x8 | 16.767 / 16.592 / 16.282 / 17.214 | +5.70% (0/4) | +3.29% (0/4) | 263.9 / 255.0 |
| 5x4 | catalog | 11x4 | 17.368 / 16.967 / 17.033 / 17.285 | +1.49% (1/4) | +1.97% (1/4) | 271.9 / 271.5 |
| 5x4 | catalog | 10x8 | 17.368 / 16.967 / 17.033 / 17.637 | +3.70% (0/4) | +3.95% (0/4) | 271.9 / 269.9 |

[Integrated grouped path](../2026-11-19-owned-phi661/README.md) · [Source](artifacts/source.patch) · [Dispatch checks](artifacts/routes.json) · [Timings](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
