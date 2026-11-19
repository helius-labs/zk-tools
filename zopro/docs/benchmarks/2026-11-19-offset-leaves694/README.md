# Apply public wire corrections in disjoint wire blocks

The canonical gather can apply a key's sorted public wire offsets in each existing conversion task, process untouched stretches with the original field conversion and emit canonical zero directly when the witness equals its offset. Other solver paths apply offsets to disjoint 2,048-wire chunks. The trial allocates no new key or witness arrays. A native test compares several block boundaries, exact and inexact coefficients, and zero input against serial subtraction and conversion. Each shape has a separate verified dispatch check.

Five rotated blocks compared the current release and the same candidate binary with the option off and on. All configurations used the same public coefficient file and 32 logical workers on the Ryzen 9950X. Each process timed thirteen warm proofs after three primes. The results are medians of process means. The independent verifier accepted all 140 exports with the expected public inputs.

| Shape | Request | Observed route | Release / off / on ms | Paired vs off (wins) | Paired vs release (wins) | Peak MiB release / on |
|---|---|---|---:|---:|---:|---:|
| 1x2 | catalog | canonical | 7.228 / 7.244 / 7.209 | -0.59% (4/5) | -0.20% (3/5) | 60.2 / 61.2 |
| 1x16 | catalog | canonical | 13.114 / 13.183 / 13.189 | +0.04% (2/5) | +0.72% (2/5) | 119.9 / 124.9 |
| 1x16 | 57 | canonical | 13.166 / 13.289 / 13.351 | +0.40% (1/5) | +1.52% (0/5) | 124.7 / 126.1 |
| 2x16 | catalog | partitioned | 16.789 / 16.878 / 16.825 | +1.52% (2/5) | +1.49% (2/5) | 259.6 / 264.9 |
| 2x16 | 57 | partitioned | 16.872 / 16.811 / 16.685 | -0.07% (3/5) | -0.71% (4/5) | 285.7 / 272.8 |
| 4x16 | catalog | partitioned | 20.311 / 19.946 / 20.065 | +0.81% (2/5) | -0.44% (3/5) | 307.0 / 344.9 |
| 4x16 | 57 | partitioned | 19.896 / 20.176 / 19.771 | -2.01% (5/5) | -0.97% (3/5) | 306.1 / 330.1 |
| 6x8 | catalog | partitioned | 22.398 / 22.617 / 22.664 | -0.30% (3/5) | +0.47% (2/5) | 345.1 / 369.1 |
| 6x8 | 57 | partitioned | 22.521 / 22.719 / 22.797 | -0.13% (3/5) | +1.22% (1/5) | 361.4 / 375.0 |

The canonical route lost four of five held-out pairs on 1×16 against its disabled control. On 4×16 the partitioned route gained 2.01% on the held-out request and lost 0.81% on the catalog request against the same control. None of the four larger shapes improved against both controls on both inputs. The AWS screen uses the original offset application.

[Implementation](artifacts/source.patch) · [Dispatch checks](artifacts/routes.json) · [Measurements](artifacts/rows.json.gz) · [Native validation](artifacts/build.json) · [Proof verification](artifacts/verification.json)
