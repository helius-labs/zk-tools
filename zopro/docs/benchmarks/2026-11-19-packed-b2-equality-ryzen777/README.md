# Checking B2 equality in a mapped replacement image

The archived [invertible B2 layout](../2026-11-19-packed-sdk506/README.md) replaces the original 8×2 point section with the same number of points and adds a 1,466,792-byte index trailer. The sparse route starts with a 63,474-point prefix. A mismatched member adds its exact scalar difference through the retained original point. The dense calculation applies the same identity to every term, including when optional scratch is denied.

This run ports the archived layout to the current SDK. The trial checks scalar equality directly when counting residuals. The original packed arm computes a field subtraction and tests the result for zero. Both compute the full field difference for every residual term that enters the proof. The ordinary image run from the same executable controls for the loader and code generation.

Five matched blocks on four 8×2 inputs used 24 main and eight frontend logical threads on Ryzen. Each process discarded three priming requests and timed thirteen proofs. All 92 exported proofs independently verified against the ordinary proving-key image. Separate route checks cover forced dense calculation and a zero optional-memory allowance.

A packed profile led both controls on all four measured inputs.

| Input | Profile | Proof ms | Paired vs same build, ordinary image | Paired vs released SDK | Paired vs original packed | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| catalog | release | 22.288 | -0.25% (3/5) | +0.00% (0/5) | +3.90% (0/5) | 405.8 |
| catalog | off | 22.499 | +0.00% (0/5) | +0.25% (2/5) | +4.21% (0/5) | 404.0 |
| catalog | packed | 21.538 | -4.04% (5/5) | -3.75% (5/5) | +0.00% (0/5) | 401.2 |
| catalog | packed-eq | 21.355 | -4.42% (5/5) | -4.18% (4/5) | -0.48% (3/5) | 403.5 |
| heldout | release | 22.409 | -0.66% (4/5) | +0.00% (0/5) | +4.86% (0/5) | 410.2 |
| heldout | off | 22.848 | +0.00% (0/5) | +0.67% (1/5) | +6.44% (0/5) | 410.5 |
| heldout | packed | 21.305 | -6.05% (5/5) | -4.63% (5/5) | +0.00% (0/5) | 406.8 |
| heldout | packed-eq | 21.490 | -5.95% (5/5) | -4.67% (5/5) | +1.05% (1/5) | 410.0 |
| v153 | release | 22.361 | -1.35% (5/5) | +0.00% (0/5) | +5.78% (0/5) | 404.0 |
| v153 | off | 22.694 | +0.00% (0/5) | +1.37% (0/5) | +7.36% (0/5) | 405.8 |
| v153 | packed | 21.139 | -6.85% (5/5) | -5.46% (5/5) | +0.00% (0/5) | 392.0 |
| v153 | packed-eq | 21.367 | -5.96% (4/5) | -4.68% (4/5) | +1.03% (0/5) | 407.5 |
| v156 | release | 22.574 | -1.19% (5/5) | +0.00% (0/5) | +4.48% (0/5) | 393.8 |
| v156 | off | 22.791 | +0.00% (0/5) | +1.20% (0/5) | +5.30% (0/5) | 415.2 |
| v156 | packed | 21.643 | -5.04% (5/5) | -4.29% (5/5) | +0.00% (0/5) | 405.8 |
| v156 | packed-eq | 21.458 | -5.71% (5/5) | -4.58% (5/5) | -0.86% (4/5) | 395.0 |

| Input | Profile | Route | Prefix / residual count for sparse, mismatches / checked for dense |
|---|---|---|---:|
| catalog | packed | sparse | 63474 / 779 |
| catalog | packed-eq | sparse | 63474 / 779 |
| catalog | packed-eq-limited | dense | 779 / 119873 |
| catalog | packed-eq-dense | dense | 0 / 64 |
| heldout | packed | sparse | 63474 / 0 |
| heldout | packed-eq | sparse | 63474 / 0 |
| heldout | packed-eq-limited | dense | 0 / 119873 |
| heldout | packed-eq-dense | dense | 0 / 64 |
| v153 | packed | sparse | 63474 / 0 |
| v153 | packed-eq | sparse | 63474 / 0 |
| v156 | packed | sparse | 63474 / 0 |
| v156 | packed-eq | sparse | 63474 / 0 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json) · [C7a scorecard](../2026-11-19-b2-origin-clock-c7a776/README.md)
