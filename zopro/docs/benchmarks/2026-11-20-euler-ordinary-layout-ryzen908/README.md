# Paired Euler FFT in the ordinary packed layout on Ryzen

The [paired four-point center](../2026-11-20-euler-paired-circulant-ryzen907/README.md) previously ran with the optional fused input and output columns enabled on every candidate. This follow-up keeps the recorded FFT layout on each shape. On 1×2 and 2×2, it retains the ordinary input packing, outer stored columns and pointwise Euler product. It combines the local inverse, Euler weights and forward passes inside the same expanded leaf. The `fused` arm uses the original center in that leaf. The `paired` arm packs distinct antisymmetric root products from two groups. The 4×2 profile already fuses its input and output columns. Its ordinary mode is the control for the paired center there.

The `shift` arm directly represents the negative power-of-two correction to the row sum, instead of scaling the sum before taking its negative. For normalized input a below 2p, the existing reciprocal estimates q with 0 ≤ 2^s a − qp < 2p. The new helper forms (q+2)p − 2^s a with limb shifts and the same reciprocal. The representative is at most 2p. Its two consumers add or subtract a value below 2p, then the final addition returns to the ordinary range. The branch uses the same workspace and holds no extra key tables.

Native checks compare the scaled lanes with scalar field arithmetic, cover zero and edge shifts, and compare the resulting center and transform with scalar DFTs and unfused transforms. Separate route probes confirm requested layouts and the fallback when local weighting is disabled under packed input columns. All 284 exported proofs independently verified.

On 1×2, `fused` had the lowest candidate median on the original input. The `fused` local transform took 0.248 / 0.249 ms and this candidate took 0.248 / 0.249 ms. The paired full-proof changes against ordinary were +0.13% / -0.53%. No profile met all matched comparisons.
On 2×2, `shift` had the lowest candidate median on the original input. The `fused` local transform took 0.575 / 0.581 ms and this candidate took 0.534 / 0.548 ms. The paired full-proof changes against ordinary were -0.98% / -1.13%. The profiles that met the comparisons were fused, shift.
On 4×2, `shift` had the lowest candidate median on the original input. The `ordinary` local transform took 0.831 / 0.831 ms and this candidate took 0.812 / 0.806 ms. The paired full-proof changes against ordinary were -0.24% / -0.26%. No profile met all matched comparisons.

The proof panel runs on 24 main and eight frontend Ryzen logical CPUs. All modes retain each profile’s input and output column settings. The release executable and the same-build ordinary control retain the original transform structure. Eight paired blocks per shape and input prime three proofs and time nineteen. Advancement requires a lower paired median and at least six wins against both the frozen release and the same-build ordinary mode on both inputs. Peak RSS covers the entire process. The clocks in the second panel time the local subtransforms between the outer inverse and forward passes across two processes with seven timed calls each. Timing processes have no clocks or route tracing.

