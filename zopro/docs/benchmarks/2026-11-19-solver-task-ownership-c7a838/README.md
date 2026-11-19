# Solver task ownership and local bands for 8×2 on C7a

All Zopro arms use the established 72+24 split, certified buffer allocation, four outer solver epochs, packed tasks, recognized B2 origins, frontend K and serial B2 conversion. The [seed and stripe modes](../2026-11-19-solver-task-ownership-ryzen837/README.md) change how workers own the task indices of a wave. The local-band and minimum-size variants instead use the existing controls for refining large tasks. The ordinary mode retains dynamic claiming and the default refinement.

The two primary inputs have six matched blocks. Two additional inputs have four. Each process primes three calls and times thirteen. Prepared Aeglos uses all 96 physical cores on the same machine. All 200 proof exports independently verified. The temporary AWS instance and storage were removed.

The lowest eligible original-request median was 18.857 ms with `seed`. No candidate beat its controls consistently across the four inputs.

| Input | Mode | Proof ms | Paired vs release | Paired vs same-build ordinary | Paired vs Aeglos | Peak MiB | Blocks at or below 18.9 ms |
|---|---|---:|---:|---:|---:|---:|---:|
| catalog | release | 19.286 | +0.00% (0/6) | +1.11% (2/6) | -37.16% (6/6) | 361.9 | 0/6 |
| catalog | ordinary | 19.133 | -1.10% (4/6) | +0.00% (0/6) | -37.83% (6/6) | 360.0 | 0/6 |
| catalog | seed | 18.857 | -1.73% (5/6) | -1.55% (4/6) | -38.73% (6/6) | 374.0 | 3/6 |
| catalog | stripe | 19.727 | +2.28% (0/6) | +3.10% (0/6) | -35.62% (6/6) | 363.6 | 0/6 |
| catalog | local-0 | 24.627 | +28.24% (0/6) | +28.58% (0/6) | -19.23% (6/6) | 359.7 | 0/6 |
| catalog | local-4 | 19.957 | +4.13% (0/6) | +4.42% (0/6) | -35.09% (6/6) | 370.8 | 0/6 |
| catalog | local-16 | 19.136 | -0.58% (4/6) | -0.60% (4/6) | -37.47% (6/6) | 363.7 | 1/6 |
| catalog | minimum-2048 | 19.609 | +2.07% (0/6) | +2.52% (0/6) | -35.57% (6/6) | 365.3 | 0/6 |
| catalog | minimum-8192 | 18.998 | -0.93% (5/6) | -0.52% (4/6) | -37.98% (6/6) | 350.1 | 1/6 |
| catalog | aeglos | 30.744 | +59.13% (0/6) | +60.85% (0/6) | +0.00% (0/6) | 2457.6 | 0/6 |
| heldout | release | 19.558 | +0.00% (0/6) | +1.95% (2/6) | -36.62% (6/6) | 362.6 | 0/6 |
| heldout | ordinary | 19.228 | -1.90% (4/6) | +0.00% (0/6) | -37.41% (6/6) | 362.7 | 1/6 |
| heldout | seed | 19.315 | -1.21% (3/6) | +0.31% (3/6) | -36.69% (6/6) | 369.6 | 0/6 |
| heldout | stripe | 20.103 | +2.10% (0/6) | +4.67% (1/6) | -34.58% (6/6) | 370.3 | 0/6 |
| heldout | local-0 | 25.099 | +28.47% (0/6) | +28.23% (0/6) | -18.52% (6/6) | 360.5 | 0/6 |
| heldout | local-4 | 20.359 | +4.47% (0/6) | +5.55% (1/6) | -33.73% (6/6) | 362.7 | 0/6 |
| heldout | local-16 | 19.291 | -0.67% (5/6) | +0.77% (2/6) | -37.06% (6/6) | 386.8 | 0/6 |
| heldout | minimum-2048 | 19.898 | +1.99% (1/6) | +3.41% (1/6) | -34.64% (6/6) | 364.1 | 0/6 |
| heldout | minimum-8192 | 19.321 | -1.29% (5/6) | -0.49% (3/6) | -37.10% (6/6) | 365.7 | 0/6 |
| heldout | aeglos | 30.607 | +57.79% (0/6) | +59.77% (0/6) | +0.00% (0/6) | 2461.7 | 0/6 |
| v153 | release | 19.326 | +0.00% (0/4) | +0.49% (1/4) | -37.42% (4/4) | 357.4 | 0/4 |
| v153 | ordinary | 19.283 | -0.49% (3/4) | +0.00% (0/4) | -37.52% (4/4) | 358.3 | 0/4 |
| v153 | seed | 19.215 | -0.52% (2/4) | -0.01% (2/4) | -37.52% (4/4) | 367.8 | 1/4 |
| v153 | stripe | 19.862 | +2.40% (0/4) | +2.91% (0/4) | -35.92% (4/4) | 358.2 | 0/4 |
| v153 | local-0 | 24.970 | +28.35% (0/4) | +29.34% (0/4) | -19.62% (4/4) | 364.2 | 0/4 |
| v153 | local-4 | 20.124 | +4.18% (0/4) | +4.40% (0/4) | -34.57% (4/4) | 376.8 | 0/4 |
| v153 | local-16 | 19.280 | -0.31% (2/4) | -0.08% (2/4) | -37.56% (4/4) | 345.5 | 0/4 |
| v153 | minimum-2048 | 19.990 | +3.99% (1/4) | +3.70% (0/4) | -35.01% (4/4) | 362.8 | 0/4 |
| v153 | minimum-8192 | 19.221 | -0.95% (3/4) | -0.64% (3/4) | -38.21% (4/4) | 371.6 | 0/4 |
| v153 | aeglos | 30.858 | +59.80% (0/4) | +60.06% (0/4) | +0.00% (0/4) | 2461.1 | 0/4 |
| v156 | release | 19.277 | +0.00% (0/4) | -0.09% (2/4) | -37.03% (4/4) | 347.0 | 0/4 |
| v156 | ordinary | 19.274 | +0.09% (2/4) | +0.00% (0/4) | -36.97% (4/4) | 363.5 | 0/4 |
| v156 | seed | 19.183 | -0.48% (4/4) | -0.40% (3/4) | -37.24% (4/4) | 351.4 | 0/4 |
| v156 | stripe | 20.012 | +3.53% (0/4) | +4.20% (0/4) | -34.35% (4/4) | 359.8 | 0/4 |
| v156 | local-0 | 25.320 | +30.66% (0/4) | +32.05% (0/4) | -17.06% (4/4) | 364.1 | 0/4 |
| v156 | local-4 | 20.190 | +4.24% (0/4) | +4.32% (0/4) | -34.23% (4/4) | 359.6 | 0/4 |
| v156 | local-16 | 19.386 | +0.57% (1/4) | +0.61% (1/4) | -36.73% (4/4) | 368.1 | 0/4 |
| v156 | minimum-2048 | 20.035 | +3.93% (0/4) | +4.44% (0/4) | -34.28% (4/4) | 361.9 | 0/4 |
| v156 | minimum-8192 | 19.260 | -0.66% (2/4) | +0.06% (2/4) | -36.93% (4/4) | 360.5 | 0/4 |
| v156 | aeglos | 30.580 | +58.81% (0/4) | +58.66% (0/4) | +0.00% (0/4) | 2458.4 | 0/4 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json) · [Source](artifacts/source.patch)

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU.

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 805 released SDK 80+16 | 7.858 | 37 / 24 | 4.71× / 3.05× | +4.158 |
| 2×2 | 805 released SDK medium K/H 64+32 | 12.029 | 59 / 30 | 4.90× / 2.49× | +6.129 |
| 4×2 | 828 frontend solver full | 15.103 | 109 / 48 | 7.22× / 3.18× | +4.203 |
| 8×2 | 833 SDK release-e4 72+24 | 19.117 | 189 / 74 | 9.89× / 3.87× | +0.217 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
