# Ryzen full-proof scorecard with a compact twelve-worker 2×2 frontend

For 2×2 both arms use the public SDK benchmark. Eight or twelve frontend workers share the same L3 in each paired block. The other seven shapes repeat identical release settings in both arms. Their paired differences measure run variation. Each process averages 51 small or eleven larger complete SDK requests after three warmups. The table reports the median of five process averages and the highest observed RSS.

| Shape | Threads | Previous ms | Current ms | Paired change | Wins / 5 | Current CPU-ms | Peak MiB previous / current | M5 CPU / Metal ms | CPU / Metal speedup | Blocks at 10× CPU / 5 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 32 | 7.315 | 7.309 | -0.06% | 3 | 152.1 | 101.2 / 64.5 | 37.0 / 24.0 | 5.06× / 3.28× | 0 |
| 2x2 | 32 | 9.928 | 9.937 | -0.35% | 4 | 200.9 | 115.0 / 119.2 | 59.0 / 30.0 | 5.94× / 3.02× | 0 |
| 4x2 | 32 | 15.337 | 15.510 | +0.71% | 0 | 407.7 | 262.0 / 263.5 | 109.0 / 48.0 | 7.03× / 3.09× | 0 |
| 8x2 | 32 | 24.274 | 23.940 | -1.32% | 4 | 660.6 | 417.5 / 443.8 | 189.0 / 74.0 | 7.89× / 3.09× | 0 |
| 12x2 | 32 | 32.215 | 32.011 | -0.56% | 3 | 887.8 | 610.0 / 607.5 | 284.0 / 101.0 | 8.87× / 3.16× | 0 |
| 12x2 | 16 | 36.299 | 36.468 | +0.47% | 2 | 526.1 | 514.8 / 535.0 | 284.0 / 101.0 | 7.79× / 2.77× | 0 |
| 16x2 | 32 | 40.002 | 40.280 | +0.55% | 2 | 1159.8 | 717.0 / 743.0 | 335.0 / 124.0 | 8.32× / 3.08× | 0 |
| 24x2 | 32 | 67.695 | 66.976 | -1.31% | 4 | 1958.0 | 1122.0 / 1140.8 | 528.0 / 176.0 | 7.88× / 2.63× | 0 |
| merge-24x1 | 32 | 65.596 | 66.085 | +0.38% | 2 | 1885.8 | 1041.9 / 1054.8 | 507.0 / 175.0 | 7.67× / 2.65× | 0 |

At 32 threads 0/8 shapes reached 10× M5 CPU in all five blocks, and 0/8 reached it by median. Each export passed independent Gnark verification.

[Samples](artifacts/processes.json.gz) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json) · [Varied 12×2 witnesses](../2026-11-19-varied-column-center427/README.md) · [Earlier C7a scorecard](../2026-11-18-overlap-c7a369/README.md)
