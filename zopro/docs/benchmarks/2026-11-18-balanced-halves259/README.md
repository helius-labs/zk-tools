# Sampled G2 partition boundary

A proof-local estimate of the work in each point range produced mixed complete-query results on the Ryzen 9950X. On 1×2 the paired median was **1.44% faster at 16 threads** and **0.69% slower at 32**. Keeping two workers' ranges closer in digit count did not produce a repeatable local query gain across these contexts. The full-proof and 96-core effects remain unmeasured.

| G2 query | Threads | Equal halves ms | Sampled split ms | Paired ON / OFF | Winning blocks |
|---|---:|---:|---:|---:|---:|
| 1×2 | 16 | 3.255 | 3.232 | 0.9856 | 2/3 |
| 1×2 | 32 | 2.871 | 2.903 | 1.0069 | 1/3 |
| 2×2 | 16 | 3.740 | 3.739 | 1.0023 | 1/3 |
| 2×2 | 32 | 3.393 | 3.357 | 1.0005 | 1/3 |

After scalar decomposition, the prototype samples the midpoint of 256 equal-sized strata, weights each sample by the two component lengths and point identity, and chooses a boundary around half the estimated work. The cut is shared by all windows. The estimator uses 2 KiB of temporary stack on x86-64. Bucket counts and point storage are unchanged. Timings include the estimator and the original complete G2 query, including grouping on the 2×2 fixture. The earlier C7a trace measured an imbalance on one input. This experiment does not establish that balancing arrivals alone balances cache access, exceptional cases or elapsed proof time. [Trace and input evidence](../2026-11-18-g2-ownership258/README.md)

Both focused native tests passed, including complete signed, identity and odd-length query checks. The generic core compiled. Three rotated process blocks for each arm and context supplied 24 processes. Each checked three primes, 17 timed queries and a postcheck against the independent full-query output digest. All 504 outputs matched. Whole-process RSS was 31 MiB in both primary32 arms and 43.25 MiB in both 2×2/32 arms. No SDK proof calls were made. [Block results](artifacts/assessment.json.gz) · [Prototype and tests](artifacts/changes.patch.gz)
