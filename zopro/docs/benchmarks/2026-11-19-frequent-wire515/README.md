# Frequent public wire offsets in full 1×2 proofs

Five public requests supplied the training scalars. The existing 1,168-offset file requires agreement on all five. The extended file contains 2,736 offsets and accepts a varying coefficient when it appeared on at least four requests and saved two signed G2 digits per training input on average. The files use the same previously supported format. All three modes use the production binary with 32 main workers and an eight-worker frontend. The original contributed to training. Requests 5–8 and 25–32 were excluded, with the latter eight covering private splits, deposits, withdrawals and data-bearing transactions. Seven randomized blocks per input use three warmups and 31 timed full calls. Peak RSS covers setup.

| Request | Off / stable / frequent ms | Frequent vs stable | Frequent vs off | Peak MiB stable / frequent |
|---|---:|---:|---:|---:|
| Original | 7.206 / 7.099 / 7.106 | -0.17% (5/7) | -1.75% (7/7) | 64.5 / 65.2 |
| 5 | 7.065 / 6.979 / 6.858 | -1.55% (7/7) | -2.87% (7/7) | 65.5 / 64.5 |
| 6 | 7.105 / 7.009 / 6.826 | -2.40% (7/7) | -3.71% (7/7) | 66.0 / 66.5 |
| 7 | 7.078 / 6.965 / 6.842 | -1.64% (7/7) | -3.16% (7/7) | 65.2 / 63.0 |
| 8 | 7.044 / 7.005 / 6.874 | -2.84% (7/7) | -2.97% (7/7) | 66.2 / 64.0 |
| 25 | 7.075 / 6.947 / 6.908 | -0.84% (7/7) | -2.38% (7/7) | 62.8 / 66.2 |
| 26 | 7.092 / 6.962 / 6.907 | -1.14% (6/7) | -3.19% (6/7) | 79.8 / 84.0 |
| 27 | 7.099 / 6.960 / 6.973 | -0.50% (4/7) | -1.80% (6/7) | 64.8 / 67.0 |
| 28 | 7.055 / 6.964 / 6.873 | -1.71% (7/7) | -2.41% (7/7) | 63.2 / 64.5 |
| 29 | 7.117 / 6.959 / 6.902 | -0.11% (4/7) | -2.03% (7/7) | 64.0 / 65.5 |
| 30 | 7.068 / 6.970 / 6.920 | -0.89% (4/7) | -1.64% (6/7) | 67.2 / 65.2 |
| 31 | 7.074 / 7.022 / 6.878 | -1.65% (6/7) | -2.88% (7/7) | 64.2 / 64.8 |
| 32 | 7.086 / 6.981 / 6.913 | -0.62% (5/7) | -2.05% (7/7) | 63.2 / 65.5 |

The original took 7.106 ms, 5.21× the supplied M5 CPU timing and 3.38× Metal. All 276 process exports verified independently. [Samples](artifacts/processes.json.gz) · [File](artifacts/frequent.wo) · [Budget probes](artifacts/probes.json) · [Identities](artifacts/inputs.json) · [Verification](artifacts/verification.json)
