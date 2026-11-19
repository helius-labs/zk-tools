# Public SDK frontend placement across eight varied inputs

Each 2×2 witness ran in four randomized pairs. Eight pinned physical cores in one L3 served the frontend, with four of their SMT siblings added in the twelve-worker arm. Both arms used the same executable, 32 main workers and the same L3 in each block. Each witness used each L3 twice. Wall time is the median of four process means, each from 31 verified requests after three warmups.

| Witness | 8 workers ms | 12 workers ms | Median paired change | Faster pairs | Peak MiB, 8 / 12 |
|---|---:|---:|---:|---:|---:|
| 1 | 9.910 | 9.862 | -0.48% | 4/4 | 115.0 / 121.2 |
| 2 | 9.923 | 9.911 | +0.05% | 2/4 | 112.8 / 120.8 |
| 3 | 9.912 | 9.859 | -0.58% | 3/4 | 115.0 / 123.5 |
| 4 | 9.880 | 9.851 | -0.19% | 3/4 | 111.2 / 122.2 |
| 5 | 9.913 | 9.985 | +0.93% | 1/4 | 111.0 / 118.8 |
| 6 | 10.044 | 9.971 | -0.94% | 4/4 | 118.2 / 121.2 |
| 7 | 9.910 | 9.865 | -0.69% | 3/4 | 116.2 / 121.0 |
| 8 | 9.934 | 9.916 | -0.59% | 3/4 | 114.2 / 119.8 |

Twelve workers had lower paired latency on 6/8 witnesses. The median paired change across witnesses was -0.53%. All 64 exports passed independent verification.

[Process samples](artifacts/processes.json.gz) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json) · [Original public SDK comparison](../2026-11-19-entry-placement442/README.md)
