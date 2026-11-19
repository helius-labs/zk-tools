# Current ordinary and experimental Zopro proofs against prepared Aeglos on Ryzen

The table covers all eight supplied M5 references and merge 54×1. Six blocks per shape use every engine order. Each process warms three requests and times seven, with one proof active. Peak RSS includes setup. Every Aeglos timed call reported a key-cache hit. All 162 independent proof exports verified against the original keys. The experimental median reached 0/8 M5 CPU targets. The 12×2 experimental arm used the **mapped** configuration from the separate eight-placement run. Ordinary and experimental are same-binary repeats on 1×2 and merge 54×1. The protocol lists each other configuration.

| Circuit | Release / experimental / Aeglos ms | Peaks in same order, MiB | Release vs M5 CPU / Metal | Experimental vs M5 CPU / Metal | 10× CPU target ms (experimental blocks meeting it) |
|---|---:|---:|---:|---:|---:|
| 1x2 | 7.170 / 7.207 / 12.873 | 59.8 / 59.5 / 423.0 | 5.16× / 3.35× | 5.13× / 3.33× | 3.7 (0/6) |
| 2x2 | 9.815 / 9.518 / 20.991 | 104.8 / 113.2 / 732.5 | 6.01× / 3.06× | 6.20× / 3.15× | 5.9 (0/6) |
| 4x2 | 15.295 / 15.148 / 37.738 | 232.2 / 258.8 / 1275.8 | 7.13× / 3.14× | 7.20× / 3.17× | 10.9 (0/6) |
| 8x2 | 24.127 / 23.780 / 86.227 | 413.0 / 466.8 / 2284.6 | 7.83× / 3.07× | 7.95× / 3.11× | 18.9 (0/6) |
| 12x2 | 30.965 / 28.709 / 240.397 | 489.0 / 517.0 / 3631.5 | 9.17× / 3.26× | 9.89× / 3.52× | 28.4 (1/6) |
| 16x2 | 39.966 / 37.628 / 241.138 | 616.2 / 695.3 / 4307.4 | 8.38× / 3.10× | 8.90× / 3.30× | 33.5 (0/6) |
| 24x2 | 62.571 / 56.503 / 459.349 | 962.1 / 895.3 / 6364.4 | 8.44× / 2.81× | 9.34× / 3.11× | 52.8 (0/6) |
| merge-24x1 | 62.655 / 61.698 / 444.176 | 972.8 / 1004.2 / 6142.4 | 8.09× / 2.79× | 8.22× / 2.84× | 50.7 (0/6) |
| merge-54x1 | 152.391 / 151.699 / 1001.550 | 1676.2 / 1675.2 / 12490.0 | — | — | — |

The ordinary release finished sooner and used less peak RSS than prepared Aeglos on all nine shapes. The largest experimental latency reduction against the release was 9.7% on 24×2. On 12×2 the experimental process median was 28.709 ms, above the 28.4 ms target. One of six blocks met it. The independent eight-placement test measured 28.527 ms using the same process-level statistic. [Eight CPU placements](../2026-11-19-rotated-reference652/README.md)

[Protocol and per-shape selections](protocol.md) · [Inputs and binaries](artifacts/pins.json) · [Exact Rust xtask workflow](artifacts/workflow.json.gz) · [Samples, counters and receipts](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
