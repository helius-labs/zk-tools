# Release check for opt-in grouped G2 decomposition

Every 5×16 arm uses the same original verification key and the named public witness. Release is the former ordinary SDK. Prior is that executable with the packed FFT enabled. Off and on use a newly compiled SDK with the same packed FFT and differ in `ZOPRO_CPU_GROUPED_G2_PHI=1`. On 1×16 the key is below the grouped admission size, and the two former-SDK arms are identical controls. Each process warms three requests, times eleven and independently exports its last proof. Separate sample profiles time fifteen. All 148 exports verified. The native core passed 157 tests, including a generator oracle at 1,023 and 1,024 distinct coefficients on a 65,537-point query. The generic x86-64 prover passed a release-mode compile check.

| Input | Mode | Warm ms | Release / arm | Paired vs release | Paired vs prior | Paired vs off | Peak MiB |
|---|---|---:|---:|---:|---:|---:|---:|
| 5x16 | release | 23.965 | 1.000× | +0.00% (0/8) | +1.70% (0/8) | +1.55% (1/8) | 413.2 |
| 5x16 | prior | 23.439 | 1.022× | -1.68% (8/8) | +0.00% (0/8) | -0.00% (4/8) | 425.5 |
| 5x16 | on | 23.200 | 1.033× | -3.72% (8/8) | -1.10% (7/8) | -1.11% (5/8) | 432.5 |
| 5x16 | off | 23.471 | 1.021× | -1.52% (7/8) | +0.01% (4/8) | +0.00% (0/8) | 434.0 |
| 5x16-1701 | release | 24.129 | 1.000× | +0.00% (0/6) | +1.91% (1/6) | +2.46% (2/6) | 407.2 |
| 5x16-1701 | prior | 23.501 | 1.027× | -1.88% (5/6) | +0.00% (0/6) | +1.09% (2/6) | 419.0 |
| 5x16-1701 | on | 22.848 | 1.056× | -4.86% (6/6) | -2.77% (6/6) | -1.70% (6/6) | 422.0 |
| 5x16-1701 | off | 23.344 | 1.034× | -2.39% (4/6) | -1.08% (4/6) | +0.00% (0/6) | 427.8 |
| 5x16-1702 | release | 24.283 | 1.000× | +0.00% (0/6) | +2.67% (1/6) | +2.28% (0/6) | 419.0 |
| 5x16-1702 | prior | 23.863 | 1.018× | -2.60% (5/6) | +0.00% (0/6) | +1.47% (2/6) | 409.5 |
| 5x16-1702 | on | 23.177 | 1.048× | -3.82% (6/6) | -2.91% (4/6) | -0.78% (4/6) | 423.5 |
| 5x16-1702 | off | 23.661 | 1.026× | -2.23% (6/6) | -1.44% (4/6) | +0.00% (0/6) | 416.2 |
| 5x16-1703 | release | 24.601 | 1.000× | +0.00% (0/6) | +4.13% (1/6) | +5.17% (2/6) | 405.2 |
| 5x16-1703 | prior | 23.574 | 1.044× | -3.97% (5/6) | +0.00% (0/6) | -0.44% (4/6) | 425.5 |
| 5x16-1703 | on | 23.206 | 1.060× | -5.85% (5/6) | -1.41% (4/6) | -0.79% (4/6) | 420.8 |
| 5x16-1703 | off | 23.607 | 1.042× | -4.91% (4/6) | +0.45% (2/6) | +0.00% (0/6) | 409.5 |
| 1x16 | release | 12.989 | 1.000× | +0.00% (0/6) | +0.76% (2/6) | -0.56% (3/6) | 242.0 |
| 1x16 | prior | 12.986 | 1.000× | -0.76% (4/6) | +0.00% (0/6) | -0.34% (3/6) | 248.2 |
| 1x16 | on | 13.119 | 0.990× | +1.18% (2/6) | +0.88% (2/6) | +0.12% (3/6) | 249.5 |
| 1x16 | off | 13.080 | 0.993× | +0.57% (3/6) | +0.35% (3/6) | +0.00% (0/6) | 240.8 |

The opt-in won 25/26 pairs against the previous ordinary release, 21/26 against packed FFT alone and 19/26 against its same-binary disabled control. The G2 change reduced paired latency on all four witnesses. On the catalog request the new configuration used 432.5 MiB versus 413.2 MiB for release. Select it for the measured 32-thread Ryzen 5×16 configuration. Other shapes, hardware and memory budgets can use the ordinary path. [Integrated sources and compiler flags](artifacts/integrated.json)

[Earlier implementation and fixtures](../2026-11-19-grouped-wide659/README.md) · [Build](artifacts/build.json) · [Exact configurations](artifacts/configs.json.gz) · [Timings and profiles](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
