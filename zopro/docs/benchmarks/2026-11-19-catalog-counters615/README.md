# Minor faults and verified latency at 32 and 96 C7a cores

Zopro's median paired latency increased at 96 cores on 31 of the 41 archived circuit shapes. On six of those 31 shapes minor faults per proof **fell**. Aeglos's latency fell on all 41 shapes, even though minor faults rose on 40. The direction of the minor-fault change alone cannot identify the cause of the 96-core behavior. On 1×2 Zopro took longer in all nine paired blocks, alongside an additional 465 CPU-ms and 894 voluntary switches per proof by median. Those counters motivate measuring worker activity and useful curve work separately.

Each change is the median of the nine within-block differences at 96 minus 32 cores. Fault counts at each width are separate medians, so their difference can differ from the median paired change. All 1,476 timed processes contributed, with seven complete warmed requests per process. Circuit, public input and engine artifacts match within each pair. The table includes all eight M5 reference shapes, the larger merge, and the two equal-domain crossover shapes. The full data covers all 41.

| Shape | Zopro latency change, ms | Zopro minor faults/proof 32 / 96 | Zopro CPU-ms/proof change | Zopro voluntary switches/proof change | Aeglos latency change, ms | Aeglos minor faults/proof 32 / 96 |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | +1.77 | 598 / 750 | +465 | +894 | -3.82 | 200 / 486 |
| 2x2 | +2.55 | 2,529 / 2,503 | +679 | +1,430 | -6.89 | 299 / 508 |
| 4x2 | +1.89 | 2,566 / 2,891 | +992 | +1,533 | -12.58 | 533 / 596 |
| 5x16 | +2.14 | 4,084 / 5,056 | +1,359 | +1,905 | -20.49 | 577 / 1,296 |
| 6x2 | +2.35 | 3,557 / 3,906 | +1,338 | +1,734 | -19.73 | 679 / 1,393 |
| 8x2 | +2.10 | 3,897 / 4,505 | +1,420 | +1,926 | -23.66 | 672 / 1,354 |
| 12x2 | +0.20 | 5,421 / 5,265 | +1,604 | +2,401 | -37.53 | 897 / 2,058 |
| 16x2 | +0.29 | 6,419 / 6,115 | +1,710 | +2,660 | -45.90 | 889 / 2,318 |
| 24x2 | -3.02 | 10,104 / 9,564 | +2,096 | +2,936 | -84.46 | 1,956 / 2,555 |
| merge-24x1 | +0.33 | 10,848 / 8,006 | +2,192 | +3,439 | -81.70 | 1,724 / 2,605 |
| merge-54x1 | -16.85 | 40,262 / 28,875 | +2,874 | +3,787 | -174.63 | 4,141 / 4,890 |

No new cloud instance or proof was needed. These are the archived builds and requests. The newer 96-core 1×2 run measured 8.060 ms for Zopro and 7.943 ms for Aeglos. Local observations about recurring packed FFT mappings apply only to the measured Ryzen setup. [Archived recordings and proof verification](../2026-11-18-full-catalog-c7a244/README.md) · [Newer 1×2 result](../2026-11-18-overlap-c7a369/README.md) · [Local fault attribution](../2026-11-19-product-faults609/README.md) · [All block pairs](artifacts/results.json) · [Reader](artifacts/compare.py)
