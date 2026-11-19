# Small public coefficient files for four wide-output shapes

Eight generated public transactions per key supplied the training coefficients. Their amounts, assets, outputs, Merkle paths and blindings varied. A file stores a B2 position and field value only where all eight transactions share a nonzero coefficient. The existing offset API binds the file to the key, admits its buffers against the preparation budget and computes the exact point corrections. Fresh witnesses can depart from every recorded value.

The original catalog input and four other transactions per shape compared the ordinary executable with the file disabled and enabled. Both used the same main pool of 32 Ryzen logical workers, sixteen physical frontend cores and four serial G2 partitions where admitted. Four order-balanced blocks each timed eleven warm proofs after three primes. The table gives medians of process means and within-block percentage changes. Preparation time is separate. RSS includes setup. All 48 fixture and 160 timing exports passed independent verification.

| Shape | Request | File KiB | Off / on ms | Paired change (wins) | Peak MiB off / on | Preparation ms |
|---|---|---:|---:|---:|---:|---:|
| 1x16 | catalog | 388.7 | 13.454 / 13.156 | -2.35% (4/4) | 135.5 / 124.6 | 29.4 |
| 1x16 | 57 | 388.7 | 13.562 / 13.193 | -2.34% (4/4) | 125.5 / 127.2 | 29.5 |
| 1x16 | 58 | 388.7 | 13.513 / 13.101 | -2.89% (4/4) | 128.8 / 119.9 | 29.8 |
| 1x16 | 59 | 388.7 | 13.602 / 13.265 | -2.30% (4/4) | 125.0 / 125.6 | 29.4 |
| 1x16 | 60 | 388.7 | 13.639 / 13.430 | -2.13% (3/4) | 126.8 / 122.5 | 29.5 |
| 2x16 | catalog | 453.0 | 17.533 / 16.859 | -3.81% (4/4) | 268.5 / 241.6 | 35.0 |
| 2x16 | 57 | 453.0 | 17.442 / 16.902 | -3.15% (3/4) | 264.5 / 241.1 | 35.3 |
| 2x16 | 58 | 453.0 | 17.625 / 16.855 | -4.06% (4/4) | 255.8 / 257.1 | 35.3 |
| 2x16 | 59 | 453.0 | 17.365 / 16.896 | -3.19% (4/4) | 271.8 / 250.4 | 35.2 |
| 2x16 | 60 | 453.0 | 17.499 / 16.972 | -2.59% (4/4) | 252.8 / 253.3 | 35.1 |
| 1x8 | catalog | 216.3 | 10.630 / 10.295 | -2.64% (4/4) | 101.2 / 104.5 | 17.4 |
| 1x8 | 57 | 216.3 | 11.041 / 10.296 | -6.39% (4/4) | 96.5 / 102.5 | 17.3 |
| 1x8 | 58 | 216.3 | 11.099 / 10.335 | -6.33% (4/4) | 100.5 / 97.8 | 17.2 |
| 1x8 | 59 | 216.3 | 10.928 / 10.267 | -6.91% (4/4) | 102.2 / 98.2 | 17.2 |
| 1x8 | 60 | 216.3 | 11.009 / 10.411 | -5.80% (4/4) | 106.5 / 104.5 | 17.4 |
| 1x4 | catalog | 121.6 | 9.037 / 8.709 | -3.16% (4/4) | 84.2 / 85.5 | 10.5 |
| 1x4 | 57 | 121.6 | 8.836 / 8.773 | -1.30% (3/4) | 81.0 / 86.2 | 10.8 |
| 1x4 | 58 | 121.6 | 8.880 / 8.832 | -0.31% (3/4) | 82.0 / 93.0 | 10.5 |
| 1x4 | 59 | 121.6 | 8.883 / 8.666 | -2.31% (4/4) | 83.2 / 86.2 | 10.9 |
| 1x4 | 60 | 121.6 | 8.903 / 8.845 | -0.34% (4/4) | 82.0 / 89.2 | 10.6 |

Across the 20 catalog and held-out cases the enabled median was lower in all 20. At 1×16 it won 19/20 paired blocks, with a 388.7 KiB file and about 30 ms of one-time preparation on this CPU. At 1×4 some held-out gains were below 0.4% and process RSS increased on all five inputs. The 96-core AWS screen will measure the machine-specific effect.

The files are available for [1×4](artifacts/1x4.wo.gz), [1×8](artifacts/1x8.wo.gz), [1×16](artifacts/1x16.wo.gz) and [2×16](artifacts/2x16.wo.gz). Hashes in the coverage file refer to decompressed bytes.

[Wire coverage](artifacts/coverage.json) · [Fixture identities](artifacts/fixtures.json) · [Block measurements](artifacts/measurements.json.gz) · [Verification](artifacts/verification.json)
