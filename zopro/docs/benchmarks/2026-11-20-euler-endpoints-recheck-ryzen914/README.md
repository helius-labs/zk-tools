# Fresh calibration of the Euler FFT endpoint variants

This run rechecks the [fused input and output variants](../2026-11-20-euler-fused-ends-ryzen912/README.md) after the large difference between their two bridge controls [failed to reproduce](../2026-11-20-euler-layout-diagnosis-ryzen913/README.md). It uses the exact original variant binary and frozen bridge, with the exact release references from the previous single-workspace experiment. The release reference for 2×2 uses the qualified shift variant. No kernel or input changed in this retest.

Twelve counterbalanced blocks on each input use fresh unprivileged processes on the same 24 main and eight frontend Ryzen logical CPUs. Every process primes three proofs and times 19. The protocol was written before capture. Each input calibrates when the paired median absolute change is at most 2% for both same-configuration binary comparisons. A variant then needs a lower median paired latency and nine wins over each of the release reference, same-build ordinary and frozen bridge on both inputs. RSS includes key loading. All 432 exported proofs independently verified.

On 1×2, `ends` took 6.927 / 6.792 ms on catalog and held-out. Its paired changes against the release reference were -0.37% / +0.01%, with 7/12 / 6/12 wins. Both endpoint profiles fell short of the full comparison.
On 2×2, `ends` took 9.399 / 9.387 ms on catalog and held-out. Its paired changes against the release reference were -0.45% / -0.80%, with 9/12 / 11/12 wins. Both endpoint profiles fell short of the full comparison.
On 4×2, `ends` took 13.669 / 13.625 ms on catalog and held-out. Its paired changes against the release reference were -0.26% / -0.12%, with 7/12 / 9/12 wins. Both endpoint profiles fell short of the full comparison.

| Shape | Input | Same-build ordinary vs release | Same-build bridge vs frozen bridge | Calibration |
|---|---|---:|---:|---|
| 1x2 | catalog | +0.55% | +0.97% | Pass |
| 1x2 | heldout | +0.31% | -0.09% | Pass |
| 2x2 | catalog | -0.20% | +0.17% | Pass |
| 2x2 | heldout | +0.00% | +0.20% | Pass |
| 4x2 | catalog | +0.31% | -0.09% | Pass |
| 4x2 | heldout | -0.03% | +0.45% | Pass |

