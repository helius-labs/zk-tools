# Checked indexed grouping across the ordinary release SDK

The same candidate and deployed parent run across all cases. Latency is the median over six process means. The two execution-order columns contain within-block median changes with the parent first and candidate first, respectively. Peak memory includes key setup and all warmed requests.

| Shape | Request or configuration | Parent / checked index ms | Paired change | Parent first / candidate first | Peak parent / checked MiB | Checked vs M5 CPU / Metal |
|---|---|---:|---:|---:|---:|---:|
| 1x2 | Original | 7.056 / 7.012 | -1.08% (4/6) | -0.87% / -1.29% | 62.0 / 60.8 | 5.28× / 3.42× |
| 2x2 | Original | 9.602 / 9.624 | +0.18% (2/6) | +0.88% / -0.95% | 107.2 / 106.5 | 6.13× / 3.12× |
| 4x2 | Original | 15.710 / 15.217 | -2.27% (4/6) | -3.82% / -2.17% | 252.8 / 258.8 | 7.16× / 3.15× |
| 8x2 | Original | 24.130 / 24.343 | +0.67% (2/6) | +1.59% / +0.40% | 442.5 / 437.2 | 7.76× / 3.04× |
| 12x2 | Original | 31.527 / 30.800 | -1.72% (6/6) | -2.50% / -1.20% | 575.3 / 521.1 | 9.22× / 3.28× |
| 16x2 | Original | 39.955 / 39.740 | -0.86% (6/6) | -1.07% / -0.65% | 660.8 / 632.3 | 8.43× / 3.12× |
| 24x2 | Original | 64.466 / 62.979 | -3.15% (5/6) | -3.11% / -3.21% | 1060.0 / 996.0 | 8.38× / 2.79× |
| merge-24x1 | Original | 63.874 / 63.123 | -1.22% (5/6) | -1.39% / -1.05% | 1040.3 / 1027.5 | 8.03× / 2.77× |
| merge-54x1 | Original | 156.777 / 153.207 | -2.01% (6/6) | -2.19% / -0.74% | 1705.7 / 1697.9 | — |
| 12x2 | Scattered 139 | 46.873 / 45.567 | -2.74% (6/6) | -3.02% / -2.45% | 967.3 / 776.4 | — |
| 1x2 | Option unset | 7.189 / 7.138 | -0.90% (4/6) | +0.39% / -0.90% | 66.0 / 62.5 | 5.18× / 3.36× |
| 2x2 | Option unset | 9.612 / 9.672 | +0.19% (2/6) | +0.12% / +1.00% | 112.0 / 104.2 | 6.10× / 3.10× |

All 192 exports verified independently. [Protocol](protocol.md) · [Order-specific results](artifacts/paired.json) · [Samples and CPU profiles](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
