# Exact signed H options on C7a

Each 16×2 setting uses seven fresh processes at 96 cores or five at 64. The primary comparison uses the same compiled executable with unsigned-19 and signed-18. The previous integrated SDK and original signed prototype are independent controls. Peak RSS is the maximum across the complete SDK processes.

| Shape | Cores | Layout | Previous ms | Unsigned ms | Clean signed ms | Trial ms | Paired vs unsigned | Faster blocks | Paired vs previous | Faster blocks | Peak MiB unsigned / signed |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 16x2 | 96 | 18s | 36.783 | 36.108 | 35.841 | 36.182 | -0.71% | 4/7 | -2.56% | 7/7 | 836 / 785 |
| 16x2 | 64 | 18s | 34.676 | 33.980 | 34.924 | 34.550 | +1.62% | 1/5 | -0.36% | 3/5 | 781 / 793 |
| merge-24x1 | 96 | 18s | 51.658 | 51.260 | 51.326 | — | -2.43% | 4/5 | -0.93% | 4/5 | 1062 / 1059 |
| 24x2 | 96 | 17s | 50.276 | 50.617 | 49.265 | — | -1.25% | 3/5 | +0.31% | 2/5 | 1248 / 1274 |

## Eight-shape scorecard

The widths were fixed in experiment 363. Small shapes keep the same solver/G2 settings. The four larger shapes use signed-18 in this scorecard.

| Shape | Physical cores | SDK ms | Peak MiB | M5 CPU / Metal ms | CPU / Metal speedup | Blocks at 10× CPU |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 96 | 8.326 | 44 | 37 / 24 | 4.44× / 2.88× | 0/5 |
| 2x2 | 48 | 13.277 | 98 | 59 / 30 | 4.44× / 2.26× | 0/5 |
| 4x2 | 48 | 17.248 | 269 | 109 / 48 | 6.32× / 2.78× | 0/5 |
| 8x2 | 48 | 23.846 | 457 | 189 / 74 | 7.93× / 3.10× | 0/5 |
| 12x2 | 48 | 31.054 | 642 | 284 / 101 | 9.15× / 3.25× | 0/5 |
| 16x2 | 64 | 34.924 | 793 | 335 / 124 | 9.59× / 3.55× | 0/5 |
| 24x2 | 64 | 48.477 | 1264 | 528 / 176 | 10.89× / 3.63× | 5/5 |
| merge-24x1 | 64 | 48.471 | 1022 | 507 / 175 | 10.46× / 3.61× | 5/5 |

[AWS protocol](artifacts/aws-protocol.md) · [All process samples](artifacts/aws.json.gz) · [Native route checks](artifacts/routes.json)
