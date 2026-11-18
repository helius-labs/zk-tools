# One reduction tree per final G2 bucket

Joining the two point partitions before reduction made the complete G2 query slower on the local Ryzen 9950X. At 32 threads it added 10.3% for 1×2 and 13.7% for 2×2 against the previous tree, using paired block ratios. It also raised peak RSS. This implementation missed the predeclared local criterion for spending a C7a run on it.

|Query|Threads|Current streaming ms|Previous tree ms|Joined tree ms|Joined / previous|Peak RSS, current / previous / joined MiB|
|---|---:|---:|---:|---:|---:|---:|
|1×2|16|3.270|3.470|3.484|1.009|28 / 37 / 40.25|
|1×2|32|2.928|3.001|3.314|1.103|30.75 / 48.50 / 57|
|2×2|16|3.816|4.123|4.220|1.029|40.25 / 50 / 54|
|2×2|32|3.378|3.755|4.270|1.137|42.50 / 58.75 / 73.50|

The [current AWS trace](../2026-11-18-c7a-phase251/README.md) measured a 2.19 ms mean positive interval between A and G2 returns at 96 cores. The prototype gives each ordinary final G2 bucket a single tree over the two signed index spans. Two input halves still build their routing lists in parallel. A leaf borrows its original point. Combining the ranges eliminates one layer of partition merges and repeated high-component transforms. Each final tree now has access to the full query. The scratch forecast covers that larger worst case. Routing, traversal order and useful parallelism change together. The measurement does not isolate a particular cause of the regression.

The three executables have identical bytes on separate file backings. All 36 processes ran three primes, 17 timed queries and a postcheck, totalling 756 complete query outputs. The means above are medians of three process means, and peak RSS is the largest process high-water mark in the cell. The tests use two captured queries, independent Arkworks output digests, synthetic identities and signed/exceptional cases. Both focused native tests passed and the generic core compiled. A reader initially expected the SCREEN marker at the beginning of the test-harness line. After fixing the parser it reconstructed all records from the original stdout and checked receipt hashes. No query was replayed. This experiment made no full SDK calls and did not measure the new algorithm at 96 physical cores.

[Mechanism and progression rule](artifacts/protocol.md) · [All timing records](artifacts/assessment.json.gz) · [Private source changes relative to the paired-fill harness](artifacts/changes.patch.gz)
