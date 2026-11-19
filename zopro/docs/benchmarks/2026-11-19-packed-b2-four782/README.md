# Mapped replacement B2 on 4×2

The starting partition came from the [earlier point-layout study](../2026-11-19-packed-basis505/README.md). Four groups touched public-wire offsets and were removed. All of their members stay as independent points. The replacement retains the original 95,537 points and adds 764,312 bytes of index metadata. The sparse prefix has 44,072 points. A differential checker reconstructed every original point and compared original and transformed B2 sums on two archived vectors and an independent full-width random vector.

The same SDK compares the ordinary image, the packed image with the exclusive G2 kernel, and the packed image with the ordinary grouped kernel. Both packed arms use direct equality checks for residual detection and compute exact scalar differences for the emitted corrections. The packed offset file is attached to its matching image. All proofs verify against the original image and public witness.

Five matched blocks on four inputs use Ryzen with 24 main and eight frontend logical threads. Each process discarded three priming requests and timed thirteen. All 92 proof exports independently verified. Route checks include a zero optional-memory allowance.

A packed profile led both controls on all four inputs.

| Input | Profile | Proof ms | Paired vs same build, ordinary image | Paired vs released SDK | Paired vs packed exclusive | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| catalog | release | 14.481 | -0.00% (3/5) | +0.00% (0/5) | +3.99% (0/5) | 202.0 |
| catalog | ordinary | 14.423 | +0.00% (0/5) | +0.00% (2/5) | +3.37% (0/5) | 216.0 |
| catalog | packed | 13.981 | -3.26% (5/5) | -3.84% (5/5) | +0.00% (0/5) | 223.8 |
| catalog | packed-raw | 14.293 | -1.21% (3/5) | -0.34% (3/5) | +2.22% (0/5) | 212.5 |
| heldout | release | 14.431 | +0.22% (2/5) | +0.00% (0/5) | +5.09% (0/5) | 196.2 |
| heldout | ordinary | 14.421 | +0.00% (0/5) | -0.22% (3/5) | +4.36% (0/5) | 202.2 |
| heldout | packed | 13.714 | -4.18% (5/5) | -4.85% (5/5) | +0.00% (0/5) | 210.5 |
| heldout | packed-raw | 14.146 | -1.10% (4/5) | -1.30% (4/5) | +3.15% (0/5) | 225.0 |
| v153 | release | 14.433 | -0.64% (3/5) | +0.00% (0/5) | +3.46% (0/5) | 203.5 |
| v153 | ordinary | 14.477 | +0.00% (0/5) | +0.65% (2/5) | +4.67% (1/5) | 203.5 |
| v153 | packed | 13.950 | -4.46% (4/5) | -3.35% (5/5) | +0.00% (0/5) | 222.5 |
| v153 | packed-raw | 14.488 | -0.18% (3/5) | -0.06% (3/5) | +2.87% (0/5) | 215.5 |
| v156 | release | 14.461 | +0.03% (2/5) | +0.00% (0/5) | +3.57% (0/5) | 201.0 |
| v156 | ordinary | 14.457 | +0.00% (0/5) | -0.03% (3/5) | +3.28% (0/5) | 194.0 |
| v156 | packed | 13.918 | -3.18% (5/5) | -3.44% (5/5) | +0.00% (0/5) | 209.5 |
| v156 | packed-raw | 14.373 | +0.06% (2/5) | -0.83% (4/5) | +3.27% (0/5) | 222.2 |

| Input | Profile | Route | Prefix / residual count for sparse, mismatches / checked for dense |
|---|---|---|---:|
| catalog | packed | sparse | 44072 / 38 |
| catalog | packed-raw | sparse | 44072 / 38 |
| catalog | packed-raw-limited | dense | 38 / 51465 |
| heldout | packed | sparse | 44072 / 0 |
| heldout | packed-raw | sparse | 44072 / 0 |
| heldout | packed-raw-limited | dense | 0 / 51465 |
| v153 | packed | sparse | 44072 / 0 |
| v153 | packed-raw | sparse | 44072 / 0 |
| v153 | packed-raw-limited | dense | 0 / 51465 |
| v156 | packed | sparse | 44072 / 0 |
| v156 | packed-raw | sparse | 44072 / 0 |
| v156 | packed-raw-limited | dense | 0 / 51465 |

[Point check](artifacts/layout-check.json) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Kernel switch](artifacts/kernel.patch) · [Verification](artifacts/verification.json)
