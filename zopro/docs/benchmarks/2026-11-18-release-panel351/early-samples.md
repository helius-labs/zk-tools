# CPU samples early in the curve computation

For each traced 1×2 proof, count Crossbeam CPU-clock samples in the first millisecond after the product FFT returns. Every proof was still in the canonical core when this fixed observation window ended. Compare those counts with the time remaining from that point until the SDK returned. Correlations are Spearman rank coefficients calculated separately within each 51-request process. Counting over a fixed interval avoids a longer request automatically contributing more observations. These measurements are exploratory and do not establish causality.

| Cores | Ranges | ≥10 ms requests | Median Crossbeam samples, <10 / ≥10 ms | Within-process correlations |
|---:|---|---:|---:|---:|
| 64 | two | 10/153 | 7 / 4.5 | -0.26 / -0.30 / +0.06 |
| 80 | serial | 4/153 | 10 / 0.5 | -0.14 / +0.12 / -0.11 |
| 88 | serial | 4/153 | 14 / 0.5 | +0.06 / -0.15 / -0.03 |
| 96 | serial | 4/153 | 17 / 0.5 | -0.10 / +0.20 / +0.02 |
| 96 | two | 79/153 | 16 / 17 | +0.15 / +0.05 / +0.08 |

At 96 cores with the ordinary two-range configuration, slow and fast requests had similar early counts. Within-process rank correlations were +0.05 to +0.15. An early count alone gives little support for a per-request switch.

Sampling runs at 500 Hz per active thread. The absolute counts in a 1 ms window are small. The primary uninstrumented panel reports user-visible request latency.

[Request observations](artifacts/early-samples.json.gz) · [Stage profiles](prefix.md)
