# More frontend workers within one shared L3

The main pool remains pinned to 32 Ryzen logical CPUs. The auxiliary pool can use eight physical cores in one L3, add four or eight of their SMT siblings, or use all sixteen physical cores across both L3s. Four screening blocks selected a same-L3 SMT width on each shape. Seven new blocks then compared it with eight physical workers in the same L3, all sixteen physical workers, and the default integrated release. Each process timed 51 complete proofs.

| Stage | Shape | Placement | Median ms | Median CPU-ms | Peak MiB | M5 CPU / Metal speedup |
|---|---|---|---:|---:|---:|---:|
| screen | 1x2 | phys8 | 7.168 | 150.5 | 91.2 | 5.16× / 3.35× |
| screen | 1x2 | smt12 | 7.104 | 152.7 | 70.2 | 5.21× / 3.38× |
| screen | 1x2 | smt16 | 7.188 | 159.8 | 70.0 | 5.15× / 3.34× |
| screen | 1x2 | phys16 | 7.284 | 159.7 | 68.8 | 5.08× / 3.29× |
| confirm | 1x2 | phys8 | 7.239 | 151.3 | 64.0 | 5.11× / 3.32× |
| confirm | 1x2 | smt12 | 7.215 | 153.5 | 68.0 | 5.13× / 3.33× |
| confirm | 1x2 | phys16 | 7.310 | 160.9 | 70.2 | 5.06× / 3.28× |
| confirm | 1x2 | release | 7.339 | 152.2 | 67.0 | 5.04× / 3.27× |
| screen | 2x2 | phys8 | 10.136 | 203.4 | 122.2 | 5.82× / 2.96× |
| screen | 2x2 | smt12 | 9.933 | 202.6 | 125.0 | 5.94× / 3.02× |
| screen | 2x2 | smt16 | 10.353 | 222.3 | 132.2 | 5.70× / 2.90× |
| screen | 2x2 | phys16 | 10.493 | 225.0 | 119.2 | 5.62× / 2.86× |
| confirm | 2x2 | phys8 | 10.150 | 204.1 | 120.0 | 5.81× / 2.96× |
| confirm | 2x2 | smt12 | 9.979 | 204.2 | 127.2 | 5.91× / 3.01× |
| confirm | 2x2 | phys16 | 10.374 | 225.6 | 119.8 | 5.69× / 2.89× |
| confirm | 2x2 | release | 10.230 | 203.9 | 116.0 | 5.77× / 2.93× |

| Shape | Selected width | Paired vs release (wins / 7) | Paired vs compact8 (wins / 7) | Compact8 difference by L3 | Qualified |
|---|---|---:|---:|---:|---|
| 1x2 | smt12 | -3.28% (6) | -0.75% (4) | -0.26% / -1.08% | no |
| 2x2 | smt12 | -2.59% (7) | -1.61% (7) | -2.10% / -1.50% | yes |

On 2×2 the twelve-worker placement won all seven pairs against both controls. Its peak RSS was 127.2 MiB compared with 120.0 MiB for compact8. On 1×2 the same placement won four of seven pairs against compact8. The eight-witness 2×2 comparison is the next test before changing the Ryzen profile.

[CPU masks](artifacts/masks.json) · [Protocol](artifacts/protocol.md) · [Process samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)
