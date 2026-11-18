# Worker timing within and after G2

Each cell has three plain and three traced processes, each timing 51 requests. Task columns give medians of the 153 traced requests. Start spread runs from first range entry to last range entry. Reduction covers the interval from the last range return until the G2 kernel returns. These are elapsed times that include nested Rayon work. The observer used the independently calibrated executable.

| Cores | Ranges | Plain ms | Observer shift | Distinct range workers | Start spread ms | Median / longest range ms | Reduction ms | After G2 ms |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 32 | 2 | 10.820 | -0.17% | 14 | 2.892 | 2.684 / 4.487 | 0.430 | 0.018 |
| 32 | 4 | 11.158 | +1.04% | 26 | 4.114 | 1.462 / 3.029 | 0.486 | 0.017 |
| 64 | 2 | 8.513 | +0.28% | 22 | 0.009 | 2.817 / 3.271 | 0.541 | 0.019 |
| 64 | 4 | 9.054 | +2.80% | 43 | 1.472 | 1.554 / 3.443 | 0.303 | 0.019 |
| 96 | 2 | 10.132 | +1.83% | 22 | 0.012 | 2.911 / 3.393 | 0.672 | 0.024 |
| 96 | 4 | 9.016 | +1.18% | 48 | 0.013 | 1.645 / 2.355 | 0.516 | 0.424 |

[Full-proof comparisons](README.md) · [Request intervals and sampled symbols](artifacts/worker-traces.json.gz) · [Verification](artifacts/worker-verification.json)

## Slow requests at 96 cores

Slow means at least 10 ms end to end. Each timing is the median within its traced subgroup. Start delay runs from the G2 kernel entry to the first point range.

| Ranges | Request group | Count | Proof ms | Start delay ms | First to last range start ms | Longest range ms | After G2 ms |
|---:|---|---:|---:|---:|---:|---:|---:|
| 2 | under 10 ms | 84 | 8.924 | 0.125 | 0.012 | 3.385 | 0.021 |
| 2 | at least 10 ms | 69 | 12.167 | 0.122 | 0.012 | 3.408 | 2.717 |
| 4 | under 10 ms | 116 | 8.589 | 0.294 | 0.013 | 2.374 | 0.569 |
| 4 | at least 10 ms | 37 | 11.061 | 3.555 | 0.012 | 2.280 | 0.019 |

With two ranges, slow requests spent 2.717 ms between the G2 return and the canonical core return. G1 symbols accounted for 55% of the 2,021 userspace CPU-clock samples in that interval. With four ranges, slow requests spent 3.555 ms before the first range entered. Both effects depend on the selected geometry.

[CPU samples and symbol counts](samples.md)
