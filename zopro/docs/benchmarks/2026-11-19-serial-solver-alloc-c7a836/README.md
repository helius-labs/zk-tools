# Allocation for the certified solver on C7a

The [Ryzen trial](../2026-11-19-serial-solver-alloc-ryzen835/README.md) tested sequential allocation after the solver establishes definite assignment. With those buffers, allocating reserves capacity and the instruction workers write the field entries. A second setting moves the complete solver onto the frontend pool. The controls retain the existing allocation joins and run the instructions on the frontend. All arms use four solver epochs, packed work, recognized B2 origins and serial B2 conversion.

Zopro uses 72 main and 24 frontend cores. Prepared Aeglos uses all 96 cores. There are six blocks per primary input and four per additional input. Each process primes three calls and times thirteen. All 164 exported proofs independently verified, including initialized fallback at zero optional workspace. The temporary instance and storage were removed.

- 8×2 `serial` took 19.053 / 19.200 ms across original and held-out, paired -0.06% / -0.30% from the same-build ordinary arm.

No initialization mode beat both controls consistently across its tested inputs.

| Shape | Input | Mode | Proof ms | Paired vs ordinary | Paired vs release | Paired vs Aeglos | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|
| 8×2 | catalog | release | 19.204 | +0.61% (2/6) | +0.00% (0/6) | -36.91% (6/6) | 375.5 |
| 8×2 | catalog | ordinary | 19.087 | +0.00% (0/6) | -0.61% (4/6) | -37.63% (6/6) | 361.4 |
| 8×2 | catalog | serial | 19.053 | -0.06% (3/6) | -0.67% (4/6) | -37.45% (6/6) | 352.2 |
| 8×2 | catalog | full | 19.080 | +0.13% (3/6) | -0.97% (5/6) | -37.59% (6/6) | 359.8 |
| 8×2 | catalog | full-serial | 19.062 | -0.07% (3/6) | -0.62% (5/6) | -37.36% (6/6) | 337.4 |
| 8×2 | catalog | aeglos | 30.291 | +60.33% (0/6) | +58.49% (0/6) | +0.00% (0/6) | 2463.9 |
| 8×2 | catalog | ordinary-clock | 19.260 | +0.90% (2/6) | +0.23% (2/6) | -36.60% (6/6) | 363.4 |
| 8×2 | catalog | serial-clock | 19.000 | +0.08% (3/6) | -0.78% (5/6) | -37.48% (6/6) | 360.4 |
| 8×2 | catalog | full-serial-clock | 19.177 | +0.39% (3/6) | -0.37% (3/6) | -36.87% (6/6) | 343.9 |
| 8×2 | heldout | release | 19.388 | +1.28% (1/6) | +0.00% (0/6) | -37.19% (6/6) | 365.8 |
| 8×2 | heldout | ordinary | 19.167 | +0.00% (0/6) | -1.26% (5/6) | -37.66% (6/6) | 381.7 |
| 8×2 | heldout | serial | 19.200 | -0.30% (4/6) | -0.77% (4/6) | -37.56% (6/6) | 359.8 |
| 8×2 | heldout | full | 19.328 | +1.09% (2/6) | +0.37% (2/6) | -37.16% (6/6) | 360.5 |
| 8×2 | heldout | full-serial | 19.190 | -0.21% (3/6) | -1.26% (6/6) | -37.97% (6/6) | 341.8 |
| 8×2 | heldout | aeglos | 30.743 | +60.42% (0/6) | +59.22% (0/6) | +0.00% (0/6) | 2492.0 |
| 8×2 | heldout | ordinary-clock | 19.204 | +0.36% (3/6) | -0.91% (6/6) | -37.44% (6/6) | 361.9 |
| 8×2 | heldout | serial-clock | 19.551 | +2.60% (0/6) | +1.38% (2/6) | -36.00% (6/6) | 372.7 |
| 8×2 | heldout | full-serial-clock | 19.616 | +2.37% (0/6) | +1.21% (2/6) | -36.41% (6/6) | 347.2 |
| 8×2 | v153 | release | 19.497 | +2.07% (0/4) | +0.00% (0/4) | -35.74% (4/4) | 371.1 |
| 8×2 | v153 | ordinary | 19.101 | +0.00% (0/4) | -2.03% (4/4) | -37.15% (4/4) | 361.7 |
| 8×2 | v153 | serial | 19.289 | +0.73% (1/4) | -1.32% (4/4) | -36.67% (4/4) | 354.6 |
| 8×2 | v153 | full | 19.190 | +0.57% (1/4) | -1.80% (4/4) | -37.03% (4/4) | 367.9 |
| 8×2 | v153 | full-serial | 19.176 | +0.61% (1/4) | -1.54% (4/4) | -37.14% (4/4) | 337.3 |
| 8×2 | v153 | aeglos | 30.445 | +59.10% (0/4) | +55.63% (0/4) | +0.00% (0/4) | 2462.4 |
| 8×2 | v156 | release | 19.504 | +1.43% (1/4) | +0.00% (0/4) | -35.72% (4/4) | 363.4 |
| 8×2 | v156 | ordinary | 19.229 | +0.00% (0/4) | -1.41% (3/4) | -36.64% (4/4) | 370.2 |
| 8×2 | v156 | serial | 18.960 | -0.98% (3/4) | -2.78% (4/4) | -37.51% (4/4) | 362.7 |
| 8×2 | v156 | full | 19.061 | -0.95% (4/4) | -2.27% (4/4) | -37.32% (4/4) | 357.5 |
| 8×2 | v156 | full-serial | 19.315 | +0.49% (1/4) | -0.98% (4/4) | -36.35% (4/4) | 358.1 |
| 8×2 | v156 | aeglos | 30.342 | +57.84% (0/4) | +55.57% (0/4) | +0.00% (0/4) | 2456.0 |

## Clocked stage times

| Shape | Input | Schedule | Proof ms | Paired clock effect | Solve ms | Gather ms | Quotient ms | Curve interval ms |
|---|---|---|---:|---:|---:|---:|---:|---:|
| 8×2 | catalog | ordinary | 19.260 | +0.90% | 3.143 | 1.030 | 2.909 | 10.109 |
| 8×2 | catalog | serial | 19.000 | -0.06% | 3.151 | 1.045 | 2.852 | 9.989 |
| 8×2 | catalog | full-serial | 19.177 | +0.26% | 3.218 | 1.026 | 2.884 | 10.010 |
| 8×2 | heldout | ordinary | 19.204 | +0.36% | 3.081 | 1.018 | 2.876 | 10.178 |
| 8×2 | heldout | serial | 19.551 | +2.23% | 3.132 | 1.009 | 2.903 | 10.356 |
| 8×2 | heldout | full-serial | 19.616 | +2.61% | 3.220 | 1.017 | 2.911 | 10.393 |

[Samples and clocks](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)

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
