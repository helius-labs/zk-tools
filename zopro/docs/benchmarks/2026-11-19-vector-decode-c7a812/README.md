# Eight-lane scalar decoding on C7a

The [Ryzen trial](../2026-11-19-vector-decode-ryzen811/README.md) compares the original scalar multiplier, standalone scalar reduction, and decoding eight lanes per AVX-512 IFMA operation. The same modes run on C7a. They use existing proof buffers during canonical gather and the in-place conversion for curve queries. The zero optional-memory cases verify with the solver's ordinary fallback where needed.

Five matched blocks per input run on a c7a.24xlarge. 1×2 uses 80 main and 16 frontend physical cores. The other Zopro shapes use 64 and 32. 2×2 uses the existing medium K/H option. 4×2 uses the frontend solver. 8×2 uses the existing origin and serial B2 options. Prepared Aeglos uses all 96 cores. Every process primes three calls and measures thirteen. All 216 exported proofs independently verified against the original keys. The instance and temporary storage were removed.

- 1×2 `reduced` took 7.776 / 7.789 ms across original and held-out, paired -0.90% / -0.05% from the same-build ordinary arm.
- 2×2 `vector` took 11.968 / 12.071 ms across original and held-out, paired -0.60% / -0.07% from the same-build ordinary arm.
- 4×2 `reduced` took 15.377 / 15.420 ms across original and held-out, paired +0.90% / -1.20% from the same-build ordinary arm.
- 8×2 `reduced` took 20.326 / 20.397 ms across original and held-out, paired +0.30% / -0.53% from the same-build ordinary arm.

The decoders did not improve both primary inputs consistently against both controls. The decoder source remains experimental.

