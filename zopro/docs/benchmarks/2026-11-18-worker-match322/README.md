# Integrated frontend pool and Aeglos worker configuration on C7a

At 64 physical cores the integrated Zopro binary with 16 frontend workers took 8.464 ms and 54 MiB peak RSS on 1×2, versus 9.462 ms disabled. The paired gain was 10.88% with 7/7 wins and 7/7 process means at or below 9 ms. The previously measured enabled executable took 8.385 ms. Aeglos with 64 native workers and a Go limit of 64 on that same affinity mask took 9.076 ms and 447 MiB. At 96 cores with 96 native and 96 Go workers Aeglos took 7.990 ms.

All cells have seven randomized process means. The Aeglos setting gives its native count; the Go limit matches except where stated. M5 ratios use the supplied CPU and Metal timings.

| Shape | Affinity cores | Setting | Median ms | CPU ms | Peak MiB | M5 CPU / Metal | At CPU target |
|---|---:|---|---:|---:|---:|---:|---:|
| 12x2 | 48 | w18 | 32.387 | 1205 | 667 | 8.77× / 3.12× | 0/7 |
| 12x2 | 48 | w19 | 32.770 | 1207 | 657 | 8.67× / 3.08× | 0/7 |
| 12x2 | 56 | w18 | 32.626 | 1367 | 650 | 8.70× / 3.10× | 0/7 |
| 12x2 | 56 | w19 | 32.317 | 1363 | 673 | 8.79× / 3.13× | 0/7 |
| 16x2 | 48 | general-both | 56.350 | 1053 | 817 | 5.94× / 2.20× | 0/7 |
| 16x2 | 48 | general-off | 36.217 | 1365 | 724 | 9.25× / 3.42× | 0/7 |
| 16x2 | 48 | h19 | 35.963 | 1359 | 784 | 9.32× / 3.45× | 0/7 |
| 16x2 | 48 | hoff | 36.144 | 1396 | 782 | 9.27× / 3.43× | 0/7 |
| 16x2 | 56 | h19 | 36.263 | 1550 | 789 | 9.24× / 3.42× | 0/7 |
| 16x2 | 56 | hoff | 36.628 | 1593 | 799 | 9.15× / 3.39× | 0/7 |
| 16x2 | 64 | h19 | 36.656 | 1743 | 775 | 9.14× / 3.38× | 0/7 |
| 16x2 | 64 | hoff | 37.304 | 1798 | 788 | 8.98× / 3.32× | 0/7 |
| 1x2 | 64 | aeg48 | 9.150 | 364 | 439 | 4.04× / 2.62× | 0/7 |
| 1x2 | 64 | aeg64 | 9.076 | 423 | 447 | 4.08× / 2.64× | 0/7 |
| 1x2 | 64 | aeg96 | 8.934 | 447 | 483 | 4.14× / 2.69× | 0/7 |
| 1x2 | 64 | aeg96-go64 | 8.935 | 443 | 484 | 4.14× / 2.69× | 0/7 |
| 1x2 | 64 | integral-both | 9.152 | 297 | 53 | 4.04× / 2.62× | 0/7 |
| 1x2 | 64 | integral-four | 9.747 | 389 | 66 | 3.80× / 2.46× | 0/7 |
| 1x2 | 64 | integral-front | 8.431 | 274 | 53 | 4.39× / 2.85× | 0/7 |
| 1x2 | 64 | integral-off | 9.435 | 374 | 65 | 3.92× / 2.54× | 0/7 |
| 1x2 | 64 | pool0 | 9.462 | 376 | 71 | 3.91× / 2.54× | 0/7 |
| 1x2 | 64 | pool16 | 8.464 | 275 | 54 | 4.37× / 2.84× | 0/7 |
| 1x2 | 64 | pool24 | 8.482 | 283 | 55 | 4.36× / 2.83× | 0/7 |
| 1x2 | 64 | pool32 | 8.477 | 293 | 53 | 4.36× / 2.83× | 0/7 |
| 1x2 | 64 | pool8 | 8.991 | 268 | 51 | 4.12× / 2.67× | 0/7 |
| 1x2 | 64 | retained16 | 8.385 | 273 | 56 | 4.41× / 2.86× | 0/7 |
| 1x2 | 96 | aeg96 | 7.990 | 511 | 479 | 4.63× / 3.00× | 0/7 |
| 1x2 | 96 | integral-both | 9.099 | 411 | 50 | 4.07× / 2.64× | 0/7 |
| 1x2 | 96 | integral-four | 9.966 | 571 | 58 | 3.71× / 2.41× | 0/7 |
| 1x2 | 96 | integral-front | 10.046 | 406 | 55 | 3.68× / 2.39× | 0/7 |
| 1x2 | 96 | integral-off | 10.736 | 622 | 60 | 3.45× / 2.24× | 0/7 |
| 1x2 | 96 | pool0 | 10.738 | 621 | 63 | 3.45× / 2.24× | 0/7 |
| 1x2 | 96 | pool16 | 10.131 | 405 | 53 | 3.65× / 2.37× | 0/7 |
| 1x2 | 96 | retained16 | 10.167 | 408 | 54 | 3.64× / 2.36× | 0/7 |
| 24x2 | 48 | h19 | 51.286 | 1980 | 1245 | 10.30× / 3.43× | 6/7 |
| 24x2 | 48 | hoff | 51.921 | 2037 | 1254 | 10.17× / 3.39× | 6/7 |
| 24x2 | 56 | h19 | 50.728 | 2170 | 1200 | 10.41× / 3.47× | 7/7 |
| 24x2 | 56 | hoff | 51.920 | 2246 | 1275 | 10.17× / 3.39× | 5/7 |
| 24x2 | 64 | h19 | 50.825 | 2406 | 1247 | 10.39× / 3.46× | 7/7 |
| 24x2 | 64 | hoff | 52.668 | 2504 | 1261 | 10.03× / 3.34× | 4/7 |
| 2x2 | 48 | integral-both | 13.658 | 351 | 99 | 4.32× / 2.20× | 0/7 |
| 2x2 | 48 | integral-four | 13.972 | 427 | 232 | 4.22× / 2.15× | 0/7 |
| 2x2 | 48 | integral-front | 14.300 | 342 | 99 | 4.13× / 2.10× | 0/7 |
| 2x2 | 48 | pool0 | 14.346 | 417 | 232 | 4.11× / 2.09× | 0/7 |
| 2x2 | 48 | pool16 | 14.089 | 342 | 101 | 4.19× / 2.13× | 0/7 |
| 8x2 | 48 | general-both | 30.135 | 681 | 436 | 6.27× / 2.46× | 0/7 |
| 8x2 | 48 | general-off | 24.496 | 965 | 443 | 7.72× / 3.02× | 0/7 |
| merge-24x1 | 48 | h19 | 54.750 | 1953 | 1113 | 9.26× / 3.20× | 0/7 |
| merge-24x1 | 48 | hoff | 54.522 | 2003 | 1112 | 9.30× / 3.21× | 0/7 |
| merge-24x1 | 56 | h19 | 53.741 | 2153 | 1135 | 9.43× / 3.26× | 0/7 |
| merge-24x1 | 56 | hoff | 54.945 | 2226 | 1138 | 9.23× / 3.18× | 0/7 |
| merge-24x1 | 64 | h19 | 54.338 | 2403 | 1134 | 9.33× / 3.22× | 0/7 |
| merge-24x1 | 64 | hoff | 54.843 | 2452 | 1172 | 9.24× / 3.19× | 0/7 |

[Primary comparison](artifacts/primary.json) · [Settings](artifacts/settings.json) · [Process and request data](artifacts/processes.json.gz) · [Proof verification](artifacts/verification.json)

## Why the 96-core result warrants a separate test

The experimental executable lets us change G2 partitioning while holding frontend workers, affinity and the rest of the binary fixed. The ordinary G2 path uses two point ranges and 12-bit windows. The alternate path uses four ranges and 11-bit windows. Each cell below uses 16 frontend workers.

| Physical cores | Ordinary G2, ms | Alternate G2, ms | Paired change in latency | Faster blocks |
|---:|---:|---:|---:|---:|
| 64 | 8.431 | 9.152 | +8.08% | 0/7 |
| 96 | 10.046 | 9.099 | -9.99% | 7/7 |

The change reduced latency in every block at 96 cores and increased it in every block at 64. The best 96-core experimental setting took 9.099 ms against Aeglos at 7.990 ms. At 64 cores the integrated Zopro setting also beat the fastest tested Aeglos worker configuration on that mask (96 native workers, 8.934 ms). A repeat of the G2 comparison on a fresh host would test whether the difference persists before introducing a worker-count policy.
