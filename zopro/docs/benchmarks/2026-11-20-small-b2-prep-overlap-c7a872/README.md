# Staging 2×2 B2 preparation during the quotient on C7a

The staged modes use the usual main and frontend pools. The quotient FFT stays on the frontend. Early placement overlaps B2 grouping and scalar decomposition with the FFT while keeping bucket processing in the usual curve phase. The serial option performs only the scalar decomposition serially. The staged wait control runs the same grouping before the curves and after the FFT. The ordinary and release controls retain the original schedule. Zopro uses 64 main and 32 frontend physical cores. Prepared Aeglos uses all 96 physical cores. [Ryzen comparison](../2026-11-20-small-b2-prep-overlap-ryzen871/README.md)

Eight counterbalanced process blocks per input each prime three proofs and time thirteen. An early profile advances with a lower paired median and at least six wins per input against all three Zopro controls. All 104 exports independently verified. Separate probes checked phase overlap and ordinary fallback under zero optional workspace. The temporary instance and storage were removed.

The fastest early profile on the original request was early at 15.664 ms. It took 15.730 ms on the held-out request. Neither early mode consistently improved the ordinary, staged wait and released controls on both inputs.

| Input | Profile | Proof ms | Paired vs ordinary | Paired vs wait | Paired vs release | Paired vs Aeglos | Peak MiB | Blocks at or below 5.9 ms |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | release | 12.234 | -0.12% (4/8) | -28.97% (8/8) | +0.00% (0/8) | +2.25% (0/8) | 74.6 | 0/8 |
| catalog | ordinary | 12.248 | +0.00% (0/8) | -28.74% (8/8) | +0.12% (4/8) | +2.15% (0/8) | 76.7 | 0/8 |
| catalog | wait | 17.225 | +40.33% (0/8) | +0.00% (0/8) | +40.80% (0/8) | +44.73% (0/8) | 111.2 | 0/8 |
| catalog | early | 15.664 | +27.85% (0/8) | -9.75% (8/8) | +27.92% (0/8) | +30.38% (0/8) | 116.5 | 0/8 |
| catalog | early-serial | 15.749 | +28.69% (0/8) | -9.14% (8/8) | +29.04% (0/8) | +32.68% (0/8) | 112.8 | 0/8 |
| catalog | aeglos | 11.935 | -2.10% (8/8) | -30.90% (8/8) | -2.20% (8/8) | +0.00% (0/8) | 811.5 | 0/8 |
| heldout | release | 12.332 | +0.66% (3/8) | -27.88% (8/8) | +0.00% (0/8) | +2.67% (1/8) | 76.0 | 0/8 |
| heldout | ordinary | 12.269 | +0.00% (0/8) | -29.01% (8/8) | -0.65% (5/8) | +1.24% (2/8) | 74.5 | 0/8 |
| heldout | wait | 17.204 | +40.88% (0/8) | +0.00% (0/8) | +38.66% (0/8) | +41.77% (0/8) | 114.2 | 0/8 |
| heldout | early | 15.730 | +27.37% (0/8) | -10.59% (8/8) | +26.98% (0/8) | +28.50% (0/8) | 112.7 | 0/8 |
| heldout | early-serial | 15.805 | +28.85% (0/8) | -9.00% (8/8) | +27.95% (0/8) | +30.15% (0/8) | 115.0 | 0/8 |
| heldout | aeglos | 12.109 | -1.22% (6/8) | -29.46% (8/8) | -2.60% (7/8) | +0.00% (0/8) | 803.8 | 0/8 |

The separately clocked phases include scheduling within each closure. Times are medians of four calls per probe, including the three primes.

| Input | Probe | FFT ms | B2 preparation ms | Overlap ms |
|---|---|---:|---:|---:|
| catalog | wait | 1.934 | 3.118 | 0.000 |
| catalog | early | 3.298 | 3.290 | 3.288 |
| catalog | early-serial | 3.216 | 3.203 | 3.203 |
| heldout | wait | 2.074 | 3.115 | 0.000 |
| heldout | early | 2.656 | 3.070 | 2.643 |
| heldout | early-serial | 3.408 | 3.446 | 3.394 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Routes](artifacts/routes.json) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**5/8** original requests reach 10× M5 CPU on the recorded profiles.

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 805 released SDK 80+16 | 7.858 | 37 / 24 | 4.71× / 3.05× | +4.158 |
| 2×2 | 805 released SDK medium K/H 64+32 | 12.029 | 59 / 30 | 4.90× / 2.49× | +6.129 |
| 4×2 | 854 seed4-emit | 14.726 | 109 / 48 | 7.40× / 3.26× | +3.826 |
| 8×2 | 852 SDK seeded tasks and B2 vector conversion | 18.849 | 189 / 74 | 10.03× / 3.93× | -0.051 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
