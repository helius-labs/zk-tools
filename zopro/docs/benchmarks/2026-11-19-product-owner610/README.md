# Stable worker ownership during folded product evaluation

Each process uses three warmups and eleven timed proofs. Latencies are medians of five process means. Minor faults cover the timed calls. Peak RSS covers the entire process.

| Shape | Request | Prior dispatch / main / dispatch / owner ms | Owner vs dispatch / main | Minor faults per proof main / dispatch / owner | Peak MiB main / dispatch / owner | Owner vs M5 CPU / Metal |
|---|---|---:|---:|---:|---:|---:|
| 12x2 | Original | 28.708 / 28.506 / 28.741 / 29.016 | +1.40% (1/5) / +0.92% (0/5) | 3,335 / 1,262 / 995 | 574.5 / 467.8 / 459.8 | 9.79× / 3.48× |
| 12x2 | 57 | 28.795 / 28.988 / 28.891 / 28.973 | -0.11% (3/5) / +1.40% (1/5) | 3,078 / 805 / 1,259 | 571.3 / 466.8 / 511.9 | — |
| 24x2 | Original | 60.770 / 59.293 / 60.513 / 61.294 | +0.81% (1/5) / +2.60% (0/5) | 7,662 / 19,346 / 19,360 | 1118.5 / 910.3 / 930.7 | 8.61× / 2.87× |
| 24x2 | 103 | 59.540 / 57.427 / 59.532 / 60.218 | +0.29% (2/5) / +3.13% (1/5) | 6,948 / 19,072 / 19,325 | 1167.3 / 892.5 / 915.0 | — |

All 82 exports verified, including the two separate worker-selection probes. [Protocol](protocol.md) · [Samples](artifacts/samples.json.gz) · [Source](artifacts/source.patch) · [Fault profile](../2026-11-19-product-faults609/README.md)
