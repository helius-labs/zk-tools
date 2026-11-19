# Independent 4×2 phase confirmation

Both releases use the same pinned 4×2 request and 32 workers. Plain processes use the original executables. Traced processes use byte-identical copies with entry and return probes on the SDK, solver, product FFT and curve queries. Their seven consecutive intervals account for each traced SDK request. The table compares the median of process means and the median paired percentage change.

| Observation | Previous / current ms | Paired change | Current slower |
|---|---:|---:|---:|
| plain | 15.234 / 15.297 | +0.43% | 6/10 |
| trace | 16.068 / 15.645 | -2.11% | 2/10 |

| Traced interval | Previous / current mean ms | Median paired difference ms | Paired range ms | Current slower |
|---|---:|---:|---:|---:|
| SDK entry to solver | 0.3655 / 0.3627 | -0.0021 | -0.0163 to +0.0039 | 4/10 |
| Solver | 2.0715 / 2.0866 | +0.0441 | -0.2099 to +0.0818 | 8/10 |
| Solver return to FFT | 0.4088 / 0.3861 | -0.0010 | -0.2560 to +0.0262 | 5/10 |
| FFT product | 2.7961 / 2.7502 | +0.0214 | -0.9676 to +0.3170 | 6/10 |
| FFT return to curve queries | 0.0099 / 0.0081 | -0.0003 | -0.0161 to +0.0006 | 3/10 |
| Curve queries | 9.8979 / 9.3500 | -0.4232 | -1.6585 to -0.1026 | 0/10 |
| Curve return to SDK return | 0.7255 / 0.7256 | -0.0011 | -0.0142 to +0.0227 | 4/10 |

The 4×2 gap narrowed from +1.13% in the earlier plain comparison to +0.43% here, with six of ten plain blocks slower. Probing changed the ranking. The curve queries in the newer traced binary returned sooner in all ten blocks. The traced FFT difference was +0.021 ms by median and changed sign across blocks. These data do not establish that the FFT caused the earlier 4×2 regression.

[Process samples and intervals](artifacts/processes.json.gz) · [Probe addresses](artifacts/sites.json) · [Verification](artifacts/verification.json) · [Earlier paired test](../2026-11-19-inactive-small432/README.md)
