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

## Same Aeglos executable on Ryzen and archived C7a

Both hosts used the same Aeglos executable, key, request and public output digest. Each timed request reported a prepared-key cache hit. The archived runner used nine blocks at each of 32 or 96 physical C7a cores. This run used five blocks at 32 hardware threads on 16 physical Ryzen cores. All took seven timed requests after three warmups. The three Aeglos columns here are arithmetic means so the contribution from MSM time is additive. The final column gives the fresh local Zopro median minus the supplied M5 CPU target. Host architecture, physical core count and runtime conditions all differ. The AWS measurements predate the mapped-image candidate.

| Shape | Aeglos mean ms Ryzen 32 / C7a 32 / C7a 96 | Ryzen 32 minus C7a 32 ms | MSM contribution to difference, ms | Current Ryzen Zopro above 10× CPU target, ms |
|---|---:|---:|---:|---:|
| 1x2 | 12.953 / 12.153 / 8.312 | +0.800 | +1.741 | +3.524 |
| 2x2 | 21.093 / 19.700 / 12.061 | +1.393 | +2.501 | +3.922 |
| 4x2 | 38.178 / 30.875 / 18.397 | +7.303 | +8.038 | +4.710 |
| 8x2 | 86.067 / 54.928 / 31.304 | +31.139 | +31.402 | +5.057 |
| 12x2 | 185.187 / 86.559 / 49.190 | +98.628 | +93.042 | +3.026 |
| 16x2 | 240.842 / 103.146 / 56.869 | +137.696 | +131.968 | +6.071 |
| 24x2 | 460.015 / 171.193 / 87.882 | +288.822 | +263.958 | +11.241 |
| merge-24x1 | 445.646 / 166.008 / 84.578 | +279.638 | +253.722 | +11.179 |
| merge-54x1 | 1002.530 / 348.025 / 173.308 | +654.505 | +652.160 | — |

All 135 local public outputs, proof hashes and verifier receipts were checked again. The archived verifier and host methodology are documented in the [prior comparison](../2026-11-19-matched-engines531/README.md). [Hash identities and stage means](artifacts/matched-hosts.json)
