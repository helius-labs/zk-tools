# Independently weighted 4×2 G2 ranges on 96 C7a cores

Two point ranges use the same G2 endomorphism decomposition. The grouped Phi arm merges the affine buckets before weighting. The independent arm weights each range within its window job and combines their projective results in the final fold. The paired control isolates weighting order and task structure. The release provides the ordinary B2 path. Zopro uses the same seeded solver and 64 main and 32 frontend physical cores in each arm. Prepared Aeglos uses all 96 physical cores. [Ryzen comparison](../2026-11-20-flat-g2-weighted-ranges-ryzen868/README.md)

Eight counterbalanced process blocks per primary input and four per additional input each prime three proofs and time thirteen. A candidate advances with lower paired medians and at least six of eight primary wins and three of four additional wins against its controls. All 104 exports independently verified. Separate probes checked route selection and fallback at zero optional workspace. The temporary instance and storage were removed.

Independent weighting took 15.986 ms on the original request and 16.018 ms on the held-out request. Neither grouped Phi nor independent weighting consistently improved its controls across all four inputs.

| Input | Profile | Proof ms | Paired vs Phi | Paired vs release | Paired vs Aeglos | Peak MiB | Blocks at or below 10.9 ms |
|---|---|---:|---:|---:|---:|---:|---:|
| catalog | release | 14.948 | -6.13% (8/8) | +0.00% (0/8) | -18.46% (8/8) | 203.5 | 0/8 |
| catalog | phi | 15.958 | +0.00% (0/8) | +6.54% (0/8) | -12.91% (8/8) | 201.9 | 0/8 |
| catalog | weighted2 | 15.986 | -0.01% (4/8) | +6.49% (0/8) | -13.08% (8/8) | 196.6 | 0/8 |
| catalog | aeglos | 18.319 | +14.82% (0/8) | +22.65% (0/8) | +0.00% (0/8) | 1343.1 | 0/8 |
| heldout | release | 15.072 | -5.76% (8/8) | +0.00% (0/8) | -19.73% (8/8) | 204.3 | 0/8 |
| heldout | phi | 16.102 | +0.00% (0/8) | +6.11% (0/8) | -13.11% (8/8) | 207.9 | 0/8 |
| heldout | weighted2 | 16.018 | -0.48% (4/8) | +6.97% (0/8) | -14.34% (8/8) | 196.0 | 0/8 |
| heldout | aeglos | 18.569 | +15.08% (0/8) | +24.58% (0/8) | +0.00% (0/8) | 1344.9 | 0/8 |
| v153 | release | 15.046 | -6.25% (4/4) | +0.00% (0/4) | -19.37% (4/4) | 202.0 | 0/4 |
| v153 | phi | 16.113 | +0.00% (0/4) | +6.67% (0/4) | -14.25% (4/4) | 201.9 | 0/4 |
| v153 | weighted2 | 15.875 | -1.27% (3/4) | +5.84% (0/4) | -15.30% (4/4) | 196.6 | 0/4 |
| v153 | aeglos | 18.872 | +16.62% (0/4) | +24.02% (0/4) | +0.00% (0/4) | 1346.5 | 0/4 |
| v156 | release | 15.310 | -5.05% (4/4) | +0.00% (0/4) | -17.67% (4/4) | 193.6 | 0/4 |
| v156 | phi | 16.290 | +0.00% (0/4) | +5.33% (0/4) | -11.71% (4/4) | 194.5 | 0/4 |
| v156 | weighted2 | 15.893 | -2.39% (4/4) | +3.85% (0/4) | -13.89% (4/4) | 204.2 | 0/4 |
| v156 | aeglos | 18.457 | +13.26% (0/4) | +21.54% (0/4) | +0.00% (0/4) | 1349.7 | 0/4 |

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
