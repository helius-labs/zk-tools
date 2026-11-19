# Solver task packing across Ryzen worker counts

Each process generated and verified 21 timed 16×2 proofs after three warmups. The two arms used the same executable and differed only in solver task packing. CPU time sums user and system time across threads per request. Each row contains five randomized pairs.

| Workers | Physical cores | Off ms | On ms | Paired change | Faster blocks / 5 | CPU-ms off / on | Peak MiB off / on | M5 CPU / Metal speedup with packing |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 8 | 8 | 69.768 | 68.769 | -1.36% | 5 | 530.0 / 525.3 | 513.5 / 514.3 | 4.87× / 1.80× |
| 16 | 16 | 45.146 | 44.632 | -1.23% | 3 | 662.0 / 658.3 | 628.8 / 641.0 | 7.51× / 2.78× |
| 32 | 16 | 40.233 | 40.371 | +0.18% | 2 | 1159.4 / 1155.9 | 772.8 / 780.0 | 8.30× / 3.07× |

Packing cleared the four-win screen only at eight physical cores. Moving from 16 to 32 workers on the same 16 cores shortened the proof by about 10–11% in both arms while increasing CPU time per proof. These measurements exercise Ryzen SMT. C7a exposes one hardware thread per core and needs a separate measurement.

[Protocol](artifacts/protocol.md) · [Samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)
