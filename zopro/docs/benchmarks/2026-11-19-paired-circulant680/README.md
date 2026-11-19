# Paired circulant and four-quad transforms on the integrated native SDK

For a four-point inverse FFT, coefficient-index weighting and the matching forward FFT, the circulant formula uses the difference between the two neighboring lanes. Opposite lanes have opposite differences. The paired kernel combines the two independent differences from each of two quads before a single multiplication by the scaled fourth root. A shared sum provides both corrections. It bounds the diagonal factor to the existing 28-bit multiplier range and uses the ordinary path above that bound. The four-quad comparator uses the transpose method from study 676. Both procedures preserve the original scratch layout and proving key. The native equivalence tests check both against separately executed inverse, weighting and forward passes.

The three candidate arms use the same executable derived from the integrated SDK. Ordinary and frozen use its unmodified parent. Ordinary applies the catalog settings to the unmodified integrated candidate executable. The staged AWS pilot also retains a separate older reference executable. Frozen, off, four and paired enable the packed input and local Euler options together. Each input includes a separate dispatch diagnostic, balanced full-proof timing blocks and one additional sampled CPU run per arm. All processes pin 32 main hardware threads and verify the exported proof with the original key and public inputs. The table summarizes medians of warm process means and lists the largest process RSS.

| Input | Mode | Warm ms | Paired vs off | Paired vs frozen | Paired vs ordinary | Peak MiB | M5 CPU / Metal |
|---|---|---:|---:|---:|---:|---:|---:|
| merge-24x1 | ordinary | 63.372 | -0.20% (5/8) | +0.41% (4/8) | +0.00% (0/8) | 1033.2 | 8.00× / 2.76× |
| merge-24x1 | frozen | 63.066 | -0.03% (4/8) | +0.00% (0/8) | -0.41% (4/8) | 1051.9 | 8.04× / 2.77× |
| merge-24x1 | off | 63.307 | +0.00% (0/8) | +0.03% (4/8) | +0.20% (3/8) | 1028.5 | 8.01× / 2.76× |
| merge-24x1 | four | 63.067 | +0.17% (4/8) | -0.03% (4/8) | -0.11% (4/8) | 1006.8 | 8.04× / 2.77× |
| merge-24x1 | paired | 63.043 | -0.49% (5/8) | -0.20% (5/8) | -1.24% (5/8) | 1043.2 | 8.04× / 2.78× |
| merge-54x1 | ordinary | 153.536 | +0.23% (4/8) | +0.91% (2/8) | +0.00% (0/8) | 1697.5 | — |
| merge-54x1 | frozen | 152.900 | -0.84% (5/8) | +0.00% (0/8) | -0.91% (6/8) | 1682.0 | — |
| merge-54x1 | off | 154.041 | +0.00% (0/8) | +0.85% (3/8) | -0.22% (4/8) | 1774.4 | — |
| merge-54x1 | four | 153.136 | -0.23% (4/8) | +0.22% (4/8) | -0.10% (4/8) | 1777.7 | — |
| merge-54x1 | paired | 153.900 | +0.01% (4/8) | +1.01% (2/8) | -0.26% (5/8) | 1791.0 | — |
| 12x2 | ordinary | 31.020 | +0.55% (1/6) | +0.32% (2/6) | +0.00% (0/6) | 540.0 | 9.16× / 3.26× |
| 12x2 | frozen | 31.256 | +1.11% (2/6) | +0.00% (0/6) | -0.32% (4/6) | 526.4 | 9.09× / 3.23× |
| 12x2 | off | 30.904 | +0.00% (0/6) | -1.09% (4/6) | -0.55% (5/6) | 537.8 | 9.19× / 3.27× |
| 12x2 | four | 30.730 | -0.36% (3/6) | -0.94% (4/6) | -1.00% (5/6) | 516.9 | 9.24× / 3.29× |
| 12x2 | paired | 30.828 | +0.04% (3/6) | -1.34% (5/6) | -0.89% (5/6) | 510.8 | 9.21× / 3.28× |
| 24x2 | ordinary | 63.204 | +0.01% (3/6) | -1.08% (4/6) | +0.00% (0/6) | 990.0 | 8.35× / 2.78× |
| 24x2 | frozen | 63.424 | -0.61% (4/6) | +0.00% (0/6) | +1.09% (2/6) | 975.6 | 8.32× / 2.77× |
| 24x2 | off | 63.095 | +0.00% (0/6) | +0.62% (2/6) | +0.02% (3/6) | 964.3 | 8.37× / 2.79× |
| 24x2 | four | 63.752 | +1.44% (2/6) | +0.47% (3/6) | +0.51% (2/6) | 995.3 | 8.28× / 2.76× |
| 24x2 | paired | 63.399 | +0.47% (2/6) | +0.05% (3/6) | -0.17% (3/6) | 1019.2 | 8.33× / 2.78× |
| 5x16 | ordinary | 24.009 | +2.88% (0/6) | +1.85% (0/6) | +0.00% (0/6) | 410.5 | — |
| 5x16 | frozen | 23.516 | +0.94% (0/6) | +0.00% (0/6) | -1.82% (6/6) | 424.0 | — |
| 5x16 | off | 23.366 | +0.00% (0/6) | -0.93% (6/6) | -2.80% (6/6) | 416.8 | — |
| 5x16 | four | 23.522 | +0.63% (1/6) | -0.14% (3/6) | -2.45% (5/6) | 417.8 | — |
| 5x16 | paired | 23.353 | +0.23% (2/6) | -0.59% (5/6) | -2.90% (6/6) | 405.5 | — |
| 1x2 | ordinary | 7.226 | +0.97% (2/6) | -0.96% (4/6) | +0.00% (0/6) | 61.8 | 5.12× / 3.32× |
| 1x2 | frozen | 7.311 | +2.07% (0/6) | +0.00% (0/6) | +0.97% (2/6) | 61.0 | 5.06× / 3.28× |
| 1x2 | off | 7.167 | +0.00% (0/6) | -2.03% (6/6) | -0.96% (4/6) | 60.2 | 5.16× / 3.35× |
| 1x2 | four | 7.098 | -0.89% (4/6) | -2.61% (6/6) | -1.66% (6/6) | 60.8 | 5.21× / 3.38× |
| 1x2 | paired | 7.103 | -0.96% (4/6) | -2.70% (6/6) | -1.75% (6/6) | 60.8 | 5.21× / 3.38× |
| merge-24x1-v121 | ordinary | 66.037 | -0.03% (2/4) | -0.03% (2/4) | +0.00% (0/4) | 1117.8 | — |
| merge-24x1-v121 | frozen | 65.940 | +0.04% (2/4) | +0.00% (0/4) | +0.03% (2/4) | 1080.0 | — |
| merge-24x1-v121 | off | 66.257 | +0.00% (0/4) | -0.04% (2/4) | +0.03% (2/4) | 1098.6 | — |
| merge-24x1-v121 | four | 65.924 | -0.24% (2/4) | +0.36% (1/4) | +0.44% (1/4) | 1110.3 | — |
| merge-24x1-v121 | paired | 66.188 | +0.04% (2/4) | +0.38% (1/4) | -0.29% (2/4) | 1070.2 | — |
| merge-24x1-v128 | ordinary | 68.674 | -0.49% (2/4) | -0.08% (2/4) | +0.00% (0/4) | 1102.2 | — |
| merge-24x1-v128 | frozen | 68.577 | -0.31% (2/4) | +0.00% (0/4) | +0.09% (2/4) | 1133.0 | — |
| merge-24x1-v128 | off | 68.709 | +0.00% (0/4) | +0.32% (2/4) | +0.50% (2/4) | 1119.8 | — |
| merge-24x1-v128 | four | 68.957 | +0.12% (2/4) | +0.61% (2/4) | +0.99% (1/4) | 1110.0 | — |
| merge-24x1-v128 | paired | 69.289 | +0.30% (1/4) | +0.90% (0/4) | +0.99% (0/4) | 1165.1 | — |

The paired transform on the original merge24 saved a paired 0.49% against off and 0.20% against frozen. On held-out witnesses 121 and 128 it cost 0.04% and 0.30% against off, and lost seven of eight comparisons with frozen. On 1×2, four and paired took 7.098 and 7.103 ms, against 7.167 ms off and 7.311 ms frozen. Each beat off in four of six blocks, and frozen in six. This result leaves a small 1×2 effect open to confirmation on other requests. Across all eight inputs, 288 independent proof exports verified and 157 native tests passed. The existing runtime configuration is retained.

[Source patch](artifacts/source.patch) · [Build](artifacts/build.json) · [Configuration](artifacts/config.json.gz) · [Entry evidence](artifacts/markers.json) · [Full-proof measurements](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
