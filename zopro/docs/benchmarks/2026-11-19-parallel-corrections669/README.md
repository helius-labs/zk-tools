# Parallel collection of sparse G1 and G2 corrections

The experimental prover already counts exceptions per group. The candidate sums those counts in tiles, computes the output range for each tile and writes points and canonical scalar differences directly into matching slices. G1 and G2 use the same implementation. Extra metadata is one offset per tile plus a sentinel; the normal memory reservation covers both output arrays. An unavailable reservation uses the ordinary collector. A Rust oracle checks order and exact field values across empty tiles and irregular boundaries.

Two original 12×2 placements, two other 12×2 witnesses and the original 8×2 and 16×2 requests use the exact mapped executable as frozen control. Off, G1, G2 and both use one new executable. G1/G2/both select tiles of 128 groups, and coarse selects 512 for both curves. Each timing process warms three proofs and measures eleven. The table gives medians of process means. Separate profiles and diagnostics are excluded from latency. All exported proofs verified independently against the original images.

| Shape or input | Collector | Warm ms | Paired vs off | Paired vs frozen | Peak MiB | M5 CPU / Metal (10× CPU blocks) |
|---|---|---:|---:|---:|---:|---:|
| original0 | frozen | 28.510 | -0.32% (5/8) | +0.00% (0/8) | 561.0 | 9.96× / 3.54× (4/8) |
| original0 | off | 28.550 | +0.00% (0/8) | +0.32% (3/8) | 543.6 | 9.95× / 3.54× (2/8) |
| original0 | coarse | 28.615 | -0.32% (6/8) | +0.04% (4/8) | 539.8 | 9.93× / 3.53× (2/8) |
| original0 | g1 | 28.609 | +0.12% (4/8) | +0.22% (4/8) | 542.2 | 9.93× / 3.53× (2/8) |
| original0 | both | 28.894 | +0.69% (3/8) | +1.30% (1/8) | 546.3 | 9.83× / 3.50× (0/8) |
| original0 | g2 | 28.685 | +0.76% (3/8) | +0.62% (2/8) | 554.0 | 9.90× / 3.52× (1/8) |
| original4 | frozen | 28.579 | +0.46% (2/6) | +0.00% (0/6) | 531.8 | 9.94× / 3.53× (2/6) |
| original4 | off | 28.552 | +0.00% (0/6) | -0.46% (4/6) | 534.8 | 9.95× / 3.54× (2/6) |
| original4 | coarse | 28.564 | -0.20% (4/6) | +0.29% (2/6) | 547.9 | 9.94× / 3.54× (1/6) |
| original4 | g1 | 28.987 | +1.58% (2/6) | +1.47% (1/6) | 551.0 | 9.80× / 3.48× (1/6) |
| original4 | both | 28.513 | +0.13% (3/6) | -0.56% (4/6) | 548.8 | 9.96× / 3.54× (2/6) |
| original4 | g2 | 28.675 | +0.17% (3/6) | +0.94% (2/6) | 538.5 | 9.90× / 3.52× (0/6) |
| held58 | frozen | 28.608 | +0.25% (1/4) | +0.00% (0/4) | 537.0 | — |
| held58 | off | 28.550 | +0.00% (0/4) | -0.25% (3/4) | 547.5 | — |
| held58 | coarse | 28.517 | -0.12% (3/4) | +0.40% (2/4) | 541.8 | — |
| held58 | g1 | 28.486 | -0.21% (3/4) | -0.25% (2/4) | 533.0 | — |
| held58 | both | 28.623 | +0.11% (1/4) | +0.47% (2/4) | 529.7 | — |
| held58 | g2 | 28.521 | -0.10% (2/4) | +0.74% (2/4) | 544.0 | — |
| scattered139 | frozen | 44.790 | -0.38% (3/4) | +0.00% (0/4) | 1008.5 | — |
| scattered139 | off | 45.349 | +0.00% (0/4) | +0.38% (1/4) | 1013.5 | — |
| scattered139 | coarse | 44.671 | -1.16% (2/4) | -0.42% (3/4) | 954.8 | — |
| scattered139 | g1 | 44.636 | -1.36% (3/4) | -0.48% (4/4) | 1011.8 | — |
| scattered139 | both | 45.052 | -0.55% (2/4) | +0.21% (2/4) | 958.3 | — |
| scattered139 | g2 | 45.131 | -1.28% (3/4) | -0.02% (2/4) | 964.6 | — |
| 8x2 | frozen | 23.276 | -1.26% (5/6) | +0.00% (0/6) | 502.8 | 8.12× / 3.18× (0/6) |
| 8x2 | off | 23.571 | +0.00% (0/6) | +1.27% (1/6) | 524.2 | 8.02× / 3.14× (0/6) |
| 8x2 | coarse | 23.725 | +0.00% (3/6) | +2.64% (2/6) | 527.2 | 7.97× / 3.12× (0/6) |
| 8x2 | g1 | 23.246 | -1.37% (4/6) | -0.19% (4/6) | 516.0 | 8.13× / 3.18× (0/6) |
| 8x2 | both | 23.228 | -1.13% (4/6) | +0.97% (2/6) | 511.0 | 8.14× / 3.19× (0/6) |
| 8x2 | g2 | 23.218 | -1.80% (4/6) | -0.37% (4/6) | 516.8 | 8.14× / 3.19× (0/6) |
| 16x2 | frozen | 38.042 | +1.01% (1/6) | +0.00% (0/6) | 734.4 | 8.81× / 3.26× (0/6) |
| 16x2 | off | 37.715 | +0.00% (0/6) | -0.99% (5/6) | 758.1 | 8.88× / 3.29× (0/6) |
| 16x2 | coarse | 37.812 | -0.28% (3/6) | -0.47% (3/6) | 733.1 | 8.86× / 3.28× (0/6) |
| 16x2 | g1 | 37.439 | -0.11% (3/6) | -1.57% (4/6) | 768.1 | 8.95× / 3.31× (0/6) |
| 16x2 | both | 37.674 | +0.09% (3/6) | -0.18% (3/6) | 750.6 | 8.89× / 3.29× (0/6) |
| 16x2 | g2 | 37.077 | -1.69% (4/6) | -2.67% (4/6) | 741.8 | 9.04× / 3.34× (0/6) |

On the original 12×2 input, parallelizing G2 alone won 5/14 blocks against the same-binary control across the two placements. The 512-group tile size won 10/14 against disabled and 6/14 against the original executable. None of its placement medians reached 28.4 ms. The scattered witness entered neither parallel collector. Its timing differences therefore provide no evidence for the algorithm.

On 8×2 and 16×2 the G2 collector ran on 269 and 1,046 corrections respectively. It won 4/6 pairs against each control on each shape. At 16×2 this gave 37.077 ms against 38.042 ms frozen, with a 1.69% paired saving against disabled. This result warrants a repeat on these shapes before changing the release. The 160 native tests passed. All 246 proof exports verified.

[Incremental source](artifacts/source.patch) · [Actual collection sizes](artifacts/diagnostics.json) · [Exact configurations](artifacts/configs.json.gz) · [Warm samples and profiles](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
