# Skipping butterflies on known zero-padded FFT inputs

For the first inverse radix-32 columns, the input length determines which rows are zero. The candidate skips operations with two absent operands and computes only the twiddle product when the upper operand is absent. The remaining transform uses the existing kernel. Cached and generated roots were tested across all 33 possible prefix lengths against the ordinary transform. These are verified 32-thread Ryzen SDK requests, with the previously qualified per-shape FFT settings on every arm. Each process warmed up three requests and timed eleven.

| Shape | FFT domain / occupied rows | Parent / disabled / enabled ms | Paired vs parent (wins) | Paired vs disabled (wins) | CPU-ms disabled / enabled | Peak MiB disabled / enabled | M5 CPU / Metal speedup |
|---|---:|---:|---:|---:|---:|---:|---:|
| 12x2 | 524,288 / 280,242 | 32.157 / 32.392 / 32.032 | -0.08% (4/7) | -1.02% (4/7) | 891.8 / 887.0 | 627.0 / 610.8 | 8.87× / 3.15× |
| 16x2 | 524,288 / 371,122 | 39.847 / 39.987 / 39.935 | +0.05% (3/7) | -0.55% (4/7) | 1152.7 / 1155.9 | 739.5 / 742.0 | 8.39× / 3.11× |
| 24x2 | 1,048,576 / 554,226 | 67.852 / 67.370 / 67.706 | -0.94% (4/7) | +0.50% (3/7) | 1949.5 / 1960.0 | 1117.9 / 1112.2 | 7.80× / 2.60× |
| merge-24x1 | 1,048,576 / 525,759 | 64.990 / 64.715 / 64.826 | +0.40% (2/7) | +0.05% (2/7) | 1853.3 / 1861.0 | 1028.2 / 1039.5 | 7.82× / 2.70× |

No shape met the initial full-proof criterion against both controls.

[Prototype patch and arithmetic test](artifacts/source.patch) · [Process samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [Protocol](artifacts/protocol.md)
