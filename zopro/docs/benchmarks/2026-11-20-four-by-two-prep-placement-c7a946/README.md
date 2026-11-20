# Independent solver and FFT placement on C7a 4×2

The following placements cleared both full-proof controls on both inputs. `solver8, solver16`. The lowest alternate median on the original input was **14.510 ms** with `solver16`. The same placement took **14.488 ms** on the held-out input. The original-input target is **10.9 ms**.

The previously measured profile runs the general witness interpreter on 32 frontend workers and computes the quotient on 64 main workers. The alternate interpreter pools use one or two whole eight-core L3 groups within the existing frontend mask. The FFT runs on the main pool or on the original 32-core frontend. Canonical gather and all curve placements retain the original schedule. K uses the original 32-core frontend pool.

The frozen control uses the exact seeded SDK. Twelve counterbalanced blocks per input prime three requests and time seventeen. The table reports medians of process means. A placement advances when the same-build control calibrates within 2% and the candidate has a lower paired median and at least nine wins against both controls on both inputs.

| Input | Placement | Proof ms | Paired vs same build | Paired vs frozen | Peak MiB |
|---|---|---:|---:|---:|---:|
| catalog | frozen | 14.903 | -0.17% (6/12) | +0.00% (0/12) | 217.6 |
| catalog | ordinary | 14.892 | +0.00% (0/12) | +0.18% (6/12) | 209.4 |
| catalog | front-fft | 14.922 | +0.18% (5/12) | +0.52% (5/12) | 203.4 |
| catalog | solver8 | 14.690 | -1.95% (11/12) | -1.69% (9/12) | 219.9 |
| catalog | solver16 | 14.510 | -1.86% (11/12) | -3.03% (11/12) | 225.2 |
| catalog | solver8-front | 14.829 | -0.69% (9/12) | -0.83% (8/12) | 220.0 |
| catalog | solver16-front | 14.770 | -0.45% (8/12) | -0.61% (7/12) | 219.2 |
| heldout | frozen | 14.875 | -0.12% (6/12) | +0.00% (0/12) | 217.7 |
| heldout | ordinary | 14.976 | +0.00% (0/12) | +0.12% (6/12) | 210.2 |
| heldout | front-fft | 15.171 | +1.49% (0/12) | +2.00% (1/12) | 201.9 |
| heldout | solver8 | 14.618 | -2.10% (11/12) | -1.98% (11/12) | 216.1 |
| heldout | solver16 | 14.488 | -2.64% (12/12) | -2.67% (12/12) | 214.8 |
| heldout | solver8-front | 14.991 | +0.23% (3/12) | +0.74% (4/12) | 217.7 |
| heldout | solver16-front | 14.915 | -0.56% (7/12) | -0.23% (7/12) | 205.7 |

## Separate stage clocks

Each placement uses three clocked blocks. The reported solve span includes buffer setup and the interpreter. Gather starts after the solve, and the quotient span includes pool dispatch where applicable. The clock deltas compare each clocked run with the corresponding unclocked block. The calibration band is ±3%.

| Input | Placement | Solve ms | Gather ms | Quotient ms | Clock effect | Within band |
|---|---|---:|---:|---:|---:|---|
| catalog | ordinary | 3.009 | 1.044 | 2.053 | +0.27% (1/3) | yes |
| catalog | front-fft | 2.992 | 1.071 | 2.255 | +1.04% (0/3) | yes |
| catalog | solver8 | 2.638 | 1.061 | 2.034 | -0.31% (2/3) | yes |
| catalog | solver16 | 2.527 | 1.036 | 2.069 | +1.39% (1/3) | yes |
| catalog | solver8-front | 2.718 | 1.015 | 2.189 | -0.19% (2/3) | yes |
| catalog | solver16-front | 2.655 | 1.017 | 2.152 | -1.97% (2/3) | yes |
| heldout | ordinary | 2.973 | 1.054 | 2.032 | +0.16% (1/3) | yes |
| heldout | front-fft | 2.992 | 1.061 | 2.169 | -0.87% (2/3) | yes |
| heldout | solver8 | 2.752 | 1.030 | 2.056 | +0.12% (1/3) | yes |
| heldout | solver16 | 2.662 | 1.016 | 2.072 | +2.42% (0/3) | yes |
| heldout | solver8-front | 2.800 | 1.043 | 2.194 | +0.46% (1/3) | yes |
| heldout | solver16-front | 2.639 | 1.008 | 2.222 | +1.08% (1/3) | yes |

The initial Ryzen preflight exposed a route mismatch. The solver and FFT hooks were moved to the general folded path before the C7a run. All four native tests passed. The corrected preflight verified twelve Ryzen exports and checked the active widths. The 204 C7a exports were verified on the instance and again after retrieval. The temporary instance and its bucket were removed.

## Eight supplied M5 references

Five of eight supplied shapes reach 10× against M5 CPU. The 4×2 row uses the qualified 16-worker interpreter candidate. The other measurements come from the [prior eight-shape report](../2026-11-20-packed-final-sdk-c7a944/README.md#eight-supplied-m5-references).

| Shape | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup |
|---|---:|---:|---:|
| 1×2 | 7.736 | 37 / 24 | 4.78× / 3.10× |
| 2×2 | 11.898 | 59 / 30 | 4.96× / 2.52× |
| 4×2 | 14.510 | 109 / 48 | 7.51× / 3.31× |
| 8×2 | 18.849 | 189 / 74 | 10.03× / 3.93× |
| 12×2 | 27.790 | 284 / 101 | 10.22× / 3.63× |
| 16×2 | 30.884 | 335 / 124 | 10.85× / 4.01× |
| 24×2 | 47.046 | 528 / 176 | 11.22× / 3.74× |
| merge-24×1 | 45.724 | 507 / 175 | 11.09× / 3.83× |

[Results](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Samples](artifacts/cloud-rows.json.gz) · [Experimental source](artifacts/source.patch.gz) · [Cleanup](artifacts/cleanup.json)
