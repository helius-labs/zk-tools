# Ready-task grouping on AWS C7a

Each cell uses five block-randomized fresh processes on the same host. Results are complete SDK wall times. CPU time sums all threads. Peak memory is the maximum whole-process resident set across the five processes. The route marker records whether at least one wave exercised grouping.

| Shape | Cores | Groups | Release ms | Disabled ms | Grouped ms | Change vs disabled | Faster blocks | Release / grouped CPU-ms | Peak MiB | Marked processes |
|---|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 64 | half | 8.344 | 8.279 | 8.349 | +0.85% | 0/5 | 279 / 281 | 51 | 5/5 |
| 1x2 | 64 | one | 8.344 | 8.279 | 8.308 | +0.41% | 0/5 | 279 / 282 | 50 | 5/5 |
| 1x2 | 64 | two | 8.344 | 8.279 | 8.265 | +0.19% | 1/5 | 279 / 282 | 55 | 5/5 |
| 16x2 | 64 | half | 34.277 | 34.581 | 34.716 | -0.36% | 3/5 | 1684 / 1720 | 719 | 5/5 |
| 16x2 | 64 | one | 34.277 | 34.581 | 34.052 | -2.51% | 3/5 | 1684 / 1701 | 724 | 5/5 |
| 16x2 | 64 | two | 34.277 | 34.581 | 34.580 | -1.03% | 3/5 | 1684 / 1713 | 729 | 5/5 |
| 16x2 | 96 | half | 37.065 | 37.334 | 36.838 | -1.57% | 4/5 | 2527 / 2611 | 724 | 5/5 |
| 16x2 | 96 | one | 37.065 | 37.334 | 37.609 | +1.15% | 0/5 | 2527 / 2651 | 712 | 5/5 |
| 16x2 | 96 | two | 37.065 | 37.334 | 37.382 | +0.13% | 2/5 | 2527 / 2634 | 722 | 5/5 |
| merge-24x1 | 64 | half | 48.910 | 49.058 | 50.886 | +3.25% | 1/5 | 2312 / 2411 | 1053 | 5/5 |
| merge-24x1 | 64 | one | 48.910 | 49.058 | 49.660 | -0.36% | 3/5 | 2312 / 2356 | 1048 | 5/5 |
| merge-24x1 | 64 | two | 48.910 | 49.058 | 49.517 | +1.61% | 1/5 | 2312 / 2355 | 1036 | 5/5 |
| merge-24x1 | 96 | half | 53.588 | 53.139 | 53.808 | +1.37% | 1/5 | 3422 / 3578 | 1077 | 5/5 |
| merge-24x1 | 96 | one | 53.588 | 53.139 | 53.946 | +0.30% | 2/5 | 3422 / 3577 | 1051 | 5/5 |
| merge-24x1 | 96 | two | 53.588 | 53.139 | 54.032 | -0.10% | 3/5 | 3422 / 3548 | 1065 | 5/5 |

[Protocol](artifacts/aws-protocol.md) · [Full AWS timings](artifacts/aws.json.gz) · [Local schedule counts](README.md)
