# Combining the packed B2 prefix and correction MSM

The original 8×2 input has 779 nonzero corrections in the [mapped replacement image](../2026-11-19-packed-b2-equality-c7a778/README.md). Its held-out input has zero. The trial groups the packed prefix and exact corrections in one MSM. Only the correction points are copied. Grouped reduction borrows the mapped prefix. If grouping declines, the same input arrays feed the original two calculations. The scratch reservation covers both prefix buffer capacities if extending the vector requires moving it.

Five matched blocks on both inputs use Ryzen with 24 main and eight frontend logical threads. Each process discards three warmups and times thirteen full proofs. The separate packed path from the same executable and the earlier packed executable provide controls. Route checks confirm the join and the zero-memory fallback. All 46 proof exports independently verified against the ordinary key.

The merged MSM led both packed controls on the original input.

| Input | Profile | Proof ms | Paired vs same build, separate queries | Paired vs frozen packed | Paired vs released SDK | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| catalog | release | 22.188 | +3.97% (0/5) | +3.47% (0/5) | +0.00% (0/5) | 401.0 |
| catalog | packed-eq | 21.318 | +0.00% (0/5) | -1.11% (4/5) | -3.82% (5/5) | 392.5 |
| catalog | packed-eq-frozen | 21.445 | +1.12% (1/5) | +0.00% (0/5) | -3.35% (5/5) | 400.0 |
| catalog | packed-eq-joined | 21.347 | -0.23% (3/5) | -1.03% (4/5) | -3.41% (5/5) | 392.2 |
| heldout | release | 22.517 | +5.19% (0/5) | +4.65% (0/5) | +0.00% (0/5) | 417.2 |
| heldout | packed-eq | 21.433 | +0.00% (0/5) | -0.47% (3/5) | -4.94% (5/5) | 393.0 |
| heldout | packed-eq-frozen | 21.529 | +0.47% (2/5) | +0.00% (0/5) | -4.44% (5/5) | 415.2 |
| heldout | packed-eq-joined | 21.517 | +0.39% (2/5) | -0.01% (3/5) | -4.72% (5/5) | 417.5 |

| Input | Profile | Route | Prefix / correction count for sparse or joined |
|---|---|---|---:|
| catalog | packed-eq | sparse | 63474 / 779 |
| catalog | packed-eq-joined | joined | 63474 / 779 |
| catalog | packed-eq-joined-limited | dense | 779 / 119873 |
| heldout | packed-eq | sparse | 63474 / 0 |
| heldout | packed-eq-joined | sparse | 63474 / 0 |
| heldout | packed-eq-joined-limited | dense | 0 / 119873 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/joined.patch) · [Verification](artifacts/verification.json)
