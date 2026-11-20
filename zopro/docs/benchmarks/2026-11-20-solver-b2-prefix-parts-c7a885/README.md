# Parallel prefix fills after B2 suffix staging on C7a

The [Ryzen trial](../2026-11-20-solver-b2-prefix-parts-ryzen884/README.md) measured lower 2×2 latency than release on both inputs. This follow-up uses the same executable on c7a.24xlarge. The early split uses the main pool to stage a borrowed suffix during the solve, then divides each remaining prefix window between two fills. Both fills merge with the suffix before one weighted reduction. The late split starts the suffix after solving. The single-prefix arm retains the same early placement.

Zopro uses 64 main and 32 frontend physical cores. Prepared Aeglos uses all 96. Eight matched blocks per input prime three proofs and time thirteen. Each early split comparison against the four Zopro controls requires a lower paired median and six wins per input. All 104 proof exports independently verified, including the ordinary path at zero optional workspace. The temporary instance and storage were removed.

The early split took 12.262 ms on the original request and 12.231 ms on the held-out request. The early split fell short of the matched Zopro comparison across both inputs.

| Input | Profile | Proof ms | Paired vs ordinary | Paired vs one prefix fill | Paired vs split late | Paired vs release | Paired vs Aeglos | Peak MiB | Blocks at or below 5.9 ms |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| catalog | release | 12.005 | +0.61% (3/8) | -14.38% (8/8) | -6.72% (8/8) | +0.00% (0/8) | +3.35% (1/8) | 79.7 | 0/8 |
| catalog | ordinary | 11.927 | +0.00% (0/8) | -14.28% (8/8) | -6.98% (8/8) | -0.60% (5/8) | +2.45% (1/8) | 76.7 | 0/8 |
| catalog | main-early | 13.948 | +16.66% (0/8) | +0.00% (0/8) | +8.31% (0/8) | +16.79% (0/8) | +19.89% (0/8) | 81.2 | 0/8 |
| catalog | split-late | 12.887 | +7.50% (0/8) | -7.68% (8/8) | +0.00% (0/8) | +7.20% (0/8) | +11.18% (0/8) | 79.0 | 0/8 |
| catalog | split-early | 12.262 | +3.12% (0/8) | -12.29% (8/8) | -4.85% (8/8) | +2.06% (0/8) | +5.78% (0/8) | 79.0 | 0/8 |
| catalog | aeglos | 11.604 | -2.39% (7/8) | -16.59% (8/8) | -10.05% (8/8) | -3.24% (7/8) | +0.00% (0/8) | 809.4 | 0/8 |
| heldout | release | 12.133 | +0.42% (3/8) | -13.28% (8/8) | -6.14% (8/8) | +0.00% (0/8) | +5.33% (0/8) | 76.0 | 0/8 |
| heldout | ordinary | 12.088 | +0.00% (0/8) | -12.67% (8/8) | -6.37% (8/8) | -0.42% (5/8) | +6.00% (1/8) | 75.2 | 0/8 |
| heldout | main-early | 13.974 | +14.51% (0/8) | +0.00% (0/8) | +8.42% (0/8) | +15.31% (0/8) | +21.26% (0/8) | 81.2 | 0/8 |
| heldout | split-late | 12.890 | +6.80% (0/8) | -7.77% (8/8) | +0.00% (0/8) | +6.54% (0/8) | +11.84% (0/8) | 78.3 | 0/8 |
| heldout | split-early | 12.231 | +0.40% (0/8) | -12.21% (8/8) | -5.08% (8/8) | +1.08% (2/8) | +6.32% (0/8) | 79.7 | 0/8 |
| heldout | aeglos | 11.525 | -5.66% (7/8) | -17.53% (8/8) | -10.59% (8/8) | -5.06% (8/8) | +0.00% (0/8) | 802.9 | 0/8 |

Separate probes record the workers and phases. Times are medians of four calls including primes.

| Input | Probe | FFT ms | B2 suffix ms | Overlap ms |
|---|---|---:|---:|---:|
| catalog | main-early | 1.356 | 3.202 | 1.356 |
| catalog | split-late | 1.488 | 3.141 | 1.488 |
| catalog | split-early | 1.438 | 3.095 | 1.438 |
| heldout | main-early | 1.567 | 2.991 | 1.567 |
| heldout | split-late | 1.394 | 3.206 | 1.394 |
| heldout | split-early | 1.539 | 2.932 | 1.539 |

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
