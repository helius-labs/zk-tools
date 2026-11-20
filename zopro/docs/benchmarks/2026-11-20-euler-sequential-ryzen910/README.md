# Reusing one Euler derivative workspace on Ryzen

The ordinary evaluation-basis quotient packs both polynomials into one workspace and transforms their derivatives together. This trial uses all eight lanes for one polynomial at a time. It first forms D(B). As it computes A·D(B) into the existing output vector, it replaces the derivative slots with the original A values. The same workspace then forms D(A), and the final pass adds D(A)·B. D includes the ordinary domain scaling. The input key, roots, solver, commitment and verifier keep the same semantics.

Each compact pack is 256 bytes. The main derivative workspace drops from N/4 to N/8 packs, a saving of 1, 2 and 4 MiB at 1×2, 2×2 and 4×2 respectively. That is an exact storage count, separate from process peak memory. The outer column pass joins adjacent four-lane root vectors. The terminal butterflies reuse the original two- and one-row SIMD kernels. Two derivative passes reuse the allocation and retain the original B rows to finish the product.

The first compile caught a mismatch between the four-lane wrapper and the native lane type. The corrected implementation joins the roots through the existing wrapper. Native checks compare all product rows with derivatives independently evaluated from polynomial terms on eight domain sizes through 262,144 and two root choices. They cover empty input and a partial B prefix. Separate route probes confirm the configured workspace and every proof export verifies independently. All 180 exports passed.

On 1×2, the ordinary quotient took 0.535 / 0.542 ms and the single-workspace route took 0.688 / 0.705 ms. The paired full-proof changes against ordinary were +2.22% / +3.47%. Maximum process RSS changed 57.8 → 57.0 / 58.2 → 56.2 MiB. The candidate fell short of the paired latency requirement.
On 2×2, the ordinary quotient took 1.222 / 1.207 ms and the single-workspace route took 1.180 / 1.196 ms. The paired full-proof changes against ordinary were -0.55% / -0.06%. Maximum process RSS changed 98.0 → 96.8 / 97.5 → 96.5 MiB. The candidate fell short of the paired latency requirement.
On 4×2, the ordinary quotient took 2.258 / 2.223 ms and the single-workspace route took 2.498 / 2.595 ms. The paired full-proof changes against ordinary were +3.29% / +1.95%. Maximum process RSS changed 224.8 → 212.2 / 218.5 → 231.0 MiB. The candidate fell short of the paired latency requirement.

The proof panel runs on 24 main and eight frontend Ryzen logical CPUs. The 2×2 Zopro controls use the shift profile from the [previous Ryzen experiment](../2026-11-20-euler-ordinary-layout-ryzen908/README.md). The 1×2 and 4×2 controls use their previously recorded FFT settings. Eight paired blocks per shape and input prime three proofs and time nineteen. Advancement requires a lower paired median and at least six wins against both the frozen release and the same-build ordinary mode on both inputs. Peak RSS covers the entire process. The separate clocks measure complete Euler quotient calls in two processes with seven timed requests each. The main latency runs have no observer clocks or tracing.

| Shape | Input | Mode | Full proof ms | Paired vs release | Paired vs ordinary | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1x2 | catalog | release | 6.868 | +0.00% (0/8) | +0.09% (3/8) | 57.5 |
| 1x2 | catalog | ordinary | 6.869 | -0.09% (5/8) | +0.00% (0/8) | 57.8 |
| 1x2 | catalog | sequential | 7.017 | +2.10% (0/8) | +2.22% (0/8) | 57.0 |
| 1x2 | heldout | release | 6.793 | +0.00% (0/8) | +0.05% (3/8) | 57.2 |
| 1x2 | heldout | ordinary | 6.778 | -0.05% (5/8) | +0.00% (0/8) | 58.2 |
| 1x2 | heldout | sequential | 6.965 | +3.06% (0/8) | +3.47% (0/8) | 56.2 |
| 2x2 | catalog | release | 9.416 | +0.00% (0/8) | +0.37% (2/8) | 96.2 |
| 2x2 | catalog | ordinary | 9.398 | -0.37% (6/8) | +0.00% (0/8) | 98.0 |
| 2x2 | catalog | sequential | 9.346 | -0.63% (8/8) | -0.55% (7/8) | 96.8 |
| 2x2 | heldout | release | 9.394 | +0.00% (0/8) | -0.02% (4/8) | 94.8 |
| 2x2 | heldout | ordinary | 9.398 | +0.02% (4/8) | +0.00% (0/8) | 97.5 |
| 2x2 | heldout | sequential | 9.403 | -0.37% (5/8) | -0.06% (5/8) | 96.5 |
| 4x2 | catalog | release | 13.590 | +0.00% (0/8) | +0.29% (4/8) | 228.8 |
| 4x2 | catalog | ordinary | 13.616 | -0.28% (4/8) | +0.00% (0/8) | 224.8 |
| 4x2 | catalog | sequential | 14.096 | +3.52% (0/8) | +3.29% (0/8) | 212.2 |
| 4x2 | heldout | release | 13.607 | +0.00% (0/8) | +0.06% (3/8) | 220.8 |
| 4x2 | heldout | ordinary | 13.625 | -0.06% (5/8) | +0.00% (0/8) | 218.5 |
| 4x2 | heldout | sequential | 13.919 | +1.81% (0/8) | +1.95% (0/8) | 231.0 |

| Shape | Input | Mode | Quotient FFT ms |
|---|---|---|---:|
| 1x2 | catalog | ordinary | 0.535 |
| 1x2 | catalog | sequential | 0.688 |
| 1x2 | heldout | ordinary | 0.542 |
| 1x2 | heldout | sequential | 0.705 |
| 2x2 | catalog | ordinary | 1.222 |
| 2x2 | catalog | sequential | 1.180 |
| 2x2 | heldout | ordinary | 1.207 |
| 2x2 | heldout | sequential | 1.196 |
| 4x2 | catalog | ordinary | 2.258 |
| 4x2 | catalog | sequential | 2.498 |
| 4x2 | heldout | ordinary | 2.223 |
| 4x2 | heldout | sequential | 2.595 |

[Source](artifacts/source.patch) · [Samples and traces](artifacts/samples.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

## Eight supplied M5 references

The [recorded C7a scorecard](../2026-11-20-typed-small-proof-c7a898/README.md) still reaches 10× M5 CPU on **5/8** original requests. AWS authentication was expired before this experiment.

| Shape | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup |
|---|---:|---:|---:|
| 1×2 | 7.858 | 37 / 24 | 4.71× / 3.05× |
| 2×2 | 12.029 | 59 / 30 | 4.90× / 2.49× |
| 4×2 | 14.726 | 109 / 48 | 7.40× / 3.26× |
| 8×2 | 18.849 | 189 / 74 | 10.03× / 3.93× |
| 12×2 | 27.790 | 284 / 101 | 10.22× / 3.63× |
| 16×2 | 30.884 | 335 / 124 | 10.85× / 4.01× |
| 24×2 | 47.046 | 528 / 176 | 11.22× / 3.74× |
| merge-24×1 | 45.724 | 507 / 175 | 11.09× / 3.83× |
