# Confirming the paired Euler output path on C7a

The new build retains the paired emitter from the previous experiment. It evaluates the pointwise product during the final radix-32 column pass and writes it into the existing A allocation. The switch applies when the packed local Euler path is active. The enabled and disabled arms share a binary and use the same FFT options. The official arm uses the recorded SDK configuration, which uses a different FFT profile on 16×2. The same physical host provides 64 main and 32 frontend cores. The 8×2 panel also compares a 56+40 split, the full worker pool and prepared Aeglos.

Five matched blocks per input warm three requests and time thirteen. Eight routing checks record entry to the packed local Euler branch. The native FFT tests passed, generic examples compiled, and the verifier independently checked all 168 exports. The temporary instance and storage were removed.

| Shape | Enabled original / second ms | Paired changes vs same-build off | Paired changes vs official SDK |
|---|---:|---:|---:|
| 8x2 | 22.583 / 22.636 | -0.79% / -2.45% | -0.77% / -1.72% |
| 12x2 | 29.056 / 29.089 | -1.90% / -3.57% | -0.86% / -2.75% |
| 16x2 | 31.251 / 30.234 | +1.20% / -1.02% | +0.81% / -1.63% |
| merge-24x1 | 45.724 / 42.457 | -1.64% / -2.45% | -1.88% / -3.22% |

| Shape | Input | Settings | Full proof ms | Paired vs same-build off | Paired vs official | Peak MiB | M5 CPU / Metal |
|---|---|---|---:|---:|---:|---:|---:|
| 8x2 | catalog | aeglos | 30.292 | +33.62% (0/5) | +31.46% (0/5) | 2457.7 | 6.24× / 2.44× |
| 8x2 | catalog | full | 25.262 | +10.88% (0/5) | +10.25% (0/5) | 430.3 | 7.48× / 2.93× |
| 8x2 | catalog | off | 22.691 | +0.00% (0/5) | -0.31% (3/5) | 439.7 | 8.33× / 3.26× |
| 8x2 | catalog | off-56 | 22.589 | +0.00% (0/5) | -1.41% (5/5) | 444.2 | 8.37× / 3.28× |
| 8x2 | catalog | official | 22.804 | +0.31% (2/5) | +0.00% (0/5) | 438.1 | 8.29× / 3.25× |
| 8x2 | catalog | on | 22.583 | -0.79% (3/5) | -0.77% (5/5) | 424.3 | 8.37× / 3.28× |
| 8x2 | catalog | on-56 | 22.460 | -0.22% (3/5) | -1.51% (5/5) | 458.7 | 8.42× / 3.29× |
| 8x2 | heldout | aeglos | 30.324 | +31.12% (0/5) | +30.84% (0/5) | 2452.5 | — |
| 8x2 | heldout | full | 25.891 | +11.88% (0/5) | +12.42% (0/5) | 420.8 | — |
| 8x2 | heldout | off | 23.184 | +0.00% (0/5) | -0.02% (3/5) | 447.6 | — |
| 8x2 | heldout | off-56 | 22.705 | +0.00% (0/5) | -1.66% (5/5) | 431.9 | — |
| 8x2 | heldout | official | 23.189 | +0.02% (2/5) | +0.00% (0/5) | 453.1 | — |
| 8x2 | heldout | on | 22.636 | -2.45% (5/5) | -1.72% (5/5) | 444.8 | — |
| 8x2 | heldout | on-56 | 22.832 | +0.56% (1/5) | -0.93% (4/5) | 457.9 | — |
| 12x2 | catalog | off | 29.659 | +0.00% (0/5) | +1.28% (1/5) | 538.0 | 9.58× / 3.41× |
| 12x2 | catalog | official | 29.283 | -1.27% (4/5) | +0.00% (0/5) | 547.6 | 9.70× / 3.45× |
| 12x2 | catalog | on | 29.056 | -1.90% (5/5) | -0.86% (4/5) | 547.8 | 9.77× / 3.48× |
| 12x2 | heldout | off | 30.053 | +0.00% (0/5) | +1.93% (2/5) | 520.8 | — |
| 12x2 | heldout | official | 29.911 | -1.90% (3/5) | +0.00% (0/5) | 537.2 | — |
| 12x2 | heldout | on | 29.089 | -3.57% (5/5) | -2.75% (4/5) | 547.0 | — |
| 16x2 | catalog | off | 30.881 | +0.00% (0/5) | -0.08% (3/5) | 635.1 | 10.85× / 4.02× |
| 16x2 | catalog | official | 30.884 | +0.08% (2/5) | +0.00% (0/5) | 674.0 | 10.85× / 4.01× |
| 16x2 | catalog | on | 31.251 | +1.20% (1/5) | +0.81% (1/5) | 644.3 | 10.72× / 3.97× |
| 16x2 | heldout | off | 31.047 | +0.00% (0/5) | -1.80% (4/5) | 658.2 | — |
| 16x2 | heldout | official | 31.617 | +1.83% (1/5) | +0.00% (0/5) | 665.1 | — |
| 16x2 | heldout | on | 30.234 | -1.02% (4/5) | -1.63% (4/5) | 670.2 | — |
| merge-24x1 | catalog | off | 46.757 | +0.00% (0/5) | -0.42% (3/5) | 1064.2 | 10.84× / 3.74× |
| merge-24x1 | catalog | official | 46.279 | +0.42% (2/5) | +0.00% (0/5) | 1078.2 | 10.96× / 3.78× |
| merge-24x1 | catalog | on | 45.724 | -1.64% (3/5) | -1.88% (4/5) | 1050.5 | 11.09× / 3.83× |
| merge-24x1 | heldout | off | 43.448 | +0.00% (0/5) | +0.70% (2/5) | 1023.0 | — |
| merge-24x1 | heldout | official | 43.495 | -0.69% (3/5) | +0.00% (0/5) | 1022.6 | — |
| merge-24x1 | heldout | on | 42.457 | -2.45% (5/5) | -3.22% (4/5) | 1021.4 | — |

| Shape | Input | Packed local Euler entries |
|---|---|---:|
| 8x2 | catalog | 4/4 |
| 8x2 | heldout | 4/4 |
| 12x2 | catalog | 4/4 |
| 12x2 | heldout | 4/4 |
| 16x2 | catalog | 4/4 |
| 16x2 | heldout | 4/4 |
| merge-24x1 | catalog | 4/4 |
| merge-24x1 | heldout | 4/4 |

[Exact settings](artifacts/workflow.json.gz) · [Samples](artifacts/rows.json.gz) · [Source](artifacts/source.patch) · [Proof verification](artifacts/verification.json) · [Previous 12×2 and M5 measurements](../2026-11-19-indexed-confirm743/README.md)
