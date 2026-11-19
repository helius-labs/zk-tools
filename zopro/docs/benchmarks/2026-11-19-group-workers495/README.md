# Worker counts with sampled scalar groups on 12×2

All configurations use the same sampled-group executable and the same 16 physical Ryzen cores. Each additional pair of workers uses one sibling per L3 group, with placements rotated by block. Five randomized blocks measure 11 warm proofs per process after three primes. The original fixture, adjacent-leaf request 58 and scattered-leaf request 139 were selected before timing. Median process means and within-block changes are reported below. RSS is the highest process peak at that setting. All 60 exported proofs passed independent Gnark verification.

| Request | Workers | Proof ms | Paired vs 32 (wins) | CPU-ms/proof | Peak MiB |
|---|---:|---:|---:|---:|---:|
| Original | 24 | 30.640 | +0.86% (0/5) | 659.2 | 570.8 |
| Original | 28 | 29.372 | -2.27% (5/5) | 736.4 | 596.6 |
| Original | 30 | 29.794 | -0.96% (5/5) | 783.0 | 620.8 |
| Original | 32 | 30.312 | reference | 840.1 | 617.2 |
| 58 | 24 | 30.840 | +3.81% (0/5) | 661.6 | 600.2 |
| 58 | 28 | 29.356 | -1.87% (4/5) | 731.9 | 623.5 |
| 58 | 30 | 29.626 | -1.06% (4/5) | 780.1 | 636.8 |
| 58 | 32 | 29.883 | reference | 828.8 | 623.9 |
| 139 | 24 | 47.950 | -0.01% (3/5) | 1038.9 | 939.4 |
| 139 | 28 | 47.075 | -1.42% (5/5) | 1136.1 | 954.1 |
| 139 | 30 | 47.611 | +0.24% (2/5) | 1188.8 | 992.8 |
| 139 | 32 | 47.729 | reference | 1252.4 | 1047.7 |

[Process measurements](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [Sampled groups](../2026-11-19-sampled-groups487/README.md)
