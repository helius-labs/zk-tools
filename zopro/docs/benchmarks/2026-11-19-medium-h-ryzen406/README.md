# H-query layouts and compact solver outputs on Ryzen 12×2

All settings use the same integrated prover executable. The H layouts use 19 unsigned windows or 17 or 18 signed windows, each with one or two point ranges as indicated. Compact Poseidon output addresses can be enabled independently. Linux uprobes on a byte-identical copy established which H implementations ran. Three shuffled screening blocks precede seven fresh matched confirmation blocks. Each confirmation process timed 31 complete verified requests after three warmups.

## Screening on 32 hardware threads

| Layout | Compact outputs | Median ms | Peak MiB |
|---|---:|---:|---:|
| ordinary | 0 | 34.065 | 588.2 |
| ordinary | 1 | 33.326 | 621.2 |
| 19x1 | 0 | 33.482 | 609.5 |
| 19x1 | 1 | 33.418 | 649.2 |
| 19x2 | 0 | 35.531 | 637.2 |
| 19x2 | 1 | 35.925 | 614.5 |
| 17sx2 | 0 | 37.352 | 649.0 |
| 17sx2 | 1 | 36.422 | 624.2 |
| 18sx2 | 0 | 34.859 | 611.2 |
| 18sx2 | 1 | 34.531 | 606.0 |

## Confirmation

| Workers | Layout | Compact outputs | Median ms | CPU-ms per proof | Peak MiB | M5 CPU / Metal speedup | Blocks ≤28.4 ms / 7 |
|---:|---|---:|---:|---:|---:|---:|---:|
| 16 | ordinary | 0 | 38.643 | 569.8 | 547.5 | 7.35× / 2.61× | 0 |
| 16 | ordinary | 1 | 38.483 | 565.6 | 544.0 | 7.38× / 2.62× | 0 |
| 32 | ordinary | 0 | 33.412 | 927.1 | 696.5 | 8.50× / 3.02× | 0 |
| 32 | ordinary | 1 | 33.002 | 919.6 | 707.2 | 8.61× / 3.06× | 0 |

The screen selected the ordinary H path with `ZOPRO_CPU_POSEIDON_EMIT=1`. Uprobes confirmed that the alternative H kernels executed. Compact outputs also won five of seven blocks at 16 physical cores. The largest 32-worker process peak rose from 696.5 to 707.2 MiB. At 32 workers the paired change vs the ordinary baseline was -0.87% and it won 5 of seven blocks. The prespecified configuration comparison passed.

[Protocol](artifacts/protocol.md) · [Route checks](artifacts/routes.json) · [Decision](artifacts/decision.json) · [Process samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)
