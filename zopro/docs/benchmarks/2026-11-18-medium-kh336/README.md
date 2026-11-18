# Combining medium private K and H multiscalar multiplications

The 2×2 control at 32 logical workers took 10.717 ms. Results for combined processing were raw 10.633 ms (-0.50% paired, 3/5 faster), group 11.874 ms (+9.67% paired, 2/5 faster).

At 16 physical cores, combining the original streams reduced 2×2 latency in all five blocks, from 12.118 to 11.832 ms (2.51% paired), with 97 versus 109 MiB peak RSS. This was a secondary setting. The primary 32-worker setting missed the threshold, so the candidate remains experimental.

| Shape | Workers | Mode | Median ms | Paired vs off | Wins | CPU ms | Peak MiB |
|---|---:|---|---:|---:|---:|---:|---:|
| 1x2 | 16 | retained | 8.842 | +0.15% | 2/5 | 100.6 | 60 |
| 1x2 | 16 | off | 8.826 | +0.00% | 0/5 | 100.6 | 62 |
| 1x2 | 16 | raw | 8.969 | -0.42% | 3/5 | 100.6 | 62 |
| 1x2 | 16 | group | 8.799 | +0.07% | 2/5 | 100.7 | 63 |
| 1x2 | 32 | retained | 7.707 | -0.55% | 3/5 | 152.0 | 64 |
| 1x2 | 32 | off | 7.703 | +0.00% | 0/5 | 152.5 | 64 |
| 1x2 | 32 | raw | 7.909 | +2.94% | 2/5 | 151.9 | 70 |
| 1x2 | 32 | group | 7.775 | +0.56% | 2/5 | 152.6 | 63 |
| 2x2 | 16 | retained | 12.172 | -0.91% | 3/5 | 137.5 | 105 |
| 2x2 | 16 | off | 12.118 | +0.00% | 0/5 | 136.9 | 109 |
| 2x2 | 16 | raw | 11.832 | -2.51% | 5/5 | 135.2 | 97 |
| 2x2 | 16 | group | 13.122 | +8.41% | 0/5 | 129.8 | 101 |
| 2x2 | 32 | retained | 10.869 | -1.55% | 3/5 | 204.8 | 113 |
| 2x2 | 32 | off | 10.717 | +0.00% | 0/5 | 201.7 | 124 |
| 2x2 | 32 | raw | 10.633 | -0.50% | 3/5 | 198.0 | 108 |
| 2x2 | 32 | group | 11.874 | +9.67% | 2/5 | 199.8 | 107 |

All 80 timed processes and five separate routing checks exported proofs that verified against the pinned public witness. Zero optional memory selected the existing fallback. Timings are medians of five process means. Peak memory is the largest process RSS in each cell.

[Protocol](artifacts/protocol.md) · [Process data](artifacts/processes.json.gz) · [Route counts](artifacts/routing.json) · [Verification](artifacts/verification.json)
