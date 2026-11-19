# B2 window geometry after scalar grouping

The experimental path keeps the existing scalar groups and affine point sums. It varies only the final B2 MSM. The `u` modes use the existing unsigned bucket code, `s` uses signed Booth digits, and `t` computes those same signed digits in a proof-local window-ordered matrix before bucket evaluation. Names give the bits per window and number of point ranges. The allocator admits any extra window and digit storage under the normal proof workspace policy. Budget denial leaves the established path available. The matrix stores at most about 2 MiB for the measured group sizes and retains no data across proofs.

C7a uses 64 main and 32 frontend physical cores for Zopro and 96 cores for warm Aeglos. 8×2 uses the released opt-in origin and serial conversion settings, and 4×2 uses ordinary grouping. The primary requests use four paired blocks on 4×2 and six on 8×2. The 8×2 follow-up inputs use four blocks and a subset of modes. Each measured process runs three warmups and thirteen timed proofs. Ryzen replays the primary comparison with 24 main and eight frontend logical CPUs. Its reported run follows an excluded timing attempt that overlapped compiler activity.

The independent G2 generator oracle passed for signed digit matrices with signed, zero, opposite, and identity cases at one and eight workers. All 468 published proof exports independently verified. Separate routing samples exercised all three mode types and the zero optional-memory fallback. AWS instance and temporary storage were removed.

- **4×2**. The lowest mean of the two primary paired changes belongs to `s12x3`. It took 15.274 / 15.438 ms and the paired changes versus the same-build control were +0.56% / +0.50%.
- **8×2**. The lowest mean of the two primary paired changes belongs to `s12x3`. It took 20.197 / 20.430 ms and the paired changes versus the same-build control were -1.35% / +0.17%.

The screen found no layout with repeatable gains against both controls on the primary requests and consistent performance on its available follow-up inputs. The code remains experimental.

