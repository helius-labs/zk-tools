# Caller-allocated folded FFT scratch in the ordinary Linux SDK

The Linux SDK now supports opting into the smaller FFT pool with `ZOPRO_CPU_FFT_PRODUCT_FRONTEND=1` and an attached frontend pool. On a Ryzen 9 9950X with 32 main workers and 30 product workers, peak RSS fell in **all 28 paired runs** against the same executable with the option off. On the original request the maximum fell from **1,035.4 to 891.2 MiB**, while median latency was **63.167 and 63.144 ms**. The four paired median latency changes ranged from a 1.32% improvement to a 0.21% regression. The reason to enable this option here is memory.

Both native builds passed 155 core tests apiece. Three diagnostic exports confirmed that the product used 32 workers with no attached pool, 32 with a parked pool, and 30 with dispatch enabled. The generic x86-64 build passed a compile check. Timings use the ordinary catalog key, one active proof and the benchmark's tuned glibc configuration. A follow-up on the original request with stock glibc found 892.3 MiB on main and 897.9 MiB with the option enabled. The memory saving is specific to the measured tuned allocator profile. Mobile execution and concurrent serving remain unmeasured. [Stock allocator comparison](../2026-11-19-allocator-pool624/README.md) [Build identities, allocator settings and paired comparisons](artifacts/qualification.json)

The ordinary key and its public wire offsets are identical in all five arms. The parked control attaches the same pool without dispatching the product. Each time is the median of seven complete process means. Peak RSS includes setup and all requests.

| 24×2 request | Release / fresh / off / parked / pooled ms | Pooled vs parked / off | Peak release / fresh / off / parked / pooled MiB | Pooled CPU-ms / minor faults per proof | Pooled vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| Original | 63.938 / 63.695 / 63.167 / 64.035 / 63.144 | -1.39% (5/7) / -0.23% (4/7) | 1027.5 / 1058.7 / 1035.4 / 1036.9 / 891.2 | 1798.6 / 3,186 | 8.36× / 2.79× |
| 97 | 63.455 / 63.566 / 63.895 / 64.148 / 63.677 | -0.99% (7/7) / -0.49% (5/7) | 1070.5 / 1033.5 / 1040.6 / 1038.2 / 894.5 | 1798.8 / 3,659 | — |
| 99 | 63.629 / 62.999 / 63.038 / 64.428 / 62.546 | -3.30% (6/7) / -1.32% (6/7) | 1028.6 / 1025.2 / 1032.9 / 1026.3 / 938.7 | 1782.1 / 3,855 | — |
| 101 | 63.205 / 63.500 / 63.161 / 64.053 / 63.350 | -1.78% (5/7) / +0.21% (3/7) | 1049.2 / 1011.4 / 1003.3 / 1025.4 / 931.2 | 1790.6 / 4,145 | — |

All 143 exports verified. [Protocol](protocol.md) · [Release-source patch](artifacts/source.patch) · [Samples](artifacts/samples.json.gz) · [Verification](artifacts/verification.json)
