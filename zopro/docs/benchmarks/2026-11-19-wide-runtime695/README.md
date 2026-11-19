# Idle search limits for the 96-worker main prover pool

A compile of the current release uses an opt-in change to Rayon Core 1.13.0. The original scheduler attempts 32 rounds before a worker enters its sleep handshake. With 96 physical workers the trial attempts two. Outer shortens only the outer worker loop. Both also shortens idle search while a worker is helping inside a parallel operation. Pools with fewer than 64 workers retain the original limits. The prior wakeup protocol and job queues remain in use. A dispatch marker checked the actual nested and outer limits in each 96-worker process. Rayon library tests included concurrent callers and idle transitions on a 96-worker pool.

Rows come from four on-server screening blocks per shape on the same c7a.24xlarge. Each arm used three warmups and seven timed proofs with the same key, request and 96-core mask. The release control uses the matching shape configuration and its ordinary executable. Off, outer and both share the patched executable. The direct scheduler comparisons use off. Peak RSS includes setup. All 216 process exports from this screen passed independent reverification as part of the full catalog audit. This report counts those exports once across the two studies.

| Shape | Release control | Release / off / outer / both ms | Outer vs off | Both vs off | CPU-ms off / outer / both | Peak MiB off / outer / both |
|---|---|---:|---:|---:|---:|---:|
| 1x16 | coeff | 16.771 / 16.383 / 17.249 / 17.069 | +4.81% (0/4) | +3.32% (0/4) | 715.1 / 569.8 / 548.3 | 102.2 / 98.5 / 95.4 |
| 1x2 | server-primary | 8.029 / 8.090 / 8.212 / 8.138 | +1.75% (1/4) | +0.77% (1/4) | 375.4 / 313.8 / 298.4 | 43.8 / 38.6 / 45.4 |
| 1x4 | coeff | 12.067 / 12.244 / 12.934 / 12.293 | +3.94% (2/4) | +1.10% (2/4) | 567.0 / 431.8 / 409.0 | 64.8 / 66.1 / 61.0 |
| 1x8 | coeff | 14.410 / 13.728 / 14.300 / 13.810 | +4.95% (0/4) | +0.47% (2/4) | 616.6 / 472.6 / 449.0 | 79.0 / 80.4 / 80.5 |
| 2x16 | coeff | 21.901 / 21.645 / 28.596 / 28.738 | +32.22% (0/4) | +32.12% (0/4) | 1258.5 / 1450.4 / 1300.4 | 206.4 / 198.1 / 203.4 |
| 2x2 | front16 | 12.834 / 12.749 / 13.352 / 12.984 | +3.89% (0/4) | +0.95% (2/4) | 590.1 / 446.3 / 426.3 | 83.4 / 79.8 / 78.2 |
| 2x4 | coeff | 14.322 / 14.200 / 14.136 / 14.230 | -0.43% (2/4) | +1.06% (2/4) | 637.0 / 465.2 / 446.4 | 85.0 / 88.0 / 87.9 |
| 2x8 | coeff | 16.895 / 16.853 / 17.437 / 17.001 | +1.66% (1/4) | +1.15% (2/4) | 753.7 / 617.7 / 585.7 | 126.9 / 120.9 / 124.0 |
| 3x2 | coeff | 17.731 / 17.449 / 24.269 / 24.339 | +39.09% (0/4) | +38.94% (0/4) | 1081.1 / 1279.7 / 1149.9 | 173.4 / 156.9 / 151.0 |
| 3x4 | coeff | 17.717 / 17.930 / 24.643 / 24.732 | +37.58% (0/4) | +37.68% (0/4) | 1108.8 / 1317.7 / 1179.6 | 179.3 / 168.9 / 166.0 |
| 3x8 | coeff | 20.489 / 19.825 / 26.436 / 27.125 | +33.34% (0/4) | +36.34% (0/4) | 1196.2 / 1395.3 / 1245.0 | 203.6 / 189.2 / 202.1 |
| 4x16 | coeff | 24.842 / 24.930 / 31.200 / 30.776 | +25.92% (0/4) | +24.11% (0/4) | 1409.0 / 1559.0 / 1387.3 | 262.8 / 241.7 / 245.4 |
| 4x2 | coeff | 18.200 / 18.653 / 25.171 / 25.341 | +34.45% (0/4) | +36.68% (0/4) | 1148.2 / 1341.3 / 1195.0 | 198.9 / 188.6 / 174.3 |
| 4x4 | coeff | 19.557 / 19.959 / 25.528 / 25.938 | +27.90% (0/4) | +30.05% (0/4) | 1200.5 / 1370.3 / 1218.9 | 213.2 / 189.9 / 183.1 |
| 4x8 | coeff | 20.946 / 21.070 / 27.290 / 27.317 | +29.53% (0/4) | +30.50% (0/4) | 1251.3 / 1447.9 / 1286.4 | 240.9 / 220.8 / 215.4 |
| 5x16 | coeff | 26.648 / 26.623 / 36.549 / 38.733 | +37.09% (0/4) | +44.75% (0/4) | 1540.7 / 1959.7 / 1897.4 | 315.1 / 291.9 / 288.2 |
| 5x2 | coeff | 19.433 / 20.047 / 26.170 / 26.880 | +31.01% (0/4) | +33.77% (0/4) | 1214.5 / 1391.5 / 1266.3 | 217.6 / 214.0 / 203.4 |
| 5x4 | coeff | 21.023 / 21.329 / 26.810 / 26.968 | +27.08% (0/4) | +27.33% (0/4) | 1254.0 / 1410.0 / 1283.5 | 233.4 / 232.0 / 223.9 |

The complete screen selected a scheduler variant on 3 shapes. The selection file records their eight fresh confirmation blocks against prepared Aeglos in the final catalog.

[Runtime changes](artifacts/runtime.patch) · [Test results](artifacts/build.json) · [Screen measurements](artifacts/screen-rows.json.gz) · [Selected configurations in confirmation](artifacts/selected-confirmation.json) · [Full catalog](../2026-11-19-loss-routes689/README.md)
