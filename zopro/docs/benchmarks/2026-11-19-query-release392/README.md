# Query readiness at observed solver barriers on Ryzen

The diagnostic pairs static query wire readiness with actual elapsed times for the outer solver bands. The census and timed processes are separate. The release uses the clean integrated solver. The clock and disabled controls share one executable. Median SDK latencies are in milliseconds.

| Shape | Blocks | Release | Disabled | Clock | Paired clock vs disabled | Paired disabled vs release | Within calibration |
|---|---:|---:|---:|---:|---:|---:|---|
| 1x2 | 5 | 7.335 | 7.326 | 7.370 | +0.16% | +0.12% | yes |
| 2x2 | 5 | 10.166 | 10.214 | 10.279 | +0.93% | -0.81% | yes |
| 16x2 | 3 | 44.951 | 44.754 | 44.796 | +0.35% | -0.44% | yes |

After the first 1×2 barrier, 16,676 of 30,089 B-query entries were ready. They occupied three contiguous ranges. Only 0.347 ms of solver execution remained, against a 7.335 ms release proof and a 3.7 ms M5 CPU target. Even hiding that whole solver tail would cover under one tenth of the gap. On 16×2, 31.2% of B entries were ready with 1.760 ms of solver time left. These upper bounds on pre-solver-completion overlap help size the benefit of any implementation. No curve work started early in this experiment. [Input partition analysis](partitions.md)

## Scalar availability and solver time left

Barrier zero represents initialized inputs. An entry becomes ready after its last writer. A completed class has all occurrences of the same nonzero, nonunit scalar available. A split class has occurrences on both sides of the barrier. Counts depend on the pinned request. Time left is the median of within-process means for the remaining outer bands. For a shape outside calibration, the clocks are diagnostic only.

| Shape | Query | Barrier | Ready entries | Ordered prefix | Entries in complete 1,024-entry chunks | Completed classes | Split classes | Total classes | Solver ms left |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | A | 0 | 0.7% | 141 | 0 | 55 | 6 | 17453 | 0.750 |
| 1x2 | A | 1 | 57.3% | 3340 | 9216 | 8625 | 2 | 17453 | 0.347 |
| 1x2 | A | 2 | 73.7% | 4992 | 11790 | 12022 | 2 | 17453 | 0.225 |
| 1x2 | A | 3 | 89.5% | 6645 | 15886 | 15268 | 2 | 17453 | 0.105 |
| 1x2 | A | 4 | 100.0% | 21006 | 21006 | 17453 | 0 | 17453 | 0.000 |
| 1x2 | B | 0 | 0.5% | 142 | 0 | 57 | 5 | 26141 | 0.750 |
| 1x2 | B | 1 | 55.4% | 4911 | 13312 | 12903 | 3 | 26141 | 0.347 |
| 1x2 | B | 2 | 72.6% | 7380 | 19849 | 18001 | 3 | 26141 | 0.225 |
| 1x2 | B | 3 | 89.0% | 9849 | 23945 | 22869 | 3 | 26141 | 0.105 |
| 1x2 | B | 4 | 100.0% | 30089 | 30089 | 26141 | 0 | 26141 | 0.000 |
| 2x2 | A | 0 | 0.6% | 232 | 0 | 63 | 8 | 19267 | 1.055 |
| 2x2 | A | 1 | 51.4% | 3601 | 13312 | 10439 | 2 | 19267 | 0.439 |
| 2x2 | A | 2 | 69.9% | 5253 | 22301 | 13836 | 2 | 19267 | 0.280 |
| 2x2 | A | 3 | 87.9% | 6906 | 26397 | 17082 | 2 | 19267 | 0.121 |
| 2x2 | A | 4 | 100.0% | 36637 | 36637 | 19267 | 0 | 19267 | 0.000 |
| 2x2 | B | 0 | 0.4% | 231 | 0 | 67 | 5 | 28851 | 1.055 |
| 2x2 | B | 1 | 49.1% | 5265 | 22528 | 15613 | 3 | 28851 | 0.439 |
| 2x2 | B | 2 | 68.5% | 7734 | 31794 | 20711 | 3 | 28851 | 0.280 |
| 2x2 | B | 3 | 87.3% | 10203 | 42034 | 25579 | 3 | 28851 | 0.121 |
| 2x2 | B | 4 | 100.0% | 52274 | 52274 | 28851 | 0 | 28851 | 0.000 |
| 16x2 | A | 0 | 0.6% | 1492 | 1024 | 99 | 38 | 34325 | 3.424 |
| 16x2 | A | 1 | 34.7% | 2581 | 49152 | 20233 | 163 | 34325 | 1.760 |
| 16x2 | A | 2 | 50.2% | 3684 | 90112 | 25060 | 163 | 34325 | 1.330 |
| 16x2 | A | 3 | 65.0% | 8879 | 129668 | 28894 | 2 | 34325 | 0.897 |
| 16x2 | A | 4 | 79.1% | 9992 | 165508 | 31078 | 2 | 34325 | 0.539 |
| 16x2 | A | 5 | 93.1% | 11104 | 217732 | 33241 | 0 | 34325 | 0.188 |
| 16x2 | A | 6 | 100.0% | 252548 | 252548 | 34325 | 0 | 34325 | 0.000 |
| 16x2 | B | 0 | 0.4% | 1477 | 1024 | 118 | 20 | 51596 | 3.424 |
| 16x2 | B | 1 | 31.2% | 3112 | 74752 | 30458 | 244 | 51596 | 1.760 |
| 16x2 | B | 2 | 47.6% | 4766 | 136192 | 37705 | 244 | 51596 | 1.330 |
| 16x2 | B | 3 | 63.2% | 12914 | 194503 | 43456 | 3 | 51596 | 0.897 |
| 16x2 | B | 4 | 78.0% | 14576 | 249799 | 46731 | 3 | 51596 | 0.539 |
| 16x2 | B | 5 | 92.7% | 16237 | 318407 | 49973 | 0 | 51596 | 0.188 |
| 16x2 | B | 6 | 100.0% | 358343 | 358343 | 51596 | 0 | 51596 | 0.000 |

A split class could require extra point work if grouped separately on either side of a barrier. The diagnostic records its presence. It does not assign it an arithmetic cost. Available time before the solver finishes is an upper bound for a proposed overlap at that barrier. Query grouping, bucket work and worker contention would consume part of it.

[Protocol](artifacts/protocol.md) · [Counts](artifacts/readiness.json) · [Timed samples](artifacts/samples.json.gz) · [Verification](artifacts/verification.json)
