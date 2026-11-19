# Linux allocator effects on complete warm proofs

Four process configurations use identical executables and request data within each cohort. The first uses the benchmark's glibc settings of 32 arenas, a 1 GiB mmap threshold and disabled automatic trimming. The next three use stock glibc, jemalloc or mimalloc. The alternate shared libraries come from verified Ubuntu packages extracted privately. Dynamic linker probes confirmed the providers of malloc, calloc, realloc and free in both prover executables. No process changes the machine allocator or retains witness data between processes.

Five randomized blocks use 32 Ryzen workers, three warmups and eleven timed calls per process, or 31 calls for 1×2 and 2×2. The release covers all eight supplied shapes. The grouped build uses the same prepared A/K/B2 groups and direct nested B2 view for its applicable circuits. Peak RSS includes loading and priming. The fastest median per row is descriptive and was selected on the same data.

| Shape | Request | Build | Tuned glibc / stock glibc / jemalloc / mimalloc ms | Paired stock glibc, jemalloc and mimalloc versus tuned | Peak MiB in mode order | Fastest vs M5 CPU / Metal |
|---|---|---|---:|---|---:|---:|
| 1x2 | Original | Release | 7.178 / 7.476 / 7.132 / 7.790 | +2.84% (0/5) / +0.14% (2/5) / +9.49% (0/5) | 65 / 72 / 112 / 212 | jemalloc 5.19× / 3.37× |
| 2x2 | Original | Release | 9.810 / 10.389 / 9.870 / 10.660 | +7.23% (0/5) / +2.44% (1/5) / +11.51% (0/5) | 117 / 118 / 166 / 306 | tuned 6.01× / 3.06× |
| 4x2 | Original | Release | 15.978 / 15.778 / 16.256 / 19.352 | -1.21% (3/5) / +2.03% (0/5) / +20.53% (0/5) | 260 / 249 / 256 / 441 | glibc 6.91× / 3.04× |
| 8x2 | Original | Release | 24.388 / 24.599 / 27.113 / 28.344 | +0.64% (2/5) / +10.92% (1/5) / +16.22% (1/5) | 432 / 428 / 399 / 617 | tuned 7.75× / 3.03× |
| 12x2 | Original | Release | 31.736 / 34.174 / 37.629 / 34.509 | +10.07% (0/5) / +18.68% (0/5) / +9.56% (0/5) | 568 / 433 / 453 / 626 | tuned 8.95× / 3.18× |
| 16x2 | Original | Release | 40.809 / 44.588 / 53.112 / 47.240 | +9.26% (1/5) / +30.15% (0/5) / +13.96% (0/5) | 684 / 560 / 557 / 735 | tuned 8.21× / 3.04× |
| 24x2 | Original | Release | 63.965 / 69.533 / 82.099 / 76.870 | +9.46% (0/5) / +26.80% (0/5) / +16.75% (0/5) | 1018 / 834 / 783 / 912 | tuned 8.25× / 2.75× |
| merge-24x1 | Original | Release | 64.259 / 69.214 / 80.420 / 76.095 | +7.39% (0/5) / +25.75% (0/5) / +17.75% (0/5) | 1040 / 939 / 746 / 891 | tuned 7.89× / 2.72× |
| 12x2 | Original | Grouped | 29.675 / 32.823 / 35.056 / 31.888 | +11.55% (0/5) / +17.05% (0/5) / +6.83% (0/5) | 540 / 442 / 470 / 621 | tuned 9.57× / 3.40× |
| 24x2 | Original | Grouped | 60.474 / 66.128 / 76.142 / 69.436 | +9.26% (0/5) / +26.58% (0/5) / +15.94% (0/5) | 1136 / 854 / 839 / 928 | tuned 8.73× / 2.91× |
| 12x2 | 142 | Grouped | 45.759 / 49.119 / 52.798 / 51.525 | +6.46% (1/5) / +15.62% (1/5) / +13.22% (1/5) | 980 / 597 / 598 / 787 | tuned — |
| 24x2 | 103 | Grouped | 58.619 / 62.215 / 76.942 / 68.856 | +6.98% (0/5) / +31.06% (0/5) / +19.32% (0/5) | 1095 / 903 / 834 / 934 | tuned — |

All 240 process exports verified independently. [Timings, minor faults and CPU use](artifacts/samples.json.gz) · [Allocator identities](artifacts/bindings.json) · [Verification](artifacts/verification.json)
