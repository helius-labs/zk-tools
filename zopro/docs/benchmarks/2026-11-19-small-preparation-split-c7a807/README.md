# Solver, gather and quotient pool placement on C7a

The [Ryzen trial](../2026-11-19-small-preparation-split-ryzen806/README.md) keeps witness solving on the frontend. It moves canonical gather and quotient FFT to the main pool, runs those operations concurrently there, or overlaps frontend gather with the main-pool FFT. The release path uses the existing frontend preparation. No proving-key data is retained by these modes. The ordinary preparation route handles denied optional memory.

One c7a.24xlarge compares the three placements against the released SDK, the same-build ordinary path, and prepared Aeglos. Zopro uses 80 main and 16 frontend cores for 1×2 and 64 and 32 for 2×2. All Zopro 2×2 arms use medium K/H fusion. Each process primes three calls and times thirteen. Five matched blocks per input include warm Aeglos with all 96 cores. All 136 exports independently verified against the original keys. Routing checks confirmed each placement and zero-memory fallback. The instance and temporary storage were removed.

- 1×2 `parallel` took 8.363 / 8.343 ms across original and held-out, paired +6.94% / +6.45% from the same-build ordinary arm.
- 2×2 `split` took 12.153 / 12.163 ms across original and held-out, paired +0.91% / +0.17% from the same-build ordinary arm.

None of the layouts improved both primary inputs consistently against both controls. Selected profiles remain experimental.

| Shape | Input | Mode | Proof ms | Paired vs ordinary | Paired vs release | Paired vs Aeglos | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|
| 1×2 | catalog | release | 7.834 | +0.27% (2/5) | +0.00% (0/5) | -3.04% (5/5) | 42.3 |
| 1×2 | catalog | ordinary | 7.810 | +0.00% (0/5) | -0.27% (3/5) | -2.60% (5/5) | 42.2 |
| 1×2 | catalog | main | 8.631 | +10.48% (0/5) | +9.54% (0/5) | +6.76% (0/5) | 39.3 |
| 1×2 | catalog | parallel | 8.363 | +6.94% (0/5) | +6.78% (0/5) | +3.31% (0/5) | 38.5 |
| 1×2 | catalog | split | 8.457 | +8.02% (0/5) | +7.95% (0/5) | +4.67% (0/5) | 37.6 |
| 1×2 | catalog | aeglos | 8.080 | +2.67% (0/5) | +3.14% (0/5) | +0.00% (0/5) | 459.2 |
| 1×2 | heldout | release | 7.804 | +0.31% (2/5) | +0.00% (0/5) | -3.06% (5/5) | 39.4 |
| 1×2 | heldout | ordinary | 7.817 | +0.00% (0/5) | -0.31% (3/5) | -3.04% (3/5) | 41.5 |
| 1×2 | heldout | main | 8.549 | +9.37% (1/5) | +9.55% (0/5) | +4.80% (0/5) | 40.8 |
| 1×2 | heldout | parallel | 8.343 | +6.45% (1/5) | +6.71% (0/5) | +2.87% (0/5) | 36.2 |
| 1×2 | heldout | split | 8.463 | +8.39% (1/5) | +8.48% (0/5) | +3.74% (0/5) | 36.2 |
| 1×2 | heldout | aeglos | 8.110 | +3.14% (2/5) | +3.15% (0/5) | +0.00% (0/5) | 457.4 |
| 2×2 | catalog | release | 11.918 | -1.26% (5/5) | +0.00% (0/5) | +1.38% (1/5) | 72.9 |
| 2×2 | catalog | ordinary | 12.065 | +0.00% (0/5) | +1.27% (0/5) | +2.43% (0/5) | 72.2 |
| 2×2 | catalog | main | 12.448 | +2.35% (0/5) | +4.00% (0/5) | +4.43% (0/5) | 94.7 |
| 2×2 | catalog | parallel | 12.327 | +2.30% (0/5) | +3.60% (0/5) | +4.89% (0/5) | 95.4 |
| 2×2 | catalog | split | 12.153 | +0.91% (2/5) | +1.40% (0/5) | +4.40% (1/5) | 112.8 |
| 2×2 | catalog | aeglos | 11.752 | -2.38% (5/5) | -1.36% (4/5) | +0.00% (0/5) | 816.3 |
| 2×2 | heldout | release | 12.068 | -0.90% (4/5) | +0.00% (0/5) | +4.13% (0/5) | 79.1 |
| 2×2 | heldout | ordinary | 12.204 | +0.00% (0/5) | +0.91% (1/5) | +5.04% (0/5) | 76.0 |
| 2×2 | heldout | main | 12.356 | +1.24% (1/5) | +2.36% (0/5) | +7.00% (0/5) | 100.8 |
| 2×2 | heldout | parallel | 12.632 | +4.68% (1/5) | +4.06% (0/5) | +8.36% (0/5) | 93.3 |
| 2×2 | heldout | split | 12.163 | +0.17% (2/5) | +1.09% (0/5) | +5.22% (0/5) | 97.6 |
| 2×2 | heldout | aeglos | 11.547 | -4.80% (5/5) | -3.96% (5/5) | +0.00% (0/5) | 803.0 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU.

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 805 released SDK 80+16 | 7.858 | 37 / 24 | 4.71× / 3.05× | +4.158 |
| 2×2 | 805 released SDK medium K/H 64+32 | 12.029 | 59 / 30 | 4.90× / 2.49× | +6.129 |
| 4×2 | 757 solve-front-on | 15.257 | 109 / 48 | 7.14× / 3.15× | +4.357 |
| 8×2 | 781 recognized B2 origins replay | 20.098 | 189 / 74 | 9.40× / 3.68× | +1.198 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
