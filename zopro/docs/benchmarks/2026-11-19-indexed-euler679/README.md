# Transposed Euler butterflies with indexed output and caller-owned storage

This build retains the indexed SDK's preallocated FFT output and its separate interior transform entry point. Four uses the transposed four-quad Euler center. Tile also brings the two neighboring inverse and forward layers into the same sixteen-point block. It loads the corresponding fixed-size root vectors before visiting the independent blocks and uses the existing expanded scratch buffer. Frozen is the original indexed executable. Off, four and tile share an executable. Native equivalence tests compare the complete transform with independently applied inverse, scalar diagonal and forward operations.

Each input ran four arms at its established physical placement, eight counterbalanced blocks on the original fixtures and four on each held-out witness. Separate diagnostic exports confirmed the tiled entry and independent profiles covered each arm. All 158 proof exports verified against the original keys and expected public inputs. The table uses medians of warm process means, within-block latency ratios and maximum observed process RSS.

| Input | Mode | Warm ms | Paired vs off (wins) | Paired vs frozen (wins) | CPU-ms/proof | Peak MiB | M5 CPU / Metal |
|---|---|---:|---:|---:|---:|---:|---:|
| 24x2-0 | frozen | 56.825 | -0.60% (5/8) | +0.00% (0/8) | 1585.8 | 958.4 | 9.29× / 3.10× |
| 24x2-0 | off | 57.146 | +0.00% (0/8) | +0.60% (3/8) | 1581.8 | 985.7 | 9.24× / 3.08× |
| 24x2-0 | four | 57.306 | -0.23% (4/8) | +0.65% (3/8) | 1583.2 | 970.7 | 9.21× / 3.07× |
| 24x2-0 | tile | 56.461 | -1.20% (5/8) | -1.11% (6/8) | 1586.2 | 957.3 | 9.35× / 3.12× |
| 12x2-0 | frozen | 28.839 | +1.05% (2/8) | +0.00% (0/8) | 746.1 | 559.2 | 9.85× / 3.50× |
| 12x2-0 | off | 28.683 | +0.00% (0/8) | -1.04% (6/8) | 742.3 | 561.6 | 9.90× / 3.52× |
| 12x2-0 | four | 28.456 | -0.02% (4/8) | -1.50% (6/8) | 739.8 | 562.0 | 9.98× / 3.55× |
| 12x2-0 | tile | 28.516 | -0.18% (6/8) | -0.81% (6/8) | 740.5 | 556.5 | 9.96× / 3.54× |
| 24x2-97 | frozen | 55.996 | +1.54% (0/4) | +0.00% (0/4) | 1558.7 | 943.9 | — |
| 24x2-97 | off | 55.182 | +0.00% (0/4) | -1.50% (4/4) | 1547.4 | 896.4 | — |
| 24x2-97 | four | 55.702 | +0.50% (1/4) | +0.22% (1/4) | 1554.0 | 942.6 | — |
| 24x2-97 | tile | 55.312 | +0.36% (2/4) | -1.65% (3/4) | 1546.6 | 907.4 | — |
| 12x2-58 | frozen | 29.105 | +0.45% (1/4) | +0.00% (0/4) | 747.2 | 562.8 | — |
| 12x2-58 | off | 28.928 | +0.00% (0/4) | -0.44% (3/4) | 749.9 | 553.8 | — |
| 12x2-58 | four | 28.918 | -0.41% (2/4) | -0.34% (2/4) | 743.5 | 586.0 | — |
| 12x2-58 | tile | 28.920 | -0.18% (2/4) | -0.66% (3/4) | 750.1 | 554.8 | — |
| 24x2-99 | frozen | 55.411 | -0.15% (3/4) | +0.00% (0/4) | 1550.4 | 946.4 | — |
| 24x2-99 | off | 55.597 | +0.00% (0/4) | +0.15% (1/4) | 1545.0 | 949.4 | — |
| 24x2-99 | four | 55.584 | -0.29% (3/4) | -0.36% (3/4) | 1548.2 | 949.0 | — |
| 24x2-99 | tile | 55.842 | +0.23% (2/4) | +0.42% (2/4) | 1548.7 | 924.6 | — |
| 24x2-101 | frozen | 55.185 | -0.78% (3/4) | +0.00% (0/4) | 1555.3 | 907.0 | — |
| 24x2-101 | off | 55.820 | +0.00% (0/4) | +0.79% (1/4) | 1557.3 | 923.7 | — |
| 24x2-101 | four | 55.097 | -1.29% (3/4) | -0.08% (2/4) | 1552.9 | 921.6 | — |
| 24x2-101 | tile | 55.613 | +0.31% (2/4) | +0.77% (1/4) | 1558.4 | 936.8 | — |

The tile kernel lowered the original 24×2 paired median by 1.20% against the same executable with the option disabled and by 1.11% against the previous indexed build. On all three additional 24×2 witnesses it split the four pairs evenly against disabled, with paired median costs of 0.23–0.36%. The original 12×2 tile median was 28.516 ms against the 28.4 ms target. All 162 native tests passed. These results leave the existing profile in place.

[First four-quad experiment](../2026-11-19-quad-euler676/README.md) · [Source patch](artifacts/source.patch) · [Build](artifacts/build.json) · [Entry diagnostics](artifacts/markers.json) · [Configurations](artifacts/configs.json.gz) · [Timing and profiles](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
