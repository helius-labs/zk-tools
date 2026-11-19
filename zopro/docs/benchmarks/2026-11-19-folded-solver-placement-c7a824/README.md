# Packed solver placement for 8×2 on C7a

The [previous timeline](../2026-11-19-folded-timeline-c7a822/README.md) measured about 4.6 ms in the 8×2 solver before the quotient and curve queries. This run tests the existing frontend solver option with packed task claiming and the current B2 origin and serial conversion settings. The ordinary comparison places the solver on the main pool. The unpacked arm disables both packed tasks and claiming. Curve and FFT options are shared among all Zopro arms.

The primary layout pairs 64 main and 32 frontend physical cores. Two additional layouts use 56+40 and 72+24. Prepared Aeglos uses all 96 cores of the same c7a.24xlarge. The primary requests have six matched blocks and the two additional requests have four. Each process primes three calls and times thirteen. All 140 exports independently verified. The temporary instance and storage were removed.

On the original input, the solver span was 4.628 ms on the main pool and 3.946 ms on the frontend. The full-proof medians were 20.159 and 19.687 ms, respectively. The packed frontend solver beat both controls on all four inputs.

| Input | Mode | Proof ms | Paired vs packed main | Paired vs release | Paired vs Aeglos | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| catalog | release | 20.221 | +0.63% (2/6) | +0.00% (0/6) | -33.63% (6/6) | 405.6 |
| catalog | main-packed | 20.159 | +0.00% (0/6) | -0.63% (4/6) | -33.99% (6/6) | 416.7 |
| catalog | front-packed | 19.687 | -2.12% (6/6) | -2.57% (6/6) | -35.35% (6/6) | 372.5 |
| catalog | front-unpacked | 20.712 | +2.91% (0/6) | +2.12% (0/6) | -32.19% (6/6) | 374.9 |
| catalog | front-packed-56 | 20.371 | +1.05% (0/6) | +0.97% (0/6) | -32.83% (6/6) | 365.7 |
| catalog | front-packed-72 | 19.363 | -4.32% (6/6) | -4.47% (6/6) | -36.52% (6/6) | 371.7 |
| catalog | main-packed-clock | 20.356 | +0.76% (1/6) | +0.19% (1/6) | -33.08% (6/6) | 406.1 |
| catalog | front-packed-clock | 19.810 | -1.97% (6/6) | -2.04% (6/6) | -34.87% (6/6) | 377.8 |
| catalog | aeglos | 30.386 | +51.50% (0/6) | +50.67% (0/6) | +0.00% (0/6) | 2460.0 |
| heldout | release | 20.545 | +0.82% (2/6) | +0.00% (0/6) | -33.91% (6/6) | 413.8 |
| heldout | main-packed | 20.404 | +0.00% (0/6) | -0.81% (4/6) | -34.14% (6/6) | 403.1 |
| heldout | front-packed | 19.979 | -2.26% (6/6) | -2.37% (6/6) | -35.52% (6/6) | 377.5 |
| heldout | front-unpacked | 21.190 | +3.75% (0/6) | +3.74% (0/6) | -31.84% (6/6) | 375.3 |
| heldout | front-packed-56 | 20.553 | +0.43% (1/6) | +0.45% (3/6) | -33.77% (6/6) | 384.0 |
| heldout | front-packed-72 | 19.671 | -3.41% (6/6) | -3.44% (6/6) | -36.27% (6/6) | 389.0 |
| heldout | main-packed-clock | 20.473 | +0.04% (3/6) | -0.28% (3/6) | -34.00% (6/6) | 407.7 |
| heldout | front-packed-clock | 19.874 | -2.58% (6/6) | -2.27% (6/6) | -35.86% (6/6) | 383.7 |
| heldout | aeglos | 30.974 | +51.85% (0/6) | +51.31% (0/6) | +0.00% (0/6) | 2467.6 |
| v153 | release | 20.631 | +0.89% (2/4) | +0.00% (0/4) | -32.02% (4/4) | 410.5 |
| v153 | main-packed | 20.530 | +0.00% (0/4) | -0.87% (2/4) | -32.94% (4/4) | 416.2 |
| v153 | front-packed | 20.065 | -2.57% (4/4) | -2.62% (4/4) | -34.78% (4/4) | 372.6 |
| v153 | aeglos | 30.669 | +49.13% (0/4) | +47.14% (0/4) | +0.00% (0/4) | 2479.3 |
| v156 | release | 20.394 | -0.40% (2/4) | +0.00% (0/4) | -33.75% (4/4) | 425.8 |
| v156 | main-packed | 20.400 | +0.00% (0/4) | +0.40% (2/4) | -33.20% (4/4) | 406.3 |
| v156 | front-packed | 20.037 | -1.78% (4/4) | -2.00% (4/4) | -34.82% (4/4) | 382.8 |
| v156 | aeglos | 30.664 | +49.70% (0/4) | +50.94% (0/4) | +0.00% (0/4) | 2463.0 |

## Clocked stage times

| Input | Schedule | Proof ms | Paired clock effect | Solve ms | Gather ms | Quotient ms | Curve interval ms |
|---|---|---:|---:|---:|---:|---:|---:|
| catalog | main-packed | 20.356 | +0.76% | 4.628 | 0.910 | 2.750 | 10.052 |
| catalog | front-packed | 19.810 | +0.90% | 3.946 | 0.858 | 2.844 | 10.130 |
| heldout | main-packed | 20.473 | +0.04% | 4.635 | 0.895 | 2.760 | 10.207 |
| heldout | front-packed | 19.874 | -0.07% | 3.911 | 0.883 | 2.838 | 10.196 |

[Samples and clocks](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU.

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 805 released SDK 80+16 | 7.858 | 37 / 24 | 4.71× / 3.05× | +4.158 |
| 2×2 | 805 released SDK medium K/H 64+32 | 12.029 | 59 / 30 | 4.90× / 2.49× | +6.129 |
| 4×2 | 757 solve-front-on | 15.257 | 109 / 48 | 7.14× / 3.15× | +4.357 |
| 8×2 | 824 packed frontend solver 64+32 | 19.687 | 189 / 74 | 9.60× / 3.76× | +0.787 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
