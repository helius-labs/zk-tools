# Full frontend solver and seeded task confirmation on C7a

The [SDK qualification](../2026-11-19-frontend-solver-full-ryzen839/README.md) moved the existing full frontend solver path into the SDK option. This run compares it against the instruction-only setting on four inputs per shape. It also repeats the [seeded solver schedule](../2026-11-19-solver-task-ownership-c7a838/README.md) on 8×2 and tests the existing encoded Poseidon emission and a higher refinement minimum. The seed modes use the task-ordinary control. Other modes use the ordinary SDK control.

The 4×2 layout uses 64 main and 32 frontend cores. The 8×2 layout uses 72 main and 24 frontend cores, with four solver epochs, packed tasks, certified buffers, recognized B2 origins and serial conversion. Prepared Aeglos uses the same 96 physical cores. There are four blocks per 4×2 input, six per primary 8×2 input and four per additional 8×2 input. Each process primes three calls and times thirteen. All 292 exported proofs independently verified, including checks at zero optional workspace and with a detached frontend. The temporary instance and storage were removed.

- 4×2 `full` took 15.139 / 15.242 ms across the original and held-out inputs.
- 8×2 `seed-emit` took 18.787 / 19.173 ms across the original and held-out inputs.

No profile improved both controls consistently across all four inputs.

