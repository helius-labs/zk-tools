# Reuse a twiddle across radix-32 FFT butterflies

Within one column, each of five FFT stages performs sixteen butterflies. At a fixed stage and position, consecutive blocks use the same twiddle. The existing loop visits each block first. Both candidates visit each twiddle first, broadcast it once at the source level, then apply it to every corresponding butterfly. One candidate loops over the five stages. The other spells out their five sizes in forward and inverse order so the compiler sees constant strides. Both keep the original key and scratch size. Optimizing compilers may already hoist some loads in the original.

Five full-proof blocks per shape rotate the five arms through all five execution positions. Selected uses the prior per-shape profile. Parent uses the indexed-output executable with packed Euler enabled. Off, root and fixed use a new build of that same source under the same settings as parent. Timing is the median of warm process means. Paired changes compare the fixed-stride route with the controls in the same block. Verified diagnostic requests exercised the new dispatch on every shape. Every process exported a proof and matching public inputs for the independent verifier.

| Shape | Fixed kernel seen | Selected / parent / off / root / fixed ms | Fixed vs off | Fixed vs parent | Off / fixed peak MiB | Fixed vs M5 CPU / Metal |
|---|---|---:|---:|---:|---:|---:|
| 1x2 | yes | 7.037 / 7.262 / 7.130 / 7.098 / 7.183 | +0.28% (2/5) | -1.05% (3/5) | 60.8 / 61.5 | 5.15× / 3.34× |
| 2x2 | yes | 9.560 / 9.413 / 9.316 / 9.287 / 9.426 | -0.44% (3/5) | +0.14% (2/5) | 110.2 / 107.8 | 6.26× / 3.18× |
| 4x2 | yes | 14.817 / 14.774 / 14.403 / 14.358 / 14.409 | +0.95% (1/5) | -2.93% (3/5) | 276.8 / 270.8 | 7.56× / 3.33× |
| 8x2 | yes | 23.286 / 22.893 / 22.291 / 22.260 / 22.166 | -0.46% (3/5) | -1.97% (5/5) | 444.5 / 438.8 | 8.53× / 3.34× |
| 12x2 | yes | 28.644 / 28.739 / 28.272 / 28.443 / 28.294 | +0.08% (2/5) | -0.87% (4/5) | 526.6 / 526.5 | 10.04× / 3.57× |
| 16x2 | yes | 37.674 / 36.354 / 35.898 / 35.712 / 36.213 | +1.07% (2/5) | -0.64% (4/5) | 696.6 / 678.8 | 9.25× / 3.42× |
| 24x2 | yes | 56.386 / 57.002 / 57.367 / 56.545 / 55.919 | -2.21% (5/5) | -1.58% (5/5) | 873.7 / 877.8 | 9.44× / 3.15× |
| merge-24x1 | yes | 62.027 / 61.047 / 60.378 / 61.112 / 60.658 | +0.02% (2/5) | -1.10% (4/5) | 1030.2 / 1004.2 | 8.36× / 2.89× |
| merge-54x1 | yes | 151.361 / 151.884 / 150.808 / 152.865 / 156.115 | +2.26% (0/5) | +2.10% (0/5) | 1794.1 / 1697.9 | — |

On 24×2, fixed strides took 55.919 ms, against 57.367 ms disabled and 57.002 ms for the original indexed build. All five pairs favored fixed strides against both controls. The 52.8 ms target remains 3.119 ms away. On 12×2, fixed strides took 28.294 ms (10.04× CPU and 3.57× Metal) and disabled took 28.272 ms. Fixed strides crossed the 28.4 ms target in 3/5 blocks and disabled did so in 3/5. The paired median against disabled was 0.08% slower. Merge54 lost all five pairs against both disabled and the original build. The 24×2 result warrants a narrower comparison on more requests. The general configuration is retained. All 162 native core tests passed with fixed strides and 21 FFT tests passed with the looped stages. All 234 fresh exports verified.

[Build](artifacts/build.json) · [Source change](artifacts/source.patch) · [Workflow](artifacts/workflow.json.gz) · [Input pins](artifacts/pins.json.gz) · [Dispatch markers](artifacts/markers.json) · [Full-proof timings](artifacts/rows.json.gz) · [Comparison by shape](artifacts/summary.json) · [Verification](artifacts/verification.json)
