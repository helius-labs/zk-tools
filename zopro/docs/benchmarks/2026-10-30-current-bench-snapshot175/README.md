# CPU prover benchmark snapshot

30 October 2026. Evidence through repository commit `7c3b320dcfe2d89aa7ba1d908917c10c9c4356bc`. This document consolidates completed measurements; writing it adds no benchmark, test, proof or AWS execution.

Accepted local **1×2 remains 8.022165 ms / 134 MiB peak process RSS**. The latest AWS c7a.24xlarge comparison remains **11.915591 ms at 32 workers for Zopro versus 7.959184 ms at 96 workers for Aeglos Sorted**. Zopro has lower best-tested latency on six of nine AWS circuits and lower observed RSS on all nine. The supplied M5 CPU 10× objective remains **0/8**.

The latest optimization candidate, balanced G2 windows, records a **0.416% paired primary proof gain** versus accepted but fails its complete qualification gate and remains unadopted. Subsequent stage profiling and source-order readiness analysis clarify where to investigate; neither changes production or establishes a speedup. The cumulative distinct Gnark-verified export ledger is **25,231**.

## Accepted local proof results

Ryzen 9 9950X, 16 physical cores, 32 logical CPUs, 128 GiB RAM, shared host; 32 workers below. Milliseconds and MiB. CPU and Metal values are supplied historical M5 references, not new matched-hardware Apple measurements. Speedup divides reference time by Zopro time.

|Circuit|Zopro ms|Peak RSS MiB|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|134.00|37|4.612×|24|2.992×|3.7|
|2x2|11.335969|275.00|59|5.205×|30|2.646×|5.9|
|4x2|15.945544|315.00|109|6.836×|48|3.010×|10.9|
|8x2|24.090485|512.50|189|7.845×|74|3.072×|18.9|
|12x2|33.844855|707.25|284|8.391×|101|2.984×|28.4|
|16x2|39.931557|826.25|335|8.389×|124|3.105×|33.5|
|24x2|66.233309|1328.75|528|7.972×|176|2.657×|52.8|
|merge-24x1|65.273224|1175.50|507|7.767×|175|2.681×|50.7|
|merge-54x1|162.250464|1831.87|—|—|—|—|—|

There is no M5 reference for 54→1. Reaching 3.7 ms on primary 1×2 requires approximately 54% lower latency than the accepted result.

Accepted measurements use 15 process blocks, three priming proofs, then 151 primary or 31 other requests per process. SDK timings include fresh witness solving, randomized proving, internal verification and serialization. Loading/preparation are outside the latency interval and inside lifetime RSS. Peak RSS is the largest process high-water mark across blocks; it is not minimum RAM or incremental proof workspace. The local SDK executable SHA-256 is `20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f`.

The accepted build's original performance gate failed physical-16 control calibration. Adoption was an explicit engineering exception; a separate final-artifact qualification passed. The latest candidate does not inherit that exception. [Adoption](../2026-10-29-balanced-kh-adoption/README.md), [final artifact](../2026-10-29-balanced-kh-production/README.md).

## Latest full-proof optimization experiment (not adopted)

The candidate replaces the indexed narrow G2 top window with eleven balanced ordinary windows. It reuses borrowed points, the existing affine batches and LIFO queue. It adds no per-key lookup expansion, copied key point array, duplicate execution plan or persistent cache. Field arithmetic and workspace forecasts remain unchanged; unsupported shapes retain their original G2 route. [Implementation and source qualification](../2026-10-30-g2-balanced-sdk175/README.md).

This is a separate 31-block trial against the exact accepted executable and a fresh build with the candidate disabled (OFF). All nine shapes use 32 workers; primary also uses 16 physical cores. Blocks rotate and reverse case/variant order, with three primes followed by 151 primary or 31 other requests. No owned build, analyzer or independent workload overlaps capture. The following accepted-column values are fresh observations, not replacements for the accepted qualification table above.

