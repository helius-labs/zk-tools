# Dedicated raw G1 A and exclusive G2 grouping as per-key SDK options

The new options expose the dedicated G1 A route and the exclusive G2 grouping bypass to the Rust prover. The dedicated route runs raw A on the attached smaller pool for the native fused K/H path when an additional workspace reservation succeeds. Every proof computes its curve results from its own witness. Caller-selected pool placement controls CPU affinity.

The combined API mode with 80 main and 16 dedicated workers had a lower median than prepared Aeglos on 6/6 transactions and won 34/36 paired blocks. Its original transaction measured 7.953 ms, or 4.65× against the supplied M5 CPU and 3.02× against Metal. The two new fixtures in this confirmation are v27 and v28. The other four appeared in the dedicated-pool discovery experiment.

All cells use one c7a.24xlarge and access to the same 96 physical cores. Prepared Aeglos reported cache hits on all timed requests. Six blocks per input used three primes and fifteen timed calls. The experimental predecessor supplies a matched control at the same split width. Eight verification-only captures recorded raw G1 calls. With the route admitted all four observed calls used the dedicated cores. The zero-budget captures contain raw calls on both pools because memory gating changes the proving path. Eight additional Ryzen captures compare enabled and disabled settings under the same zero cap. Their main/frontend entry pattern matched. The native core test suite passed 157 tests, and the generic SDK example set compiled. An independent verifier checked all 304 API-panel exports. The instance and temporary resources were removed.

| Input | Configuration | Median ms | Paired vs full control | Paired vs same-width control | Paired vs Aeglos | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| catalog | release | 8.036 | -0.40% (4/6) | -0.40% (4/6) | -2.06% (5/6) | 43.9 |
| catalog | off | 8.046 | +0.00% (0/6) | +0.00% (0/6) | -1.82% (5/6) | 45.1 |
| catalog | bypass | 7.927 | -1.60% (6/6) | -1.60% (6/6) | -3.57% (5/6) | 43.8 |
| catalog | 80-off | 8.775 | +8.92% (0/6) | +0.00% (0/6) | +6.68% (0/6) | 39.3 |
| catalog | 80-api | 7.905 | -1.77% (6/6) | -9.83% (6/6) | -3.64% (5/6) | 40.8 |
| catalog | 80-api-bypass | 7.953 | -1.34% (5/6) | -9.41% (5/6) | -3.07% (4/6) | 43.1 |
| catalog | 80-trial-bypass | 7.779 | -3.56% (6/6) | -11.35% (6/6) | -5.29% (6/6) | 45.3 |
| catalog | aeglos | 8.224 | +1.86% (1/6) | +1.86% (1/6) | +0.00% (0/6) | 459.4 |
| heldout | release | 11.148 | +0.16% (2/6) | +0.16% (2/6) | +30.60% (0/6) | 41.6 |
| heldout | off | 11.142 | +0.00% (0/6) | +0.00% (0/6) | +30.23% (0/6) | 42.3 |
| heldout | bypass | 10.375 | -6.80% (6/6) | -6.80% (6/6) | +22.18% (0/6) | 39.3 |
| heldout | 80-off | 11.172 | +0.55% (2/6) | +0.00% (0/6) | +31.07% (0/6) | 35.6 |
| heldout | 80-api | 7.878 | -29.22% (6/6) | -29.50% (6/6) | -7.67% (6/6) | 41.4 |
| heldout | 80-api-bypass | 7.888 | -29.13% (6/6) | -29.45% (6/6) | -7.55% (6/6) | 41.7 |
| heldout | 80-trial-bypass | 7.720 | -30.74% (6/6) | -30.92% (6/6) | -9.65% (6/6) | 40.7 |
| heldout | aeglos | 8.541 | -23.21% (6/6) | -23.21% (6/6) | +0.00% (0/6) | 463.8 |
| v25 | release | 11.126 | +0.39% (2/6) | +0.39% (2/6) | +35.10% (0/6) | 44.5 |
| v25 | off | 11.102 | +0.00% (0/6) | +0.00% (0/6) | +35.46% (0/6) | 42.4 |
| v25 | bypass | 10.344 | -6.78% (6/6) | -6.78% (6/6) | +24.43% (0/6) | 40.0 |
| v25 | 80-off | 11.158 | +0.62% (2/6) | +0.00% (0/6) | +35.17% (0/6) | 37.8 |
| v25 | 80-api | 7.886 | -29.27% (6/6) | -29.32% (6/6) | -4.31% (6/6) | 37.8 |
| v25 | 80-api-bypass | 7.839 | -29.50% (6/6) | -29.75% (6/6) | -5.33% (6/6) | 39.3 |
| v25 | 80-trial-bypass | 7.732 | -30.40% (6/6) | -30.70% (6/6) | -6.94% (6/6) | 43.0 |
| v25 | aeglos | 8.239 | -26.17% (6/6) | -26.17% (6/6) | +0.00% (0/6) | 458.6 |
| v26 | release | 11.108 | +0.02% (2/6) | +0.02% (2/6) | +33.12% (0/6) | 43.2 |
| v26 | off | 11.106 | +0.00% (0/6) | +0.00% (0/6) | +33.15% (0/6) | 42.4 |
| v26 | bypass | 10.358 | -7.17% (6/6) | -7.17% (6/6) | +24.12% (0/6) | 41.4 |
| v26 | 80-off | 11.213 | +0.92% (1/6) | +0.00% (0/6) | +34.76% (0/6) | 35.6 |
| v26 | 80-api | 7.867 | -29.16% (6/6) | -29.72% (6/6) | -5.16% (6/6) | 43.1 |
| v26 | 80-api-bypass | 7.864 | -29.37% (6/6) | -29.95% (6/6) | -5.35% (6/6) | 41.5 |
| v26 | 80-trial-bypass | 7.736 | -30.47% (6/6) | -31.12% (6/6) | -6.74% (6/6) | 41.5 |
| v26 | aeglos | 8.336 | -24.89% (6/6) | -24.89% (6/6) | +0.00% (0/6) | 460.0 |
| v27 | release | 11.128 | -0.59% (4/6) | -0.59% (4/6) | +32.98% (0/6) | 43.0 |
| v27 | off | 11.174 | +0.00% (0/6) | +0.00% (0/6) | +35.09% (0/6) | 40.9 |
| v27 | bypass | 10.425 | -6.73% (6/6) | -6.73% (6/6) | +24.64% (0/6) | 43.7 |
| v27 | 80-off | 11.156 | +0.19% (2/6) | +0.00% (0/6) | +33.82% (0/6) | 36.3 |
| v27 | 80-api | 7.879 | -29.46% (6/6) | -29.38% (6/6) | -5.33% (6/6) | 40.8 |
| v27 | 80-api-bypass | 7.877 | -29.69% (6/6) | -29.38% (6/6) | -5.47% (6/6) | 40.7 |
| v27 | 80-trial-bypass | 7.749 | -30.85% (6/6) | -30.67% (6/6) | -6.97% (6/6) | 41.5 |
| v27 | aeglos | 8.329 | -25.97% (6/6) | -25.97% (6/6) | +0.00% (0/6) | 460.6 |
| v28 | release | 11.173 | +0.32% (2/6) | +0.32% (2/6) | +36.42% (0/6) | 42.2 |
| v28 | off | 11.138 | +0.00% (0/6) | +0.00% (0/6) | +36.16% (0/6) | 39.3 |
| v28 | bypass | 10.326 | -7.46% (6/6) | -7.46% (6/6) | +25.81% (0/6) | 40.0 |
| v28 | 80-off | 11.171 | -0.00% (3/6) | +0.00% (0/6) | +36.16% (0/6) | 37.2 |
| v28 | 80-api | 7.880 | -29.29% (6/6) | -29.50% (6/6) | -3.60% (6/6) | 39.3 |
| v28 | 80-api-bypass | 7.840 | -29.63% (6/6) | -29.89% (6/6) | -4.31% (6/6) | 38.5 |
| v28 | 80-trial-bypass | 7.705 | -30.91% (6/6) | -31.00% (6/6) | -5.74% (6/6) | 41.5 |
| v28 | aeglos | 8.208 | -26.56% (6/6) | -26.56% (6/6) | +0.00% (0/6) | 459.0 |

