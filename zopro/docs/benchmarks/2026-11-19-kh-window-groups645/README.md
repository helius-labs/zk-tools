# Grouping K/H windows on the shared worker pool

Five arms use the ordinary release, the candidate with ordinary scheduling, and 20, 16 or 12 K/H window jobs. Each case has ten balanced process blocks and one separate profile per arm. The 156 native core tests passed and the generator oracle passed in all three grouping modes. All 284 independent exports verified. Each dispatch probe ran three priming proofs and one measured proof. The 2×2 path did not dispatch balanced K/H.

Both caps slowed every 1×2 case against the one-window-per-job control. On the original at 32 workers, 16 jobs took 7.818 ms and 12 jobs took 7.771 ms, against 6.946 ms for 20 jobs. The capped modes lost all 80 paired comparisons with 20 jobs across the four 1×2 cases. Keep the ordinary schedule. The 2×2 check recorded zero uses of the modified path. [Matched phase measurements](../2026-11-19-kh-phase646/protocol.md)

| Input and workers | Release / ordinary / 20 / 16 / 12 jobs ms | 16 vs 20 / ordinary / release | 12 vs 20 / ordinary / release | Peaks in same order, MiB | 16 jobs vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| 1x2-0-w32 | 7.017 / 7.004 / 6.946 / 7.818 / 7.771 | +11.92% (0/10) / +11.21% (0/10) / +11.08% (0/10) | +12.14% (0/10) / +11.02% (0/10) / +10.90% (0/10) | 61.5 / 61.8 / 61.5 / 61.5 / 60.5 | 4.73× / 3.07× |
| 1x2-0-w16 | 8.282 / 8.252 / 8.234 / 8.855 / 8.863 | +7.47% (0/10) / +8.23% (0/10) / +7.07% (1/10) | +7.30% (0/10) / +7.63% (0/10) / +6.73% (1/10) | 57.2 / 57.2 / 57.2 / 56.2 / 56.2 | 4.18× / 2.71× |
| 1x2-29-w32 | 7.012 / 6.945 / 6.967 / 7.547 / 7.635 | +8.46% (0/10) / +8.61% (0/10) / +7.04% (1/10) | +9.33% (0/10) / +9.51% (0/10) / +8.63% (1/10) | 59.5 / 58.5 / 58.5 / 58.0 / 56.8 | — |
| 1x2-32-w32 | 6.994 / 6.957 / 6.965 / 7.598 / 7.562 | +9.42% (0/10) / +8.86% (0/10) / +7.62% (0/10) | +8.81% (0/10) / +8.70% (0/10) / +7.53% (0/10) | 58.8 / 59.2 / 59.0 / 57.5 / 59.0 | — |
| 2x2-0-w32 | 9.605 / 9.539 / 9.594 / 9.627 / 9.587 | +0.29% (3/10) / +0.66% (1/10) / +0.23% (5/10) | -0.22% (6/10) / +0.36% (4/10) / -0.57% (7/10) | 110.2 / 113.8 / 107.8 / 107.5 / 110.0 | 6.13× / 3.12× |

[Protocol](protocol.md) · [Paired comparisons](artifacts/summary.json) · [Dispatch traces](artifacts/traces.json.gz) · [Samples and CPU profiles](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
