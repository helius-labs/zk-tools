# H window placement and signed digits for 4×2 on C7a

The [H point-split run](../2026-11-20-four-by-two-h-split-c7a893/README.md) added a second set of weighted reductions. Here, the frontend mode sends two existing H digit windows, indexed 1 and 11 of 22, to the frontend. Each window retains the original point ranges and weighted reduction. The main-indexed mode schedules those two windows separately on the main pool. A distinct arithmetic comparison uses 20 signed width-13 windows and an explicit width-12 unsigned control. Ordinary windows use 4,096 buckets in both cases. The signed and unsigned leading windows retain their respective narrow reductions. On each route, H uses 98,674 points and two point ranges.

K stays on the frontend. The seeded solver, FFT, and verification retain the frozen settings. Zopro uses 64 main and 32 frontend physical cores. Prepared Aeglos uses all 96. Eight counterbalanced blocks per input prime three requests and time thirteen. A mode advances with lower paired medians and six wins against each applicable Zopro control on each input. All 172 exports independently verified. Separate traces checked routing and the zero-optional-workspace fallback. The temporary instance and storage were removed.

The frozen seeded profile took 15.131 / 15.195 ms. Frontend windows took 15.172 / 15.318 ms, with paired changes of +0.42% / +1.65% against ordinary. Signed H took 15.138 / 15.409 ms, with paired changes of -0.17% / +1.75% against ordinary. Neither approach produced a consistent latency improvement.

| Input | Mode | Proof ms | Paired vs ordinary | Paired vs frozen | Paired vs main-indexed or unsigned12 | Paired vs Aeglos | Peak MiB | Blocks at or below 10.9 ms |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | release | 15.509 | +2.28% (0/8) | +2.78% (0/8) | — | -17.06% (8/8) | 217.7 | 0/8 |
| catalog | frozen | 15.131 | -0.38% (6/8) | +0.00% (0/8) | — | -19.02% (8/8) | 192.9 | 0/8 |
| catalog | ordinary | 15.155 | +0.00% (0/8) | +0.39% (2/8) | — | -18.91% (8/8) | 192.2 | 0/8 |
| catalog | main2 | 15.228 | +0.49% (3/8) | +0.92% (3/8) | — | -18.68% (8/8) | 208.6 | 0/8 |
| catalog | front2 | 15.172 | +0.42% (3/8) | +0.83% (3/8) | -0.13% (4/8) | -19.41% (8/8) | 207.9 | 0/8 |
| catalog | unsigned12 | 15.258 | +0.63% (2/8) | +1.02% (2/8) | — | -18.72% (8/8) | 197.5 | 0/8 |
| catalog | signed13 | 15.138 | -0.17% (5/8) | +0.92% (3/8) | -0.79% (5/8) | -19.27% (8/8) | 201.1 | 0/8 |
| catalog | aeglos | 18.777 | +23.32% (0/8) | +23.48% (0/8) | — | +0.00% (0/8) | 1355.1 | 0/8 |
| heldout | release | 15.559 | +3.06% (0/8) | +2.49% (0/8) | — | -17.35% (8/8) | 209.4 | 0/8 |
| heldout | frozen | 15.195 | +0.69% (1/8) | +0.00% (0/8) | — | -18.99% (8/8) | 198.1 | 0/8 |
| heldout | ordinary | 15.107 | +0.00% (0/8) | -0.68% (7/8) | — | -19.29% (8/8) | 202.8 | 0/8 |
| heldout | main2 | 15.230 | +1.14% (2/8) | +0.01% (4/8) | — | -18.16% (8/8) | 196.7 | 0/8 |
| heldout | front2 | 15.318 | +1.65% (1/8) | +1.12% (1/8) | +0.21% (2/8) | -18.16% (8/8) | 199.6 | 0/8 |
| heldout | unsigned12 | 15.356 | +1.48% (0/8) | +1.28% (3/8) | — | -18.08% (8/8) | 207.9 | 0/8 |
| heldout | signed13 | 15.409 | +1.75% (1/8) | +1.51% (2/8) | -0.52% (5/8) | -18.21% (8/8) | 205.8 | 0/8 |
| heldout | aeglos | 18.726 | +23.90% (0/8) | +23.44% (0/8) | — | +0.00% (0/8) | 1364.3 | 0/8 |

In three separate blocks, stage clocks measure the curve span and each query. Linux per-worker schedstat sums scheduled CPU time for each pinned pool across the timed requests. The counts include solver and FFT work, scheduling and worker spin. These blocks measure placement effects and do not enter the latency comparison.

| Input | Mode | Diagnostic proof ms | Curves ms | A ms | K ms | H ms | B2 ms | Main CPU-ms/proof | Front CPU-ms/proof |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| catalog | ordinary | 15.227 | 7.761 | 6.347 | 7.297 | 5.401 | 7.332 | 538.7 | 125.2 |
| catalog | main2 | 15.158 | 7.624 | 5.602 | 7.458 | 6.502 | 7.441 | 528.1 | 126.3 |
| catalog | front2 | 15.291 | 7.684 | 5.685 | 7.103 | 5.854 | 7.510 | 526.5 | 144.2 |
| catalog | unsigned12 | 15.177 | 7.663 | 5.605 | 7.418 | 5.885 | 7.552 | 526.5 | 126.0 |
| catalog | signed13 | 15.284 | 7.765 | 5.626 | 7.487 | 6.215 | 7.378 | 529.6 | 125.9 |
| heldout | ordinary | 15.151 | 7.627 | 5.607 | 7.248 | 5.605 | 7.385 | 531.2 | 125.0 |
| heldout | main2 | 15.240 | 7.730 | 5.406 | 7.412 | 6.468 | 7.474 | 531.6 | 126.9 |
| heldout | front2 | 15.540 | 8.061 | 5.276 | 7.432 | 6.804 | 7.577 | 536.1 | 145.5 |
| heldout | unsigned12 | 15.690 | 8.119 | 6.492 | 7.561 | 6.528 | 7.811 | 545.1 | 128.2 |
| heldout | signed13 | 15.341 | 7.754 | 5.284 | 7.531 | 5.811 | 7.359 | 530.3 | 126.7 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Placement source](artifacts/source.patch) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**5/8** original requests reach 10× M5 CPU on the recorded profiles.

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|---:|
| 1×2 | 805 released SDK 80+16 | 7.858 | 37 / 24 | 4.71× / 3.05× | +4.158 |
| 2×2 | 805 released SDK medium K/H 64+32 | 12.029 | 59 / 30 | 4.90× / 2.49× | +6.129 |
| 4×2 | 854 seed4-emit | 14.726 | 109 / 48 | 7.40× / 3.26× | +3.826 |
| 8×2 | 852 SDK seeded tasks and B2 vector conversion | 18.849 | 189 / 74 | 10.03× / 3.93× | -0.051 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
