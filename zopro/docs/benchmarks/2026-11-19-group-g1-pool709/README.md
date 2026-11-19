# Placing an individual G1 query on six dedicated cores

In the large-key grouped branch, the A and private K queries start alongside H and B2. This trial sends one selected G1 query to the existing frontend pool. The other three stay on the main pool. Before executing on the six auxiliary workers, the selected query checks current memory headroom for its point, grouping and leaf workspace. On denial the uncalled query returns to the main pool. The curve arithmetic and proving key stay identical.

Every process had sixteen physical Ryzen cores. Integrated and shared assigned sixteen main workers with an overlapping four-worker frontend. Idle, A and K use ten main workers and six disjoint frontend workers. A and K differ only in the selected curve query. The zero-budget probe ran on 2×16. Each of the four paired blocks ran three primes and eleven timed proofs. Separate second inputs were used on 2×16 and 4×16. All 195 process exports verified.

The original integrated profile had lower median latency on every tested input against both placements. Sending K alone to the auxiliary pool reduced the ten-core idle latency on 2×16, but its 20.941 ms median still exceeded the integrated run at 19.053 ms. This local result retains the original layout for these grouped queries.

| Shape | Request | G1 query | Integrated / shared / idle / active ms | Active vs idle | Active vs integrated | Peak MiB integrated / active |
|---|---|---|---:|---:|---:|---:|
| 2x16 | catalog | A | 19.053 / 18.561 / 23.470 / 22.022 | -6.54% (4/4) | +16.11% (0/4) | 204.6 / 169.3 |
| 2x16 | catalog | K | 19.053 / 18.561 / 23.470 / 20.941 | -10.68% (4/4) | +10.45% (0/4) | 204.6 / 165.9 |
| 2x16 | heldout | A | 18.841 / 18.677 / 23.529 / 21.934 | -6.78% (4/4) | +15.77% (0/4) | 200.5 / 174.2 |
| 2x16 | heldout | K | 18.841 / 18.677 / 23.529 / 20.775 | -11.94% (4/4) | +11.06% (0/4) | 200.5 / 170.7 |
| 3x2 | catalog | A | 15.166 / 15.015 / 18.834 / 17.705 | -5.88% (4/4) | +17.08% (0/4) | 167.0 / 141.8 |
| 3x2 | catalog | K | 15.166 / 15.015 / 18.834 / 16.932 | -9.78% (4/4) | +11.64% (0/4) | 167.0 / 142.2 |
| 3x4 | catalog | A | 15.957 / 15.803 / 19.900 / 18.556 | -6.92% (4/4) | +16.53% (0/4) | 175.5 / 142.5 |
| 3x4 | catalog | K | 15.957 / 15.803 / 19.900 / 17.684 | -11.17% (4/4) | +11.05% (0/4) | 175.5 / 142.8 |
| 3x8 | catalog | A | 17.874 / 17.610 / 22.235 / 20.580 | -7.22% (4/4) | +15.35% (0/4) | 197.0 / 172.0 |
| 3x8 | catalog | K | 17.874 / 17.610 / 22.235 / 19.613 | -11.83% (4/4) | +9.63% (0/4) | 197.0 / 161.2 |
| 4x16 | catalog | A | 23.026 / 22.916 / 29.349 / 27.299 | -7.04% (4/4) | +18.75% (0/4) | 248.3 / 218.6 |
| 4x16 | catalog | K | 23.026 / 22.916 / 29.349 / 26.054 | -11.86% (4/4) | +13.01% (0/4) | 248.3 / 218.1 |
| 4x16 | heldout | A | 23.131 / 23.061 / 28.978 / 27.222 | -5.95% (4/4) | +17.54% (0/4) | 244.7 / 215.9 |
| 4x16 | heldout | K | 23.131 / 23.061 / 28.978 / 25.952 | -10.45% (4/4) | +11.82% (0/4) | 244.7 / 225.4 |
| 4x4 | catalog | A | 18.038 / 18.038 / 22.726 / 21.494 | -5.39% (4/4) | +18.80% (0/4) | 184.5 / 169.2 |
| 4x4 | catalog | K | 18.038 / 18.038 / 22.726 / 20.362 | -10.54% (4/4) | +13.15% (0/4) | 184.5 / 168.8 |
| 4x8 | catalog | A | 20.128 / 19.878 / 24.915 / 23.336 | -6.23% (4/4) | +16.42% (0/4) | 213.2 / 191.5 |
| 4x8 | catalog | K | 20.128 / 19.878 / 24.915 / 22.485 | -9.52% (4/4) | +11.71% (0/4) | 213.2 / 188.5 |

[Implementation](artifacts/source.patch) · [Dispatch and reserved bytes](artifacts/routes.json) · [Measurements](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
