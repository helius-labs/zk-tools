# Bounded rejection before constructing the canonical scalar dictionary

The two 1×2 diagnostic exports checked the 21,006-element admission on the exact candidate with the diversity setting enabled and disabled. The timing uses the shape's normal server settings with the same release binaries and fixtures throughout.

| Shape | Input | Parent / indexed / checked index ms | Checked vs parent / indexed | Peaks in same order, MiB | Checked vs M5 CPU / Metal |
|---|---|---:|---:|---:|---:|
| 1x2 | Original | 6.973 / 7.067 / 6.927 | -1.30% (4/6) / -1.79% (6/6) | 62.2 / 60.5 / 60.8 | 5.34× / 3.46× |
| 1x2 | 29 | 6.878 / 6.950 / 6.864 | +0.37% (2/6) / -0.60% (5/6) | 62.2 / 60.5 / 57.8 | — |
| 1x2 | 32 | 6.898 / 6.974 / 6.908 | +0.21% (2/6) / -0.77% (4/6) | 62.0 / 61.2 / 56.8 | — |
| 2x2 | Original | 9.591 / 9.611 / 9.604 | -0.38% (3/6) / -0.40% (4/6) | 106.5 / 105.2 / 106.8 | 6.14× / 3.12× |
| 4x2 | Original | 15.546 / 15.266 / 15.170 | -1.63% (6/6) / -0.22% (4/6) | 256.8 / 241.8 / 244.5 | 7.19× / 3.16× |

All 126 exports verified independently, including the initial probe and three debugger census exports. The original request missed its preregistered rejection criterion. [Exact current census](artifacts/census.json) [Protocol](protocol.md) · [Admission probes](artifacts/probes.json) · [Samples and profiles](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
