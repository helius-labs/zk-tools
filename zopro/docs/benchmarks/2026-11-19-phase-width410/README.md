# Ryzen stage times with one and two threads per core

Five randomized matched blocks per shape hold the 16 physical cores fixed while enabling one or both hardware threads on each. Both widths use the same prover binary and settings. Small proofs use eight frontend workers within the same CPU mask. Each process primed three requests and timed 21. The plain columns are medians of process means except RSS, which is the largest lifetime peak. Stage columns are medians over 105 traced proofs per cell.

| Shape | Threads | Plain ms | CPU-ms | Peak MiB | Trace overhead | Solver ms | FFT ms | H ms | G2 kernel ms |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 16 | 8.337 | 99.1 | 57.2 | +0.03% | 0.791 | 0.497 | — | 5.967 |
| 1x2 | 32 | 7.277 | 151.2 | 64.8 | +2.09% | 0.845 | 0.560 | — | 4.633 |
| 12x2 | 16 | 38.051 | 558.5 | 544.2 | +0.43% | 2.701 | 9.857 | 22.219 | 7.124 |
| 12x2 | 32 | 33.209 | 916.8 | 683.5 | +2.89% | 2.751 | 9.640 | 14.711 | 4.721 |

On 1x2, 32 threads changed paired plain latency by -12.72% and won 5/5 blocks. On 12x2, 32 threads changed paired plain latency by -12.06% and won 5/5 blocks.

Stage intervals can overlap. Treat stage magnitudes for any cell with observer overhead outside ±5% as diagnostic. The G2 column measures the active short-query or raw kernel for its shape. Compare widths within each shape. C7a has one hardware thread per physical core and needs separate measurements.

[Protocol](artifacts/protocol.md) · [Raw process samples and stage intervals](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)
