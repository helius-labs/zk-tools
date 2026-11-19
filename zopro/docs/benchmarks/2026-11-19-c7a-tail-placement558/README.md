# When G2 prefix arithmetic runs relative to H on C7a

The archived C7a profiles contain 132 timed requests from twelve processes and two circuit shapes. All twelve exported proofs verified again. Every request had an instruction-pointer sample in the named G2 prefix arithmetic function during the encompassing curve query. On every request at most 8% or at least 96% of these samples fell after H returned. Splitting requests at 50% therefore yields the same groups as any threshold from 10% to 90%. These are samples at 500 Hz per active thread. The first and last samples need not coincide with the start and end of B2.

| Shape | Cores | Majority of G2 prefix samples relative to H return | Requests / 33 | H ms | Time from H return to join ms | Whole curve query ms | Whole SDK ms |
|---|---:|---|---:|---:|---:|---:|---:|
| 16×2 | 64 | Before | 24 | 18.919 | 0.523 | 20.651 | 35.914 |
| 16×2 | 64 | After | 9 | 13.359 | 6.448 | 21.249 | 36.133 |
| 16×2 | 96 | Before | 19 | 18.206 | 0.217 | 20.367 | 37.790 |
| 16×2 | 96 | After | 14 | 14.144 | 5.462 | 21.159 | 38.300 |
| Merge-24×1 | 64 | Before | 22 | 25.021 | 0.163 | 26.136 | 50.082 |
| Merge-24×1 | 64 | After | 11 | 18.962 | 8.331 | 28.467 | 51.692 |
| Merge-24×1 | 96 | Before | 16 | 25.026 | 0.163 | 26.294 | 51.361 |
| Merge-24×1 | 96 | After | 17 | 19.593 | 7.596 | 28.798 | 53.714 |

None of the 7,987 G2 prefix samples preceded H entry. Of the 81 requests in the before group, all 81 had samples during the H call. Those requests had 4,910 samples during H and eight afterward. On the other 51 requests just two samples fell during H and 3,067 fell afterward.

Each time above is a median within the indicated group. In both shapes the H call was shorter when G2 prefix samples fell later. The time between H return and the join therefore exaggerates the difference in complete proof time.

The next table uses arithmetic means. The change in frequency uses the increase in the after-H group's frequency times the average of the two widths' within-width group differences. The remaining term averages the within-group changes at the two widths' mixture weights. The terms sum to the observed change.

| Shape | Interval | 96 minus 64 ms | Group-frequency contribution ms | Within-group contribution ms |
|---|---|---:|---:|---:|
| 16×2 | After H | +0.506 | +0.757 | -0.251 |
| 16×2 | Whole curve query | -0.338 | +0.103 | -0.441 |
| 16×2 | Whole proof | +1.412 | +0.077 | +1.335 |
| Merge-24×1 | After H | +1.421 | +1.378 | +0.043 |
| Merge-24×1 | Whole curve query | +1.338 | +0.389 | +0.949 |
| Merge-24×1 | Whole proof | +2.077 | +0.387 | +1.690 |

On merge the change in group frequency accounted arithmetically for 1.378 ms of the 1.421 ms longer post-H interval, but only 0.387 ms of the 2.077 ms longer full request. On 16×2 the whole curve query actually finished 0.338 ms sooner on average at 96 cores. This decomposition describes the observed requests. A change in branch scheduling could also change both groups' durations.

We also ranked pinned workers by G2 prefix samples on the other two processes of the same width and shape. At 96 cores on merge, the busiest quarter in that training set accounted for 36.4%, 38.2% and 38.3% of after-H samples in the last six held-out requests of each process, compared with 22.6%, 25.7% and 23.9% of before-H samples when ranked on that interval. Affinity alone does not establish why those workers occurred more often.

[Archived trace and placement hashes](../2026-11-18-c7a-worker-attribution381/artifacts/inputs.json) · [Per-request samples and times](artifacts/requests.json) · [Worker counts](artifacts/workers.json.gz) · [Held-out worker placement](artifacts/comparison.json) · [Reader and verification](artifacts/analyze.py) · [Verified exports](artifacts/verification.json)
