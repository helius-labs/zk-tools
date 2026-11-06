# Queue draining with indexed-top packing

**Not adopted.** The fixed combination reduces primary32 complete G2 query latency by 1.69% versus the same executable's original path, but its additional gain over draining alone is inconclusive. The declared necessary screen fails primary gain versus DRAIN and one unchanged-A stability check. No full-SDK follow-up, finalizer composition or accepted performance change follows.

The positive observations remain: primary32 BOTH/ORIGINAL is 0.983076 [0.973224,0.992680], with 14/15 wins. The equal-log two-population32 aggregate improves 1.64% versus ORIGINAL and 0.54% versus DRAIN, passing both aggregate checks. These are query results, not proof timings, and do not override the failed complete screen.

## Fixed implementation

This combines the existing [contained indexed-top packing](../2026-11-06-g2-contained-index176/README.md) with the guarded G2 queue-drain method previously used in the [finalization/drain composition](../2026-11-05-finalize-drain-bundle176/README.md). Their earlier failed qualifications remain unchanged. The new implementation uses one executable with independent process-cached packing/draining selectors and four configurations: ORIGINAL, DRAIN, PACK and BOTH. This is composition evidence, not a new arithmetic formula or an unchanged component retry.

Packing replaces only the signed indexed leading-window reducer and keeps the original typed whole-workspace containment and optional-budget fallback. The ten ordinary windows delegate to the original borrowed G2 implementation. Draining uses its existing affine batches and SIMD-width eligibility guard before scalar fallback. The finalizer and G1 drain changes are excluded: they do not execute in this G2 query comparison. All A computations remain original.

Window fields/capacities, field arithmetic, Phi2, scalar grouping, point inputs and window geometry are unchanged. No proving-key table, second key, retained execution plan, digit matrix or new runtime pool is added. Packing metadata and leaf scratch remain covered by the inherited checked layout; explicit optional-budget presence selects the original reducer before packing allocation. The queue method adds no heap buffer. This does not establish a full stack bound, minimum RAM, concurrent/device safety or arbitrary pressure qualification.

## Complete-query results

Ryzen 9950X, 32 logical CPUs. Cases use one worker pinned to CPU2, 16 workers on CPUs0–15, or 32 workers on CPUs0–31, with 512 KiB worker stacks. Fifteen fixed rotated/reversed process blocks cover all seven contexts and four configurations. B processes use 3 primes, 17 timed queries and 1 postcheck; A uses 3/47/1. Fixture loading and pool setup are excluded; scalar reset, original grouping/admission, Phi2, allocation, all MSM work and normalization are included. Displayed times are medians of 15 process medians. Paired ratios need not equal quotients of displayed times.

|Context|ORIGINAL ms|DRAIN ms|PACK ms|BOTH ms|BOTH/ORIGINAL [bounds]|BOTH/DRAIN [bounds]|
|---|---:|---:|---:|---:|---|---|
|1x2-B-w1|37.220183|37.156221|32.840480|32.833136|0.881358 [0.879250,0.882432]|0.883153 [0.882536,0.883898]|
|1x2-B-w16|3.655957|3.664994|3.659122|3.652560|0.988827 [0.980932,1.010244]|0.993797 [0.984069,1.003715]|
|1x2-B-w32|3.315285|3.264949|3.291329|3.256503|0.983076 [0.973224,0.992680]|0.995709 [0.991264,1.003984]|
|2x2-B-w1|39.888155|39.764402|35.487161|35.376642|0.887436 [0.884584,0.888143]|0.889936 [0.889248,0.890637]|
|2x2-B-w16|4.288740|4.258553|4.224890|4.233717|0.987883 [0.974806,1.009120]|0.996061 [0.983897,1.012022]|
|2x2-B-w32|3.884247|3.875562|3.864741|3.834804|0.980181 [0.975456,1.000158]|0.993496 [0.987074,0.998689]|
|1x2-A-w32, unchanged|1.170769|1.177942|1.178092|1.189154|1.012466 [0.990965,1.035712]|1.006478 [0.992592,1.019219]|

|Two-population32 aggregate|Median|Lower|Upper|Wins/15|Gain check|
|---|---:|---:|---:|---:|---|
|BOTH/ORIGINAL|0.983626|0.974695|0.987179|13|pass|
|BOTH/DRAIN|0.994602|0.989248|0.999694|12|pass|
|BOTH/PACK, descriptive|0.995524|0.984780|1.000066|11|not a selection gate|

The [prospective protocol](artifacts/protocol.md) requires BOTH non-regression versus ORIGINAL and DRAIN in all six B contexts, primary32 and aggregate gain versus both controls, and all six pairwise unchanged-A intervals inside [1/1.03,1.03]. All B non-regression checks pass. Primary32 versus DRAIN has 9/15 wins and an interval spanning parity, so its gain check fails. A's BOTH/ORIGINAL upper bound exceeds 1.03; the other five A intervals pass. That is a failed stability check, not an established A algorithm regression. All six factorial comparisons are retained in [analysis](artifacts/analysis.json).

Bounds use ranks 4–12 of 15, nominal 96.484375% marginal under independent/exchangeable blocks, without simultaneous coverage. There is no minimum percentage gain requirement. No observation is excluded, block extended, threshold changed, successful stage repeated or favorable subset promoted. The fixed composition is closed without SDK follow-up. The accepted full-proof executable is not a calibration arm in this component screen.

## CPU service, resources and profiles

