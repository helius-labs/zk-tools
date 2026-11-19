# G2 bucket handoff for two independent ranges

The flat schedules use one task for each window/range pair. The second finishing task performs its window's merge and weighted reduction. Every case has eight balanced process blocks per arm and one CPU profile per arm. All 157 native core tests passed and all 186 exported proofs verified.

| Input and workers | Release / nested / window-major / range-major ms | Range-major vs window-major / nested / release | Peaks in same order, MiB | Range-major vs M5 CPU / Metal |
|---|---:|---:|---:|---:|
| 1x2-0-w32 | 7.019 / 7.255 / 7.143 / 7.201 | +0.57% (2/8) / -1.04% (5/8) / +2.48% (1/8) | 60.2 / 62.5 / 60.0 / 61.5 | 5.14× / 3.33× |
| 1x2-0-w16 | 8.268 / 8.711 / 8.650 / 8.675 | -0.44% (5/8) / -0.51% (8/8) / +4.02% (0/8) | 56.8 / 56.8 / 57.0 / 56.2 | 4.26× / 2.77× |
| 1x2-29-w32 | 6.982 / 7.195 / 7.173 / 7.136 | -0.93% (6/8) / -0.76% (6/8) / +2.26% (0/8) | 58.8 / 58.8 / 58.5 / 58.0 | — |
| 1x2-32-w32 | 6.983 / 7.126 / 7.217 / 7.134 | -1.15% (7/8) / +0.06% (4/8) / +2.50% (0/8) | 57.8 / 58.5 / 58.5 / 58.2 | — |
| 2x2-0-w32 | 9.587 / 9.978 / 9.928 / 9.936 | +0.30% (3/8) / -0.51% (6/8) / +2.92% (0/8) | 112.0 / 112.2 / 109.5 / 109.2 | 5.94× / 3.02× |

[Protocol](protocol.md) · [All paired comparisons](artifacts/summary.json) · [Diagnostic events](artifacts/traces.json.gz) · [Samples and CPU profiles](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
