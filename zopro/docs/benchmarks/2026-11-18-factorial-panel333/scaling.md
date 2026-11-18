# Where 1×2 loses time at 96 C7a cores

The same integrated Zopro binary ran at 64 and 96 physical cores on one c7a.24xlarge. Both settings placed 16 frontend workers within that affinity mask. Each primary cell has seven fresh processes and 101 warm requests per process. CPU time is aggregate user plus system time. The final column divides per-process CPU time by elapsed time before taking the median.

| Cores | Implementation | Elapsed ms | CPU-ms per request | Average cores in use |
|---:|---|---:|---:|---:|
| 64 | Zopro | 8.444 | 273.5 | 32.5 |
| 96 | Zopro | 9.918 | 399.3 | 40.5 |
| 96 | Zopro four-range experiment | 9.070 | 411.0 | 45.3 |
| 64 | Aeglos | 8.753 | 428.0 | 48.8 |
| 96 | Aeglos | 7.877 | 515.7 | 65.5 |

## Phase timing

A separate diagnostic used the previously qualified experimental binary on the same host. Each cell has three plain and three traced processes of 51 requests. The columns are medians of process means, so they can differ slightly when summed. G2 is elapsed time inside the G2 scalar multiplication kernel (`g2_phi::raw_exclusive`), including work Rayon can schedule while a nested join waits. The interval after G2 ends when the canonical proving core returns.

| Cores | G2 ranges | Plain ms | Traced ms | Core entry to FFT | FFT | FFT end to G2 start | G2 | G2 end to core return | Outside core |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 64 | 2 | 8.455 | 8.648 | 2.056 | 0.817 | 0.513 | 4.048 | 0.021 | 1.201 |
| 64 | 4 | 9.320 | 9.507 | 2.063 | 0.821 | 0.507 | 4.906 | 0.019 | 1.184 |
| 72 | 2 | 8.629 | 8.771 | 2.080 | 0.815 | 0.515 | 4.134 | 0.019 | 1.216 |
| 72 | 4 | 8.905 | 9.058 | 2.058 | 0.824 | 0.516 | 4.424 | 0.018 | 1.210 |
| 96 | 2 | 10.343 | 10.227 | 2.142 | 0.844 | 0.509 | 5.129 | 0.534 | 1.249 |
| 96 | 4 | 8.979 | 9.090 | 2.166 | 0.836 | 0.505 | 3.960 | 0.336 | 1.290 |

From 64 to 96 cores with ordinary G2, the G2 interval grew by 1.08 ms and the interval after G2 grew by 0.51 ms. The FFT changed by 0.03 ms. At 96 cores, four ranges shortened G2 to 3.96 ms, while 0.34 ms still elapsed between G2 finishing and the core returning. These measurements locate the regression. Distinguishing extra bucket work, worker migration, and time waiting for another query needs worker-level evidence.

[Primary timings](same-host.md) · [Eight-shape comparison](README.md) · [Per-process interval data](artifacts/trace-segments.json)
