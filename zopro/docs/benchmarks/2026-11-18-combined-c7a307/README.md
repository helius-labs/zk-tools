# C7a test of combined grouping, G2 windows, FFT input and H windows

Seven randomized fresh processes per configuration, three primes per process. Latency is the median of process means for sequential warm verified SDK requests. Every host export was re-verified locally against the pinned image and public inputs. Peak RSS includes setup. Aeglos used 96 cores for all shapes. The selected combined setting uses scalar diversity and four point ranges on the two smaller shapes, or direct FFT input and 13-bit dense H windows on the others. Each optimization retains its own eligibility rules.

| Shape | Zopro cores | Accepted ms | Combined ms | Aeglos ms | Peak MiB accepted / combined / Aeglos | M5 CPU / Metal speedup combined | Target ms | Blocks at target |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 96 | 12.458 | 9.713 | 7.891 | 109 / 63 / 473 | 3.81× / 2.47× | 3.7 | 0/7 |
| 1x2 | 64 | 11.013 | 9.652 | 7.891 | 125 / 70 / 473 | 3.83× / 2.49× | 3.7 | 0/7 |
| 2x2 | 48 | 14.111 | 13.693 | 11.490 | 291 / 294 / 828 | 4.31× / 2.19× | 5.9 | 0/7 |
| 4x2 | 48 | 17.961 | 17.938 | 18.073 | 274 / 282 / 1348 | 6.08× / 2.68× | 10.9 | 0/7 |
| 8x2 | 48 | 24.526 | 24.479 | 30.775 | 473 / 465 / 2466 | 7.72× / 3.02× | 18.9 | 0/7 |
| 12x2 | 48 | 32.895 | 32.219 | 48.384 | 652 / 683 / 3722 | 8.81× / 3.13× | 28.4 | 0/7 |
| 16x2 | 48 | 36.974 | 36.108 | 56.009 | 808 / 780 / 4614 | 9.28× / 3.43× | 33.5 | 0/7 |
| 24x2 | 48 | 52.933 | 52.540 | 86.125 | 1221 / 1258 / 6775 | 10.05× / 3.35× | 52.8 | 4/7 |
| merge-24x1 | 48 | 55.892 | 54.014 | 84.253 | 1094 / 1108 / 6621 | 9.39× / 3.24× | 50.7 | 0/7 |
| merge-54x1 | 96 | 122.505 | 119.267 | 171.768 | 1870 / 1818 / 12989 | — | — | — |

## Prespecified comparisons

| Shape / cores | Candidate | On / off / accepted ms | Paired change | Wins | Calibration | Pass |
|---|---|---:|---:|---:|---:|---|
| 1x2 / 96 | cert-on | 10.698 / 12.280 / 12.458 | -12.82% | 7/7 | -1.33% | True |
| 24x2 / 48 | fft-on | 51.884 / 52.911 / 52.933 | -1.16% | 4/7 | -0.76% | False |
| 1x2 / 96 | geom-split | 10.968 / 12.248 / 12.458 | -10.46% | 7/7 | -1.71% | True |
| 2x2 / 48 | geom-split | 13.629 / 14.125 / 14.111 | -3.12% | 7/7 | +0.10% | True |
| 1x2 / 96 | combo-g2 | 9.713 / 12.268 / 12.458 | -20.88% | 7/7 | -2.34% | True |
| 24x2 / 48 | combo-large | 52.540 / 53.193 / 52.933 | -1.39% | 6/7 | -0.62% | False |

## Measurements by setting

