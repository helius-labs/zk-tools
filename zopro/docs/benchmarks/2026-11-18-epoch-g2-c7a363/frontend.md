# Small canonical frontend width and solver epochs

Each screen setting uses five fresh processes. Select the smallest median at each width before seven subsequently randomized confirmations of 71 requests. Same-host Aeglos uses 96 pinned physical cores, 96 native workers and GOMAXPROCS 96.

| Prover | Main cores | Front workers | Epochs | G2 ranges | Holdout ms | CPU-ms | Peak MiB | Paired vs Aeglos | Faster blocks | M5 CPU / Metal speedup |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Zopro | 64 | 16 | 4 | 2 | 8.101 | 277.5 | 53 | +0.70% | 2/7 | 4.57× / 2.96× |
| Aeglos | 96 | — | — | — | 8.020 | 502.3 | 472 | +0.00% | 0/7 | 4.61× / 2.99× |
| Zopro | 96 | 16 | 16 | 4 | 8.563 | 407.1 | 51 | +6.82% | 0/7 | 4.32× / 2.80× |
| Zopro | 96 | 16 | 4 | 4 | 8.260 | 405.5 | 49 | +2.83% | 2/7 | 4.48× / 2.91× |

| Main cores | Selected epochs / frontend | Screen ms | Confirmation ms |
|---:|---:|---:|---:|
| 64 | 4 / 16 | 8.092 | 8.101 |
| 96 | 4 / 16 | 8.269 | 8.260 |

[Protocol](artifacts/front-protocol.md) · [Samples](artifacts/supplement.json.gz)

For the 96-core 1×2 confirmation the SDK harness pinned main workers on CPUs 0–95 and attached a 16-worker frontend pool on CPUs 0–15. It set `ZOPRO_SOLVE_EPOCHS=4`, `ZOPRO_CPU_G2_RANGES=4` and `ZOPRO_CPU_G2_DECOMPOSITION=serial`. At 64 cores it attached the same frontend width and used two G2 ranges. `ZOPRO_BENCH_SHARED_FRONTEND` controls pool construction in the benchmark driver. SDK callers supply their pool through `set_frontend_pool`. [Shared environment](artifacts/profile.env)
