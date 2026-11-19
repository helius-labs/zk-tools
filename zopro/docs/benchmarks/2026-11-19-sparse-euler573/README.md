# Streaming the nonzero H terms through the existing windows

The candidate checks a strided scalar sample. At least a quarter must be zero to allocate a charged one-bit-per-row mask. Each of the nineteen unsigned windows then visits only the live positions, with original key points accessed in place. The generic and reservation-fallback paths retain the ordinary window calculation. A boundary test exercises all-zero, dense and strided masks and unaligned ranges, including empty ranges.

Five randomized blocks compare the prior executable, a fresh disabled control and the enabled setting on the original and preconverted merge keys. The trace process cross-checks every live-count measurement with the earlier independently verified public-origin screen. Reported full-proof timings come from separate observer-free processes on 32 Ryzen workers.

| Key | Input | Prior / fresh control / candidate ms | Paired vs control | Paired vs prior | Candidate vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| ordinary | Original | 64.660 / 64.116 / 64.384 | -1.64% (3/5) | -0.84% (3/5) | 7.87× / 2.72× |
| mapped | 124 | 70.102 / 69.956 / 68.818 | -2.08% (4/5) | -0.74% (3/5) | — |
| mapped | 122 | 66.464 / 67.714 / 65.619 | -0.67% (5/5) | -0.30% (4/5) | — |
| mapped | Original | 61.145 / 62.092 / 61.570 | -1.22% (4/5) | -0.62% (3/5) | 8.23× / 2.84× |
| mapped | 109 | 63.803 / 65.060 / 62.984 | -3.20% (4/5) | -2.67% (4/5) | — |
| mapped | 112 | 66.075 / 67.770 / 64.953 | -2.61% (5/5) | -1.70% (5/5) | — |

All 96 proof exports verified against the original key. [Measurements](artifacts/samples.json.gz) · [Source change](artifacts/source.patch) · [Verification](artifacts/verification.json)
