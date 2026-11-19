# Exact private-scalar grouping before fused K/H

The diagnostic adds exact grouping to the private K terms before their fused MSM with the quotient H. It sums the affine points that share a canonical scalar and runs the existing fused kernel over the reduced K and original H inputs. All buffers live within the proof's optional preparation policy. The ordinary fused path handles high diversity and unavailable budget.

The 1×2 release and same-build control use the ordinary fused implementation. The 2×2 trial enables the existing medium K/H option in both same-build arms. The Ryzen runs use 24 main and eight frontend logical CPUs with six matched blocks per input. Each process warms three calls and measures twenty-one. All 84 exports independently verified. Separate routing checks ran each input with the zero optional-memory allowance. A zero observed class count denotes a declined grouping call. A dash denotes a fused grouping call that was never reached.

- 1×2 changed by -0.28% and -0.04% versus the same-build control.
- 2×2 changed by +4.21% and +4.03% versus the same-build control.

| Shape | Input | K terms | Indexed classes |
|---|---|---:|---:|
| 1×2 | catalog | 30529 | 0 |
| 1×2 | heldout | 30529 | 0 |
| 2×2 | catalog | 53079 | 27287 |
| 2×2 | heldout | 53079 | 27340 |

| Shape | Input | Profile | Proof ms | Paired vs same-build control | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1×2 | catalog | release | 6.907 | -0.59% (5/6) | +0.00% (0/6) | 57.8 |
| 1×2 | catalog | ordinary | 6.948 | +0.00% (0/6) | +0.60% (1/6) | 58.0 |
| 1×2 | catalog | group | 6.898 | -0.28% (3/6) | +0.33% (3/6) | 58.5 |
| 1×2 | heldout | release | 6.825 | +0.18% (3/6) | +0.00% (0/6) | 58.0 |
| 1×2 | heldout | ordinary | 6.815 | +0.00% (0/6) | -0.18% (3/6) | 58.8 |
| 1×2 | heldout | group | 6.828 | -0.04% (3/6) | +0.17% (3/6) | 59.8 |
| 2×2 | catalog | release | 9.830 | +3.20% (0/6) | +0.00% (0/6) | 105.8 |
| 2×2 | catalog | ordinary | 9.540 | +0.00% (0/6) | -3.10% (6/6) | 93.8 |
| 2×2 | catalog | group | 9.930 | +4.21% (0/6) | +1.13% (0/6) | 96.2 |
| 2×2 | heldout | release | 9.803 | +2.71% (0/6) | +0.00% (0/6) | 104.0 |
| 2×2 | heldout | ordinary | 9.564 | +0.00% (0/6) | -2.64% (6/6) | 100.5 |
| 2×2 | heldout | group | 9.903 | +4.03% (0/6) | +1.16% (0/6) | 94.2 |

[Samples](artifacts/rows.json.gz) · [Routes](artifacts/routes.json) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)
