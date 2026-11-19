# Grouped B2 scalar classification on C7a

Zopro ran on a single c7a.24xlarge with 64 main cores and 32 frontend cores. All Zopro arms used the fused quotient FFT and late frontend K. The 8×2 arms also used the previously selected solver packing and the 4×2 arms used the frontend solver. Prepared Aeglos used 96 cores. Inputs and proving keys were pinned.

The trial varies the initial hash-table capacity for B2 and the policy for converting distinct scalars out of Montgomery form. Reserve candidates request one-third or one-quarter of the B2 term count, rounded up and bounded by the existing grouping cutoff. Hashbrown may round different requests to the same physical capacity. Serial conversion retains the same field operation. The output grouping order, point reduction, window geometry, and memory admission remain the same.

- **4×2.** The lowest relative-time candidate was `reserve3-serial` at 15.471 / 15.420 ms across original and second inputs. Released Zopro took 15.722 / 15.521 ms and prepared Aeglos took 18.794 / 18.447 ms. Peak RSS was 213.2 / 210.1 MiB for the candidate and 204.9 / 199.1 MiB for released Zopro. The existing reference remains the fastest qualifying measurement.
- **8×2.** The lowest relative-time candidate was `reserve3-serial` at 20.510 / 20.872 ms across original and second inputs. Released Zopro took 20.826 / 21.193 ms and prepared Aeglos took 31.421 / 31.545 ms. Peak RSS was 430.2 / 400.8 MiB for the candidate and 418.5 / 413.9 MiB for released Zopro. `reserve3-serial` improves the previous reference and won the paired comparison against all three Zopro controls on both inputs.

## B2 classification attribution

The table gives medians of process means on the ordinary configuration. Each traced process used three warmups and nine measured requests. Hash time includes zeroing the class vector, allocating the table, inserting class representatives and dropping the table. List construction includes prefix offsets and index scatter. The last column compares full-proof latency with and without the clocks over four matched blocks. These intervals can include time when the proof thread is descheduled or executes work stolen from another task. The machine-readable summary also includes the matched two-block traces of the reserve and conversion candidates.

| Shape | Input | Requested hash capacity | Sample ms | Admission ms | Hash pass ms | Index lists ms | Conversion ms | Total classification ms | Paired traced vs plain |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 4×2 | catalog | 28832 | 0.041 | 0.459 | 1.974 | 0.340 | 0.762 | 3.610 | -1.31% (3/4) |
| 4×2 | heldout | 27856 | 0.039 | 0.484 | 2.067 | 0.284 | 0.727 | 3.570 | +0.50% (1/4) |
| 8×2 | catalog | 30096 | 0.041 | 0.337 | 2.909 | 0.477 | 1.100 | 4.908 | +0.34% (1/4) |
| 8×2 | heldout | 30080 | 0.039 | 0.314 | 2.900 | 0.491 | 0.973 | 4.752 | -0.79% (2/4) |

## Full-proof timing

The primary 8×2 inputs use six matched blocks and the 4×2 inputs use four. Two additional 8×2 transactions use four blocks. Each process discarded three priming requests and measured thirteen. Peak RSS is the highest process value over the blocks. Phase diagnostics were disabled during the timing panel. All tuned grouping configurations matched the native full-group metadata oracle. All 288 C7a and 12 Ryzen proof exports passed independent verification, including the zero optional-preparation fallback. The temporary instance, volume, and bucket were removed.

