# Partitioned fused K/H for larger grouped proofs on Ryzen

The fused path increased paired full-proof latency by 6.52–10.47% on 4×2 and 8.65–12.59% on 8×2. The grouped 4×2 path streamed about 30,000 K points out of 97,075 input terms. The 2×2 control stayed on its original route.

The prototype streams K and H through the same signed windows. Each point range reduces its own buckets, and the resulting points are added in the final window fold. The grouped modes first apply existing K scalar grouping. Workspace admission occurs before dispatch, and the original K, H and B2 route handles denial.

| Shape | Request | Profile | Proof ms | Paired vs separate | K input / streamed | Parts | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|
| 2×2 | catalog | Separate K and H | 11.330 | +0.00% (0/6) | — | — | 87.8 |
| 2×2 | catalog | Fused, two ranges | 11.393 | +0.36% (2/6) | — | — | 88.5 |
| 2×2 | catalog | Group K, fuse two ranges | 11.292 | -0.39% (5/6) | — | — | 86.0 |
| 2×2 | catalog | Group K, fuse four ranges | 11.433 | +0.55% (2/6) | — | — | 87.5 |
| 2×2 | heldout | Separate K and H | 11.395 | +0.00% (0/6) | — | — | 86.8 |
| 2×2 | heldout | Fused, two ranges | 11.322 | -0.57% (5/6) | — | — | 86.5 |
| 2×2 | heldout | Group K, fuse two ranges | 11.356 | -0.27% (4/6) | — | — | 89.8 |
| 2×2 | heldout | Group K, fuse four ranges | 11.436 | +0.26% (2/6) | — | — | 86.8 |
| 4×2 | catalog | Separate K and H | 15.089 | +0.00% (0/6) | — | — | 217.0 |
| 4×2 | catalog | Fused, two ranges | 16.652 | +10.47% (0/6) | 97,075 / 97,075 | 2 | 155.2 |
| 4×2 | catalog | Group K, fuse two ranges | 16.092 | +7.22% (0/6) | 97,075 / 29,828 | 2 | 160.5 |
| 4×2 | catalog | Group K, fuse four ranges | 16.305 | +8.03% (0/6) | 97,075 / 29,828 | 4 | 148.5 |
| 4×2 | heldout | Separate K and H | 15.127 | +0.00% (0/6) | — | — | 217.2 |
| 4×2 | heldout | Fused, two ranges | 16.350 | +8.38% (0/6) | 97,075 / 97,075 | 2 | 159.5 |
| 4×2 | heldout | Group K, fuse two ranges | 16.053 | +6.52% (0/6) | 97,075 / 30,052 | 2 | 142.0 |
| 4×2 | heldout | Group K, fuse four ranges | 16.322 | +9.04% (0/6) | 97,075 / 30,052 | 4 | 143.0 |
| 8×2 | catalog | Separate K and H | 24.748 | +0.00% (0/6) | — | — | 372.0 |
| 8×2 | catalog | Fused, two ranges | 27.716 | +12.47% (0/6) | 186,411 / 186,411 | 2 | 279.2 |
| 8×2 | catalog | Group K, fuse two ranges | 26.987 | +9.52% (0/6) | 186,411 / 37,973 | 2 | 265.5 |
| 8×2 | catalog | Group K, fuse four ranges | 26.945 | +8.94% (0/6) | 186,411 / 37,973 | 4 | 259.8 |
| 8×2 | heldout | Separate K and H | 24.769 | +0.00% (0/6) | — | — | 382.8 |
| 8×2 | heldout | Fused, two ranges | 28.006 | +12.59% (0/6) | 186,411 / 186,411 | 2 | 273.0 |
| 8×2 | heldout | Group K, fuse two ranges | 27.323 | +10.70% (0/6) | 186,411 / 39,792 | 2 | 266.2 |
| 8×2 | heldout | Group K, fuse four ranges | 26.958 | +8.65% (0/6) | 186,411 / 39,792 | 4 | 284.2 |

One native test compares odd K and H segment lengths and signed coefficients with an independent generator result. Six timing blocks prime three proofs and time thirteen on 24 main and eight frontend Ryzen logical workers. Separate route checks cover group formation, shape rejection and forced zero-workspace denial. All 172 exports independently verified.

[Summary](artifacts/summary.json) · [Samples and route checks](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Trial patch](artifacts/source.patch.gz)
