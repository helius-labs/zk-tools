# Point tiling for canonical G1 queries on Ryzen

No point-tiling profile passed both dispatch and paired timing checks on both requests. Tiling K increased paired latency by 1.15–2.69% on 1×4 and 1.63–2.59% on 1×16. Tiling A changed 2×2 latency by −0.22% to +0.54%. On 1×2, the diversity or workspace checks kept every request on ordinary grouping.

The experiment classifies the already canonical A and K coefficients in 16 shards. It reduces each accepted group to one point using the existing tile reducer and then runs the existing MSM. The original proving key stays shared. The diversity check and proof workspace reservation gate the optional path.

| Shape | Input | Main+G2 | Mode | Full proof ms | Paired vs ordinary | Tiled calls in diagnostics | Peak MiB |
|---|---|---|---|---:|---:|---|---:|
| 1×2 | catalog | 16+16 | Ordinary grouping | 6.757 | +0.00% (0/6) | A 0/3 | 56.5 |
| 1×2 | catalog | 16+16 | Tile A | 6.713 | -0.75% (4/6) | A 0/3 | 58.0 |
| 1×2 | catalog | 20+12 | Ordinary grouping | 6.501 | +0.00% (0/6) | A 0/3 | 59.2 |
| 1×2 | catalog | 20+12 | Tile A | 6.515 | +0.32% (3/6) | A 0/3 | 60.5 |
| 1×2 | heldout | 16+16 | Ordinary grouping | 6.566 | +0.00% (0/6) | A 0/3 | 59.0 |
| 1×2 | heldout | 16+16 | Tile A | 6.569 | +0.29% (3/6) | A 0/3 | 59.0 |
| 1×2 | heldout | 20+12 | Ordinary grouping | 6.373 | +0.00% (0/6) | A 0/3 | 58.2 |
| 1×2 | heldout | 20+12 | Tile A | 6.352 | -0.58% (4/6) | A 0/3 | 60.0 |
| 1×4 | catalog | 16+16 | Ordinary grouping | 8.074 | +0.00% (0/6) | A 0/3, K 0/3 | 94.2 |
| 1×4 | catalog | 16+16 | Tile A | 8.048 | -0.04% (4/6) | A 3/3, K 0/3 | 96.2 |
| 1×4 | catalog | 16+16 | Tile K | 8.170 | +1.32% (1/6) | A 0/3, K 3/3 | 106.8 |
| 1×4 | catalog | 16+16 | Tile A and K | 8.175 | +1.06% (0/6) | A 3/3, K 3/3 | 109.5 |
| 1×4 | catalog | 20+12 | Ordinary grouping | 8.134 | +0.00% (0/6) | A 0/3, K 0/3 | 108.0 |
| 1×4 | catalog | 20+12 | Tile A | 8.129 | +0.28% (2/6) | A 3/3, K 0/3 | 110.0 |
| 1×4 | catalog | 20+12 | Tile K | 8.273 | +2.47% (1/6) | A 0/3, K 3/3 | 116.0 |
| 1×4 | catalog | 20+12 | Tile A and K | 8.324 | +2.70% (0/6) | A 3/3, K 3/3 | 115.5 |
| 1×4 | heldout | 16+16 | Ordinary grouping | 8.046 | +0.00% (0/6) | A 0/3, K 0/3 | 93.5 |
| 1×4 | heldout | 16+16 | Tile A | 8.108 | +0.81% (1/6) | A 3/3, K 0/3 | 98.2 |
| 1×4 | heldout | 16+16 | Tile K | 8.148 | +1.15% (0/6) | A 0/3, K 3/3 | 100.0 |
| 1×4 | heldout | 16+16 | Tile A and K | 8.184 | +1.91% (1/6) | A 3/3, K 3/3 | 107.8 |
| 1×4 | heldout | 20+12 | Ordinary grouping | 8.148 | +0.00% (0/6) | A 0/3, K 0/3 | 107.8 |
| 1×4 | heldout | 20+12 | Tile A | 8.211 | +0.85% (1/6) | A 3/3, K 0/3 | 106.5 |
| 1×4 | heldout | 20+12 | Tile K | 8.383 | +2.69% (0/6) | A 0/3, K 3/3 | 110.8 |
| 1×4 | heldout | 20+12 | Tile A and K | 8.334 | +1.71% (0/6) | A 3/3, K 3/3 | 111.8 |
| 1×16 | catalog | 16+16 | Ordinary grouping | 12.318 | +0.00% (0/6) | A 0/3, K 0/3 | 132.2 |
| 1×16 | catalog | 16+16 | Tile A | 12.392 | +0.50% (1/6) | A 3/3, K 0/3 | 135.8 |
| 1×16 | catalog | 16+16 | Tile K | 12.550 | +2.27% (0/6) | A 0/3, K 3/3 | 145.4 |
| 1×16 | catalog | 16+16 | Tile A and K | 12.592 | +2.30% (0/6) | A 3/3, K 3/3 | 148.1 |
| 1×16 | catalog | 20+12 | Ordinary grouping | 12.038 | +0.00% (0/6) | A 0/3, K 0/3 | 151.1 |
| 1×16 | catalog | 20+12 | Tile A | 12.112 | +0.43% (2/6) | A 3/3, K 0/3 | 149.6 |
| 1×16 | catalog | 20+12 | Tile K | 12.311 | +2.59% (0/6) | A 0/3, K 3/3 | 169.0 |
| 1×16 | catalog | 20+12 | Tile A and K | 12.304 | +2.11% (0/6) | A 3/3, K 3/3 | 165.9 |
| 1×16 | heldout | 16+16 | Ordinary grouping | 12.368 | +0.00% (0/6) | A 0/3, K 0/3 | 142.4 |
| 1×16 | heldout | 16+16 | Tile A | 12.410 | +0.12% (3/6) | A 3/3, K 0/3 | 141.6 |
| 1×16 | heldout | 16+16 | Tile K | 12.591 | +1.91% (2/6) | A 0/3, K 3/3 | 148.9 |
| 1×16 | heldout | 16+16 | Tile A and K | 12.610 | +1.21% (1/6) | A 3/3, K 3/3 | 145.9 |
| 1×16 | heldout | 20+12 | Ordinary grouping | 12.086 | +0.00% (0/6) | A 0/3, K 0/3 | 155.9 |
| 1×16 | heldout | 20+12 | Tile A | 12.093 | +0.16% (3/6) | A 3/3, K 0/3 | 152.2 |
| 1×16 | heldout | 20+12 | Tile K | 12.254 | +1.63% (0/6) | A 0/3, K 3/3 | 169.5 |
| 1×16 | heldout | 20+12 | Tile A and K | 12.313 | +2.02% (0/6) | A 3/3, K 3/3 | 164.1 |
| 2×2 | catalog | 16+16 | Ordinary grouping | 10.382 | +0.00% (0/6) | A 0/3 | 87.0 |
| 2×2 | catalog | 16+16 | Tile A | 10.455 | +0.54% (2/6) | A 3/3 | 83.5 |
| 2×2 | catalog | 20+12 | Ordinary grouping | 10.063 | +0.00% (0/6) | A 0/3 | 89.2 |
| 2×2 | catalog | 20+12 | Tile A | 10.111 | +0.18% (1/6) | A 3/3 | 90.5 |
| 2×2 | heldout | 16+16 | Ordinary grouping | 10.411 | +0.00% (0/6) | A 0/3 | 83.5 |
| 2×2 | heldout | 16+16 | Tile A | 10.392 | -0.22% (5/6) | A 3/3 | 87.0 |
| 2×2 | heldout | 20+12 | Ordinary grouping | 10.035 | +0.00% (0/6) | A 0/3 | 85.2 |
| 2×2 | heldout | 20+12 | Tile A | 10.134 | +0.39% (2/6) | A 3/3 | 88.2 |

Six timing blocks prime three proofs and time thirteen per process. A separate diagnostic block records which queries used tiles. Two native point-sum tests passed, and Gnark independently verified all 336 exports. The full-proof route for 1×2 and 2×2 uses fused K/H, so this trial applies tiling to A only there.

[Summary](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Trial patch](artifacts/source.patch.gz)
