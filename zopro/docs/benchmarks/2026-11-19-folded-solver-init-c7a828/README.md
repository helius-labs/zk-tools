# Solver buffer initialization and placement on C7a

The [Ryzen qualification](../2026-11-19-folded-solver-init-ryzen827/README.md) confirmed that the definite-assignment check admits the uninitialized buffer path for both fixtures. The C7a trial separates skipping initialization from moving allocation and witness copying onto the pool already running the instructions. The usual route uses initialized solver vectors. At zero optional memory the solver retains initialized storage. FFT and curve routines are shared.

The 4×2 layout uses 64 main and 32 frontend cores. The 8×2 layout uses 72 and 24, with packed solver tasks, recognized B2 origins and serial scalar conversion. Prepared Aeglos uses all 96 cores. There are four blocks per 4×2 input, six per primary 8×2 input and four per additional 8×2 input. Each process primes three calls and times thirteen. All 224 exported proofs independently verified, including the zero-workspace fallback. The temporary instance and storage were removed.

- 4×2 `full` took 15.103 / 15.143 ms across original and held-out, paired -1.55% / -1.91% from the same-build ordinary arm.
- 8×2 `full-uninit` took 19.052 / 19.350 ms across original and held-out, paired -1.75% / -2.21% from the same-build ordinary arm.

The following modes gained consistently against both controls across their tested inputs: 4×2 full.

| Shape | Input | Mode | Proof ms | Paired vs ordinary | Paired vs release | Paired vs Aeglos | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|
| 4×2 | catalog | release | 15.483 | +1.00% (2/4) | +0.00% (0/4) | -13.95% (4/4) | 213.2 |
| 4×2 | catalog | ordinary | 15.341 | +0.00% (0/4) | -0.98% (2/4) | -14.89% (4/4) | 196.7 |
| 4×2 | catalog | uninit | 15.265 | -0.87% (4/4) | -1.49% (4/4) | -15.35% (4/4) | 194.4 |
| 4×2 | catalog | full | 15.103 | -1.55% (3/4) | -1.57% (3/4) | -15.88% (4/4) | 197.4 |
| 4×2 | catalog | full-uninit | 15.349 | +0.12% (1/4) | -1.35% (3/4) | -15.00% (4/4) | 191.4 |
| 4×2 | catalog | aeglos | 18.007 | +17.49% (0/4) | +16.22% (0/4) | +0.00% (0/4) | 1346.8 |
| 4×2 | catalog | ordinary-clock | 15.365 | +0.02% (1/4) | -0.65% (2/4) | -14.49% (4/4) | 207.9 |
| 4×2 | catalog | full-uninit-clock | 15.373 | +0.21% (2/4) | -0.38% (3/4) | -14.29% (4/4) | 195.1 |
| 4×2 | heldout | release | 15.430 | -0.05% (2/4) | +0.00% (0/4) | -15.66% (4/4) | 205.6 |
| 4×2 | heldout | ordinary | 15.439 | +0.00% (0/4) | +0.06% (2/4) | -15.57% (4/4) | 206.4 |
| 4×2 | heldout | uninit | 15.355 | -0.46% (2/4) | -0.54% (3/4) | -16.18% (4/4) | 197.4 |
| 4×2 | heldout | full | 15.143 | -1.91% (4/4) | -2.00% (4/4) | -17.21% (4/4) | 205.0 |
| 4×2 | heldout | full-uninit | 15.382 | +0.09% (1/4) | -0.76% (3/4) | -15.11% (4/4) | 184.7 |
| 4×2 | heldout | aeglos | 18.276 | +18.44% (0/4) | +18.61% (0/4) | +0.00% (0/4) | 1352.3 |
| 4×2 | heldout | ordinary-clock | 15.370 | -0.47% (3/4) | -0.36% (3/4) | -15.75% (4/4) | 208.7 |
| 4×2 | heldout | full-uninit-clock | 15.481 | +0.69% (1/4) | +0.45% (1/4) | -15.22% (4/4) | 185.5 |
| 8×2 | catalog | release | 19.504 | +0.90% (1/6) | +0.00% (0/6) | -36.35% (6/6) | 365.8 |
| 8×2 | catalog | ordinary | 19.408 | +0.00% (0/6) | -0.89% (5/6) | -36.72% (6/6) | 387.6 |
| 8×2 | catalog | uninit | 19.052 | -1.23% (5/6) | -2.11% (6/6) | -37.25% (6/6) | 372.4 |
| 8×2 | catalog | full | 19.580 | +0.62% (1/6) | +0.43% (2/6) | -35.63% (6/6) | 374.0 |
| 8×2 | catalog | full-uninit | 19.052 | -1.75% (4/6) | -1.94% (6/6) | -37.27% (6/6) | 367.3 |
| 8×2 | catalog | aeglos | 30.495 | +58.03% (0/6) | +57.11% (0/6) | +0.00% (0/6) | 2460.3 |
| 8×2 | catalog | ordinary-clock | 19.597 | +1.23% (0/6) | +0.33% (1/6) | -35.66% (6/6) | 386.2 |
| 8×2 | catalog | full-uninit-clock | 19.248 | -0.57% (4/6) | -0.93% (6/6) | -36.88% (6/6) | 355.5 |
| 8×2 | heldout | release | 19.713 | +0.43% (3/6) | +0.00% (0/6) | -35.86% (6/6) | 374.7 |
| 8×2 | heldout | ordinary | 19.771 | +0.00% (0/6) | -0.43% (3/6) | -36.20% (6/6) | 376.3 |
| 8×2 | heldout | uninit | 19.511 | -1.46% (5/6) | -0.75% (4/6) | -36.26% (6/6) | 366.5 |
| 8×2 | heldout | full | 19.737 | +0.20% (3/6) | +0.15% (3/6) | -35.88% (6/6) | 385.9 |
| 8×2 | heldout | full-uninit | 19.350 | -2.21% (5/6) | -1.47% (6/6) | -36.87% (6/6) | 367.8 |
| 8×2 | heldout | aeglos | 30.733 | +56.75% (0/6) | +55.91% (0/6) | +0.00% (0/6) | 2464.0 |
| 8×2 | heldout | ordinary-clock | 19.742 | -0.19% (4/6) | +0.89% (2/6) | -35.57% (6/6) | 382.9 |
| 8×2 | heldout | full-uninit-clock | 19.659 | -0.26% (3/6) | -0.65% (4/6) | -35.99% (6/6) | 366.4 |
| 8×2 | v153 | release | 19.698 | -0.74% (3/4) | +0.00% (0/4) | -35.73% (4/4) | 363.5 |
| 8×2 | v153 | ordinary | 19.779 | +0.00% (0/4) | +0.74% (1/4) | -35.55% (4/4) | 371.5 |
| 8×2 | v153 | uninit | 19.435 | -2.07% (4/4) | -1.66% (4/4) | -36.88% (4/4) | 367.3 |
| 8×2 | v153 | full | 19.344 | -2.28% (4/4) | -1.88% (4/4) | -37.02% (4/4) | 373.4 |
| 8×2 | v153 | full-uninit | 19.395 | -2.65% (3/4) | -2.24% (3/4) | -37.26% (4/4) | 368.0 |
| 8×2 | v153 | aeglos | 30.775 | +55.16% (0/4) | +55.59% (0/4) | +0.00% (0/4) | 2460.6 |
| 8×2 | v156 | release | 19.733 | +0.00% (2/4) | +0.00% (0/4) | -35.67% (4/4) | 381.6 |
| 8×2 | v156 | ordinary | 19.702 | +0.00% (0/4) | -0.00% (2/4) | -35.67% (4/4) | 375.5 |
| 8×2 | v156 | uninit | 19.591 | -1.34% (4/4) | -1.07% (4/4) | -36.37% (4/4) | 371.8 |
| 8×2 | v156 | full | 19.830 | +0.11% (1/4) | +0.25% (1/4) | -35.06% (4/4) | 383.8 |
| 8×2 | v156 | full-uninit | 19.218 | -2.39% (4/4) | -2.71% (4/4) | -37.58% (4/4) | 355.3 |
| 8×2 | v156 | aeglos | 30.787 | +55.47% (0/4) | +55.47% (0/4) | +0.00% (0/4) | 2460.8 |

