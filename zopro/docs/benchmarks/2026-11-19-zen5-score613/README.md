# Nine-shape Ryzen comparison with native Zen 5 code generation

Each row includes five independently shuffled processes per engine and CPU target. Times are medians of complete verified request means. Peak RSS covers setup and all proof calls. The frozen experimental binary and both fresh builds use the same per-shape profile. The release and Aeglos retain their established profiles. Aeglos reported a prepared-key hit on every measured call.

| Shape | Release / frozen / Zen 4 / Zen 5 / Aeglos ms | Zen 5 vs Zen 4 | Fresh Zen 4 vs frozen | Peak Zen 4 / Zen 5 / Aeglos MiB | Zen 5 vs M5 CPU / Metal | 10× CPU target ms (processes passing) |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 7.075 / 7.058 / 7.079 / 7.066 / 12.864 | -0.05% (3/5) | +0.43% (2/5) | 66.0 / 64.8 / 427.2 | 5.24× / 3.40× | 3.7 (0/5) |
| 2x2 | 9.546 / 9.521 / 9.524 / 9.515 / 20.977 | -0.14% (4/5) | +0.29% (2/5) | 122.0 / 120.0 / 736.6 | 6.20× / 3.15× | 5.9 (0/5) |
| 4x2 | 15.327 / 15.241 / 15.172 / 15.416 / 38.065 | +1.65% (2/5) | -1.38% (4/5) | 294.0 / 301.2 / 1266.3 | 7.07× / 3.11× | 10.9 (0/5) |
| 8x2 | 24.390 / 23.387 / 23.552 / 23.846 / 86.087 | +1.64% (2/5) | +1.17% (2/5) | 501.0 / 531.0 / 2294.1 | 7.93× / 3.10× | 18.9 (0/5) |
| 12x2 | 31.097 / 28.572 / 28.779 / 28.681 / 240.903 | -0.81% (3/5) | -0.39% (3/5) | 651.9 / 660.8 / 3723.1 | 9.90× / 3.52× | 28.4 (1/5) |
| 16x2 | 41.080 / 37.964 / 37.645 / 37.015 / 241.724 | -1.36% (5/5) | -0.56% (4/5) | 749.2 / 776.3 / 4299.0 | 9.05× / 3.35× | 33.5 (0/5) |
| 24x2 | 65.116 / 59.856 / 60.831 / 60.216 / 460.555 | -0.68% (3/5) | +1.72% (1/5) | 1111.8 / 1065.4 / 6371.8 | 8.77× / 2.92× | 52.8 (0/5) |
| merge-24x1 | 64.215 / 61.851 / 63.092 / 62.622 / 443.174 | -0.55% (3/5) | +3.57% (1/5) | 1029.6 / 1024.5 / 6152.9 | 8.10× / 2.79× | 50.7 (0/5) |
| merge-54x1 | 156.185 / 153.778 / 156.218 / 155.489 / 1005.024 | -0.33% (3/5) | +1.63% (2/5) | 1714.7 / 1689.1 / 12551.7 | — | — |

The native target met 0 of eight supplied M5 CPU thresholds by median and led Aeglos on 9 of nine shapes. All 225 exported proofs verified independently. [Compiler flags and target features](artifacts/compiler.json) · [Raw samples](artifacts/samples.json.gz) · [Binary and input identities](artifacts/pins.json) · [Verification](artifacts/verification.json)
