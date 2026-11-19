# Solver and G2 switches with frontend K placement

The experiment uses the released SDK with the per-key K option and fused FFT at 64+32 disjoint cores. The 4×2 control also enables the frontend solver, as in the selected earlier profile. The trial changes existing runtime switches for owned grouped Phi, direct Poseidon output encoding, the number of solver epochs and packed solver task claiming. All Zopro variants use the same executable, keys and arithmetic configuration outside the listed switches. Aeglos uses all 96 physical cores with its prepared keys.

The primary two transactions use six matched blocks. Two additional inputs use three blocks for the baseline and the Phi and encoding variants. Each process primes three requests and times thirteen. Phi labels record the requested runtime switch. The timing panel has no additional route instrumentation. The independent verifier checked all 264 exports. The temporary instance and storage were removed.

| Shape | Input | Profile | Proof ms | Paired vs prior profile | Paired vs named control | Paired vs Aeglos | Peak MiB | M5 CPU / Metal |
|---|---|---|---:|---|---|---|---:|---:|
| 4x2 | catalog | aeglos | 18.719 | +21.60% (0/6) | off, +21.60% (0/6) | +0.00% (0/6) | 1358.4 | 5.82× / 2.56× |
| 4x2 | catalog | emit | 15.411 | +0.11% (2/6) | off, +0.11% (2/6) | -17.52% (6/6) | 207.2 | 7.07× / 3.11× |
| 4x2 | catalog | emit-phi | 16.541 | +6.86% (0/6) | emit, +7.46% (0/6) | -11.66% (6/6) | 216.2 | 6.59× / 2.90× |
| 4x2 | catalog | epoch12 | 15.896 | +3.45% (0/6) | off, +3.45% (0/6) | -15.04% (6/6) | 206.4 | 6.86× / 3.02× |
| 4x2 | catalog | epoch4 | 15.385 | -1.09% (4/6) | off, -1.09% (4/6) | -18.23% (6/6) | 198.9 | 7.08× / 3.12× |
| 4x2 | catalog | epoch4-emit | 15.538 | +0.53% (2/6) | epoch4, +0.62% (1/6) | -17.41% (6/6) | 199.7 | 7.01× / 3.09× |
| 4x2 | catalog | off | 15.403 | +0.00% (0/6) | off, +0.00% (0/6) | -17.77% (6/6) | 213.2 | 7.08× / 3.12× |
| 4x2 | catalog | packed | 15.992 | +4.03% (0/6) | off, +4.03% (0/6) | -14.05% (6/6) | 205.6 | 6.82× / 3.00× |
| 4x2 | catalog | phi | 16.561 | +7.72% (0/6) | off, +7.72% (0/6) | -11.33% (6/6) | 216.1 | 6.58× / 2.90× |
| 4x2 | heldout | aeglos | 18.473 | +19.53% (0/6) | off, +19.53% (0/6) | +0.00% (0/6) | 1364.8 | — |
| 4x2 | heldout | emit | 15.540 | +0.95% (2/6) | off, +0.95% (2/6) | -16.16% (6/6) | 202.8 | — |
| 4x2 | heldout | emit-phi | 16.491 | +6.86% (0/6) | emit, +6.33% (0/6) | -11.02% (6/6) | 204.9 | — |
| 4x2 | heldout | epoch12 | 15.935 | +3.05% (1/6) | off, +3.05% (1/6) | -13.96% (6/6) | 204.9 | — |
| 4x2 | heldout | epoch4 | 15.396 | -0.62% (4/6) | off, -0.62% (4/6) | -16.86% (6/6) | 203.5 | — |
| 4x2 | heldout | epoch4-emit | 15.435 | -0.47% (4/6) | epoch4, +0.36% (2/6) | -16.59% (6/6) | 201.9 | — |
| 4x2 | heldout | off | 15.428 | +0.00% (0/6) | off, +0.00% (0/6) | -16.34% (6/6) | 206.3 | — |
| 4x2 | heldout | packed | 16.009 | +2.97% (0/6) | off, +2.97% (0/6) | -13.81% (6/6) | 211.6 | — |
| 4x2 | heldout | phi | 16.580 | +7.47% (0/6) | off, +7.47% (0/6) | -10.17% (6/6) | 215.5 | — |
| 4x2 | v153 | emit | 15.493 | -0.62% (2/3) | off, -0.62% (2/3) | — | 201.2 | — |
| 4x2 | v153 | emit-phi | 16.540 | +6.10% (0/3) | emit, +7.35% (0/3) | — | 201.2 | — |
| 4x2 | v153 | off | 15.589 | +0.00% (0/3) | off, +0.00% (0/3) | — | 193.1 | — |
| 4x2 | v153 | phi | 16.664 | +7.19% (0/3) | off, +7.19% (0/3) | — | 208.0 | — |
| 4x2 | v156 | emit | 15.351 | -2.05% (2/3) | off, -2.05% (2/3) | — | 212.4 | — |
| 4x2 | v156 | emit-phi | 16.703 | +6.53% (0/3) | emit, +8.43% (0/3) | — | 213.1 | — |
| 4x2 | v156 | off | 15.639 | +0.00% (0/3) | off, +0.00% (0/3) | — | 207.2 | — |
| 4x2 | v156 | phi | 16.375 | +4.33% (0/3) | off, +4.33% (0/3) | — | 212.4 | — |
| 8x2 | catalog | aeglos | 31.044 | +45.30% (0/6) | off, +45.30% (0/6) | +0.00% (0/6) | 2463.1 | 6.09× / 2.38× |
| 8x2 | catalog | emit | 21.372 | +0.42% (2/6) | off, +0.42% (2/6) | -31.23% (6/6) | 433.3 | 8.84× / 3.46× |
| 8x2 | catalog | emit-phi | 22.720 | +6.42% (0/6) | emit, +5.97% (0/6) | -26.87% (6/6) | 419.7 | 8.32× / 3.26× |
| 8x2 | catalog | epoch12 | 22.080 | +3.79% (1/6) | off, +3.79% (1/6) | -29.46% (6/6) | 412.9 | 8.56× / 3.35× |
| 8x2 | catalog | epoch4 | 21.417 | +0.24% (2/6) | off, +0.24% (2/6) | -31.17% (6/6) | 415.9 | 8.82× / 3.46× |
| 8x2 | catalog | epoch4-emit | 21.192 | -0.80% (6/6) | epoch4, -1.01% (6/6) | -31.57% (6/6) | 402.9 | 8.92× / 3.49× |
| 8x2 | catalog | off | 21.355 | +0.00% (0/6) | off, +0.00% (0/6) | -31.18% (6/6) | 410.1 | 8.85× / 3.47× |
| 8x2 | catalog | packed | 20.818 | -2.55% (5/6) | off, -2.55% (5/6) | -32.77% (6/6) | 426.6 | 9.08× / 3.55× |
| 8x2 | catalog | phi | 22.605 | +5.85% (0/6) | off, +5.85% (0/6) | -27.26% (6/6) | 405.9 | 8.36× / 3.27× |
| 8x2 | heldout | aeglos | 31.390 | +44.64% (0/6) | off, +44.64% (0/6) | +0.00% (0/6) | 2459.2 | — |
| 8x2 | heldout | emit | 21.715 | -0.03% (3/6) | off, -0.03% (3/6) | -30.61% (6/6) | 414.4 | — |
| 8x2 | heldout | emit-phi | 23.145 | +6.06% (0/6) | emit, +5.69% (0/6) | -26.49% (6/6) | 403.2 | — |
| 8x2 | heldout | epoch12 | 22.396 | +3.00% (0/6) | off, +3.00% (0/6) | -28.62% (6/6) | 407.5 | — |
| 8x2 | heldout | epoch4 | 21.496 | -0.93% (5/6) | off, -0.93% (5/6) | -31.70% (6/6) | 413.1 | — |
| 8x2 | heldout | epoch4-emit | 21.655 | -0.53% (3/6) | epoch4, +0.75% (2/6) | -30.58% (6/6) | 411.4 | — |
| 8x2 | heldout | off | 21.742 | +0.00% (0/6) | off, +0.00% (0/6) | -30.86% (6/6) | 424.9 | — |
| 8x2 | heldout | packed | 21.088 | -3.29% (6/6) | off, -3.29% (6/6) | -33.05% (6/6) | 425.9 | — |
| 8x2 | heldout | phi | 23.004 | +6.00% (0/6) | off, +6.00% (0/6) | -26.83% (6/6) | 418.4 | — |
| 8x2 | v153 | emit | 21.766 | -0.22% (2/3) | off, -0.22% (2/3) | — | 395.8 | — |
| 8x2 | v153 | emit-phi | 23.064 | +5.88% (0/3) | emit, +5.96% (0/3) | — | 419.0 | — |
| 8x2 | v153 | off | 21.624 | +0.00% (0/3) | off, +0.00% (0/3) | — | 412.8 | — |
| 8x2 | v153 | phi | 22.985 | +5.58% (0/3) | off, +5.58% (0/3) | — | 412.9 | — |
| 8x2 | v156 | emit | 21.705 | +0.31% (1/3) | off, +0.31% (1/3) | — | 401.8 | — |
| 8x2 | v156 | emit-phi | 23.036 | +6.84% (0/3) | emit, +5.75% (0/3) | — | 407.0 | — |
| 8x2 | v156 | off | 21.639 | +0.00% (0/3) | off, +0.00% (0/3) | — | 407.8 | — |
| 8x2 | v156 | phi | 22.908 | +5.87% (0/3) | off, +5.87% (0/3) | — | 405.6 | — |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json) · [Previous M5 comparison](../2026-11-19-frontend-k-c7a757/README.md)
## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU on measured C7a profiles. [Previous comparison](../2026-11-19-frontend-k-c7a757/README.md)

| Shape | Configuration | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 737 dedicated G1 80+16 | 7.953 | 37 / 24 | 4.65× / 3.02× | +4.253 |
| 2×2 | 750 archived-band 72+24 | 12.358 | 59 / 30 | 4.77× / 2.43× | +6.458 |
| 4×2 | 757 solve-front-on | 15.257 | 109 / 48 | 7.14× / 3.15× | +4.357 |
| 8×2 | 758 packed | 20.818 | 189 / 74 | 9.08× / 3.55× | +1.918 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
