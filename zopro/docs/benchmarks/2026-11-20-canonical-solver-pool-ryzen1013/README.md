# An independent witness instruction pool on the canonical path

The canonical-wire route previously used the ambient pool for witness instructions. The trial forwards an explicitly configured smaller solver pool to the existing executor. Allocation, scalar gathering, the quotient, and G1/G2 placement follow the same route as before. The worker masks use 16+16 Ryzen logical CPUs, with the solver using a subset of the G2 mask before G2 starts.

| Shape | Request | Selected profile | Proof ms | Paired vs main | Instructions ms in diagnostics | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1×4 | catalog | Main (16) (selected) | 8.153 | +0.00% (0/6) | 0.811 | 128.2 |
| 1×4 | catalog | Solver 4 | 8.813 | +8.40% (1/6) | 1.460 | 133.2 |
| 1×4 | catalog | Solver 8 | 8.553 | +4.72% (1/6) | 1.160 | 116.2 |
| 1×4 | catalog | Solver 12 | 8.565 | +4.72% (1/6) | 1.191 | 118.5 |
| 1×4 | heldout | Main (16) (selected) | 8.200 | +0.00% (0/6) | 0.812 | 110.5 |
| 1×4 | heldout | Solver 4 | 8.796 | +7.73% (1/6) | 1.408 | 103.0 |
| 1×4 | heldout | Solver 8 | 8.522 | +5.00% (0/6) | 1.195 | 126.0 |
| 1×4 | heldout | Solver 12 | 8.534 | +4.83% (1/6) | 1.197 | 101.5 |
| 1×8 | catalog | Main (16) (selected) | 9.661 | +0.00% (0/6) | 1.207 | 121.2 |
| 1×8 | catalog | Solver 4 | 10.343 | +6.79% (0/6) | 1.848 | 119.0 |
| 1×8 | catalog | Solver 8 | 10.121 | +4.84% (0/6) | 1.631 | 142.8 |
| 1×8 | catalog | Solver 12 | 10.055 | +4.04% (0/6) | 1.624 | 126.0 |
| 1×8 | heldout | Main (16) (selected) | 9.687 | +0.00% (0/6) | 1.172 | 128.8 |
| 1×8 | heldout | Solver 4 | 10.325 | +5.77% (2/6) | 1.813 | 136.2 |
| 1×8 | heldout | Solver 8 | 10.133 | +4.61% (2/6) | 1.628 | 120.0 |
| 1×8 | heldout | Solver 12 | 10.041 | +3.97% (1/6) | 1.628 | 136.0 |
| 1×16 | catalog | Main (16) (selected) | 12.458 | +0.00% (0/6) | 1.865 | 143.2 |
| 1×16 | catalog | Solver 4 | 13.140 | +5.74% (0/6) | 2.542 | 140.9 |
| 1×16 | catalog | Solver 8 | 12.567 | +1.30% (1/6) | 2.079 | 151.9 |
| 1×16 | catalog | Solver 12 | 12.709 | +2.46% (0/6) | 2.112 | 139.5 |
| 1×16 | heldout | Main (16) (selected) | 12.578 | +0.00% (0/6) | 1.858 | 152.4 |
| 1×16 | heldout | Solver 4 | 13.192 | +5.00% (0/6) | 2.605 | 154.0 |
| 1×16 | heldout | Solver 8 | 12.819 | +1.55% (1/6) | 2.089 | 149.2 |
| 1×16 | heldout | Solver 12 | 12.750 | +1.55% (1/6) | 2.141 | 173.1 |
| 2×2 | catalog | Main (16) (selected) | 10.015 | +0.00% (0/6) | 0.920 | 135.2 |
| 2×2 | catalog | Solver 4 | 10.193 | +1.98% (0/6) | 1.181 | 124.5 |
| 2×2 | catalog | Solver 8 | 10.024 | +0.44% (2/6) | 0.842 | 111.8 |
| 2×2 | catalog | Solver 12 | 9.949 | -0.76% (4/6) | 0.901 | 102.8 |
| 2×2 | heldout | Main (16) (selected) | 9.911 | +0.00% (0/6) | 0.887 | 88.2 |
| 2×2 | heldout | Solver 4 | 10.255 | +3.13% (0/6) | 1.178 | 90.5 |
| 2×2 | heldout | Solver 8 | 9.856 | -0.74% (5/6) | 0.858 | 90.0 |
| 2×2 | heldout | Solver 12 | 9.914 | +0.06% (2/6) | 0.904 | 89.0 |

Selection requires at least five paired wins in six blocks and a lower paired median on both requests. Among qualifying widths, selection minimizes the worse-request median. Pools were confirmed from within the instruction executor during separate two-proof diagnostics. Six timing blocks prime three proofs and time thirteen. All 224 exports passed independent verification.

[Summary](artifacts/summary.json) · [Samples and diagnostics](artifacts/rows.json.gz) · [Candidate patch](artifacts/source.patch.gz)