|Case|Accepted ms|Fresh OFF ms|Candidate ms|Candidate peak RSS MiB|
|---|---:|---:|---:|---:|
|1x2-w32|8.042119|8.071193|8.013709|133.75|
|1x2-w16|9.697928|9.735707|9.668965|91.75|
|2x2-w32|11.319767|11.295129|11.225406|273.25|
|4x2-w32|16.048913|15.955362|15.945477|317.25|
|8x2-w32|24.300257|24.227034|24.096192|507.50|
|12x2-w32|33.908826|33.961610|33.908155|723.00|
|16x2-w32|40.282697|40.110852|40.052991|864.00|
|24x2-w32|66.310026|65.961542|65.972846|1312.61|
|merge-24x1-w32|65.245781|65.131764|65.075009|1177.45|
|merge-54x1-w32|162.256330|161.470804|161.864048|1924.79|

Paired candidate/control ratios below preserve block pairing and need not equal ratios of the displayed medians. Lower is faster. Bounds use sorted ranks 10–22 of 31, giving nominal 97.055% individual coverage under independent/exchangeable blocks, without a familywise guarantee.

|Comparison|Paired ratio|Bounds|Wins /31|Declared check|
|---|---:|---|---:|---|
|1x2-w32/ON/accepted|0.995843881|[0.994439313, 0.999216146]|23|Pass|
|1x2-w32/ON/OFF|0.992710208|[0.989573279, 0.998194757]|24|Pass|
|1x2-w16/ON/accepted|0.990701533|[0.974113289, 1.029951524]|16|Fail|
|1x2-w16/ON/OFF|0.993878085|[0.977024397, 1.030084544]|16|Fail|
|Original-eight aggregate / accepted|0.993530048|[0.991543018, 0.997731795]|27|Pass|
|Original-eight aggregate / OFF|0.996951509|[0.994048990, 1.000201673]|21|Fail|

Primary32 gain checks pass against both controls: 0.416% versus accepted and 0.729% versus OFF. The former corresponds to about 33 μs on the contemporaneous accepted median. The earlier complete G2 query improved 3.110% versus its frozen baseline, but that larger component gain did not transfer proportionally to complete proofs. [Query results](../2026-10-30-g2-balanced175/README.md).

The original-eight equal-log aggregate improves an estimated 0.647% versus accepted, with bounds below parity. Versus OFF, the estimate improves 0.305% but the bounds span parity. Physical-16 estimates also improve, but both upper bounds exceed the declared 1.02 nonregression tolerance. Those checks fail through uncertainty; the experiment does not establish a 3% physical-16 slowdown. All control-calibration, maximum-RSS and diagnostic checks pass. The complete gate is false and the candidate is not adopted. No favorable subset, changed threshold or unchanged rerun replaces that decision.

Primary32 summed CPU service falls from 182.948 to 171.309 ms/request versus accepted (about 6.36%), while wall latency improves 0.416% by the paired estimate. This difference does not isolate a cause. Six PMU panels meet running-coverage checks and six fixed-period profiles pass attribution/loss checks. Balanced-route symbols appear in ON profiles and not in either control. Existing absolute sample-period accounting limits remain; profile shares are qualitative, not exclusive costs or removable wall time. [All observations, RSS, counters, profiles and failed checks](../2026-10-30-g2-balanced-sdk175/README.md).

## Latest AWS comparison with Aeglos

c7a.24xlarge has 96 physical cores without SMT. The benchmark container has a 96-CPU quota, 30 GiB RAM and no swap; this is a measurement setting, not an architecture requirement. Each row selects the fastest median among 32/64/96 workers for each prover, with RSS from that selection. This is descriptive; the source report retains the full equal-worker matrix. AWS uses the accepted source built for Zen 4. Aeglos uses Sorted at commit `0290613e8eeca17a4ce88f6f2d726e8e06a5385d`, with prepared-key cache hits checked.

