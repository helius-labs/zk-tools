# Four independent Euler butterflies per SIMD vector

At the innermost four-point Euler transform, the original SIMD multiplication applies roots in the pattern (1, t, 1, t). The candidate transposes four independent quads into vectors organized by coefficient index. Their odd coefficients share one vector multiplication by t per direction, and the even coefficients skip multiplication by one. The four diagonal weights and both transforms are preserved. A second candidate only composes successive lane permutations in the original two-quad schedule. Both candidates use the existing expanded leaf buffer. Full-transform equivalence tests compare all three versions against the separate inverse, scalar weighting and forward operations.

Each of the nine shapes ran five blocks, with each mode in every execution position and shuffled shape order. Selected uses the exact experimental executable and per-shape configuration from panel 653. Parent uses the mapped experimental binary with packed Euler enabled. Off, direct and four use one new executable under the same configuration as parent. On shapes with different selected and parent profiles, the off and parent comparisons isolate different effects. A separate verified diagnostic recorded whether the packed Euler function ran. Each timing process primed three requests and measured nine or thirteen single proofs. The table reports the median of process means, paired changes for four, and the largest whole-process RSS. Every one of the 234 process exports independently verified against the original key and public inputs. All timings are from Ryzen.

| Shape | Four entered | Selected / parent / off / direct / four ms | Four vs off (wins) | Four vs parent (wins) | Off / four peak MiB | Four vs M5 CPU / Metal |
|---|---|---:|---:|---:|---:|---:|
| 1x2 | yes | 7.111 / 7.129 / 7.113 / 7.128 / 7.121 | -0.04% (3/5) | +0.72% (2/5) | 61.8 / 68.8 | 5.20× / 3.37× |
| 2x2 | yes | 9.509 / 9.464 / 9.360 / 9.430 / 9.353 | +0.38% (2/5) | -1.04% (4/5) | 111.8 / 112.2 | 6.31× / 3.21× |
| 4x2 | yes | 14.792 / 14.840 / 14.809 / 14.666 / 14.778 | +0.34% (2/5) | -0.45% (3/5) | 293.8 / 290.2 | 7.38× / 3.25× |
| 8x2 | yes | 23.303 / 23.166 / 22.371 / 22.753 / 23.032 | +1.14% (0/5) | -1.02% (3/5) | 507.5 / 478.8 | 8.21× / 3.21× |
| 12x2 | yes | 28.370 / 28.658 / 28.372 / 28.460 / 28.443 | -0.36% (3/5) | -0.87% (3/5) | 568.3 / 546.2 | 9.99× / 3.55× |
| 16x2 | yes | 37.243 / 36.526 / 36.306 / 36.639 / 36.513 | -0.34% (3/5) | -0.04% (3/5) | 707.2 / 713.1 | 9.17× / 3.40× |
| 24x2 | yes | 57.214 / 60.291 / 60.000 / 59.488 / 59.579 | -0.11% (3/5) | -1.41% (4/5) | 1065.4 / 1118.0 | 8.86× / 2.95× |
| merge-24x1 | yes | 61.647 / 61.253 / 61.448 / 61.376 / 60.873 | -1.70% (5/5) | -0.62% (5/5) | 1005.5 / 975.0 | 8.33× / 2.87× |
| merge-54x1 | yes | 149.836 / 153.191 / 155.026 / 155.176 / 152.682 | -1.61% (3/5) | -0.84% (5/5) | 1763.1 / 1673.2 | — |

Merge 24×1 was the strongest result for the four-quad layout. Its 60.873 ms median beat both controls in all five paired blocks and reduced the observed peak by 30.5 MiB versus disabled. On merge54 the candidate beat the mapped parent five times but still took longer than the previously selected executable. On 8×2 it lost all five pairs against disabled.

The four-quad candidate reached none of the eight supplied 10× CPU thresholds. On 12×2 it missed 28.4 ms by 0.043 ms. The selected and parent rows on this shape are repeats with identical binaries and settings. Their medians, 28.370 and 28.658 ms, straddle the threshold. This spread limits any conclusion from an individual placement or a single block. All 159 native core tests passed.

[Source patch](artifacts/source.patch) · [Build](artifacts/build.json) · [Entry diagnostics](artifacts/markers.json) · [Workflow](artifacts/workflow.json.gz) · [Input pins](artifacts/pins.json.gz) · [Raw timings](artifacts/rows.json.gz) · [Paired results](artifacts/summary.json) · [Verification](artifacts/verification.json)
