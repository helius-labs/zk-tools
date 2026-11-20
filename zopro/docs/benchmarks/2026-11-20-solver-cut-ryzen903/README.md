# Solver band boundaries selected from dependencies on Ryzen

The [packed solver](../2026-11-19-packed-claims-release393/README.md) places outer bands by blending cumulative work and dependency depth. This experiment chooses boundaries from the pinned image dependency graph. It estimates a band's span by its longest component, with the existing task packing and local refinement. The four-band choice minimizes total span over 32 evenly spaced depth intervals and the baseline cuts. The shorter choice adds a fixed cost of 128 operation units per band and selects a grid partition. The `short-default` control uses the same number of bands with ordinary depth placement. The model treats every operation equally and ignores finite-worker scheduling and Poseidon specialization.

The SDK takes the chosen cuts as depth indices. Dependency checks, input order within tasks, inner refinement, Poseidon recognition and optional preparation admission use the existing implementation. The cut list has at most three indices. The malformed-cut probe selects the ordinary schedule. The zero-optional-workspace probe verifies the existing fallback. Native solver tests pass, including the new input validation check.

1×2 had its lowest candidate median with `cut-four`. It took 6.897 and 6.793 ms, with paired changes of -0.53% and -1.24% against same-build ordinary. Every candidate fell short of at least one matched requirement.

2×2 had its lowest candidate median with `short-selected`. It took 9.623 and 9.624 ms, with paired changes of +0.33% and +0.39% against same-build ordinary. Every candidate fell short of at least one matched requirement.

4×2 had its lowest candidate median with `short-selected`. It took 13.645 and 13.616 ms, with paired changes of +0.14% and +0.21% against same-build ordinary. Every candidate fell short of at least one matched requirement.

Ryzen uses 24 main and eight frontend logical CPUs. Each timing process primes three proofs and times 17. Eight matched blocks cover each input and shape. Advancement requires lower paired medians and six wins against release and same-build ordinary on both inputs. The explicit shorter schedule also needs to beat the default placement at the same band count. All 308 exports independently verified.

| Shape | Input | Profile | Full proof ms | Paired vs release | Paired vs ordinary | Paired vs short default | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|
| 1x2 | catalog | release | 6.869 | +0.00% (0/8) | -1.15% (7/8) | -2.88% (8/8) | 58.0 |
| 1x2 | catalog | ordinary | 6.939 | +1.16% (1/8) | +0.00% (0/8) | -1.79% (8/8) | 57.5 |
| 1x2 | catalog | cut-four | 6.897 | +0.44% (3/8) | -0.53% (7/8) | -2.41% (8/8) | 57.8 |
| 1x2 | catalog | short-default | 7.070 | +2.96% (0/8) | +1.82% (0/8) | +0.00% (0/8) | 58.5 |
| 1x2 | catalog | short-selected | 6.908 | +0.54% (1/8) | -0.47% (6/8) | -2.25% (8/8) | 58.2 |
| 1x2 | heldout | release | 6.788 | +0.00% (0/8) | -1.10% (8/8) | -2.35% (8/8) | 58.8 |
| 1x2 | heldout | ordinary | 6.849 | +1.12% (0/8) | +0.00% (0/8) | -1.31% (8/8) | 58.0 |
| 1x2 | heldout | cut-four | 6.793 | +0.11% (3/8) | -1.24% (7/8) | -2.61% (8/8) | 58.5 |
| 1x2 | heldout | short-default | 6.947 | +2.40% (0/8) | +1.33% (0/8) | +0.00% (0/8) | 58.2 |
| 1x2 | heldout | short-selected | 6.788 | -0.43% (5/8) | -1.20% (8/8) | -2.29% (8/8) | 58.8 |
| 2x2 | catalog | release | 9.488 | +0.00% (0/8) | -1.30% (6/8) | -5.51% (8/8) | 99.2 |
| 2x2 | catalog | ordinary | 9.596 | +1.32% (2/8) | +0.00% (0/8) | -4.18% (7/8) | 99.5 |
| 2x2 | catalog | cut-four | 9.675 | +2.01% (0/8) | +0.96% (1/8) | -3.75% (8/8) | 94.8 |
| 2x2 | catalog | short-default | 10.053 | +5.83% (0/8) | +4.36% (1/8) | +0.00% (0/8) | 94.5 |
| 2x2 | catalog | short-selected | 9.623 | +1.52% (0/8) | +0.33% (2/8) | -3.98% (8/8) | 97.5 |
| 2x2 | heldout | release | 9.516 | +0.00% (0/8) | -0.83% (7/8) | -5.13% (8/8) | 94.5 |
| 2x2 | heldout | ordinary | 9.607 | +0.84% (1/8) | +0.00% (0/8) | -4.24% (8/8) | 98.8 |
| 2x2 | heldout | cut-four | 9.667 | +1.62% (0/8) | +0.74% (1/8) | -3.56% (8/8) | 94.2 |
| 2x2 | heldout | short-default | 10.022 | +5.41% (0/8) | +4.43% (0/8) | +0.00% (0/8) | 99.2 |
| 2x2 | heldout | short-selected | 9.624 | +1.35% (0/8) | +0.39% (2/8) | -3.98% (8/8) | 93.8 |
| 4x2 | catalog | release | 13.613 | +0.00% (0/8) | -0.30% (6/8) | -8.01% (8/8) | 223.0 |
| 4x2 | catalog | ordinary | 13.660 | +0.30% (2/8) | +0.00% (0/8) | -7.81% (8/8) | 214.8 |
| 4x2 | catalog | cut-four | 13.809 | +1.22% (0/8) | +1.15% (1/8) | -6.88% (8/8) | 210.0 |
| 4x2 | catalog | short-default | 14.803 | +8.70% (0/8) | +8.47% (0/8) | +0.00% (0/8) | 215.2 |
| 4x2 | catalog | short-selected | 13.645 | +0.34% (3/8) | +0.14% (4/8) | -7.73% (8/8) | 215.2 |
| 4x2 | heldout | release | 13.588 | +0.00% (0/8) | +0.14% (3/8) | -8.25% (8/8) | 212.8 |
| 4x2 | heldout | ordinary | 13.584 | -0.14% (5/8) | +0.00% (0/8) | -8.33% (8/8) | 210.2 |
| 4x2 | heldout | cut-four | 13.785 | +1.21% (2/8) | +1.01% (0/8) | -7.11% (8/8) | 216.5 |
| 4x2 | heldout | short-default | 14.814 | +8.99% (0/8) | +9.09% (0/8) | +0.00% (0/8) | 212.5 |
| 4x2 | heldout | short-selected | 13.616 | +0.40% (2/8) | +0.21% (3/8) | -7.95% (8/8) | 222.0 |

