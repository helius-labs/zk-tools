# Dedicated worker pool for one G1 query on Ryzen

Placing H on a separate pool lowered paired latency by 1.37–1.62% on 1×4. Placing K there lowered it by 2.55–3.30% on 1×16. Both won all six paired blocks on both inputs against the faster two-pool control. The small 1×2 and 2×2 proofs stayed close to the faster two-pool control with A on the third pool.

All timed configurations use 32 Ryzen logical CPUs, partitioned by fixed affinity. The extra eight workers are idle, added to the main pool, added to the G2 pool, or assigned to one G1 query. The proof uses the same curve and witness code. The dedicated query reserves its full workspace under the existing budget.

| Shape | Request | Worker placement | Proof ms | Paired vs 20+12 | Paired vs 12+20 | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1×2 | catalog | 12+12, third pool idle | 7.407 | +13.22% (0/6) | -7.74% (6/6) | 52.5 |
| 1×2 | catalog | 20+12, cores to main | 6.536 | +0.00% (0/6) | -18.39% (6/6) | 62.0 |
| 1×2 | catalog | 12+20, cores to G2 | 8.013 | +22.53% (0/6) | +0.00% (0/6) | 55.8 |
| 1×2 | catalog | 12+12+8, A | 6.525 | +0.21% (3/6) | -18.87% (6/6) | 57.2 |
| 1×2 | catalog | 12+12+8, K/H | 8.747 | +34.40% (0/6) | +8.95% (0/6) | 56.8 |
| 1×2 | heldout | 12+12, third pool idle | 7.226 | +12.86% (0/6) | -6.67% (6/6) | 53.0 |
| 1×2 | heldout | 20+12, cores to main | 6.368 | +0.00% (0/6) | -17.24% (6/6) | 59.2 |
| 1×2 | heldout | 12+20, cores to G2 | 7.748 | +20.84% (0/6) | +0.00% (0/6) | 55.0 |
| 1×2 | heldout | 12+12+8, A | 6.348 | -0.41% (5/6) | -18.20% (6/6) | 57.5 |
| 1×2 | heldout | 12+12+8, K/H | 8.348 | +30.06% (0/6) | +8.01% (1/6) | 56.2 |
| 1×4 | catalog | 12+12, third pool idle | 9.191 | +12.46% (0/6) | -4.71% (5/6) | 83.5 |
| 1×4 | catalog | 20+12, cores to main | 8.174 | +0.00% (0/6) | -14.99% (6/6) | 106.0 |
| 1×4 | catalog | 12+20, cores to G2 | 9.621 | +17.65% (0/6) | +0.00% (0/6) | 93.5 |
| 1×4 | catalog | 12+12+8, A | 8.104 | -0.69% (4/6) | -15.69% (6/6) | 90.5 |
| 1×4 | catalog | 12+12+8, K | 8.076 | -1.08% (5/6) | -15.72% (6/6) | 68.5 |
| 1×4 | catalog | 12+12+8, H | 8.043 | -1.37% (6/6) | -16.53% (6/6) | 93.0 |
| 1×4 | heldout | 12+12, third pool idle | 9.144 | +11.70% (0/6) | -4.20% (6/6) | 90.2 |
| 1×4 | heldout | 20+12, cores to main | 8.179 | +0.00% (0/6) | -14.32% (6/6) | 103.8 |
| 1×4 | heldout | 12+20, cores to G2 | 9.540 | +16.71% (0/6) | +0.00% (0/6) | 86.8 |
| 1×4 | heldout | 12+12+8, A | 8.074 | -1.29% (4/6) | -15.27% (6/6) | 92.8 |
| 1×4 | heldout | 12+12+8, K | 8.108 | -0.23% (5/6) | -15.01% (6/6) | 69.0 |
| 1×4 | heldout | 12+12+8, H | 8.051 | -1.62% (6/6) | -15.78% (6/6) | 91.2 |
| 1×16 | catalog | 12+12, third pool idle | 13.621 | +13.72% (0/6) | -5.05% (6/6) | 121.1 |
| 1×16 | catalog | 20+12, cores to main | 11.985 | +0.00% (0/6) | -16.70% (6/6) | 154.9 |
| 1×16 | catalog | 12+20, cores to G2 | 14.379 | +20.05% (0/6) | +0.00% (0/6) | 118.9 |
| 1×16 | catalog | 12+12+8, A | 12.292 | +2.47% (0/6) | -14.51% (6/6) | 134.2 |
| 1×16 | catalog | 12+12+8, K | 11.679 | -2.55% (6/6) | -18.82% (6/6) | 93.4 |
| 1×16 | catalog | 12+12+8, H | 11.866 | -1.18% (6/6) | -17.94% (6/6) | 131.4 |
| 1×16 | heldout | 12+12, third pool idle | 13.643 | +12.52% (0/6) | -4.89% (6/6) | 124.7 |
| 1×16 | heldout | 20+12, cores to main | 12.129 | +0.00% (0/6) | -15.34% (6/6) | 154.2 |
| 1×16 | heldout | 12+20, cores to G2 | 14.358 | +18.12% (0/6) | +0.00% (0/6) | 127.2 |
| 1×16 | heldout | 12+12+8, A | 12.294 | +1.36% (0/6) | -14.38% (6/6) | 132.2 |
| 1×16 | heldout | 12+12+8, K | 11.745 | -3.30% (6/6) | -18.46% (6/6) | 91.9 |
| 1×16 | heldout | 12+12+8, H | 11.848 | -2.32% (6/6) | -17.38% (6/6) | 123.5 |
| 2×2 | catalog | 12+12, third pool idle | 10.886 | +7.75% (1/6) | -7.28% (6/6) | 83.5 |
| 2×2 | catalog | 20+12, cores to main | 10.073 | +0.00% (0/6) | -13.97% (5/6) | 89.2 |
| 2×2 | catalog | 12+20, cores to G2 | 11.802 | +16.24% (1/6) | +0.00% (0/6) | 86.5 |
| 2×2 | catalog | 12+12+8, A | 10.086 | -0.05% (3/6) | -14.41% (5/6) | 114.0 |
| 2×2 | catalog | 12+12+8, K/H | 12.704 | +25.96% (1/6) | +7.93% (0/6) | 84.8 |
| 2×2 | heldout | 12+12, third pool idle | 10.999 | +8.91% (0/6) | -5.73% (6/6) | 82.2 |
| 2×2 | heldout | 20+12, cores to main | 10.035 | +0.00% (0/6) | -14.31% (6/6) | 91.0 |
| 2×2 | heldout | 12+20, cores to G2 | 11.691 | +16.70% (0/6) | +0.00% (0/6) | 80.5 |
| 2×2 | heldout | 12+12+8, A | 10.019 | +0.14% (3/6) | -14.33% (6/6) | 95.2 |
| 2×2 | heldout | 12+12+8, K/H | 12.709 | +26.65% (0/6) | +8.30% (0/6) | 84.8 |

Separate diagnostic runs confirm the selected query, worker count and completion timestamps. Main-pool query calls can execute each other’s work via Rayon, so their durations alone do not measure CPU time attributable to individual queries. Six timing blocks prime three proofs and time thirteen per process. All 308 exports passed independent Gnark verification. The runner rejected overlapping CPU masks.

[Summary](artifacts/summary.json) · [Samples and query diagnostics](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Trial patch](artifacts/source.patch.gz)
