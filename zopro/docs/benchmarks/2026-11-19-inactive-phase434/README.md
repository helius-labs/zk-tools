# 4×2 SDK phase timing with the optional FFT paths disabled

Both releases use the same pinned 4×2 request and 32 workers. Plain processes use the original executables. Traced processes use byte-identical copies with entry and return probes on the SDK, solver, product FFT and curve queries. Their seven consecutive intervals account for each traced SDK request. The table compares the median of process means and the median paired percentage change.

| Observation | Previous / current ms | Paired change | Current slower |
|---|---:|---:|---:|
| plain | 15.458 / 15.369 | -0.96% | 2/6 |
| trace | 15.773 / 15.892 | +0.75% | 4/6 |

| Traced interval | Previous / current mean ms | Median paired difference ms | Paired range ms | Current slower |
|---|---:|---:|---:|---:|
| SDK entry to solver | 0.3684 / 0.3617 | -0.0057 | -0.0122 to -0.0014 | 0/6 |
| Solver | 2.2155 / 2.0256 | -0.0308 | -0.9646 to -0.0137 | 0/6 |
| Solver return to FFT | 0.4065 / 0.3759 | -0.0028 | -0.1485 to +0.0023 | 2/6 |
| FFT product | 2.7743 / 2.7260 | -0.0292 | -0.3661 to +0.2367 | 3/6 |
| FFT return to curve queries | 0.0085 / 0.0083 | -0.0003 | -0.0008 to +0.0006 | 1/6 |
| Curve queries | 9.5920 / 9.6129 | +0.1895 | -0.8030 to +0.3242 | 4/6 |
| Curve return to SDK return | 0.9291 / 0.7785 | +0.0336 | -1.1104 to +0.0793 | 5/6 |

The preceding frontend experiment’s independent verification overlapped part of this run, including block 2. Those samples are retained. The separate ten-block confirmation started after that verification had finished.

The six plain pairs gave a median change of −0.96% and the six traced pairs gave +0.75%. The signs disagree. The second block also shows a large excursion in both directions. [Ten-block follow-up](../2026-11-19-inactive-confirm435/README.md)

[Process samples and intervals](artifacts/processes.json.gz) · [Probe addresses](artifacts/sites.json) · [Verification](artifacts/verification.json) · [Earlier paired test](../2026-11-19-inactive-small432/README.md)
