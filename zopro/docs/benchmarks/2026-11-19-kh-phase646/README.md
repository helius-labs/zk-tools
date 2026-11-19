# G2 and K/H completion under shared-pool window grouping

Groups of two keep the original K/H arithmetic and reduce the number of jobs competing for workers. The 20-job mode uses the grouped iterator with one window per job. Each mode ran in four balanced blocks with 17 timed calls in each plain or traced process. Independent verification passed for all 32 exports. Every timed trace contained 22 G2 range fills, 40 balanced K/H feed calls, and 21 ordinary G1 range closures. Interval and endpoint columns are medians of process medians. Plain and observed latencies are medians of process means.

Limiting K/H to 12 jobs raised the number of G2 fills starting before the first returned from 11 to 19. Against 20 jobs, the traced median G2 return shifted from 6.678 to 6.648 ms after SDK entry. The final K/H feed shifted from 4.319 to 7.649 ms and finished after G2 in 58/68 proofs. The plain proof slowed from 7.075 to 7.882 ms. Shifting workers changed which query finished last and did little for the G2 endpoint. The traced 12-job arm itself carried an 11.79% paired observer penalty.

| Mode | Plain ms | Observed ms | Paired observer change | G2 entry to return ms | Entry to first fill ms | First to last fill start ms | Final fill end to G2 return ms |
|---|---:|---:|---:|---:|---:|---:|---:|
| off | 7.034 | 7.407 | +4.47% | 4.362 | 0.081 | 2.172 | 0.246 |
| j20 | 7.075 | 7.400 | +5.50% | 4.419 | 0.082 | 2.201 | 0.252 |
| j16 | 7.720 | 8.115 | +4.30% | 4.299 | 0.069 | 2.130 | 0.250 |
| j12 | 7.882 | 8.726 | +11.79% | 4.237 | 0.068 | 1.971 | 0.255 |

| Mode | G2 return from SDK entry ms | Final K/H feed return from SDK entry ms | K/H feed returned after G2 |
|---|---:|---:|---:|
| off | 6.716 | 4.229 | 0/68 |
| j20 | 6.678 | 4.319 | 1/68 |
| j16 | 6.617 | 7.276 | 56/68 |
| j12 | 6.648 | 7.649 | 58/68 |

## Mean scheduled threads inside the named functions

Each entry sums the function intervals across threads, subtracts per-thread off-CPU time and divides by elapsed interval length. The launch interval goes from the first to the final fill start. The following interval ends at the last fill return. The K/H endpoint excludes subsequent weighting, so it is a lower bound on K/H completion.

| Mode | Interval | G2 fill closures | Balanced K/H feed | Ordinary borrowed G1 range closures |
|---|---|---:|---:|---:|
| off | launch | 10.96 | 15.40 | 1.09 |
| off | fills after launch | 9.55 | 0.00 | 7.12 |
| j20 | launch | 10.97 | 15.30 | 1.32 |
| j20 | fills after launch | 9.42 | 0.00 | 7.13 |
| j16 | launch | 14.56 | 12.96 | 1.06 |
| j16 | fills after launch | 6.40 | 2.66 | 7.21 |
| j12 | launch | 18.55 | 10.26 | 0.74 |
| j12 | fills after launch | 5.01 | 5.24 | 7.46 |

[Protocol](protocol.md) · [Uninstrumented experiment](../2026-11-19-kh-window-groups645/README.md) · [Process samples](artifacts/runs.json.gz) · [Per-proof phase times](artifacts/query-intervals.json.gz) · [Process endpoints](artifacts/endpoints.json) · [Verification](artifacts/verification.json)

## How the 22 fills started

Times below are relative to the first range start. Each cell gives the median of the four process medians. A range can finish before all ranges get a worker.

| Input or mode | Start of fill 6 ms | Fill 11 ms | Fill 17 ms | Fill 22 ms | First fill return ms | Fills started before first return |
|---|---:|---:|---:|---:|---:|---:|
| j12 | 0.002 | 0.004 | 0.006 | 1.971 | 1.835 | 19.0 |
| j16 | 0.002 | 0.004 | 1.840 | 2.130 | 1.806 | 15.0 |
| j20 | 0.002 | 0.003 | 1.956 | 2.201 | 1.794 | 11.0 |
| off | 0.002 | 0.003 | 1.977 | 2.172 | 1.810 | 11.0 |

[Sorted start times for each proof](artifacts/launch-ranks.json.gz)
