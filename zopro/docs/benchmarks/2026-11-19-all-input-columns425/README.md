# FFT input-column coverage and timing across the eight reference shapes

All rows used the same integrated executable and 32 hardware threads on one Ryzen 9950X. The trace column counts calls on three timed proofs per shape. Eligible shapes first used three paired blocks. The confirmation used seven fresh blocks for each shape chosen by the screen. Each timing process completed three warmups and 21 verified requests. For each pair, the only setting changed was FFT input packing.

| Shape | Calls / 3 | Screen paired change (wins / 3) | Confirmation off / on (ms) | Paired change (wins / 7) | Peak MiB off / on | M5 CPU / Metal speedup, on |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 3 | +0.32% (1/3) | — | — | — | — |
| 2x2 | 3 | -0.27% (2/3) | 10.120 / 10.103 | +0.02% (3/7) | 115.2 / 114.0 | 5.84× / 2.97× |
| 4x2 | 3 | +0.14% (1/3) | — | — | — | — |
| 8x2 | 3 | -2.52% (3/3) | 23.937 / 23.594 | -0.73% (5/7) | 476.0 / 477.2 | 8.01× / 3.14× |
| 12x2 | 3 | -1.04% (3/3) | 33.077 / 32.656 | -0.61% (5/7) | 662.0 / 658.5 | 8.70× / 3.09× |
| 16x2 | 3 | -0.30% (3/3) | 40.699 / 40.180 | -1.50% (6/7) | 800.2 / 791.8 | 8.34× / 3.09× |
| 24x2 | 3 | -2.86% (3/3) | 70.078 / 68.171 | -2.99% (7/7) | 1208.1 / 1201.2 | 7.75× / 2.58× |
| merge-24x1 | 3 | -1.64% (3/3) | 67.465 / 65.925 | -2.18% (7/7) | 1129.8 / 1110.2 | 7.69× / 2.65× |

The confirmation criterion was met by 16x2, 24x2, merge-24x1. Empty confirmation cells indicate that the shape had no eligible call or that its screen did not select it. The measurements use one original request per shape. Independent varied requests were tested previously on 12×2.

[Selection and eligible shapes](artifacts/selection.json) · [Protocol](artifacts/protocol.md) · [All process measurements](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [Full integrated CPU and Metal scorecard](../2026-11-19-qualified-score419/README.md)
