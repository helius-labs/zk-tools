# Pairing antisymmetric Euler FFT products on Ryzen

The four-point center of the Euler FFT applies the inverse and forward DFT around diagonal weights q + kN/4. For the four input rows x, the result at j is (4q + 2N)x[j] − (N/2)sum(x) + (N/2)i(x[j−1] − x[j+1]), where i is the inverse fourth root and indices wrap modulo four. The root products at j and j+2 differ only by sign. The paired implementation calculates the two distinct products for each of two adjacent groups in one eight-lane multiply. It packs both row sums into one small-integer multiply and uses one small-integer multiply for each diagonal. The ordinary center uses two general vector multiplies and two small-integer multiplies for the same groups. The direct arm evaluates the same circulant independently per group. It uses two general and four small-integer multiplies. All arms keep the packed workspace and existing root tables.

The direct and paired variants passed native checks against scalar four-point DFTs and the independent unfused transforms. The tests use both root orientations, 448 randomized pairs, zero lanes and domains through 2²⁶. Route probes confirm the three modes in the local packed FFT and verify fallback with local weighting disabled. All 248 exported proofs independently verified.

On 1×2, `paired` had the lowest candidate median on the original input. The ordinary local transform took 0.253 / 0.252 ms and this candidate took 0.244 / 0.240 ms. The paired full-proof changes against ordinary were -0.54% / -0.52%. No profile met all matched comparisons.
On 2×2, `paired` had the lowest candidate median on the original input. The ordinary local transform took 0.580 / 0.582 ms and this candidate took 0.562 / 0.560 ms. The paired full-proof changes against ordinary were -0.62% / -0.56%. No profile met all matched comparisons.
On 4×2, `paired` had the lowest candidate median on the original input. The ordinary local transform took 0.844 / 0.900 ms and this candidate took 0.820 / 0.799 ms. The paired full-proof changes against ordinary were +0.12% / -0.25%. No profile met all matched comparisons.

The proof panel runs on 24 main and eight frontend Ryzen logical CPUs. Same-build ordinary, direct and paired use packed FFT input columns, local weighting and fused output. The frozen controls retain the recorded per-shape settings, with local weighting disabled on 1×2 and 2×2 and enabled on 4×2. Eight paired blocks per shape and input prime three proofs and time nineteen. Advancement requires a lower paired median and at least six wins against both the frozen release and the same-build ordinary mode on both inputs. Peak RSS covers the entire process. The clocks in the second panel time the local subtransforms between the outer inverse and forward passes across two processes with seven timed calls each. Timing processes have no clocks or route tracing.

| Shape | Input | Mode | Full proof ms | Paired vs release | Paired vs ordinary | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1x2 | catalog | release | 6.879 | +0.00% (0/8) | -0.84% (7/8) | 58.8 |
| 1x2 | catalog | ordinary | 6.952 | +0.84% (1/8) | +0.00% (0/8) | 57.8 |
| 1x2 | catalog | direct | 6.966 | +1.13% (0/8) | -0.11% (4/8) | 58.2 |
| 1x2 | catalog | paired | 6.902 | +0.90% (1/8) | -0.54% (5/8) | 58.0 |
| 1x2 | heldout | release | 6.764 | +0.00% (0/8) | -0.93% (5/8) | 58.0 |
| 1x2 | heldout | ordinary | 6.820 | +0.94% (3/8) | +0.00% (0/8) | 58.2 |
| 1x2 | heldout | direct | 6.804 | +0.71% (3/8) | -0.18% (5/8) | 58.0 |
| 1x2 | heldout | paired | 6.790 | +0.84% (2/8) | -0.52% (6/8) | 58.2 |
| 2x2 | catalog | release | 9.446 | +0.00% (0/8) | +0.24% (2/8) | 96.0 |
| 2x2 | catalog | ordinary | 9.426 | -0.24% (6/8) | +0.00% (0/8) | 95.2 |
| 2x2 | catalog | direct | 9.426 | -0.35% (6/8) | +0.12% (3/8) | 95.8 |
| 2x2 | catalog | paired | 9.381 | -0.70% (7/8) | -0.62% (6/8) | 99.2 |
| 2x2 | heldout | release | 9.489 | +0.00% (0/8) | +0.43% (2/8) | 95.0 |
| 2x2 | heldout | ordinary | 9.440 | -0.42% (6/8) | +0.00% (0/8) | 96.2 |
| 2x2 | heldout | direct | 9.465 | -0.11% (6/8) | +0.42% (3/8) | 94.0 |
| 2x2 | heldout | paired | 9.392 | -0.92% (8/8) | -0.56% (5/8) | 98.0 |
| 4x2 | catalog | release | 13.666 | +0.00% (0/8) | +0.13% (3/8) | 217.8 |
| 4x2 | catalog | ordinary | 13.667 | -0.13% (5/8) | +0.00% (0/8) | 216.0 |
| 4x2 | catalog | direct | 13.712 | +0.33% (2/8) | +0.39% (1/8) | 229.0 |
| 4x2 | catalog | paired | 13.691 | +0.35% (3/8) | +0.12% (3/8) | 225.0 |
| 4x2 | heldout | release | 13.624 | +0.00% (0/8) | -0.70% (7/8) | 221.2 |
| 4x2 | heldout | ordinary | 13.709 | +0.70% (1/8) | +0.00% (0/8) | 218.0 |
| 4x2 | heldout | direct | 13.598 | -0.01% (4/8) | -0.48% (6/8) | 230.0 |
| 4x2 | heldout | paired | 13.637 | +0.24% (3/8) | -0.25% (5/8) | 226.2 |

| Shape | Input | Mode | Local FFT ms |
|---|---|---|---:|
| 1x2 | catalog | ordinary | 0.253 |
| 1x2 | catalog | direct | 0.259 |
| 1x2 | catalog | paired | 0.244 |
| 1x2 | heldout | ordinary | 0.252 |
| 1x2 | heldout | direct | 0.257 |
| 1x2 | heldout | paired | 0.240 |
| 2x2 | catalog | ordinary | 0.580 |
| 2x2 | catalog | direct | 0.590 |
| 2x2 | catalog | paired | 0.562 |
| 2x2 | heldout | ordinary | 0.582 |
| 2x2 | heldout | direct | 0.590 |
| 2x2 | heldout | paired | 0.560 |
| 4x2 | catalog | ordinary | 0.844 |
| 4x2 | catalog | direct | 0.846 |
| 4x2 | catalog | paired | 0.820 |
| 4x2 | heldout | ordinary | 0.900 |
| 4x2 | heldout | direct | 0.844 |
| 4x2 | heldout | paired | 0.799 |

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