| Input | Route test | Observed raw G1 entries | CPU IDs |
|---|---|---:|---|
| catalog | 80-api-bypass | 4 | 80 |
| catalog | 80-off | 4 | 0, 64 |
| catalog | 80-api-denied | 8 | 0, 80 |
| catalog | 80-api-detached | 4 | 0, 33 |
| heldout | 80-api-bypass | 4 | 80 |
| heldout | 80-off | 4 | 0, 34 |
| heldout | 80-api-denied | 8 | 0, 80 |
| heldout | 80-api-detached | 4 | 0, 10 |

[Measurements](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Proof verification](artifacts/verification.json) · [All eight M5 references](../2026-11-19-equal-core-scorecard707/README.md)


## C7a catalog and Apple references

The two-input comparison now favors Zopro by median on **30/41** shapes. Zopro won every matched block on **27/41** and had lower maximum observed RSS on **41/41**. Forty rows come from the previous paired catalog. The 1×2 row uses the API confirmation on the original and first held-out transactions. The other four transactions above measure sensitivity for that same key. [Full catalog CSV](artifacts/full-catalog.csv)

Using the catalog settings and 96 physical C7a cores, **1/8** shapes reached 10× the supplied M5 CPU time. The M5 CPU and Metal comparisons use the original request for each shape. Other host widths and the local 12×2 experiment have their own reports.

| Shape | C7a ms | M5 CPU / Metal ms | Speedup vs CPU / Metal | 10× M5 CPU target ms |
|---|---:|---:|---:|---:|
| 1×2 | 7.953 | 37 / 24 | 4.65× / 3.02× | 3.7 |
| 2×2 | 12.513 | 59 / 30 | 4.72× / 2.40× | 5.9 |
| 4×2 | 17.210 | 109 / 48 | 6.33× / 2.79× | 10.9 |
| 8×2 | 25.794 | 189 / 74 | 7.33× / 2.87× | 18.9 |
| 12×2 | 32.603 | 284 / 101 | 8.71× / 3.10× | 28.4 |
| 16×2 | 34.278 | 335 / 124 | 9.77× / 3.62× | 33.5 |
| 24×2 | 47.046 | 528 / 176 | 11.22× / 3.74× | 52.8 |
| merge-24×1 | 51.427 | 507 / 175 | 9.86× / 3.40× | 50.7 |
