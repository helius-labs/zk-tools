# Overlapping point grouping and scalar decomposition in G2

On the 16+16 Ryzen layout, coarse decomposition improved the paired catalog medians for 1×8 and 1×16 by 1.89% and 1.98%. The held-out changes were 0.46% faster and 0.22% slower. The existing order remains selected.

After grouping identifies equal scalar classes, point reduction reads the class indices while scalar decomposition mutates the independent unique-scalar array. The candidate runs them together on the existing G2 pool before the same eleven-bit, four-range MSM. The serial arm keeps the original scalar loop. The coarse arm gives Rayon contiguous 1,024-scalar chunks. Both release the class indices after reduction and use the existing four-range workspace charge.

| Shape | Request | Main + G2 | Ordinary / serial / coarse proof ms | Paired serial | Paired coarse | Coarse overlap ms | Peak ordinary / coarse MiB |
|---|---|---:|---:|---:|---:|---:|---:|
| 1×8 | catalog | 8+8 | 12.080 / 12.100 / 12.074 | +0.47% (2/6) | -0.22% (4/6) | 0.167 | 102.5 / 98.5 |
| 1×8 | catalog | 16+16 | 9.849 / 9.762 / 9.682 | -1.27% (5/6) | -1.89% (4/6) | 0.274 | 118.8 / 120.2 |
| 1×8 | heldout | 8+8 | 12.116 / 12.089 / 12.148 | -0.27% (6/6) | +0.05% (2/6) | 0.164 | 100.8 / 98.2 |
| 1×8 | heldout | 16+16 | 9.819 / 9.831 / 9.750 | -0.15% (3/6) | -0.46% (6/6) | 0.258 | 116.5 / 121.5 |
| 1×16 | catalog | 8+8 | 15.343 / 15.315 / 15.309 | +0.46% (2/6) | +0.17% (3/6) | 0.237 | 119.2 / 123.5 |
| 1×16 | catalog | 16+16 | 12.713 / 12.636 / 12.501 | -1.07% (4/6) | -1.98% (6/6) | 0.375 | 142.4 / 145.6 |
| 1×16 | heldout | 8+8 | 15.302 / 15.393 / 15.316 | +0.49% (2/6) | +0.05% (3/6) | 0.208 | 121.9 / 120.1 |
| 1×16 | heldout | 16+16 | 12.719 / 12.653 / 12.719 | -0.37% (4/6) | +0.22% (3/6) | 0.355 | 146.9 / 163.9 |

Six blocks prime three proofs and time thirteen. Separate diagnostics clock two proofs per arm. The 8+8 layout uses sixteen physical Ryzen cores. The 16+16 layout uses SMT siblings. The trial forces four ranges at both widths to exercise the C7a geometry. All 168 exports passed independent verification.

[Summary](artifacts/summary.json) · [Samples and diagnostics](artifacts/rows.json.gz) · [Candidate patch](artifacts/source.patch.gz)