|Circuit|Zopro ms|Workers|Aeglos ms|Workers|Zopro RSS MiB|Aeglos RSS MiB|
|---|---:|---:|---:|---:|---:|---:|
|1x2|11.915591|32|7.959184|96|120.97|480.14|
|2x2|15.895282|32|11.822046|96|230.36|811.11|
|4x2|19.992623|32|18.492036|96|276.21|1351.90|
|8x2|26.897505|64|31.530774|96|450.82|2491.09|
|12x2|34.861964|64|48.579665|96|648.76|3714.61|
|16x2|38.983221|64|57.135549|96|766.20|4612.36|
|24x2|54.034119|64|86.666616|96|1225.00|6758.14|
|merge-24x1|57.132869|64|84.350321|96|1109.38|6608.05|
|merge-54x1|122.396711|64|173.236337|96|1877.03|13115.29|

AWS measurements use 15 process blocks after three primes, with 101 primary or 17 other timed requests per process. For 54→1 the selected peaks are 1.83 GiB for Zopro and 12.81 GiB for Aeglos. These configuration-specific observations do not isolate the reason for the memory or latency difference. [Full AWS report and artifacts](../2026-10-29-proof-pgo-c7a175/README.md).

|1×2 workers|Zopro ms|Aeglos ms|Zopro summed CPU ms/request|Aeglos summed CPU ms/request|
|---|---:|---:|---:|---:|
|32|11.915591|11.867991|256.128|328.248|
|64|11.994779|8.773943|420.399|430.446|
|96|13.777708|7.959184|704.247|513.394|

Aeglos lowers 1×2 latency as workers increase in this sweep. Zopro spends substantially more CPU without reducing latency. Critical-path work and coordination remain useful investigation targets; RAM capacity, cache behavior, syscalls or synchronization have not individually been established as the explanation. Local SMT measurements cannot predict this AWS scaling directly. No new AWS or matched local Aeglos capture was added by the balanced G2 trial.

## New diagnostic evidence

The newer [in-process stage timeline](../2026-10-30-query-timeline175/README.md) uses the accepted algorithm with low-frequency timestamps. Thirty-one fixed blocks compare the exact accepted executable, fresh OFF and traced at one, 16 and 32 workers. OFF and traced share an executable. Both control comparisons fit the declared factor 1.01 at one and 32 workers; both physical-16 checks fail and those observations remain instrumented-only. No timing sample or topology is discarded or repeated.

At 32 workers, fresh accepted/OFF/traced medians are **7.955612 / 7.954394 / 7.946960 ms**. These are diagnostic observations of the existing algorithm, not a newly accepted implementation or a replacement for the qualification table.

|32-worker stage or ordering observation|Result|
|---|---:|
|Native prefix through product completion, median|1.864279 ms|
|Solver interval, median|1.131876 ms|
|Product interval, median|0.544177 ms|
|Three-query join interval, median|5.237387 ms|
|K/H returns last|2,136 / 4,681 (45.6%)|
|B2 returns last|1,656 / 4,681 (35.4%)|
|A returns last|889 / 4,681 (19.0%)|
|Last query return minus second-last, median|0.008335 ms|

Solver and product are nested within the prefix; query functions run parallel work and can wait for internal Rayon tasks. These separate medians cannot be added to reconstruct a proof. Return order is not an exclusive CPU cost, scheduler-ready timestamp or removable wall-time fraction. The evidence does not establish G2, K/H or A as the sole bottleneck. It explains why a G2 component gain alone is insufficient to predict a full-proof gain, without establishing a causal model. At one worker, all three queries contribute serially; B2 returns last by execution order.

The [witness/query readiness census](../2026-10-30-wire-readiness175/README.md) then examines two exact accepted solver programs without solving witnesses or timing proofs:

|Query|Points|Ready at 50% of program ops|Key-order prefix at 50%|Ready at 90%|Key-order prefix at 90%|
|---|---:|---:|---:|---:|---:|
|1x2/A|21,006|10,621|2,896 (13.8%)|18,930|6,722 (32.0%)|
|1x2/B|30,089|14,573|4,248 (14.1%)|26,991|9,964 (33.1%)|
|2x2/A|36,637|18,548|3,563 (9.7%)|33,024|7,119 (19.4%)|
|2x2/B|52,274|25,256|5,209 (10.0%)|46,883|10,521 (20.1%)|

