# Ordinary SDK grouped MSM with both execution orders

Each latency contrast has three parent-first and three indexed-first blocks. Individual process means cover eleven full SDK calls. Peak RSS is the maximum over the six latency processes of each variant and input. Percentages are within-block latency changes from the parent.

| Shape | Request | Parent / indexed ms | Paired change | Parent first / indexed first | Peak parent / indexed MiB | Indexed vs M5 CPU / Metal |
|---|---|---:|---:|---:|---:|---:|
| 1x2 | Original | 6.985 / 7.063 | +1.18% (0/6) | +1.10% / +1.25% | 61.8 / 60.0 | 5.24× / 3.40× |
| 2x2 | Original | 9.590 / 9.599 | +0.08% (3/6) | -0.22% / +0.65% | 113.8 / 106.0 | 6.15× / 3.13× |
| 4x2 | Original | 15.537 / 15.221 | -1.72% (5/6) | -1.64% / -2.08% | 249.5 / 249.0 | 7.16× / 3.15× |
| 8x2 | Original | 24.015 / 24.019 | -0.37% (3/6) | -0.79% / +0.05% | 425.0 / 435.5 | 7.87× / 3.08× |
| 12x2 | Original | 31.492 / 30.803 | -2.64% (5/6) | -2.59% / -2.69% | 568.3 / 522.3 | 9.22× / 3.28× |
| 16x2 | Original | 39.808 / 39.446 | +0.65% (2/6) | +0.75% / -1.35% | 672.6 / 647.9 | 8.49× / 3.14× |
| 24x2 | Original | 63.911 / 62.615 | -1.70% (6/6) | -1.98% / -1.41% | 1048.0 / 990.7 | 8.43× / 2.81× |
| merge-24x1 | Original | 64.102 / 63.436 | -1.67% (5/6) | -0.19% / -1.90% | 1018.5 / 1022.7 | 7.99× / 2.76× |
| merge-54x1 | Original | 157.825 / 152.989 | -3.11% (6/6) | -4.67% / -1.62% | 1699.2 / 1699.9 | — |
| 4x2 | 153 | 15.864 / 15.435 | -2.70% (6/6) | -2.69% / -2.72% | 262.0 / 252.8 | — |
| 4x2 | 154 | 15.714 / 15.650 | -0.72% (5/6) | -1.14% / -0.08% | 255.5 / 247.5 | — |
| 4x2 | 156 | 16.151 / 15.562 | -4.91% (6/6) | -6.14% / -1.45% | 265.5 / 258.2 | — |
| 12x2 | 139 | 47.872 / 45.363 | -5.68% (6/6) | -5.34% / -6.21% | 999.6 / 757.6 | — |

Each of the 208 exports verified, including 52 independent profile exports. [Protocol](protocol.md) · [Order-specific statistics](artifacts/paired.json) · [Samples and CPU profiles](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
