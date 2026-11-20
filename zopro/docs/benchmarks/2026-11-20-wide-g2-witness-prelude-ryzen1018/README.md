# Witness work on the wider G2 pool on Ryzen

At 8+16 workers, moving witness instructions to the G2 pool reduced paired latency by 1.60–2.78% on 1×4 and 1.30–1.35% on 1×16. It won all six blocks on both inputs for each shape. The physical 6+10 layout and both 1×2 and 2×2 layouts fell short of the two-input criterion.

The G2 pool is idle during the canonical witness prelude. The instruction arm uses it only to execute solver instructions. The full arm moves allocation, instructions and witness gathering together. The G1, quotient and G2 pools and the preparation budget stay fixed. The 6+10 layout uses physical cores. The 8+16 layout uses eight main cores and both hardware threads on each of eight separate G2 cores.

| Shape | Input | Main+G2 | Witness profile | Proof ms | Paired vs main | Instructions / prelude ms | Peak MiB |
|---|---|---|---|---:|---:|---:|---:|
| 1×2 | catalog | 6+10 | Main pool | 10.665 | +0.00% (0/6) | 0.640 / 0.745 | 48.8 |
| 1×2 | catalog | 6+10 | G2 pool for instructions | 10.791 | +0.64% (1/6) | 0.717 / 0.854 | 49.8 |
| 1×2 | catalog | 6+10 | G2 pool for full prelude | 10.972 | +3.17% (0/6) | 0.721 / 0.922 | 50.2 |
| 1×2 | catalog | 8+16 | Main pool | 8.796 | +0.00% (0/6) | 0.598 / 0.687 | 57.2 |
| 1×2 | catalog | 8+16 | G2 pool for instructions | 8.984 | +2.73% (0/6) | 0.656 / 0.808 | 56.2 |
| 1×2 | catalog | 8+16 | G2 pool for full prelude | 9.064 | +3.57% (0/6) | 0.739 / 1.034 | 57.5 |
| 1×2 | heldout | 6+10 | Main pool | 10.284 | +0.00% (0/6) | 0.660 / 0.809 | 48.2 |
| 1×2 | heldout | 6+10 | G2 pool for instructions | 10.609 | +4.35% (2/6) | 0.704 / 0.845 | 49.2 |
| 1×2 | heldout | 6+10 | G2 pool for full prelude | 10.489 | +1.23% (3/6) | 0.820 / 1.038 | 49.0 |
| 1×2 | heldout | 8+16 | Main pool | 8.477 | +0.00% (0/6) | 0.606 / 0.712 | 55.8 |
| 1×2 | heldout | 8+16 | G2 pool for instructions | 8.700 | +2.37% (0/6) | 0.665 / 0.798 | 55.5 |
| 1×2 | heldout | 8+16 | G2 pool for full prelude | 8.767 | +3.44% (0/6) | 0.624 / 0.904 | 55.5 |
| 1×4 | catalog | 6+10 | Main pool | 13.307 | +0.00% (0/6) | 1.310 / 1.448 | 73.0 |
| 1×4 | catalog | 6+10 | G2 pool for instructions | 13.274 | -0.55% (4/6) | 1.220 / 1.401 | 71.2 |
| 1×4 | catalog | 6+10 | G2 pool for full prelude | 13.368 | +0.50% (3/6) | 1.177 / 1.385 | 74.2 |
| 1×4 | catalog | 8+16 | Main pool | 10.533 | +0.00% (0/6) | 1.056 / 1.182 | 82.2 |
| 1×4 | catalog | 8+16 | G2 pool for instructions | 10.271 | -2.78% (6/6) | 0.760 / 0.912 | 82.5 |
| 1×4 | catalog | 8+16 | G2 pool for full prelude | 10.273 | -1.84% (6/6) | 0.874 / 1.153 | 86.5 |
| 1×4 | heldout | 6+10 | Main pool | 13.284 | +0.00% (0/6) | 1.324 / 1.470 | 71.5 |
| 1×4 | heldout | 6+10 | G2 pool for instructions | 13.251 | -0.43% (4/6) | 1.193 / 1.360 | 72.5 |
| 1×4 | heldout | 6+10 | G2 pool for full prelude | 13.239 | -0.91% (5/6) | 1.191 / 1.412 | 74.5 |
| 1×4 | heldout | 8+16 | Main pool | 10.501 | +0.00% (0/6) | 1.086 / 1.234 | 80.5 |
| 1×4 | heldout | 8+16 | G2 pool for instructions | 10.295 | -1.60% (6/6) | 0.916 / 1.077 | 81.2 |
| 1×4 | heldout | 8+16 | G2 pool for full prelude | 10.329 | -1.36% (6/6) | 0.866 / 1.112 | 85.0 |
| 1×16 | catalog | 6+10 | Main pool | 19.354 | +0.00% (0/6) | 2.240 / 2.507 | 106.2 |
| 1×16 | catalog | 6+10 | G2 pool for instructions | 19.405 | -0.36% (3/6) | 2.204 / 2.511 | 101.6 |
| 1×16 | catalog | 6+10 | G2 pool for full prelude | 19.453 | -0.29% (3/6) | 2.120 / 2.440 | 108.4 |
| 1×16 | catalog | 8+16 | Main pool | 15.641 | +0.00% (0/6) | 1.884 / 2.139 | 114.5 |
| 1×16 | catalog | 8+16 | G2 pool for instructions | 15.403 | -1.35% (6/6) | 1.711 / 2.003 | 115.2 |
| 1×16 | catalog | 8+16 | G2 pool for full prelude | 15.534 | -0.63% (5/6) | 1.582 / 2.023 | 124.7 |
| 1×16 | heldout | 6+10 | Main pool | 19.318 | +0.00% (0/6) | 2.241 / 2.555 | 101.0 |
| 1×16 | heldout | 6+10 | G2 pool for instructions | 19.418 | +0.79% (1/6) | 2.120 / 2.405 | 101.8 |
| 1×16 | heldout | 6+10 | G2 pool for full prelude | 19.364 | -0.08% (3/6) | 2.384 / 2.781 | 108.2 |
| 1×16 | heldout | 8+16 | Main pool | 15.699 | +0.00% (0/6) | 1.913 / 2.200 | 116.5 |
| 1×16 | heldout | 8+16 | G2 pool for instructions | 15.491 | -1.30% (6/6) | 1.599 / 1.890 | 112.2 |
| 1×16 | heldout | 8+16 | G2 pool for full prelude | 15.478 | -1.34% (6/6) | 1.633 / 2.090 | 120.0 |
| 2×2 | catalog | 6+10 | Main pool | 16.587 | +0.00% (0/6) | 0.880 / 1.132 | 73.8 |
| 2×2 | catalog | 6+10 | G2 pool for instructions | 16.490 | -0.31% (5/6) | 0.983 / 1.215 | 75.5 |
| 2×2 | catalog | 6+10 | G2 pool for full prelude | 16.452 | -0.26% (4/6) | 0.985 / 1.292 | 76.5 |
| 2×2 | catalog | 8+16 | Main pool | 12.639 | +0.00% (0/6) | 0.776 / 1.028 | 87.5 |
| 2×2 | catalog | 8+16 | G2 pool for instructions | 12.990 | +3.50% (0/6) | 0.945 / 1.208 | 79.0 |
| 2×2 | catalog | 8+16 | G2 pool for full prelude | 12.906 | +2.31% (1/6) | 0.777 / 1.197 | 83.8 |
| 2×2 | heldout | 6+10 | Main pool | 16.423 | +0.00% (0/6) | 0.875 / 1.120 | 74.8 |
| 2×2 | heldout | 6+10 | G2 pool for instructions | 16.523 | +0.97% (2/6) | 1.160 / 1.464 | 74.5 |
| 2×2 | heldout | 6+10 | G2 pool for full prelude | 16.441 | +0.98% (2/6) | 1.129 / 1.450 | 76.5 |
| 2×2 | heldout | 8+16 | Main pool | 12.777 | +0.00% (0/6) | 0.766 / 1.017 | 85.2 |
| 2×2 | heldout | 8+16 | G2 pool for instructions | 12.937 | +0.90% (2/6) | 0.781 / 0.995 | 79.5 |
| 2×2 | heldout | 8+16 | G2 pool for full prelude | 13.078 | +1.56% (1/6) | 0.886 / 1.359 | 84.2 |

Six timing blocks prime three proofs and time thirteen per process. A separate two-proof block records the instruction worker count and prelude duration. All 336 proof exports verified independently. The experiment stays in the isolated source tree.

[Summary](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Trial patch](artifacts/source.patch.gz)