| Shape | Input | Profile | Proof ms | Peak MiB | Paired vs release reference | Paired vs ordinary | Paired vs frozen bridge |
|---|---|---|---:|---:|---:|---:|---:|
| 1x2 | catalog | reference | 6.902 | 58.8 | +0.00% (0/12) | -0.54% (8/12) | +0.37% (4/12) |
| 1x2 | catalog | ordinary | 6.976 | 58.0 | +0.55% (4/12) | +0.00% (0/12) | +0.85% (2/12) |
| 1x2 | catalog | prior | 6.894 | 57.8 | -0.37% (8/12) | -0.84% (10/12) | +0.00% (0/12) |
| 1x2 | catalog | bridge | 6.954 | 57.5 | +0.47% (4/12) | +0.15% (5/12) | +0.97% (2/12) |
| 1x2 | catalog | emit | 6.951 | 57.0 | +0.56% (3/12) | +0.05% (6/12) | +0.88% (1/12) |
| 1x2 | catalog | ends | 6.927 | 58.2 | -0.37% (7/12) | -1.06% (9/12) | +0.62% (4/12) |
| 1x2 | heldout | reference | 6.818 | 58.5 | +0.00% (0/12) | -0.31% (9/12) | +0.52% (5/12) |
| 1x2 | heldout | ordinary | 6.839 | 58.2 | +0.31% (3/12) | +0.00% (0/12) | +0.86% (0/12) |
| 1x2 | heldout | prior | 6.788 | 57.8 | -0.52% (7/12) | -0.85% (12/12) | +0.00% (0/12) |
| 1x2 | heldout | bridge | 6.777 | 57.8 | -0.56% (8/12) | -1.05% (10/12) | -0.09% (9/12) |
| 1x2 | heldout | emit | 6.790 | 57.2 | -0.26% (7/12) | -0.78% (10/12) | +0.02% (6/12) |
| 1x2 | heldout | ends | 6.792 | 57.0 | +0.01% (6/12) | -0.68% (9/12) | +0.23% (4/12) |
| 2x2 | catalog | reference | 9.431 | 98.8 | +0.00% (0/12) | +0.20% (5/12) | +0.52% (4/12) |
| 2x2 | catalog | ordinary | 9.426 | 99.5 | -0.20% (7/12) | +0.00% (0/12) | +0.48% (3/12) |
| 2x2 | catalog | prior | 9.379 | 92.0 | -0.52% (8/12) | -0.48% (9/12) | +0.00% (0/12) |
| 2x2 | catalog | bridge | 9.394 | 93.8 | -0.40% (8/12) | -0.32% (8/12) | +0.17% (5/12) |
| 2x2 | catalog | emit | 9.403 | 91.8 | -0.29% (9/12) | -0.31% (7/12) | +0.09% (6/12) |
| 2x2 | catalog | ends | 9.399 | 98.2 | -0.45% (9/12) | -0.21% (7/12) | +0.47% (4/12) |
| 2x2 | heldout | reference | 9.443 | 97.8 | +0.00% (0/12) | -0.00% (6/12) | +0.56% (2/12) |
| 2x2 | heldout | ordinary | 9.428 | 99.2 | +0.00% (6/12) | +0.00% (0/12) | +0.71% (5/12) |
| 2x2 | heldout | prior | 9.425 | 91.8 | -0.56% (10/12) | -0.71% (7/12) | +0.00% (0/12) |
| 2x2 | heldout | bridge | 9.421 | 92.0 | -0.34% (8/12) | -0.01% (6/12) | +0.20% (4/12) |
| 2x2 | heldout | emit | 9.369 | 92.2 | -0.79% (12/12) | -0.72% (12/12) | -0.31% (10/12) |
| 2x2 | heldout | ends | 9.387 | 91.8 | -0.80% (11/12) | -0.67% (9/12) | -0.39% (7/12) |
| 4x2 | catalog | reference | 13.736 | 216.2 | +0.00% (0/12) | -0.31% (7/12) | +0.13% (5/12) |
| 4x2 | catalog | ordinary | 13.693 | 236.8 | +0.31% (5/12) | +0.00% (0/12) | +0.04% (6/12) |
| 4x2 | catalog | prior | 13.693 | 213.2 | -0.13% (7/12) | -0.04% (6/12) | +0.00% (0/12) |
| 4x2 | catalog | bridge | 13.720 | 216.0 | -0.10% (8/12) | +0.09% (4/12) | -0.09% (7/12) |
| 4x2 | catalog | emit | 13.710 | 215.2 | +0.12% (4/12) | -0.03% (6/12) | -0.05% (7/12) |
| 4x2 | catalog | ends | 13.669 | 205.8 | -0.26% (7/12) | -0.07% (6/12) | -0.08% (6/12) |
| 4x2 | heldout | reference | 13.697 | 210.5 | +0.00% (0/12) | +0.03% (6/12) | +0.13% (6/12) |
| 4x2 | heldout | ordinary | 13.696 | 222.2 | -0.03% (6/12) | +0.00% (0/12) | +0.08% (5/12) |
| 4x2 | heldout | prior | 13.693 | 218.8 | -0.13% (6/12) | -0.08% (7/12) | +0.00% (0/12) |
| 4x2 | heldout | bridge | 13.792 | 222.2 | +0.49% (2/12) | +0.64% (3/12) | +0.45% (2/12) |
| 4x2 | heldout | emit | 13.743 | 221.0 | -0.18% (7/12) | +0.22% (3/12) | +0.24% (4/12) |
| 4x2 | heldout | ends | 13.625 | 212.2 | -0.12% (9/12) | -0.31% (8/12) | -0.42% (8/12) |

The [recorded C7a comparison against all eight M5 CPU and Metal references](../2026-11-20-euler-fused-ends-ryzen912/README.md#eight-supplied-m5-references) remains at **5/8 at 10× M5 CPU**. AWS authentication was expired when checked before this run.

[Predeclared protocol](artifacts/protocol.json) · [Samples](artifacts/runs.json.gz) · [Pairwise results](artifacts/summary.json) · [Verification](artifacts/verification.json)
