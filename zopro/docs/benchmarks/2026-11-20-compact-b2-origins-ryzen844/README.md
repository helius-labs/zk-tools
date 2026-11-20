# Compact B2 grouping storage on Ryzen

The compact variant gives each recognized Poseidon state class the number of slots required by its width. The conventional layout uses 624 slots per distinct state. The origin lookup obtains a span index and coordinate with a shift and a mask. The offsets variant uses the existing start offsets as writer cursors and restores them after the stable scatter. The combined variant applies both. Full scalar equality checks and the original working-memory reservation still apply.

Zopro uses 24 main and eight frontend logical CPUs with the 8×2 four-epoch solver profile. The original and first held-out inputs have five matched blocks. The two additional inputs have four. Each timing process primes three proofs and measures thirteen. All 94 process exports independently verified. Three native tests cover state matching, width separation and the restoration of class offsets.

Separate probes reached the compact path at ordinary memory settings and exercised the fallback with zero optional workspace. The compact path used 27492 to 27492 slots across the route probes.

No variant improved both controls consistently across the four inputs. The code remains experimental.

| Shape | Input | Profile | Proof ms | Paired vs same-build control | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 8×2 | catalog | release | 22.521 | +0.48% (1/5) | +0.00% (0/5) | 370.0 |
| 8×2 | catalog | ordinary | 22.413 | +0.00% (0/5) | -0.48% (4/5) | 360.8 |
| 8×2 | catalog | compact | 22.445 | +0.79% (2/5) | +0.08% (2/5) | 382.2 |
| 8×2 | catalog | offsets | 22.645 | +1.12% (1/5) | +0.35% (1/5) | 358.8 |
| 8×2 | catalog | both | 22.742 | +1.47% (1/5) | +1.02% (0/5) | 365.5 |
| 8×2 | heldout | release | 22.879 | +1.90% (1/5) | +0.00% (0/5) | 364.2 |
| 8×2 | heldout | ordinary | 22.465 | +0.00% (0/5) | -1.87% (4/5) | 375.8 |
| 8×2 | heldout | compact | 22.510 | -0.29% (3/5) | -1.02% (3/5) | 356.8 |
| 8×2 | heldout | offsets | 22.772 | +1.06% (0/5) | +0.32% (2/5) | 358.5 |
| 8×2 | heldout | both | 22.734 | +1.39% (0/5) | -0.81% (3/5) | 368.5 |
| 8×2 | v153 | release | 22.755 | +0.61% (1/4) | +0.00% (0/4) | 374.2 |
| 8×2 | v153 | ordinary | 22.687 | +0.00% (0/4) | -0.60% (3/4) | 358.8 |
| 8×2 | v153 | compact | 22.571 | -0.55% (3/4) | -0.81% (3/4) | 373.2 |
| 8×2 | v153 | offsets | 22.713 | -0.44% (3/4) | -0.03% (2/4) | 363.0 |
| 8×2 | v153 | both | 22.825 | +0.99% (1/4) | +0.02% (2/4) | 370.8 |
| 8×2 | v156 | release | 22.707 | +0.20% (2/4) | +0.00% (0/4) | 362.2 |
| 8×2 | v156 | ordinary | 22.625 | +0.00% (0/4) | -0.13% (2/4) | 366.8 |
| 8×2 | v156 | compact | 22.623 | +0.23% (2/4) | -0.16% (2/4) | 369.0 |
| 8×2 | v156 | offsets | 22.625 | +0.35% (2/4) | -0.95% (3/4) | 373.0 |
| 8×2 | v156 | both | 22.732 | +0.55% (2/4) | +0.13% (2/4) | 379.2 |

[Source](artifacts/source.patch) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)