## Clocked stage times

| Shape | Input | Schedule | Proof ms | Paired clock effect | Solve ms | Gather ms | Quotient ms | Curve interval ms |
|---|---|---|---:|---:|---:|---:|---:|---:|
| 4×2 | catalog | ordinary | 15.365 | +0.02% | 3.496 | 0.995 | 2.052 | 7.316 |
| 4×2 | catalog | full-uninit | 15.373 | +0.50% | 3.387 | 1.013 | 2.061 | 7.423 |
| 4×2 | heldout | ordinary | 15.370 | -0.47% | 3.521 | 0.977 | 2.038 | 7.330 |
| 4×2 | heldout | full-uninit | 15.481 | +0.65% | 3.318 | 1.004 | 2.061 | 7.547 |
| 8×2 | catalog | ordinary | 19.597 | +1.23% | 3.670 | 0.986 | 2.842 | 10.099 |
| 8×2 | catalog | full-uninit | 19.248 | +0.59% | 3.226 | 1.008 | 2.864 | 10.084 |
| 8×2 | heldout | ordinary | 19.742 | -0.19% | 3.693 | 0.972 | 2.841 | 10.110 |
| 8×2 | heldout | full-uninit | 19.659 | +1.13% | 3.291 | 1.017 | 2.873 | 10.355 |

[Samples and clocks](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU.

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 805 released SDK 80+16 | 7.858 | 37 / 24 | 4.71× / 3.05× | +4.158 |
| 2×2 | 805 released SDK medium K/H 64+32 | 12.029 | 59 / 30 | 4.90× / 2.49× | +6.129 |
| 4×2 | 828 frontend solver full | 15.103 | 109 / 48 | 7.22× / 3.18× | +4.203 |
| 8×2 | 826 released SDK 72+24 packed frontend | 19.499 | 189 / 74 | 9.69× / 3.79× | +0.599 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
