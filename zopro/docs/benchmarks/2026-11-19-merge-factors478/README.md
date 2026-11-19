# Local Euler weighting and public wire offsets together on merge-24×1

The training and held-out sets each use 2, 3, 4, 8, 12, 16, 20 and 24 real inputs. The main nullifier tree contains only its sentinel. Eight public training inputs determined a key-bound offset file. Eight separate requests and the original were each run through the same 32-worker Ryzen checkout with both options off, offsets alone, local FFT weighting alone and both together. Five randomized matched blocks per input and configuration each used three primes and eleven timed full SDK calls.

| Input | Real coins | Neither / file / Euler / both ms | File vs neither | Both vs Euler | Both vs file |
|---:|---:|---:|---:|---:|---:|
| Original | Reference | 65.716 / 62.880 / 63.587 / 60.450 | -4.31% (5/5) | -4.17% (5/5) | -2.40% (5/5) |
| 113 | 2 | 65.071 / 61.124 / 62.760 / 59.023 | -6.26% (4/5) | -6.54% (5/5) | -3.44% (5/5) |
| 114 | 3 | 62.250 / 60.859 / 60.268 / 58.156 | -0.46% (4/5) | -3.94% (5/5) | -4.87% (5/5) |
| 115 | 4 | 65.930 / 62.106 / 62.876 / 59.172 | -5.82% (5/5) | -6.53% (5/5) | -4.77% (5/5) |
| 116 | 8 | 67.921 / 62.390 / 65.313 / 60.024 | -8.62% (4/5) | -9.32% (5/5) | -3.95% (5/5) |
| 117 | 12 | 64.012 / 61.282 / 62.630 / 60.839 | -3.59% (4/5) | -4.45% (4/5) | -0.51% (5/5) |
| 118 | 16 | 69.206 / 63.643 / 68.705 / 62.340 | -7.32% (5/5) | -9.26% (5/5) | -3.50% (4/5) |
| 119 | 20 | 70.149 / 64.479 / 67.871 / 62.898 | -8.08% (5/5) | -7.04% (5/5) | -1.92% (5/5) |
| 120 | 24 | 65.640 / 62.673 / 64.768 / 60.832 | -4.54% (5/5) | -6.08% (5/5) | -2.94% (5/5) |

Against the profile selected in the [varied Euler qualification](../2026-11-19-larger-varied473/README.md), the file changed paired latency by -6.53% across the held-out inputs, with 8/8 faster. The original measured 60.450 ms on that file-enabled profile. The combination passed the latency screen. All 196 exports passed independent Gnark verification. With an optional preparation allowance of 0, 1, 4, 8 or 16 MiB, the prover used the ordinary path. It admitted the file at 32 and 64 MiB. The seven additional exports also passed Gnark verification. The file occupies 8,507,424 bytes and took 586.22 ms to prepare per key. Performance on populated nullifier trees is evaluated separately before accepting a profile.

[Results](artifacts/score.json) · [Samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)
