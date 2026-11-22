# Physical phone proof benchmarks

The AWS Device Farm phones and the wired iPhone 17e used the same pinned keys and witness bytes as the M5 Pro benchmark.

All five phones completed all 41 shapes. Each Android phone recorded 123 verified CPU process blocks. Each iPhone recorded 123 verified CPU blocks and 123 verified Metal blocks.

A value requires three fresh processes. A dash indicates an incomplete measurement or an unavailable counter.

[M5 Pro, AWS, and phones CSV](comparison-with-mobile.csv) · [Phone results CSV](mobile.csv) · [Raw blocks](artifacts/mobile-blocks.jsonl)

| Device | OS version | Run |
|---|---|---|
| Apple iPhone 15 | 18.5 | AWS Device Farm, passed |
| Apple iPhone 17 Pro | 26.3.1 | AWS Device Farm, passed |
| Samsung Galaxy A56 | Android 15 | AWS Device Farm, passed |
| Samsung Galaxy S25 | Android 15 | AWS Device Farm, passed |
| Apple iPhone 17e | 27.2.1 | USB, passed |

## iPhone 17e current release across all 41 shapes

The signed `990508c` build ran on the wired iPhone 17e with iOS 27.2.1. Each shape used the original pinned key and witness and ran three fresh processes per backend. All 246 process blocks verified, for 1,968 proofs. The run recorded thermal state `serious`.

Each process ran eight proofs. Warm latency averages the last five prove-and-verify times, and each reported value is the median of the three processes. The comparison with `f294171` uses separate wired benchmark sessions. The CSV reports peak physical footprint and peak RSS separately.

The fastest backend improved on 41/41 shapes. The median speedup was 2.24×, with a maximum of 4.83× on 16×2. Metal was fastest on 41/41 shapes.

The current phone was faster than AWS c7a Gnark on 10/41 shapes. AWS c7a Aeglos and Zopro were faster on all 41 shapes.

[All 41 iPhone shapes with M5 Pro and AWS c7a time and memory](iphone-17e-release-comparison.csv) · [Raw current release blocks](artifacts/iphone-17e-current-release-blocks.jsonl)

| Shape | Original iPhone warm ms | Current iPhone warm ms | Speedup | Current peak physical footprint MiB | Current peak RSS MiB |
|---|---:|---:|---:|---:|---:|
| 1×2 | 79.2 | 58.0 | 1.37× | 118.8 | 96.4 |
| 8×2 | 799.3 | 187.2 | 4.27× | 323.8 | 325.4 |
| 16×2 | 1401.8 | 290.4 | 4.83× | 506.2 | 575.7 |
| 24×2 | 2063.6 | 999.7 | 2.06× | 773.1 | 884.2 |
| 48×2 | 3886.9 | 1755.7 | 2.21× | 1373.8 | 1664.4 |
| 49×2 | 3938.2 | 1762.0 | 2.24× | 1412.4 | 1685.2 |
| merge-54×1 | 4116.1 | 1557.2 | 2.64× | 1463.8 | 1739.5 |

## iPhone 17e paired release comparison

Two signed release builds ran back to back on a wired iPhone 17e with iOS 27.2.1. The control used `576e8ee`, which has the same prover source as the commit that added the wired baseline. The candidate used `dab92b6` with repeated witness queries compacted on Apple family 9. Both builds used the same pinned keys and witnesses.

Each cell shows the median of three fresh process blocks. Each block ran eight proofs with native verification and averaged the last five for warm latency. The speedup column shows the median of the three within-block ratios. All 60 process blocks verified. The app sampled thermal state before and after each block and reported `serious`. The five sampled shapes cover the smallest proof, three intermediate shapes, and the largest merge.

[Raw paired process blocks](artifacts/iphone-17e-grouped-query-blocks.jsonl)

| Shape | Backend | Warm ms control → candidate | Speedup | Peak physical footprint MiB control → candidate | Peak RSS MiB control → candidate |
|---|---|---:|---:|---:|---:|
| 1×2 | CPU | 125.2 → 123.3 | 1.02× | 20.2 → 24.1 | 55.3 → 60.2 |
| 1×2 | Metal | 79.7 → 73.9 | 1.10× | 157.8 → 146.1 | 119.9 → 110.9 |
| 8×2 | CPU | 718.7 → 406.0 | 1.81× | 91.6 → 94.9 | 207.5 → 211.7 |
| 8×2 | Metal | 755.5 → 182.2 | 4.15× | 382.6 → 315.1 | 425.2 → 356.9 |
| 16×8 | CPU | 1443.0 → 710.8 | 2.03× | 163.8 → 171.1 | 376.9 → 385.4 |
| 16×8 | Metal | 1471.7 → 326.6 | 3.40× | 609.1 → 512.8 | 752.8 → 647.7 |
| 24×2 | CPU | 2618.6 → 1296.9 | 2.02× | 268.4 → 279.1 | 584.7 → 602.2 |
| 24×2 | Metal | 2071.8 → 663.5 | 2.29× | 907.7 → 774.6 | 1153.3 → 1006.2 |
| merge-54×1 | CPU | 6292.7 → 3474.5 | 1.81× | 538.3 → 545.5 | 1180.8 → 1189.9 |
| merge-54×1 | Metal | 4036.5 → 1841.2 | 2.19× | 1691.2 → 1406.7 | 2261.5 → 1953.8 |

## iPhone 17e paired GPU recoding comparison

Two signed Metal release builds ran over USB on iPhone 17e with iOS 27.2.1. `dab92b6` is the witness-grouping build from the preceding comparison. `5102771` keeps recoded quotient digits on the GPU when live process memory allows. Both builds use the same pinned keys and witnesses.

All 24 process blocks passed native verification. Each block ran eight proofs and averaged the last five. The table reports the median of three blocks per build, and the speedup column uses the median of the within-block ratios. The app reported thermal state `serious` before and after the blocks.

[Raw paired GPU recoding blocks](artifacts/iphone-17e-gpu-recoding-blocks.jsonl)

| Shape | Warm ms control → candidate | Speedup | Peak physical footprint MiB control → candidate | Peak RSS MiB control → candidate |
|---|---:|---:|---:|---:|
| 1×2 | 78.9 → 77.8 | 1.01× | 146.0 → 146.3 | 110.9 → 111.2 |
| 8×2 | 242.1 → 198.1 | 1.27× | 315.6 → 314.1 | 357.4 → 325.2 |
| 24×2 | 1029.2 → 835.5 | 1.23× | 773.5 → 772.6 | 1005.2 → 883.6 |
| merge-54×1 | 1969.6 → 1898.0 | 1.04× | 1415.8 → 1459.9 | 1953.6 → 1737.7 |

The largest merge was faster in two pairs. Its within-pair speed ranged from 0.64× to 1.04×.

## iPhone 17e paired host bucket reduction comparison

Two signed Metal release builds ran over USB on iPhone 17e with iOS 27.2.1. `5102771` is the device recoding build from the preceding comparison. `6b04d94` sums the buckets in XYZZ coordinates on the host so it can convert each window once. Both builds use the same pinned keys and witnesses.

