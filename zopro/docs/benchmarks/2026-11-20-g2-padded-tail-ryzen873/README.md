# Padding G2 batch tails for SIMD on Ryzen

The variant executes residual borrowed G2 batches of four through seven pairs as one SIMD group. Dummy lanes contribute one to the prefix product and have no destination. The shared inverse also covers preceding full groups. Tails of one through three pairs use the usual scalar route. The change adds no point buffer or table.

The native generator test compares changing outputs across every tail length, no full group through 27 full groups, four sign patterns, and repeated scratch use. All 138 SDK proof exports independently verified.

The worker pools use 24 main and eight frontend Ryzen logical CPUs. The 1×2 and 2×2 trials use the ordinary exclusive G2 query. The 4×2 same-build control and trial enable grouped Phi to exercise this kernel. The 4×2 release uses its ordinary grouped B2 path. Seven paired blocks per input prime three proofs and time thirteen. Separate probes count padded groups. Advancement requires lower paired medians and five wins against each control on both inputs.

The padded tail did not consistently improve both controls across both inputs of any tested shape. The SDK default is unchanged.

| Shape | Input | Mode | Proof ms | Paired vs ordinary | Paired vs release | CPU ms / proof | Minor faults / proof | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | release | 6.877 | -0.23% (5/7) | +0.00% (0/7) | 133.0 | 132 | 57.8 |
| 1×2 | catalog | ordinary | 6.879 | +0.00% (0/7) | +0.24% (2/7) | 133.7 | 111 | 57.8 |
| 1×2 | catalog | padded | 6.875 | +0.33% (3/7) | -0.06% (5/7) | 134.0 | 125 | 58.5 |
| 1×2 | heldout | release | 6.748 | -0.55% (4/7) | +0.00% (0/7) | 128.9 | 129 | 57.5 |
| 1×2 | heldout | ordinary | 6.823 | +0.00% (0/7) | +0.55% (3/7) | 129.5 | 121 | 57.2 |
| 1×2 | heldout | padded | 6.802 | -0.48% (4/7) | +0.99% (2/7) | 128.7 | 125 | 58.0 |
| 2×2 | catalog | release | 9.486 | -0.33% (6/7) | +0.00% (0/7) | 181.7 | 247 | 95.0 |
| 2×2 | catalog | ordinary | 9.539 | +0.00% (0/7) | +0.33% (1/7) | 181.5 | 298 | 97.0 |
| 2×2 | catalog | padded | 9.526 | +0.03% (3/7) | +0.24% (2/7) | 181.3 | 295 | 99.2 |
| 2×2 | heldout | release | 9.483 | +0.07% (3/7) | +0.00% (0/7) | 181.7 | 186 | 94.2 |
| 2×2 | heldout | ordinary | 9.494 | +0.00% (0/7) | -0.07% (4/7) | 181.8 | 237 | 93.5 |
| 2×2 | heldout | padded | 9.487 | -0.14% (4/7) | +0.12% (3/7) | 181.8 | 260 | 94.2 |
| 4×2 | catalog | release | 13.693 | +2.18% (0/7) | +0.00% (0/7) | 293.4 | 929 | 204.8 |
| 4×2 | catalog | ordinary | 13.342 | +0.00% (0/7) | -2.13% (7/7) | 283.5 | 890 | 200.8 |
| 4×2 | catalog | padded | 13.369 | +0.46% (2/7) | -2.38% (6/7) | 283.5 | 991 | 216.5 |
| 4×2 | heldout | release | 13.661 | +3.30% (0/7) | +0.00% (0/7) | 292.3 | 1001 | 208.5 |
| 4×2 | heldout | ordinary | 13.239 | +0.00% (0/7) | -3.19% (7/7) | 283.2 | 847 | 200.0 |
| 4×2 | heldout | padded | 13.321 | +0.59% (3/7) | -1.32% (6/7) | 283.4 | 845 | 201.2 |

| Shape | Input | Padded groups per proof | Real pairs in padded groups per proof |
|---|---|---:|---:|
| 1×2 | catalog | 16.0 | 98.0 |
| 1×2 | heldout | 18.0 | 103.0 |
| 2×2 | catalog | 21.0 | 115.0 |
| 2×2 | heldout | 18.0 | 108.0 |
| 4×2 | catalog | 23.0 | 119.0 |
| 4×2 | heldout | 20.0 | 115.0 |

[Source](artifacts/source.patch) · [Summary](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)
