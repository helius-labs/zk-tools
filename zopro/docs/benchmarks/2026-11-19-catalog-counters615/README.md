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

## How much of the historical gap could worker count explain?

Aeglos at 96 workers finished ahead of **both** measured Zopro widths in all nine blocks on 16 shapes. On two more shapes it finished ahead of both in six blocks (5×16) and eight blocks (5×2). On the other 23 shapes at least one measured Zopro width finished ahead of Aeglos in every block. For the 16 consistent gaps, selecting between the observed 32- and 96-worker executions could never have reversed a single block.

The table reports the median paired latency difference in milliseconds from Aeglos at 96 workers. A positive value favors Aeglos. For each held-out block, the width selection uses the median paired difference across the other eight blocks on the same shape. The best observed result uses the faster Zopro execution in the held-out block itself. Across all 41 shapes, both methods left 18 positive median gaps.

| Shape | Eight-block selection, ms | Best observed, ms | Blocks with both Zopro widths slower |
|---|---:|---:|---:|
| 1×2 | +2.596 | +2.596 | 9/9 |
| 1×16 | +5.651 | +5.651 | 9/9 |
| 4×2 | +1.343 | +1.343 | 9/9 |
| 5×16 | +0.159 | +0.159 | 6/9 |
| 6×2 | −2.821 | −2.821 | 0/9 |
| Merge-54×1 | −47.722 | −47.722 | 0/9 |

These comparisons reuse the same host, request and historical builds in every fold. They cover two whole-process widths. Intermediate widths, per-stage scheduling and later runtime changes require their own measurements. On the newer 96-core 1×2 run, the observed median gap between the two provers was 0.117 ms. [All 369 held-out comparisons](artifacts/width-comparison.json) · [Reader](artifacts/compare.py) · [Newer 1×2 measurement](../2026-11-18-overlap-c7a369/README.md)
