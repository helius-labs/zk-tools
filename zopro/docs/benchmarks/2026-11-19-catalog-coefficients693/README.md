# Public coefficient files on sixteen further transfer shapes

Eight generated public transactions per key supplied the training coefficients. Their amounts, assets, outputs, Merkle paths and blindings varied. A file stores a B2 position and field value only where all eight transactions share a nonzero coefficient. The existing offset API binds the file to the key, admits its buffers against the preparation budget and computes the exact point corrections. Fresh witnesses can depart from every recorded value.

The original catalog input and two other transactions per shape compared the ordinary executable with the file disabled and enabled. Both used the same main pool of 32 Ryzen logical workers, sixteen physical frontend cores and four serial G2 partitions where admitted. Four order-balanced blocks each timed eleven warm proofs after three primes. The table gives medians of process means and within-block percentage changes. Preparation time is separate. RSS includes setup. All 192 fixture and 384 timing exports passed independent verification.

| Shape | Request | File KiB | Off / on ms | Paired change (wins) | Peak MiB off / on | Preparation ms |
|---|---|---:|---:|---:|---:|---:|
| 2x4 | catalog | 185.1 | 11.021 / 10.844 | -1.60% (3/4) | 109.0 / 111.0 | 16.1 |
| 2x4 | 57 | 185.1 | 10.883 / 10.757 | -1.07% (3/4) | 113.2 / 126.2 | 16.3 |
| 2x4 | 60 | 185.1 | 11.000 / 10.740 | -2.37% (3/4) | 110.5 / 112.5 | 16.3 |
| 2x8 | catalog | 280.0 | 13.833 / 13.744 | -0.47% (2/4) | 146.0 / 151.0 | 22.9 |
| 2x8 | 57 | 280.0 | 14.124 / 13.671 | -2.81% (4/4) | 146.0 / 144.2 | 23.1 |
| 2x8 | 60 | 280.0 | 13.978 / 13.740 | -1.37% (4/4) | 150.2 / 140.0 | 23.0 |
| 3x2 | catalog | 201.6 | 13.834 / 13.545 | -2.94% (4/4) | 203.5 / 199.0 | 18.7 |
| 3x2 | 57 | 201.6 | 13.838 / 13.537 | -2.48% (4/4) | 205.8 / 205.8 | 18.7 |
| 3x2 | 60 | 201.6 | 13.750 / 13.377 | -2.92% (4/4) | 208.2 / 208.2 | 18.6 |
| 3x4 | catalog | 238.7 | 14.379 / 14.079 | -2.23% (4/4) | 222.2 / 199.8 | 21.6 |
| 3x4 | 57 | 238.7 | 14.312 / 14.068 | -1.71% (4/4) | 215.2 / 210.0 | 21.3 |
| 3x4 | 60 | 238.7 | 14.326 / 14.271 | -0.85% (2/4) | 215.0 / 217.2 | 21.0 |
| 3x8 | catalog | 333.9 | 16.267 / 15.607 | -3.62% (4/4) | 264.0 / 234.4 | 27.7 |
| 3x8 | 57 | 333.9 | 16.240 / 15.759 | -2.31% (4/4) | 242.5 / 239.8 | 28.1 |
| 3x8 | 60 | 333.9 | 16.045 / 15.593 | -2.42% (3/4) | 247.5 / 243.2 | 27.7 |
| 4x2 | catalog | 255.2 | 15.207 / 15.017 | -1.25% (4/4) | 223.0 / 225.0 | 23.5 |
| 4x2 | 57 | 255.2 | 15.560 / 14.953 | -3.45% (4/4) | 229.0 / 223.0 | 23.4 |
| 4x2 | 60 | 255.2 | 15.235 / 14.976 | -1.35% (3/4) | 254.5 / 220.0 | 23.4 |
| 4x4 | catalog | 292.4 | 15.912 / 15.409 | -3.17% (4/4) | 259.2 / 232.5 | 26.1 |
| 4x4 | 57 | 292.4 | 15.986 / 15.499 | -3.13% (4/4) | 251.2 / 226.5 | 26.1 |
| 4x4 | 60 | 292.4 | 16.069 / 15.544 | -3.22% (4/4) | 258.0 / 242.5 | 26.0 |
| 4x8 | catalog | 387.9 | 17.612 / 17.170 | -2.96% (4/4) | 288.5 / 259.5 | 32.6 |
| 4x8 | 57 | 387.9 | 17.603 / 17.144 | -1.98% (4/4) | 297.5 / 270.7 | 33.0 |
| 4x8 | 60 | 387.9 | 17.764 / 17.211 | -2.40% (4/4) | 280.2 / 260.2 | 32.8 |
| 4x16 | catalog | 562.0 | 20.654 / 20.190 | -2.68% (4/4) | 338.2 / 291.8 | 44.7 |
| 4x16 | 57 | 562.0 | 21.005 / 20.072 | -4.44% (4/4) | 334.8 / 293.7 | 44.6 |
| 4x16 | 60 | 562.0 | 20.682 / 19.928 | -3.54% (4/4) | 315.5 / 306.2 | 45.0 |
| 5x2 | catalog | 319.5 | 17.492 / 17.007 | -2.86% (4/4) | 276.5 / 261.8 | 29.1 |
| 5x2 | 57 | 319.5 | 17.362 / 17.006 | -2.05% (3/4) | 274.0 / 278.8 | 29.0 |
| 5x2 | 60 | 319.5 | 17.267 / 16.819 | -2.64% (4/4) | 271.2 / 246.2 | 28.7 |
| 5x4 | catalog | 356.9 | 18.048 / 17.511 | -2.80% (3/4) | 279.2 / 264.0 | 31.5 |
| 5x4 | 57 | 356.9 | 18.347 / 17.415 | -4.45% (4/4) | 268.8 / 257.2 | 31.7 |
| 5x4 | 60 | 356.9 | 18.024 / 17.499 | -3.23% (3/4) | 278.5 / 267.2 | 31.9 |
| 5x8 | catalog | 452.6 | 21.400 / 20.858 | -2.80% (4/4) | 332.5 / 322.8 | 38.6 |
| 5x8 | 57 | 452.6 | 21.903 / 20.924 | -4.81% (4/4) | 348.2 / 320.1 | 38.3 |
| 5x8 | 60 | 452.6 | 21.449 / 21.048 | -2.21% (2/4) | 324.0 / 336.4 | 38.4 |
| 5x16 | catalog | 627.3 | 24.458 / 23.779 | -2.55% (4/4) | 387.2 / 356.6 | 50.4 |
| 5x16 | 57 | 627.3 | 24.579 / 23.610 | -3.94% (4/4) | 371.0 / 361.8 | 50.3 |
| 5x16 | 60 | 627.3 | 24.228 / 23.588 | -2.64% (4/4) | 371.0 / 360.1 | 50.4 |
| 6x2 | catalog | 373.2 | 20.914 / 20.499 | -1.77% (4/4) | 329.8 / 307.0 | 33.9 |
| 6x2 | 57 | 373.2 | 21.070 / 20.591 | -2.47% (4/4) | 329.2 / 299.0 | 34.2 |
| 6x2 | 60 | 373.2 | 21.027 / 20.247 | -3.48% (4/4) | 343.0 / 299.7 | 33.7 |
| 6x4 | catalog | 410.7 | 21.549 / 20.947 | -2.92% (4/4) | 345.2 / 327.9 | 36.2 |
| 6x4 | 57 | 410.7 | 21.614 / 21.108 | -2.34% (4/4) | 339.8 / 346.7 | 36.2 |
| 6x4 | 60 | 410.7 | 21.828 / 20.986 | -4.59% (3/4) | 342.5 / 312.4 | 36.3 |
| 6x8 | catalog | 506.7 | 23.489 / 22.557 | -3.68% (4/4) | 367.5 / 339.0 | 43.2 |
| 6x8 | 57 | 506.7 | 23.148 / 22.687 | -1.63% (3/4) | 360.5 / 373.3 | 43.0 |
| 6x8 | 60 | 506.7 | 23.420 / 22.619 | -3.53% (4/4) | 366.5 / 334.1 | 43.2 |

All 48 shape–request combinations had a lower median paired latency with the file enabled. The enabled setting won 176/192 paired blocks. Across the original 16 catalog cases, paired reductions ranged from 0.47% on 2×8 to 3.68% on 6×8. The files range from 185 to 627 KiB per key. Each enabled process also pays preparation time, reported above. The compressed coefficient files in the artifacts directory are named by shape, and the coverage file identifies their decompressed SHA-256 hashes.

[Wire coverage](artifacts/coverage.json) · [Fixture identities](artifacts/fixtures.json) · [Block measurements](artifacts/measurements.json.gz) · [Verification](artifacts/verification.json)
