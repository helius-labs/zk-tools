# Indexed grouping across reference and held-out transfers

Seven randomized blocks use the same binaries and public requests across settings. Every comparison uses full proof latency. Peaks cover startup, priming and all timed proofs.

| Shape | Input | Prior / ordinary / indexed ms | Indexed vs ordinary / prior | Peak prior / ordinary / indexed MiB | Indexed vs M5 CPU / Metal (10× CPU blocks) |
|---|---|---:|---:|---:|---:|
| 4x2 | Original | 15.119 / 15.368 / 14.863 | -3.14% (6/7) / -2.51% (5/7) | 338.5 / 342.8 / 310.2 | 7.33× / 3.23× (0/7) |
| 4x2 | 153 | 15.563 / 15.683 / 15.229 | -2.99% (7/7) / -1.22% (6/7) | 358.5 / 348.2 / 323.0 | — |
| 4x2 | 154 | 15.738 / 16.049 / 15.004 | -6.40% (6/7) / -3.00% (6/7) | 345.8 / 364.2 / 308.8 | — |
| 4x2 | 156 | 15.721 / 15.682 / 15.160 | -3.36% (6/7) / -2.09% (7/7) | 342.5 / 339.5 / 322.5 | — |
| 12x2 | Original | 28.651 / 28.835 / 28.672 | -1.41% (5/7) / -0.00% (4/7) | 649.7 / 663.1 / 599.6 | 9.91× / 3.52× (1/7) |
| 12x2 | 58 | 28.484 / 28.622 / 28.644 | +0.22% (2/7) / +0.17% (2/7) | 659.0 / 644.0 / 615.8 | — |
| 12x2 | 61 | 28.705 / 28.554 / 28.562 | +0.38% (3/7) / +0.04% (3/7) | 656.0 / 653.9 / 603.2 | — |
| 12x2 | 139 | 45.019 / 44.985 / 43.384 | -3.96% (7/7) / -3.72% (7/7) | 1193.5 / 1169.3 / 839.3 | — |

All 168 exports verified independently. [Protocol](protocol.md) · [Samples](artifacts/samples.json.gz) · [Verification](artifacts/verification.json)