| Shape | Input | Mode | Proof ms | Paired vs ordinary | Paired vs release | Paired vs scalar reduction | Paired vs Aeglos | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | release | 7.766 | -1.03% (5/5) | +0.00% (0/5) | -0.13% (3/5) | -4.47% (4/5) | 41.5 |
| 1×2 | catalog | ordinary | 7.861 | +0.00% (0/5) | +1.04% (0/5) | +0.91% (1/5) | -3.19% (4/5) | 43.0 |
| 1×2 | catalog | reduced | 7.776 | -0.90% (4/5) | +0.13% (2/5) | +0.00% (0/5) | -4.84% (3/5) | 40.8 |
| 1×2 | catalog | vector | 7.808 | -0.67% (4/5) | +0.50% (1/5) | +0.36% (2/5) | -3.84% (3/5) | 43.1 |
| 1×2 | catalog | aeglos | 8.120 | +3.30% (1/5) | +4.67% (1/5) | +5.09% (2/5) | +0.00% (0/5) | 460.7 |
| 1×2 | heldout | release | 7.816 | +0.45% (1/5) | +0.00% (0/5) | +0.29% (2/5) | -4.23% (5/5) | 43.1 |
| 1×2 | heldout | ordinary | 7.774 | +0.00% (0/5) | -0.45% (4/5) | +0.05% (2/5) | -4.62% (5/5) | 42.2 |
| 1×2 | heldout | reduced | 7.789 | -0.05% (3/5) | -0.29% (3/5) | +0.00% (0/5) | -4.67% (5/5) | 40.1 |
| 1×2 | heldout | vector | 7.787 | +0.07% (2/5) | -0.38% (4/5) | -0.02% (3/5) | -4.70% (5/5) | 39.3 |
| 1×2 | heldout | aeglos | 8.171 | +4.85% (0/5) | +4.42% (0/5) | +4.90% (0/5) | +0.00% (0/5) | 462.3 |
| 2×2 | catalog | release | 12.018 | -0.30% (4/5) | +0.00% (0/5) | +0.73% (1/5) | +3.04% (1/5) | 74.5 |
| 2×2 | catalog | ordinary | 12.036 | +0.00% (0/5) | +0.30% (1/5) | +1.04% (2/5) | +3.21% (0/5) | 71.5 |
| 2×2 | catalog | reduced | 12.040 | -1.02% (3/5) | -0.73% (4/5) | +0.00% (0/5) | +4.47% (1/5) | 73.8 |
| 2×2 | catalog | vector | 11.968 | -0.60% (5/5) | -0.28% (4/5) | -0.61% (3/5) | +2.76% (2/5) | 76.0 |
| 2×2 | catalog | aeglos | 11.544 | -3.11% (5/5) | -2.95% (4/5) | -4.28% (4/5) | +0.00% (0/5) | 806.9 |
| 2×2 | heldout | release | 12.017 | +0.63% (1/5) | +0.00% (0/5) | -0.41% (4/5) | +1.66% (1/5) | 74.5 |
| 2×2 | heldout | ordinary | 12.028 | +0.00% (0/5) | -0.62% (4/5) | -0.75% (4/5) | +3.56% (1/5) | 74.4 |
| 2×2 | heldout | reduced | 12.126 | +0.76% (1/5) | +0.42% (1/5) | +0.00% (0/5) | +2.44% (1/5) | 75.3 |
| 2×2 | heldout | vector | 12.071 | -0.07% (3/5) | -0.19% (3/5) | -0.40% (4/5) | +2.88% (1/5) | 72.3 |
| 2×2 | heldout | aeglos | 11.672 | -3.44% (4/5) | -1.64% (4/5) | -2.39% (4/5) | +0.00% (0/5) | 811.6 |
| 4×2 | catalog | release | 15.288 | +0.81% (2/5) | +0.00% (0/5) | -1.48% (4/5) | -15.95% (5/5) | 211.8 |
| 4×2 | catalog | ordinary | 15.320 | +0.00% (0/5) | -0.80% (3/5) | -0.89% (4/5) | -15.49% (5/5) | 216.9 |
| 4×2 | catalog | reduced | 15.377 | +0.90% (1/5) | +1.51% (1/5) | +0.00% (0/5) | -14.78% (5/5) | 205.0 |
| 4×2 | catalog | vector | 15.382 | +0.79% (1/5) | +0.82% (1/5) | -0.52% (4/5) | -15.12% (5/5) | 206.4 |
| 4×2 | catalog | aeglos | 18.088 | +18.33% (0/5) | +18.97% (0/5) | +17.34% (0/5) | +0.00% (0/5) | 1349.1 |
| 4×2 | heldout | release | 15.382 | -0.75% (3/5) | +0.00% (0/5) | -0.26% (3/5) | -14.31% (5/5) | 207.3 |
| 4×2 | heldout | ordinary | 15.498 | +0.00% (0/5) | +0.75% (2/5) | +1.21% (2/5) | -14.14% (5/5) | 200.4 |
| 4×2 | heldout | reduced | 15.420 | -1.20% (3/5) | +0.26% (2/5) | +0.00% (0/5) | -14.51% (5/5) | 199.6 |
| 4×2 | heldout | vector | 15.456 | -0.77% (4/5) | -0.08% (3/5) | +0.24% (1/5) | -14.38% (5/5) | 204.9 |
| 4×2 | heldout | aeglos | 18.051 | +16.47% (0/5) | +16.69% (0/5) | +16.98% (0/5) | +0.00% (0/5) | 1343.0 |
| 8×2 | catalog | release | 20.197 | -0.33% (3/5) | +0.00% (0/5) | -0.85% (5/5) | -33.42% (5/5) | 409.5 |
| 8×2 | catalog | ordinary | 20.291 | +0.00% (0/5) | +0.33% (2/5) | -0.30% (3/5) | -33.43% (5/5) | 413.7 |
| 8×2 | catalog | reduced | 20.326 | +0.30% (2/5) | +0.86% (0/5) | +0.00% (0/5) | -32.84% (5/5) | 424.1 |
| 8×2 | catalog | vector | 20.181 | -0.34% (4/5) | +0.32% (2/5) | -0.86% (4/5) | -33.72% (5/5) | 398.9 |
| 8×2 | catalog | aeglos | 30.390 | +50.22% (0/5) | +50.19% (0/5) | +48.90% (0/5) | +0.00% (0/5) | 2448.8 |
| 8×2 | heldout | release | 20.308 | -0.85% (4/5) | +0.00% (0/5) | +0.56% (2/5) | -33.62% (5/5) | 409.3 |
| 8×2 | heldout | ordinary | 20.471 | +0.00% (0/5) | +0.85% (1/5) | +0.54% (2/5) | -33.05% (5/5) | 407.0 |
| 8×2 | heldout | reduced | 20.397 | -0.53% (3/5) | -0.56% (3/5) | +0.00% (0/5) | -32.90% (5/5) | 401.7 |
| 8×2 | heldout | vector | 20.433 | +0.34% (2/5) | +0.61% (1/5) | -0.00% (3/5) | -32.98% (5/5) | 413.0 |
| 8×2 | heldout | aeglos | 30.563 | +49.37% (0/5) | +50.65% (0/5) | +49.04% (0/5) | +0.00% (0/5) | 2463.8 |

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
