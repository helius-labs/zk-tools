# Fusing input and output in the single-workspace Euler FFT on Ryzen

The [previous bridge](../2026-11-20-euler-derivative-bridge-ryzen911/README.md) keeps the derivative boundary inside a column tile. This experiment extends that approach to the edges of the transform. The `emit` arm finishes D(A) in a tile and adds D(A)·B directly to the output. It removes the last full packed-workspace write and the read of the active prefix. The `ends` arm also packs B directly into the first inverse columns. Those tasks replace a full compact write and read, and initialize the one existing workspace in place. Both modes retain the same bridge, roots, task sizing and N/8 main workspace packs.

Native tests compare the fused results to independently evaluated polynomials on eight domain sizes and two root orientations. Exact comparisons cover unequal nonzero row counts and short final packs with one, five and 24 workers. The SDK route probes confirm each enabled boundary and the expected task sizes. All 312 exported proofs independently verified.

On 1×2, the complete quotient took 0.502 / 0.501 ms with the previous bridge, 0.488 / 0.504 ms with fused output and 0.477 / 0.480 ms with both ends fused. The `ends` arm had the lowest catalog median among the candidates. Its paired proof changes were -0.92% / -0.20% against the same-build bridge and -0.24% / -0.58% against ordinary. Both modes fell short of the paired requirement.
On 2×2, the complete quotient took 1.114 / 1.160 ms with the previous bridge, 1.157 / 1.121 ms with fused output and 1.155 / 1.165 ms with both ends fused. The `emit` arm had the lowest catalog median among the candidates. Its paired proof changes were +0.02% / +0.43% against the same-build bridge and -0.66% / -0.58% against ordinary. Both modes fell short of the paired requirement.
On 4×2, the complete quotient took 2.253 / 2.267 ms with the previous bridge, 2.203 / 2.259 ms with fused output and 2.149 / 2.205 ms with both ends fused. The `ends` arm had the lowest catalog median among the candidates. Its paired proof changes were -0.20% / -0.50% against the same-build bridge and -0.47% / -0.49% against ordinary. Both modes fell short of the paired requirement.

The fresh bridge control was 13.28–13.43% slower than the frozen bridge on 1×2 and 7.96–8.74% slower on 2×2 despite identical run settings. The cross-build comparison therefore had a substantial control discrepancy. Separate plain and counter runs did not reproduce that gap. [Control diagnosis](../2026-11-20-euler-layout-diagnosis-ryzen913/README.md)

The proof panel runs on 24 main and eight frontend Ryzen logical CPUs. The ordinary arm uses the original quotient on 1×2 and 4×2 and the qualified shift quotient on 2×2. The frozen prior and same-build bridge arms use the earlier fused derivative boundary. Advancement needs lower paired medians and six wins against both ordinary and prior on both inputs. Eight paired blocks per shape and input prime three proofs and time nineteen. Peak RSS covers the entire process. The separate clocks measure complete Euler quotient calls in two processes with seven timed requests each. The main latency runs have no observer clocks or tracing.

| Shape | Input | Mode | Full proof ms | Paired vs prior | Paired vs ordinary | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1x2 | catalog | prior | 6.861 | +0.00% (0/8) | -11.15% (8/8) | 57.5 |
| 1x2 | catalog | ordinary | 7.729 | +12.55% (0/8) | +0.00% (0/8) | 58.2 |
| 1x2 | catalog | bridge | 7.783 | +13.43% (0/8) | +0.66% (3/8) | 57.5 |
| 1x2 | catalog | emit | 7.776 | +13.31% (0/8) | +0.65% (3/8) | 57.0 |
| 1x2 | catalog | ends | 7.755 | +12.80% (0/8) | -0.24% (5/8) | 56.8 |
| 1x2 | heldout | prior | 6.777 | +0.00% (0/8) | -12.01% (8/8) | 57.2 |
| 1x2 | heldout | ordinary | 7.723 | +13.65% (0/8) | +0.00% (0/8) | 58.2 |
| 1x2 | heldout | bridge | 7.672 | +13.28% (0/8) | -0.83% (6/8) | 57.2 |
| 1x2 | heldout | emit | 7.667 | +13.05% (0/8) | -0.68% (7/8) | 57.5 |
| 1x2 | heldout | ends | 7.670 | +12.90% (0/8) | -0.58% (7/8) | 57.2 |
| 2x2 | catalog | prior | 9.387 | +0.00% (0/8) | -8.14% (8/8) | 92.8 |
| 2x2 | catalog | ordinary | 10.206 | +8.86% (0/8) | +0.00% (0/8) | 94.0 |
| 2x2 | catalog | bridge | 10.188 | +8.74% (0/8) | -0.15% (4/8) | 92.2 |
| 2x2 | catalog | emit | 10.153 | +8.23% (0/8) | -0.66% (5/8) | 95.2 |
| 2x2 | catalog | ends | 10.218 | +9.04% (0/8) | -0.29% (5/8) | 92.2 |
| 2x2 | heldout | prior | 9.411 | +0.00% (0/8) | -8.30% (8/8) | 96.5 |
| 2x2 | heldout | ordinary | 10.263 | +9.05% (0/8) | +0.00% (0/8) | 98.8 |
| 2x2 | heldout | bridge | 10.148 | +7.96% (0/8) | -0.96% (8/8) | 91.8 |
| 2x2 | heldout | emit | 10.217 | +8.73% (0/8) | -0.58% (8/8) | 91.0 |
| 2x2 | heldout | ends | 10.185 | +7.99% (0/8) | -0.89% (5/8) | 94.0 |
| 4x2 | catalog | prior | 13.682 | +0.00% (0/8) | -0.63% (6/8) | 218.0 |
| 4x2 | catalog | ordinary | 13.735 | +0.63% (2/8) | +0.00% (0/8) | 224.5 |
| 4x2 | catalog | bridge | 13.689 | +0.13% (2/8) | -0.67% (5/8) | 208.0 |
| 4x2 | catalog | emit | 13.699 | +0.21% (3/8) | -0.27% (5/8) | 210.5 |
| 4x2 | catalog | ends | 13.637 | -0.05% (4/8) | -0.47% (5/8) | 210.5 |
| 4x2 | heldout | prior | 13.625 | +0.00% (0/8) | +0.09% (3/8) | 208.0 |
| 4x2 | heldout | ordinary | 13.620 | -0.09% (5/8) | +0.00% (0/8) | 221.0 |
| 4x2 | heldout | bridge | 13.641 | +0.04% (4/8) | -0.07% (4/8) | 217.0 |
| 4x2 | heldout | emit | 13.686 | +0.32% (3/8) | +0.29% (1/8) | 210.2 |
| 4x2 | heldout | ends | 13.608 | -0.41% (6/8) | -0.49% (6/8) | 208.5 |

