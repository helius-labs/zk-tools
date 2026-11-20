# Four-range release control for 1×2 B2 on C7a

The [initial C7a run](../2026-11-20-b2-joint-top-c7a888/README.md) set two G2 ranges for its Zopro release and ordinary controls. The [earlier 1×2 release comparison](../2026-11-20-g2-active-runs-c7a858/README.md) used four. This follow-up compares both range settings with the same released executable, inputs and affinity. The candidate and its same-build ordinary control both use the four-range environment. The candidate explicitly divides the G2 query into three 12-bit ranges and combines both components in the leading window.

Zopro uses 80 main and 16 frontend physical cores, and prepared Aeglos uses all 96. Each process primes three proofs and times thirteen. Each principal transaction has eight paired blocks and each additional transaction has four. A joint mode advances with a lower paired median and at least six wins per principal input and three per additional input against its Zopro controls. All 123 exports independently verified. Separate probes confirmed the selected routes and the fallback at zero optional workspace. The temporary instance and storage were removed.

The four-range release took 7.878 and 7.874 ms on the original and held-out requests. Prepared Aeglos took 8.160 and 8.196 ms on those requests.

The joint mode took 9.178 ms on the original request and 8.818 ms on the held-out request. The joint mode did not improve both four-range Zopro controls across all four inputs.

| Input | Profile | Proof ms | Paired vs ordinary4 | Paired vs release2 | Paired vs release4 | Paired vs Aeglos | Peak MiB | Blocks at or below 3.7 ms |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | release4 | 7.878 | -0.53% (5/8) | -12.88% (8/8) | +0.00% (0/8) | -3.32% (8/8) | 42.9 | 0/8 |
| catalog | release2 | 9.117 | +15.37% (0/8) | +0.00% (0/8) | +14.78% (0/8) | +11.37% (0/8) | 42.3 | 0/8 |
| catalog | ordinary4 | 7.902 | +0.00% (0/8) | -13.32% (8/8) | +0.53% (3/8) | -3.25% (8/8) | 44.4 | 0/8 |
| catalog | 12x3-joint | 9.178 | +16.85% (0/8) | +1.67% (3/8) | +16.51% (0/8) | +11.63% (0/8) | 40.8 | 0/8 |
| catalog | aeglos | 8.160 | +3.36% (0/8) | -10.20% (8/8) | +3.45% (0/8) | +0.00% (0/8) | 458.6 | 0/8 |
| heldout | release4 | 7.874 | -0.15% (4/8) | -10.46% (8/8) | +0.00% (0/8) | -4.24% (8/8) | 42.2 | 0/8 |
| heldout | release2 | 8.776 | +10.68% (0/8) | +0.00% (0/8) | +11.68% (0/8) | +4.94% (0/8) | 40.0 | 0/8 |
| heldout | ordinary4 | 7.855 | +0.00% (0/8) | -9.64% (8/8) | +0.15% (4/8) | -3.96% (7/8) | 45.2 | 0/8 |
| heldout | 12x3-joint | 8.818 | +11.87% (0/8) | +2.48% (2/8) | +12.94% (0/8) | +7.27% (0/8) | 41.5 | 0/8 |
| heldout | aeglos | 8.196 | +4.13% (1/8) | -4.70% (8/8) | +4.42% (0/8) | +0.00% (0/8) | 462.1 | 0/8 |
| v27 | release4 | 7.865 | -0.37% (3/4) | -12.49% (4/4) | +0.00% (0/4) | -3.52% (4/4) | 39.9 | 0/4 |
| v27 | release2 | 9.089 | +14.30% (0/4) | +0.00% (0/4) | +14.32% (0/4) | +11.36% (0/4) | 40.1 | 0/4 |
| v27 | ordinary4 | 7.869 | +0.00% (0/4) | -12.47% (4/4) | +0.37% (1/4) | -2.93% (4/4) | 39.3 | 0/4 |
| v27 | 12x3-joint | 8.992 | +9.83% (0/4) | +0.14% (2/4) | +10.49% (0/4) | +7.79% (0/4) | 37.7 | 0/4 |
| v27 | aeglos | 8.164 | +3.02% (0/4) | -10.20% (4/4) | +3.65% (0/4) | +0.00% (0/4) | 459.3 | 0/4 |
| v28 | release4 | 7.904 | +0.73% (2/4) | -10.48% (4/4) | +0.00% (0/4) | -3.40% (3/4) | 40.1 | 0/4 |
| v28 | release2 | 9.042 | +12.65% (0/4) | +0.00% (0/4) | +11.73% (0/4) | +8.87% (0/4) | 40.0 | 0/4 |
| v28 | ordinary4 | 7.813 | +0.00% (0/4) | -11.19% (4/4) | -0.72% (2/4) | -3.10% (4/4) | 40.0 | 0/4 |
| v28 | 12x3-joint | 9.390 | +20.18% (0/4) | +6.03% (1/4) | +19.30% (0/4) | +15.24% (0/4) | 40.9 | 0/4 |
| v28 | aeglos | 8.148 | +3.20% (0/4) | -8.03% (4/4) | +3.52% (1/4) | +0.00% (0/4) | 453.9 | 0/4 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Routes](artifacts/routes.json) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**5/8** original requests reach 10× M5 CPU on the recorded profiles.

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|---:|
| 1×2 | 805 released SDK 80+16 | 7.858 | 37 / 24 | 4.71× / 3.05× | +4.158 |
| 2×2 | 805 released SDK medium K/H 64+32 | 12.029 | 59 / 30 | 4.90× / 2.49× | +6.129 |
| 4×2 | 854 seed4-emit | 14.726 | 109 / 48 | 7.40× / 3.26× | +3.826 |
| 8×2 | 852 SDK seeded tasks and B2 vector conversion | 18.849 | 189 / 74 | 10.03× / 3.93× | -0.051 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
