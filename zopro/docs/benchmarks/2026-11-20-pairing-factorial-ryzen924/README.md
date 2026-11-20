# Isolating packed final exponentiation on 1×2

Packed final exponentiation alone took **6.829 / 6.711 / 6.724 ms** across three inputs and lowered paired 1×2 latency by **0.35% / 0.83% / 0.84%** versus same-build ordinary. Both paired Miller variants fell short of the incremental test against packed final exponentiation. The local results support carrying packed final exponentiation alone into C7a confirmation.

This run uses the exact [paired Miller executable](../2026-11-20-paired-sparse-miller-ryzen923/README.md). It measures ordinary, scalar and IFMA Miller paths with ordinary and packed final exponentiation, plus the frozen release. The host uses 24 main and eight frontend Ryzen hardware threads. Each process primes three calls and measures seventeen. Eight counterbalanced blocks cover the original, held-out, and v27 inputs. All 168 process exports independently verified with Gnark using the same proving image and three distinct public inputs.

The table reports paired medians in original, held-out and v27 order. Ordinary calibration versus frozen release was -0.62% / -0.11% / +0.12%. Advancement requires calibration within 2% and lower medians with at least six paired wins out of eight against both controls on all three inputs. Packed final exponentiation cleared both controls on its own and with IFMA paired lines. Keeping either paired Miller variant with packed final exponentiation also requires at least six paired wins against packed final alone on every input.

| Mode | Paired change vs ordinary | Paired change vs packed final |
|---|---:|---:|
| Packed final | -0.35% / -0.83% / -0.84% | — |
| Scalar paired lines and packed final | -0.74% / -1.12% / -0.95% | +0.08% / -0.45% / -0.28% |
| IFMA paired lines and packed final | -0.80% / -1.10% / -1.11% | -0.21% / +0.04% / +0.02% |

The within-block interaction subtracts the two isolated changes from the observed joint change. The medians in microseconds were -10.8 / -100.3 / +13.3 for scalar lines and +11.4 / +10.1 / -6.1 for IFMA lines.

<details>
<summary>Proof timings and peak process RSS</summary>

| Input | Mode | Proof ms | Paired vs release | Paired vs ordinary | Paired vs packed final | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| catalog | release | 6.898 | +0.00% (0/8) | +0.62% (3/8) | +1.06% (1/8) | 58.5 |
| catalog | ordinary | 6.872 | -0.62% (5/8) | +0.00% (0/8) | +0.35% (2/8) | 58.5 |
| catalog | paired | 6.871 | -0.16% (4/8) | -0.13% (4/8) | +0.57% (2/8) | 58.8 |
| catalog | ifma | 6.833 | -1.03% (7/8) | -0.46% (5/8) | -0.05% (4/8) | 58.2 |
| catalog | final | 6.829 | -1.05% (7/8) | -0.35% (6/8) | +0.00% (0/8) | 58.8 |
| catalog | combined | 6.851 | -0.68% (7/8) | -0.74% (6/8) | +0.08% (4/8) | 57.2 |
| catalog | ifma-final | 6.813 | -1.07% (7/8) | -0.80% (7/8) | -0.21% (5/8) | 58.0 |
| heldout | release | 6.749 | +0.00% (0/8) | +0.11% (4/8) | +0.84% (0/8) | 57.5 |
| heldout | ordinary | 6.763 | -0.11% (4/8) | +0.00% (0/8) | +0.84% (1/8) | 58.2 |
| heldout | paired | 6.789 | +0.27% (4/8) | +0.44% (2/8) | +1.04% (1/8) | 58.0 |
| heldout | ifma | 6.736 | -0.54% (6/8) | -0.27% (4/8) | +0.25% (2/8) | 58.2 |
| heldout | final | 6.711 | -0.84% (8/8) | -0.83% (7/8) | +0.00% (0/8) | 57.8 |
| heldout | combined | 6.653 | -1.31% (7/8) | -1.12% (6/8) | -0.45% (6/8) | 58.8 |
| heldout | ifma-final | 6.700 | -0.53% (8/8) | -1.10% (6/8) | +0.04% (3/8) | 58.0 |
| v27 | release | 6.783 | +0.00% (0/8) | -0.12% (5/8) | +1.00% (2/8) | 57.8 |
| v27 | ordinary | 6.778 | +0.12% (3/8) | +0.00% (0/8) | +0.85% (2/8) | 58.0 |
| v27 | paired | 6.763 | -0.07% (4/8) | -0.32% (5/8) | +0.35% (3/8) | 58.2 |
| v27 | ifma | 6.766 | -0.44% (6/8) | +0.15% (4/8) | +0.22% (3/8) | 58.2 |
| v27 | final | 6.724 | -0.99% (6/8) | -0.84% (6/8) | +0.00% (0/8) | 58.2 |
| v27 | combined | 6.720 | -0.78% (6/8) | -0.95% (5/8) | -0.28% (5/8) | 57.5 |
| v27 | ifma-final | 6.734 | -0.90% (7/8) | -1.11% (7/8) | +0.02% (4/8) | 59.2 |

</details>

The C7a score remains five of eight at 10× M5 CPU. The scorecard includes both M5 reference timings for all eight shapes. [C7a scorecard](../2026-11-20-typed-small-proof-c7a898/README.md#eight-supplied-m5-reference-timings)

[Protocol](artifacts/protocol.json) · [Samples](artifacts/rows.json.gz) · [Summary](artifacts/summary.json) · [Proof verification](artifacts/verification.json)
