# Page advice on the packed first inverse FFT layer

The table compares ordinary and advised output in the same executable. Latencies are medians of five process means, faults per proof cover the timed loop, and peak memory includes setup. A successful advice call permits the kernel to use transparent huge pages. The trial measures its consequences without assuming the backing page size.

| Shape | Request | Pool | Ordinary / advised ms | Paired change | Minor faults ordinary / advised | Peak MiB ordinary / advised | Advised vs M5 CPU / Metal |
|---|---|---|---:|---:|---:|---:|---:|
| 12x2 | Original | main | 28.729 / 28.749 | +0.48% (2/5) | 3,330 / 3,154 | 579.5 / 597.0 | 9.88× / 3.51× |
| 12x2 | Original | auxiliary | 28.670 / 28.749 | -0.40% (3/5) | 752 / 940 | 446.3 / 458.4 | 9.88× / 3.51× |
| 12x2 | 57 | main | 28.513 / 28.441 | +0.07% (2/5) | 3,225 / 2,985 | 582.9 / 579.7 | — |
| 12x2 | 57 | auxiliary | 28.824 / 28.883 | +0.20% (1/5) | 934 / 945 | 460.3 / 448.3 | — |
| 24x2 | Original | main | 58.877 / 58.436 | -0.46% (3/5) | 5,917 / 6,314 | 1105.5 / 1116.7 | 9.04× / 3.01× |
| 24x2 | Original | auxiliary | 60.576 / 65.427 | +8.01% (0/5) | 19,582 / 3,756 | 892.2 / 945.7 | 8.07× / 2.69× |
| 24x2 | 103 | main | 56.682 / 57.746 | +1.35% (1/5) | 5,618 / 6,012 | 1080.8 / 1132.9 | — |
| 24x2 | 103 | auxiliary | 59.356 / 64.433 | +8.67% (0/5) | 19,045 / 3,265 | 897.7 / 890.2 | — |

All 104 exports verified independently. [Protocol](protocol.md) · [Per-process timings, advice receipts and frozen controls](artifacts/samples.json.gz) · [Source](artifacts/source.patch) · [Fault profile](../2026-11-19-product-faults609/README.md)
