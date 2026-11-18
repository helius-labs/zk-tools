# Sharing a bounded frontend pool across SDK clients

A service can give several keys the same Rayon pool for the constraint solve, scalar preparation and FFT on the canonical path. Each proof returns to the main pool for its multi-scalar multiplications. The benchmark explicitly creates and pins the extra workers. The candidate SDK creates none by default. The patch is relative to the inline-grouping source from experiment 304, with that grouping option enabled in all arms.

On the 16-core Ryzen with 32 logical workers, neither frontend width passed the 1×2 screen. Both won three of five pairs against disabled. With two clients, median throughput was lower with both separate pools. The production default keeps the original pool.

| Shape | Mode | Median ms | Paired vs disabled | Wins | CPU ms / proof | Peak MiB | M5 CPU / Metal speedup |
|---|---|---:|---:|---:|---:|---:|---:|
| 1x2 | reference | 8.206 | +0.03% | 2/5 | 165 | 78 | 4.51× / 2.92× |
| 1x2 | off | 8.827 | +0.00% | 0/5 | 166 | 84 | 4.19× / 2.72× |
| 1x2 | front8 | 7.990 | -13.70% | 3/5 | 151 | 72 | 4.63× / 3.00× |
| 1x2 | front16 | 7.525 | -14.81% | 3/5 | 161 | 86 | 4.92× / 3.19× |
| 2x2 | reference | 10.445 | -0.56% | 4/5 | 239 | 284 | 5.65× / 2.87× |
| 2x2 | off | 10.532 | +0.00% | 0/5 | 239 | 288 | 5.60× / 2.85× |
| 2x2 | front8 | 10.827 | +2.93% | 0/5 | 203 | 118 | 5.45× / 2.77× |
| 2x2 | front16 | 11.044 | +5.65% | 0/5 | 226 | 124 | 5.34× / 2.72× |

## Two simultaneous 1×2 clients

The two SDK instances share the same mapped image, main pool and optional frontend pool. Each client primes three times before a barrier, then proves 21 requests. The time window ends when both clients finish. Every process exports a separate verified proof from each client. Latency p95 is the median of the five within-process nearest-rank p95s for 42 requests.

| Extra frontend workers | Median proofs/s | Median request p95 ms | CPU ms / proof | Peak process MiB |
|---:|---:|---:|---:|---:|
| 0 | 153.21 | 19.64 | 171 | 154 |
| 8 | 144.14 | 21.39 | 154 | 126 |
| 16 | 144.44 | 21.77 | 166 | 148 |

All 70 exported proofs passed independent verification. Local measurements cannot establish the performance effect on 96 physical cores.

[Source patch relative to 304](artifacts/source.patch) · [One-client results](artifacts/rows.json.gz) · [Two-client results](artifacts/concurrent.json.gz) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json)
