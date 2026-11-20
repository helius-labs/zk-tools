# Fused K/H queries for larger folded proofs on Ryzen

The trial shares the buckets of the private K and quotient H queries when the folded quotient has unit scale and the CPU path omits B1. One variant feeds both original point slices directly. A second combines equal K scalars before feeding the fused buckets. A third computes two independent fused sums over disjoint ranges. It borrows the original point slices and charges optional workspace before selecting the new route. The arithmetic test compares separate and fused sums across odd-sized partitions.

The 4×2 and 8×2 proofs use 24 main and eight frontend logical CPUs. The 8×2 controls and candidates enable the recorded seeded solver and vector B2 profile. Each input has five matched blocks, three primes and thirteen timed calls per process. All 116 exports independently verified, including the fallback at zero optional workspace.

No fused mode improved both controls consistently across both inputs of either shape. The code remains experimental.



| Shape | Input | Profile | Proof ms | Paired vs same-build control | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 4×2 | catalog | release | 14.380 | +0.04% (2/5) | +0.00% (0/5) | 198.2 |
| 4×2 | catalog | ordinary | 14.407 | +0.00% (0/5) | -0.04% (3/5) | 197.5 |
| 4×2 | catalog | raw | 15.435 | +7.60% (0/5) | +7.56% (1/5) | 187.2 |
| 4×2 | catalog | group | 15.596 | +8.25% (0/5) | +8.75% (1/5) | 204.0 |
| 4×2 | catalog | striped | 15.640 | +8.89% (0/5) | +8.61% (1/5) | 211.0 |
| 4×2 | heldout | release | 14.342 | -0.69% (4/5) | +0.00% (0/5) | 198.0 |
| 4×2 | heldout | ordinary | 14.381 | +0.00% (0/5) | +0.70% (1/5) | 204.0 |
| 4×2 | heldout | raw | 15.261 | +5.74% (0/5) | +6.44% (0/5) | 202.2 |
| 4×2 | heldout | group | 15.523 | +7.46% (0/5) | +8.19% (0/5) | 192.0 |
| 4×2 | heldout | striped | 15.616 | +8.72% (0/5) | +9.80% (0/5) | 198.0 |
| 8×2 | catalog | release | 22.181 | +0.22% (1/5) | +0.00% (0/5) | 377.5 |
| 8×2 | catalog | ordinary | 22.236 | +0.00% (0/5) | -0.22% (4/5) | 358.0 |
| 8×2 | catalog | raw | 24.655 | +10.88% (0/5) | +11.16% (0/5) | 365.5 |
| 8×2 | catalog | group | 25.044 | +13.65% (0/5) | +12.41% (0/5) | 361.2 |
| 8×2 | catalog | striped | 24.723 | +11.75% (0/5) | +10.98% (0/5) | 348.5 |
| 8×2 | heldout | release | 22.596 | -0.47% (3/5) | +0.00% (0/5) | 370.0 |
| 8×2 | heldout | ordinary | 22.472 | +0.00% (0/5) | +0.47% (2/5) | 376.0 |
| 8×2 | heldout | raw | 24.869 | +10.35% (0/5) | +10.53% (0/5) | 360.8 |
| 8×2 | heldout | group | 25.300 | +12.59% (0/5) | +11.46% (0/5) | 362.8 |
| 8×2 | heldout | striped | 25.112 | +11.99% (0/5) | +12.13% (0/5) | 355.0 |

## Observed query sizes

The final column gives the K scalar classes admitted by the ordinary exact grouping step. Zero indicates that the step declined and the fused query used the original slice.

| Shape | Input | Probe | K terms | H terms | K classes |
|---|---|---|---:|---:|---:|
| 4×2 | catalog | group | 97,075 | 98,674 | 29,828 |
| 4×2 | catalog | striped | 97,075 | 98,674 | 29,828 |
| 4×2 | heldout | group | 97,075 | 98,674 | 30,052 |
| 4×2 | heldout | striped | 97,075 | 98,674 | 30,052 |
| 8×2 | catalog | group | 186,411 | 189,618 | 37,973 |
| 8×2 | catalog | striped | 186,411 | 189,618 | 37,973 |
| 8×2 | heldout | group | 186,411 | 189,618 | 39,792 |
| 8×2 | heldout | striped | 186,411 | 189,618 | 39,792 |

[Source](artifacts/source.patch) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)
