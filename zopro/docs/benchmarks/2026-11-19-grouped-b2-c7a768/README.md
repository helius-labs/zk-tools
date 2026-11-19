# Grouped B2 window layouts on C7a

Zopro ran on a single c7a.24xlarge with 64 main cores and 32 frontend cores. All Zopro arms used the fused quotient FFT and late frontend K. The 8×2 arms also used the previously selected solver packing and the 4×2 arms used the frontend solver. Prepared Aeglos used 96 cores. Inputs and proving keys were pinned.

The trial changes the signed or unsigned window width and point partitions in the final grouped G2 MSM. It retains the ordinary exact scalar classification and point reduction. The release, disabled trial and explicit baseline layout control for binary and dispatch differences. The trial charges the estimated temporary workspace and falls back to the ordinary path on denial. There is no persistent group-point table.

- **4×2.** The lowest relative-time layout was `u12x2` at 15.836 / 15.992 ms across original and second inputs. Released Zopro took 15.511 / 15.531 ms and prepared Aeglos took 18.296 / 18.132 ms. Peak RSS was 222.3 / 217.7 MiB for the layout and 195.9 / 196.6 MiB for released Zopro. The existing reference remains the fastest qualifying measurement.
- **8×2.** The lowest relative-time layout was `s13x3` at 20.950 / 21.114 ms across original and second inputs. Released Zopro took 20.922 / 20.979 ms and prepared Aeglos took 31.180 / 31.209 ms. Peak RSS was 428.9 / 425.0 MiB for the layout and 422.2 / 422.1 MiB for released Zopro. The existing reference remains the fastest qualifying measurement.

## B2 attribution

The table gives medians of process means on the ordinary layout. Each process discarded three warmups and timed nine requests. The final columns compare matched runs with and without the clocks. These B2 intervals overlap with other curve work. The candidate layout route probes used only one timed proof per process and serve as routing evidence.

| Shape | Input | Terms | Nonzero scalar classes | Classification ms | Point reduction ms | Final MSM ms | Plain / traced proof ms | Paired traced vs plain |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| 4×2 | catalog | 95537 | 29738 | 3.538 | 0.751 | 2.836 | 15.395 / 15.549 | +0.59% (1/4) |
| 4×2 | heldout | 95537 | 29961 | 3.641 | 0.796 | 2.868 | 15.440 / 15.647 | +1.34% (1/4) |
| 8×2 | catalog | 183347 | 37850 | 4.818 | 1.961 | 3.657 | 20.818 / 20.949 | +0.70% (2/4) |
| 8×2 | heldout | 183347 | 39666 | 4.531 | 2.230 | 3.862 | 21.268 / 21.194 | -0.79% (2/4) |

## Full-proof timing

The primary 8×2 inputs use six matched blocks and the 4×2 inputs use four. Two additional 8×2 transactions use four blocks. Each process discarded three priming requests and measured thirteen. Peak RSS is the highest process value over the blocks. Phase diagnostics were disabled during the timing panel. Each signed and unsigned configuration passed the native G2 oracle. All 288 C7a and 18 Ryzen proof exports passed independent verification, including the zero optional-preparation fallback. The temporary instance, volume, and bucket were removed.

