# Confirming 64 main workers on three M5 reference circuits

The released Rust SDK ran on a fresh c7a.24xlarge. The prespecified 64+32 placement used 64 main workers and a frontend on the other 32 physical cores. The 64-worker comparison omitted the frontend. The ordinary setting used 96 main workers. The attached frontend can handle solver and quotient preparation. The experiment used the ordinary G1 evaluator for all three reference circuits.

The 16×2 catalog result was **31.200 ms** with the split and **33.708 ms** at 96 main workers. The split result was **10.74×** the supplied M5 CPU speed and **3.97×** Metal, with 6/6 blocks at or below 33.5 ms. The second input ran at 31.483 ms with the same layout. The two other reference circuits use the same placement comparison.

Each reference arm ran in six paired blocks with three priming and thirteen timed requests. Prepared Aeglos used all 96 cores and every timed call reported a cache hit. Verification passed all 144 reference exports.

| Shape | Input | Profile | Median ms | Paired vs 96 main | Paired vs Aeglos | Peak MiB | M5 CPU / Metal |
|---|---|---|---:|---:|---:|---:|---:|
| 12x2 | catalog | off | 32.146 | +0.00% (0/6) | -33.63% (6/6) | 535.7 | 8.83× / 3.14× |
| 12x2 | catalog | 64-split | 29.697 | -7.56% (6/6) | -38.04% (6/6) | 532.7 | 9.56× / 3.40× |
| 12x2 | catalog | 64-main-only | 30.197 | -6.63% (6/6) | -37.25% (6/6) | 538.0 | 9.40× / 3.34× |
| 12x2 | catalog | aeglos | 48.145 | +50.68% (0/6) | +0.00% (0/6) | 3702.5 | 5.90× / 2.10× |
| 12x2 | heldout | off | 32.443 | +0.00% (0/6) | -32.51% (6/6) | 552.9 | — |
| 12x2 | heldout | 64-split | 29.871 | -7.33% (6/6) | -37.60% (6/6) | 540.9 | — |
| 12x2 | heldout | 64-main-only | 30.169 | -7.20% (6/6) | -37.07% (6/6) | 535.7 | — |
| 12x2 | heldout | aeglos | 48.038 | +48.17% (0/6) | +0.00% (0/6) | 3707.7 | — |
| 16x2 | catalog | off | 33.708 | +0.00% (0/6) | -39.57% (6/6) | 698.9 | 9.94× / 3.68× |
| 16x2 | catalog | 64-split | 31.200 | -7.44% (6/6) | -44.28% (6/6) | 673.5 | 10.74× / 3.97× |
| 16x2 | catalog | 64-main-only | 31.897 | -5.52% (6/6) | -43.16% (6/6) | 657.5 | 10.50× / 3.89× |
| 16x2 | catalog | aeglos | 55.911 | +65.49% (0/6) | +0.00% (0/6) | 4598.2 | 5.99× / 2.22× |
| 16x2 | heldout | off | 33.767 | +0.00% (0/6) | -40.03% (6/6) | 673.2 | — |
| 16x2 | heldout | 64-split | 31.483 | -6.98% (6/6) | -44.02% (6/6) | 663.4 | — |
| 16x2 | heldout | 64-main-only | 31.831 | -5.75% (6/6) | -43.18% (6/6) | 676.9 | — |
| 16x2 | heldout | aeglos | 56.347 | +66.76% (0/6) | +0.00% (0/6) | 4619.9 | — |
| merge-24x1 | catalog | off | 50.573 | +0.00% (0/6) | -39.54% (6/6) | 1110.1 | 10.03× / 3.46× |
| merge-24x1 | catalog | 64-split | 46.118 | -8.69% (6/6) | -45.10% (6/6) | 1057.2 | 10.99× / 3.79× |
| merge-24x1 | catalog | 64-main-only | 47.668 | -5.89% (6/6) | -43.14% (6/6) | 1073.0 | 10.64× / 3.67× |
| merge-24x1 | catalog | aeglos | 83.886 | +65.40% (0/6) | +0.00% (0/6) | 6595.3 | 6.04× / 2.09× |
| merge-24x1 | heldout | off | 47.234 | +0.00% (0/6) | -43.81% (6/6) | 1054.4 | — |
| merge-24x1 | heldout | 64-split | 43.742 | -7.90% (6/6) | -47.86% (6/6) | 1081.4 | — |
| merge-24x1 | heldout | 64-main-only | 43.828 | -7.58% (6/6) | -47.62% (6/6) | 1007.2 | — |
| merge-24x1 | heldout | aeglos | 84.001 | +77.98% (0/6) | +0.00% (0/6) | 6606.9 | — |

## Overlapping B2 and the quotient on 1×2

