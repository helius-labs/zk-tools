# Starting H while the other queries classify scalars

The H branch has no dependency on the choice of a representative for A, K or B2. The experimental join starts H alongside the subtree that classifies groups and calculates the other three queries. Four settings in one executable toggle the join order and cross-query sharing independently. The earlier unmodified adaptive executable provides a fifth control. All configurations use one prepared group plan per shape. The 12×2 requests use 30 workers and the larger requests use 32. Four randomized matched blocks each use three warmups and eleven timed proofs.

| Shape | Request | Earlier adaptive / independent / shared / early independent / early shared ms | Early shared vs shared | Early shared vs early independent | Early shared vs earlier adaptive | Peak MiB earlier / early shared |
|---|---|---:|---:|---:|---:|---:|
| 12x2 | Original | 29.028 / 29.460 / 29.838 / 29.242 / 29.042 | -2.49% (4/4) | -0.99% (3/4) | -0.32% (2/4) | 527.2 / 521.1 |
| 12x2 | 59 | 29.333 / 29.325 / 29.458 / 29.259 / 29.146 | -0.44% (3/4) | -0.14% (2/4) | -1.15% (3/4) | 520.0 / 520.3 |
| 12x2 | 141 | 47.349 / 47.682 / 47.751 / 47.603 / 47.151 | -1.23% (4/4) | -0.28% (2/4) | -0.41% (2/4) | 991.0 / 1031.2 |
| 16x2 | Original | 37.494 / 37.176 / 40.042 / 37.555 / 38.380 | -3.87% (4/4) | +2.20% (1/4) | +2.21% (0/4) | 768.0 / 729.7 |
| 24x2 | Original | 59.074 / 59.052 / 63.641 / 58.546 / 60.231 | -5.31% (4/4) | +2.95% (1/4) | +2.90% (0/4) | 1125.1 / 1117.8 |
| 24x2 | 99 | 58.845 / 57.837 / 64.216 / 58.251 / 59.263 | -7.67% (4/4) | +1.63% (1/4) | +0.73% (2/4) | 1128.6 / 1095.0 |

All 132 exports verified independently. [Measurements](artifacts/processes.json.gz) · [Probes](artifacts/probes.json) · [Change](artifacts/incremental.patch) · [Verification](artifacts/verification.json)
