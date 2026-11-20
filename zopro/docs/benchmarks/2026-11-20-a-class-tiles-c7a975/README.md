# Direct point tiles for A on C7a 4×2

The combined layout with 32 A shards took **14.165 ms** on catalog and **14.226 ms** on held-out. It beat the exact K/B2 build by 0.85% and 1.00% in paired medians, winning 11/12 and 9/12 blocks. It beat same-build K/B2 by 1.84% and 0.91%, winning 12/12 and 9/12 blocks. The 32-shard layout was the only candidate that passed all four timing controls on both inputs. The [Ryzen run](../2026-11-20-a-class-tiles-ryzen974/README.md) showed higher full-proof latency with the same code.

A classifies its 67,154 scalar rows in independent shards. A global list of reduction tiles writes directly into disjoint ranges of the compact affine and scalar outputs. Each tile uses the previous point-reduction math and the existing window passes consume the rows in shard order. The trial used 64 main, 32 frontend and 16 solver workers. [Source and previous trial](../2026-11-20-a-class-tiles-ryzen974/README.md)

| Input | Mode | Proof ms | Paired vs ordinary | Paired vs exact 970 K and B2 | Paired vs same-build K and B2 | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| Catalog | Frozen | 14.796 | +0.04% (6/12) | +3.41% (0/12) | +2.67% (0/12) | 221.4 |
| Catalog | Ordinary | 14.770 | — | +3.76% (0/12) | +2.76% (0/12) | 215.5 |
| Catalog | 970 K and B2 | 14.263 | -3.62% (12/12) | — | -0.81% (11/12) | 213.2 |
| Catalog | Same-build K and B2 | 14.423 | -2.69% (12/12) | +0.81% (1/12) | — | 208.7 |
| Catalog | A with 16 shards | 14.507 | -2.11% (12/12) | +1.75% (0/12) | +0.65% (3/12) | 213.2 |
| Catalog | A with 16 shards plus K and B2 | 14.225 | -3.80% (12/12) | -0.13% (8/12) | -1.21% (11/12) | 209.5 |
| Catalog | A with 32 shards plus K and B2 | 14.165 | -4.34% (12/12) | -0.85% (11/12) | -1.84% (12/12) | 218.6 |
| Held-out | Frozen | 14.812 | +0.10% (5/12) | +2.91% (0/12) | +3.16% (0/12) | 211.0 |
| Held-out | Ordinary | 14.841 | — | +3.11% (0/12) | +3.26% (0/12) | 209.5 |
| Held-out | 970 K and B2 | 14.304 | -3.02% (12/12) | — | +0.00% (6/12) | 218.5 |
| Held-out | Same-build K and B2 | 14.358 | -3.16% (12/12) | -0.00% (6/12) | — | 213.8 |
| Held-out | A with 16 shards | 14.619 | -1.45% (11/12) | +1.89% (1/12) | +1.62% (0/12) | 214.6 |
| Held-out | A with 16 shards plus K and B2 | 14.226 | -3.75% (12/12) | -0.96% (9/12) | -1.13% (10/12) | 213.2 |
| Held-out | A with 32 shards plus K and B2 | 14.226 | -3.82% (11/12) | -1.00% (9/12) | -0.91% (9/12) | 226.0 |

| Input | Mode | A prep and compaction change ms | A window change ms | A query change ms | H query change ms | K query change ms | B2 query change ms |
|---|---|---:|---:|---:|---:|---:|---:|
| Catalog | A with 16 shards | -1.926 | +0.263 | -1.561 | -0.418 | -0.123 | -0.249 |
| Catalog | A with 16 shards plus K and B2 | -1.954 | +0.941 | -1.014 | -0.179 | +0.264 | +0.094 |
| Catalog | A with 32 shards plus K and B2 | -1.924 | +0.668 | -1.066 | -0.175 | -0.014 | +0.070 |
| Held-out | A with 16 shards | -1.228 | -0.023 | -1.251 | -0.524 | -0.012 | -0.150 |
| Held-out | A with 16 shards plus K and B2 | -1.459 | +0.569 | -0.885 | -0.117 | +0.243 | +0.265 |
| Held-out | A with 32 shards plus K and B2 | -1.668 | +0.467 | -1.257 | -0.299 | +0.416 | +0.294 |

| Input | Mode | Curve stage ms | A finished last | H finished last | K finished last | B2 finished last |
|---|---|---:|---:|---:|---:|---:|
| Catalog | Ordinary | 7.597 | 10 | 3 | 31 | 34 |
| Catalog | Same-build K and B2 | 7.169 | 17 | 24 | 0 | 37 |
| Catalog | A with 16 shards | 7.275 | 0 | 2 | 55 | 21 |
| Catalog | A with 16 shards plus K and B2 | 6.866 | 1 | 27 | 1 | 49 |
| Catalog | A with 32 shards plus K and B2 | 6.807 | 3 | 30 | 0 | 45 |
| Held-out | Ordinary | 7.528 | 4 | 1 | 30 | 43 |
| Held-out | Same-build K and B2 | 7.121 | 23 | 27 | 0 | 28 |
| Held-out | A with 16 shards | 7.396 | 0 | 1 | 50 | 27 |
| Held-out | A with 16 shards plus K and B2 | 6.872 | 1 | 33 | 0 | 44 |
| Held-out | A with 32 shards plus K and B2 | 6.908 | 3 | 31 | 0 | 44 |

With 32 shards, A preparation and point reduction saved 1.92 and 1.67 ms against same-build K/B2. A finished last three times per input out of 78 traces. H and B2 accounted for the other 75 traces on each input. Whole-process peak RSS was 218.6 and 226.0 MiB, up from 208.7 and 213.8 MiB with same-build K/B2. Phase changes are paired against ordinary for A alone and against same-build K and B2 for the combined modes. The additional worst-case construction reservations for A are 24.36 MiB with 16 shards and 24.39 MiB with 32. Ordinary calibrated within 0.10% of frozen. Same-build K and B2 calibrated within 0.81% of the exact 970 executable. The largest paired shift with phase instrumentation was 2.75%.

Each input used 12 paired blocks of seventeen proofs and 6 separate blocks of thirteen proofs with clocks. Route checks compared every member of each scalar class and the complete query point. A separate probe denied optional scratch and exercised the ordinary paths. All 240 proof exports independently verified. The temporary C7a instance and storage were removed.

## Eight supplied M5 examples

The 4×2 row uses the qualifying C7a mode with A with 32 shards plus K and B2. The other seven rows retain the recorded C7a timings. [Previous scorecard](../2026-11-20-shard-point-tiles-c7a971/README.md#eight-supplied-m5-examples)

| Shape | Recorded C7a ms | M5 CPU / Metal ms | CPU / Metal speedup |
|---|---:|---:|---:|
| 1×2 | 7.736 | 37 / 24 | 4.78× / 3.10× |
| 2×2 | 11.898 | 59 / 30 | 4.96× / 2.52× |
| 4×2 | 14.165 | 109 / 48 | 7.70× / 3.39× |
| 8×2 | 18.849 | 189 / 74 | 10.03× / 3.93× |
| 12×2 | 27.790 | 284 / 101 | 10.22× / 3.63× |
| 16×2 | 30.884 | 335 / 124 | 10.85× / 4.01× |
| 24×2 | 47.046 | 528 / 176 | 11.22× / 3.74× |
| merge-24×1 | 45.724 | 507 / 175 | 11.09× / 3.83× |

[Results](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Cleanup](artifacts/cleanup.json)
