# Fuse FFT input packing with the first five inverse stages

The Rust candidate reads A and B into a 32-vector column, computes five inverse butterflies per vector lane and writes the transformed column into the packed buffer. Inputs past the supplied polynomial length use zeroes. Existing root tables and the remaining transform stay in use. The optional path keeps one full-size buffer. Probes on a byte-identical copy counted three calls for each enabled grain and zero when disabled on three timed 12×2 proofs per setting. The native oracle compared every output lane against the existing inverse on unequal input lengths, zeros, and full inputs.

## 12×2 at 32 hardware threads

| Columns per task | Screen ms |
|---|---:|
| off | 33.285 |
| release | 33.374 |
| 16 | 33.531 |
| 64 | 32.913 |

Seven fresh matched blocks confirm the selected grain of 64 columns. Each process times 31 verified requests after three warmups. Paired changes compare process means from the same randomized block.

| Workers | Mode | Median ms | CPU-ms | Peak MiB | M5 CPU / Metal |
|---:|---|---:|---:|---:|---:|
| 16 | release | 38.222 | 565.0 | 546.0 | 7.43× / 2.64× |
| 16 | off | 38.536 | 565.3 | 558.8 | 7.37× / 2.62× |
| 16 | 64 | 37.886 | 556.1 | 547.2 | 7.50× / 2.67× |
| 32 | release | 33.274 | 920.6 | 728.2 | 8.54× / 3.04× |
| 32 | off | 33.454 | 919.4 | 696.5 | 8.49× / 3.02× |
| 32 | 64 | 32.965 | 904.8 | 700.8 | 8.62× / 3.06× |

| Workers | Paired vs release | Wins / 7 | Paired vs disabled | Wins / 7 |
|---:|---:|---:|---:|---:|
| 16 | -0.61% | 5 | -1.59% | 5 |
| 32 | -0.97% | 6 | -1.36% | 6 |

The 32-thread candidate met both prespecified comparisons.

## Shape coverage

One fresh process per row with eleven timed requests. These are coverage checks on the candidate executable. They do not establish per-shape gains.

| Shape | ms | Peak MiB | M5 CPU / Metal ms | CPU / Metal speedup |
|---|---:|---:|---:|---:|
| 1x2 | 7.753 | 60.2 | 37.0 / 24.0 | 4.77× / 3.10× |
| 2x2 | 10.586 | 105.8 | 59.0 / 30.0 | 5.57× / 2.83× |
| 4x2 | 15.571 | 258.5 | 109.0 / 48.0 | 7.00× / 3.08× |
| 8x2 | 24.064 | 407.0 | 189.0 / 74.0 | 7.85× / 3.08× |
| 12x2 | 33.363 | 591.2 | 284.0 / 101.0 | 8.51× / 3.03× |
| 16x2 | 40.215 | 687.8 | 335.0 / 124.0 | 8.33× / 3.08× |
| 24x2 | 67.495 | 1071.5 | 528.0 / 176.0 | 7.82× / 2.61× |
| merge-24x1 | 65.520 | 980.6 | 507.0 / 175.0 | 7.74× / 2.67× |

[Candidate source](artifacts/source.patch) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json) · [Process samples](artifacts/processes.json.gz) · [Decision](artifacts/decision.json)
