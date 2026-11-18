# Fused Euler FFT on C7a

The merge24 comparison fell short of the prespecified screen for the fused FFT. Each block exercised the accepted executable and both modes of the experimental executable on the same 48 physical cores. Groups of six shared L3 caches alternated between blocks.

| Shape | Accepted ms | Disabled ms | Enabled ms | Paired vs disabled | Enabled wins | Paired vs accepted | Paired CPU vs disabled | Peak MiB accepted / disabled / enabled | Enabled vs M5 CPU / Metal | Enabled blocks at target |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 16x2 | 37.155 | 37.019 | 36.242 | -1.36% | 8/8 | -1.96% | -1.77% | 790 / 770 / 778 | 9.24× / 3.42× | 0/8 |
| merge-24x1 | 55.634 | 55.860 | 55.451 | +0.67% | 3/8 | -0.28% | +0.37% | 1135 / 1094 / 1147 | 9.14× / 3.16× | 0/8 |
| 24x2 | 54.111 | 53.209 | 53.591 | +1.30% | 3/8 | -0.63% | -0.34% | 1196 / 1286 / 1236 | 9.85× / 3.28× | 1/8 |

Each latency is the median of eight process means with three primes and 21 timed requests per process. Relative changes compare the same block and CPU mask. CPU includes user and system time per proof. RSS is the highest whole-process peak for each arm, including preflight. The CPU target is 33.5 ms for 16×2, 50.7 ms for merge24 and 52.8 ms for 24×2.

16×2 improved in all eight pairs and used 1.77% less CPU per proof. Its median paired wall gain of 1.36% merits retention as a secondary result. The 5.13% paired local gain on merge24 did not repeat on C7a. Although the enabled merge24 median is lower than the disabled median, the paired ratio favors disabled in five blocks. The experiment supports keeping the general production path as it is.

The primary screen required a 2% median paired reduction against disabled, at least seven paired wins, and a lower median than accepted. The [local experiment](../2026-11-18-euler-center281/README.md) contains the algebra check and the source patch. Three eligible general-path fixtures were tested on one C7a instance.

All 81 fresh exports passed independent Gnark verification on the instance and again on the local host. See the [process means and samples](artifacts/processes.json.gz), [paired block comparisons](artifacts/cells.json), [protocol](artifacts/protocol.md) and [verification receipt](artifacts/verification.json).
