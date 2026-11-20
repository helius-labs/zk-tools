# Fused K/H point tiling on 96-core C7a

Point tiling saved 0.97–1.10 ms in the K/H MSM and added 1.25–1.36 ms of compaction. Warm proof latency rose by 2.26–2.88% in paired medians against the same-build control. Aeglos retained the 2×2 lead on both requests. The pinned Zopro profile took 11.916 ms on the catalog request, 4.95× the supplied M5 CPU speed and 2.52× Metal.

The [Ryzen prototype](../2026-11-20-fused-kh-class-tiles-ryzen982/README.md) reduces canonical private K rows in direct point tiles, then takes the existing joint K/H MSM with the original H inputs. Zopro uses 64 main and 32 frontend physical cores. Aeglos uses all 96 cores with a prepared-key cache hit on every timed proof.

| Request | Profile | Proof ms | Paired vs pinned | Paired vs same-build | K/H MSM ms | Compaction ms | Peak MiB |
|---|---|---:|---:|---:|---:|---:|---:|
| catalog | Pinned Zopro | 11.916 | — | -0.29% (7/12) | — | — | 77.4 |
| catalog | Same-build control | 11.912 | +0.29% (5/12) | — | 5.413 | — | 80.6 |
| catalog | Eight shards | 12.175 | +2.54% (0/12) | +2.34% (1/12) | 4.333 | 1.336 | 76.8 |
| catalog | Sixteen shards | 12.111 | +2.61% (0/12) | +2.26% (0/12) | 4.436 | 1.251 | 76.8 |
| catalog | Thirty-two shards | 12.225 | +2.70% (0/12) | +2.88% (1/12) | 4.438 | 1.336 | 78.3 |
| catalog | Prepared Aeglos | 11.491 | -3.84% (12/12) | -2.75% (12/12) | — | — | 811.0 |
| heldout | Pinned Zopro | 11.902 | — | -0.08% (7/12) | — | — | 79.7 |
| heldout | Same-build control | 11.939 | +0.08% (5/12) | — | 5.509 | — | 81.3 |
| heldout | Eight shards | 12.213 | +2.31% (1/12) | +2.58% (1/12) | 4.363 | 1.322 | 77.4 |
| heldout | Sixteen shards | 12.253 | +2.83% (0/12) | +2.53% (0/12) | 4.442 | 1.261 | 78.2 |
| heldout | Thirty-two shards | 12.224 | +2.74% (0/12) | +2.50% (0/12) | 4.411 | 1.364 | 75.2 |
| heldout | Prepared Aeglos | 11.509 | -3.54% (10/12) | -3.60% (12/12) | — | — | 808.8 |

Twelve paired blocks prime three proofs and time seventeen. The clock panel comes from four separate blocks. Each active route preserved the complete K/H curve point and optional-workspace denial selected the ordinary path. All 186 exports independently verified. The temporary host and storage were removed.

[Samples and routes](artifacts/rows.json.gz) · [Summary](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Cleanup](artifacts/cleanup.json)
