# Splitting the 4×2 H query between pools on C7a

The [whole-query placement run](../2026-11-20-four-by-two-kh-placement-c7a892/README.md) found that putting both K and H on the frontend raised latency. This trial keeps K on the frontend and tests splitting H into 12,334 + 86,340 points and 24,668 + 74,006 points. The head can run on the frontend or main pool. The tail always runs on the main pool. The two modes for each split use the same point and coefficient ranges and add their separate curve sums. Both ranges borrow their input slices. The optional reservation covers their aggregate workspace and a denied reservation leaves H unsplit.

The seeded solver, FFT and verification remain as in the frozen profile. Zopro uses 64 main and 32 frontend physical cores. Prepared Aeglos uses all 96. Eight counterbalanced blocks per input prime three requests and time thirteen. A split advances with a lower paired median and six wins against both unmodified controls on both inputs. The frontend profiles must also clear the corresponding main-pool split. All 172 exports independently verified, including zero-workspace fallbacks. The temporary instance and storage were removed.

The frozen seeded profile took 15.284 / 15.096 ms on the original and held-out inputs. The faster frontend split on the original input was `front8` at 15.791 / 15.771 ms, with paired changes of +4.70% / +4.72% against the same-build ordinary profile. None of the split profiles cleared its matched controls on both inputs.

| Input | Mode | Proof ms | Paired vs ordinary | Paired vs frozen | Paired vs same split on main | Paired vs Aeglos | Peak MiB | Blocks at or below 10.9 ms |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | release | 15.514 | +2.81% (0/8) | +2.30% (1/8) | — | -16.53% (8/8) | 208.8 | 0/8 |
| catalog | frozen | 15.284 | +0.41% (4/8) | +0.00% (0/8) | — | -18.84% (8/8) | 205.7 | 0/8 |
| catalog | ordinary | 15.124 | +0.00% (0/8) | -0.40% (4/8) | — | -19.73% (8/8) | 199.1 | 0/8 |
| catalog | main8 | 16.496 | +8.59% (0/8) | +8.02% (0/8) | — | -11.80% (8/8) | 207.2 | 0/8 |
| catalog | front8 | 15.791 | +4.70% (0/8) | +4.07% (1/8) | -3.87% (8/8) | -15.71% (8/8) | 210.1 | 0/8 |
| catalog | main4 | 17.770 | +17.03% (0/8) | +16.31% (0/8) | — | -4.90% (8/8) | 197.6 | 0/8 |
| catalog | front4 | 16.948 | +11.32% (0/8) | +10.79% (0/8) | -4.47% (8/8) | -9.63% (8/8) | 212.7 | 0/8 |
| catalog | aeglos | 18.755 | +24.57% (0/8) | +23.21% (0/8) | — | +0.00% (0/8) | 1349.1 | 0/8 |
| heldout | release | 15.680 | +4.44% (0/8) | +3.79% (0/8) | — | -16.10% (8/8) | 208.7 | 0/8 |
| heldout | frozen | 15.096 | +0.26% (4/8) | +0.00% (0/8) | — | -19.05% (8/8) | 194.4 | 0/8 |
| heldout | ordinary | 15.083 | +0.00% (0/8) | -0.25% (4/8) | — | -18.52% (8/8) | 202.1 | 0/8 |
| heldout | main8 | 16.429 | +8.93% (0/8) | +8.91% (0/8) | — | -12.03% (8/8) | 205.0 | 0/8 |
| heldout | front8 | 15.771 | +4.72% (0/8) | +4.83% (0/8) | -4.37% (8/8) | -15.63% (8/8) | 203.5 | 0/8 |
| heldout | main4 | 17.700 | +17.06% (0/8) | +17.67% (0/8) | — | -4.70% (7/8) | 218.6 | 0/8 |
| heldout | front4 | 17.032 | +13.01% (0/8) | +12.86% (0/8) | -4.48% (8/8) | -8.66% (8/8) | 207.9 | 0/8 |
| heldout | aeglos | 18.648 | +22.73% (0/8) | +23.53% (0/8) | — | +0.00% (0/8) | 1352.1 | 0/8 |

In three separate blocks, stage clocks measure the curve span and each query. Linux per-worker schedstat sums scheduled CPU time for each pinned pool across the timed requests. The counts include solver and FFT work, scheduling and worker spin. These blocks measure placement effects and do not enter the latency comparison.

| Input | Mode | Diagnostic proof ms | Curves ms | A ms | K ms | H ms | H head ms | H tail ms | B2 ms | Main CPU-ms/proof | Front CPU-ms/proof |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| catalog | ordinary | 15.156 | 7.619 | 5.159 | 7.262 | 5.824 | — | — | 7.354 | 539.3 | 126.4 |
| catalog | main8 | 16.791 | 9.108 | 8.190 | 7.471 | 8.021 | 4.753 | 6.446 | 8.432 | 578.0 | 129.0 |
| catalog | front8 | 16.149 | 8.457 | 7.352 | 7.028 | 7.294 | 1.189 | 5.887 | 7.981 | 541.9 | 151.0 |
| catalog | main4 | 18.101 | 10.516 | 9.630 | 7.506 | 8.720 | 5.349 | 6.227 | 9.748 | 599.2 | 124.3 |
| catalog | front4 | 17.066 | 9.479 | 8.732 | 7.331 | 7.725 | 1.840 | 5.386 | 8.670 | 556.9 | 159.9 |
| heldout | ordinary | 15.112 | 7.472 | 5.251 | 7.309 | 5.752 | — | — | 7.160 | 528.8 | 126.6 |
| heldout | main8 | 16.473 | 8.974 | 7.422 | 7.301 | 8.138 | 5.009 | 6.523 | 8.420 | 565.5 | 127.4 |
| heldout | front8 | 15.967 | 8.301 | 7.472 | 7.156 | 7.650 | 1.212 | 6.211 | 7.792 | 536.6 | 152.9 |
| heldout | main4 | 17.781 | 10.214 | 9.831 | 7.728 | 8.301 | 5.765 | 5.788 | 9.001 | 590.0 | 124.9 |
| heldout | front4 | 16.997 | 9.477 | 9.201 | 7.297 | 7.201 | 1.996 | 4.972 | 8.025 | 563.4 | 159.7 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Placement source](artifacts/source.patch) · [Verification](artifacts/verification.json)

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