| Shape | Cores | Setting | Median ms | Median CPU ms | Peak MiB | 1×2 block means ≤9 ms |
|---|---:|---|---:|---:|---:|---:|
| 12x2 | 48 | accepted | 32.895 | 1262 | 652 | — |
| 12x2 | 48 | combo-large | 32.219 | 1223 | 683 | — |
| 12x2 | 48 | combo-off | 32.812 | 1256 | 658 | — |
| 12x2 | 96 | aeglos | 48.384 | 4075 | 3722 | — |
| 16x2 | 48 | accepted | 36.974 | 1434 | 808 | — |
| 16x2 | 48 | combo-large | 36.108 | 1392 | 780 | — |
| 16x2 | 48 | combo-off | 36.762 | 1426 | 750 | — |
| 16x2 | 96 | aeglos | 56.009 | 4834 | 4614 | — |
| 1x2 | 64 | accepted | 11.013 | 401 | 125 | 0/7 |
| 1x2 | 64 | cert-on | 9.398 | 373 | 67 | 0/7 |
| 1x2 | 64 | combo-g2 | 9.652 | 389 | 70 | 0/7 |
| 1x2 | 64 | combo-group | 9.390 | 374 | 66 | 0/7 |
| 1x2 | 64 | combo-off | 11.055 | 389 | 118 | 0/7 |
| 1x2 | 96 | accepted | 12.458 | 692 | 109 | 0/7 |
| 1x2 | 96 | aeglos | 7.891 | 505 | 473 | 7/7 |
| 1x2 | 96 | cert-off | 12.280 | 623 | 111 | 0/7 |
| 1x2 | 96 | cert-on | 10.698 | 618 | 61 | 0/7 |
| 1x2 | 96 | combo-g2 | 9.713 | 563 | 63 | 0/7 |
| 1x2 | 96 | combo-group | 10.656 | 622 | 61 | 0/7 |
| 1x2 | 96 | combo-off | 12.268 | 625 | 113 | 0/7 |
| 1x2 | 96 | combo-split | 10.845 | 600 | 115 | 0/7 |
| 1x2 | 96 | geom-off | 12.248 | 619 | 110 | 0/7 |
| 1x2 | 96 | geom-split | 10.968 | 592 | 114 | 0/7 |
| 1x2 | 96 | geom-tree | 10.881 | 587 | 114 | 0/7 |
| 24x2 | 48 | accepted | 52.933 | 2075 | 1221 | — |
| 24x2 | 48 | combo-direct | 52.328 | 2057 | 1228 | — |
| 24x2 | 48 | combo-h13 | 52.318 | 2062 | 1237 | — |
| 24x2 | 48 | combo-large | 52.540 | 2066 | 1258 | — |
| 24x2 | 48 | combo-off | 53.193 | 2085 | 1221 | — |
| 24x2 | 48 | fft-off | 52.911 | 2071 | 1250 | — |
| 24x2 | 48 | fft-on | 51.884 | 2048 | 1228 | — |
| 24x2 | 96 | aeglos | 86.125 | 7318 | 6775 | — |
| 2x2 | 48 | accepted | 14.111 | 411 | 291 | — |
| 2x2 | 48 | combo-g2 | 13.693 | 422 | 294 | — |
| 2x2 | 48 | combo-off | 14.040 | 409 | 297 | — |
| 2x2 | 48 | geom-off | 14.125 | 409 | 306 | — |
| 2x2 | 48 | geom-split | 13.629 | 419 | 301 | — |
| 2x2 | 48 | geom-tree | 13.838 | 420 | 309 | — |
| 2x2 | 96 | aeglos | 11.490 | 813 | 828 | — |
| 4x2 | 48 | accepted | 17.961 | 642 | 274 | — |
| 4x2 | 48 | combo-large | 17.938 | 634 | 282 | — |
| 4x2 | 48 | combo-off | 18.216 | 652 | 268 | — |
| 4x2 | 96 | aeglos | 18.073 | 1389 | 1348 | — |
| 8x2 | 48 | accepted | 24.526 | 978 | 473 | — |
| 8x2 | 48 | combo-large | 24.479 | 966 | 465 | — |
| 8x2 | 48 | combo-off | 24.827 | 984 | 458 | — |
| 8x2 | 96 | aeglos | 30.775 | 2559 | 2466 | — |
| merge-24x1 | 48 | accepted | 55.892 | 2050 | 1094 | — |
| merge-24x1 | 48 | combo-large | 54.014 | 1981 | 1108 | — |
| merge-24x1 | 48 | combo-off | 55.451 | 2029 | 1109 | — |
| merge-24x1 | 96 | aeglos | 84.253 | 7190 | 6621 | — |
| merge-54x1 | 96 | accepted | 122.505 | 6422 | 1870 | — |
| merge-54x1 | 96 | aeglos | 171.768 | 14774 | 12989 | — |
| merge-54x1 | 96 | combo-large | 119.267 | 6036 | 1818 | — |
| merge-54x1 | 96 | combo-off | 121.851 | 6211 | 1838 | — |

[Block and request data](artifacts/rows.json.gz) · [Prespecified checks](artifacts/checks.json) · [Protocol](artifacts/protocol.md) · [Settings](artifacts/settings.json) · [Verification](artifacts/verification.json)
