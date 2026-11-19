# Locating the input-sensitive 1×2 delay on C7a

The two largest positive changes from the catalog transaction to the second transaction were **after curves (+2.279 ms)** and **curve span (+0.872 ms)**. The paired median change for the whole traced SDK request was +3.218 ms. In the G1 call the interval from canonical entry to raw entry changed by -0.917 ms and the raw evaluation changed by +2.091 ms.

The original 24 warm traces came from four paired blocks on six transactions. Every capture contained eighteen complete SDK invocations, including three warmups, one solver call, one FFT call, one canonical G1 call with a nested raw evaluation and one exclusive G2 call per invocation. Every trace had zero lost records. Seven consecutive intervals partition each of the 360 timed SDK invocations exactly. The curve span starts at the earlier G1 or G2 entry and ends at the later return. Other concurrently executing work may outlive those observed calls. The interval after the curves ends at the SDK return and can include uninstrumented proving work, synchronization and verification. The nested G1 and G2 rows describe portions of the curve span. Median differences use within-block process means and need not add across rows. All proof and public hashes matched the previously verified exports.

| Input | Interval | Traced ms | Paired difference from catalog ms | Min to max block difference ms |
|---|---|---:|---:|---:|
| catalog | before solver | 0.371 | +0.000 | +0.000 … +0.000 |
| catalog | solver | 1.650 | +0.000 | +0.000 … +0.000 |
| catalog | between solver fft | 0.239 | +0.000 | +0.000 … +0.000 |
| catalog | fft | 0.825 | +0.000 | +0.000 … +0.000 |
| catalog | before curves | 0.024 | +0.000 | +0.000 … +0.000 |
| catalog | curve span | 4.754 | +0.000 | +0.000 … +0.000 |
| catalog | after curves | 0.905 | +0.000 | +0.000 … +0.000 |
| catalog | sdk | 8.775 | +0.000 | +0.000 … +0.000 |
| catalog | g1 to raw | 1.181 | +0.000 | +0.000 … +0.000 |
| catalog | g1 raw | 2.289 | +0.000 | +0.000 … +0.000 |
| catalog | g1 return | 0.001 | +0.000 | +0.000 … +0.000 |
| catalog | g2 | 4.747 | +0.000 | +0.000 … +0.000 |
| heldout | before solver | 0.372 | +0.001 | -0.018 … +0.009 |
| heldout | solver | 1.623 | -0.024 | -0.089 … +0.053 |
| heldout | between solver fft | 0.236 | -0.003 | -0.026 … +0.002 |
| heldout | fft | 0.836 | +0.013 | -0.017 … +0.047 |
| heldout | before curves | 0.024 | +0.000 | -0.000 … +0.001 |
| heldout | curve span | 5.690 | +0.872 | +0.018 … +1.583 |
| heldout | after curves | 3.215 | +2.279 | +1.852 … +3.046 |
| heldout | sdk | 11.967 | +3.218 | +2.840 … +3.461 |
| heldout | g1 to raw | 0.265 | -0.917 | -0.926 … -0.876 |
| heldout | g1 raw | 4.369 | +2.091 | +1.853 … +2.563 |
| heldout | g1 return | 0.001 | -0.000 | -0.000 … +0.000 |
| heldout | g2 | 5.661 | +0.850 | -0.204 … +1.343 |
| v25 | before solver | 0.375 | +0.003 | +0.002 … +0.004 |
| v25 | solver | 1.654 | +0.008 | -0.088 … +0.050 |
| v25 | between solver fft | 0.228 | -0.015 | -0.034 … +0.001 |
| v25 | fft | 0.835 | +0.013 | -0.033 … +0.050 |
| v25 | before curves | 0.024 | -0.000 | -0.001 … +0.001 |
| v25 | curve span | 5.428 | +0.703 | +0.194 … +1.153 |
| v25 | after curves | 3.457 | +2.567 | +2.014 … +3.213 |
| v25 | sdk | 12.061 | +3.206 | +3.102 … +3.569 |
| v25 | g1 to raw | 0.266 | -0.915 | -0.925 … -0.876 |
| v25 | g1 raw | 4.547 | +2.377 | +1.862 … +2.607 |
| v25 | g1 return | 0.001 | +0.000 | -0.000 … +0.000 |
| v25 | g2 | 5.262 | +0.399 | +0.060 … +0.841 |
| v26 | before solver | 0.372 | +0.001 | -0.003 … +0.006 |
| v26 | solver | 1.641 | -0.022 | -0.061 … +0.068 |
| v26 | between solver fft | 0.231 | -0.009 | -0.031 … +0.002 |
| v26 | fft | 0.810 | -0.006 | -0.021 … +0.001 |
| v26 | before curves | 0.024 | +0.000 | -0.001 … +0.001 |
| v26 | curve span | 5.621 | +0.756 | +0.601 … +1.380 |
| v26 | after curves | 3.199 | +2.263 | +1.965 … +2.706 |
| v26 | sdk | 11.936 | +3.167 | +2.931 … +3.317 |
| v26 | g1 to raw | 0.268 | -0.914 | -0.922 … -0.873 |
| v26 | g1 raw | 4.706 | +2.432 | +2.272 … +2.958 |
| v26 | g1 return | 0.001 | +0.000 | -0.000 … +0.000 |
| v26 | g2 | 5.212 | +0.524 | +0.121 … +0.891 |
| v27 | before solver | 0.374 | -0.000 | -0.011 … +0.010 |
| v27 | solver | 1.612 | -0.041 | -0.088 … +0.033 |
| v27 | between solver fft | 0.234 | -0.006 | -0.012 … -0.003 |
| v27 | fft | 0.819 | -0.007 | -0.025 … +0.033 |
| v27 | before curves | 0.024 | +0.001 | -0.002 … +0.001 |
| v27 | curve span | 5.501 | +0.869 | +0.315 … +1.544 |
| v27 | after curves | 3.440 | +2.536 | +1.689 … +2.940 |
| v27 | sdk | 12.068 | +3.228 | +3.155 … +3.515 |
| v27 | g1 to raw | 0.266 | -0.915 | -0.928 … -0.872 |
| v27 | g1 raw | 4.553 | +2.382 | +1.848 … +2.591 |
| v27 | g1 return | 0.001 | +0.000 | -0.000 … +0.000 |
| v27 | g2 | 5.438 | +0.760 | +0.297 … +1.492 |
| v28 | before solver | 0.375 | +0.004 | -0.007 … +0.007 |
| v28 | solver | 1.667 | +0.018 | -0.066 … +0.102 |
| v28 | between solver fft | 0.227 | -0.012 | -0.038 … -0.002 |
| v28 | fft | 0.847 | +0.033 | -0.041 … +0.056 |
| v28 | before curves | 0.024 | -0.000 | -0.001 … +0.000 |
| v28 | curve span | 5.360 | +0.587 | +0.233 … +0.952 |
| v28 | after curves | 3.651 | +2.710 | +2.414 … +3.001 |
| v28 | sdk | 12.132 | +3.324 | +3.127 … +3.514 |
| v28 | g1 to raw | 0.265 | -0.915 | -0.928 … -0.873 |
| v28 | g1 raw | 4.615 | +2.310 | +1.953 … +2.588 |
| v28 | g1 return | 0.001 | +0.000 | -0.000 … +0.000 |
| v28 | g2 | 5.236 | +0.425 | +0.075 … +0.728 |

[Original controlled full-proof times](../2026-11-19-varied-phases716/README.md) · [Request intervals and trace hashes](artifacts/intervals.json.gz)
