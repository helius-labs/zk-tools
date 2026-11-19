# Public wire offset trial on the 12×2 general solver

The Ryzen 9950X ran the original request and eight held-out transactions at 32 pinned logical workers. Each of seven randomized blocks compared the accepted binary, the prototype without the file and the same prototype with the file. Each process primed three requests and timed eleven warm full SDK calls. Each of the nine inputs also passed an initial file-enabled proof check. The coefficient file was frozen using [eight separate public training inputs](../2026-11-19-large-wire465/README.md).

| Input | Release ms | Disabled ms | Enabled ms | Paired vs release | Paired vs disabled | Peak RSS disabled / enabled MiB |
|---:|---:|---:|---:|---:|---:|---:|
| 0 | 32.525 | 32.246 | 31.475 | -3.18% (7/7) | -2.66% (7/7) | 632.0 / 562.0 |
| 57 | 32.484 | 32.556 | 31.753 | -0.93% (5/7) | -0.92% (6/7) | 601.0 / 566.4 |
| 58 | 32.472 | 32.454 | 31.341 | -3.27% (6/7) | -2.65% (5/7) | 623.0 / 583.4 |
| 59 | 32.499 | 32.480 | 31.502 | -2.95% (7/7) | -4.29% (7/7) | 607.8 / 560.9 |
| 60 | 32.506 | 32.395 | 32.078 | -1.10% (5/7) | -0.31% (4/7) | 609.8 / 582.3 |
| 61 | 32.668 | 32.322 | 31.632 | -3.18% (6/7) | -1.80% (6/7) | 623.5 / 571.6 |
| 62 | 32.413 | 32.582 | 31.513 | -3.16% (5/7) | -3.28% (5/7) | 620.8 / 573.0 |
| 63 | 32.409 | 32.293 | 31.412 | -3.39% (7/7) | -3.14% (7/7) | 608.0 / 555.3 |
| 64 | 32.497 | 32.645 | 31.122 | -3.69% (7/7) | -4.38% (7/7) | 620.8 / 562.6 |

On the original request the prototype measured 31.475 ms, 9.02× the M5 CPU reference and 3.21× the Metal reference. Across the eight held-out requests the median paired change was -3.17% versus release (8/8 faster) and -2.90% versus disabled (8/8 faster). The trial passed the timing criterion of favorable paired medians on the original and at least six held-out improvements against each control.

Eager preparation took 64.73 ms per key. At optional preparation budgets of zero, 1 MiB and 4 MiB the prover selected ordinary proving, ordinary proving, the offset path, respectively. All 201 process exports passed independent Gnark verification. The general-solver change remains experimental. The accepted scorecard continues to use ordinary proving on 12×2.

[Process samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [Candidate patch](artifacts/source.patch)
