# Count-first solver on C7a

The corrected solver preserved the previous prototype’s warm latency in all six measured contexts. Counting edges before reserving adjacency storage cut the merge54 construction reservation from 143.1 MiB to 7.18 MiB. Whole-process peak memory still grows across repeated proofs. On merge54 at 32 cores the corrected build peaked at 2,316 MiB, 275 MiB above the accepted build.

At 96 cores on 1×2 the corrected build took **12.046 ms**. Enabling readiness reduced wall time in all three paired blocks, with a median reduction of **1.80%**. CPU time rose **3.80%** relative to the same binary with readiness disabled, failing the predeclared primary CPU check. The accepted reference took 12.439 ms. The previous [Aeglos measurement](../2026-11-18-c7a-scaling245/README.md) was 7.861 ms on the same instance type. That comparison spans different runs. The prototype stays opt-in for experiments.

## Warm verified single-proof latency

One c7a.24xlarge with 96 physical cores, SMT disabled and fixed worker affinity. Each cell reports the median of three process means in milliseconds. Each paired percentage is the median of the three within-block ratios. REF is the accepted build, OFF and ON share the corrected executable, and OLD enables readiness in the earlier executable.

|Shape|Cores|REF ms|OFF ms|OLD ms|ON ms|ON/OFF wall change|ON/OFF CPU change|
|---|---:|---:|---:|---:|---:|---:|---:|
|1x2 |32|10.838|10.900|10.579|10.521|-2.81%|0.49%|
|1x2 |96|12.439|12.160|11.949|12.046|-1.80%|3.80%|
|2x2 |32|14.590|14.517|13.650|13.592|-6.52%|-2.49%|
|2x2 |96|16.354|16.287|15.215|15.123|-6.73%|-2.31%|
|merge-54x1 |32|142.354|141.301|137.471|137.593|-1.03%|3.11%|
|merge-54x1 |96|123.545|121.891|115.096|115.413|-5.31%|-5.48%|

The largest repeatable directional gain was on 2×2. At 96 cores wall time fell 6.73% and CPU time fell 2.31% against OFF. At 96 cores on merge54 the reductions were 5.31% and 5.48%. All six corrected-versus-previous constructor comparisons passed the prespecified 3% latency screen. All six OFF/REF latency calibrations passed. Three blocks give directional evidence. Production adoption would require broader qualification.

## Memory

The first four columns after core count give the maximum lifetime process RSS across the three plain timing blocks. The final two are constructor capacity charges reported during preflight. All values are MiB. Old and new produced the same graph dimensions in each context.

|Shape|Cores|REF peak|OFF peak|OLD peak|ON peak|OLD construction|ON construction|
|---|---:|---:|---:|---:|---:|---:|---:|
|1x2|32|121.7|125.0|121.1|124.0|3.780|0.193|
|1x2|96|114.3|119.8|118.2|115.1|3.780|0.193|
|2x2|32|266.4|270.4|268.8|263.5|6.568|0.332|
|2x2|96|268.7|270.2|259.8|263.6|6.568|0.332|
|merge-54x1|32|2040.8|2095.1|2238.9|2316.2|143.118|7.179|
|merge-54x1|96|2058.4|2068.0|2228.6|2212.9|143.118|7.179|

The constructor screen passed its direct OLD RSS guard in all six contexts. Against REF, merge54 at 32 cores still exceeds the earlier production comparison guard of 10% plus 8 MiB. On this shape the 32-core one-timed-proof preflights peaked at 1,526 MiB with readiness on and 1,521 MiB with it off. After 17 timed requests the maximum difference was 221 MiB. These are independent processes with three primes each. They motivate measuring memory between requests and cannot localize the allocations. No major faults occurred in any of the plain merge54 timing blocks. Minimum device RAM remains unmeasured.

## Verification and profiles

The fixed run covered 24 functional preflights, 72 plain timing processes and 18 observer/profiler processes. All 10,140 SDK calls included proof verification. All 114 exported proofs passed Gnark verification on AWS and again after hash-checked retrieval. The distinct verified-export inventory reached 64,421. The unit used 30 GiB of infrastructure headroom with swap disabled. Instance termination, zero tagged volumes, temporary bucket removal and local signing-parameter removal were checked.

All six dual-event recordings passed event/period, sample-count, symbol, loss and process/thread/CPU mapping checks. All 12 counter readings were fully scheduled. Three of four observer comparisons exceeded the declared factor of 1.05. OFF stat/plain was 1.043, OFF dual/plain 1.069, ON stat/plain 1.057 and ON dual/plain 1.065. These profiles characterize perturbed executions. The plain timing results were collected first.

Against the supplied M5 CPU and Metal references, the corrected 32-core 1×2 result is 3.52× and 2.28×, respectively. At 96 cores it is 3.07× and 1.99×. The corresponding 2×2 results are 4.34× and 2.21× at 32 cores, and 3.90× and 1.98× at 96. Merge54 has no supplied M5 reference. The [all-eight comparison](../2026-11-18-paired-weighted-all234/README.md) remains the reference for the broader goal.

[Fixed protocol](artifacts/protocol.md) · [Block comparisons and memory](artifacts/comparisons.json) · [Profile aggregates](artifacts/analysis.json.gz) · [Verified inventory](artifacts/review.json)
