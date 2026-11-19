# One or two ranges per signed H window on Ryzen 12×2

All timed processes used compact Poseidon output addresses. The signed implementations split each scalar into 17 or 18 windows. Two-range processing fills two sets of buckets per window and combines them. Route probes on a private executable copy verified entry to the signed implementation. The confirmation uses fresh processes and 31 verified requests each.

## Screening at 32 threads

| Layout | Compact outputs | Median ms | Peak MiB |
|---|---:|---:|---:|
| ordinary | 1 | 33.666 | 600.2 |
| 17sx1 | 1 | 33.806 | 617.5 |
| 18sx1 | 1 | 32.874 | 612.5 |
| 17sx2 | 1 | 36.082 | 616.0 |
| 18sx2 | 1 | 34.937 | 602.8 |

## Confirmation

| Workers | Layout | Compact outputs | Median ms | CPU-ms per proof | Peak MiB | M5 CPU / Metal speedup | Blocks ≤28.4 ms / 7 |
|---:|---|---:|---:|---:|---:|---:|---:|
| 16 | ordinary | 1 | 38.334 | 564.6 | 558.8 | 7.41× / 2.63× | 0 |
| 16 | 18sx2 | 1 | 37.852 | 555.6 | 563.0 | 7.50× / 2.67× | 0 |
| 16 | 18sx1 | 1 | 37.094 | 542.3 | 559.2 | 7.66× / 2.72× | 0 |
| 32 | ordinary | 1 | 33.280 | 917.1 | 706.0 | 8.53× / 3.03× | 0 |
| 32 | 18sx2 | 1 | 34.543 | 982.8 | 703.5 | 8.22× / 2.92× | 0 |
| 32 | 18sx1 | 1 | 33.258 | 896.6 | 710.2 | 8.54× / 3.04× | 0 |

The selected setting was 18sx1 with compact outputs set to 1. At 32 workers the paired change vs the ordinary baseline was +1.23% and it won 3 of seven blocks. Ordinary H remains the 32-thread setting. At 16 physical cores the single-range layout won all seven matched blocks against both controls, with paired gains of 3.08% versus ordinary H and 1.89% versus the two-range layout. This was the secondary width.

[Protocol](artifacts/protocol.md) · [Route checks](artifacts/routes.json) · [Decision](artifacts/decision.json) · [Process samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)
