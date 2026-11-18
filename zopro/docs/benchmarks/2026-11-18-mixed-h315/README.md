# Mixed-width windows for the dense H query

On 16 physical Ryzen cores the 19-window layout improved merge24 full-proof latency by 3.65% in paired blocks, winning all five. It also improved 24×2 by 2.58% in four of five blocks. The 20-window limb-aligned alternative missed the primary threshold. The C7a replication includes the 19-window layout.

The 254-bit scalar is split into seven 14-bit and twelve 13-bit windows. This removes one window relative to uniform 13 bits. Each window scans the full point array and uses the existing affine buckets. The 20-window alternative uses boundaries aligned to scalar limbs. Both reserve positive incremental scratch above the uniform path before execution.

| Shape | Windows | Median ms | Paired vs disabled | Wins / 5 | Median CPU ms | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| merge-24x1 | reference | 73.826 | +0.46% | 2 | 1063 | 887 |
| merge-24x1 | off | 75.863 | +0.00% | — | 1093 | 900 |
| merge-24x1 | 19 | 68.843 | -3.65% | 5 | 1008 | 880 |
| merge-24x1 | word | 71.651 | -0.42% | 3 | 1032 | 906 |
| 16x2 | reference | 46.128 | -0.92% | 3 | 681 | 597 |
| 16x2 | off | 45.811 | +0.00% | — | 681 | 612 |
| 16x2 | 19 | 44.715 | -1.85% | 5 | 653 | 596 |
| 16x2 | word | 48.622 | +2.09% | 1 | 683 | 579 |
| 24x2 | reference | 76.881 | -0.53% | 3 | 1141 | 936 |
| 24x2 | off | 72.927 | +0.00% | — | 1094 | 943 |
| 24x2 | 19 | 70.784 | -2.58% | 4 | 1049 | 987 |
| 24x2 | word | 72.581 | -1.27% | 3 | 1069 | 922 |

Each fresh process primed three times and timed 11 SDK requests. Percentages use paired process means, so they can differ from ratios of the displayed medians. Sixty process exports and four independent routing exports passed verification. The G1 reduction probe recorded 37 calls on the 19-window route, 39 on the limb-aligned route and zero when disabled or when optional memory was denied. A generator-sum test covers full-width scalars, zero, identity and negative points. The patch applies to experiment 306.
