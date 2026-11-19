# Independent confirmation of repeat-distance grouping

The all-shape screen favored 16×2 on both its original request and a second request. This trial tests four more 16×2 requests and the original in seven fresh blocks using the identical clean experimental binary. The original and request 75 run both on sixteen physical cores and on the full 32 hardware threads. Three 4×2 requests test the smaller signal seen on its original input. Each cell includes an ordinary build setting, its candidate setting, and the integrated production release. Five earlier blocks are excluded from these medians. Each process uses three warmup proofs and eleven timed proofs.

| Shape | Request | Workers | Release / ordinary / candidate ms | Paired vs ordinary | Paired vs release | Ordinary / candidate peak MiB | Original vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|---:|---:|
| 4x2 | Original | 32 | 15.378 / 15.259 / 15.358 | +0.27% (2/7) | -0.12% (4/7) | 272.2 / 266.2 | 7.10× / 3.13× |
| 16x2 | Original | 16 | 43.723 / 43.483 / 43.321 | -0.57% (4/7) | -1.01% (5/7) | 579.5 / 568.8 | 7.73× / 2.86× |
| 16x2 | Original | 32 | 38.780 / 39.620 / 38.963 | -2.51% (5/7) | -0.36% (4/7) | 638.3 / 638.4 | 8.60× / 3.18× |
| 16x2 | 73 | 32 | 39.352 / 39.325 / 39.155 | -1.01% (4/7) | -0.34% (4/7) | 681.7 / 672.1 | — |
| 16x2 | 75 | 16 | 43.701 / 43.814 / 43.837 | +0.13% (3/7) | +0.51% (2/7) | 580.2 / 588.9 | — |
| 16x2 | 75 | 32 | 39.751 / 39.634 / 39.190 | -1.98% (6/7) | -1.41% (6/7) | 657.4 / 662.3 | — |
| 16x2 | 78 | 32 | 39.984 / 39.349 / 39.460 | -0.20% (5/7) | -1.88% (5/7) | 660.5 / 663.2 | — |
| 16x2 | 80 | 32 | 39.697 / 39.192 / 39.322 | -0.23% (4/7) | -0.95% (6/7) | 665.9 / 691.0 | — |
| 4x2 | 37 | 32 | 15.589 / 15.726 / 15.639 | -0.02% (4/7) | +0.78% (2/7) | 280.2 / 278.0 | — |
| 4x2 | 40 | 32 | 15.574 / 15.778 / 15.828 | +0.15% (3/7) | +2.11% (2/7) | 266.0 / 269.0 | — |

All 210 exported proofs verified independently. [Samples](artifacts/samples.json.gz) · [Proof identities](artifacts/verification.json)
