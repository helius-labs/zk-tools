# Solver epochs and disjoint core placement for 8×2 on C7a

The [first epoch screen](../2026-11-19-folded-uninit-epochs-c7a829/README.md) measured 19.10 ms with four waves on a 72+24 split. This run compares two, three and five waves with four. It also tests three contiguous middle or three separated frontend L3 groups, with an explicit main CPU list that stays disjoint. The base arm uses initialized solver buffers and eight waves. The released four-wave arm and the same-build ordinary arm enable the existing definite-assignment check and use the tail three L3 groups. All Zopro arms use the same curve arithmetic, with packed solver work, frontend K, recognized B2 origins and serial B2 conversion.

The two primary inputs have six matched blocks. Two additional inputs have four. Each process primes three calls and times thirteen. Prepared Aeglos uses the same 96 physical cores. All 180 AWS and six Ryzen proof exports independently verified. The separate Ryzen probes and two placement unit tests checked the explicit CPU selection. The temporary AWS instance and storage were removed.

The lowest eligible original-request median was 18.964 ms with `ordinary`. release-e4 beat its controls across all four inputs. It measured 19.117 ms on the original request and 19.448 ms on the second.

| Input | Mode | Proof ms | Paired vs initialized base | Paired vs released 4 waves | Paired vs same-build 4 waves | Paired vs Aeglos | Peak MiB | Blocks at or below 18.9 ms |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | base | 19.701 | +0.00% (0/6) | +2.68% (0/6) | +3.53% (0/6) | -34.70% (6/6) | 375.3 | 0/6 |
| catalog | release-e4 | 19.117 | -2.61% (6/6) | +0.00% (0/6) | +1.29% (1/6) | -36.22% (6/6) | 364.4 | 1/6 |
| catalog | ordinary | 18.964 | -3.41% (6/6) | -1.27% (5/6) | +0.00% (0/6) | -37.08% (6/6) | 351.6 | 3/6 |
| catalog | epochs-2 | 20.444 | +3.60% (0/6) | +6.61% (0/6) | +7.68% (0/6) | -32.26% (6/6) | 364.1 | 0/6 |
| catalog | epochs-3 | 19.756 | +0.48% (1/6) | +3.25% (0/6) | +4.59% (0/6) | -33.99% (6/6) | 358.3 | 0/6 |
| catalog | epochs-5 | 19.136 | -2.95% (6/6) | +0.30% (2/6) | +0.90% (1/6) | -36.01% (6/6) | 357.6 | 0/6 |
| catalog | front-middle | 19.080 | -3.68% (6/6) | -1.13% (4/6) | +0.49% (3/6) | -36.76% (6/6) | 358.3 | 2/6 |
| catalog | front-spread | 19.236 | -2.73% (6/6) | -0.21% (3/6) | +1.52% (2/6) | -36.30% (6/6) | 360.4 | 2/6 |
| catalog | aeglos | 30.113 | +53.14% (0/6) | +56.78% (0/6) | +58.93% (0/6) | +0.00% (0/6) | 2459.4 | 0/6 |
| heldout | base | 19.803 | +0.00% (0/6) | +2.29% (0/6) | +2.76% (0/6) | -35.34% (6/6) | 380.1 | 0/6 |
| heldout | release-e4 | 19.448 | -2.24% (6/6) | +0.00% (0/6) | +0.80% (2/6) | -36.61% (6/6) | 362.6 | 0/6 |
| heldout | ordinary | 19.306 | -2.67% (6/6) | -0.80% (4/6) | +0.00% (0/6) | -36.90% (6/6) | 365.8 | 0/6 |
| heldout | epochs-2 | 20.819 | +5.02% (0/6) | +7.05% (0/6) | +8.27% (0/6) | -31.82% (6/6) | 383.7 | 0/6 |
| heldout | epochs-3 | 20.112 | +1.33% (1/6) | +3.13% (0/6) | +4.70% (0/6) | -34.12% (6/6) | 361.2 | 0/6 |
| heldout | epochs-5 | 19.448 | -1.67% (6/6) | +0.28% (2/6) | +1.17% (1/6) | -36.74% (6/6) | 372.3 | 0/6 |
| heldout | front-middle | 19.267 | -2.20% (6/6) | -1.19% (4/6) | -0.41% (5/6) | -37.06% (6/6) | 368.1 | 1/6 |
| heldout | front-spread | 19.146 | -2.72% (6/6) | -0.73% (4/6) | -0.49% (4/6) | -37.29% (6/6) | 365.8 | 0/6 |
| heldout | aeglos | 30.558 | +54.67% (0/6) | +57.75% (0/6) | +58.49% (0/6) | +0.00% (0/6) | 2455.6 | 0/6 |
| v153 | base | 19.665 | +0.00% (0/4) | +1.65% (1/4) | +2.99% (0/4) | -34.71% (4/4) | 371.0 | 0/4 |
| v153 | release-e4 | 19.396 | -1.62% (3/4) | +0.00% (0/4) | +0.92% (1/4) | -36.26% (4/4) | 365.6 | 0/4 |
| v153 | ordinary | 19.194 | -2.89% (4/4) | -0.91% (3/4) | +0.00% (0/4) | -37.36% (4/4) | 360.5 | 0/4 |
| v153 | epochs-2 | 20.923 | +6.21% (0/4) | +7.87% (0/4) | +8.86% (0/4) | -31.17% (4/4) | 385.9 | 0/4 |
| v153 | epochs-3 | 20.096 | +2.81% (1/4) | +4.10% (0/4) | +4.70% (0/4) | -33.84% (4/4) | 361.2 | 0/4 |
| v153 | epochs-5 | 19.574 | -0.46% (3/4) | +1.00% (0/4) | +2.16% (0/4) | -35.90% (4/4) | 363.3 | 0/4 |
| v153 | front-middle | 19.354 | -1.73% (3/4) | -0.22% (3/4) | +0.70% (1/4) | -36.47% (4/4) | 356.0 | 0/4 |
| v153 | front-spread | 19.250 | -2.09% (4/4) | -0.88% (3/4) | +0.20% (1/4) | -36.86% (4/4) | 356.8 | 0/4 |
| v153 | aeglos | 30.507 | +53.17% (0/4) | +56.89% (0/4) | +59.63% (0/4) | +0.00% (0/4) | 2470.0 | 0/4 |
| v156 | base | 19.779 | +0.00% (0/4) | +2.25% (1/4) | +2.03% (1/4) | -35.91% (4/4) | 382.3 | 0/4 |
| v156 | release-e4 | 19.584 | -2.19% (3/4) | +0.00% (0/4) | +0.74% (1/4) | -36.46% (4/4) | 376.2 | 0/4 |
| v156 | ordinary | 19.318 | -1.98% (3/4) | -0.73% (3/4) | +0.00% (0/4) | -36.52% (4/4) | 360.4 | 1/4 |
| v156 | epochs-2 | 20.779 | +5.05% (1/4) | +5.65% (0/4) | +7.34% (0/4) | -32.12% (4/4) | 364.8 | 0/4 |
| v156 | epochs-3 | 20.197 | +2.36% (0/4) | +4.10% (0/4) | +4.55% (0/4) | -34.03% (4/4) | 364.2 | 0/4 |
| v156 | epochs-5 | 19.449 | -0.41% (2/4) | +0.26% (2/4) | +0.68% (1/4) | -36.64% (4/4) | 358.2 | 0/4 |
| v156 | front-middle | 19.379 | -1.39% (4/4) | +0.19% (2/4) | +0.30% (2/4) | -36.86% (4/4) | 381.6 | 0/4 |
| v156 | front-spread | 19.303 | -1.44% (4/4) | -1.65% (3/4) | +0.56% (2/4) | -36.81% (4/4) | 358.9 | 0/4 |
| v156 | aeglos | 30.478 | +56.06% (0/4) | +57.40% (0/4) | +57.53% (0/4) | +0.00% (0/4) | 2466.1 | 0/4 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json) · [Affinity source](artifacts/source.patch)

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU.

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 805 released SDK 80+16 | 7.858 | 37 / 24 | 4.71× / 3.05× | +4.158 |
| 2×2 | 805 released SDK medium K/H 64+32 | 12.029 | 59 / 30 | 4.90× / 2.49× | +6.129 |
| 4×2 | 828 frontend solver full | 15.103 | 109 / 48 | 7.22× / 3.18× | +4.203 |
| 8×2 | 833 SDK release-e4 72+24 | 19.117 | 189 / 74 | 9.89× / 3.87× | +0.217 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
