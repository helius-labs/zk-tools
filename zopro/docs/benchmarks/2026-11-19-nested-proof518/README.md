# Two-stage B2 groups in full 12×2 proofs

The lower stage evaluates the fine groups and corrects any disagreements on original points. The upper stage evaluates the broad groups on the resulting compact basis. If the upper-stage sampling, correction-count or memory test declines, the prover retains the fine groups. An unavailable lower stage returns to the ordinary B2 calculation. The same plan is used on every input. All grouped configurations use the same fixed broad A/K plan and public offsets. The linear configuration enables A/K alone. Five randomized blocks use 30 pinned Ryzen workers, three warmups and eleven timed calls. Peak RSS includes setup.

| Request | Groups off / A,K / broad / fine / two-stage ms | Two-stage vs broad | Two-stage vs fine | Two-stage vs off | Peak MiB off / broad / two-stage |
|---|---:|---:|---:|---:|---:|
| Original | 31.400 / 30.529 / 29.763 / 30.067 / 29.397 | -0.13% (4/5) | -2.23% (5/5) | -5.29% (5/5) | 553.5 / 516.5 / 585.3 |
| 59 | 30.977 / 30.632 / 29.235 / 30.009 / 29.510 | +0.65% (0/5) | -1.63% (5/5) | -4.73% (5/5) | 577.8 / 517.5 / 580.8 |
| 64 | 31.465 / 30.665 / 29.752 / 29.898 / 29.804 | +0.62% (1/5) | -0.89% (4/5) | -4.88% (5/5) | 550.9 / 529.4 / 580.6 |
| 141 | 47.403 / 47.181 / 47.046 / 45.059 / 45.130 | -4.07% (5/5) | +0.16% (2/5) | -4.85% (5/5) | 1024.3 / 994.3 / 974.1 |
| 142 | 47.713 / 46.910 / 47.147 / 45.033 / 45.171 | -4.19% (5/5) | +0.33% (1/5) | -4.75% (5/5) | 983.8 / 1064.0 / 1009.2 |
| 143 | 47.514 / 47.301 / 46.800 / 45.143 / 45.080 | -2.37% (5/5) | -0.14% (3/5) | -4.95% (5/5) | 980.0 / 984.2 / 1012.5 |
| 144 | 46.991 / 46.917 / 46.927 / 45.162 / 45.089 | -3.99% (5/5) | -0.83% (4/5) | -4.24% (5/5) | 990.8 / 972.5 / 1003.9 |

Original M5 CPU and Metal speedups were 9.66× and 3.44×. 0/5 blocks met 28.4 ms. All 185 exports verified against the original key. [Samples](artifacts/processes.json.gz) · [Budget and route probes](artifacts/probes.json) · [Incremental source](artifacts/incremental.patch) · [Nested plan](artifacts/nested.bin.gz)
