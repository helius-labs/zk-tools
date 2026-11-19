# Full-proof integration of an unshifted G2 bucket tree

An earlier 96-core experiment reduced standalone 1×2 G2 query time by 24% using a balanced bucket tree. This trial integrates it with the current proof path and supports both 12-bit/two-range and 11-bit/four-range geometries. Ordinary windows route signed point indices into 256-bucket bands. Independent tasks reduce each band with borrowed input points and two intermediate point arrays. The narrow top window retains its specialized implementation. Optional proof-local scratch is charged before use. The process can fall back to streaming when its memory budget declines the request. Neither geometry retains a point table between proofs.

A native generator-sum test covers both geometries, signed and zero scalars, inverse and identity bases, and two Rayon widths. On a Ryzen 9950X the functional check selected both geometries wherever admitted and verified a 1×16 proof with the preparation ceiling set to zero. Four timing blocks compared release, disabled and enabled executables at each geometry with the same small public coefficient file. Each process used 32 logical workers, three warmups and eleven timed full proofs. All 191 process exports passed verification. The 96-core AWS full-proof screen is pending.

| Shape | Input | G2 ranges | Release / off / tree ms | Tree vs off (wins) | Tree vs release (wins) | Peak MiB release / tree |
|---|---|---:|---:|---:|---:|---:|
| 1x16 | catalog | 2 | 12.982 / 12.881 / 13.819 | +7.15% (0/4) | +6.32% (0/4) | 121.2 / 143.4 |
| 1x16 | catalog | 4 | 13.198 / 13.241 / 14.044 | +6.73% (0/4) | +6.40% (0/4) | 120.1 / 135.4 |
| 1x16 | 57 | 2 | 12.933 / 12.865 / 13.623 | +5.85% (0/4) | +5.88% (0/4) | 129.9 / 141.7 |
| 1x16 | 57 | 4 | 13.334 / 13.136 / 13.981 | +6.37% (0/4) | +5.57% (0/4) | 119.4 / 145.6 |
| 1x2 | catalog | 2 | 7.030 / 7.016 / 7.339 | +4.51% (0/4) | +5.32% (1/4) | 61.8 / 77.0 |
| 1x2 | catalog | 4 | 7.226 / 7.223 / 7.492 | +3.84% (0/4) | +3.84% (0/4) | 59.5 / 71.2 |
| 1x4 | catalog | 2 | 8.608 / 8.580 / 9.207 | +7.05% (0/4) | +6.35% (0/4) | 91.5 / 101.5 |
| 1x4 | catalog | 4 | 8.756 / 8.821 / 9.289 | +5.45% (0/4) | +5.41% (0/4) | 86.8 / 97.5 |
| 1x8 | catalog | 2 | 10.072 / 10.043 / 10.724 | +6.76% (0/4) | +6.41% (0/4) | 102.8 / 118.2 |
| 1x8 | catalog | 4 | 10.477 / 10.361 / 10.980 | +6.48% (0/4) | +5.25% (1/4) | 97.5 / 115.8 |
| 2x2 | catalog | 2 | 9.799 / 9.766 / 10.096 | +3.10% (1/4) | +3.03% (1/4) | 106.0 / 118.8 |
| 2x2 | catalog | 4 | 9.792 / 9.951 / 10.326 | +3.32% (0/4) | +5.73% (0/4) | 108.2 / 116.5 |
| 2x4 | catalog | 2 | 10.744 / 10.540 / 11.299 | +6.98% (0/4) | +5.51% (0/4) | 114.5 / 123.0 |
| 2x4 | catalog | 4 | 10.774 / 10.833 / 11.365 | +5.77% (0/4) | +5.07% (0/4) | 114.0 / 122.0 |

All fourteen local input/geometry comparisons had higher median latency with the tree enabled. The tree won one of 56 paired blocks against its disabled control. For 1×16 on four ranges, peak RSS increased from 120.1 to 135.4 MiB on the catalog input. The 96-core trial tests whether the width-dependent standalone improvement carries into the full proof.

[Earlier C7a query measurements](../2026-11-17-bucket-tree-c7a200/README.md) · [Source](artifacts/source.patch) · [Dispatch and memory fallback](artifacts/routes.json) · [Timings](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