All 24 process blocks passed native verification. Each block ran eight proofs and averaged the last five. The table reports the median of three blocks per build, and the speedup column uses the median of the within-block ratios. The app reported thermal state `serious` before and after the blocks.

[Raw paired host bucket reduction blocks](artifacts/iphone-17e-host-bucket-blocks.jsonl)

| Shape | Warm ms control → candidate | Speedup | Peak physical footprint MiB control → candidate | Peak RSS MiB control → candidate |
|---|---:|---:|---:|---:|
| 1×2 | 75.4 → 73.9 | 1.00× | 146.2 → 146.3 | 111.0 → 111.1 |
| 8×2 | 200.0 → 177.9 | 1.12× | 314.5 → 314.7 | 325.6 → 325.8 |
| 24×2 | 995.7 → 801.5 | 1.13× | 773.9 → 772.6 | 885.0 → 883.6 |
| merge-54×1 | 1898.0 → 1815.9 | 1.04× | 1461.7 → 1463.0 | 1742.5 → 1740.9 |

The largest merge had within-pair speed ratios from 0.64× to 1.04×. Peak RSS and physical footprint changed by less than 2 MiB for that shape.

## iPhone 17e paired short-proof window comparison

Two signed Metal release builds ran over USB on iPhone 17e with iOS 27.2.1. `5102771` is the device recoding build. `8220b63` caps the Metal G1 A, K, and quotient windows at eight bits on short proofs. Both builds use the same pinned keys and witnesses.

All 24 process blocks passed native verification. Each block ran eight proofs and averaged the last five. The table reports the median of three blocks per build, and the speedup column uses the median of the within-block ratios. The app reported thermal state `serious` before and after the blocks.

[Raw paired short-proof window blocks](artifacts/iphone-17e-short-window-blocks.jsonl)

| Shape | Warm ms control → candidate | Speedup | Peak physical footprint MiB control → candidate | Peak RSS MiB control → candidate |
|---|---:|---:|---:|---:|
| 1×2 | 78.9 → 55.1 | 1.35× | 146.4 → 118.8 | 111.2 → 96.3 |
| 1×4 | 92.3 → 85.7 | 1.08× | 158.8 → 144.7 | 127.7 → 115.6 |
| 2×2 | 113.1 → 106.4 | 1.09× | 183.7 → 171.5 | 156.1 → 145.7 |
| 2×4 | 114.2 → 115.1 | 0.99× | 188.0 → 175.1 | 161.4 → 150.4 |

The host reduction and short-proof window runs each measured one change against `5102771`.

## iPhone 17e combined host reduction and short windows

The signed `1ed3005` release combines host bucket reduction and narrower short-proof Metal windows. The signed `5102771` control already includes GPU quotient recoding. Both builds ran over USB on the same iPhone 17e with iOS 27.2.1 and used the same pinned keys and witnesses.

All 28 process blocks passed native verification. Each block ran eight proofs and averaged the last five for warm latency. Each shape ran twice per build, interleaved within each block. Latency and memory show the median of the two measurements. The speedup column shows both ratios in block order. The phone reported `serious` thermal state before and after every block.

[Raw combined release blocks](artifacts/iphone-17e-combined-window-host-bucket-blocks.jsonl)

| Shape | Warm ms control → candidate | Paired speedup | Peak physical footprint MiB control → candidate | Peak RSS MiB control → candidate |
|---|---:|---:|---:|---:|
| 1×2 | 88.6 → 58.2 | 1.40× / 1.64× | 146.2 → 118.6 | 111.1 → 96.1 |
| 1×4 | 98.2 → 93.3 | 1.12× / 1.00× | 158.5 → 145.0 | 127.4 → 115.8 |
| 2×2 | 107.5 → 105.3 | 0.98× / 1.07× | 184.0 → 171.4 | 156.3 → 145.7 |
| 2×4 | 130.2 → 119.1 | 1.32× / 0.90× | 188.0 → 175.1 | 161.4 → 150.4 |
| 8×2 | 172.1 → 175.8 | 0.94× / 1.02× | 318.4 → 313.3 | 325.1 → 324.6 |
| 24×2 | 685.9 → 707.5 | 0.90× / 1.01× | 772.7 → 774.5 | 883.8 → 885.6 |
| merge-54×1 | 1830.4 → 1882.6 | 1.09× / 0.86× | 1449.7 → 1464.6 | 1741.0 → 1738.7 |

The smallest shape was faster in both pairs and used 27.6 MiB less peak physical footprint. For 8×2, 24×2, and the largest merge, the candidate was faster in one block and slower in the other.

## iPhone 17e paired G2 placement comparison

Two signed Metal release builds ran over USB on an iPhone 17e with iOS 27.2.1. `1ed3005` is the combined host bucket reduction and short windows build. `c9dcafe` runs G2 on the CPU within the measured short-query size ranges on Apple family 9. Metal still computes the quotient and G1 queries. Both builds used the same pinned keys and witnesses.

All 42 fresh process blocks passed native verification. Each block ran eight proofs and averaged the last five for warm latency. The table shows medians of three blocks per build. The speedup column uses the median of the three within-block ratios. The recorded thermal states were `serious`.

[Raw paired G2 placement blocks](artifacts/iphone-17e-short-g2-release-blocks.jsonl)

| Shape | Warm ms control → candidate | Paired speedup | Load ms control → candidate | Peak physical footprint MiB control → candidate | Peak RSS MiB control → candidate |
|---|---:|---:|---:|---:|---:|
| 1×2 | 54.6 → 54.4 | 1.01× | 96.9 → 98.0 | 118.8 → 118.9 | 96.4 → 96.3 |
| 1×4 | 89.7 → 70.2 | 1.28× | 97.9 → 75.1 | 144.8 → 105.1 | 115.8 → 109.7 |
| 1×8 | 112.7 → 117.3 | 0.97× | 99.8 → 99.5 | 158.2 → 158.2 | 131.5 → 131.4 |
| 1×16 | 153.5 → 123.8 | 1.26× | 104.9 → 73.8 | 178.0 → 142.4 | 156.8 → 155.2 |
| 2×2 | 109.7 → 88.9 | 1.26× | 101.8 → 74.3 | 171.5 → 129.9 | 145.7 → 137.9 |
| 2×4 | 115.9 → 94.5 | 1.25× | 103.8 → 74.1 | 175.2 → 133.3 | 150.7 → 142.6 |
| 2×8 | 154.1 → 136.5 | 1.15× | 115.0 → 87.5 | 222.1 → 180.4 | 202.9 → 195.1 |

All five shapes selected for CPU G2 were faster in every pair and used about 36–42 MiB less peak physical memory. The 1×2 and 1×8 controls exercised the GPU split. The 1×8 control had a 4.6 ms higher median in the candidate build.

The candidate also passed 656 native proof verifications on the M5 Pro, with eight CPU and eight Metal proofs for every pinned shape.

## iPhone 17e paired quotient MSM kernel comparison

Two signed Metal release builds ran over USB on the iPhone 17e with iOS 27.2.1 and the same pinned keys and witnesses. The candidate uses radix-29 Montgomery arithmetic for the first level of the largest quotient MSMs on Apple family 9.