| Shape | Input | Mode | Proof ms | Paired vs same-build control | Paired vs release | Paired vs Aeglos | Peak MiB | Blocks within target |
|---|---|---|---:|---:|---:|---:|---:|---:|
| 4×2 | catalog | release | 15.255 | +0.71% (1/4) | +0.00% (0/4) | -17.54% (4/4) | 201.9 | 0/4 |
| 4×2 | catalog | ordinary | 15.312 | +0.00% (0/4) | -0.70% (3/4) | -17.37% (4/4) | 201.1 | 0/4 |
| 4×2 | catalog | full | 15.139 | -1.39% (3/4) | -1.02% (4/4) | -17.70% (4/4) | 203.5 | 0/4 |
| 4×2 | catalog | full-uninit | 15.359 | +1.21% (1/4) | +1.08% (1/4) | -16.48% (4/4) | 195.2 | 0/4 |
| 4×2 | catalog | aeglos | 18.399 | +21.03% (0/4) | +21.27% (0/4) | +0.00% (0/4) | 1350.5 | 0/4 |
| 4×2 | heldout | release | 15.474 | +0.34% (2/4) | +0.00% (0/4) | -14.69% (4/4) | 202.7 | 0/4 |
| 4×2 | heldout | ordinary | 15.300 | +0.00% (0/4) | -0.34% (2/4) | -15.37% (4/4) | 221.7 | 0/4 |
| 4×2 | heldout | full | 15.242 | -0.23% (2/4) | -1.13% (3/4) | -15.57% (4/4) | 204.3 | 0/4 |
| 4×2 | heldout | full-uninit | 15.405 | +0.32% (1/4) | +0.11% (2/4) | -14.67% (4/4) | 190.6 | 0/4 |
| 4×2 | heldout | aeglos | 18.053 | +18.16% (0/4) | +17.22% (0/4) | +0.00% (0/4) | 1348.1 | 0/4 |
| 4×2 | v153 | release | 15.343 | +0.19% (1/4) | +0.00% (0/4) | -15.20% (4/4) | 204.9 | 0/4 |
| 4×2 | v153 | ordinary | 15.318 | +0.00% (0/4) | -0.19% (3/4) | -15.04% (4/4) | 213.2 | 0/4 |
| 4×2 | v153 | full | 15.213 | -1.04% (4/4) | -0.76% (4/4) | -16.00% (4/4) | 206.5 | 0/4 |
| 4×2 | v153 | full-uninit | 15.640 | +2.10% (0/4) | +1.86% (0/4) | -13.29% (4/4) | 189.9 | 0/4 |
| 4×2 | v153 | aeglos | 18.070 | +17.71% (0/4) | +17.92% (0/4) | +0.00% (0/4) | 1345.7 | 0/4 |
| 4×2 | v156 | release | 15.496 | +0.92% (1/4) | +0.00% (0/4) | -15.53% (4/4) | 208.7 | 0/4 |
| 4×2 | v156 | ordinary | 15.386 | +0.00% (0/4) | -0.90% (3/4) | -16.05% (4/4) | 200.6 | 0/4 |
| 4×2 | v156 | full | 15.072 | -1.73% (4/4) | -2.82% (4/4) | -17.53% (4/4) | 212.4 | 0/4 |
| 4×2 | v156 | full-uninit | 15.399 | +0.34% (1/4) | -0.63% (3/4) | -15.90% (4/4) | 190.7 | 0/4 |
| 4×2 | v156 | aeglos | 18.275 | +19.13% (0/4) | +18.38% (0/4) | +0.00% (0/4) | 1360.4 | 0/4 |
| 8×2 | catalog | release | 19.340 | +1.24% (3/6) | +0.00% (0/6) | -36.76% (6/6) | 361.3 | 0/6 |
| 8×2 | catalog | ordinary | 19.008 | +0.00% (0/6) | -1.21% (3/6) | -37.37% (6/6) | 358.9 | 3/6 |
| 8×2 | catalog | full | 19.294 | +0.20% (3/6) | +0.34% (2/6) | -37.07% (6/6) | 369.7 | 0/6 |
| 8×2 | catalog | emit | 19.185 | -0.00% (3/6) | -0.75% (4/6) | -36.90% (6/6) | 366.4 | 1/6 |
| 8×2 | catalog | full-emit | 19.093 | +0.09% (3/6) | -1.13% (5/6) | -37.28% (6/6) | 367.3 | 1/6 |
| 8×2 | catalog | task-ordinary | 18.970 | +0.00% (0/6) | -1.44% (6/6) | -37.43% (6/6) | 343.9 | 2/6 |
| 8×2 | catalog | seed | 19.056 | +0.63% (1/6) | -1.00% (4/6) | -37.48% (6/6) | 351.5 | 0/6 |
| 8×2 | catalog | seed-minimum | 19.100 | +0.48% (2/6) | -0.93% (5/6) | -36.96% (6/6) | 362.7 | 2/6 |
| 8×2 | catalog | seed-emit | 18.787 | -0.73% (4/6) | -2.35% (5/6) | -37.98% (6/6) | 368.4 | 4/6 |
| 8×2 | catalog | aeglos | 30.521 | +59.67% (0/6) | +58.13% (0/6) | +0.00% (0/6) | 2495.2 | 0/6 |
| 8×2 | heldout | release | 19.442 | +0.54% (2/6) | +0.00% (0/6) | -37.49% (6/6) | 374.6 | 0/6 |
| 8×2 | heldout | ordinary | 19.352 | +0.00% (0/6) | -0.53% (4/6) | -38.12% (6/6) | 359.0 | 0/6 |
| 8×2 | heldout | full | 19.336 | +0.18% (1/6) | -0.12% (3/6) | -37.46% (6/6) | 353.6 | 0/6 |
| 8×2 | heldout | emit | 19.260 | -0.14% (3/6) | -0.40% (5/6) | -38.36% (6/6) | 365.7 | 0/6 |
| 8×2 | heldout | full-emit | 19.370 | +0.18% (2/6) | -1.05% (4/6) | -38.40% (6/6) | 361.4 | 0/6 |
| 8×2 | heldout | task-ordinary | 19.098 | +0.00% (0/6) | -1.52% (5/6) | -38.62% (6/6) | 365.0 | 0/6 |
| 8×2 | heldout | seed | 19.276 | +0.35% (1/6) | -0.69% (4/6) | -38.01% (6/6) | 365.8 | 0/6 |
| 8×2 | heldout | seed-minimum | 19.139 | +0.28% (3/6) | -1.22% (5/6) | -38.65% (6/6) | 354.6 | 0/6 |
| 8×2 | heldout | seed-emit | 19.173 | +0.28% (3/6) | -1.31% (5/6) | -38.51% (6/6) | 368.0 | 0/6 |
| 8×2 | heldout | aeglos | 31.284 | +61.62% (0/6) | +59.99% (0/6) | +0.00% (0/6) | 2462.0 | 0/6 |
| 8×2 | v153 | release | 19.349 | -0.20% (2/4) | +0.00% (0/4) | -36.60% (4/4) | 379.2 | 0/4 |
| 8×2 | v153 | ordinary | 19.439 | +0.00% (0/4) | +0.20% (2/4) | -36.27% (4/4) | 359.6 | 0/4 |
| 8×2 | v153 | full | 19.416 | -0.12% (2/4) | +0.31% (1/4) | -36.43% (4/4) | 372.5 | 0/4 |
| 8×2 | v153 | emit | 19.352 | -0.41% (2/4) | +0.02% (2/4) | -37.06% (4/4) | 354.8 | 0/4 |
| 8×2 | v153 | full-emit | 19.244 | -1.08% (3/4) | -0.61% (2/4) | -36.50% (4/4) | 362.6 | 0/4 |
| 8×2 | v153 | task-ordinary | 19.241 | +0.00% (0/4) | -0.66% (3/4) | -37.16% (4/4) | 362.7 | 0/4 |
| 8×2 | v153 | seed | 19.193 | -0.25% (3/4) | -1.27% (3/4) | -37.49% (4/4) | 359.8 | 0/4 |
| 8×2 | v153 | seed-minimum | 19.267 | +0.52% (1/4) | -0.21% (3/4) | -36.74% (4/4) | 357.4 | 0/4 |
| 8×2 | v153 | seed-emit | 18.994 | -1.27% (4/4) | -1.27% (4/4) | -37.95% (4/4) | 377.0 | 1/4 |
| 8×2 | v153 | aeglos | 30.456 | +56.91% (0/4) | +57.72% (0/4) | +0.00% (0/4) | 2458.2 | 0/4 |
| 8×2 | v156 | release | 19.442 | +0.29% (2/4) | +0.00% (0/4) | -36.15% (4/4) | 360.0 | 0/4 |
| 8×2 | v156 | ordinary | 19.429 | +0.00% (0/4) | -0.29% (2/4) | -36.31% (4/4) | 358.3 | 0/4 |
| 8×2 | v156 | full | 19.442 | +0.72% (1/4) | +0.43% (1/4) | -36.10% (4/4) | 369.4 | 0/4 |
| 8×2 | v156 | emit | 19.324 | -0.54% (3/4) | -0.55% (3/4) | -36.23% (4/4) | 359.8 | 0/4 |
| 8×2 | v156 | full-emit | 19.308 | -0.46% (3/4) | -0.74% (3/4) | -36.87% (4/4) | 368.7 | 0/4 |
| 8×2 | v156 | task-ordinary | 19.086 | +0.00% (0/4) | -1.83% (4/4) | -37.21% (4/4) | 350.6 | 1/4 |
| 8×2 | v156 | seed | 19.145 | +0.31% (1/4) | -1.09% (2/4) | -36.96% (4/4) | 358.8 | 0/4 |
| 8×2 | v156 | seed-minimum | 19.177 | +0.56% (2/4) | -1.60% (3/4) | -36.80% (4/4) | 342.5 | 0/4 |
| 8×2 | v156 | seed-emit | 19.258 | +0.72% (1/4) | -0.70% (2/4) | -36.70% (4/4) | 378.4 | 0/4 |
| 8×2 | v156 | aeglos | 30.369 | +57.04% (0/4) | +56.63% (0/4) | +0.00% (0/4) | 2495.4 | 0/4 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [SDK source](artifacts/source.patch) · [Seeded solver source](artifacts/ownership-source.patch) · [Verification](artifacts/verification.json)

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
