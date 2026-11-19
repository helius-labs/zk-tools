# Mapping first-level group representatives through a compact index slice

On 12×2 the times are prior / dense / prior view / mapped in milliseconds. The sampled 2×2 experiment uses prior sampled / ordinary / current sampled. Comparisons use seven pairs per request and the same affinity within each pair. RSS is the highest process peak.

| Shape | Input | Upper group input | Full proof medians ms | Selected vs ordinary control / prior | Peak control / selected MiB | Selected vs M5 CPU / Metal |
|---|---|---|---:|---:|---:|---:|
| 12x2 | Original | copied | 28.637 / 28.544 / 28.429 / 28.375 | -0.87% (5/7) / -0.28% (4/7) | 624.4 / 582.8 | 10.01× / 3.56× |
| 2x2 | Original | — | 9.419 / 9.607 / 9.437 | -1.78% (7/7) / +0.18% (3/7) | 117.5 / 121.5 | 6.25× / 3.18× |
| 12x2 | 57 | copied | 28.343 / 28.423 / 28.164 / 28.274 | -0.61% (5/7) / -0.40% (6/7) | 621.0 / 569.8 | — |
| 12x2 | 139 | borrowed | 44.654 / 44.854 / 44.647 / 45.020 | +0.95% (1/7) / +0.64% (1/7) | 1133.0 / 1163.4 | — |
| 12x2 | 143 | copied | 44.689 / 45.069 / 44.938 / 45.160 | +0.37% (2/7) / +1.12% (2/7) | 1113.4 / 1156.5 | — |

All 137 exports verified independently. The ordinary 2×2 control uses the ordinary key and original H routing. The two sampled settings use the centered key. [Protocol](protocol.md) · [Implementation relative to the sampled H build](artifacts/source.patch) · [Samples and group routes](artifacts/samples.json.gz) · [Verification](artifacts/verification.json)