| Shape | Input | Mode | Quotient FFT ms |
|---|---|---|---:|
| 1x2 | catalog | ordinary | 0.529 |
| 1x2 | catalog | bridge | 0.502 |
| 1x2 | catalog | emit | 0.488 |
| 1x2 | catalog | ends | 0.477 |
| 1x2 | heldout | ordinary | 0.524 |
| 1x2 | heldout | bridge | 0.501 |
| 1x2 | heldout | emit | 0.504 |
| 1x2 | heldout | ends | 0.480 |
| 2x2 | catalog | ordinary | 1.247 |
| 2x2 | catalog | bridge | 1.114 |
| 2x2 | catalog | emit | 1.157 |
| 2x2 | catalog | ends | 1.155 |
| 2x2 | heldout | ordinary | 1.175 |
| 2x2 | heldout | bridge | 1.160 |
| 2x2 | heldout | emit | 1.121 |
| 2x2 | heldout | ends | 1.165 |
| 4x2 | catalog | ordinary | 2.274 |
| 4x2 | catalog | bridge | 2.253 |
| 4x2 | catalog | emit | 2.203 |
| 4x2 | catalog | ends | 2.149 |
| 4x2 | heldout | ordinary | 2.219 |
| 4x2 | heldout | bridge | 2.267 |
| 4x2 | heldout | emit | 2.259 |
| 4x2 | heldout | ends | 2.205 |

| Shape | Input | Same-build bridge vs frozen prior | Emit vs bridge | Ends vs emit |
|---|---|---:|---:|---:|
| 1x2 | catalog | +13.43% (0/8) | -0.24% (4/8) | -0.42% (4/8) |
| 1x2 | heldout | +13.28% (0/8) | -0.02% (4/8) | +0.06% (4/8) |
| 2x2 | catalog | +8.74% (0/8) | +0.02% (4/8) | +0.36% (2/8) |
| 2x2 | heldout | +7.96% (0/8) | +0.43% (1/8) | -0.44% (5/8) |
| 4x2 | catalog | +0.13% (2/8) | +0.40% (2/8) | -0.45% (5/8) |
| 4x2 | heldout | +0.04% (4/8) | +0.62% (3/8) | -0.67% (6/8) |

| Shape | Input | Mode | FFT workers | Columns per task | Outer tasks |
|---|---|---|---:|---:|---:|
| 1x2 | catalog | bridge | 8 | 16 | 8 |
| 1x2 | catalog | emit | 8 | 16 | 8 |
| 1x2 | catalog | ends | 8 | 16 | 8 |
| 1x2 | heldout | bridge | 8 | 16 | 8 |
| 1x2 | heldout | emit | 8 | 16 | 8 |
| 1x2 | heldout | ends | 8 | 16 | 8 |
| 2x2 | catalog | bridge | 8 | 32 | 8 |
| 2x2 | catalog | emit | 8 | 32 | 8 |
| 2x2 | catalog | ends | 8 | 32 | 8 |
| 2x2 | heldout | bridge | 8 | 32 | 8 |
| 2x2 | heldout | emit | 8 | 32 | 8 |
| 2x2 | heldout | ends | 8 | 32 | 8 |
| 4x2 | catalog | bridge | 24 | 16 | 32 |
| 4x2 | catalog | emit | 24 | 16 | 32 |
| 4x2 | catalog | ends | 24 | 16 | 32 |
| 4x2 | heldout | bridge | 24 | 16 | 32 |
| 4x2 | heldout | emit | 24 | 16 | 32 |
| 4x2 | heldout | ends | 24 | 16 | 32 |

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
