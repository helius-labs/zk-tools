# Folding the first inverse FFT butterfly into input packing

The input-column packer uses the public polynomial lengths to identify padded rows. When only the upper operand of the first inverse butterfly is zero, the lower output equals the input and the upper output needs only the existing twiddle multiplication. If both operands are zero, their outputs are initialized to zero. The other four radix-32 layers keep their usual arithmetic. Native transform tests compared the complete inverse result for empty, full, unequal and partial input lengths with both root layouts. Each experimental setting passed the same oracle.

Five randomized blocks compared a frozen predecessor, the new executable with folding disabled, and the same executable with folding enabled. Each process warmed three proofs and timed eleven full calls. The 12×2 tests use the earlier two-stage B2 plan on 30 workers. The other shapes use their adaptive groups on 32. Peak RSS includes setup.

| Shape | Request | Frozen / disabled / enabled ms | Enabled vs disabled | Enabled vs frozen | CPU-ms disabled / enabled | Peak MiB disabled / enabled |
|---|---|---:|---:|---:|---:|---:|
| 12x2 | Original | 29.439 / 29.653 / 29.936 | +0.13% (2/5) | +0.78% (2/5) | 767.6 / 774.3 | 579.9 / 583.0 |
| 12x2 | 59 | 29.669 / 29.876 / 29.364 | -1.74% (4/5) | -0.32% (3/5) | 769.6 / 767.7 | 579.0 / 567.0 |
| 12x2 | 141 | 45.305 / 45.434 / 45.170 | -1.03% (3/5) | +0.23% (2/5) | 1183.1 / 1180.9 | 967.7 / 962.8 |
| 16x2 | Original | 37.027 / 37.426 / 38.370 | +2.52% (1/5) | +2.37% (1/5) | 1048.7 / 1055.7 | 757.0 / 752.4 |
| 16x2 | 79 | 37.183 / 37.072 / 36.904 | -0.45% (4/5) | -1.01% (4/5) | 1031.4 / 1026.5 | 757.1 / 753.8 |
| 24x2 | Original | 59.480 / 59.803 / 59.568 | +0.06% (2/5) | +0.01% (2/5) | 1705.7 / 1694.2 | 1117.9 / 1128.3 |
| 24x2 | 103 | 58.337 / 57.755 / 58.473 | +0.53% (2/5) | +0.21% (2/5) | 1634.0 / 1643.7 | 1137.9 / 1163.3 |

Original inputs, speedups against M5 CPU / Metal: 12x2 9.49× / 3.37×, 16x2 8.73× / 3.23×, 24x2 8.86× / 2.95×. All 105 process exports verified independently. [Samples](artifacts/processes.json.gz) · [Source and oracle](artifacts/incremental.patch) · [Verification](artifacts/verification.json)