A separate panel checks the [early-B2 implementation](../2026-11-19-early-b2-disjoint739/README.md) with 80 main cores and a 16-core frontend on disjoint physical CPU IDs. The frontend starts the FFT while the main pool starts B2. The experimental off and early arms share the same binary. The accepted SDK supplies another control. Four transactions ran in five paired blocks with three primes and fifteen timed requests. Four additional proofs checked route activation. Gnark checked all 104 exports.

The originally supplied M5 request measured 10.396 ms in the early arm for 3.56× CPU and 2.31× Metal. The instance was terminated and temporary resources were removed.

| Input | Profile | Median ms | Paired vs same-build off | Paired vs SDK | Paired vs Aeglos | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| catalog | sdk | 7.812 | -0.19% (3/5) | +0.00% (0/5) | -5.90% (5/5) | 44.5 |
| catalog | off | 7.821 | +0.00% (0/5) | +0.19% (2/5) | -5.99% (5/5) | 45.3 |
| catalog | early | 10.396 | +32.93% (0/5) | +33.18% (0/5) | +25.39% (0/5) | 51.2 |
| catalog | early-grouped | 10.535 | +35.31% (0/5) | +34.70% (0/5) | +27.21% (0/5) | 50.5 |
| catalog | aeglos | 8.354 | +6.37% (0/5) | +6.27% (0/5) | +0.00% (0/5) | 462.9 |
| heldout | sdk | 7.794 | -0.36% (3/5) | +0.00% (0/5) | -5.16% (5/5) | 43.0 |
| heldout | off | 7.832 | +0.00% (0/5) | +0.36% (2/5) | -4.82% (5/5) | 40.0 |
| heldout | early | 10.387 | +31.75% (0/5) | +32.41% (0/5) | +27.44% (0/5) | 48.2 |
| heldout | early-grouped | 10.470 | +32.85% (0/5) | +34.14% (0/5) | +28.56% (0/5) | 51.2 |
| heldout | aeglos | 8.206 | +5.06% (0/5) | +5.44% (0/5) | +0.00% (0/5) | 457.1 |
| v27 | sdk | 7.822 | +0.43% (2/5) | +0.00% (0/5) | -3.09% (5/5) | 43.7 |
| v27 | off | 7.786 | +0.00% (0/5) | -0.43% (3/5) | -3.50% (5/5) | 39.9 |
| v27 | early | 10.374 | +33.51% (0/5) | +32.41% (0/5) | +29.07% (0/5) | 51.9 |
| v27 | early-grouped | 10.505 | +35.21% (0/5) | +34.03% (0/5) | +31.23% (0/5) | 50.5 |
| v27 | aeglos | 8.052 | +3.63% (0/5) | +3.19% (0/5) | +0.00% (0/5) | 457.4 |
| v28 | sdk | 7.752 | +0.17% (2/5) | +0.00% (0/5) | -6.62% (5/5) | 40.7 |
| v28 | off | 7.739 | +0.00% (0/5) | -0.17% (3/5) | -6.31% (5/5) | 41.5 |
| v28 | early | 10.362 | +33.56% (0/5) | +33.13% (0/5) | +25.34% (0/5) | 51.3 |
| v28 | early-grouped | 10.401 | +33.96% (0/5) | +33.29% (0/5) | +25.51% (0/5) | 50.5 |
| v28 | aeglos | 8.290 | +6.73% (0/5) | +7.08% (0/5) | +0.00% (0/5) | 460.6 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Proof verification](artifacts/verification.json) · [Screening results](../2026-11-19-wide-a-pool738/README.md) · [Eight-shape M5 comparison](../2026-11-19-raw-a-api737/README.md)


## Eight-shape M5 comparison on C7a

This comparison includes **3/8** verified 10× CPU results. It uses the original request for each supplied M5 measurement. For the three new shapes, the table uses the prespecified 64+32 placement when it improved the previous catalog time and beat prepared Aeglos on both inputs. Other rows retain their source and measured configuration from the [previous catalog](../2026-11-19-raw-a-api737/README.md). Every run had access to 96 physical C7a cores.

| Shape | Study and profile | C7a ms | M5 CPU / Metal ms | Speedup vs CPU / Metal | 10× CPU target ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 737 catalog | 7.953 | 37 / 24 | 4.65× / 3.02× | 3.7 |
| 2×2 | 707 catalog | 12.513 | 59 / 30 | 4.72× / 2.40× | 5.9 |
| 4×2 | 714 catalog | 17.210 | 109 / 48 | 6.33× / 2.79× | 10.9 |
| 8×2 | 707 catalog | 25.794 | 189 / 74 | 7.33× / 2.87× | 18.9 |
| 12×2 | 740 64+32 | 29.697 | 284 / 101 | 9.56× / 3.40× | 28.4 |
| 16×2 | 740 64+32 | 31.200 | 335 / 124 | 10.74× / 3.97× | 33.5 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | 52.8 |
| merge-24×1 | 740 64+32 | 46.118 | 507 / 175 | 10.99× / 3.79× | 50.7 |
