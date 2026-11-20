# Sorting compact fingerprints for B2 scalar grouping on Ryzen

This experiment replaces the serial dictionary pass for eligible B2 scalars with an in-place parallel sort of eight-byte fingerprint and position records. It compares all four limbs inside each matching run. The sorted pass declines after eight distinct values with the same fingerprint, and the ordinary dictionary handles the complete request. Reusing the record array to recover class labels preserves first-occurrence class order, per-class point order and exclusion of zeros. The extra proof-local array is separately reserved, with no retained point or scalar table.

The native test checks exact agreement with ordinary grouping through 52,274 scalars, zero values, deliberate collisions and fallback on a large collision run. All 144 SDK proof exports independently verified. The route probes check optional-memory denial and report whether the candidate reached sorting.

The Ryzen run uses 24 main and eight frontend logical CPUs. The 2×2 arms use medium fused K/H when admitted and the 4×2 arms keep their ordinary grouped B2 curve geometry. Seven matched blocks per input prime three proofs and time thirteen. An eligible profile must reach the candidate and record a lower paired median with five wins against both controls on both inputs. Stage clocks come from separate probes and include sampling, allocation, group/index assembly and scalar conversion where the proof uses it.

The fingerprint sort did not consistently improve both controls on both inputs of a routed shape. The SDK default is unchanged.

| Shape | Input | Mode | Proof ms | Paired vs ordinary | Paired vs release | CPU ms / proof | Minor faults / proof | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | release | 6.871 | -0.55% (5/7) | +0.00% (0/7) | 134.0 | 111 | 57.8 |
| 1×2 | catalog | ordinary | 6.916 | +0.00% (0/7) | +0.55% (2/7) | 135.0 | 107 | 58.0 |
| 1×2 | catalog | sort | 6.895 | -0.23% (4/7) | +0.19% (2/7) | 133.3 | 126 | 57.2 |
| 1×2 | heldout | release | 6.764 | -0.02% (4/7) | +0.00% (0/7) | 128.6 | 127 | 57.5 |
| 1×2 | heldout | ordinary | 6.751 | +0.00% (0/7) | +0.02% (3/7) | 128.6 | 117 | 57.5 |
| 1×2 | heldout | sort | 6.789 | +0.42% (1/7) | +0.65% (1/7) | 128.7 | 139 | 57.5 |
| 2×2 | catalog | release | 9.489 | -1.64% (5/7) | +0.00% (0/7) | 181.2 | 291 | 94.0 |
| 2×2 | catalog | ordinary | 9.614 | +0.00% (0/7) | +1.67% (2/7) | 181.4 | 216 | 99.5 |
| 2×2 | catalog | sort | 9.515 | -1.96% (5/7) | +0.11% (3/7) | 182.1 | 266 | 98.0 |
| 2×2 | heldout | release | 9.515 | +0.12% (3/7) | +0.00% (0/7) | 181.8 | 332 | 93.8 |
| 2×2 | heldout | ordinary | 9.504 | +0.00% (0/7) | -0.11% (4/7) | 181.8 | 274 | 97.8 |
| 2×2 | heldout | sort | 9.500 | +0.30% (3/7) | +0.18% (3/7) | 182.3 | 259 | 96.8 |
| 4×2 | catalog | release | 13.697 | -0.81% (5/7) | +0.00% (0/7) | 292.0 | 887 | 215.2 |
| 4×2 | catalog | ordinary | 13.754 | +0.00% (0/7) | +0.81% (2/7) | 293.0 | 889 | 202.8 |
| 4×2 | catalog | sort | 14.474 | +5.36% (0/7) | +5.55% (0/7) | 296.3 | 870 | 202.8 |
| 4×2 | heldout | release | 13.717 | -1.22% (5/7) | +0.00% (0/7) | 293.0 | 1014 | 201.2 |
| 4×2 | heldout | ordinary | 13.882 | +0.00% (0/7) | +1.23% (2/7) | 292.2 | 895 | 203.0 |
| 4×2 | heldout | sort | 14.343 | +4.73% (1/7) | +4.82% (0/7) | 297.2 | 899 | 204.8 |

| Shape | Input | B2 scalars | Resulting groups | Sort calls | Ordinary grouping ms | Sort-requested grouping ms |
|---|---|---:|---:|---:|---:|---:|
| 1×2 | catalog | — | — | 0 | — | — |
| 1×2 | heldout | — | — | 0 | — | — |
| 2×2 | catalog | 52,274 | 27,218 | 4/4 | 1.064 | 1.606 |
| 2×2 | heldout | 52,274 | 27,267 | 4/4 | 1.055 | 1.752 |
| 4×2 | catalog | 95,537 | 29,738 | 4/4 | 3.518 | 6.476 |
| 4×2 | heldout | 95,537 | 29,961 | 4/4 | 2.939 | 6.381 |

[Source](artifacts/source.patch) · [Summary](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)