| Shape | Input | Profile | Proof ms | Paired vs release | Paired vs disabled trial | Paired vs baseline layout | Paired vs Aeglos | Peak MiB | M5 CPU / Metal |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|
| 4×2 | catalog | aeglos | 18.794 | +19.10% (0/4) | +20.86% (0/4) | +20.59% (0/4) | +0.00% (0/4) | 1348.7 | 5.80× / 2.55× |
| 4×2 | catalog | b2-base | 15.586 | -0.54% (3/4) | +0.41% (1/4) | +0.00% (0/4) | -17.06% (4/4) | 201.2 | 6.99× / 3.08× |
| 4×2 | catalog | off | 15.552 | -1.22% (3/4) | +0.00% (0/4) | -0.40% (3/4) | -17.25% (4/4) | 193.0 | 7.01× / 3.09× |
| 4×2 | catalog | release | 15.722 | +0.00% (0/4) | +1.23% (1/4) | +0.55% (1/4) | -16.03% (4/4) | 204.9 | 6.93× / 3.05× |
| 4×2 | catalog | reserve3 | 15.503 | -1.13% (3/4) | -0.61% (3/4) | -0.83% (3/4) | -17.29% (4/4) | 208.7 | 7.03× / 3.10× |
| 4×2 | catalog | reserve3-serial | 15.471 | -2.52% (3/4) | -0.65% (3/4) | -1.06% (3/4) | -17.22% (4/4) | 213.2 | 7.05× / 3.10× |
| 4×2 | catalog | reserve4 | 15.774 | -0.03% (2/4) | +1.46% (0/4) | +1.01% (2/4) | -16.18% (4/4) | 209.4 | 6.91× / 3.04× |
| 4×2 | catalog | serial | 15.436 | -1.75% (4/4) | -0.74% (3/4) | -0.96% (3/4) | -17.86% (4/4) | 208.7 | 7.06× / 3.11× |
| 4×2 | heldout | aeglos | 18.447 | +18.42% (0/4) | +17.96% (0/4) | +20.00% (0/4) | +0.00% (0/4) | 1362.2 | — |
| 4×2 | heldout | b2-base | 15.543 | +0.63% (1/4) | -0.53% (3/4) | +0.00% (0/4) | -16.67% (4/4) | 215.5 | — |
| 4×2 | heldout | off | 15.652 | +0.29% (1/4) | +0.00% (0/4) | +0.54% (1/4) | -15.21% (4/4) | 201.9 | — |
| 4×2 | heldout | release | 15.521 | +0.00% (0/4) | -0.29% (3/4) | -0.63% (3/4) | -15.53% (4/4) | 199.1 | — |
| 4×2 | heldout | reserve3 | 15.503 | +0.23% (1/4) | -0.79% (3/4) | -0.26% (3/4) | -16.55% (4/4) | 196.1 | — |
| 4×2 | heldout | reserve3-serial | 15.420 | -0.85% (3/4) | -1.48% (3/4) | -0.59% (2/4) | -16.27% (4/4) | 210.1 | — |
| 4×2 | heldout | reserve4 | 15.587 | +0.11% (2/4) | -0.42% (3/4) | +0.35% (1/4) | -15.50% (4/4) | 206.4 | — |
| 4×2 | heldout | serial | 15.518 | -0.73% (3/4) | -0.58% (3/4) | -0.44% (2/4) | -15.74% (4/4) | 197.6 | — |
| 8×2 | catalog | aeglos | 31.421 | +49.86% (0/6) | +50.99% (0/6) | +50.50% (0/6) | +0.00% (0/6) | 2459.9 | 6.02× / 2.36× |
| 8×2 | catalog | b2-base | 20.753 | -0.22% (5/6) | -1.00% (4/6) | +0.00% (0/6) | -33.56% (6/6) | 417.8 | 9.11× / 3.57× |
| 8×2 | catalog | off | 20.905 | +0.11% (3/6) | +0.00% (0/6) | +1.01% (2/6) | -33.77% (6/6) | 416.9 | 9.04× / 3.54× |
| 8×2 | catalog | release | 20.826 | +0.00% (0/6) | -0.11% (3/6) | +0.22% (1/6) | -33.27% (6/6) | 418.5 | 9.08× / 3.55× |
| 8×2 | catalog | reserve3 | 21.020 | +0.64% (1/6) | +0.76% (2/6) | +1.32% (2/6) | -32.99% (6/6) | 420.3 | 8.99× / 3.52× |
| 8×2 | catalog | reserve3-serial | 20.510 | -1.18% (6/6) | -2.04% (5/6) | -0.99% (5/6) | -34.51% (6/6) | 430.2 | 9.22× / 3.61× |
| 8×2 | catalog | reserve4 | 20.774 | -0.57% (4/6) | -0.18% (3/6) | -0.24% (4/6) | -33.93% (6/6) | 416.1 | 9.10× / 3.56× |
| 8×2 | catalog | serial | 20.708 | -0.57% (6/6) | -1.15% (4/6) | -0.34% (4/6) | -33.94% (6/6) | 418.1 | 9.13× / 3.57× |
| 8×2 | heldout | aeglos | 31.545 | +48.94% (0/6) | +49.17% (0/6) | +50.33% (0/6) | +0.00% (0/6) | 2458.5 | — |
| 8×2 | heldout | b2-base | 21.037 | -0.63% (3/6) | -0.38% (4/6) | +0.00% (0/6) | -33.48% (6/6) | 414.6 | — |
| 8×2 | heldout | off | 21.158 | -0.19% (4/6) | +0.00% (0/6) | +0.38% (2/6) | -32.96% (6/6) | 413.1 | — |
| 8×2 | heldout | release | 21.193 | +0.00% (0/6) | +0.19% (2/6) | +0.64% (3/6) | -32.86% (6/6) | 413.9 | — |
| 8×2 | heldout | reserve3 | 21.154 | -0.11% (3/6) | +0.17% (3/6) | +0.57% (2/6) | -32.94% (6/6) | 422.8 | — |
| 8×2 | heldout | reserve3-serial | 20.872 | -1.46% (6/6) | -1.23% (5/6) | -0.50% (5/6) | -33.71% (6/6) | 400.8 | — |
| 8×2 | heldout | reserve4 | 21.057 | -0.85% (4/6) | -0.43% (4/6) | +0.29% (2/6) | -33.58% (6/6) | 409.9 | — |
| 8×2 | heldout | serial | 20.842 | -1.43% (6/6) | -1.36% (5/6) | -0.98% (6/6) | -33.98% (6/6) | 418.8 | — |
| 8×2 | v153 | b2-base | 21.106 | +0.05% (2/4) | +0.13% (2/4) | +0.00% (0/4) | — | 420.5 | — |
| 8×2 | v153 | off | 21.027 | +0.05% (2/4) | +0.00% (0/4) | -0.13% (2/4) | — | 415.0 | — |
| 8×2 | v153 | release | 21.072 | +0.00% (0/4) | -0.05% (2/4) | -0.05% (2/4) | — | 398.0 | — |
| 8×2 | v153 | reserve3 | 21.273 | +0.86% (0/4) | +1.13% (1/4) | +0.63% (1/4) | — | 422.8 | — |
| 8×2 | v153 | serial | 20.883 | -0.92% (3/4) | -0.47% (3/4) | -0.85% (4/4) | — | 432.4 | — |
| 8×2 | v156 | b2-base | 21.047 | -0.06% (2/4) | +0.11% (2/4) | +0.00% (0/4) | — | 407.8 | — |
| 8×2 | v156 | off | 21.085 | +0.03% (2/4) | +0.00% (0/4) | -0.10% (2/4) | — | 400.0 | — |
| 8×2 | v156 | release | 21.059 | +0.00% (0/4) | -0.02% (2/4) | +0.06% (2/4) | — | 407.6 | — |
| 8×2 | v156 | reserve3 | 21.092 | +0.12% (2/4) | -0.04% (2/4) | +0.12% (2/4) | — | 398.7 | — |
| 8×2 | v156 | serial | 20.979 | -0.68% (3/4) | -0.45% (2/4) | -0.70% (3/4) | — | 420.5 | — |

[Samples](artifacts/rows.json.gz) · [Phase and timing summary](artifacts/summary.json) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU on the measured C7a profiles. [Previous comparison](../2026-11-19-grouped-b2-c7a768/README.md)

| Shape | Configuration | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 737 dedicated G1 80+16 | 7.953 | 37 / 24 | 4.65× / 3.02× | +4.253 |
| 2×2 | 750 archived-band 72+24 | 12.358 | 59 / 30 | 4.77× / 2.43× | +6.458 |
| 4×2 | 757 solve-front-on | 15.257 | 109 / 48 | 7.14× / 3.15× | +4.357 |
| 8×2 | 771 B2 classification reserve3-serial | 20.510 | 189 / 74 | 9.22× / 3.61× | +1.610 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
