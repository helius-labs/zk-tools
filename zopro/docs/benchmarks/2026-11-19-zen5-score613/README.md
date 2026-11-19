# Verified Ryzen latency and memory across nine shapes

The frozen candidate and both fresh builds produced byte-identical SDK executables. Each CPU-target build passed 159 serialized core tests. The three timing labels therefore give three repeats of the same program in every randomized block. Their spread estimates measurement variability. Cargo recorded distinct target flags and produced different intermediate core archives. [Build checks](artifacts/build-checks.json) · [Compiler flags](artifacts/compiler.json)

The experimental Zopro profile led prepared Aeglos on 9/9 shapes. It met 0/8 supplied M5 CPU 10× thresholds by median on this Ryzen 9 9950X. All 225 independently exported proofs verified. Every Aeglos timed call reported a prepared-key cache hit. [Verification](artifacts/verification.json) · [Binary and input identities](artifacts/pins.json)

| Shape | Candidate ms | Release ms | Aeglos ms | Candidate / Aeglos peak MiB | Candidate vs M5 CPU / Metal | 10× CPU blocks | Same-binary spread |
|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 7.112 | 7.075 | 12.864 | 68.5 / 427.2 | 5.20× / 3.37× | 0/5 | 1.42% |
| 2x2 | 9.521 | 9.546 | 20.977 | 122.0 / 736.6 | 6.20× / 3.15× | 0/5 | 1.51% |
| 4x2 | 15.241 | 15.327 | 38.065 | 301.2 / 1266.3 | 7.15× / 3.15× | 0/5 | 2.29% |
| 8x2 | 23.552 | 24.390 | 86.087 | 531.0 / 2294.1 | 8.02× / 3.14× | 0/5 | 5.16% |
| 12x2 | 28.664 | 31.097 | 240.903 | 660.8 / 3723.1 | 9.91× / 3.52× | 0/5 | 1.35% |
| 16x2 | 37.645 | 41.080 | 241.724 | 776.3 / 4299.0 | 8.90× / 3.29× | 0/5 | 3.52% |
| 24x2 | 60.216 | 65.116 | 460.555 | 1147.9 / 6371.8 | 8.77× / 2.92× | 0/5 | 3.21% |
| merge-24x1 | 62.622 | 64.215 | 443.174 | 1040.8 / 6152.9 | 8.10× / 2.79× | 0/5 | 3.57% |
| merge-54x1 | 155.489 | 156.185 | 1005.024 | 1799.5 / 12551.7 | — | — | 1.63% |

For each program and shape, latency is the median of five shuffled blocks. Within each block the candidate uses the median of its three process means. Release and Aeglos each have one process per block. The spread column is the median within-block difference from fastest to slowest identical execution, relative to the fastest. Peak RSS is the largest observed across all measured processes for that program and shape. CPU and Metal ratios use the supplied M5 reference times. [Per-block results](artifacts/identical-sdk-blocks.json) · [Raw samples](artifacts/samples.json.gz)

## Subsequent focused measurements

These later Ryzen trials used fresh controls on their own inputs. The nine-shape panel above remains the simultaneous comparison with Aeglos. The 10× M5 CPU cutoffs for 4×2, 12×2 and 24×2 are 10.9, 28.4 and 52.8 ms respectively.

| Original input and measured profile | Median ms | Peak MiB | M5 CPU / Metal | Report |
|---|---:|---:|---:|---|
| 4×2 experimental indexed grouping | 14.863 | 310.2 | 7.33× / 3.23× | [Independent transfers](../2026-11-19-indexed-transfer622/README.md) |
| 12×2 experimental indexed grouping | 28.672 | 599.6 | 9.91× / 3.52× | [Independent transfers](../2026-11-19-indexed-transfer622/README.md) |
| 24×2 experimental tiled output and smaller pool | 56.579 | 996.3 | 9.33× / 3.11× | [Combined changes](../2026-11-19-combined-output623/README.md) |

The ordinary release SDK also gained an optional smaller folded FFT pool. Its peak RSS improvement reproduced with the measured tuned glibc settings. On the original 24×2 request, stock glibc gave the lowest peak with the main pool. [SDK release comparison](../2026-11-19-release-pool621/README.md) · [Stock and tuned glibc](../2026-11-19-allocator-pool624/README.md)

## Original per-label measurements

All three experimental columns below executed identical bytes. The compiler-label deltas quantify repeatability on this machine.

| Shape | Release / frozen / Zen 4 / Zen 5 / Aeglos ms | Zen 5 vs Zen 4 | Fresh Zen 4 vs frozen | Peak Zen 4 / Zen 5 / Aeglos MiB | Zen 5 vs M5 CPU / Metal | 10× CPU target ms (processes passing) |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 7.075 / 7.058 / 7.079 / 7.066 / 12.864 | -0.05% (3/5) | +0.43% (2/5) | 66.0 / 64.8 / 427.2 | 5.24× / 3.40× | 3.7 (0/5) |
| 2x2 | 9.546 / 9.521 / 9.524 / 9.515 / 20.977 | -0.14% (4/5) | +0.29% (2/5) | 122.0 / 120.0 / 736.6 | 6.20× / 3.15× | 5.9 (0/5) |
| 4x2 | 15.327 / 15.241 / 15.172 / 15.416 / 38.065 | +1.65% (2/5) | -1.38% (4/5) | 294.0 / 301.2 / 1266.3 | 7.07× / 3.11× | 10.9 (0/5) |
| 8x2 | 24.390 / 23.387 / 23.552 / 23.846 / 86.087 | +1.64% (2/5) | +1.17% (2/5) | 501.0 / 531.0 / 2294.1 | 7.93× / 3.10× | 18.9 (0/5) |
| 12x2 | 31.097 / 28.572 / 28.779 / 28.681 / 240.903 | -0.81% (3/5) | -0.39% (3/5) | 651.9 / 660.8 / 3723.1 | 9.90× / 3.52× | 28.4 (1/5) |
| 16x2 | 41.080 / 37.964 / 37.645 / 37.015 / 241.724 | -1.36% (5/5) | -0.56% (4/5) | 749.2 / 776.3 / 4299.0 | 9.05× / 3.35× | 33.5 (0/5) |
| 24x2 | 65.116 / 59.856 / 60.831 / 60.216 / 460.555 | -0.68% (3/5) | +1.72% (1/5) | 1111.8 / 1065.4 / 6371.8 | 8.77× / 2.92× | 52.8 (0/5) |
| merge-24x1 | 64.215 / 61.851 / 63.092 / 62.622 / 443.174 | -0.55% (3/5) | +3.57% (1/5) | 1029.6 / 1024.5 / 6152.9 | 8.10× / 2.79× | 50.7 (0/5) |
| merge-54x1 | 156.185 / 153.778 / 156.218 / 155.489 / 1005.024 | -0.33% (3/5) | +1.63% (2/5) | 1714.7 / 1689.1 / 12551.7 | — | — |
