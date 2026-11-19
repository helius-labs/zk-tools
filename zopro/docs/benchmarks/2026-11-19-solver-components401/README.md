# Scheduling independent groups across adjacent solver epochs

For each contiguous interval of epochs, connect parent tasks if one reads an output produced by another within that interval. Each connected component can execute independently. The model keeps all existing inner waves and preserves the epoch joins within each component. Splitting the full sequence into intervals introduces an ordinary join at each split. A dynamic program selects the partition with the lowest unit-operation span. Each model uses the verified task graph from experiment 399.

| Shape | Packing | Existing span | Best span with pairs or single epochs | Best span with arbitrary intervals |
|---|---|---:|---:|---:|
| 16x2 | off | 13096 | 13096 | 13065 |
| 16x2 | on | 14045 | 12474 | 12474 |
| 1x2 | off | 11500 | 11500 | 11500 |
| 1x2 | on | 11419 | 11419 | 11407 |
| 2x2 | off | 12931 | 12931 | 12931 |
| 2x2 | on | 11936 | 11936 | 11936 |
| 4x2 | off | 12145 | 11969 | 11557 |
| 4x2 | on | 15003 | 14789 | 13296 |
| 8x2 | off | 13196 | 12354 | 12340 |
| 8x2 | on | 12301 | 11793 | 11485 |
| 12x2 | off | 12727 | 12726 | 12315 |
| 12x2 | on | 12303 | 11801 | 11801 |
| 24x2 | off | 14661 | 13842 | 13828 |
| 24x2 | on | 13029 | 12838 | 12695 |
| merge-24x1 | off | 34834 | 33599 | 27403 |
| merge-24x1 | on | 39426 | 38125 | 35536 |

On packed 16×2, disjoint pairs can reduce the modeled span by 11.2%, from 14,045 to 12,474. Removing all artificial joins in the earlier model yielded 9,963. On unpacked 16×2, using pairs yields no reduction. For 1×2 and 2×2, pairs give the same span as the current schedule in both packing configurations. These results motivate measuring a compact parent-index schedule specifically on packed 16×2.

[Interval costs and choices](artifacts/partitions.json) · [Verified source graphs](../2026-11-19-solver-leaf-dependencies399/README.md)
