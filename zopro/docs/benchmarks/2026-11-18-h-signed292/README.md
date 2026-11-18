# Signed windows for the dense H query

The experiment reused the signed streaming MSM with one point partition. Signed width 14 and unsigned width 13 use 8,192 ordinary buckets. Signed width 15 and unsigned width 14 use 16,384. Wider digits reduce the number of full passes and change the work required to fill and reduce buckets. The signed workspace forecast charges the selected kernel.

The primary signed-14 comparison on 24×2 won three of five blocks and reduced paired latency by 1.55%. The experiment required four wins and 2%. The secondary signed-15 mode used 1.47–2.75% less aggregate CPU across the four shapes, with smaller changes in wall time. The production default stays as tested before this experiment.

| Shape | Mode | Median ms | Paired wall change vs matching unsigned | Wins / 5 | Paired CPU change | Peak MiB | M5 CPU / Metal |
|---|---|---:|---:|---:|---:|---:|---:|
| 24x2 | parent13 | 73.612 | +0.32% | 2 | -0.22% | 961 | 7.17× / 2.39× |
| 24x2 | 13 | 74.619 | +0.00% | — | +0.00% | 971 | 7.08× / 2.36× |
| 24x2 | signed14 | 73.818 | -1.55% | 3 | -0.78% | 950 | 7.15× / 2.38× |
| 24x2 | 14 | 73.101 | +0.00% | — | +0.00% | 961 | 7.22× / 2.41× |
| 24x2 | signed15 | 72.054 | -1.00% | 4 | -2.59% | 947 | 7.33× / 2.44× |
| merge-24x1 | parent13 | 72.996 | -1.37% | 3 | -0.25% | 881 | 6.95× / 2.40× |
| merge-24x1 | 13 | 72.377 | +0.00% | — | +0.00% | 889 | 7.00× / 2.42× |
| merge-24x1 | signed14 | 72.174 | -0.28% | 3 | -1.46% | 861 | 7.02× / 2.42× |
| merge-24x1 | 14 | 71.819 | +0.00% | — | +0.00% | 876 | 7.06× / 2.44× |
| merge-24x1 | signed15 | 72.057 | -0.10% | 3 | -2.75% | 902 | 7.04× / 2.43× |
| 16x2 | parent13 | 45.686 | +1.03% | 2 | +0.07% | 590 | 7.33× / 2.71× |
| 16x2 | 13 | 45.295 | +0.00% | — | +0.00% | 595 | 7.40× / 2.74× |
| 16x2 | signed14 | 45.045 | -0.40% | 4 | -0.67% | 596 | 7.44× / 2.75× |
| 16x2 | 14 | 45.093 | +0.00% | — | +0.00% | 598 | 7.43× / 2.75× |
| 16x2 | signed15 | 44.526 | -0.68% | 4 | -2.51% | 630 | 7.52× / 2.78× |
| 12x2 | parent13 | 38.559 | -1.23% | 4 | +0.30% | 522 | 7.37× / 2.62× |
| 12x2 | 13 | 39.040 | +0.00% | — | +0.00% | 512 | 7.27× / 2.59× |
| 12x2 | signed14 | 38.548 | +0.09% | 2 | -0.33% | 508 | 7.37× / 2.62× |
| 12x2 | 14 | 38.432 | +0.00% | — | +0.00% | 527 | 7.39× / 2.63× |
| 12x2 | signed15 | 39.824 | -1.30% | 3 | -1.47% | 510 | 7.13× / 2.54× |

Each cell used five fresh processes, three primes and 11 timed proofs on 16 physical Ryzen cores. Parent13 is the exact 285 executable at unsigned 13. Each signed mode is paired against the unsigned control with the same bucket count. The generator-sum oracle passed for both signed widths, and all 100 distinct full-proof process exports passed independent Gnark verification. [Pairs](artifacts/cells.json) · [Samples](artifacts/rows.json.gz) · [Patch](artifacts/source.patch.gz) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json)

All 146 native core tests passed with signed width 15 enabled. The generic core library passed an offline compile check.
