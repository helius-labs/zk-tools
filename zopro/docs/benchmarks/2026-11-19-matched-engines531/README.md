# Matched warm proofs on Ryzen and archived Aeglos phase timings on C7a

The Ryzen 9950X exposed 16 physical cores with SMT. Five randomized blocks compared the integrated Zopro release with the pinned Aeglos CPU executable on the same public requests, verifier and CPU masks. At 16 workers each used one hardware thread per physical core. At 32 each used both siblings. Each independent process warmed three proofs then timed seven complete requests. The 1×2 Zopro frontend used eight workers; the 2×2 frontend used eight at width 16 and twelve at width 32. Merge54 used the merge24 profile as a supplemental measurement. The original reference set supplies no M5 time for merge54.

| Shape | Ryzen workers | Zopro ms | Aeglos ms | Zopro paired vs Aeglos | Peak RSS MiB Zopro / Aeglos | Zopro vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 16 | 8.347 | 12.879 | -35.68% (5/5) | 58.0 / 402.2 | 4.43× / 2.88× |
| 1x2 | 32 | 7.216 | 12.909 | -43.72% (5/5) | 60.2 / 418.1 | 5.13× / 3.33× |
| 2x2 | 16 | 11.133 | 21.775 | -49.03% (5/5) | 100.8 / 723.5 | 5.30× / 2.69× |
| 2x2 | 32 | 9.741 | 20.936 | -53.40% (5/5) | 108.0 / 727.5 | 6.06× / 3.08× |
| 4x2 | 16 | 16.978 | 38.725 | -56.05% (5/5) | 201.8 / 1269.1 | 6.42× / 2.83× |
| 4x2 | 32 | 15.599 | 37.828 | -58.69% (5/5) | 234.0 / 1266.7 | 6.99× / 3.08× |
| 8x2 | 16 | 26.821 | 82.573 | -67.64% (5/5) | 341.5 / 2264.5 | 7.05× / 2.76× |
| 8x2 | 32 | 23.864 | 85.840 | -72.22% (5/5) | 408.0 / 2276.7 | 7.92× / 3.10× |
| 12x2 | 16 | 36.761 | 178.588 | -79.49% (5/5) | 470.3 / 3436.6 | 7.73× / 2.75× |
| 12x2 | 32 | 31.482 | 184.383 | -82.88% (5/5) | 519.7 / 3366.7 | 9.02× / 3.21× |
| 16x2 | 16 | 43.555 | 232.116 | -81.22% (5/5) | 564.4 / 4325.4 | 7.69× / 2.85× |
| 16x2 | 32 | 39.673 | 240.311 | -83.49% (5/5) | 635.4 / 4297.1 | 8.44× / 3.13× |
| 24x2 | 16 | 69.758 | 453.342 | -84.51% (5/5) | 846.3 / 6461.7 | 7.57× / 2.52× |
| 24x2 | 32 | 63.255 | 461.385 | -86.17% (5/5) | 1005.1 / 6382.2 | 8.35× / 2.78× |
| merge-24x1 | 16 | 68.799 | 436.536 | -84.22% (5/5) | 874.4 / 6264.7 | 7.37× / 2.54× |
| merge-24x1 | 32 | 64.282 | 443.994 | -85.58% (5/5) | 965.0 / 6153.9 | 7.89× / 2.72× |
| merge-54x1 | 16 | 155.943 | 1011.658 | -84.60% (5/5) | 1535.0 / 12640.0 | — |
| merge-54x1 | 32 | 155.053 | 1002.327 | -84.57% (5/5) | 1655.1 / 12400.8 | — |

## Cross-host timing on a byte-identical Aeglos executable

The archived C7a.24xlarge run used nine blocks of seven timed calls at each of 32 and 96 physical cores. The binary, key, image, request and public output hashes match the new local run. The 32-worker Ryzen column uses SMT on 16 cores. Cross-host differences combine physical core count, microarchitecture, memory, clock and runtime conditions. Stage differences do not isolate the independent effect of any one of these. All values below are arithmetic means. Remaining time is end-to-end latency less the reported MSM, FFT and witness durations.

| Shape | Verified ms Ryzen 32 / C7a 32 / C7a 96 | Ryzen 32 minus C7a 32 ms | Contribution from MSM / FFT / witness / remaining ms |
|---|---:|---:|---:|
| 1x2 | 12.891 / 12.153 / 8.312 | +0.738 | +1.644 / -0.142 / -0.603 / -0.161 |
| 2x2 | 20.923 / 19.700 / 12.061 | +1.223 | +2.373 / -0.204 / -0.550 / -0.397 |
| 4x2 | 37.770 / 30.875 / 18.397 | +6.895 | +7.677 / +0.010 / -0.380 / -0.413 |
| 8x2 | 85.969 / 54.928 / 31.304 | +31.041 | +31.251 / +1.109 / -0.756 / -0.563 |
| 12x2 | 184.271 / 86.559 / 49.190 | +97.712 | +93.286 / +5.367 / -0.170 / -0.770 |
| 16x2 | 240.368 / 103.146 / 56.869 | +137.222 | +131.960 / +5.666 / +0.499 / -0.904 |
| 24x2 | 460.525 / 171.193 / 87.882 | +289.332 | +264.512 / +22.951 / +3.054 / -1.185 |
| merge-24x1 | 444.220 / 166.008 / 84.578 | +278.212 | +252.664 / +23.243 / +3.385 / -1.080 |
| merge-54x1 | 1002.847 / 348.025 / 173.308 | +654.822 | +651.414 / +1.571 / +3.916 / -2.079 |

