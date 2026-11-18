# Local dependency bands and proof latency on Ryzen

The integrated prover already supports two controls for refining large tasks into smaller sets of independent instructions. All runs have eight outer epochs. The standard configuration uses eight local bands and a 4096-operation minimum. The early configuration lowers the minimum to 1024. The deep and coarse configurations use sixteen and four bands respectively at the standard minimum. Schedule counts come from a separate diagnostic executable. All timed requests use the integrated release.

| Shape | Configuration | Leaf tasks | Work / span under unit costs | Full proof ms | Paired change vs standard | Faster blocks | Peak MiB |
|---|---|---:|---:|---:|---:|---:|---:|
| 1x2 | standard | 118 | 2.66 | 7.856 | +0.00% | 0/5 | 87 |
| 1x2 | early | 235 | 2.66 | 7.982 | +5.21% | 2/5 | 76 |
| 1x2 | deep | 156 | 2.67 | 7.623 | -3.10% | 3/5 | 64 |
| 1x2 | coarse | 88 | 2.66 | 7.591 | -4.11% | 3/5 | 63 |
| 16x2 | standard | 829 | 28.45 | 45.431 | +0.00% | 0/5 | 606 |
| 16x2 | early | 2444 | 28.46 | 46.958 | +3.67% | 0/5 | 608 |
| 16x2 | deep | 1210 | 29.06 | 45.657 | +1.22% | 2/5 | 612 |
| 16x2 | coarse | 631 | 16.26 | 47.079 | +3.63% | 1/5 | 630 |
| merge-24x1 | standard | 1378 | 15.11 | 72.092 | +0.00% | 0/5 | 900 |
| merge-24x1 | early | 3823 | 15.11 | 73.040 | +1.34% | 1/5 | 926 |
| merge-24x1 | deep | 2101 | 15.92 | 71.898 | +0.02% | 2/5 | 959 |
| merge-24x1 | coarse | 918 | 12.91 | 71.849 | +0.06% | 2/5 | 883 |

Each process exported a proof that passed the independent Gnark verifier. The work/span calculation counts stored instruction entries equally. For 1×2, the post-initial epochs contain at most three top-level tasks at the standard setting. Dividing tasks sooner roughly doubles the total number of leaves while leaving the modeled parallelism near 2.66. The full-proof measurements test the effect of that change with actual operation costs.

[Protocol](artifacts/protocol.md) · [Timings](artifacts/timings.json.gz) · [Schedules](artifacts/schedules.json.gz) · [Verification](artifacts/verification.json)
