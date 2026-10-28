# Borrowed G2 pending operands

The accepted CPU source now borrows G2 pending operands instead of copying points. The tested SDK measures **8.366931 ms** for 1×2 versus 8.563320 ms accepted: paired **2.137% lower latency**, with 15/15 winning blocks. Against the rebuilt control, the paired improvement is 1.691%, also with 15/15 wins. Field source, key formats and conservative memory forecasts remain unchanged.

Adoption explicitly accepts the failed 16×2 nonregression upper bound of 1.025268 (paired median 1.0000168), uncertain behavior with 16 physical workers, and the unresolved forced-pressure condition that timed out on the accepted baseline. The final and historical performance gates remain **false**. See the [decision](artifacts/decision.md), [review](artifacts/review.json), and [complete benchmark study](../2026-10-28-g2-borrowed-production/README.md). No observation is excluded or repeated.

Primary RSS median/max is 131.25/135.00 MiB versus 130.25/132.75 MiB accepted. Peaks are mixed; the earlier 1.4 MiB reduction in requested allocation traffic is not an RSS guarantee. Merge 54×1 measures 164.388184 ms, with median/max RSS of 1802.20/1885.21 MiB. No M5 reference was supplied for that shape. No new AWS, Aeglos, giant-circuit or mobile measurement is claimed.

| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup | Required ms for CPU 10× |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 8.366931 | 37.0 | 4.422× | 24.0 | 2.868× | 3.7 |
| 2x2 | 11.539335 | 59.0 | 5.113× | 30.0 | 2.600× | 5.9 |
| 4x2 | 16.540233 | 109.0 | 6.590× | 48.0 | 2.902× | 10.9 |
| 8x2 | 25.136722 | 189.0 | 7.519× | 74.0 | 2.944× | 18.9 |
| 12x2 | 34.858408 | 284.0 | 8.147× | 101.0 | 2.897× | 28.4 |
| 16x2 | 41.377043 | 335.0 | 8.096× | 124.0 | 2.997× | 33.5 |
| 24x2 | 67.764001 | 528.0 | 7.792× | 176.0 | 2.597× | 52.8 |
| merge-24x1 | 67.096516 | 507.0 | 7.556× | 175.0 | 2.608× | 50.7 |

The M5 figures are supplied historical references. Local Ryzen 9950X warm single-proof latency includes solving, randomized proving, SDK verification and serialization. The original all-eight 10× CPU goal remains **active and unmet, 0/8**.

All 174 production files exactly match the measured source. The clean build passes 122 core, two raw-format and six cubic tests; 477 fresh exports passed independent Gnark verification, and 26,208 SDK requests verified internally. Adoption adds no benchmark or proof; the independent-proof ledger stays at 15,028. Portable, fallback, concurrency and resource evidence is inherited within its recorded scope. See the [accepted manifest](artifacts/production-source-manifest.json).

SDK: `/home/ubuntu/src/zopro-perf-work/g2-borrowed-production/native/candidate-sdk`; SHA-256: `81033d7786e14821cb76494de7faa2ebcc9e40c3222708372b21861fdeb56c20`.