The control was `c9dcafe` and the candidate was `03533c2`. All 24 fresh process blocks passed native verification. Each block ran eight proofs and averaged the last five for warm latency. The table uses medians of the three blocks per build. Paired speedup uses the median of the three within-block ratios. Recorded thermal states were `nominal`, `fair`, `serious`.
The run resumed after a USB disconnect. The raw blocks retain their timestamps and thermal readings.

[Raw paired quotient kernel blocks](artifacts/iphone-17e-radix29-release-blocks.jsonl)

| Shape | Warm ms control → candidate | Paired speedup | Faster pairs | Load ms control → candidate | Peak physical footprint MiB control → candidate | Peak RSS MiB control → candidate |
|---|---:|---:|---:|---:|---:|---:|
| 1×2 | 62.6 → 55.6 | 1.14× | 2/3 | 98.9 → 99.7 | 119.0 → 118.7 | 96.5 → 96.2 |
| 48×2 | 1094.5 → 1019.9 | 1.13× | 3/3 | 364.0 → 363.7 | 1403.5 → 1431.7 | 1664.9 → 1664.0 |
| 49×2 | 1113.8 → 979.3 | 1.17× | 3/3 | 361.4 → 386.3 | 1438.3 → 1437.7 | 1684.9 → 1686.1 |
| merge-54×1 | 1088.8 → 1260.5 | 1.08× | 2/3 | 366.6 → 369.5 | 1466.1 → 1444.0 | 1738.2 → 1741.5 |

The 1×2 shape controls for run-to-run variation. The candidate also passed 656 native proof verifications on the M5 Pro, with eight CPU and eight Metal proofs for each of the 41 pinned shapes.

The final build `990508c` preserves the benchmarked default Metal library and the control’s size-optimized library byte for byte. It verified all eight blocks in a repeat of merge-54×1 with alternating build order. The median paired speedup was 1.27×, with 3/4 pairs faster.

| Shape | Warm ms control → final | Load ms control → final | Peak physical footprint MiB control → final | Peak RSS MiB control → final |
|---|---:|---:|---:|---:|
| merge-54×1 | 1304.4 → 1032.7 | 394.0 → 374.0 | 1425.2 → 1454.2 | 1738.0 → 1739.1 |

The default Metal reference tests passed on the M5 Pro. Forcing size optimization reproduced the G1 and G2 reference-test failures seen in the control.

## iPhone 17e bounded witness solving and quotient window trial

The signed experimental build `cd0ca01` ran on the wired iPhone 17e with the original pinned keys and witnesses. The control used the release settings of a 13-bit quotient window cap and the existing witness solver. The trial used a bounded schedule for recognized Poseidon operations and a 15-bit quotient window cap on merge-54×1 and 49×2. Each setting changed independently within the same build. The release results are in the full-catalog section above.

Each setting ran three fresh processes. All 36 process blocks verified, for 288 proofs. The phone reported thermal state `serious`. Warm latency averages the last five of eight prove-and-verify times per process. The table uses medians across processes. Paired speedup is the median of the within-block ratios to the control.

[Raw paired solver and quotient trial blocks](artifacts/iphone-17e-bounded-quotient-trial-blocks.jsonl)

| Shape | Quotient window cap | Bounded witness schedule | Warm ms | Paired speedup | Faster pairs | Load ms | Peak physical footprint MiB | Peak RSS MiB |
|---|---:|---|---:|---:|---:|---:|---:|---:|
| merge-54×1 | 13 | Off | 1572.5 | — | — | 382.3 | 1449.3 | 1741.2 |
| merge-54×1 | 13 | On | 1467.5 | 1.05× | 3/3 | 706.6 | 1468.5 | 1743.8 |
| merge-54×1 | 15 | Off | 1544.8 | 1.02× | 2/3 | 378.2 | 1413.3 | 1766.0 |
| merge-54×1 | 15 | On | 1306.4 | 1.20× | 3/3 | 708.2 | 1447.7 | 1773.5 |
| 49×2 | 13 | Off | 1720.0 | — | — | 358.7 | 1381.2 | 1685.3 |
| 49×2 | 13 | On | 1638.0 | 1.05× | 3/3 | 665.4 | 1434.2 | 1690.1 |
| 49×2 | 15 | Off | 1730.9 | 1.00× | 1/3 | 358.7 | 1398.8 | 1707.7 |
| 49×2 | 15 | On | 1559.4 | 1.11× | 3/3 | 679.4 | 1414.2 | 1715.3 |
| 24×2 | 13 | Off | 544.2 | — | — | 241.5 | 774.1 | 885.2 |
| 24×2 | 13 | On | 791.3 | 1.08× | 2/3 | 405.0 | 775.9 | 887.5 |
| 8×2 | 13 | Off | 180.5 | — | — | 151.5 | 323.3 | 324.8 |
| 8×2 | 13 | On | 165.2 | 1.07× | 3/3 | 207.8 | 323.5 | 326.3 |

The combined settings improved warm latency in all three pairs for each large shape. Median key load for merge-54×1 rose from 382 to 708 ms and peak RSS rose by 32 MiB. For 24×2, the bounded schedule was faster in two pairs, and its median warm latency rose from 544 to 791 ms.

## Warm proof latency in milliseconds

