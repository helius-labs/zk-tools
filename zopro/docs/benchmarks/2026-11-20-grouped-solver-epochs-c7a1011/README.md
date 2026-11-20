# Solver epochs and LLC placement on C7a

Both shapes use disjoint 72+24 core pools on one c7a.24xlarge. The controls keep the selected solver widths of 16 for 4×2 and 20 for 8×2. The two mask variants redistribute the same solver workers across the three frontend L3 groups. The other arms change the number of solver epochs on the control mask.

| Shape | Request | Selected | Selected / control ms | Paired vs control | M5 CPU / Metal | 10× CPU blocks |
|---|---|---|---:|---:|---:|---:|
| 4×2 | catalog | Selected control | 13.958 / 13.958 | +0.00% (0/10) | 7.81× / 3.44× | 0/10 |
| 4×2 | heldout | Selected control | 13.919 / 13.919 | +0.00% (0/10) | 7.83× / 3.45× | 0/10 |
| 8×2 | catalog | Selected control | 18.574 / 18.574 | +0.00% (0/10) | 10.18× / 3.98× | 9/10 |
| 8×2 | heldout | Selected control | 18.920 / 18.920 | +0.00% (0/10) | 9.99× / 3.91× | 4/10 |

A new profile qualifies with at least seven paired wins in ten blocks and a lower paired median on both requests. When several qualify, selection uses the lowest worse-request median. Each process primes three proofs and times seventeen.

## 8×2 against the 18.9 ms M5 CPU target

| Profile | Catalog proof ms | Held-out proof ms | Catalog / held-out M5 CPU | Catalog / held-out blocks at target |
|---|---:|---:|---:|---:|
| Selected control | 18.574 | 18.920 | 10.18× / 9.99× | 9/10 / 4/10 |
| Balanced LLC mask | 18.646 | 18.866 | 10.14× / 10.02× | 10/10 / 5/10 |
| Alternate LLC mask | 18.777 | 18.834 | 10.07× / 10.04× | 8/10 / 6/10 |

The masks shown below the control reached the target on both request medians in this run.

| Shape | Request | Profile | Proof ms | Paired vs control | Paired vs Aeglos | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 4×2 | catalog | Pinned catalog | 14.242 | +2.10% (0/10) | -21.81% (10/10) | 214.7 |
| 4×2 | catalog | Selected control | 13.958 | +0.00% (0/10) | -23.14% (10/10) | 213.2 |
| 4×2 | catalog | Balanced LLC mask | 14.148 | +1.81% (3/10) | -22.48% (10/10) | 219.1 |
| 4×2 | catalog | Alternate LLC mask | 14.010 | +0.28% (3/10) | -23.10% (10/10) | 210.2 |
| 4×2 | catalog | Three solver epochs | 14.345 | +2.40% (1/10) | -21.03% (10/10) | 211.0 |
| 4×2 | catalog | Five solver epochs | 13.913 | -0.25% (7/10) | -23.84% (10/10) | 223.7 |
| 4×2 | catalog | Two solver epochs | 15.971 | +14.59% (0/10) | -12.27% (10/10) | 211.7 |
| 4×2 | catalog | Prepared Aeglos | 18.252 | +30.10% (0/10) | +0.00% (0/10) | 1348.8 |
| 4×2 | heldout | Pinned catalog | 14.128 | +1.67% (2/10) | -22.10% (10/10) | 214.7 |
| 4×2 | heldout | Selected control | 13.919 | +0.00% (0/10) | -23.34% (10/10) | 223.0 |
| 4×2 | heldout | Balanced LLC mask | 13.991 | +0.22% (4/10) | -22.64% (10/10) | 210.9 |
| 4×2 | heldout | Alternate LLC mask | 14.020 | +1.04% (3/10) | -22.58% (10/10) | 211.7 |
| 4×2 | heldout | Three solver epochs | 14.268 | +2.75% (0/10) | -21.64% (10/10) | 219.9 |
| 4×2 | heldout | Five solver epochs | 13.975 | +0.56% (4/10) | -23.02% (10/10) | 214.0 |
| 4×2 | heldout | Two solver epochs | 16.060 | +15.69% (0/10) | -11.61% (10/10) | 218.4 |
| 4×2 | heldout | Prepared Aeglos | 18.104 | +30.45% (0/10) | +0.00% (0/10) | 1349.7 |
| 8×2 | catalog | Pinned catalog | 19.044 | +2.89% (0/10) | -38.24% (10/10) | 389.6 |
| 8×2 | catalog | Selected control | 18.574 | +0.00% (0/10) | -39.82% (10/10) | 356.9 |
| 8×2 | catalog | Balanced LLC mask | 18.646 | +0.29% (5/10) | -39.59% (10/10) | 360.3 |
| 8×2 | catalog | Alternate LLC mask | 18.777 | +0.97% (2/10) | -39.06% (10/10) | 371.1 |
| 8×2 | catalog | Three solver epochs | 19.307 | +4.70% (0/10) | -37.24% (10/10) | 368.7 |
| 8×2 | catalog | Five solver epochs | 18.880 | +1.56% (2/10) | -38.52% (10/10) | 363.7 |
| 8×2 | catalog | Two solver epochs | 20.143 | +8.29% (0/10) | -34.64% (10/10) | 363.7 |
| 8×2 | catalog | Prepared Aeglos | 30.906 | +66.19% (0/10) | +0.00% (0/10) | 2491.6 |
| 8×2 | heldout | Pinned catalog | 19.316 | +2.02% (0/10) | -37.14% (10/10) | 394.2 |
| 8×2 | heldout | Selected control | 18.920 | +0.00% (0/10) | -38.31% (10/10) | 364.9 |
| 8×2 | heldout | Balanced LLC mask | 18.866 | -0.37% (6/10) | -38.47% (10/10) | 371.4 |
| 8×2 | heldout | Alternate LLC mask | 18.834 | -0.09% (6/10) | -38.23% (10/10) | 380.3 |
| 8×2 | heldout | Three solver epochs | 19.757 | +4.38% (0/10) | -35.63% (10/10) | 372.7 |
| 8×2 | heldout | Five solver epochs | 19.093 | +0.64% (3/10) | -38.49% (10/10) | 374.1 |
| 8×2 | heldout | Two solver epochs | 20.356 | +7.55% (0/10) | -33.25% (10/10) | 366.7 |
| 8×2 | heldout | Prepared Aeglos | 30.600 | +62.11% (0/10) | +0.00% (0/10) | 2476.5 |

All 320 C7a exports and six local setup exports passed independent verification. The temporary AWS resources were removed. The [eight-shape scorecard](../2026-11-20-grouped-solver-widths-c7a1006/README.md#all-eight-supplied-m5-references) contains the other CPU and Metal comparisons.

[Summary](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Cleanup](artifacts/cleanup.json)
