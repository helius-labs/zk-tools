# Unequal contiguous G2 point ranges

Five arms use the ordinary release, the equal split in the candidate, and first partitions of 7/16, 9/16 or 10/16 of the points, rounded up. Each case has ten balanced process blocks and one separate profile per arm. The 157 native core tests passed and the ragged-input generator oracle passed at each unequal split. All 284 independent exports verified. Each dispatch probe ran three priming proofs and one measured proof. Every trace observed the requested two-way split.

Giving the first partition 9/16 of the points beat 7/16 in all 40 pairs across the four 1×2 cases. The comparison with equal halves was much smaller. On the 32-thread original it split 5/10 pairs, and on the 16-core original it won 8/10 against equal and 9/10 against release. The 10/16 split slowed each 32-thread reference compared with release. An independent 16-core comparison repeated the gain on the original and found near parity against release on two additional inputs. [Confirmation results](../2026-11-19-range16-confirm650/README.md)

| Input and workers | Release / equal / 7 / 9 / 10 sixteenths ms | 9 vs 7 / equal / release | 10 vs 7 / equal / release | Peaks in same order, MiB | 9/16 vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| 1x2-0-w32 | 7.092 / 7.061 / 7.222 / 7.059 / 7.148 | -2.26% (10/10) / +0.08% (5/10) / -0.51% (6/10) | -0.88% (6/10) / +0.45% (3/10) / +0.33% (2/10) | 61.2 / 61.2 / 62.8 / 60.5 / 60.2 | 5.24× / 3.40× |
| 1x2-0-w16 | 8.317 / 8.300 / 8.364 / 8.216 / 8.272 | -1.46% (10/10) / -1.20% (8/10) / -0.88% (9/10) | -0.76% (9/10) / -0.61% (6/10) / -0.50% (7/10) | 56.8 / 57.2 / 57.0 / 56.2 / 56.5 | 4.50× / 2.92× |
| 1x2-29-w32 | 6.982 / 7.025 / 7.135 / 7.040 / 7.158 | -1.68% (10/10) / -0.18% (5/10) / +0.17% (3/10) | -0.07% (5/10) / +1.70% (1/10) / +2.70% (2/10) | 58.5 / 59.0 / 59.2 / 59.0 / 58.0 | — |
| 1x2-32-w32 | 6.994 / 7.052 / 7.139 / 7.004 / 7.130 | -1.95% (10/10) / -0.89% (7/10) / +0.39% (3/10) | -0.25% (7/10) / +1.03% (1/10) / +2.17% (0/10) | 58.8 / 59.2 / 58.0 / 58.2 / 58.2 | — |
| 2x2-0-w32 | 9.642 / 9.696 / 9.801 / 9.702 / 9.879 | -0.22% (7/10) / +0.06% (5/10) / +0.74% (4/10) | +1.00% (3/10) / +2.12% (2/10) / +2.29% (0/10) | 111.2 / 113.0 / 113.2 / 114.8 / 110.8 | 6.08× / 3.09× |

[Protocol](protocol.md) · [Paired comparisons](artifacts/summary.json) · [Dispatch traces](artifacts/traces.json.gz) · [Samples and CPU profiles](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