|Context|CPU ms/query ORIGINAL / DRAIN / PACK / BOTH|Process RSS MiB ORIGINAL / DRAIN / PACK / BOTH|
|---|---|---|
|1x2-B-w16|45.048 /44.796 /39.778 /39.718|28.00 /28.00 /27.75 /27.75|
|1x2-B-w32|71.113 /69.741 /64.261 /63.543|34.50 /34.25 /32.25 /31.75|
|2x2-B-w16|48.258 /48.029 /43.197 /43.240|42.75 /42.50 /42.75 /43.25|
|2x2-B-w32|76.215 /75.248 /70.099 /69.683|60.25 /61.50 /53.75 /60.00|

CPU is summed user+system service per request, summarized across process means. RSS is maximum lifetime rusage among latency processes, including loading/primes. These counters can differ from page-table observations and are diagnostics, not a rigorous peak or minimum RAM. Solo primary/grouped2x2 BOTH/ORIGINAL improvements are 11.86%/11.26%, each with 15/15 wins; those do not establish a parallel or full-proof gain of that size.

All 420 latency processes finish before four primary32 PMU panels and four separate profiles. PMU counts exclude primes via FIFO and have 100% scheduled time. Single panels, not uncertainty estimates:

|Configuration|Instructions M/query|Cycles M/query|Cache misses M/query|Profile samples|Unknown|
|---|---:|---:|---:|---:|---:|
|ORIGINAL|567.855|319.448|1.047421|20,502|190|
|DRAIN|566.771|319.286|1.116982|20,393|188|
|PACK|518.596|289.745|1.112129|18,493|155|
|BOTH|517.008|288.314|1.106085|18,419|152|

BOTH uses 8.96% fewer instructions and 9.75% fewer cycles than ORIGINAL in these panels. The leading work reduction persists, while primary32 wall time improves less. Profiles retain similar borrowed affine-apply sample counts (8,313→8,151); reduce-layer samples fall 2,887→2,000 and inverse samples 1,595→852. This supports reduced CPU work in the packing path. Flat samples include inlined work; they do not identify the exclusive proof critical path or establish scheduler/cache/DRAM causality. Cache misses rise in the single panels. All four profiles qualify on sample/unknown/period/PID/TID/CPU/mmap/FORK/zero-loss checks; 77,807 samples total. No owned compilation, hashing, analysis or second workload overlaps recording.

## Correctness and provenance

Both draining modes pass 130 unfiltered core tests on their first executions. The suites include the reused three containment tests and two queue tests, covering complete generator/exponent oracles, signs, identities, opposite/doubling cases, queue/pending/SIMD boundaries, explicit-budget fallback, every supported half-length, and 1/4/16/32 workers with 512 KiB stacks. Unchanged field/prover/image qualification is inherited, not rerun.

All 28 fresh preflights pass the pinned independent Arkworks whole-MSM hashes before latency. All 15,688 complete benchmark queries, including primes/postchecks, match the oracle and keep point inputs unchanged. PACK/BOTH B report four actual packed-top entries; other routes remain original. These calls are not proofs. No SDK/Gnark export or AWS resource is created; the canonical distinct proof ledger remains 30,846.

The private 180-file snapshot has 170 of 176 originals byte-exact. Reversal recovers the prior 178-file contained snapshot and all 176 accepted originals. A separate reviewer reconstructs originals forward, verifies actual reused bodies, the driver inverse and optimized native/fat-LTO/one-CGU compiler command, 434 pin occurrences, all 481 workflow command argv/environment projections/exits/stream hashes, every timing/CPU/RSS gate, PMU counts and 77,807 profile samples with four additional raw loss checks. Review passes on its first execution. It uses raw caller observations and independent median/aggregate calculations.

Two pre-build preparation failures remain recorded: the first setup audit attempted UTF8 decoding of a binary fixture; the finishing helper resumes from the unchanged source snapshot and compares binary files as bytes. A first xtask invocation used `perf` instead of `workflow` and failed configuration parsing before starting a child. Neither failure reruns a successful native build, test or capture. All four native and 477 capture workflow commands then succeed on their first executions.

## Accepted comparisons remain unchanged

Inherited local 32-worker full-proof values and supplied M5 references, not timings of this query experiment. Accepted source176/SDK `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661` remains unchanged. Different hardware does not establish equal hardware efficiency.

|Shape|Accepted CPU ms|M5 CPU ms|CPU/local|M5 Metal ms|Metal/local|CPU10x limit ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.928660|37|4.667x|24|3.027x|3.7|
|2x2|11.151932|59|5.291x|30|2.690x|5.9|
|4x2|16.086386|109|6.776x|48|2.984x|10.9|
|8x2|24.179453|189|7.817x|74|3.060x|18.9|
|12x2|33.884455|284|8.381x|101|2.981x|28.4|
|16x2|40.077381|335|8.359x|124|3.094x|33.5|
|24x2|65.850777|528|8.018x|176|2.673x|52.8|
|merge24x1|65.060937|507|7.793x|175|2.690x|50.7|

Merge54 remains 161.883899 ms without a supplied M5 reference. The all-eight CPU10x goal stays active/unmet, 0/8. No new Aeglos/AWS, concurrent-prover, huge-circuit, device, pressure, swap or minimum-RAM result follows. The inherited accepted primary RSS 132.75 MiB is not remeasured here.

[Review](artifacts/review.json), [analysis](artifacts/analysis.json), [protocol](artifacts/protocol.md), lossless receipts/helpers, source snapshot, four raw profiles and [checksums](SHA256SUMS) retain the experiment. Private runtime point/scalar/key/witness arrays and executable/library blobs are excluded.
