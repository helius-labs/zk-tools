# Linux allocator fault and CPU costs

The same 240 verified processes recorded warm minor faults, kernel CPU time and total CPU time. Each row shows the median per proof across five process blocks. The four entries in each cell correspond to tuned glibc, stock glibc, jemalloc and mimalloc. Kernel CPU time sums work from all process threads and is included in total CPU time.

| Shape | Request | Build | Minor faults per proof | Kernel CPU-ms per proof | Total CPU-ms per proof |
|---|---|---|---:|---:|---:|
| 1x2 | Original | Release | 79 / 1309 / 275 / 580 | 3.2 / 5.0 / 3.7 / 5.2 | 146.9 / 146.1 / 146.4 / 158.5 |
| 2x2 | Original | Release | 161 / 2517 / 581 / 1043 | 6.1 / 11.3 / 6.2 / 9.2 | 193.4 / 195.9 / 199.2 / 222.9 |
| 4x2 | Original | Release | 2094 / 2029 / 4228 / 4010 | 16.0 / 18.3 / 28.5 / 27.1 | 413.8 / 413.4 / 421.0 / 518.1 |
| 8x2 | Original | Release | 3303 / 3279 / 13184 / 6588 | 27.7 / 27.6 / 74.2 / 47.2 | 667.1 / 667.0 / 708.4 / 773.6 |
| 12x2 | Original | Release | 3832 / 11935 / 27731 / 6527 | 20.2 / 64.6 / 136.9 / 30.9 | 869.7 / 880.9 / 944.8 / 952.8 |
| 16x2 | Original | Release | 4318 / 23154 / 45629 / 12069 | 24.7 / 145.3 / 309.6 / 67.4 | 1135.2 / 1222.5 / 1372.5 / 1319.2 |
| 24x2 | Original | Release | 5608 / 27772 / 73714 / 41383 | 37.1 / 117.4 / 510.7 / 221.0 | 1835.5 / 1860.6 / 2182.9 / 2142.0 |
| merge-24x1 | Original | Release | 6871 / 29946 / 72341 / 44035 | 46.7 / 125.8 / 491.5 / 214.6 | 1812.7 / 1802.4 / 2143.6 / 2121.8 |
| 12x2 | Original | Grouped | 2602 / 16257 / 26712 / 6475 | 18.4 / 87.6 / 129.6 / 35.7 | 814.4 / 842.4 / 874.1 / 862.3 |
| 24x2 | Original | Grouped | 6168 / 34881 / 73142 / 34694 | 36.2 / 172.9 / 506.5 / 167.5 | 1701.2 / 1732.0 / 2008.4 / 1913.1 |
| 24x2 | 103 | Grouped | 6851 / 28473 / 72953 / 35626 | 35.6 / 118.4 / 496.9 / 171.2 | 1655.4 / 1645.5 / 1981.7 / 1878.0 |
| 12x2 | 142 | Grouped | 8658 / 21839 / 35804 / 10471 | 30.0 / 94.7 / 160.4 / 53.0 | 1239.8 / 1258.8 / 1309.9 / 1360.2 |

On original grouped 24×2, stock glibc incurred 34,881 minor faults per proof and used 172.9 kernel CPU-ms. The tuned settings gave 6,169 faults and 36.2 kernel CPU-ms. Total CPU time changed by only 30.8 ms while elapsed time changed by 5.7 ms. Jemalloc had the highest kernel CPU cost on this request. The profiles differ in several allocator policies, and these observations do not assign a causal latency saving to an individual fault or setting.

[Full-proof latency and memory](README.md) · [Raw warm counters](artifacts/samples.json.gz)
