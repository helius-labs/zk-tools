# Compact twelve-worker frontend across eight 2×2 witnesses

Four paired process blocks measured each request on 32 Ryzen hardware threads. The main pool stayed fixed. The compact eight-worker and twelve-worker placements used the same shared L3 in each block; the extra four workers used SMT siblings. Both L3s were used twice per witness. The unmodified release used its original eight-worker placement. Each process timed 31 verified calls after three warmups.

| Witness | Exact release / compact8 / compact12 ms | Paired vs release | Paired vs compact8 | Peak MiB compact8 / compact12 |
|---:|---:|---:|---:|---:|
| 1 | 10.151 / 10.159 / 9.965 | -2.00% | -1.93% | 118.0 / 123.0 |
| 2 | 10.241 / 10.139 / 9.955 | -3.07% | -1.60% | 117.8 / 125.2 |
| 3 | 10.219 / 10.226 / 9.940 | -2.58% | -2.52% | 117.5 / 122.0 |
| 4 | 10.245 / 10.226 / 9.929 | -3.08% | -2.27% | 118.5 / 123.2 |
| 5 | 10.272 / 10.139 / 9.991 | -2.73% | -1.43% | 114.0 / 124.2 |
| 6 | 10.310 / 10.171 / 9.997 | -3.04% | -1.79% | 117.0 / 122.8 |
| 7 | 10.189 / 10.157 / 9.925 | -2.81% | -2.47% | 117.5 / 122.8 |
| 8 | 10.162 / 10.144 / 10.002 | -1.39% | -1.26% | 113.8 / 123.5 |

Across the eight witnesses the median paired change versus release was -2.77% with 8/8 witness wins.

Across the eight witnesses the median paired change versus compact8 was -1.86% with 8/8 witness wins.
All 96 exports passed independent Gnark verification. These witnesses qualify the same-L3 twelve-worker frontend for the tested Ryzen 2×2 profile.

[Process samples](artifacts/processes.json.gz) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json) · [Original-input confirmation](../2026-11-19-frontend-smt437/README.md)
