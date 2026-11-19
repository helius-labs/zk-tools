# Merge and weight G2 range buckets in one tile task

The staged and fused modes use the same 512-bucket arithmetic, per-tile forest allocation and global combination. The disabled arm retains the regular paired-weighted implementation in the candidate binary. Each case has eight balanced process blocks per arm and one CPU profile per arm. All 157 native core tests passed and all 186 exported proofs verified.

| Input and workers | Release / disabled / staged / fused ms | Fused vs staged / disabled / release | Peaks in same order, MiB | Fused vs M5 CPU / Metal |
|---|---:|---:|---:|---:|
| 1x2-0-w32 | 7.146 / 6.986 / 7.007 / 7.039 | +0.34% (3/8) / +1.15% (1/8) / -0.56% (4/8) | 62.8 / 65.5 / 72.0 / 66.0 | 5.26× / 3.41× |
| 1x2-0-w16 | 8.235 / 8.230 / 8.210 / 8.208 | +0.02% (4/8) / -0.22% (5/8) / -0.57% (6/8) | 56.5 / 57.0 / 57.5 / 58.0 | 4.51× / 2.92× |
| 1x2-29-w32 | 6.961 / 6.943 / 6.918 / 6.898 | -0.39% (6/8) / -1.12% (7/8) / -0.62% (4/8) | 58.2 / 59.2 / 59.2 / 59.8 | — |
| 1x2-32-w32 | 6.987 / 6.977 / 6.972 / 7.000 | +0.01% (4/8) / -0.33% (5/8) / +0.28% (4/8) | 58.5 / 59.0 / 58.8 / 59.0 | — |
| 2x2-0-w32 | 9.641 / 9.582 / 9.610 / 9.571 | -1.13% (6/8) / -0.59% (5/8) / -1.50% (7/8) | 109.2 / 109.5 / 108.2 / 109.0 | 6.16× / 3.13× |

[Protocol](protocol.md) · [All paired comparisons](artifacts/summary.json) · [Diagnostic events](artifacts/traces.json.gz) · [Samples and CPU profiles](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