Separate clocked probes run the real prepared solver and record operation span before tagging, the number of operations in recognized Poseidon spans and solver execution time. The clock starts after preparation and leaves curve and verifier costs outside its measurement. Two processes of seven timed calls provide each table row. The first reader reordered dependency reads, which changed the root order and overcounted some packed leaves by up to two per band. The corrected reader preserves read order. It exactly matches the SDK's per-band operation totals, spans and leaf counts. The selected cuts, modeled spans and proof captures were identical under this reporting correction.

| Shape | Input | Profile | Bands | Unit-cost span | Tagged operations | Solver ms |
|---|---|---|---:|---:|---:|---:|
| 1x2 | catalog | ordinary | 4 | 11,419 | 19,866 | 0.697 |
| 1x2 | catalog | cut-four | 4 | 10,621 | 17,631 | 0.672 |
| 1x2 | catalog | short-default | 3 | 11,438 | 17,406 | 0.803 |
| 1x2 | catalog | short-selected | 3 | 10,621 | 18,099 | 0.651 |
| 1x2 | heldout | ordinary | 4 | 11,419 | 19,866 | 0.701 |
| 1x2 | heldout | cut-four | 4 | 10,621 | 17,631 | 0.670 |
| 1x2 | heldout | short-default | 3 | 11,438 | 17,406 | 0.807 |
| 1x2 | heldout | short-selected | 3 | 10,621 | 18,099 | 0.648 |
| 2x2 | catalog | ordinary | 4 | 11,936 | 35,628 | 0.917 |
| 2x2 | catalog | cut-four | 4 | 11,780 | 29,562 | 1.026 |
| 2x2 | catalog | short-default | 2 | 15,532 | 27,774 | 1.371 |
| 2x2 | catalog | short-selected | 2 | 11,780 | 31,668 | 0.980 |
| 2x2 | heldout | ordinary | 4 | 11,936 | 35,628 | 0.904 |
| 2x2 | heldout | cut-four | 4 | 11,780 | 29,562 | 1.018 |
| 2x2 | heldout | short-default | 2 | 15,532 | 27,774 | 1.385 |
| 2x2 | heldout | short-selected | 2 | 11,780 | 31,668 | 0.979 |
| 4x2 | catalog | ordinary | 4 | 11,750 | 65,193 | 1.470 |
| 4x2 | catalog | cut-four | 4 | 11,273 | 62,877 | 1.589 |
| 4x2 | catalog | short-default | 2 | 21,068 | 51,459 | 2.621 |
| 4x2 | catalog | short-selected | 2 | 11,273 | 60,069 | 1.469 |
| 4x2 | heldout | ordinary | 4 | 11,750 | 65,193 | 1.503 |
| 4x2 | heldout | cut-four | 4 | 11,273 | 62,877 | 1.636 |
| 4x2 | heldout | short-default | 2 | 21,068 | 51,459 | 2.601 |
| 4x2 | heldout | short-selected | 2 | 11,273 | 60,069 | 1.496 |

[Source](artifacts/source.patch) · [Dependency reader](artifacts/cuts.rs) · [Model](artifacts/model.json) · [SDK samples and route traces](artifacts/samples.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

## Eight supplied M5 references

The [recorded C7a scorecard](../2026-11-20-typed-small-proof-c7a898/README.md) still reaches 10× M5 CPU on **5/8** original requests.

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
