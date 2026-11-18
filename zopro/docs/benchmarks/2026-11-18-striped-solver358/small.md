# Matched 1×2 results on C7a

All cells use the same C7a host, witness and verifier. The 96-core Aeglos cell uses 96 native workers and GOMAXPROCS of 96. Zopro uses a 16-worker frontend inside each process affinity mask. The ordinary release uses two G2 ranges. Four-serial and all three grouping experiments use four G2 ranges with serial scalar decomposition. Times are the medians of seven process means, each timing 61 requests after three primes.

| Prover / setting | Physical cores | Wall ms | Aggregate CPU-ms/proof | Peak MiB | Relative to M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| ordinary | 32 | 10.893 | 222.3 | 63 | 3.40× / 2.20× |
| ordinary | 64 | 8.510 | 279.3 | 51 | 4.35× / 2.82× |
| ordinary | 96 | 10.250 | 413.1 | 51 | 3.61× / 2.34× |
| four-serial | 96 | 8.592 | 387.8 | 49 | 4.31× / 2.79× |
| stripe-off | 96 | 8.546 | 408.2 | 48 | 4.33× / 2.81× |
| stripe-one | 96 | 8.591 | 409.5 | 51 | 4.31× / 2.79× |
| stripe-two | 96 | 8.560 | 407.5 | 50 | 4.32× / 2.80× |
| aeglos | 96 | 7.982 | 509.7 | 471 | 4.64× / 3.01× |

| 96-core configuration | Comparison | Paired wall change | Faster blocks |
|---|---|---:|---:|
| ordinary | ordinary at 32 | -6.20% | 7/7 |
| ordinary | ordinary at 64 | +20.12% | 0/7 |
| four-serial | ordinary at 64 | +0.76% | 1/7 |
| four-serial | ordinary at 96 | -16.23% | 7/7 |
| four-serial | aeglos at 96 | +7.52% | 0/7 |
| stripe-one | stripe-off at 96 | +0.24% | 2/7 |
| stripe-two | stripe-off at 96 | +0.13% | 1/7 |
| stripe-one | four-serial at 96 | -0.02% | 4/7 |
| stripe-two | four-serial at 96 | -0.16% | 4/7 |

[Protocol](artifacts/small-protocol.md) · [Full timings](artifacts/small.json.gz)
