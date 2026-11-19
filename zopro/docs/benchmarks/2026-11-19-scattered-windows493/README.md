# H windows across seven scattered 12×2 requests

The preceding [three-request screen](../2026-11-19-group-windows492/README.md) found a 2.05% paired improvement from 19 unsigned H windows on scattered-leaf request 137. This follow-up uses the other seven public scattered-leaf requests from that workload. The same sampled-group executable ran with 32 pinned Ryzen workers, three primes and eleven timed proofs per process. Five randomized blocks compare ordinary H with the single-range 19-window configuration. Latencies are medians of process means. Percent changes compare paired processes within each block. RSS is the maximum across complete processes.

| Request | Ordinary ms | 19 windows ms | Paired change | Faster blocks | Peak MiB ordinary / 19 |
|---|---:|---:|---:|---:|---:|
| 138 | 47.520 | 48.064 | +0.42% | 2/5 | 982.3 / 1005.0 |
| 139 | 47.726 | 47.852 | +0.26% | 2/5 | 1025.2 / 1012.9 |
| 140 | 47.665 | 49.149 | +3.16% | 0/5 | 1003.5 / 972.4 |
| 141 | 47.580 | 48.186 | +1.27% | 1/5 | 1021.0 / 1028.1 |
| 142 | 47.478 | 48.610 | +2.70% | 0/5 | 1002.7 / 994.4 |
| 143 | 47.725 | 48.132 | +0.65% | 2/5 | 1022.9 / 1027.5 |
| 144 | 47.775 | 48.298 | +0.70% | 2/5 | 1010.1 / 986.8 |

Across the seven inputs the median paired change was +0.70%, with 0/7 favorable. All 70 exported proofs passed independent Gnark verification.

[Process measurements](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)
