# Prepared groups on four- and eight-input transfers

Each shape uses eight distinct public training requests with adjacent state leaves. Groups of three or more B2 positions agree on a varying wide scalar across all eight requests. The same wires map into A and private K. Four held-out inputs per shape cover private transfer, deposit, withdrawal and a data-bearing output. All requests were generated with independent expected public witnesses. The two production configurations use the original settings on 32 Ryzen workers. Four randomized blocks compare the accepted release, the experimental build with no groups, and that build with sampled groups. Each timing process used three primes and seventeen measured calls.

| Shape | Request | Release / off / grouped, ms | Grouped vs release | Grouped vs off | Peak MiB release / grouped |
|---|---:|---:|---:|---:|---:|
| 4x2 | Original | 15.763 / 15.478 / 15.152 | -4.61% (4/4) | -1.71% (4/4) | 266.0 / 329.2 |
| 4x2 | 153 | 15.623 / 15.722 / 15.689 | -0.72% (2/4) | +0.45% (1/4) | 279.8 / 334.0 |
| 4x2 | 154 | 15.613 / 15.912 / 15.247 | -2.50% (4/4) | -4.17% (4/4) | 287.5 / 331.8 |
| 4x2 | 155 | 15.944 / 15.665 / 15.494 | -2.14% (4/4) | -0.97% (2/4) | 277.8 / 321.5 |
| 4x2 | 156 | 16.205 / 15.647 / 15.567 | -3.10% (4/4) | -0.28% (2/4) | 279.5 / 326.2 |
| 8x2 | Original | 23.871 / 24.453 / 23.218 | -2.89% (4/4) | -4.50% (4/4) | 466.0 / 544.5 |
| 8x2 | 153 | 24.189 / 24.270 / 22.951 | -4.75% (4/4) | -4.37% (4/4) | 471.5 / 539.8 |
| 8x2 | 154 | 24.635 / 24.466 / 22.874 | -6.12% (4/4) | -7.03% (4/4) | 474.0 / 554.0 |
| 8x2 | 155 | 24.710 / 23.957 / 23.160 | -6.35% (4/4) | -3.01% (4/4) | 460.2 / 544.5 |
| 8x2 | 156 | 24.428 / 24.717 / 22.930 | -5.86% (3/4) | -7.47% (4/4) | 461.8 / 542.0 |

All 142 exported proofs passed independent Gnark verification. [Measurements](artifacts/processes.json.gz) · [Fixtures](artifacts/fixtures.json) · [Routes](artifacts/probes.json) · [Group dimensions](artifacts/layouts.json) · [Verification](artifacts/verification.json)
