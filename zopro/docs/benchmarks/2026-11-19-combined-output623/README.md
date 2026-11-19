# Joint effect of folded FFT output and a smaller product pool

The original 24×2 experimental proof took **56.579 ms** with tiled output on the 30-worker product pool, down from 60.227 ms in the ordinary arm. That is **9.33× M5 CPU and 3.11× Metal**, with 3.779 ms still to save to reach the 52.8 ms CPU target. The combined change improved paired median latency on all four inputs against the ordinary arm (18/20 paired blocks), and against the frozen executable (17/20).

For this Ryzen experimental profile, the combined path is the latency choice. Pool dispatch alone had the lowest peak RSS of the four active configurations on all four inputs. Adding tiled output increased its maximum peak by 28–104 MiB depending on the input. On the original, the combined path reached 996.3 MiB, compared with 891.9 MiB for pool dispatch alone and 1,131.5 MiB for the ordinary path. The release SDK pool option has a separate comparison on the ordinary key. [SDK result](../2026-11-19-release-pool621/README.md)

Five fresh blocks compare both changes on four 24×2 inputs. Each comparison charges pool dispatch, allocation and the complete verified proof. The interaction is the within-block combined latency change less the sum of the two individual latency changes. A positive interaction means the combined saving was smaller than the sum.

| Request | Frozen / ordinary / tiled / pool / both ms | Both vs ordinary / pool / tiled | Interaction ms | Peak ordinary / tiled / pool / both MiB | Both vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| Original | 59.463 / 60.227 / 58.077 / 58.313 / 56.579 | -5.52% (4/5) / -2.90% (4/5) / -1.65% (4/5) | +2.066 | 1131.5 / 1110.8 / 891.9 / 996.3 | 9.33× / 3.11× |
| 97 | 59.086 / 57.767 / 58.110 / 57.455 / 56.240 | -3.13% (5/5) / -2.19% (5/5) / -2.86% (4/5) | -0.743 | 1096.5 / 1152.7 / 903.7 / 931.9 | — |
| 99 | 58.572 / 59.426 / 56.707 / 56.863 / 56.726 | -5.20% (5/5) / -0.33% (4/5) / +0.33% (2/5) | +2.651 | 1142.8 / 1101.4 / 892.3 / 936.5 | — |
| 101 | 59.508 / 58.550 / 56.722 / 56.962 / 56.685 | -2.98% (4/5) / -1.43% (3/5) / +0.53% (2/5) | +1.778 | 1096.8 / 1157.7 / 909.2 / 1006.5 | — |

All 104 exports verified independently. [Protocol](protocol.md) · [Samples](artifacts/samples.json.gz) · [Verification](artifacts/verification.json)