| Shape | M5 CPU | M5 Metal | AWS Zopro | AWS Aeglos | AWS Gnark | iPhone 15 CPU | iPhone 15 Metal | iPhone 17 Pro CPU | iPhone 17 Pro Metal | Galaxy A56 CPU | Galaxy S25 CPU | iPhone 17e CPU | iPhone 17e Metal |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1×2 | 15.7 | 11.5 | 7.8 | 8.3 | 49.8 | 166.7 | 108.0 | 100.6 | 60.4 | 304.2 | 131.8 | 123.5 | 79.2 |
| 1×4 | 26.0 | 14.6 | 8.7 | 10.2 | 60.7 | 209.5 | 144.3 | 135.1 | 69.6 | 407.1 | 184.6 | 177.3 | 108.8 |
| 1×8 | 31.1 | 17.1 | 10.6 | 10.9 | 73.0 | 235.8 | 163.7 | 154.4 | 80.2 | 464.3 | 209.1 | 185.8 | 117.5 |
| 1×16 | 33.5 | 20.8 | 11.9 | 12.7 | 86.1 | 299.3 | 192.6 | 192.7 | 99.8 | 550.3 | 261.6 | 284.4 | 178.9 |
| 2×2 | 23.7 | 15.3 | 10.3 | 12.2 | 83.1 | 287.0 | 180.1 | 180.9 | 92.1 | 497.7 | 238.2 | 277.0 | 138.2 |
| 2×4 | 24.9 | 17.8 | 11.4 | 12.4 | 88.1 | 300.9 | 185.4 | 189.7 | 97.2 | 520.5 | 246.3 | 341.5 | 155.0 |
| 2×8 | 43.9 | 28.1 | 13.3 | 15.4 | 109.0 | 423.7 | 247.1 | 263.0 | 114.7 | 707.4 | 327.9 | 466.6 | 237.2 |
| 2×16 | 48.2 | 29.0 | 17.1 | 17.3 | 121.2 | 493.3 | 293.8 | 317.0 | 157.4 | 786.5 | 456.2 | 548.9 | 292.5 |
| 3×2 | 42.5 | 27.2 | 14.3 | 16.3 | 120.5 | 462.3 | 296.2 | 289.1 | 136.1 | 743.1 | 430.8 | 496.3 | 238.2 |
| 3×4 | 43.8 | 28.1 | 14.8 | 17.2 | 124.1 | 492.2 | 335.2 | 297.9 | 143.0 | 759.6 | 369.2 | 501.6 | 254.5 |
| 3×8 | 45.0 | 27.9 | 15.6 | 17.8 | 134.0 | 507.9 | 333.6 | 318.5 | 155.9 | 797.3 | 391.1 | 587.9 | 357.9 |
| 4×2 | 45.0 | 28.2 | 14.1 | 19.1 | 144.4 | 536.3 | 354.1 | 341.0 | 166.8 | 841.9 | 534.5 | 551.1 | 394.7 |
| 4×4 | 44.8 | 27.5 | 15.9 | 18.9 | 148.9 | 563.2 | 351.9 | 348.5 | 173.5 | 872.0 | 489.9 | 636.4 | 349.9 |
| 4×8 | 47.2 | 28.8 | 16.8 | 19.9 | 155.1 | 630.5 | 460.6 | 369.3 | 186.4 | 873.7 | 518.8 | 652.5 | 333.5 |
| 4×16 | 52.8 | 32.2 | 18.8 | 21.8 | 169.6 | 727.4 | 573.6 | 419.6 | 210.5 | 973.7 | 520.5 | 716.0 | 394.0 |
| 5×2 | 45.6 | 30.1 | 21.0 | 20.9 | 167.5 | 658.4 | 498.5 | 392.1 | 197.5 | 993.2 | 612.7 | 636.3 | 427.6 |
| 5×4 | 46.1 | 31.2 | 20.4 | 21.0 | 169.3 | 673.3 | 505.6 | 401.9 | 205.0 | 961.6 | 548.9 | 697.4 | 482.7 |
| 5×8 | 66.3 | 36.8 | 25.6 | 27.3 | 200.8 | 926.6 | 763.4 | 533.3 | 223.5 | 1271.7 | 651.0 | 971.9 | 630.4 |
| 5×16 | 71.1 | 39.0 | 26.5 | 27.9 | 213.9 | 1028.1 | 863.1 | 568.8 | 250.8 | 1321.6 | 807.2 | 963.8 | 689.8 |
| 6×2 | 65.1 | 31.3 | 25.1 | 28.1 | 216.3 | 1001.1 | 826.3 | 549.6 | 237.4 | 1292.7 | 715.9 | 986.0 | 741.4 |
| 6×4 | 67.2 | 36.3 | 25.5 | 27.9 | 217.0 | 993.4 | 856.8 | 567.9 | 241.2 | 1333.4 | 768.0 | 958.9 | 724.9 |
| 6×8 | 67.8 | 37.6 | 26.4 | 29.0 | 221.2 | 1022.6 | 909.0 | 586.1 | 253.5 | 1372.5 | 760.6 | 954.1 | 739.8 |
| 8×2 | 67.3 | 33.5 | 18.7 | 31.1 | 253.2 | 1184.8 | 1113.8 | 643.0 | 299.6 | 1505.2 | 875.4 | 1063.2 | 799.3 |
| 8×4 | 67.8 | 33.9 | 27.5 | 32.4 | 254.3 | 1159.5 | 1112.5 | 667.2 | 300.1 | 1534.4 | 883.1 | 1053.1 | 803.9 |
| 8×8 | 70.7 | 35.8 | 28.4 | 33.1 | 262.5 | 1169.6 | 1114.7 | 679.0 | 312.8 | 1547.5 | 893.2 | 1084.8 | 833.9 |
| 8×16 | 75.7 | 41.2 | 30.6 | 34.9 | 274.8 | 1238.5 | 1188.4 | 715.3 | 337.9 | 1685.2 | 953.2 | 1144.9 | 902.6 |
| 12×2 | 111.8 | 48.7 | 29.5 | 49.6 | 382.7 | 1783.4 | 1573.4 | 1014.5 | 441.1 | 2457.9 | 1328.4 | 1629.9 | 1223.4 |
| 12×4 | 120.1 | 50.5 | 36.6 | 49.6 | 393.3 | 1790.6 | 1593.7 | 1044.6 | 437.9 | 2472.1 | 1461.6 | 1656.8 | 1213.5 |
| 12×8 | 120.1 | 51.0 | 37.8 | 52.7 | 393.4 | 1828.1 | 1677.1 | 1061.9 | 440.9 | 2530.2 | 1427.0 | 1709.0 | 1246.0 |
| 16×2 | 114.4 | 52.5 | 33.4 | 58.1 | 459.6 | 2139.8 | 1782.9 | 1209.9 | 516.6 | 2810.2 | 1759.9 | 1895.8 | 1401.8 |
| 16×4 | 121.4 | 52.2 | 42.6 | 58.2 | 462.5 | 2117.6 | 1881.3 | 1234.7 | 537.8 | 2828.3 | 1724.1 | 1897.3 | 1414.4 |
| 16×8 | 127.9 | 55.2 | 43.2 | 59.4 | 463.6 | 2094.0 | 1939.2 | 1233.3 | 536.8 | 2886.7 | 1792.2 | 1935.2 | 1435.5 |
| 24×2 | 256.5 | 88.7 | 47.5 | 87.4 | 660.1 | 3301.4 | 2525.8 | 1963.6 | 804.8 | 4625.5 | 2940.6 | 3064.9 | 2063.6 |
| 24×4 | 240.7 | 88.6 | 58.5 | 88.2 | 665.6 | 3338.0 | 2512.3 | 1965.6 | 810.8 | 4614.6 | 2822.8 | 3103.3 | 2066.9 |
| 32×2 | 247.1 | 93.3 | 69.1 | 103.5 | 770.8 | 3873.7 | 2932.5 | 2287.9 | 988.3 | 5329.2 | 3464.2 | 3647.0 | 2440.0 |
| 40×2 | 247.2 | 99.5 | 80.4 | 118.6 | 871.0 | 4423.0 | 3401.7 | 2622.9 | 1209.2 | 6067.8 | 3795.1 | 4100.1 | 2833.2 |
| 48×2 | 556.0 | 265.3 | 104.9 | 167.5 | 1147.3 | 6374.1 | 5026.2 | 3841.2 | 1592.2 | 8797.1 | 5620.5 | 5959.9 | 3886.9 |
| 49×2 | 563.8 | 267.8 | 108.3 | 170.0 | 1100.7 | 6437.4 | 5099.5 | 3843.2 | 1720.2 | 8847.5 | 6044.4 | 6074.3 | 3938.2 |
| merge-8×1 | 65.0 | 32.4 | 29.2 | 30.6 | 243.7 | 1101.5 | 1016.6 | 645.2 | 290.7 | 1475.3 | 932.4 | 971.4 | 680.7 |
| merge-24×1 | 237.2 | 87.4 | 46.2 | 86.1 | 683.5 | 3199.5 | 2469.0 | 1898.8 | 783.4 | 4510.8 | 3062.1 | 3043.4 | 2023.3 |
| merge-54×1 | 550.8 | 266.7 | 105.2 | 174.6 | 1203.7 | 6670.8 | 5246.2 | 4007.2 | 1703.9 | 9245.2 | 5936.7 | 6271.4 | 4116.1 |

