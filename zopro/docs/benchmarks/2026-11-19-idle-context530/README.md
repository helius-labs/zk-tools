# Shorter idle scans in the outer worker loop and inside tasks

A worker in Rayon's outer run loop waits on its permanent termination latch. A worker helping while it waits for a join, scope or a cross-pool call uses another latch. The experiment compares the existing default of 32 idle rounds with three policies that shorten the search in both contexts, only in the outer loop, or only inside tasks. The shorter search uses ceil(128 / (workers−1)) rounds in pools of at least 16 workers. The worker identifies its context once per latch wait and retains the existing notification and sleep protocol. Each policy passed 103 Rayon library tests, including concurrent callers and repeated idle-to-busy transitions.

Five randomized blocks use the same binary, public requests and fixed CPU masks within each comparison. Each process warms three calls and times 31 small or eleven large proofs. RSS includes key preparation. Full proof times and CPU and Metal speedups per policy are in the summary.

| Shape | Workers | Default / both / outer / nested ms | Both / outer / nested vs default | CPU-ms default / both / outer / nested | Peak MiB default / both / outer / nested |
|---|---:|---:|---:|---:|---:|
| 1x2 | 32 | 7.173 / 7.124 / 7.161 / 7.160 | -0.19% (3/5) / -0.14% (3/5) / +0.12% (2/5) | 146.4 / 138.5 / 141.1 / 142.6 | 66 / 68 / 64 / 65 |
| 12x2 | 32 | 31.908 / 32.378 / 32.160 / 32.464 | +2.49% (0/5) / +0.79% (2/5) / +3.17% (1/5) | 878.7 / 860.7 / 872.0 / 882.2 | 589 / 462 / 464 / 565 |
| 16x2 | 32 | 39.799 / 40.407 / 40.603 / 40.111 | +3.83% (1/5) / +1.88% (1/5) / +0.13% (2/5) | 1145.5 / 1134.6 / 1152.2 / 1142.0 | 662 / 577 / 569 / 676 |
| 24x2 | 32 | 63.797 / 64.985 / 62.943 / 65.111 | +1.66% (1/5) / -1.14% (4/5) / +2.21% (1/5) | 1845.2 / 1834.3 / 1813.9 / 1834.6 | 1036 / 858 / 874 / 1003 |
| merge-24x1 | 32 | 62.897 / 63.794 / 63.159 / 63.412 | +1.24% (1/5) / +0.32% (1/5) / +0.79% (1/5) | 1798.8 / 1782.4 / 1783.1 / 1789.8 | 1051 / 844 / 833 / 1039 |
| 16x2 | 24 | 39.859 / 40.833 / 39.956 / 40.049 | +2.33% (0/5) / +1.25% (2/5) / +0.75% (1/5) | 865.4 / 861.9 / 863.6 / 864.8 | 617 / 543 / 538 / 621 |
| merge-24x1 | 24 | 63.578 / 63.844 / 63.193 / 63.700 | +0.54% (1/5) / +0.08% (2/5) / +0.93% (2/5) | 1399.7 / 1373.5 / 1378.2 / 1393.8 | 991 / 786 / 837 / 967 |

All 140 process exports verified independently. [Summary](artifacts/summary.json) · [Samples](artifacts/processes.json.gz) · [Scheduler change](artifacts/context.patch) · [Verification](artifacts/verification.json)
