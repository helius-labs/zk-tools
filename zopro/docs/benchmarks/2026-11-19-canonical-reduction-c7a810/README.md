# Reduction-only Montgomery scalar conversion on C7a

The [Ryzen decoder trial](../2026-11-19-canonical-reduction-ryzen809/README.md) specializes conversion out of Montgomery form to the four necessary cancellation rounds. It uses the existing representations and final canonical correction. The benchmark enables it only in canonical gather and the MSM input conversion. The release and same-build ordinary controls use the full native multiply.

One c7a.24xlarge runs five matched blocks on each input. 1×2 uses 80 main and 16 frontend physical cores. Other Zopro shapes use 64 and 32. 2×2 uses medium K/H and 8×2 uses the existing origin and serial B2 options. Prepared Aeglos uses 96 cores. Every process primes three calls and times thirteen. All 176 exports independently verified against the original keys, including the zero optional-memory cases. The instance and temporary storage were removed.

- 1×2 `reduced` took 7.853 / 7.819 ms across original and held-out, paired -0.04% / +0.34% from the same-build ordinary arm.
- 2×2 `reduced` took 11.997 / 12.030 ms across original and held-out, paired -0.85% / -0.60% from the same-build ordinary arm.
- 4×2 `reduced` took 15.289 / 15.340 ms across original and held-out, paired -1.51% / -1.28% from the same-build ordinary arm.
- 8×2 `reduced` took 19.984 / 20.534 ms across original and held-out, paired -1.32% / +1.39% from the same-build ordinary arm.

The reduction-only decoder did not improve both primary inputs consistently against both controls. The source is experimental.

| Shape | Input | Mode | Proof ms | Paired vs ordinary | Paired vs release | Paired vs Aeglos | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|
| 1×2 | catalog | release | 7.816 | -0.07% (3/5) | +0.00% (0/5) | -4.66% (5/5) | 43.8 |
| 1×2 | catalog | ordinary | 7.837 | +0.00% (0/5) | +0.07% (2/5) | -4.05% (5/5) | 40.8 |
| 1×2 | catalog | reduced | 7.853 | -0.04% (3/5) | +0.03% (2/5) | -3.93% (5/5) | 42.2 |
| 1×2 | catalog | aeglos | 8.174 | +4.22% (0/5) | +4.89% (0/5) | +0.00% (0/5) | 461.8 |
| 1×2 | heldout | release | 7.768 | -0.33% (4/5) | +0.00% (0/5) | -3.12% (5/5) | 40.0 |
| 1×2 | heldout | ordinary | 7.813 | +0.00% (0/5) | +0.33% (1/5) | -2.81% (5/5) | 40.8 |
| 1×2 | heldout | reduced | 7.819 | +0.34% (2/5) | +0.46% (1/5) | -2.56% (5/5) | 39.2 |
| 1×2 | heldout | aeglos | 8.047 | +2.89% (0/5) | +3.22% (0/5) | +0.00% (0/5) | 455.3 |
| 2×2 | catalog | release | 12.056 | -0.32% (4/5) | +0.00% (0/5) | +4.15% (1/5) | 72.2 |
| 2×2 | catalog | ordinary | 12.084 | +0.00% (0/5) | +0.32% (1/5) | +4.88% (1/5) | 75.3 |
| 2×2 | catalog | reduced | 11.997 | -0.85% (4/5) | -0.78% (3/5) | +4.66% (1/5) | 71.5 |
| 2×2 | catalog | aeglos | 11.565 | -4.65% (4/5) | -3.99% (4/5) | +0.00% (0/5) | 811.4 |
| 2×2 | heldout | release | 12.045 | +0.15% (2/5) | +0.00% (0/5) | +4.96% (0/5) | 69.2 |
| 2×2 | heldout | ordinary | 12.101 | +0.00% (0/5) | -0.15% (3/5) | +3.75% (0/5) | 73.0 |
| 2×2 | heldout | reduced | 12.030 | -0.60% (3/5) | -0.64% (3/5) | +3.28% (0/5) | 76.6 |
| 2×2 | heldout | aeglos | 11.659 | -3.62% (5/5) | -4.72% (5/5) | +0.00% (0/5) | 811.3 |
| 4×2 | catalog | release | 15.301 | -0.63% (4/5) | +0.00% (0/5) | -15.03% (5/5) | 206.6 |
| 4×2 | catalog | ordinary | 15.523 | +0.00% (0/5) | +0.63% (1/5) | -13.55% (5/5) | 207.8 |
| 4×2 | catalog | reduced | 15.289 | -1.51% (4/5) | -0.55% (4/5) | -15.12% (5/5) | 202.7 |
| 4×2 | catalog | aeglos | 18.000 | +15.67% (0/5) | +17.69% (0/5) | +0.00% (0/5) | 1349.7 |
| 4×2 | heldout | release | 15.318 | -1.63% (4/5) | +0.00% (0/5) | -15.87% (5/5) | 205.0 |
| 4×2 | heldout | ordinary | 15.703 | +0.00% (0/5) | +1.66% (1/5) | -14.77% (5/5) | 215.4 |
| 4×2 | heldout | reduced | 15.340 | -1.28% (4/5) | +0.09% (2/5) | -16.18% (5/5) | 200.4 |
| 4×2 | heldout | aeglos | 18.263 | +17.33% (0/5) | +18.86% (0/5) | +0.00% (0/5) | 1347.2 |
| 8×2 | catalog | release | 20.231 | +0.65% (2/5) | +0.00% (0/5) | -33.59% (5/5) | 404.1 |
| 8×2 | catalog | ordinary | 20.144 | +0.00% (0/5) | -0.65% (3/5) | -34.25% (5/5) | 412.9 |
| 8×2 | catalog | reduced | 19.984 | -1.32% (3/5) | -1.22% (3/5) | -34.86% (5/5) | 403.3 |
| 8×2 | catalog | aeglos | 30.557 | +52.10% (0/5) | +50.58% (0/5) | +0.00% (0/5) | 2492.2 |
| 8×2 | heldout | release | 20.447 | +0.68% (2/5) | +0.00% (0/5) | -32.75% (5/5) | 405.5 |
| 8×2 | heldout | ordinary | 20.287 | +0.00% (0/5) | -0.67% (3/5) | -33.16% (5/5) | 410.7 |
| 8×2 | heldout | reduced | 20.534 | +1.39% (2/5) | -0.51% (3/5) | -32.23% (5/5) | 408.2 |
| 8×2 | heldout | aeglos | 30.404 | +49.62% (0/5) | +48.70% (0/5) | +0.00% (0/5) | 2511.0 |

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