| Shape | Input | Mode | Full proof ms | Paired vs release | Paired vs ordinary | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1x2 | catalog | release | 6.896 | +0.00% (0/8) | -0.00% (4/8) | 58.8 |
| 1x2 | catalog | ordinary | 6.959 | +0.00% (4/8) | +0.00% (0/8) | 58.2 |
| 1x2 | catalog | fused | 6.934 | +0.72% (3/8) | +0.13% (4/8) | 58.0 |
| 1x2 | catalog | paired | 6.962 | +0.51% (3/8) | +0.12% (4/8) | 58.8 |
| 1x2 | catalog | shift | 6.946 | +0.45% (3/8) | +0.09% (2/8) | 58.0 |
| 1x2 | heldout | release | 6.831 | +0.00% (0/8) | -0.36% (4/8) | 58.0 |
| 1x2 | heldout | ordinary | 6.866 | +0.36% (4/8) | +0.00% (0/8) | 58.5 |
| 1x2 | heldout | fused | 6.821 | -0.12% (4/8) | -0.53% (6/8) | 58.0 |
| 1x2 | heldout | paired | 6.790 | -0.48% (5/8) | -1.04% (7/8) | 58.2 |
| 1x2 | heldout | shift | 6.802 | -0.41% (5/8) | -0.68% (8/8) | 58.8 |
| 2x2 | catalog | release | 9.495 | +0.00% (0/8) | -0.24% (6/8) | 95.2 |
| 2x2 | catalog | ordinary | 9.531 | +0.24% (2/8) | +0.00% (0/8) | 98.8 |
| 2x2 | catalog | fused | 9.455 | -0.36% (7/8) | -0.53% (7/8) | 99.2 |
| 2x2 | catalog | paired | 9.508 | -0.22% (6/8) | -0.17% (5/8) | 94.0 |
| 2x2 | catalog | shift | 9.422 | -0.76% (6/8) | -0.98% (6/8) | 96.2 |
| 2x2 | heldout | release | 9.504 | +0.00% (0/8) | -0.37% (6/8) | 98.8 |
| 2x2 | heldout | ordinary | 9.557 | +0.37% (2/8) | +0.00% (0/8) | 94.8 |
| 2x2 | heldout | fused | 9.489 | -0.50% (6/8) | -0.97% (7/8) | 97.8 |
| 2x2 | heldout | paired | 9.459 | -0.74% (7/8) | -1.31% (8/8) | 95.0 |
| 2x2 | heldout | shift | 9.427 | -0.98% (6/8) | -1.13% (8/8) | 96.0 |
| 4x2 | catalog | release | 13.624 | +0.00% (0/8) | -0.35% (5/8) | 216.0 |
| 4x2 | catalog | ordinary | 13.660 | +0.35% (3/8) | +0.00% (0/8) | 223.0 |
| 4x2 | catalog | paired | 13.638 | -0.06% (5/8) | -0.04% (4/8) | 212.5 |
| 4x2 | catalog | shift | 13.638 | +0.10% (3/8) | -0.24% (5/8) | 220.8 |
| 4x2 | heldout | release | 13.605 | +0.00% (0/8) | -0.21% (5/8) | 214.8 |
| 4x2 | heldout | ordinary | 13.668 | +0.21% (3/8) | +0.00% (0/8) | 217.8 |
| 4x2 | heldout | paired | 13.666 | +0.04% (4/8) | -0.16% (4/8) | 223.8 |
| 4x2 | heldout | shift | 13.650 | +0.33% (3/8) | -0.26% (5/8) | 220.0 |

| Shape | Input | Mode | Local FFT ms |
|---|---|---|---:|
| 1x2 | catalog | fused | 0.248 |
| 1x2 | catalog | paired | 0.238 |
| 1x2 | catalog | shift | 0.233 |
| 1x2 | heldout | fused | 0.249 |
| 1x2 | heldout | paired | 0.242 |
| 1x2 | heldout | shift | 0.232 |
| 2x2 | catalog | fused | 0.575 |
| 2x2 | catalog | paired | 0.547 |
| 2x2 | catalog | shift | 0.534 |
| 2x2 | heldout | fused | 0.581 |
| 2x2 | heldout | paired | 0.559 |
| 2x2 | heldout | shift | 0.548 |
| 4x2 | catalog | ordinary | 0.831 |
| 4x2 | catalog | paired | 0.839 |
| 4x2 | catalog | shift | 0.812 |
| 4x2 | heldout | ordinary | 0.831 |
| 4x2 | heldout | paired | 0.811 |
| 4x2 | heldout | shift | 0.806 |

| Shape | Input | Same-layout center control | Paired vs center control | Shift vs paired |
|---|---|---|---:|---:|
| 1x2 | catalog | fused | -0.28% (5/8) | +0.18% (3/8) |
| 1x2 | heldout | fused | -0.13% (4/8) | +0.30% (3/8) |
| 2x2 | catalog | fused | +0.38% (3/8) | -0.63% (6/8) |
| 2x2 | heldout | fused | -0.60% (5/8) | -0.47% (6/8) |
| 4x2 | catalog | ordinary | -0.04% (4/8) | -0.08% (4/8) |
| 4x2 | heldout | ordinary | -0.16% (4/8) | +0.48% (3/8) |

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
