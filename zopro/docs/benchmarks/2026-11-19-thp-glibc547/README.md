# Transparent huge pages on glibc prover allocations

Two runs of the same binary use glibc.malloc.hugetlb set to zero or one, retaining the existing 32 arenas, 1 GiB mmap threshold and disabled automatic trimming. The loader reported the expected values for all four tunables. The Linux host enables transparent huge pages on advice. Separate diagnostic processes record resident anonymous huge pages over startup, three primes and the warm loop. The maximum observed value is sampled and can be below the true peak. No monitoring runs during the timing comparison.

| Shape | Maximum observed anonymous huge pages MiB ordinary / advised |
|---|---:|
| 1x2 | 0.0 / 6.0 |
| 12x2 | 0.0 / 134.0 |
| 24x2 | 0.0 / 270.0 |

Five randomized process blocks use 32 Ryzen workers. Each process primes three proofs and times 31 calls on small shapes or eleven elsewhere. Peak process RSS includes key loading and priming. Minor faults cover the warm calls. CPU and Metal speedups use the supplied M5 references.

| Shape | Request | Build | Ordinary / advised ms | Paired change | Peak MiB ordinary / advised | Minor faults per proof ordinary / advised | Advised vs M5 CPU / Metal |
|---|---|---|---:|---:|---:|---:|---:|
| 1x2 | Original | Release | 7.202 / 7.080 | -1.70% (5/5) | 64.8 / 66.1 | 79 / 61 | 5.23× / 3.39× |
| 2x2 | Original | Release | 9.747 / 9.707 | -0.24% (3/5) | 113.8 / 116.4 | 186 / 169 | 6.08× / 3.09× |
| 4x2 | Original | Release | 15.586 / 15.855 | +2.15% (1/5) | 258.5 / 259.8 | 1853 / 2068 | 6.87× / 3.03× |
| 8x2 | Original | Release | 24.418 / 24.548 | -0.58% (3/5) | 435.8 / 461.4 | 3261 / 3398 | 7.70× / 3.01× |
| 12x2 | Original | Release | 32.004 / 32.807 | -0.06% (3/5) | 565.0 / 571.6 | 3319 / 3784 | 8.66× / 3.08× |
| 16x2 | Original | Release | 40.082 / 40.674 | +1.48% (2/5) | 672.2 / 685.2 | 3989 / 3632 | 8.24× / 3.05× |
| 24x2 | Original | Release | 64.958 / 64.705 | +0.08% (2/5) | 1017.4 / 1008.8 | 5116 / 5300 | 8.16× / 2.72× |
| merge-24x1 | Original | Release | 65.641 / 65.804 | +0.25% (2/5) | 1064.0 / 1022.5 | 5932 / 5483 | 7.70× / 2.66× |
| 12x2 | Original | Grouped | 29.399 / 29.880 | +3.09% (2/5) | 547.6 / 543.9 | 2775 / 2685 | 9.50× / 3.38× |
| 24x2 | Original | Grouped | 60.593 / 61.212 | +0.61% (2/5) | 1128.8 / 1114.3 | 7588 / 5687 | 8.63× / 2.88× |
| 12x2 | 142 | Grouped | 45.908 / 45.709 | -0.82% (4/5) | 964.5 / 955.0 | 8158 / 8990 | — |
| 24x2 | 103 | Grouped | 59.160 / 57.825 | -2.85% (4/5) | 1096.0 / 1144.0 | 5835 / 6048 | — |

All 126 process exports verified independently. [Timings](artifacts/samples.json.gz) · [Residency probes](artifacts/probes.json.gz) · [Policy](artifacts/policy.json) · [Verification](artifacts/verification.json)
