# Parallel exact scalar classes for one grouped G2 query

The large-key B2 query owns its scalar classes. It can reorder the classes while preserving the correspondence between each distinct coefficient and all of its points. For each proof this trial counts and partitions nonzero point indices by a few bits of their stored scalar. Each of eight or 32 shards classifies its indices independently using all four scalar limbs for equality. It converts the distinct values to canonical form and preserves input order inside each class. Shard outputs concatenate directly. Other query consumers keep the original grouping order.

Admission checks the existing grouping budget and a second conservative scratch reservation. The trial covers 65,536–262,144 B2 points and retains no data between proofs. Metadata for prior and proposed execution is private to the process. A focused oracle covers full-scalar collisions in one shard, distinct shards, zero inputs, admission boundaries and stable group membership.

The Ryzen run uses 32 logical workers, the same public coefficient assets, packed FFT and grouped G2 in the integrated and trial arms. Four paired blocks per input ran eleven timed proofs after three primes. Four shapes include a separately generated transaction. A 2×16 zero-budget case tests the fallback. All 323 process exports verified.

Eight-way and 32-way classification reduced the paired median against their same-binary control on 3/15 and 2/15 inputs, respectively. No unresolved large key qualified for the equal-core screen under the requirement that every measured input beat both controls. The original grouping remains the measured local choice across this panel.

| Shape | Request | Shards | Release / integrated / off / enabled ms | vs off (wins) | vs integrated (wins) | Peak MiB release / enabled |
|---|---|---|---:|---:|---:|---:|
| 2x16 | catalog | 8 | 16.619 / 16.415 / 16.254 / 16.329 | +0.42% (1/4) | +0.47% (2/4) | 242.4 / 249.8 |
| 2x16 | catalog | 32 | 16.619 / 16.415 / 16.254 / 16.477 | +1.06% (1/4) | +1.56% (1/4) | 242.4 / 245.4 |
| 2x16 | 67 | 8 | 16.763 / 16.536 / 16.335 / 16.257 | -0.92% (3/4) | -1.86% (4/4) | 261.9 / 252.9 |
| 2x16 | 67 | 32 | 16.763 / 16.536 / 16.335 / 16.337 | -0.56% (3/4) | -1.60% (4/4) | 261.9 / 242.6 |
| 3x2 | catalog | 8 | 13.390 / 13.155 / 12.948 / 13.223 | +1.65% (0/4) | +0.37% (1/4) | 202.8 / 203.0 |
| 3x2 | catalog | 32 | 13.390 / 13.155 / 12.948 / 13.234 | +2.00% (0/4) | +0.92% (1/4) | 202.8 / 205.8 |
| 3x4 | catalog | 8 | 13.999 / 13.630 / 13.444 / 13.694 | +1.99% (0/4) | -0.22% (3/4) | 227.0 / 210.5 |
| 3x4 | catalog | 32 | 13.999 / 13.630 / 13.444 / 13.762 | +1.58% (0/4) | +0.67% (2/4) | 227.0 / 214.2 |
| 3x4 | 67 | 8 | 13.797 / 13.722 / 13.612 / 13.469 | -1.12% (3/4) | -0.87% (2/4) | 212.5 / 225.0 |
| 3x4 | 67 | 32 | 13.797 / 13.722 / 13.612 / 13.821 | +1.28% (0/4) | +1.15% (1/4) | 212.5 / 214.8 |
| 3x8 | catalog | 8 | 15.304 / 15.080 / 15.234 / 15.030 | -1.34% (3/4) | -0.22% (3/4) | 233.9 / 228.9 |
| 3x8 | catalog | 32 | 15.304 / 15.080 / 15.234 / 15.212 | -0.45% (3/4) | +0.68% (1/4) | 233.9 / 247.7 |
| 3x8 | 67 | 8 | 15.396 / 16.385 / 15.182 / 15.384 | +1.33% (1/4) | -5.68% (2/4) | 236.4 / 252.2 |
| 3x8 | 67 | 32 | 15.396 / 16.385 / 15.182 / 15.488 | +0.50% (2/4) | -6.36% (2/4) | 236.4 / 230.2 |
| 4x16 | catalog | 8 | 19.718 / 19.889 / 19.370 / 19.675 | +1.85% (0/4) | -1.08% (3/4) | 310.4 / 316.3 |
| 4x16 | catalog | 32 | 19.718 / 19.889 / 19.370 / 19.607 | +1.23% (0/4) | -1.79% (3/4) | 310.4 / 311.3 |
| 4x16 | 67 | 8 | 19.570 / 19.512 / 19.402 / 19.781 | +2.35% (1/4) | +1.78% (1/4) | 305.8 / 315.5 |
| 4x16 | 67 | 32 | 19.570 / 19.512 / 19.402 / 19.751 | +1.92% (0/4) | +1.92% (0/4) | 305.8 / 312.6 |
| 4x2 | catalog | 8 | 14.970 / 14.697 / 14.449 / 14.904 | +3.03% (0/4) | +1.94% (1/4) | 224.2 / 235.8 |
| 4x2 | catalog | 32 | 14.970 / 14.697 / 14.449 / 14.959 | +3.25% (0/4) | +2.00% (1/4) | 224.2 / 233.0 |
| 4x4 | catalog | 8 | 15.252 / 15.170 / 15.040 / 15.249 | +1.47% (1/4) | +0.32% (1/4) | 225.5 / 247.8 |
| 4x4 | catalog | 32 | 15.252 / 15.170 / 15.040 / 15.479 | +4.02% (1/4) | +1.56% (1/4) | 225.5 / 252.0 |
| 4x8 | catalog | 8 | 17.618 / 16.641 / 16.797 / 16.849 | +0.68% (1/4) | +1.40% (0/4) | 268.4 / 272.9 |
| 4x8 | catalog | 32 | 17.618 / 16.641 / 16.797 / 16.996 | +1.18% (1/4) | +2.11% (1/4) | 268.4 / 321.5 |
| 5x16 | catalog | 8 | 23.397 / 22.685 / 22.481 / 23.442 | +3.15% (0/4) | +1.73% (2/4) | 367.6 / 413.7 |
| 5x16 | catalog | 32 | 23.397 / 22.685 / 22.481 / 22.889 | +1.76% (0/4) | +0.79% (1/4) | 367.6 / 377.0 |
| 5x2 | catalog | 8 | 16.698 / 18.521 / 16.385 / 16.596 | +0.67% (2/4) | +0.17% (2/4) | 245.0 / 330.0 |
| 5x2 | catalog | 32 | 16.698 / 18.521 / 16.385 / 16.702 | +2.69% (1/4) | -8.49% (2/4) | 245.0 / 257.5 |
| 5x4 | catalog | 8 | 17.300 / 17.161 / 17.101 / 17.306 | +1.20% (1/4) | +0.22% (2/4) | 264.1 / 274.8 |
| 5x4 | catalog | 32 | 17.300 / 17.161 / 17.101 / 17.459 | +2.08% (1/4) | +2.19% (0/4) | 264.1 / 290.2 |

[Integrated grouped path](../2026-11-19-owned-phi661/README.md) · [Source](artifacts/source.patch) · [Dispatch checks](artifacts/routes.json) · [Timings](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
