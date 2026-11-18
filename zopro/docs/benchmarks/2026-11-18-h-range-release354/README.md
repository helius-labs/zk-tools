# Memory-admitted two-range H windows

The explicit two-range option applies to the existing 19-window H layout. Each window fills two independent bucket arrays, combines the equal-weight buckets and performs the weighted reduction once. The admission check forecasts the whole layout. A declined request tries the one-range layout and then the ordinary H path. No key tables persist between requests.

The [fresh C7a run](../2026-11-18-h-range-c7a355/README.md) measures the exact integrated executable. The [first C7a comparison](../2026-11-18-h-scale353/README.md) measures the earlier experimental version. The following timings qualify the integrated implementation on the 16-physical-core Ryzen host. Each cell uses five randomized processes with eleven timed SDK proofs each.

| Shape | Setting | Median ms | CPU-ms/proof | Peak MiB |
|---|---|---:|---:|---:|
| 16x2 | reference | 44.818 | 655.0 | 592 |
| 16x2 | off | 45.098 | 660.8 | 604 |
| 16x2 | qualified | 46.592 | 683.7 | 579 |
| 16x2 | two | 46.361 | 681.2 | 583 |
| 24x2 | reference | 72.929 | 1078.8 | 961 |
| 24x2 | off | 72.981 | 1086.6 | 947 |
| 24x2 | qualified | 74.529 | 1112.1 | 964 |
| 24x2 | two | 74.483 | 1111.6 | 962 |
| merge-24x1 | reference | 71.436 | 1036.7 | 901 |
| merge-24x1 | off | 71.862 | 1041.5 | 878 |
| merge-24x1 | qualified | 72.000 | 1057.3 | 902 |
| merge-24x1 | two | 73.000 | 1065.5 | 891 |

| Shape | Integrated enabled vs qualified | Integrated disabled vs retained |
|---|---:|---:|
| 16x2 | -0.50% | +0.62% |
| 24x2 | -0.06% | +0.07% |
| merge-24x1 | +1.39% | +0.60% |

All 143 native tests passed. The core and SDK also passed a generic x86-64 release build check. Both window layouts match the independent generator-sum oracle, including an odd number of points and identity points. All 74 process exports verified. Actual timed routing selected one range by default, two when enabled with ample optional memory, and the ordinary path when the optional allowance was zero.

The requested two-range run selected the one-range layout at optional allowances from 72 through 78 MiB among the tested budgets. These are optional workspace allowances on this fixture. They exclude the loaded key and cannot be used as minimum system RAM.

[Qualifying protocol](artifacts/protocol.md) · [Per-request results](artifacts/processes.json.gz) · [Observed routes](artifacts/routing.json) · [Verification](artifacts/verification.json)
