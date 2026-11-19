# Solver epochs and core placement for 8×2 on C7a

The [buffer trial](../2026-11-19-folded-solver-init-c7a828/README.md) measured 19.05 ms on the original request with the certified uninitialized solver enabled. This confirmation uses the released SDK with the established 72 main and 24 frontend cores. The base arm uses initialized buffers. The uninit arm enables the existing definite-assignment check for skipping initialization. The epoch variants change only the number of solver dependency waves configured by the same option. The affinity logs showed that the two alternative frontend CPU lists overlapped the main pool, which remained pinned to CPUs 0–71. Their 40 exports verify, and their timings are retained in the table as diagnostics. The placement comparisons exclude those two arms. All Zopro arms keep packed solver work, frontend K, recognized B2 origins and serial B2 conversion.

The two primary inputs have six matched blocks and two additional inputs have four. Each process primes three calls and times thirteen. Prepared Aeglos uses all 96 cores on the same host. All 180 exported proofs independently verified. The temporary instance and storage were removed.

The lowest eligible original-request median was 19.105 ms with `epochs-4`. No candidate beat its controls consistently across the four inputs.

| Input | Mode | Proof ms | Paired vs initialized base | Paired vs ordinary uninit | Paired vs Aeglos | Peak MiB | Blocks at or below 18.9 ms |
|---|---|---:|---:|---:|---:|---:|---:|
| catalog | base | 19.626 | +0.00% (0/6) | +0.53% (2/6) | -35.83% (6/6) | 376.3 | 0/6 |
| catalog | uninit | 19.428 | -0.53% (4/6) | +0.00% (0/6) | -36.13% (6/6) | 357.5 | 0/6 |
| catalog | epochs-4 | 19.105 | -2.25% (5/6) | -1.45% (5/6) | -37.34% (6/6) | 364.3 | 0/6 |
| catalog | epochs-6 | 19.565 | -0.35% (4/6) | +0.27% (3/6) | -36.17% (6/6) | 369.4 | 0/6 |
| catalog | epochs-10 | 19.741 | +0.48% (1/6) | +1.74% (1/6) | -35.34% (6/6) | 367.4 | 0/6 |
| catalog | epochs-12 | 20.204 | +2.69% (0/6) | +3.70% (1/6) | -33.82% (6/6) | 367.9 | 0/6 |
| catalog | front-middle | 22.027 | +12.39% (0/6) | +12.67% (0/6) | -27.97% (6/6) | 365.2 | 0/6 |
| catalog | front-spread | 22.148 | +12.66% (0/6) | +12.38% (0/6) | -28.08% (6/6) | 364.2 | 0/6 |
| catalog | aeglos | 30.484 | +55.84% (0/6) | +56.57% (0/6) | +0.00% (0/6) | 2463.7 | 0/6 |
| heldout | base | 19.795 | +0.00% (0/6) | +2.16% (0/6) | -35.66% (6/6) | 376.3 | 0/6 |
| heldout | uninit | 19.353 | -2.12% (6/6) | +0.00% (0/6) | -37.02% (6/6) | 364.2 | 0/6 |
| heldout | epochs-4 | 19.216 | -3.12% (6/6) | -0.86% (4/6) | -37.29% (6/6) | 367.3 | 0/6 |
| heldout | epochs-6 | 19.710 | -0.53% (4/6) | +1.62% (0/6) | -36.05% (6/6) | 368.1 | 0/6 |
| heldout | epochs-10 | 19.849 | +0.30% (2/6) | +2.35% (0/6) | -35.26% (6/6) | 363.2 | 0/6 |
| heldout | epochs-12 | 20.441 | +3.04% (0/6) | +5.27% (0/6) | -33.58% (6/6) | 363.7 | 0/6 |
| heldout | front-middle | 22.336 | +12.05% (0/6) | +15.03% (0/6) | -28.22% (6/6) | 395.7 | 0/6 |
| heldout | front-spread | 22.394 | +12.71% (0/6) | +15.11% (0/6) | -27.24% (6/6) | 366.4 | 0/6 |
| heldout | aeglos | 30.767 | +55.42% (0/6) | +58.78% (0/6) | +0.00% (0/6) | 2499.9 | 0/6 |
| v153 | base | 19.865 | +0.00% (0/4) | +1.17% (0/4) | -34.57% (4/4) | 392.1 | 0/4 |
| v153 | uninit | 19.688 | -1.16% (4/4) | +0.00% (0/4) | -35.48% (4/4) | 374.8 | 0/4 |
| v153 | epochs-4 | 19.284 | -2.69% (4/4) | -1.08% (4/4) | -36.24% (4/4) | 359.0 | 0/4 |
| v153 | epochs-6 | 19.936 | +0.02% (2/4) | +1.19% (0/4) | -34.44% (4/4) | 365.9 | 0/4 |
| v153 | epochs-10 | 19.871 | +0.03% (2/4) | +1.38% (0/4) | -34.53% (4/4) | 369.0 | 0/4 |
| v153 | epochs-12 | 20.641 | +3.58% (0/4) | +5.43% (0/4) | -32.26% (4/4) | 378.6 | 0/4 |
| v153 | front-middle | 22.345 | +12.41% (0/4) | +14.56% (0/4) | -26.22% (4/4) | 355.5 | 0/4 |
| v153 | front-spread | 22.259 | +11.43% (0/4) | +13.47% (0/4) | -27.16% (4/4) | 365.7 | 0/4 |
| v153 | aeglos | 30.359 | +52.82% (0/4) | +54.99% (0/4) | +0.00% (0/4) | 2459.6 | 0/4 |
| v156 | base | 19.897 | +0.00% (0/4) | +0.67% (2/4) | -34.87% (4/4) | 386.7 | 0/4 |
| v156 | uninit | 19.684 | -0.64% (2/4) | +0.00% (0/4) | -35.22% (4/4) | 362.7 | 0/4 |
| v156 | epochs-4 | 19.398 | -2.75% (4/4) | -1.39% (4/4) | -36.39% (4/4) | 370.2 | 0/4 |
| v156 | epochs-6 | 19.715 | -1.32% (3/4) | +0.51% (2/4) | -35.67% (4/4) | 360.4 | 0/4 |
| v156 | epochs-10 | 19.999 | +0.90% (1/4) | +1.61% (1/4) | -34.28% (4/4) | 380.0 | 0/4 |
| v156 | epochs-12 | 20.436 | +2.99% (0/4) | +3.39% (1/4) | -33.27% (4/4) | 363.5 | 0/4 |
| v156 | front-middle | 22.180 | +11.74% (0/4) | +12.48% (0/4) | -27.01% (4/4) | 366.8 | 0/4 |
| v156 | front-spread | 22.301 | +12.56% (0/4) | +10.65% (0/4) | -27.11% (4/4) | 361.9 | 0/4 |
| v156 | aeglos | 30.618 | +53.53% (0/4) | +54.37% (0/4) | +0.00% (0/4) | 2466.6 | 0/4 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)

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