## Phone key load in milliseconds

| Shape | iPhone 15 CPU | iPhone 15 Metal | iPhone 17 Pro CPU | iPhone 17 Pro Metal | Galaxy A56 CPU | Galaxy S25 CPU | iPhone 17e CPU | iPhone 17e Metal |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1×2 | 28.8 | 80.4 | 18.2 | 71.8 | 78.3 | 42.5 | 32.4 | 103.7 |
| 1×4 | 27.0 | 78.7 | 29.0 | 68.2 | 68.1 | 37.1 | 37.3 | 99.4 |
| 1×8 | 31.9 | 82.6 | 23.9 | 65.5 | 66.1 | 43.7 | 34.4 | 112.2 |
| 1×16 | 32.1 | 84.8 | 18.1 | 66.9 | 63.4 | 41.8 | 35.2 | 122.5 |
| 2×2 | 32.0 | 84.6 | 18.7 | 69.8 | 61.5 | 40.5 | 33.9 | 110.0 |
| 2×4 | 31.5 | 90.1 | 24.7 | 75.3 | 71.8 | 41.6 | 38.2 | 116.0 |
| 2×8 | 40.0 | 95.0 | 24.0 | 76.3 | 83.2 | 49.0 | 38.9 | 119.7 |
| 2×16 | 42.6 | 96.6 | 26.2 | 83.0 | 88.6 | 56.3 | 38.2 | 128.2 |
| 3×2 | 42.1 | 100.1 | 34.9 | 79.5 | 83.1 | 56.8 | 39.2 | 132.0 |
| 3×4 | 39.4 | 98.0 | 22.9 | 83.3 | 70.4 | 46.9 | 36.1 | 122.6 |
| 3×8 | 39.8 | 97.1 | 23.4 | 79.1 | 78.9 | 47.7 | 39.2 | 126.2 |
| 4×2 | 43.7 | 105.2 | 29.1 | 95.0 | 88.4 | 65.7 | 41.6 | 127.4 |
| 4×4 | 42.9 | 101.1 | 26.0 | 87.1 | 85.7 | 59.5 | 40.5 | 130.5 |
| 4×8 | 46.0 | 124.2 | 24.3 | 87.8 | 84.7 | 60.3 | 41.8 | 127.2 |
| 4×16 | 46.5 | 129.0 | 30.1 | 99.9 | 86.0 | 57.7 | 45.1 | 136.4 |
| 5×2 | 45.6 | 118.7 | 31.8 | 98.1 | 93.9 | 68.2 | 42.5 | 138.2 |
| 5×4 | 42.8 | 125.0 | 27.5 | 90.8 | 90.4 | 66.2 | 44.4 | 137.4 |
| 5×8 | 51.3 | 154.4 | 35.2 | 103.6 | 95.1 | 73.2 | 46.3 | 143.6 |
| 5×16 | 55.6 | 141.1 | 32.6 | 110.2 | 102.2 | 71.2 | 46.8 | 146.0 |
| 6×2 | 52.9 | 136.5 | 42.4 | 106.5 | 101.0 | 69.9 | 48.5 | 145.4 |
| 6×4 | 51.8 | 139.6 | 40.3 | 100.7 | 100.2 | 71.3 | 48.4 | 146.8 |
| 6×8 | 53.8 | 131.1 | 35.8 | 111.0 | 111.5 | 68.0 | 52.5 | 141.3 |
| 8×2 | 58.8 | 130.1 | 36.8 | 111.3 | 109.0 | 77.2 | 52.8 | 154.7 |
| 8×4 | 58.2 | 140.7 | 40.2 | 109.4 | 111.6 | 83.0 | 55.7 | 152.0 |
| 8×8 | 60.8 | 182.6 | 41.4 | 115.2 | 110.0 | 80.6 | 54.6 | 151.3 |
| 8×16 | 61.3 | 154.5 | 42.9 | 115.9 | 118.4 | 80.9 | 54.5 | 161.7 |
| 12×2 | 81.5 | 179.7 | 53.7 | 138.6 | 164.2 | 104.4 | 65.5 | 176.0 |
| 12×4 | 76.7 | 160.6 | 55.3 | 140.1 | 152.7 | 109.7 | 63.2 | 169.0 |
| 12×8 | 77.0 | 168.9 | 51.5 | 138.7 | 166.5 | 108.2 | 65.5 | 174.0 |
| 16×2 | 84.8 | 187.7 | 60.0 | 155.9 | 162.9 | 129.0 | 74.2 | 191.9 |
| 16×4 | 82.8 | 188.2 | 55.4 | 152.9 | 167.4 | 113.6 | 70.3 | 185.3 |
| 16×8 | 91.4 | 189.3 | 56.7 | 155.1 | 182.7 | 117.8 | 71.7 | 185.4 |
| 24×2 | 124.5 | 277.3 | 83.6 | 203.2 | 232.0 | 194.1 | 92.2 | 244.0 |
| 24×4 | 113.9 | 251.2 | 76.9 | 198.7 | 226.6 | 169.0 | 94.0 | 238.5 |
| 32×2 | 148.5 | 285.9 | 87.3 | 230.8 | 284.8 | 202.1 | 104.5 | 257.2 |
| 40×2 | 155.2 | 325.4 | 104.6 | 258.5 | 273.5 | 226.9 | 117.9 | 285.8 |
| 48×2 | 196.2 | 419.7 | 127.0 | 325.2 | 396.2 | 310.6 | 148.6 | 344.4 |
| 49×2 | 193.8 | 428.4 | 126.5 | 322.1 | 399.2 | 284.4 | 152.6 | 351.9 |
| merge-8×1 | 54.7 | 140.3 | 39.6 | 122.1 | 113.8 | 80.4 | 49.0 | 144.2 |
| merge-24×1 | 111.6 | 243.9 | 79.1 | 203.3 | 240.4 | 162.6 | 89.8 | 219.9 |
| merge-54×1 | 202.9 | 431.7 | 130.1 | 326.7 | 421.0 | 282.7 | 153.2 | 367.0 |

## Phone key load through first verified proof in milliseconds