| Shape | Input | Profile | Proof ms | Paired vs release | Paired vs disabled trial | Paired vs baseline layout | Paired vs Aeglos | Peak MiB | M5 CPU / Metal |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|
| 4×2 | catalog | aeglos | 18.296 | +17.94% (0/4) | +17.96% (0/4) | +19.68% (0/4) | +0.00% (0/4) | 1348.7 | 5.96× / 2.62× |
| 4×2 | catalog | b2-base | 15.287 | -1.44% (4/4) | -1.44% (4/4) | +0.00% (0/4) | -16.44% (4/4) | 209.4 | 7.13× / 3.14× |
| 4×2 | catalog | off | 15.510 | -0.02% (2/4) | +0.00% (0/4) | +1.46% (0/4) | -15.23% (4/4) | 199.6 | 7.03× / 3.09× |
| 4×2 | catalog | release | 15.511 | +0.00% (0/4) | +0.02% (2/4) | +1.46% (0/4) | -15.21% (4/4) | 195.9 | 7.03× / 3.09× |
| 4×2 | catalog | s13x2 | 15.964 | +2.99% (0/4) | +3.94% (1/4) | +4.57% (0/4) | -12.54% (4/4) | 220.0 | 6.83× / 3.01× |
| 4×2 | catalog | u12x1 | 16.594 | +6.97% (0/4) | +6.25% (0/4) | +8.55% (0/4) | -9.29% (4/4) | 233.4 | 6.57× / 2.89× |
| 4×2 | catalog | u12x2 | 15.836 | +2.10% (0/4) | +1.06% (1/4) | +3.49% (0/4) | -14.33% (4/4) | 222.3 | 6.88× / 3.03× |
| 4×2 | heldout | aeglos | 18.132 | +16.75% (0/4) | +16.72% (0/4) | +16.72% (0/4) | +0.00% (0/4) | 1349.7 | — |
| 4×2 | heldout | b2-base | 15.491 | -0.56% (3/4) | -0.73% (3/4) | +0.00% (0/4) | -14.33% (4/4) | 213.1 | — |
| 4×2 | heldout | off | 15.561 | +0.34% (1/4) | +0.00% (0/4) | +0.74% (1/4) | -14.33% (4/4) | 199.8 | — |
| 4×2 | heldout | release | 15.531 | +0.00% (0/4) | -0.34% (3/4) | +0.57% (1/4) | -14.35% (4/4) | 196.6 | — |
| 4×2 | heldout | s13x2 | 16.056 | +3.34% (0/4) | +2.88% (0/4) | +4.15% (0/4) | -11.20% (4/4) | 223.0 | — |
| 4×2 | heldout | u12x1 | 16.640 | +7.58% (0/4) | +6.94% (0/4) | +7.59% (0/4) | -7.97% (4/4) | 203.5 | — |
| 4×2 | heldout | u12x2 | 15.992 | +2.97% (0/4) | +2.82% (0/4) | +2.51% (0/4) | -12.17% (4/4) | 217.7 | — |
| 8×2 | catalog | aeglos | 31.180 | +48.53% (0/6) | +50.54% (0/6) | +51.15% (0/6) | +0.00% (0/6) | 2499.9 | 6.06× / 2.37× |
| 8×2 | catalog | b2-base | 20.684 | -1.33% (5/6) | -0.29% (4/6) | +0.00% (0/6) | -33.84% (6/6) | 423.6 | 9.14× / 3.58× |
| 8×2 | catalog | off | 20.686 | -1.59% (5/6) | +0.00% (0/6) | +0.29% (2/6) | -33.57% (6/6) | 409.2 | 9.14× / 3.58× |
| 8×2 | catalog | release | 20.922 | +0.00% (0/6) | +1.62% (1/6) | +1.36% (1/6) | -32.67% (6/6) | 422.2 | 9.03× / 3.54× |
| 8×2 | catalog | s12x2 | 20.876 | -0.22% (5/6) | +1.49% (1/6) | +1.27% (0/6) | -33.02% (6/6) | 416.3 | 9.05× / 3.54× |
| 8×2 | catalog | s13x2 | 21.109 | +1.23% (1/6) | +2.52% (0/6) | +2.14% (0/6) | -31.87% (6/6) | 429.5 | 8.95× / 3.51× |
| 8×2 | catalog | s13x3 | 20.950 | +0.39% (3/6) | +1.31% (1/6) | +1.31% (0/6) | -32.56% (6/6) | 428.9 | 9.02× / 3.53× |
| 8×2 | catalog | u11x3 | 21.246 | +1.53% (1/6) | +3.30% (0/6) | +2.95% (0/6) | -31.70% (6/6) | 407.1 | 8.90× / 3.48× |
| 8×2 | catalog | u12x1 | 22.319 | +6.29% (0/6) | +7.69% (0/6) | +7.51% (0/6) | -28.59% (6/6) | 418.3 | 8.47× / 3.32× |
| 8×2 | catalog | u12x2 | 21.045 | +0.21% (3/6) | +1.77% (0/6) | +2.03% (0/6) | -32.36% (6/6) | 420.4 | 8.98× / 3.52× |
| 8×2 | catalog | u13x1 | 22.399 | +6.75% (0/6) | +7.98% (0/6) | +8.31% (0/6) | -28.16% (6/6) | 432.1 | 8.44× / 3.30× |
| 8×2 | heldout | aeglos | 31.209 | +48.33% (0/6) | +47.79% (0/6) | +47.78% (0/6) | +0.00% (0/6) | 2495.1 | — |
| 8×2 | heldout | b2-base | 21.055 | -0.07% (3/6) | -0.04% (3/6) | +0.00% (0/6) | -32.33% (6/6) | 415.2 | — |
| 8×2 | heldout | off | 20.991 | +0.04% (3/6) | +0.00% (0/6) | +0.04% (3/6) | -32.34% (6/6) | 394.2 | — |
| 8×2 | heldout | release | 20.979 | +0.00% (0/6) | -0.04% (3/6) | +0.07% (3/6) | -32.58% (6/6) | 422.1 | — |
| 8×2 | heldout | s12x2 | 21.241 | +1.05% (1/6) | +0.75% (1/6) | +1.11% (1/6) | -32.14% (6/6) | 419.0 | — |
| 8×2 | heldout | s13x2 | 21.123 | +0.69% (0/6) | +0.78% (2/6) | +0.67% (3/6) | -32.37% (6/6) | 407.2 | — |
| 8×2 | heldout | s13x3 | 21.114 | +0.64% (1/6) | +0.94% (2/6) | -0.08% (3/6) | -32.09% (6/6) | 425.0 | — |
| 8×2 | heldout | u11x3 | 21.727 | +3.31% (0/6) | +3.84% (1/6) | +3.29% (0/6) | -30.62% (6/6) | 415.8 | — |
| 8×2 | heldout | u12x1 | 22.611 | +7.79% (0/6) | +7.69% (0/6) | +8.15% (0/6) | -27.82% (6/6) | 425.9 | — |
| 8×2 | heldout | u12x2 | 21.189 | +0.81% (2/6) | +1.00% (2/6) | +0.61% (0/6) | -31.66% (6/6) | 431.0 | — |
| 8×2 | heldout | u13x1 | 22.559 | +7.09% (0/6) | +7.36% (0/6) | +6.76% (0/6) | -27.53% (6/6) | 421.3 | — |
| 8×2 | v153 | off | 20.880 | -0.23% (3/4) | +0.00% (0/4) | — | — | 395.9 | — |
| 8×2 | v153 | release | 20.928 | +0.00% (0/4) | +0.23% (1/4) | — | — | 397.8 | — |
| 8×2 | v153 | s13x2 | 21.376 | +1.63% (0/4) | +2.45% (1/4) | — | — | 416.1 | — |
| 8×2 | v153 | u12x1 | 22.373 | +6.91% (0/4) | +6.84% (0/4) | — | — | 405.4 | — |
| 8×2 | v156 | off | 20.946 | -1.36% (3/4) | +0.00% (0/4) | — | — | 401.2 | — |
| 8×2 | v156 | release | 21.139 | +0.00% (0/4) | +1.38% (1/4) | — | — | 419.7 | — |
| 8×2 | v156 | s13x2 | 21.026 | -0.64% (2/4) | +0.76% (0/4) | — | — | 419.2 | — |
| 8×2 | v156 | u12x1 | 22.616 | +6.47% (0/4) | +7.83% (0/4) | — | — | 410.1 | — |

[Samples](artifacts/rows.json.gz) · [Phase and timing summary](artifacts/summary.json) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU on the measured C7a profiles. [Previous comparison](../2026-11-19-solver-grain-c7a764/README.md)

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
