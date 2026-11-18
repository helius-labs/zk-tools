# Solver task claims on C7a

Five randomized blocks select the fastest exercised policy per circuit and width. Seven fresh matched blocks compare that policy with the release and disabled binary. 1×2 uses a 16-worker frontend, early verification and four serial G2 ranges. 16×2 uses six solver epochs, two G2 ranges and the main pool.

| Shape | Cores | Selected | Release ms | Disabled ms | Selected ms | Paired vs release | Wins / 7 | Paired vs disabled | Wins / 7 | Release / selected CPU-ms | Peak MiB release / selected | M5 CPU / Metal gain |
|---|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 96 | both | 8.158 | 8.156 | 8.175 | +0.78% | 3 | +0.23% | 2 | 387.5 / 388.6 | 51 / 47 | 4.53× / 2.94× |
| 16x2 | 64 | leaves | 33.726 | 33.834 | 33.641 | -0.25% | 4 | -0.49% | 4 | 1658.9 / 1640.9 | 795 / 799 | 9.96× / 3.69× |
| 16x2 | 96 | twice | 36.426 | 35.983 | 36.314 | -0.76% | 5 | -0.13% | 4 | 2490.3 / 2457.2 | 798 / 836 | 9.23× / 3.41× |

Aeglos on 1×2 at 96 cores measured 8.081 ms with 464.4 MiB peak RSS in the same seven blocks.

0/7 candidate process means at 64 cores met the 33.5 ms target. At 96 cores the median of the paired changes versus disabled was −0.13%, despite the candidate having a higher standalone median. These statistics summarize the same seven pairs in different ways.

[Process samples](artifacts/processes.json.gz) · [Protocol](artifacts/protocol.md)

[Linux software counters](counters.md)
