# Paired width-three Poseidon partial rounds on Ryzen

A width-three partial round computes one S-box, then feeds its output into the next head and two tail coordinates. The two-round variant projects both heads from the original tail. It folds the effect of the first round's tail update into the second projection using a fixed public coefficient. The two tail coordinates then receive both round contributions through fused products. The candidate uses 28 public coefficients, or 896 bytes. It adds no proof-dependent cache, key copy or solver plan.

The `fused` arm reduces the second head product and the feedback term together. The `early` arm computes the feedback product before the second S-box completes. Per pair, both add one elementary product and reduce the linear Montgomery reductions from eight to six and seven, respectively. This accounting excludes the unchanged S-box arithmetic. Every call emits the original rows and wire values. The final odd partial round and the other Poseidon widths use the existing executor.

Both modes passed five native tests, including exact full-buffer comparison with the interpreter, alternate multiplication orientations, optional C rows and the generated matrix coefficients. The first fused test caught an emission-index error. The corrected candidate starts at S-box 9 after the three initial full rounds. Route probes observed 73, 139 and 269 width-three spans per proof on 1×2, 2×2 and 4×2 respectively. The zero-optional-workspace probes verified through the ordinary fallback. All 242 full-proof exports independently verified.

On 1×2, `early` had the lowest candidate median on the original input. The ordinary solve took 0.700 / 0.698 ms and this candidate took 0.695 / 0.700 ms. The paired full-proof changes against ordinary were +0.24% / -0.02%. No profile met all matched comparisons.
On 2×2, `fused` had the lowest candidate median on the original input. The ordinary solve took 0.887 / 0.882 ms and this candidate took 0.865 / 0.868 ms. The paired full-proof changes against ordinary were +0.04% / +0.02%. No profile met all matched comparisons.
On 4×2, `fused` had the lowest candidate median on the original input. The ordinary solve took 1.464 / 1.486 ms and this candidate took 1.483 / 1.481 ms. The paired full-proof changes against ordinary were +0.02% / +0.49%. No profile met all matched comparisons.

The proof panel runs on 24 main and eight frontend Ryzen logical CPUs. Eight paired blocks per shape and input prime three proofs and time nineteen. Advancement requires a lower paired median and at least six wins against both the frozen release and the same-build ordinary mode on both inputs. Peak RSS covers the entire process. The clocks in the second panel time the prepared solver across two processes with seven timed calls each. Timing processes have no per-span tracing.

| Shape | Input | Mode | Full proof ms | Paired vs release | Paired vs ordinary | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1x2 | catalog | release | 6.888 | +0.00% (0/8) | +0.05% (4/8) | 58.2 |
| 1x2 | catalog | ordinary | 6.885 | -0.05% (4/8) | +0.00% (0/8) | 58.8 |
| 1x2 | catalog | fused | 6.945 | +0.44% (3/8) | +0.74% (3/8) | 58.2 |
| 1x2 | catalog | early | 6.922 | +0.29% (3/8) | +0.24% (3/8) | 58.0 |
| 1x2 | heldout | release | 6.740 | +0.00% (0/8) | -0.55% (5/8) | 58.0 |
| 1x2 | heldout | ordinary | 6.797 | +0.55% (3/8) | +0.00% (0/8) | 58.5 |
| 1x2 | heldout | fused | 6.794 | +0.64% (3/8) | -0.15% (5/8) | 59.0 |
| 1x2 | heldout | early | 6.773 | +0.67% (2/8) | -0.02% (4/8) | 59.0 |
| 2x2 | catalog | release | 9.492 | +0.00% (0/8) | +0.04% (3/8) | 95.5 |
| 2x2 | catalog | ordinary | 9.487 | -0.04% (5/8) | +0.00% (0/8) | 95.5 |
| 2x2 | catalog | fused | 9.461 | -0.11% (5/8) | +0.04% (4/8) | 97.2 |
| 2x2 | catalog | early | 9.530 | +0.51% (2/8) | +0.35% (2/8) | 98.0 |
| 2x2 | heldout | release | 9.521 | +0.00% (0/8) | +0.60% (3/8) | 94.2 |
| 2x2 | heldout | ordinary | 9.454 | -0.60% (5/8) | +0.00% (0/8) | 95.0 |
| 2x2 | heldout | fused | 9.469 | -0.53% (6/8) | +0.02% (4/8) | 95.2 |
| 2x2 | heldout | early | 9.504 | -0.30% (4/8) | +0.46% (2/8) | 95.8 |
| 4x2 | catalog | release | 13.612 | +0.00% (0/8) | -0.89% (6/8) | 215.5 |
| 4x2 | catalog | ordinary | 13.743 | +0.90% (2/8) | +0.00% (0/8) | 226.8 |
| 4x2 | catalog | fused | 13.666 | +0.48% (3/8) | +0.02% (4/8) | 219.2 |
| 4x2 | catalog | early | 13.719 | +1.05% (3/8) | +0.24% (3/8) | 216.8 |
| 4x2 | heldout | release | 13.678 | +0.00% (0/8) | -0.53% (6/8) | 209.5 |
| 4x2 | heldout | ordinary | 13.729 | +0.54% (2/8) | +0.00% (0/8) | 214.5 |
| 4x2 | heldout | fused | 13.782 | +0.74% (0/8) | +0.49% (3/8) | 223.2 |
| 4x2 | heldout | early | 13.730 | +0.55% (1/8) | -0.03% (4/8) | 219.2 |

| Shape | Input | Mode | Solver ms |
|---|---|---|---:|
| 1x2 | catalog | ordinary | 0.700 |
| 1x2 | catalog | fused | 0.685 |
| 1x2 | catalog | early | 0.695 |
| 1x2 | heldout | ordinary | 0.698 |
| 1x2 | heldout | fused | 0.685 |
| 1x2 | heldout | early | 0.700 |
| 2x2 | catalog | ordinary | 0.887 |
| 2x2 | catalog | fused | 0.865 |
| 2x2 | catalog | early | 0.899 |
| 2x2 | heldout | ordinary | 0.882 |
| 2x2 | heldout | fused | 0.868 |
| 2x2 | heldout | early | 0.910 |
| 4x2 | catalog | ordinary | 1.464 |
| 4x2 | catalog | fused | 1.483 |
| 4x2 | catalog | early | 1.506 |
| 4x2 | heldout | ordinary | 1.486 |
| 4x2 | heldout | fused | 1.481 |
| 4x2 | heldout | early | 1.500 |

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
