# Joint effect of folded FFT output and a smaller product pool

Five fresh blocks compare both changes on four 24×2 inputs. Each comparison charges pool dispatch, allocation and the complete verified proof. The interaction is the within-block combined latency change less the sum of the two individual latency changes. A positive interaction means the combined saving was smaller than the sum.

| Request | Frozen / ordinary / tiled / pool / both ms | Both vs ordinary / pool / tiled | Interaction ms | Peak ordinary / tiled / pool / both MiB | Both vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| Original | 59.463 / 60.227 / 58.077 / 58.313 / 56.579 | -5.52% (4/5) / -2.90% (4/5) / -1.65% (4/5) | +2.066 | 1131.5 / 1110.8 / 891.9 / 996.3 | 9.33× / 3.11× |
| 97 | 59.086 / 57.767 / 58.110 / 57.455 / 56.240 | -3.13% (5/5) / -2.19% (5/5) / -2.86% (4/5) | -0.743 | 1096.5 / 1152.7 / 903.7 / 931.9 | — |
| 99 | 58.572 / 59.426 / 56.707 / 56.863 / 56.726 | -5.20% (5/5) / -0.33% (4/5) / +0.33% (2/5) | +2.651 | 1142.8 / 1101.4 / 892.3 / 936.5 | — |
| 101 | 59.508 / 58.550 / 56.722 / 56.962 / 56.685 | -2.98% (4/5) / -1.43% (3/5) / +0.53% (2/5) | +1.778 | 1096.8 / 1157.7 / 909.2 / 1006.5 | — |

All 104 exports verified independently. [Protocol](protocol.md) · [Samples](artifacts/samples.json.gz) · [Verification](artifacts/verification.json)