All 180 new exports verified. All 162 archived proof and public digests matched; one archived proof per shape and width was independently reverified. [Local samples](artifacts/local.json.gz) · [Archived AWS samples](artifacts/archived-aws.json.gz) · [Exact inputs and profiles](artifacts/config.json) · [Verification](artifacts/verification.json)

## Process CPU time during the proof loop

Each number is total process CPU time divided by elapsed loop time, summarized by the median across processes. It gives the average number of logical CPUs running the process, including scheduling and spinning. Each Ryzen width uses the same 16 physical cores. The C7a widths use distinct physical cores. These counters cannot independently identify memory-bandwidth saturation or the fraction of useful arithmetic.

| Shape | Ryzen Zopro 16 | Ryzen Zopro 32 | Ryzen Aeglos 16 | Ryzen Aeglos 32 | Archived C7a Aeglos 32 | Archived C7a Aeglos 96 |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 11.8 | 20.9 | 15.0 | 29.0 | 27.3 | 63.6 |
| 2x2 | 11.8 | 20.3 | 15.2 | 29.6 | 28.5 | 69.2 |
| 4x2 | 13.9 | 26.4 | 15.6 | 29.7 | 29.6 | 77.0 |
| 8x2 | 14.7 | 27.8 | 15.7 | 31.0 | 30.4 | 83.2 |
| 12x2 | 14.7 | 27.7 | 15.8 | 31.4 | 30.7 | 84.2 |
| 16x2 | 14.7 | 28.6 | 15.8 | 31.3 | 30.7 | 85.6 |
| 24x2 | 14.6 | 28.8 | 15.7 | 31.2 | 30.3 | 84.7 |
| merge-24x1 | 14.6 | 28.3 | 15.7 | 31.1 | 30.4 | 85.6 |
| merge-54x1 | 14.2 | 27.5 | 15.7 | 31.3 | 30.7 | 85.9 |

[CPU-ms, elapsed loop time and CPU occupancy](artifacts/cpu-time.json)

## Native memory and scheduling across hosts

For all nine circuits, the warm Aeglos native-byte counter was byte-identical on the 32-worker Ryzen and the 32-core C7a. Every process reported a 24 GiB native limit and a cache hit on each timed call. The counter measures memory tracked by the native backend. Matching byte counts do not establish the same physical memory behavior. The process RSS figures above also include allocations outside that counter.

Each scheduling or fault cell below gives the Ryzen 32, C7a 32, and C7a 96 results in that order. The first column of numeric data gives the native memory at 32 workers on either host, then at 96 on C7a. Involuntary switches are normalized by aggregate process CPU-seconds. Minor faults and system CPU time are per timed proof. Cells use medians of five local or nine AWS process observations.

| Shape | Native MiB at 32 / 96 | Involuntary switches per CPU-second | Minor faults per proof | System CPU-ms per proof |
|---|---:|---:|---:|---:|
| 1x2 | 254.6 / 254.6 | 667.1 / 906.7 / 3796.6 | 119.3 / 199.7 / 486.0 | 2.9 / 7.1 / 22.5 |
| 2x2 | 445.5 / 445.5 | 478.2 / 713.3 / 2067.1 | 275.9 / 299.3 / 508.0 | 4.6 / 8.4 / 22.8 |
| 4x2 | 802.9 / 802.8 | 314.9 / 255.5 / 273.1 | 443.0 / 532.6 / 596.1 | 7.2 / 9.0 / 30.5 |
| 8x2 | 1431.2 / 1431.1 | 188.0 / 190.2 / 406.4 | 818.9 / 672.3 / 1354.3 | 17.4 / 20.2 / 35.1 |
| 12x2 | 2237.0 / 2236.8 | 78.0 / 125.6 / 152.2 | 983.1 / 897.0 / 2057.6 | 32.7 / 35.8 / 49.8 |
| 16x2 | 2727.3 / 2727.1 | 212.0 / 100.2 / 96.0 | 1400.6 / 889.3 / 2317.9 | 60.3 / 41.7 / 50.3 |
| 24x2 | 4170.9 / 4170.5 | 374.9 / 65.9 / 52.7 | 2353.1 / 1955.9 / 2555.1 | 122.8 / 57.1 / 65.1 |
| merge-24x1 | 4028.8 / 4028.4 | 270.6 / 61.7 / 51.4 | 2015.3 / 1724.3 / 2605.4 | 137.3 / 54.9 / 58.8 |
| merge-54x1 | 8205.1 / 8217.1 | 177.1 / 19.1 / 29.4 | 3915.1 / 4140.9 / 4889.7 | 202.3 / 101.2 / 83.2 |

The same-width host timing gap persists with matching native-byte accounting. On merge-54×1 the Ryzen run had more involuntary switches per CPU-second than the 32-core C7a, while minor faults per proof were lower. These whole-loop counters cannot attribute the MSM wall-time difference to context switching, cache traffic, or memory bandwidth. [Per-process counters and proof identities](artifacts/memory-and-switches.json)