These percentages count serialized program operations, not elapsed time or readiness under the parallel schedule. Many operands become available before their key-order predecessors. The declared rejection condition—no prefix progress beyond initially known inputs by 90%—is false in all four populations. This therefore constrains simple sequential streaming assumptions without rejecting all streaming designs or qualifying an implementation. H readiness and folded K membership are not reconstructed. Final solver use also does not authorize early witness mutation: public correction and proving still consume values.

## Conclusions and next work

No new production speedup is accepted. Balanced G2 saves approximately 6.36% summed CPU service on primary32 versus its contemporaneous accepted control, but its paired wall gain is 0.416% and the complete gate fails. On AWS, adding workers to Zopro 1×2 increases CPU service without improving latency, while Aeglos scales in this sweep. The measured gap cannot currently be attributed solely to RAM capacity, cache misses, allocation, syscalls or one query.

The next implementation should target shared MSM work, arithmetic used across queries, or useful parallel execution. Before building a streaming pipeline, measure readiness under the actual parallel schedule and account for gathering, scalar grouping, G2 high/low component order and workspace admission. Overlap can hide work but does not by itself remove CPU work. Any arithmetic proposal must include carry, packing and representation-conversion costs before a component result is interpreted as a proof result. Fixed modulus constants are compatible with this direction; multi-GiB key lookup expansions are outside the intended architecture.

Exact integer dot-product arithmetic is an exploratory research lead only: no native kernel, correctness qualification or performance measurement has been completed for it. This report assigns it no expected gain. Closed arithmetic and overlap experiments retain their recorded outcomes.

The earlier combined grouping/drain candidate measured 1×2 at 7.846378 ms / 87 MiB with a 2.47% paired gain versus accepted, but failed 8×2 nonregression and physical-16 calibration. It remains a separate unadopted result. [Complete record](../2026-10-29-cached-drain175/README.md).

## Verification and limits

|Completed study|Validation retained|New distinct Gnark exports|Ledger after study|
|---|---|---:|---:|
|Balanced G2 full SDK|1,974 workflow steps; 55,788 internally verified requests including primes; native OFF/ON 127 core + 8 prover tests each; portable ON 101 core + 8 prover tests|972|24,943|
|In-process query timeline|576 workflow steps; 43,002 internally verified requests including primes; 126 core + 8 prover tests, including two journal tests|288|25,231|
|Source-order readiness census|7 workflow steps; 3 structural tests; producer counts and online frontier agree at all 11 cuts; no witness solves or proofs|0|25,231|

Independent Gnark exports are the last timed proof from each selected process, written after timing; they are not every internally verified request. Proof/public/image/verification-key bindings, hash distinctness and archive round trips are retained in the source reports. The counts above summarize existing receipts and are not newly executed validation. All 17 field sources remain identical to accepted; existing field qualification is inherited. The timeline adds no PMU or sampled-profile capture. Its separate offline review reproduces all latency cells, paired intervals, stage medians and return counts without rerunning a proof. The census uses two readiness accounting methods sharing a structural parser; it is not an independent arithmetic oracle.

No owned compiler, helper, hash, archive, analyzer or unrelated workload overlapped performance capture. All nine accepted local/AWS rows and eight supplied M5 CPU/Metal rows remain unchanged. No new matched local Aeglos or AWS comparison follows from these diagnostics. Historical reports retain their original decisions and values.

RSS is observed lifetime process peak, including loading/preparation; it is not minimum RAM, a workspace bound or a result under memory pressure. These small-circuit results do not establish forester 250/4999 performance, swap behavior, mobile/ARM/TEE performance or concurrent throughput. The latest AWS benchmark report records removal of its owned resources; this document makes no new AWS inventory check. Comparisons apply to the pinned configurations and do not generalize to all upstream implementations.

[Machine-readable snapshot](data.json), [source hashes](source-hashes.json), [checksums](SHA256SUMS), [document generator](generate.rs), [prior snapshot](../2026-10-30-current-bench-report/README.md), [rolling experiment history](../2026-10-29-current-cpu-summary/README.md).
