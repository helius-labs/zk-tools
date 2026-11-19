# Running the final grouped G2 windows on a dedicated pool

For keys with at least 65,536 B2 points, the integrated prover can group equal scalars and split the distinct coefficients as it reduces their points on the main pool. This experiment moves the final window reduction over 1,024–65,535 surviving coefficients to the existing frontend pool. It keeps the grouping and scalar split on the main pool. The current witness determines the window workspace reservation before the pool receives the work. The ordinary grouped path handles missing headroom. Neither path stores an additional point table.

Every configuration ran with the same sixteen physical Ryzen cores. The integrated baseline and same-binary shared control assigned sixteen main workers and an overlapping four-worker frontend. The other configurations assigned ten main and six frontend workers, or twelve main and four frontend workers. Each layout ran with and without moving the final G2 windows. The separate probes recorded the actual number of distinct coefficients, pool width and reserved bytes. Zero-budget cases on 2×16, 3×8 and 4×16 exercised the fallback. Four blocks of eleven timed proofs followed three primes per process, with separate second requests for 2×16, 3×8 and 4×16. The initial workflow contained the 11 catalog inputs and three zero-budget cases. A separate four-block workflow completed the three second transactions. All 361 exports independently verified.

Moving the final grouped windows to six cores won 54 of 56 pairs against the idle ten-core control. The original sixteen-core integrated profile still had lower median latency on every one of the 14 inputs. For example, at 2×16 the 10+6 split increased paired latency by 12.33% on the catalog request and 12.07% on the second request. This differs from moving the whole exclusive G2 query on smaller keys. The earlier profile remains the local choice for these grouped shapes.

| Shape | Request | Main + auxiliary | Integrated / shared / idle / active ms | Active vs idle | Active vs integrated | Peak MiB integrated / active |
|---|---|---|---:|---:|---:|---:|
| 2x16 | catalog | 10+6 | 18.905 / 18.925 / 23.761 / 21.250 | -10.67% (4/4) | +12.33% (0/4) | 207.9 / 169.4 |
| 2x16 | catalog | 12+4 | 18.905 / 18.925 / 21.388 / 26.140 | +22.16% (0/4) | +37.35% (0/4) | 207.9 / 180.0 |
| 2x16 | heldout | 10+6 | 19.032 / 18.833 / 23.769 / 21.193 | -9.88% (4/4) | +12.07% (0/4) | 198.3 / 169.9 |
| 2x16 | heldout | 12+4 | 19.032 / 18.833 / 21.311 / 26.787 | +25.63% (0/4) | +41.55% (0/4) | 198.3 / 179.5 |
| 3x2 | catalog | 10+6 | 15.237 / 15.095 / 19.093 / 18.526 | -3.41% (4/4) | +21.59% (0/4) | 164.5 / 145.0 |
| 3x2 | catalog | 12+4 | 15.237 / 15.095 / 17.277 / 21.638 | +25.25% (0/4) | +41.48% (0/4) | 164.5 / 164.0 |
| 3x4 | catalog | 10+6 | 16.066 / 15.997 / 20.119 / 19.459 | -4.69% (4/4) | +21.11% (0/4) | 174.2 / 174.2 |
| 3x4 | catalog | 12+4 | 16.066 / 15.997 / 18.107 / 22.025 | +21.55% (1/4) | +37.59% (0/4) | 174.2 / 156.5 |
| 3x8 | catalog | 10+6 | 17.901 / 17.698 / 22.155 / 21.460 | -2.87% (4/4) | +20.17% (0/4) | 190.5 / 168.8 |
| 3x8 | catalog | 12+4 | 17.901 / 17.698 / 20.166 / 25.118 | +24.61% (0/4) | +40.57% (0/4) | 190.5 / 178.8 |
| 3x8 | heldout | 10+6 | 17.713 / 17.569 / 22.192 / 20.539 | -7.45% (4/4) | +15.85% (0/4) | 202.7 / 163.5 |
| 3x8 | heldout | 12+4 | 17.713 / 17.569 / 20.007 / 24.720 | +23.67% (0/4) | +40.09% (0/4) | 202.7 / 182.7 |
| 4x16 | catalog | 10+6 | 23.323 / 23.141 / 29.210 / 26.722 | -9.09% (4/4) | +12.30% (1/4) | 263.8 / 216.1 |
| 4x16 | catalog | 12+4 | 23.323 / 23.141 / 26.334 / 31.385 | +19.29% (0/4) | +34.57% (0/4) | 263.8 / 222.6 |
| 4x16 | heldout | 10+6 | 23.284 / 23.221 / 29.391 / 26.474 | -9.46% (4/4) | +13.76% (0/4) | 244.1 / 210.8 |
| 4x16 | heldout | 12+4 | 23.284 / 23.221 / 26.423 / 31.697 | +19.64% (0/4) | +35.53% (0/4) | 244.1 / 216.5 |
| 4x2 | catalog | 10+6 | 17.412 / 17.395 / 21.883 / 21.640 | -1.12% (2/4) | +21.68% (0/4) | 208.0 / 185.5 |
| 4x2 | catalog | 12+4 | 17.412 / 17.395 / 19.881 / 24.605 | +24.33% (0/4) | +39.19% (0/4) | 208.0 / 181.2 |
| 4x4 | catalog | 10+6 | 18.171 / 18.106 / 23.014 / 21.511 | -6.97% (4/4) | +18.39% (0/4) | 188.8 / 173.2 |
| 4x4 | catalog | 12+4 | 18.171 / 18.106 / 20.687 / 25.257 | +21.80% (0/4) | +38.73% (0/4) | 188.8 / 174.5 |
| 4x8 | catalog | 10+6 | 20.014 / 20.040 / 25.370 / 24.312 | -3.51% (4/4) | +21.57% (0/4) | 221.8 / 189.9 |
| 4x8 | catalog | 12+4 | 20.014 / 20.040 / 22.660 / 27.745 | +22.51% (0/4) | +38.71% (0/4) | 221.8 / 198.8 |
| 5x16 | catalog | 10+6 | 27.566 / 27.357 / 34.378 / 32.541 | -6.99% (4/4) | +18.16% (0/4) | 327.0 / 265.2 |
| 5x16 | catalog | 12+4 | 27.566 / 27.357 / 31.118 / 37.663 | +20.45% (0/4) | +36.76% (0/4) | 327.0 / 285.6 |
| 5x2 | catalog | 10+6 | 20.124 / 20.096 / 25.199 / 22.985 | -9.38% (4/4) | +13.41% (0/4) | 218.2 / 197.0 |
| 5x2 | catalog | 12+4 | 20.124 / 20.096 / 22.759 / 27.522 | +19.54% (0/4) | +34.85% (0/4) | 218.2 / 196.2 |
| 5x4 | catalog | 10+6 | 20.900 / 20.783 / 26.180 / 24.765 | -5.28% (4/4) | +18.44% (0/4) | 223.9 / 195.5 |
| 5x4 | catalog | 12+4 | 20.900 / 20.783 / 23.600 / 29.050 | +22.98% (0/4) | +39.27% (0/4) | 223.9 / 201.2 |

[Implementation](artifacts/source.patch) · [Dispatch and reserved bytes](artifacts/routes.json) · [Measurements](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
