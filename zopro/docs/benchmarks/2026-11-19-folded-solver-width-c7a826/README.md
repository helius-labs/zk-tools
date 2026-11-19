# Packed frontend solver width for 8×2 on C7a

The [placement comparison](../2026-11-19-folded-solver-placement-c7a824/README.md) found that the packed solver ran faster on the frontend pool. This run compares six main/frontend splits using the released Zopro SDK, with packed tasks, frontend K and the same B2 origin and serial conversion options. The main-64 and front-64 controls keep 64 main and 32 frontend cores and change where the solver runs. Each Zopro configuration uses 96 physical cores in total. Prepared Aeglos uses all 96 cores of the same c7a.24xlarge.

The primary requests have six matched blocks and the two additional requests have four. Each process primes three calls and times thirteen. The timeline binary separately clocks two frontend widths. All 184 exported proofs independently verified. The temporary instance and storage were removed.

The lowest original-request median was 19.499 ms with `front-72`. front-72 beat both 64-main controls on all four inputs. It measured 19.499 ms on the original request and 19.891 ms on the second.

| Input | Mode | Proof ms | Paired vs main-64 | Paired vs front-64 | Paired vs Aeglos | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| catalog | main-64 | 20.504 | +0.00% (0/6) | +3.05% (0/6) | -33.35% (6/6) | 428.0 |
| catalog | front-64 | 19.873 | -2.96% (6/6) | +0.00% (0/6) | -35.11% (6/6) | 379.9 |
| catalog | front-68 | 19.854 | -2.69% (5/6) | +0.02% (3/6) | -34.77% (6/6) | 376.7 |
| catalog | front-72 | 19.499 | -4.36% (6/6) | -1.89% (6/6) | -35.90% (6/6) | 375.4 |
| catalog | front-76 | 20.863 | +2.02% (0/6) | +5.29% (0/6) | -31.71% (6/6) | 376.1 |
| catalog | front-80 | 20.598 | +1.37% (2/6) | +3.82% (0/6) | -32.77% (6/6) | 375.5 |
| catalog | front-84 | 21.875 | +6.42% (0/6) | +10.22% (0/6) | -28.81% (6/6) | 381.3 |
| catalog | aeglos | 30.506 | +50.05% (0/6) | +54.12% (0/6) | +0.00% (0/6) | 2493.1 |
| catalog | front-72-clock | 19.648 | -3.48% (6/6) | -0.45% (4/6) | -35.55% (6/6) | 370.9 |
| catalog | front-80-clock | 20.480 | +0.49% (2/6) | +3.26% (0/6) | -33.02% (6/6) | 399.4 |
| heldout | main-64 | 20.501 | +0.00% (0/6) | +2.00% (0/6) | -33.19% (6/6) | 416.9 |
| heldout | front-64 | 20.047 | -1.96% (6/6) | +0.00% (0/6) | -34.62% (6/6) | 376.9 |
| heldout | front-68 | 20.147 | -1.87% (6/6) | -0.00% (3/6) | -34.63% (6/6) | 379.3 |
| heldout | front-72 | 19.891 | -2.38% (6/6) | -1.08% (5/6) | -34.95% (6/6) | 378.4 |
| heldout | front-76 | 21.131 | +3.08% (0/6) | +5.07% (0/6) | -31.50% (6/6) | 379.4 |
| heldout | front-80 | 20.927 | +1.93% (0/6) | +4.37% (0/6) | -31.63% (6/6) | 392.7 |
| heldout | front-84 | 22.060 | +7.38% (0/6) | +9.53% (0/6) | -28.39% (6/6) | 390.4 |
| heldout | aeglos | 30.802 | +49.68% (0/6) | +52.96% (0/6) | +0.00% (0/6) | 2460.2 |
| heldout | front-72-clock | 19.862 | -2.61% (6/6) | -1.40% (6/6) | -35.23% (6/6) | 389.8 |
| heldout | front-80-clock | 20.763 | +0.89% (1/6) | +3.49% (0/6) | -32.60% (6/6) | 389.0 |
| v153 | main-64 | 20.402 | +0.00% (0/4) | +1.54% (0/4) | -32.53% (4/4) | 423.5 |
| v153 | front-64 | 20.113 | -1.52% (4/4) | +0.00% (0/4) | -33.85% (4/4) | 383.7 |
| v153 | front-68 | 19.918 | -2.39% (4/4) | -0.82% (3/4) | -34.13% (4/4) | 388.1 |
| v153 | front-72 | 19.892 | -2.76% (4/4) | -1.10% (4/4) | -35.01% (4/4) | 377.0 |
| v153 | front-76 | 21.180 | +3.59% (0/4) | +5.22% (0/4) | -30.30% (4/4) | 394.2 |
| v153 | front-80 | 21.000 | +2.61% (0/4) | +4.45% (0/4) | -30.82% (4/4) | 395.8 |
| v153 | front-84 | 22.115 | +8.54% (0/4) | +10.10% (0/4) | -27.65% (4/4) | 384.5 |
| v153 | aeglos | 30.355 | +48.22% (0/4) | +51.17% (0/4) | +0.00% (0/4) | 2508.4 |
| v156 | main-64 | 20.571 | +0.00% (0/4) | +3.10% (0/4) | -32.50% (4/4) | 401.8 |
| v156 | front-64 | 19.950 | -3.01% (4/4) | +0.00% (0/4) | -34.40% (4/4) | 374.0 |
| v156 | front-68 | 19.825 | -3.82% (4/4) | -0.39% (3/4) | -34.81% (4/4) | 369.4 |
| v156 | front-72 | 19.791 | -4.23% (4/4) | -1.31% (3/4) | -35.42% (4/4) | 376.2 |
| v156 | front-76 | 20.944 | +1.81% (0/4) | +4.92% (0/4) | -31.70% (4/4) | 372.2 |
| v156 | front-80 | 20.876 | +1.88% (0/4) | +5.43% (0/4) | -31.31% (4/4) | 380.6 |
| v156 | front-84 | 22.091 | +6.90% (0/4) | +10.43% (0/4) | -27.27% (4/4) | 380.0 |
| v156 | aeglos | 30.413 | +48.15% (0/4) | +52.44% (0/4) | +0.00% (0/4) | 2464.4 |

## Clocked stage times

| Input | Schedule | Proof ms | Paired clock effect | Solve ms | Gather ms | Quotient ms | Curve interval ms |
|---|---|---:|---:|---:|---:|---:|---:|
| catalog | front-72 | 19.648 | +0.54% | 3.676 | 0.974 | 2.854 | 10.130 |
| catalog | front-80 | 20.480 | -0.44% | 3.458 | 1.081 | 2.880 | 11.014 |
| heldout | front-72 | 19.862 | -0.00% | 3.703 | 0.977 | 2.851 | 10.251 |
| heldout | front-80 | 20.763 | -1.02% | 3.435 | 1.082 | 2.852 | 11.362 |

[Samples and clocks](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Clock source](artifacts/source.patch) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU.

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 805 released SDK 80+16 | 7.858 | 37 / 24 | 4.71× / 3.05× | +4.158 |
| 2×2 | 805 released SDK medium K/H 64+32 | 12.029 | 59 / 30 | 4.90× / 2.49× | +6.129 |
| 4×2 | 757 solve-front-on | 15.257 | 109 / 48 | 7.14× / 3.15× | +4.357 |
| 8×2 | 826 released SDK 72+24 packed frontend | 19.499 | 189 / 74 | 9.69× / 3.79× | +0.599 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
