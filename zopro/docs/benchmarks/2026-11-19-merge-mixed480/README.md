# Merge offsets shared across two training populations

Eight public sparse-tree fixtures and eight populated-tree fixtures determined 9,468 common nonzero coefficients. The resulting file occupies 340,896 bytes. Eight new requests use 2 through 24 real inputs with 256–312 prior main-tree nullifier insertions. The original benchmark request supplies the ninth comparison. Both configurations use the same binary and local Euler FFT weighting at 32 Ryzen workers. Five randomized pairs per request each included three primes and eleven timed proofs.

| Input | Real coins | Baseline ms | File ms | Paired change | Faster blocks | File peak MiB |
|---|---:|---:|---:|---:|---:|---:|
| original | 24 | 63.890 | 64.014 | +0.91% | 2/5 | 993.1 |
| 129 | 2 | 65.544 | 65.818 | -0.98% | 3/5 | 1018.2 |
| 130 | 3 | 68.998 | 68.651 | -0.72% | 5/5 | 1094.9 |
| 131 | 4 | 69.268 | 69.650 | +0.55% | 1/5 | 1127.4 |
| 132 | 8 | 67.934 | 67.696 | -1.77% | 3/5 | 1050.7 |
| 133 | 12 | 70.575 | 70.924 | +0.37% | 2/5 | 1163.7 |
| 134 | 16 | 72.249 | 73.431 | +0.01% | 2/5 | 1168.8 |
| 135 | 20 | 71.532 | 70.363 | -0.16% | 3/5 | 1146.2 |
| 136 | 24 | 72.591 | 72.991 | -0.60% | 4/5 | 1162.5 |

The median paired change was -0.38% across eight held-out requests, with 5/8 favorable. The candidate missed the screen requiring improvement on the original in at least three blocks, a favorable median on held-out requests, gains on at least six of eight and a favorable 24-coin result. The 106 exports verified independently with Gnark. This experiment records a candidate screen. The [qualified scorecard](../2026-11-19-merge-deep479/README.md) remains the reference.

[Results and selection](artifacts/results.json) · [Samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)
