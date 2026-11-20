# Loading complete eight-row groups in the Euler output pass

The direct load cut paired latency for 4×2 on a physical 12+4 Ryzen layout by 1.32% to 1.63%. On the faster 24+8 layout, the candidate was level or slower across both shapes and requests. The ordinary Euler output remains the baseline.

When eight rows are present in both input ranges, the candidate passes the two array references straight to the existing IFMA conversion. Partial rows keep the padded coefficient loader. The FFT schedule, grouping, curve arithmetic, and storage stay the same. A separate diagnostic confirms entry into the packed-input fused Euler output path.

| Shape | Request | Main + frontend | Archive / ordinary / direct proof ms | Paired direct vs ordinary | Peak ordinary / direct MiB |
|---|---|---:|---:|---:|---:|
| 4×2 | catalog | 12+4 | 20.218 / 20.204 / 19.826 | -1.63% (6/6) | 181.5 / 177.2 |
| 4×2 | catalog | 24+8 | 15.113 / 15.040 / 15.019 | +0.60% (2/6) | 221.5 / 224.2 |
| 4×2 | heldout | 12+4 | 20.000 / 20.178 / 19.868 | -1.32% (5/6) | 204.0 / 182.0 |
| 4×2 | heldout | 24+8 | 15.076 / 15.031 / 15.045 | +0.02% (3/6) | 217.5 / 211.2 |
| 8×2 | catalog | 12+4 | 33.574 / 34.009 / 33.819 | -0.52% (4/6) | 334.8 / 322.0 |
| 8×2 | catalog | 24+8 | 24.878 / 24.584 / 24.688 | +0.04% (3/6) | 434.2 / 370.0 |
| 8×2 | heldout | 12+4 | 34.267 / 34.272 / 34.009 | -0.62% (5/6) | 346.5 / 335.2 |
| 8×2 | heldout | 24+8 | 24.876 / 24.655 / 24.748 | +0.32% (3/6) | 387.8 / 371.0 |

The 12+4 layout uses sixteen physical Ryzen cores. The 24+8 layout uses thirty-two logical CPUs. Six blocks prime three proofs and time thirteen. All 160 process exports passed independent verification.

[Summary](artifacts/summary.json) · [Samples and dispatch checks](artifacts/rows.json.gz) · [Candidate patch](artifacts/source.patch.gz)
