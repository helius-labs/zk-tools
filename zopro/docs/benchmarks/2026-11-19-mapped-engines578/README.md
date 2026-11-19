# Verified local timings with the optional preconverted merge key

The current experimental SDK uses the preconverted image for merge-24×1. The other shapes use their ordinary Euler images and existing profile settings in the same executable. The comparison includes the accepted ordinary release and the retained Aeglos CPU executable. All three run on the same Ryzen 9950X with the same 32-worker affinity, inputs, three warmups and seven timed verified requests. Five shuffled blocks compare the full warm request. Peak RSS includes loading and setup; all measured Aeglos requests report a cache hit. All exported proofs are independently verified using the original images and previously pinned public digests.

| Shape | Accepted Zopro / current Zopro / Aeglos ms | Current paired vs accepted | Current paired vs Aeglos | Max process peak MiB current / Aeglos | Current vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| 1x2 | 7.254 / 7.224 / 12.861 | -0.51% (3/5) | -43.99% (5/5) | 61.2 / 423.0 | 5.12× / 3.32× |
| 2x2 | 9.745 / 9.822 / 21.063 | +0.49% (2/5) | -53.76% (5/5) | 104.8 / 725.3 | 6.01× / 3.05× |
| 4x2 | 15.509 / 15.610 / 38.192 | +0.70% (2/5) | -59.04% (5/5) | 241.5 / 1261.3 | 6.98× / 3.07× |
| 8x2 | 23.880 / 23.957 / 86.072 | +0.32% (2/5) | -72.17% (5/5) | 398.2 / 2286.5 | 7.89× / 3.09× |
| 12x2 | 31.393 / 31.426 / 184.078 | +0.40% (2/5) | -83.06% (5/5) | 529.7 / 3431.5 | 9.04× / 3.21× |
| 16x2 | 39.883 / 39.571 / 240.916 | -0.46% (3/5) | -83.55% (5/5) | 620.0 / 4289.5 | 8.47× / 3.13× |
| 24x2 | 64.031 / 64.041 / 458.921 | -1.26% (3/5) | -86.15% (5/5) | 958.3 / 6355.9 | 8.24× / 2.75× |
| merge-24x1 | 62.774 / 61.879 / 445.227 | -1.27% (5/5) | -86.06% (5/5) | 952.0 / 6141.9 | 8.19× / 2.83× |
| merge-54x1 | 156.743 / 156.745 / 1003.091 | -1.22% (3/5) | -84.37% (5/5) | 1650.8 / 12467.3 | — |

0 of eight supplied references meet the 10× M5 CPU target by median latency in this run. Merge-54×1 has no supplied M5 reference. All 135 exports passed independent verification. [Samples](artifacts/samples.json.gz) · [Pinned configuration](artifacts/config.json) · [Runner](artifacts/run.py) · [Verification](artifacts/verification.json)
