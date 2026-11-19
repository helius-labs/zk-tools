# Selecting a second prepared group plan after the broad plan declines

Each of A, private K and B2 tries the broad plan first. When its sample, exact correction check or workspace reservation declines, the query tries the fine plan on the original points. Both plans use the same key and public offset file for all requests. The three queries decide independently. Each plan has its own reservation, and any optional plan that cannot fit is skipped. The fine A and K groups are projections of the trained B2 groups onto their respective wire indices.

Five randomized blocks compare the 32-worker production release, the prior 30-worker two-stage B2 build, and the new 30-worker build with broad groups, fine groups, or fallback enabled for all three queries. Each process uses three warmups and eleven timed calls. Peak RSS includes setup.

| Request | Release / two-stage B2 / broad / fine / fallback ms | Fallback vs release | Fallback vs two-stage | Fallback vs broad | Peak MiB release / two-stage / fallback |
|---|---:|---:|---:|---:|---:|
| Original | 31.245 / 29.463 / 29.617 / 30.567 / 29.640 | -5.86% (5/5) | +0.31% (1/5) | +0.63% (2/5) | 563.4 / 572.3 / 564.3 |
| 57 | 31.274 / 29.326 / 29.410 / 30.511 / 29.294 | -6.65% (5/5) | -0.26% (3/5) | +0.01% (2/5) | 579.1 / 578.5 / 562.3 |
| 59 | 32.187 / 29.402 / 29.755 / 30.699 / 29.408 | -8.62% (5/5) | +0.13% (2/5) | -1.18% (4/5) | 578.0 / 579.6 / 549.4 |
| 64 | 31.517 / 29.657 / 30.031 / 30.529 / 29.972 | -4.82% (5/5) | +0.89% (0/5) | -0.20% (3/5) | 581.8 / 573.8 / 547.3 |
| 141 | 47.469 / 45.323 / 47.242 / 45.984 / 45.884 | -4.67% (5/5) | +0.84% (1/5) | -3.23% (5/5) | 972.7 / 1039.3 / 1034.8 |
| 142 | 47.200 / 45.289 / 46.691 / 46.230 / 46.261 | -1.62% (4/5) | +2.15% (0/5) | -1.41% (4/5) | 977.8 / 977.7 / 1055.0 |
| 143 | 47.485 / 45.576 / 47.404 / 46.383 / 45.626 | -3.69% (5/5) | +0.13% (2/5) | -3.92% (5/5) | 963.6 / 1013.2 / 974.8 |
| 144 | 48.412 / 45.419 / 47.353 / 44.936 / 44.950 | -6.49% (5/5) | -1.15% (4/5) | -4.52% (5/5) | 1016.6 / 986.0 / 1021.3 |

The original reached 9.58× the supplied M5 CPU result and 3.41× Metal. 0/5 blocks met 28.4 ms. All 212 exports verified against the original key. [Timings](artifacts/processes.json.gz) · [Budget probes](artifacts/budget-probes.json) · [Source](artifacts/incremental.patch)
