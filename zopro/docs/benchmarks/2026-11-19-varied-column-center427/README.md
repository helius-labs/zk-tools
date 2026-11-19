# Local Euler weighting with packed FFT inputs on eight 12×2 transfer witnesses

The 16-worker profile uses 18 signed H windows with one range. The 32-worker profile uses ordinary H. Both use compact Poseidon outputs and FFT input packing. The enabled and disabled measurements use the same production-shaped executable. The predecessor is the exact preceding release. Three shuffled fresh-process blocks per witness, width and setting timed 21 verified requests each. Ratios compare process means within each matched block.

| Threads | Witness | Predecessor ms | Disabled ms | Enabled ms | Paired vs predecessor | Paired vs disabled |
|---:|---:|---:|---:|---:|---:|---:|
| 16 | 1 | 36.177 | 37.063 | 36.482 | +0.84% | -1.20% |
| 16 | 2 | 36.579 | 36.252 | 36.015 | -1.17% | -0.50% |
| 16 | 3 | 37.185 | 36.911 | 36.578 | -0.52% | -0.90% |
| 16 | 4 | 36.729 | 36.796 | 36.210 | -1.41% | -0.85% |
| 16 | 5 | 36.662 | 36.849 | 36.427 | -0.64% | -0.67% |
| 16 | 6 | 36.708 | 36.691 | 36.185 | -1.42% | -1.96% |
| 16 | 7 | 36.725 | 36.632 | 36.060 | -2.20% | -1.56% |
| 16 | 8 | 36.390 | 36.494 | 36.412 | +0.18% | -0.47% |
| 32 | 1 | 32.913 | 32.553 | 32.185 | -0.81% | -0.65% |
| 32 | 2 | 33.704 | 33.054 | 32.235 | -4.36% | -3.07% |
| 32 | 3 | 33.161 | 32.738 | 32.204 | -4.38% | -2.68% |
| 32 | 4 | 32.314 | 32.971 | 32.242 | -0.08% | -2.07% |
| 32 | 5 | 32.668 | 32.865 | 32.544 | -0.45% | -0.06% |
| 32 | 6 | 33.453 | 33.118 | 32.253 | -3.59% | -1.41% |
| 32 | 7 | 32.754 | 33.472 | 32.485 | -0.82% | -1.99% |
| 32 | 8 | 32.568 | 35.962 | 32.183 | -1.44% | -10.51% |

| Threads | Mode | Median ms across witnesses | Median CPU-ms | Maximum peak MiB |
|---:|---|---:|---:|---:|
| 16 | release | 36.685 | 532.9 | 551.0 |
| 16 | off | 36.743 | 535.3 | 546.5 |
| 16 | on | 36.311 | 528.8 | 560.5 |
| 32 | release | 32.833 | 906.7 | 682.8 |
| 32 | off | 33.012 | 909.9 | 691.2 |
| 32 | on | 32.239 | 891.5 | 676.2 |

At 16 workers the enabled path changed paired latency by -0.91% against the predecessor (6/8 witnesses faster) and -0.88% against the identical disabled build (8/8). This profile qualified.

At 32 workers the enabled path changed paired latency by -1.13% against the predecessor (8/8 witnesses faster) and -2.03% against the identical disabled build (8/8). This profile qualified.

The opt-in implementation passed 153 native core tests and 106 portable core tests after integration. [Integration checks and source hashes](artifacts/integration.json)

[Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json) · [All process means and timings](artifacts/processes.json.gz) · [Decision](artifacts/decision.json) · [Prototype source and oracle](../2026-11-19-fft-column-center423/README.md)
