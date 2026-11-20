# Placing the 4×2 G1 K and H queries on C7a

The [curve replay](../2026-11-20-four-by-two-phase-c7a891/README.md) found that K and B2 often finish close together. This run reuses the exact seeded solver, quotient and verification settings with 64 main and 32 frontend physical cores. The ordinary profile runs K on the frontend. The main profile keeps K and H on the main pool, H moves H alone to frontend, and KH moves both there. The joined mode submits K and H as one frontend scope with a parallel join. It admits both queries' optional scratch together. The independent mode admits each query separately. A and B2 always use the same main placement. The frozen profile uses the original seeded executable, and prepared Aeglos uses all 96 cores.

Eight counterbalanced blocks per input prime three requests and time thirteen. An alternate advances with a lower paired median and at least six wins against each Zopro control on each input. All 172 exports independently verified. Separate traces checked the K/H routes and zero optional-workspace fallbacks. The temporary instance and storage were removed.

The frozen seeded profile took 15.243 / 15.142 ms on the original and held-out inputs. The best alternate on the original input was `kh` at 15.595 / 15.652 ms, with paired changes of +2.97% / +2.27% against the same-build ordinary profile. None of the alternate placements cleared both matched controls on both inputs.

| Input | Mode | Proof ms | Paired vs ordinary | Paired vs frozen | Paired vs Aeglos | Peak MiB | Blocks at or below 10.9 ms |
|---|---|---:|---:|---:|---:|---:|---:|
| catalog | release | 15.612 | +3.89% (0/8) | +2.83% (0/8) | -16.60% (8/8) | 205.8 | 0/8 |
| catalog | frozen | 15.243 | +0.64% (3/8) | +0.00% (0/8) | -19.04% (8/8) | 194.5 | 0/8 |
| catalog | ordinary | 15.150 | +0.00% (0/8) | -0.63% (5/8) | -19.39% (8/8) | 201.2 | 0/8 |
| catalog | main | 16.501 | +9.20% (0/8) | +8.77% (0/8) | -11.87% (8/8) | 220.9 | 0/8 |
| catalog | h | 16.740 | +9.76% (0/8) | +10.12% (0/8) | -10.70% (8/8) | 228.9 | 0/8 |
| catalog | kh | 15.595 | +2.97% (0/8) | +2.46% (0/8) | -16.88% (8/8) | 211.0 | 0/8 |
| catalog | joined | 15.600 | +3.39% (0/8) | +2.35% (0/8) | -16.66% (8/8) | 162.1 | 0/8 |
| catalog | aeglos | 18.783 | +24.05% (0/8) | +23.52% (0/8) | +0.00% (0/8) | 1349.4 | 0/8 |
| heldout | release | 15.693 | +3.35% (0/8) | +3.51% (1/8) | -16.76% (8/8) | 212.4 | 0/8 |
| heldout | frozen | 15.142 | +0.23% (3/8) | +0.00% (0/8) | -19.11% (8/8) | 196.7 | 0/8 |
| heldout | ordinary | 15.217 | +0.00% (0/8) | -0.23% (5/8) | -19.30% (8/8) | 202.7 | 0/8 |
| heldout | main | 16.707 | +10.36% (0/8) | +10.03% (0/8) | -11.13% (8/8) | 223.8 | 0/8 |
| heldout | h | 16.601 | +9.41% (0/8) | +9.45% (0/8) | -11.26% (8/8) | 226.6 | 0/8 |
| heldout | kh | 15.652 | +2.27% (0/8) | +3.52% (0/8) | -16.16% (8/8) | 209.7 | 0/8 |
| heldout | joined | 15.677 | +2.87% (0/8) | +3.76% (0/8) | -16.06% (8/8) | 190.6 | 0/8 |
| heldout | aeglos | 18.780 | +23.92% (0/8) | +23.63% (0/8) | +0.00% (0/8) | 1343.5 | 0/8 |

In three separate blocks, stage clocks measure the curve span and each query. Linux per-worker schedstat sums scheduled CPU time for each pinned pool across the timed requests. The counts include solver and FFT work, scheduling and worker spin. These blocks measure placement effects and do not enter the latency comparison.

| Input | Mode | Diagnostic proof ms | Curves ms | A ms | K ms | H ms | B2 ms | Main CPU-ms/proof | Front CPU-ms/proof |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| catalog | ordinary | 15.059 | 7.533 | 5.222 | 7.281 | 6.187 | 7.340| 535.0 | 128.5 |
| catalog | main | 16.792 | 9.226 | 7.184 | 8.126 | 6.645 | 8.662| 613.2 | 45.0 |
| catalog | h | 16.723 | 9.186 | 7.296 | 8.311 | 7.575 | 8.383| 512.4 | 200.9 |
| catalog | kh | 15.655 | 8.073 | 6.544 | 7.641 | 7.866 | 7.326| 431.8 | 255.8 |
| catalog | joined | 15.571 | 7.946 | 5.523 | 6.796 | 7.037 | 7.141| 431.3 | 257.8 |
| heldout | ordinary | 15.179 | 7.515 | 4.707 | 7.183 | 5.470 | 7.391| 541.4 | 127.2 |
| heldout | main | 16.698 | 9.081 | 6.224 | 7.637 | 6.950 | 8.634| 607.4 | 45.0 |
| heldout | h | 16.740 | 9.185 | 6.722 | 7.749 | 7.649 | 8.413| 519.3 | 201.2 |
| heldout | kh | 15.652 | 8.048 | 5.257 | 7.773 | 7.904 | 7.275| 431.3 | 257.3 |
| heldout | joined | 15.627 | 7.995 | 5.416 | 6.843 | 7.080 | 7.262| 435.8 | 255.9 |

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
