# Measured phase software counters

Three perf-stat processes per cell collect Linux software counters over 21 timed, verified 16×2 requests after three warmups. The table reports medians of per-process counters divided by 21.

| Cores | Mode | SDK ms | CPU-ms | Context switches | Migrations | Minor faults | Major faults |
|---:|---|---:|---:|---:|---:|---:|---:|
| 64 | leaves | 34.317 | 1667.65 | 2107.86 | 4.05 | 3175.14 | 0.00 |
| 64 | off | 34.062 | 1672.35 | 2103.90 | 3.81 | 3462.90 | 0.00 |
| 96 | off | 36.308 | 2481.15 | 3667.10 | 2.81 | 3650.95 | 0.00 |
| 96 | twice | 35.724 | 2438.55 | 3415.29 | 2.33 | 3754.00 | 0.00 |
