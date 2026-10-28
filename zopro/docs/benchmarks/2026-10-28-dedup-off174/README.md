# Existing scalar-grouping switch on the adopted SDK

One immutable adopted174-source SDK is compared with explicit ZOPRO_CPU_DEDUP=1(dedup) and0(plain). No source, native library or executable changes. This bypasses optional grouping across queries and changes its workspace forecast, so it does not isolate only G2 or justify a global default change. Seven fixed interleaved blocks, primary16/32 plus16x2/32 andmerge54/32,151primary or31other requests after3primes; no profiling or owned overlapping CPU work. Witness solving, randomized proving, SDK verification and serialization are timed; loading is excluded. Labels and within-pair argument/environment lengths match.

| Cell | Variant | Median ms | CPU ms/request | Median peak MiB | Max peak MiB |
|---|---|---:|---:|---:|---:|
| 16x2-w32 | dedup | 40.408983 | 1122.796 | 805.75 | 827.00 |
| 16x2-w32 | plain | 82.719171 | 2191.079 | 686.00 | 705.75 |
| 1x2-w16 | dedup | 9.801858 | 121.925 | 87.25 | 89.00 |
| 1x2-w16 | plain | 10.658974 | 120.857 | 62.25 | 67.00 |
| 1x2-w32 | dedup | 8.134497 | 186.815 | 132.50 | 133.50 |
| 1x2-w32 | plain | 8.719087 | 189.628 | 83.00 | 87.00 |
| merge-54x1-w32 | dedup | 162.256595 | 4436.819 | 1764.66 | 1895.00 |
| merge-54x1-w32 | plain | 298.685054 | 7902.206 | 1475.32 | 1548.39 |

| Cell | Plain/dedup paired median | Lower | Upper | Wins | Paired saving ms |
|---|---:|---:|---:|---:|---:|
| 16x2-w32 | 2.041584749 | 1.992043656 | 2.079087600 | 0/7 | -42.202033 |
| 1x2-w16 | 1.086029404 | 0.819003611 | 1.222198490 | 1/7 | -0.843248 |
| 1x2-w32 | 1.069342985 | 1.054171715 | 1.077661979 | 0/7 | -0.571217 |
| merge-54x1-w32 | 1.825291108 | 1.819460907 | 1.876537072 | 0/7 | -135.270926 |

Primary advancement condition passed: **false**. Bounds use ordered extremes of7, nominal98.4375% under independent exchangeable blocks, without familywise guarantees. The prospective primary condition requires upper<1 and at least5/7wins. Other cells are retained to expose broader tradeoffs. There is no minimum gain floor, exclusion, extension or successful-capture repeat. No source or default configuration is adopted.

The [caller diagnostic](../2026-10-28-current174-g2-callers/README.md) motivated this actual bypass comparison. Its interval alone did not establish a full-map rejection or removable CPU time. All5264SDKcalls verify internally;56fresh164-byte exports independently pass Gnark/public bindings/freshness checks, ledger15059→15115. Native arithmetic and prior fallback tests are inherited; this study does not rerun their suites. Source174, SDK81033 and all input pins match after capture. RSS is whole-process high water including load/primes, not minimum RAM or requested-allocation traffic.

New comparisons against supplied M5 references cover only1x2 and16x2.

| Cell | Mode | ms | M5 CPU speedup | M5 Metal speedup |
|---|---|---:|---:|---:|
| 16x2-w32 | dedup | 40.408983 | 8.290× | 3.069× |
| 16x2-w32 | plain | 82.719171 | 4.050× | 1.499× |
| 1x2-w32 | dedup | 8.134497 | 4.549× | 2.950× |
| 1x2-w32 | plain | 8.719087 | 4.244× | 2.753× |

The complete accepted-source all-eight comparison below is inherited, not a new candidate all-shape result. Original10×CPU goal remains active/unmet; accepted0/8. No supplied54reference. No new AWS/Aeglos, mobile/ARM/TEE, giant, swap, minimum-RAM or pressure qualification.

| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |
|---|---:|---:|---:|---:|---:|
| 12x2 | 34.858408 | 284.0 | 8.147× | 101.0 | 2.897× |
| 16x2 | 41.377043 | 335.0 | 8.096× | 124.0 | 2.997× |
| 1x2 | 8.366931 | 37.0 | 4.422× | 24.0 | 2.868× |
| 24x2 | 67.764001 | 528.0 | 7.792× | 176.0 | 2.597× |
| 2x2 | 11.539335 | 59.0 | 5.113× | 30.0 | 2.600× |
| 4x2 | 16.540233 | 109.0 | 6.590× | 48.0 | 2.902× |
| 8x2 | 25.136722 | 189.0 | 7.519× | 74.0 | 2.944× |
| merge-24x1 | 67.096516 | 507.0 | 7.556× | 175.0 | 2.608× |

[Protocol](artifacts/design.md), [all observations](artifacts/analysis.json), [proof inventory](artifacts/proof-inventory.json), [source/input pins](artifacts/pins.json).