| Shape | iPhone 15 CPU | iPhone 15 Metal | iPhone 17 Pro CPU | iPhone 17 Pro Metal | Galaxy A56 CPU | Galaxy S25 CPU | iPhone 17e CPU | iPhone 17e Metal |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1×2 | 192.5 | 263.2 | 122.2 | 144.9 | 398.4 | 178.6 | 154.4 | 245.7 |
| 1×4 | 225.8 | 265.4 | 164.4 | 155.4 | 487.6 | 229.5 | 201.0 | 273.1 |
| 1×8 | 261.4 | 287.8 | 175.9 | 156.4 | 525.5 | 252.2 | 224.4 | 314.0 |
| 1×16 | 321.3 | 322.6 | 208.1 | 176.6 | 610.3 | 296.0 | 258.5 | 342.2 |
| 2×2 | 302.7 | 306.1 | 194.7 | 175.0 | 557.3 | 275.8 | 248.7 | 302.1 |
| 2×4 | 324.5 | 318.4 | 215.6 | 179.7 | 623.2 | 297.8 | 256.9 | 309.0 |
| 2×8 | 457.6 | 385.0 | 283.4 | 222.5 | 780.7 | 381.1 | 340.4 | 373.3 |
| 2×16 | 521.2 | 426.5 | 330.9 | 256.0 | 863.1 | 524.0 | 406.0 | 442.6 |
| 3×2 | 490.3 | 412.1 | 315.9 | 234.2 | 828.0 | 484.8 | 377.2 | 415.2 |
| 3×4 | 506.2 | 437.5 | 318.6 | 243.1 | 838.1 | 424.8 | 378.5 | 400.0 |
| 3×8 | 538.4 | 422.2 | 338.5 | 243.8 | 906.0 | 451.0 | 415.7 | 419.6 |
| 4×2 | 563.9 | 455.0 | 371.1 | 270.1 | 917.8 | 594.1 | 433.3 | 448.2 |
| 4×4 | 576.5 | 459.8 | 373.4 | 260.4 | 946.8 | 544.0 | 444.8 | 437.8 |
| 4×8 | 598.1 | 485.7 | 409.3 | 274.8 | 965.2 | 596.9 | 459.5 | 455.4 |
| 4×16 | 700.8 | 516.1 | 462.3 | 308.6 | 1060.5 | 575.7 | 591.7 | 495.5 |
| 5×2 | 620.1 | 507.0 | 423.0 | 301.3 | 1008.8 | 688.3 | 474.1 | 462.1 |
| 5×4 | 637.0 | 503.4 | 445.4 | 290.6 | 1042.5 | 638.7 | 509.2 | 479.9 |
| 5×8 | 832.9 | 646.3 | 570.3 | 325.9 | 1329.4 | 882.3 | 648.3 | 540.0 |
| 5×16 | 865.6 | 634.2 | 595.0 | 358.0 | 1434.5 | 846.1 | 756.2 | 547.9 |
| 6×2 | 838.3 | 633.0 | 591.5 | 341.8 | 1395.3 | 768.9 | 698.6 | 538.4 |
| 6×4 | 866.4 | 647.6 | 598.0 | 348.3 | 1394.0 | 819.8 | 671.7 | 550.7 |
| 6×8 | 934.6 | 637.9 | 626.5 | 369.4 | 1462.4 | 804.7 | 698.0 | 561.8 |
| 8×2 | 1002.8 | 683.5 | 682.5 | 414.0 | 1665.8 | 915.7 | 920.2 | 601.5 |
| 8×4 | 1024.0 | 727.0 | 704.3 | 403.0 | 1626.6 | 1005.2 | 1020.1 | 612.0 |
| 8×8 | 1055.5 | 756.7 | 721.8 | 413.6 | 1670.2 | 993.0 | 814.8 | 620.3 |
| 8×16 | 1087.1 | 740.2 | 754.2 | 443.6 | 1735.8 | 1008.3 | 853.3 | 630.0 |
| 12×2 | 1529.7 | 1028.9 | 1064.6 | 564.6 | 2606.6 | 1581.9 | 1283.6 | 825.1 |
| 12×4 | 1534.7 | 1047.3 | 1104.6 | 564.2 | 2586.8 | 1601.6 | 1148.5 | 784.8 |
| 12×8 | 1552.5 | 1017.7 | 1091.9 | 566.5 | 2667.9 | 1532.2 | 1305.1 | 796.5 |
| 16×2 | 1744.5 | 1115.7 | 1269.0 | 642.0 | 2986.9 | 1916.2 | 1435.2 | 933.0 |
| 16×4 | 1778.5 | 1151.2 | 1245.6 | 679.8 | 3006.0 | 1688.5 | 1388.2 | 920.1 |
| 16×8 | 1826.1 | 1139.6 | 1283.8 | 667.1 | 3062.7 | 1682.2 | 1595.1 | 904.4 |
| 24×2 | 2950.2 | 1885.9 | 2123.6 | 911.2 | 4750.6 | 3388.7 | 2715.8 | 1406.5 |
| 24×4 | 2931.8 | 1821.9 | 2027.5 | 901.7 | 4668.3 | 3060.4 | 2819.9 | 1392.9 |
| 32×2 | 3494.5 | 2118.6 | 2336.3 | 1139.2 | 5544.8 | 3481.5 | 2906.3 | 1564.7 |
| 40×2 | 3910.7 | 2456.6 | 2701.8 | 1376.6 | 6349.4 | 3965.1 | 3270.9 | 1780.8 |
| 48×2 | 6294.3 | 4065.1 | 3950.8 | 1662.2 | 9014.1 | 6023.3 | 5443.3 | 2696.8 |
| 49×2 | 6078.5 | 4134.0 | 3920.0 | 1704.9 | 9220.7 | 5671.2 | 5017.2 | 2658.7 |
| merge-8×1 | 945.6 | 667.2 | 677.3 | 399.6 | 1567.8 | 991.8 | 724.9 | 577.3 |
| merge-24×1 | 2828.9 | 1788.9 | 1949.4 | 857.7 | 4696.5 | 2911.4 | 2381.7 | 1286.3 |
| merge-54×1 | 6556.8 | 4150.5 | 4053.5 | 1771.8 | 9551.5 | 5964.4 | 5544.0 | 3326.6 |

## Peak process RSS in MiB

