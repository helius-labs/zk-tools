# Ordinary Zopro SDK and prepared Aeglos across the local catalog

On the Ryzen 9 9950X at 32 hardware threads, Zopro had lower median single-proof latency on 41 of 41 shapes and lower observed peak RSS on 41 of 41. Each latency is the median of four process means. Every Aeglos timed call reported a cache hit. All 328 independently exported proofs verified against the original images and public inputs.

Zopro used less time and less memory in each of the 164 process pairs. Across the eight supplied M5 references, this SDK ran 5.13–9.21× faster than the M5 CPU and 2.78–3.33× faster than Metal. The tenfold CPU target remains open on all eight.

| Circuit | Zopro / Aeglos ms | Paired Zopro change and wins | Peak Zopro / Aeglos MiB | Zopro vs M5 CPU / Metal |
|---|---:|---:|---:|---:|
| 1x2 | 7.217 / 12.923 | -43.56% (4/4) | 59.0 / 417.0 | 5.13× / 3.33× |
| 1x4 | 9.474 / 16.672 | -43.77% (4/4) | 136.5 / 526.2 | — |
| 1x8 | 10.524 / 18.569 | -43.33% (4/4) | 173.0 / 619.8 | — |
| 1x16 | 13.099 / 22.260 | -41.30% (4/4) | 219.5 / 775.7 | — |
| 2x2 | 9.657 / 21.068 | -54.16% (4/4) | 100.8 / 721.1 | 6.11× / 3.11× |
| 2x4 | 11.078 / 21.869 | -49.10% (4/4) | 170.0 / 751.4 | — |
| 2x8 | 13.888 / 29.646 | -53.32% (4/4) | 199.5 / 932.5 | — |
| 2x16 | 17.369 / 33.489 | -48.06% (4/4) | 254.0 / 1136.1 | — |
| 3x2 | 14.297 / 31.920 | -55.48% (4/4) | 197.8 / 1098.3 | — |
| 3x4 | 14.890 / 33.082 | -54.85% (4/4) | 214.2 / 1116.7 | — |
| 3x8 | 16.063 / 35.131 | -54.16% (4/4) | 250.2 / 1189.1 | — |
| 4x2 | 15.298 / 37.893 | -59.30% (4/4) | 221.2 / 1265.9 | 7.13× / 3.14× |
| 4x4 | 16.443 / 38.259 | -57.18% (4/4) | 247.2 / 1257.6 | — |
| 4x8 | 17.836 / 40.456 | -55.91% (4/4) | 272.2 / 1342.8 | — |
| 4x16 | 20.929 / 45.386 | -53.88% (4/4) | 312.5 / 1456.9 | — |
| 5x2 | 17.683 / 43.575 | -59.42% (4/4) | 268.8 / 1413.5 | — |
| 5x4 | 18.854 / 44.863 | -57.97% (4/4) | 282.2 / 1470.5 | — |
| 5x8 | 21.128 / 65.224 | -67.77% (4/4) | 322.0 / 1776.4 | — |
| 5x16 | 24.024 / 69.827 | -65.56% (4/4) | 379.0 / 1890.9 | — |
| 6x2 | 21.193 / 68.732 | -69.02% (4/4) | 333.0 / 1846.2 | — |
| 6x4 | 21.855 / 69.289 | -68.66% (4/4) | 335.0 / 1853.8 | — |
| 6x8 | 23.368 / 72.308 | -67.66% (4/4) | 365.2 / 2045.4 | — |
| 8x2 | 24.160 / 86.311 | -71.99% (4/4) | 407.8 / 2333.4 | 7.82× / 3.06× |
| 8x4 | 25.469 / 87.971 | -71.07% (4/4) | 413.5 / 2304.8 | — |
| 8x8 | 26.698 / 91.923 | -70.96% (4/4) | 426.5 / 2418.7 | — |
| 8x16 | 30.057 / 99.688 | -69.76% (4/4) | 467.0 / 2583.0 | — |
| 12x2 | 30.836 / 183.783 | -83.26% (4/4) | 469.3 / 3370.1 | 9.21× / 3.28× |
| 12x4 | 35.079 / 186.556 | -81.22% (4/4) | 540.2 / 3441.7 | — |
| 12x8 | 35.510 / 190.527 | -81.43% (4/4) | 614.2 / 3476.3 | — |
| 16x2 | 39.495 / 241.003 | -83.61% (4/4) | 634.6 / 4289.0 | 8.48× / 3.14× |
| 16x4 | 41.394 / 243.105 | -82.92% (4/4) | 685.0 / 4320.0 | — |
| 16x8 | 41.770 / 248.376 | -83.21% (4/4) | 702.2 / 4382.7 | — |
| 24x2 | 62.926 / 460.957 | -86.31% (4/4) | 896.1 / 6334.6 | 8.39× / 2.80× |
| 24x4 | 66.425 / 461.769 | -85.66% (4/4) | 1047.4 / 6355.6 | — |
| 32x2 | 78.826 / 568.301 | -86.09% (4/4) | 1262.0 / 7971.7 | — |
| 40x2 | 92.186 / 684.849 | -86.54% (4/4) | 1463.0 / 9200.6 | — |
| 48x2 | 149.709 / 944.462 | -84.14% (4/4) | 1457.9 / 11888.2 | — |
| 49x2 | 148.883 / 958.226 | -84.46% (4/4) | 1446.2 / 12022.2 | — |
| merge-8x1 | 23.723 / 81.420 | -70.70% (4/4) | 374.2 / 2185.3 | — |
| merge-24x1 | 62.903 / 441.941 | -85.80% (4/4) | 980.0 / 6116.7 | 8.06× / 2.78× |
| merge-54x1 | 151.398 / 1001.529 | -84.88% (4/4) | 1575.6 / 12556.3 | — |

[Protocol](protocol.md) · [Summary](artifacts/summary.json) · [All samples](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)

## Where Aeglos gained time on C7a

The retained Aeglos executable ran all 41 requests faster at 96 C7a cores than at 32. Multiscalar multiplication (MSM) showed the largest elapsed-time reduction on every shape, accounting for 79–114% of the net saving. The proportion can exceed 100% when other phases get slower. On 1×2, MSM saved 4.36 ms while witness solving took 0.30 ms longer, leaving the complete proof 3.84 ms faster.

| Circuit | Verified latency saved, 32 to 96 cores (ms) | MSM saving (ms) | FFT saving (ms) | Witness saving (ms) |
|---|---:|---:|---:|---:|
| 1×2 | 3.84 | 4.36 | 0.02 | −0.30 |
| 5×16 | 20.19 | 17.59 | 2.89 | 0.28 |
| 6×2 | 19.91 | 17.46 | 3.07 | 0.02 |
| Merge 54×1 | 174.72 | 139.84 | 29.34 | 6.19 |

At 32 workers, Aeglos also spent less time in MSM on C7a than on Ryzen for all 41 shapes. The hosts have different core topologies: Ryzen used 16 physical cores with SMT, and C7a used 32 physical cores. The measurements locate elapsed time within Aeglos. Establishing which Zopro curve operations limit 96-core scaling requires timing the current Zopro release on C7a.

The phase calculations give equal weight to each process mean. They use nine archived C7a blocks per width and four fresh Ryzen blocks. All 738 archived Aeglos output hashes match the published capture receipts. “Verified latency” includes work outside the three named phases. [All phase differences](artifacts/aeglos-phase-differences.json) · [Archived process means and hashes](artifacts/aeglos-phase-processes.json.gz) · [Original C7a comparison and receipts](../2026-11-18-full-catalog-c7a244/README.md)
