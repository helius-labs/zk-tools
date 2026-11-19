# Finer input partitions for the G2 MSM

Four 11-bit ranges expose 48 independent window fills on the current small-query path. The trial compares eight 10-bit ranges (104 fills), eight 11-bit ranges (96 fills), and twelve 10-bit ranges (156 fills). Each partition uses the existing streaming fill. Neighboring results reduce in the Rayon tree. Preparation reserves positive growth in the held arrays and per-worker scratch before admission. All allocations belong to the current proof. The original schedule handles insufficient headroom. A native generator oracle covers both window widths, all three partitions, incomplete last partitions and signed scalars.

The Ryzen check uses 32 logical workers, three warmups and eleven timed proofs per process. Four blocks compare the current release, the same experimental executable disabled, and its three enabled configurations. Parent and disabled use four 11-bit ranges with serial scalar decomposition. A separate verification checks the zero-budget fallback on 1×16. All 182 process exports verified. The width labels give digit bits × point partitions.

| Shape | Request | Geometry | Release / off / enabled ms | vs off (wins) | vs release (wins) | Peak MiB release / enabled |
|---|---|---|---:|---:|---:|---:|
| 1x16 | catalog | 10x8 | 13.129 / 13.232 / 13.572 | +2.05% (0/4) | +3.38% (0/4) | 122.9 / 145.0 |
| 1x16 | catalog | 11x8 | 13.129 / 13.232 / 13.462 | +1.32% (2/4) | +1.86% (1/4) | 122.9 / 130.1 |
| 1x16 | catalog | 10x12 | 13.129 / 13.232 / 14.037 | +6.09% (0/4) | +6.08% (0/4) | 122.9 / 146.2 |
| 1x16 | 57 | 10x8 | 13.376 / 13.299 / 13.478 | +2.21% (1/4) | +1.00% (1/4) | 132.2 / 120.1 |
| 1x16 | 57 | 11x8 | 13.376 / 13.299 / 13.552 | +1.91% (1/4) | +0.84% (1/4) | 132.2 / 122.6 |
| 1x16 | 57 | 10x12 | 13.376 / 13.299 / 14.227 | +5.53% (0/4) | +6.79% (0/4) | 132.2 / 121.7 |
| 1x2 | catalog | 10x8 | 7.328 / 7.264 / 7.530 | +3.00% (0/4) | +2.77% (0/4) | 60.0 / 57.0 |
| 1x2 | catalog | 11x8 | 7.328 / 7.264 / 7.662 | +4.81% (0/4) | +4.70% (0/4) | 60.0 / 62.0 |
| 1x2 | catalog | 10x12 | 7.328 / 7.264 / 8.054 | +10.16% (0/4) | +10.04% (0/4) | 60.0 / 61.2 |
| 1x4 | catalog | 10x8 | 8.833 / 8.789 / 9.092 | +3.29% (0/4) | +2.94% (0/4) | 91.0 / 85.5 |
| 1x4 | catalog | 11x8 | 8.833 / 8.789 / 9.135 | +4.68% (0/4) | +3.83% (0/4) | 91.0 / 92.2 |
| 1x4 | catalog | 10x12 | 8.833 / 8.789 / 9.774 | +10.76% (0/4) | +10.80% (0/4) | 91.0 / 87.2 |
| 1x8 | catalog | 10x8 | 10.324 / 10.400 / 10.484 | +1.43% (1/4) | +2.07% (0/4) | 94.5 / 100.2 |
| 1x8 | catalog | 11x8 | 10.324 / 10.400 / 10.780 | +3.59% (0/4) | +4.07% (0/4) | 94.5 / 102.0 |
| 1x8 | catalog | 10x12 | 10.324 / 10.400 / 11.262 | +8.46% (0/4) | +8.76% (0/4) | 94.5 / 101.2 |
| 2x2 | catalog | 10x8 | 9.916 / 9.876 / 10.155 | +2.94% (0/4) | +2.45% (0/4) | 107.5 / 104.8 |
| 2x2 | catalog | 11x8 | 9.916 / 9.876 / 10.345 | +4.69% (0/4) | +4.23% (0/4) | 107.5 / 108.0 |
| 2x2 | catalog | 10x12 | 9.916 / 9.876 / 10.929 | +10.75% (0/4) | +10.51% (0/4) | 107.5 / 104.5 |
| 2x4 | catalog | 10x8 | 10.905 / 10.821 / 11.253 | +4.00% (0/4) | +3.65% (1/4) | 109.5 / 111.0 |
| 2x4 | catalog | 11x8 | 10.905 / 10.821 / 11.540 | +7.06% (0/4) | +5.34% (0/4) | 109.5 / 117.0 |
| 2x4 | catalog | 10x12 | 10.905 / 10.821 / 11.948 | +10.85% (0/4) | +9.13% (0/4) | 109.5 / 112.2 |
| 2x8 | catalog | 10x8 | 13.648 / 13.563 / 13.973 | +2.79% (0/4) | +2.61% (0/4) | 144.0 / 142.2 |
| 2x8 | catalog | 11x8 | 13.648 / 13.563 / 14.047 | +3.19% (0/4) | +2.98% (0/4) | 144.0 / 146.2 |
| 2x8 | catalog | 10x12 | 13.648 / 13.563 / 14.413 | +6.73% (0/4) | +5.61% (0/4) | 144.0 / 141.7 |

All 24 local geometry/input combinations had higher paired median latency than both controls. On 1×16, increasing the number of partitions to twelve cost roughly 6% on both inputs. The C7a check uses the outstanding equal-96-core cases to test for a width-specific effect.

[Earlier C7a geometry comparison](../2026-11-18-wide-g2-c7a298/README.md) · [Source](artifacts/source.patch) · [Dispatch checks](artifacts/routes.json) · [Sample timings](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
