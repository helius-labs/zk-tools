# Sixteen prover workers on sixteen or eight Ryzen cores

The same two proof engines used 16 logical CPUs on the same Ryzen 9950X. One placement used one hardware thread on each of sixteen physical cores. The other used both siblings on eight cores. Both masks placed eight logical CPUs in each L3 group. Complementary masks rotated across six randomized blocks. The 1×2 Zopro frontend used eight distinct physical cores in both placements. Each process warmed three proofs and timed seven. The native Aeglos byte count matched across both placements for each key.

| Shape | Engine | Physical / SMT ms | Paired SMT change and wins | CPU-ms physical / SMT | Peak MiB physical / SMT | Aeglos MSM ms physical / SMT |
|---|---|---:|---:|---:|---:|---:|
| 1x2 | zopro | 8.340 / 10.544 | +26.43% (0/6) | 98.1 / 128.4 | 56.8 / 58.5 | — |
| 1x2 | aeglos | 13.086 / 15.994 | +22.95% (0/6) | 189.2 / 231.4 | 408.5 / 413.1 | 10.343 / 13.020 |
| 16x2 | zopro | 43.816 / 52.486 | +19.29% (0/6) | 643.1 / 782.3 | 548.5 / 558.1 | — |
| 16x2 | aeglos | 231.870 / 260.068 | +11.85% (0/6) | 3627.4 / 4072.1 | 4316.3 / 4291.3 | 202.357 / 225.381 |
| merge-54x1 | zopro | 155.007 / 181.625 | +17.36% (0/6) | 2218.3 / 2639.4 | 1547.5 / 1548.0 | — |
| merge-54x1 | aeglos | 1002.966 / 1056.729 | +5.34% (0/6) | 15509.2 / 16625.0 | 12717.0 / 12718.0 | 909.095 / 940.501 |

These timings measure the effect of sharing physical cores at a fixed worker count and shared L3 footprint. They do not isolate the cross-host difference in memory bandwidth or instruction throughput. All 72 fresh exports verified. [CPU assignments](artifacts/masks.json) · [Process measurements](artifacts/samples.json.gz) · [Verification](artifacts/verification.json)