| Shape | Input | Profile | Proof ms | Paired vs ordinary | Paired vs release | Paired vs Aeglos | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|
| 4×2 | catalog | aeglos | 18.089 | +19.59% (0/4) | +18.21% (0/4) | +0.00% (0/4) | 1343.3 |
| 4×2 | catalog | ordinary | 15.172 | +0.00% (0/4) | -0.98% (3/4) | -16.38% (4/4) | 201.2 |
| 4×2 | catalog | release | 15.294 | +0.99% (1/4) | +0.00% (0/4) | -15.41% (4/4) | 195.4 |
| 4×2 | catalog | s11x3 | 15.797 | +4.07% (0/4) | +3.27% (0/4) | -12.64% (4/4) | 211.7 |
| 4×2 | catalog | s12x2 | 15.445 | +1.67% (0/4) | +1.09% (1/4) | -14.63% (4/4) | 203.4 |
| 4×2 | catalog | s12x3 | 15.274 | +0.56% (1/4) | -0.59% (3/4) | -15.76% (4/4) | 201.9 |
| 4×2 | catalog | t11x3 | 16.208 | +6.51% (0/4) | +5.97% (0/4) | -10.35% (4/4) | 217.0 |
| 4×2 | catalog | t12x3 | 15.686 | +2.77% (0/4) | +2.56% (1/4) | -13.24% (4/4) | 222.2 |
| 4×2 | catalog | u10x4 | 15.906 | +4.88% (0/4) | +3.65% (0/4) | -12.32% (4/4) | 208.7 |
| 4×2 | catalog | u11x1 | 16.615 | +8.98% (0/4) | +8.56% (0/4) | -8.32% (4/4) | 200.5 |
| 4×2 | catalog | u12x3 | 15.805 | +3.81% (0/4) | +3.46% (0/4) | -12.63% (4/4) | 223.0 |
| 4×2 | heldout | aeglos | 18.202 | +18.49% (0/4) | +18.54% (0/4) | +0.00% (0/4) | 1348.9 |
| 4×2 | heldout | ordinary | 15.341 | +0.00% (0/4) | +0.04% (2/4) | -15.60% (4/4) | 197.4 |
| 4×2 | heldout | release | 15.359 | -0.04% (2/4) | +0.00% (0/4) | -15.64% (4/4) | 201.2 |
| 4×2 | heldout | s11x3 | 16.096 | +3.93% (1/4) | +3.97% (0/4) | -11.85% (4/4) | 205.6 |
| 4×2 | heldout | s12x2 | 15.619 | +1.99% (1/4) | +1.69% (1/4) | -15.02% (4/4) | 202.7 |
| 4×2 | heldout | s12x3 | 15.438 | +0.50% (1/4) | +0.20% (1/4) | -15.19% (4/4) | 203.4 |
| 4×2 | heldout | t11x3 | 16.328 | +6.32% (0/4) | +6.12% (0/4) | -10.74% (4/4) | 225.2 |
| 4×2 | heldout | t12x3 | 15.807 | +2.29% (1/4) | +1.98% (0/4) | -13.56% (4/4) | 220.7 |
| 4×2 | heldout | u10x4 | 16.212 | +4.77% (0/4) | +4.81% (0/4) | -10.72% (4/4) | 193.0 |
| 4×2 | heldout | u11x1 | 16.585 | +6.91% (0/4) | +7.05% (0/4) | -9.65% (4/4) | 197.5 |
| 4×2 | heldout | u12x3 | 15.970 | +3.80% (1/4) | +3.85% (0/4) | -12.48% (4/4) | 229.0 |
| 8×2 | catalog | aeglos | 30.483 | +50.15% (0/6) | +50.29% (0/6) | +0.00% (0/6) | 2454.6 |
| 8×2 | catalog | ordinary | 20.362 | +0.00% (0/6) | +0.31% (3/6) | -33.39% (6/6) | 411.5 |
| 8×2 | catalog | release | 20.391 | -0.31% (3/6) | +0.00% (0/6) | -33.46% (6/6) | 401.7 |
| 8×2 | catalog | s11x3 | 20.838 | +2.20% (0/6) | +2.93% (0/6) | -31.34% (6/6) | 402.8 |
| 8×2 | catalog | s12x2 | 20.302 | -0.30% (4/6) | -0.56% (5/6) | -33.12% (6/6) | 413.8 |
| 8×2 | catalog | s12x3 | 20.197 | -1.35% (5/6) | -1.09% (6/6) | -34.15% (6/6) | 410.1 |
| 8×2 | catalog | t11x3 | 21.162 | +4.06% (0/6) | +4.12% (0/6) | -30.71% (6/6) | 418.4 |
| 8×2 | catalog | t12x3 | 20.222 | -0.42% (3/6) | -0.29% (3/6) | -33.50% (6/6) | 429.0 |
| 8×2 | catalog | u10x4 | 20.609 | +1.85% (2/6) | +1.42% (0/6) | -32.55% (6/6) | 415.4 |
| 8×2 | catalog | u11x1 | 21.815 | +8.13% (0/6) | +7.33% (0/6) | -28.66% (6/6) | 411.6 |
| 8×2 | catalog | u12x3 | 20.500 | +0.92% (2/6) | +0.86% (2/6) | -33.02% (6/6) | 424.5 |
| 8×2 | heldout | aeglos | 30.717 | +49.46% (0/6) | +50.47% (0/6) | +0.00% (0/6) | 2463.2 |
| 8×2 | heldout | ordinary | 20.341 | +0.00% (0/6) | +0.13% (3/6) | -33.09% (6/6) | 434.8 |
| 8×2 | heldout | release | 20.372 | -0.13% (3/6) | +0.00% (0/6) | -33.54% (6/6) | 399.4 |
| 8×2 | heldout | s11x3 | 21.069 | +3.52% (0/6) | +3.33% (0/6) | -31.24% (6/6) | 401.0 |
| 8×2 | heldout | s12x2 | 20.443 | +0.13% (3/6) | +0.12% (3/6) | -33.18% (6/6) | 415.3 |
| 8×2 | heldout | s12x3 | 20.430 | +0.17% (3/6) | -0.09% (4/6) | -33.48% (6/6) | 409.2 |
| 8×2 | heldout | t11x3 | 21.480 | +5.11% (0/6) | +5.44% (0/6) | -30.06% (6/6) | 425.6 |
| 8×2 | heldout | t12x3 | 20.549 | +0.59% (2/6) | +0.87% (2/6) | -33.10% (6/6) | 439.1 |
| 8×2 | heldout | u10x4 | 20.808 | +2.13% (0/6) | +2.07% (0/6) | -31.85% (6/6) | 419.2 |
| 8×2 | heldout | u11x1 | 22.041 | +8.24% (0/6) | +8.54% (0/6) | -27.54% (6/6) | 395.2 |
| 8×2 | heldout | u12x3 | 20.726 | +1.89% (0/6) | +1.68% (0/6) | -32.27% (6/6) | 426.6 |
| 8×2 | v153 | aeglos | 30.536 | +49.53% (0/4) | +50.43% (0/4) | +0.00% (0/4) | 2458.7 |
| 8×2 | v153 | ordinary | 20.347 | +0.00% (0/4) | +0.12% (2/4) | -33.13% (4/4) | 414.3 |
| 8×2 | v153 | release | 20.336 | -0.12% (2/4) | +0.00% (0/4) | -33.52% (4/4) | 408.3 |
| 8×2 | v153 | s12x3 | 20.386 | +0.37% (2/4) | +0.48% (2/4) | -33.24% (4/4) | 413.0 |
| 8×2 | v153 | t12x3 | 20.643 | +1.22% (1/4) | +1.52% (0/4) | -32.31% (4/4) | 419.8 |
| 8×2 | v153 | u11x1 | 22.094 | +8.61% (0/4) | +8.79% (0/4) | -27.18% (4/4) | 405.5 |
| 8×2 | v153 | u12x3 | 20.738 | +2.29% (0/4) | +2.10% (0/4) | -31.86% (4/4) | 424.9 |
| 8×2 | v156 | aeglos | 30.867 | +51.42% (0/4) | +50.30% (0/4) | +0.00% (0/4) | 2459.5 |
| 8×2 | v156 | ordinary | 20.391 | +0.00% (0/4) | -0.45% (3/4) | -33.96% (4/4) | 408.3 |
| 8×2 | v156 | release | 20.450 | +0.46% (1/4) | +0.00% (0/4) | -33.47% (4/4) | 416.0 |
| 8×2 | v156 | s12x3 | 20.510 | +0.48% (2/4) | +0.31% (1/4) | -33.65% (4/4) | 412.2 |
| 8×2 | v156 | t12x3 | 20.601 | +1.12% (1/4) | +0.82% (1/4) | -33.32% (4/4) | 419.7 |
| 8×2 | v156 | u11x1 | 21.988 | +7.76% (0/4) | +7.53% (0/4) | -28.84% (4/4) | 401.1 |
| 8×2 | v156 | u12x3 | 20.658 | +1.35% (1/4) | +1.04% (1/4) | -33.17% (4/4) | 414.7 |

[Source](artifacts/source.patch) · [C7a samples](artifacts/c7a-rows.json.gz) · [Ryzen samples](artifacts/ryzen-rows.json.gz) · [Summary](artifacts/summary.json) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU.

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 737 dedicated G1 80+16 | 7.953 | 37 / 24 | 4.65× / 3.02× | +4.253 |
| 2×2 | 750 archived-band 72+24 | 12.358 | 59 / 30 | 4.77× / 2.43× | +6.458 |
| 4×2 | 757 solve-front-on | 15.257 | 109 / 48 | 7.14× / 3.15× | +4.357 |
| 8×2 | 781 recognized B2 origins replay | 20.098 | 189 / 74 | 9.40× / 3.68× | +1.198 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