| Shape | M5 CPU | M5 Metal | AWS Zopro | AWS Aeglos | AWS Gnark | iPhone 15 CPU | iPhone 15 Metal | iPhone 17 Pro CPU | iPhone 17 Pro Metal | Galaxy A56 CPU | Galaxy S25 CPU | iPhone 17e CPU | iPhone 17e Metal |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1×2 | 71.1 | 93.3 | 40.8 | 452.9 | 264.9 | 109.0 | 165.0 | 107.2 | 173.0 | 108.8 | 109.6 | 55.1 | 119.8 |
| 1×4 | 66.6 | 92.2 | 43.0 | 571.0 | 393.4 | 118.6 | 183.2 | 116.7 | 189.3 | 109.0 | 110.0 | 64.8 | 138.6 |
| 1×8 | 75.2 | 109.0 | 62.6 | 674.0 | 413.4 | 124.8 | 192.6 | 124.0 | 199.0 | 109.2 | 110.5 | 70.9 | 148.0 |
| 1×16 | 97.0 | 130.0 | 75.4 | 805.4 | 421.2 | 134.6 | 209.7 | 133.7 | 218.3 | 110.2 | 111.0 | 81.0 | 168.8 |
| 2×2 | 95.8 | 126.4 | 60.9 | 803.8 | 433.1 | 131.0 | 204.1 | 130.2 | 212.8 | 110.2 | 113.7 | 77.4 | 161.7 |
| 2×4 | 99.0 | 114.4 | 138.1 | 807.3 | 436.6 | 133.4 | 208.0 | 132.6 | 216.5 | 110.2 | 113.7 | 79.7 | 167.2 |
| 2×8 | 131.4 | 184.2 | 140.4 | 1003.1 | 531.8 | 152.6 | 246.7 | 151.5 | 256.9 | 110.2 | 113.7 | 101.4 | 205.3 |
| 2×16 | 150.9 | 219.6 | 174.8 | 1190.0 | 537.8 | 164.8 | 275.8 | 164.3 | 292.8 | 110.2 | 113.7 | 112.5 | 243.9 |
| 3×2 | 135.4 | 195.5 | 147.2 | 1154.2 | 503.8 | 161.1 | 268.5 | 159.8 | 286.9 | 110.2 | 113.7 | 107.4 | 234.9 |
| 3×4 | 138.6 | 200.8 | 153.0 | 1192.7 | 573.4 | 163.2 | 273.4 | 162.4 | 291.3 | 110.5 | 113.7 | 112.0 | 240.3 |
| 3×8 | 148.0 | 213.3 | 163.1 | 1260.7 | 546.2 | 168.8 | 282.8 | 167.7 | 300.0 | 110.7 | 113.7 | 120.5 | 251.2 |
| 4×2 | 151.7 | 220.9 | 180.2 | 1329.5 | 535.8 | 174.8 | 299.1 | 173.7 | 315.8 | 111.7 | 113.7 | 121.8 | 266.9 |
| 4×4 | 156.8 | 227.6 | 169.1 | 1346.4 | 570.8 | 177.0 | 302.5 | 175.8 | 321.5 | 111.7 | 113.7 | 126.4 | 274.3 |
| 4×8 | 166.7 | 238.1 | 194.4 | 1445.6 | 627.9 | 182.5 | 311.5 | 181.0 | 331.2 | 113.2 | 113.8 | 128.5 | 279.8 |
| 4×16 | 184.5 | 263.0 | 209.4 | 1568.8 | 578.1 | 193.8 | 338.9 | 193.0 | 349.1 | 115.4 | 125.0 | 146.5 | 303.6 |
| 5×2 | 170.7 | 243.6 | 209.4 | 1526.9 | 585.5 | 188.3 | 321.4 | 187.1 | 344.3 | 113.2 | 118.6 | 135.9 | 295.0 |
| 5×4 | 169.3 | 248.5 | 223.7 | 1551.2 | 532.3 | 190.5 | 326.9 | 193.1 | 348.3 | 114.1 | 121.3 | 140.0 | 300.8 |
| 5×8 | 203.8 | 272.9 | 288.2 | 1909.8 | 677.0 | 221.7 | 398.6 | 220.6 | 408.8 | 147.4 | 152.4 | 175.4 | 356.1 |
| 5×16 | 218.4 | 291.9 | 321.9 | 2012.2 | 765.6 | 231.5 | 416.1 | 230.6 | 427.6 | 157.3 | 162.5 | 185.8 | 379.3 |
| 6×2 | 207.4 | 255.0 | 280.1 | 1972.4 | 729.1 | 226.9 | 408.7 | 225.8 | 420.5 | 152.0 | 158.0 | 182.1 | 371.1 |
| 6×4 | 210.3 | 281.3 | 304.0 | 1992.8 | 754.1 | 229.3 | 411.9 | 228.4 | 424.0 | 155.6 | 160.2 | 184.0 | 376.0 |
| 6×8 | 217.0 | 292.5 | 316.8 | 2145.7 | 658.4 | 234.8 | 422.7 | 234.0 | 433.7 | 160.7 | 166.0 | 188.2 | 389.0 |
| 8×2 | 233.2 | 298.7 | 304.2 | 2418.9 | 903.5 | 253.5 | 456.0 | 252.6 | 480.6 | 179.2 | 183.5 | 208.8 | 424.8 |
| 8×4 | 239.4 | 303.3 | 372.6 | 2458.1 | 921.0 | 256.4 | 460.7 | 255.7 | 484.2 | 182.9 | 186.6 | 212.0 | 428.9 |
| 8×8 | 249.4 | 312.2 | 373.1 | 2521.4 | 952.9 | 262.0 | 469.4 | 260.8 | 493.5 | 187.9 | 192.3 | 214.8 | 438.3 |
| 8×16 | 259.7 | 362.6 | 431.0 | 2678.3 | 957.4 | 271.8 | 487.2 | 270.6 | 511.1 | 198.1 | 202.1 | 228.2 | 457.2 |
| 12×2 | 339.8 | 440.7 | 454.0 | 3594.3 | 1296.6 | 363.4 | 658.8 | 362.4 | 696.1 | 290.5 | 293.5 | 317.8 | 644.2 |
| 12×4 | 333.6 | 444.7 | 517.7 | 3621.9 | 1322.0 | 365.8 | 662.6 | 365.0 | 700.5 | 293.8 | 295.8 | 320.7 | 644.5 |
| 12×8 | 335.8 | 457.0 | 537.3 | 3694.5 | 1231.9 | 371.6 | 672.8 | 371.2 | 710.0 | 299.4 | 302.1 | 327.6 | 659.2 |
| 16×2 | 396.5 | 521.2 | 573.4 | 4540.3 | 1466.6 | 417.2 | 753.5 | 416.7 | 794.1 | 344.6 | 347.7 | 374.0 | 746.8 |
| 16×4 | 398.1 | 529.2 | 641.2 | 4576.8 | 1390.8 | 420.3 | 758.4 | 419.5 | 798.8 | 348.1 | 352.8 | 373.4 | 745.5 |
| 16×8 | 401.1 | 535.3 | 673.4 | 4664.6 | 1480.5 | 426.0 | 767.4 | 425.0 | 807.8 | 353.3 | 356.4 | 379.2 | 761.8 |
| 24×2 | 591.2 | 808.1 | 882.0 | 6664.4 | 2518.1 | 638.0 | 1159.8 | 637.3 | 1208.6 | 563.9 | 569.0 | 591.3 | 1157.7 |
| 24×4 | 598.8 | 814.6 | 1029.6 | 6643.7 | 2220.9 | 641.0 | 1165.1 | 640.2 | 1212.1 | 567.7 | 571.3 | 589.3 | 1162.3 |
| 32×2 | 718.1 | 985.2 | 1231.3 | 8284.0 | 2646.2 | 747.0 | 1351.5 | 746.2 | 1405.1 | 672.8 | 677.4 | 699.1 | 1351.0 |
| 40×2 | 827.4 | 1144.6 | 1429.0 | 9650.5 | 3305.2 | 853.8 | 1536.8 | 852.9 | 1601.1 | 780.0 | 784.0 | 806.0 | 1568.9 |
| 48×2 | 1147.8 | 1540.2 | 1693.6 | 12228.6 | 4481.4 | 1185.4 | 2137.7 | 1185.0 | 2219.1 | 1112.2 | 1116.4 | 1131.9 | 2167.3 |
| 49×2 | 1160.8 | 1563.5 | 1552.5 | 12410.4 | 4615.1 | 1198.8 | 2174.0 | 1198.0 | 2249.1 | 1125.5 | 1129.6 | 1146.2 | 2196.4 |
| merge-8×1 | 224.1 | 284.5 | 324.5 | 2339.6 | 700.2 | 247.5 | 444.2 | 246.5 | 467.7 | 174.0 | 178.3 | 198.5 | 413.4 |
| merge-24×1 | 571.9 | 780.1 | 929.5 | 6517.5 | 2300.6 | 622.3 | 1131.8 | 621.1 | 1180.1 | 548.4 | 552.9 | 575.3 | 1127.2 |
| merge-54×1 | 1181.5 | 1595.8 | 1725.7 | 12878.1 | 4907.1 | 1234.5 | 2241.5 | 1233.7 | 2316.0 | 1162.4 | 1165.3 | 1183.8 | 2261.8 |

