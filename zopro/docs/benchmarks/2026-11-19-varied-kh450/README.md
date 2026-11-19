# K and H bucket sharing across eight 2×2 inputs

The exact public SDK binaries from the original comparison ran eight distinct witnesses. Each worker count and witness had four randomized blocks using each L3 twice. The sixteen-worker profile pinned eight frontend workers. The 32-worker profile pinned twelve. The table reports the median process latency for each arm and the median within-block change. Every process timed 31 verified requests after three warmups.

| Workers | Witness | Release / disabled / enabled ms | Paired vs release | Paired vs disabled | Peak MiB disabled / enabled |
|---:|---:|---:|---:|---:|---:|
| 16 | 1 | 11.571 / 11.504 / 11.330 | -2.84% | -2.29% | 108.0 / 104.0 |
| 16 | 2 | 11.614 / 11.473 / 11.255 | -3.08% | -1.93% | 110.5 / 97.5 |
| 16 | 3 | 11.642 / 11.424 / 11.218 | -3.52% | -2.03% | 108.5 / 102.8 |
| 16 | 4 | 11.543 / 11.516 / 11.357 | -2.23% | -1.40% | 108.8 / 104.2 |
| 16 | 5 | 11.549 / 11.512 / 11.261 | -2.78% | -2.12% | 110.5 / 104.5 |
| 16 | 6 | 11.491 / 11.489 / 11.206 | -3.30% | -3.17% | 108.0 / 102.8 |
| 16 | 7 | 11.611 / 11.514 / 11.284 | -2.55% | -1.64% | 107.5 / 97.2 |
| 16 | 8 | 11.518 / 11.479 / 11.295 | -2.09% | -1.76% | 114.2 / 104.2 |
| 32 | 1 | 9.870 / 9.784 / 9.834 | -0.37% | +0.43% | 117.2 / 111.8 |
| 32 | 2 | 9.925 / 9.842 / 9.790 | -0.97% | -0.12% | 117.2 / 112.8 |
| 32 | 3 | 9.858 / 9.875 / 9.761 | -1.29% | -0.88% | 119.2 / 110.5 |
| 32 | 4 | 9.920 / 9.816 / 9.851 | -0.34% | +0.64% | 115.5 / 111.2 |
| 32 | 5 | 9.937 / 9.872 / 9.782 | -1.67% | -0.43% | 115.5 / 110.8 |
| 32 | 6 | 9.954 / 9.822 / 9.818 | -0.96% | +0.08% | 117.2 / 116.5 |
| 32 | 7 | 9.868 / 9.843 / 9.881 | -0.14% | +0.46% | 119.8 / 114.0 |
| 32 | 8 | 9.911 / 9.803 / 9.755 | -1.75% | -0.52% | 116.0 / 111.2 |

With 16 main workers, the median paired change was -2.81% against the release (8/8 witnesses faster) and -1.98% against the same binary with sharing disabled (8/8 faster).

With 32 main workers, the median paired change was -0.97% against the release (8/8 witnesses faster) and -0.02% against the same binary with sharing disabled (4/8 faster).

All 192 exports passed independent Gnark verification.

[Process samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [Original-input confirmation and source](../2026-11-19-medium-kh-entry448/README.md)
