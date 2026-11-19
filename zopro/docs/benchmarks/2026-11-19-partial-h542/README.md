# Selective splitting of signed H-query windows

The signed H MSM recodes each scalar into eighteen windows. A window can use one complete range of points or two independently filled ranges followed by a bucket merge. The experiment splits only the three 15-bit windows, or every other window, to create 21 or 27 fill tasks. One and two ranges for every window supply controls. The optional memory reservation still covers the fully split layout before admitting either partial split and can fall back to one range. Generator-sum and digit reconstruction tests exercise all four policies. Separate SDK probes count the windows that actually split.

All configurations use the same point groups and direct B2 scalar view. Five randomized blocks cover an original and a separate public request per shape at 32 Ryzen workers. Each process performs three warmups followed by eleven timed proofs. The frozen build and same-build off mode use the earlier per-shape H implementation.

| Shape | Input | Frozen / ordinary / one / two / wide / alternate ms | Paired wide and alternate vs one, then vs frozen | Peak MiB in mode order |
|---|---|---:|---|---:|
| 12x2 | Original | 29.383 / 29.176 / 30.459 / 30.085 / 30.328 / 29.982 | -0.04% (3/5) / -1.72% (3/5) / +3.22% (1/5) / +3.82% (2/5) | 542 / 532 / 541 / 535 / 539 / 553 |
| 24x2 | Original | 60.494 / 59.164 / 57.615 / 58.176 / 57.877 / 58.589 | +0.23% (1/5) / +2.21% (1/5) / -4.40% (5/5) / -2.83% (5/5) | 1105 / 1102 / 1112 / 1105 / 1147 / 1120 |
| 12x2 | 142 | 45.460 / 46.952 / 46.959 / 51.237 / 47.307 / 48.410 | +1.22% (2/5) / +2.43% (1/5) / +2.71% (0/5) / +6.31% (0/5) | 993 / 987 / 980 / 967 / 1000 / 1003 |
| 24x2 | 103 | 57.897 / 59.073 / 57.258 / 58.466 / 57.770 / 56.304 | +2.74% (1/5) / -0.70% (3/5) / -0.22% (3/5) / -0.94% (3/5) | 1109 / 1128 / 1101 / 1166 / 1081 / 1094 |

Original wide-split speedups against M5 CPU and Metal were 12x2 9.36× / 3.33×, 24x2 9.12× / 3.04×. All 128 exports verified. [Samples](artifacts/timings.json.gz) · [Routes](artifacts/route.json) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)
