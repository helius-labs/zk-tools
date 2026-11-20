# Preparing small B2 queries during the quotient FFT

After the solver and canonical gather, the candidate moves B2 classification, point grouping and endomorphism decomposition into the quotient interval. The FFT uses the existing frontend pool. B2 preparation uses the main pool and keeps only proof-local data. The B2 bucket work starts in the usual curve phase after the FFT and preparation have both finished. The serial option performs the scalar decomposition on one worker. The wait control uses the same staging and runs preparation after the FFT. The ordinary control retains the original proof schedule.

The path reserves overlapping FFT workspace. The existing grouping reservation stays live through the prepared point sum. A missing pool, unsupported shape or denied workspace uses the ordinary schedule. The native generator test checks grouped and raw computation, including the fallback for very few groups. All 160 proof exports independently verified, including four route probes at zero optional workspace.

Both shapes use 24 main and eight frontend logical CPUs on Ryzen. All 2×2 arms use medium fused K/H when admitted. Seven counterbalanced process blocks per input prime three proofs and time thirteen. Route intervals come from separate instrumented processes. An early mode advances with a lower paired median and at least five wins against release, ordinary and wait on both inputs.

Early preparation did not consistently improve the release, same-build ordinary and staged wait controls on either shape. The SDK default is unchanged.

| Shape | Input | Mode | Proof ms | Paired vs ordinary | Paired vs release | Paired vs wait | CPU ms / proof | Minor faults / proof | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | release | 6.847 | -0.46% (6/7) | +0.00% (0/7) | -1.24% (6/7) | 133.6 | 100 | 58.2 |
| 1×2 | catalog | ordinary | 6.896 | +0.00% (0/7) | +0.47% (1/7) | -0.24% (6/7) | 134.0 | 125 | 58.5 |
| 1×2 | catalog | wait | 6.942 | +0.24% (1/7) | +1.25% (1/7) | +0.00% (0/7) | 132.9 | 207 | 65.8 |
| 1×2 | catalog | early | 6.884 | -0.23% (5/7) | +0.33% (2/7) | -0.86% (5/7) | 133.9 | 274 | 66.8 |
| 1×2 | catalog | early-serial | 6.880 | -0.29% (4/7) | +0.25% (2/7) | -1.33% (5/7) | 132.3 | 235 | 64.2 |
| 1×2 | heldout | release | 6.766 | +0.39% (2/7) | +0.00% (0/7) | -1.11% (6/7) | 129.3 | 117 | 58.5 |
| 1×2 | heldout | ordinary | 6.749 | +0.00% (0/7) | -0.38% (5/7) | -1.50% (7/7) | 129.2 | 121 | 57.8 |
| 1×2 | heldout | wait | 6.824 | +1.52% (0/7) | +1.13% (1/7) | +0.00% (0/7) | 128.6 | 235 | 63.8 |
| 1×2 | heldout | early | 6.836 | +1.70% (1/7) | +0.72% (0/7) | +0.04% (3/7) | 130.0 | 241 | 64.2 |
| 1×2 | heldout | early-serial | 6.800 | +0.76% (0/7) | +0.56% (3/7) | -0.35% (5/7) | 127.9 | 238 | 63.0 |
| 2×2 | catalog | release | 9.467 | +0.88% (2/7) | +0.00% (0/7) | -9.63% (7/7) | 180.9 | 265 | 97.5 |
| 2×2 | catalog | ordinary | 9.441 | +0.00% (0/7) | -0.88% (5/7) | -10.06% (6/7) | 181.4 | 219 | 99.0 |
| 2×2 | catalog | wait | 10.527 | +11.18% (1/7) | +10.66% (0/7) | +0.00% (0/7) | 183.5 | 440 | 108.5 |
| 2×2 | catalog | early | 9.607 | +1.60% (1/7) | +2.13% (1/7) | -8.57% (7/7) | 184.5 | 509 | 114.8 |
| 2×2 | catalog | early-serial | 9.701 | +2.19% (1/7) | +2.48% (0/7) | -7.92% (6/7) | 184.3 | 486 | 117.5 |
| 2×2 | heldout | release | 9.520 | +0.64% (2/7) | +0.00% (0/7) | -10.21% (7/7) | 181.9 | 308 | 98.8 |
| 2×2 | heldout | ordinary | 9.469 | +0.00% (0/7) | -0.63% (5/7) | -10.84% (7/7) | 181.8 | 183 | 97.8 |
| 2×2 | heldout | wait | 10.574 | +12.16% (0/7) | +11.36% (0/7) | +0.00% (0/7) | 184.2 | 461 | 114.0 |
| 2×2 | heldout | early | 9.669 | +2.46% (1/7) | +1.81% (2/7) | -8.47% (7/7) | 184.9 | 543 | 113.8 |
| 2×2 | heldout | early-serial | 9.860 | +3.99% (1/7) | +3.79% (1/7) | -7.30% (7/7) | 185.1 | 537 | 118.8 |

| Shape | Input | Probe | FFT ms | B2 preparation ms | Overlap ms |
|---|---|---|---:|---:|---:|
| 1×2 | catalog | wait | 0.558 | 0.049 | 0.000 |
| 1×2 | catalog | early | 0.654 | 0.059 | 0.059 |
| 1×2 | catalog | early-serial | 0.579 | 0.265 | 0.265 |
| 1×2 | heldout | wait | 0.578 | 0.048 | 0.000 |
| 1×2 | heldout | early | 0.606 | 0.057 | 0.057 |
| 1×2 | heldout | early-serial | 0.664 | 0.274 | 0.274 |
| 2×2 | catalog | wait | 1.312 | 0.941 | 0.000 |
| 2×2 | catalog | early | 1.465 | 1.184 | 1.184 |
| 2×2 | catalog | early-serial | 1.504 | 1.280 | 1.280 |
| 2×2 | heldout | wait | 1.311 | 0.919 | 0.000 |
| 2×2 | heldout | early | 1.622 | 1.211 | 1.211 |
| 2×2 | heldout | early-serial | 1.545 | 1.348 | 1.348 |

[Source](artifacts/source.patch) · [Summary](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)