## Peak Apple physical footprint in MiB

| Shape | M5 CPU | M5 Metal | iPhone 15 CPU | iPhone 15 Metal | iPhone 17 Pro CPU | iPhone 17 Pro Metal | iPhone 17e CPU | iPhone 17e Metal |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1×2 | 49.9 | 168.8 | 23.2 | 100.8 | 26.3 | 196.9 | 20.1 | 157.7 |
| 1×4 | 41.9 | 179.0 | 28.9 | 115.0 | 32.2 | 210.4 | 26.1 | 173.0 |
| 1×8 | 46.8 | 191.4 | 31.4 | 120.1 | 35.3 | 215.9 | 28.6 | 178.2 |
| 1×16 | 61.7 | 205.7 | 34.2 | 131.7 | 38.0 | 229.8 | 31.7 | 193.7 |
| 2×2 | 63.0 | 208.4 | 33.1 | 129.2 | 37.0 | 227.4 | 30.6 | 189.6 |
| 2×4 | 64.6 | 191.0 | 33.9 | 131.4 | 37.7 | 229.9 | 31.4 | 193.5 |
| 2×8 | 89.5 | 244.0 | 45.5 | 163.2 | 49.1 | 261.9 | 44.3 | 223.7 |
| 2×16 | 101.6 | 273.1 | 50.7 | 184.4 | 54.9 | 289.3 | 48.7 | 254.6 |
| 3×2 | 89.4 | 252.0 | 49.8 | 177.8 | 53.1 | 284.2 | 47.0 | 246.1 |
| 3×4 | 91.0 | 256.6 | 50.4 | 181.8 | 54.2 | 288.0 | 49.5 | 251.0 |
| 3×8 | 96.8 | 265.1 | 52.4 | 187.8 | 55.9 | 294.3 | 54.4 | 258.4 |
| 4×2 | 96.2 | 271.2 | 53.8 | 200.5 | 57.6 | 305.3 | 51.0 | 270.1 |
| 4×4 | 99.9 | 276.0 | 54.8 | 202.6 | 58.3 | 309.3 | 54.0 | 276.5 |
| 4×8 | 106.1 | 282.6 | 56.5 | 205.8 | 59.7 | 314.0 | 53.5 | 276.8 |
| 4×16 | 116.9 | 301.5 | 61.0 | 227.1 | 64.8 | 326.9 | 63.4 | 295.5 |
| 5×2 | 105.6 | 286.3 | 57.8 | 213.0 | 61.4 | 324.2 | 56.7 | 288.5 |
| 5×4 | 102.7 | 289.4 | 58.8 | 215.8 | 65.9 | 327.2 | 58.3 | 293.9 |
| 5×8 | 125.5 | 340.3 | 78.2 | 275.1 | 81.6 | 375.5 | 82.5 | 336.9 |
| 5×16 | 133.1 | 352.6 | 80.9 | 284.5 | 84.6 | 386.6 | 86.4 | 352.7 |
| 6×2 | 125.0 | 311.0 | 79.3 | 280.1 | 82.8 | 382.4 | 84.2 | 347.2 |
| 6×4 | 126.3 | 344.3 | 79.9 | 282.5 | 83.7 | 384.1 | 85.3 | 349.8 |
| 6×8 | 129.2 | 353.1 | 81.9 | 287.5 | 85.7 | 390.1 | 86.4 | 359.4 |
| 8×2 | 131.6 | 335.5 | 86.8 | 308.5 | 90.3 | 424.2 | 92.9 | 382.1 |
| 8×4 | 137.6 | 337.3 | 88.3 | 310.0 | 92.0 | 425.2 | 93.9 | 383.6 |
| 8×8 | 142.4 | 342.3 | 90.2 | 315.5 | 93.3 | 430.8 | 93.7 | 389.7 |
| 8×16 | 145.8 | 399.6 | 93.0 | 326.5 | 96.2 | 441.9 | 99.4 | 401.2 |
| 12×2 | 184.1 | 499.2 | 143.0 | 456.4 | 146.2 | 584.3 | 148.0 | 545.7 |
| 12×4 | 176.4 | 501.6 | 143.7 | 458.7 | 147.2 | 587.1 | 149.4 | 545.0 |
| 12×8 | 174.8 | 509.9 | 145.8 | 464.5 | 149.6 | 593.1 | 152.6 | 556.1 |
| 16×2 | 202.8 | 540.5 | 158.8 | 509.7 | 162.5 | 641.5 | 166.3 | 608.3 |
| 16×4 | 202.9 | 547.0 | 160.3 | 512.4 | 163.7 | 645.0 | 164.1 | 605.5 |
| 16×8 | 202.1 | 549.2 | 162.2 | 517.9 | 165.4 | 650.1 | 166.2 | 618.0 |
| 24×2 | 289.0 | 803.6 | 271.1 | 809.6 | 274.5 | 951.0 | 273.8 | 910.9 |
| 24×4 | 294.8 | 808.6 | 272.2 | 812.0 | 275.5 | 952.5 | 269.9 | 913.9 |
| 32×2 | 339.3 | 901.9 | 303.4 | 918.2 | 307.0 | 1066.4 | 306.3 | 1025.6 |
| 40×2 | 373.6 | 989.1 | 334.8 | 1027.2 | 338.5 | 1184.9 | 337.7 | 1155.2 |
| 48×2 | 554.5 | 1435.9 | 527.2 | 1507.1 | 531.1 | 1673.3 | 524.5 | 1632.9 |
| 49×2 | 558.1 | 1438.2 | 531.2 | 1520.3 | 534.6 | 1691.6 | 529.4 | 1646.7 |
| merge-8×1 | 127.1 | 325.4 | 85.2 | 301.5 | 88.8 | 415.4 | 86.8 | 375.2 |
| merge-24×1 | 281.1 | 787.3 | 266.7 | 791.4 | 269.6 | 931.8 | 270.4 | 890.8 |
| merge-54×1 | 553.1 | 1445.6 | 541.2 | 1576.5 | 544.7 | 1747.0 | 541.3 | 1704.7 |

## Measurement details

The phone and M5 builds use Zopro at `f294171359db54af54b8483957f0b3df60ed2fd5`. See the main report for AWS request-level timing and fixture differences. [M5 Pro and AWS C7a benchmarks](README.md)

The iPhone 17e ran the signed release build on a physical device connected over USB. The phone reported a serious thermal state before and after all 246 blocks. The workload order and thermal conditions differed across devices.

Each process loaded a prebuilt raw4 image, primed three proofs, and timed five. Every proof was verified and checked against the pinned public witness. Warm latency is the median of the three process means. Key load and first-proof timing are the medians of three fresh processes. The first-proof measurement begins before key loading. It excludes the earlier archive download and conversion. Memory is the largest process high-water mark in the three runs. Android records native process RSS. iOS records app process RSS and the physical footprint high-water mark. The raw blocks include the thermal state before and after each block.
