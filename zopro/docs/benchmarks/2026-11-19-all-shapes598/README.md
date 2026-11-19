# Local full-proof scorecard for the frozen CPU prover

The candidate uses one SDK executable with the per-shape profiles fixed before measurement. Five fresh shuffled blocks compare the release, candidate and Aeglos on the identical reference request with matched CPU affinity. Aeglos reported a prepared-key hit on each measured call. Reported latency is the median of the five process means. Peak RSS includes setup.

| Shape | Workers | Release / candidate / Aeglos ms | Paired candidate vs release | Max candidate / Aeglos MiB | Candidate vs M5 CPU / Metal | 10× CPU target ms (processes passing) |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 32 | 7.020 / 7.001 / 12.773 | -0.21% (4/5) | 63.5 / 429.6 | 5.28× / 3.43× | 3.7 (0/5) |
| 2x2 | 32 | 9.640 / 9.546 / 20.767 | -0.99% (4/5) | 120.0 / 739.1 | 6.18× / 3.14× | 5.9 (0/5) |
| 4x2 | 32 | 15.606 / 15.045 / 38.086 | -3.63% (4/5) | 309.8 / 1278.1 | 7.25× / 3.19× | 10.9 (0/5) |
| 8x2 | 32 | 24.058 / 23.090 / 85.749 | -4.29% (5/5) | 511.8 / 2291.1 | 8.19× / 3.20× | 18.9 (0/5) |
| 12x2 | 30 | 31.019 / 28.444 / 240.264 | -7.63% (5/5) | 659.8 / 3698.2 | 9.98× / 3.55× | 28.4 (1/5) |
| 16x2 | 32 | 39.577 / 37.254 / 240.589 | -6.09% (5/5) | 741.4 / 4312.8 | 8.99× / 3.33× | 33.5 (0/5) |
| 24x2 | 32 | 63.490 / 58.859 / 459.627 | -8.15% (5/5) | 1138.1 / 6378.0 | 8.97× / 2.99× | 52.8 (0/5) |
| merge-24x1 | 32 | 63.323 / 61.341 / 444.498 | -1.60% (5/5) | 1030.9 / 6189.4 | 8.27× / 2.85× | 50.7 (0/5) |
| merge-54x1 | 32 | 155.873 / 154.196 / 1006.422 | -0.61% (3/5) | 1703.3 / 12586.4 | — | — |

0 of eight supplied references reached their 10× CPU target in this run. Merge-54×1 has no supplied M5 reference. All 135 exported proofs verified independently. [Protocol and profiles](protocol.md) · [Samples](artifacts/samples.json.gz) · [Identities](artifacts/pins.json) · [Verification](artifacts/verification.json)
