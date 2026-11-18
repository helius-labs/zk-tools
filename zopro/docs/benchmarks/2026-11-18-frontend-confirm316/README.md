# Independent C7a frontend pool replication and full circuit panel

On 1×2 with 64 physical cores, the retained 16-worker frontend pool took 8.444 ms versus 9.470 ms disabled, with 7/7 paired wins and 7/7 process means at or below 9 ms. The independent replication passed. The previous host measured 8.492 ms. Aeglos ran in 9.004 ms at 64 cores and 7.960 ms at 96 cores on this host.

## Single-proof latency

Median of seven process means after three warmups. Peak RSS includes startup. Larger shapes compare a 48-core Zopro process with 96-core Aeglos; all rows state process affinity. M5 thresholds use the supplied CPU timings.

| Shape | Physical cores | Setting | Median ms | CPU ms | Peak MiB | M5 CPU / Metal | At CPU target |
|---|---:|---|---:|---:|---:|---:|---:|
| 12x2 | 48 | accepted | 33.109 | 1267 | 656 | 8.58× / 3.05× | 0/7 |
| 12x2 | 48 | front0 | 32.336 | 1255 | 658 | 8.78× / 3.12× | 0/7 |
| 12x2 | 48 | front16 | 32.353 | 1240 | 646 | 8.78× / 3.12× | 0/7 |
| 12x2 | 96 | aeglos | 48.773 | 4099 | 3730 | 5.82× / 2.07× | 0/7 |
| 16x2 | 48 | accepted | 37.191 | 1443 | 796 | 9.01× / 3.33× | 0/7 |
| 16x2 | 48 | combo-large | 36.045 | 1390 | 784 | 9.29× / 3.44× | 0/7 |
| 16x2 | 48 | fft-on | 36.100 | 1407 | 786 | 9.28× / 3.43× | 0/7 |
| 16x2 | 48 | front0 | 36.463 | 1422 | 783 | 9.19× / 3.40× | 0/7 |
| 16x2 | 48 | front16 | 36.465 | 1413 | 769 | 9.19× / 3.40× | 0/7 |
| 16x2 | 48 | mixed19 | 35.891 | 1361 | 781 | 9.33× / 3.45× | 0/7 |
| 16x2 | 48 | mixedoff | 36.216 | 1400 | 779 | 9.25× / 3.42× | 0/7 |
| 16x2 | 96 | aeglos | 56.650 | 4868 | 4626 | 5.91× / 2.19× | 0/7 |
| 1x2 | 48 | front0 | 9.820 | 302 | 60 | 3.77× / 2.44× | 0/7 |
| 1x2 | 48 | front16 | 9.151 | 246 | 61 | 4.04× / 2.62× | 0/7 |
| 1x2 | 56 | front0 | 9.623 | 333 | 64 | 3.84× / 2.49× | 0/7 |
| 1x2 | 56 | front16 | 8.414 | 250 | 54 | 4.40× / 2.85× | 0/7 |
| 1x2 | 64 | accepted | 10.973 | 400 | 119 | 3.37× / 2.19× | 0/7 |
| 1x2 | 64 | aeglos | 9.004 | 447 | 488 | 4.11× / 2.67× | 0/7 |
| 1x2 | 64 | canon-off | 9.474 | 389 | 65 | 3.91× / 2.53× | 0/7 |
| 1x2 | 64 | canon-on | 9.620 | 395 | 66 | 3.85× / 2.49× | 0/7 |
| 1x2 | 64 | front0 | 9.470 | 375 | 74 | 3.91× / 2.53× | 0/7 |
| 1x2 | 64 | front16 | 8.444 | 274 | 56 | 4.38× / 2.84× | 0/7 |
| 1x2 | 64 | front32 | 8.539 | 293 | 55 | 4.33× / 2.81× | 0/7 |
| 1x2 | 64 | repo-on | 9.450 | 374 | 67 | 3.92× / 2.54× | 0/7 |
| 1x2 | 72 | front0 | 9.773 | 428 | 69 | 3.79× / 2.46× | 0/7 |
| 1x2 | 72 | front16 | 9.023 | 304 | 55 | 4.10× / 2.66× | 0/7 |
| 1x2 | 96 | accepted | 12.421 | 692 | 111 | 2.98× / 1.93× | 0/7 |
| 1x2 | 96 | aeglos | 7.960 | 510 | 478 | 4.65× / 3.01× | 0/7 |
| 1x2 | 96 | canon-off | 11.072 | 705 | 67 | 3.34× / 2.17× | 0/7 |
| 1x2 | 96 | canon-on | 10.865 | 695 | 70 | 3.41× / 2.21× | 0/7 |
| 1x2 | 96 | front0 | 10.717 | 620 | 60 | 3.45× / 2.24× | 0/7 |
| 1x2 | 96 | front16 | 10.114 | 403 | 57 | 3.66× / 2.37× | 0/7 |
| 1x2 | 96 | front32 | 10.208 | 416 | 55 | 3.62× / 2.35× | 0/7 |
| 1x2 | 96 | repo-on | 10.690 | 620 | 58 | 3.46× / 2.25× | 0/7 |
| 24x2 | 48 | accepted | 53.268 | 2093 | 1181 | 9.91× / 3.30× | 3/7 |
| 24x2 | 48 | combo-large | 52.431 | 2025 | 1289 | 10.07× / 3.36× | 4/7 |
| 24x2 | 48 | fft-on | 53.629 | 2106 | 1209 | 9.85× / 3.28× | 2/7 |
| 24x2 | 48 | front0 | 54.271 | 2132 | 1250 | 9.73× / 3.24× | 1/7 |
| 24x2 | 48 | front16 | 54.225 | 2120 | 1193 | 9.74× / 3.25× | 0/7 |
| 24x2 | 48 | mixed19 | 50.702 | 1978 | 1174 | 10.41× / 3.47× | 7/7 |
| 24x2 | 48 | mixedoff | 52.761 | 2073 | 1222 | 10.01× / 3.34× | 4/7 |
| 24x2 | 96 | aeglos | 87.243 | 7405 | 6823 | 6.05× / 2.02× | 0/7 |
| 2x2 | 48 | accepted | 14.375 | 419 | 227 | 4.10× / 2.09× | 0/7 |
| 2x2 | 48 | front0 | 14.178 | 414 | 223 | 4.16× / 2.12× | 0/7 |
| 2x2 | 48 | front16 | 14.024 | 341 | 104 | 4.21× / 2.14× | 0/7 |
| 2x2 | 96 | aeglos | 11.641 | 823 | 811 | 5.07× / 2.58× | 0/7 |
| 4x2 | 48 | accepted | 18.199 | 652 | 285 | 5.99× / 2.64× | 0/7 |
| 4x2 | 48 | front0 | 18.062 | 642 | 280 | 6.03× / 2.66× | 0/7 |
| 4x2 | 48 | front16 | 17.851 | 632 | 265 | 6.11× / 2.69× | 0/7 |
| 4x2 | 96 | aeglos | 18.496 | 1379 | 1350 | 5.89× / 2.60× | 0/7 |
| 8x2 | 48 | accepted | 24.775 | 986 | 479 | 7.63× / 2.99× | 0/7 |
| 8x2 | 48 | front0 | 24.760 | 983 | 460 | 7.63× / 2.99× | 0/7 |
| 8x2 | 48 | front16 | 24.363 | 955 | 465 | 7.76× / 3.04× | 0/7 |
| 8x2 | 96 | aeglos | 31.372 | 2557 | 2507 | 6.02× / 2.36× | 0/7 |
| merge-24x1 | 48 | accepted | 55.219 | 2048 | 1135 | 9.18× / 3.17× | 0/7 |
| merge-24x1 | 48 | combo-large | 54.523 | 1996 | 1094 | 9.30× / 3.21× | 0/7 |
| merge-24x1 | 48 | fft-on | 54.878 | 2009 | 1097 | 9.24× / 3.19× | 0/7 |
| merge-24x1 | 48 | front0 | 55.457 | 2049 | 1115 | 9.14× / 3.16× | 0/7 |
| merge-24x1 | 48 | front16 | 56.643 | 2046 | 1114 | 8.95× / 3.09× | 0/7 |
| merge-24x1 | 48 | mixed19 | 53.093 | 1939 | 1112 | 9.55× / 3.30× | 0/7 |
| merge-24x1 | 48 | mixedoff | 54.301 | 2004 | 1115 | 9.34× / 3.22× | 0/7 |
| merge-24x1 | 96 | aeglos | 84.855 | 7217 | 6632 | 5.97× / 2.06× | 0/7 |

## Two simultaneous 1×2 clients

Both clients share the main pool, the frontend pool, and a loaded key. Throughput uses the time between the start and completion of the concurrent batch.

| Main workers | Frontend workers | Proofs/s | Mean request ms, median process | Peak MiB |
|---:|---:|---:|---:|---:|
| 64 | 0 | 141.32 | 13.869 | 206 |
| 64 | 16 | 139.03 | 14.285 | 152 |
| 64 | 32 | 140.04 | 14.147 | 171 |
| 96 | 0 | 128.70 | 15.339 | 204 |
| 96 | 16 | 127.59 | 15.310 | 147 |
| 96 | 32 | 126.03 | 15.698 | 168 |

[Primary comparison](artifacts/primary.json) · [Process and request data](artifacts/processes.json.gz) · [Protocol](artifacts/protocol.md) · [Proof verification](artifacts/verification.json)
