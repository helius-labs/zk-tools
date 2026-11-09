# Indexed wide-band G2: complete-query comparison

**Rejected.** At 32 workers the complete primary G2 query takes **4.298202 ms**, versus fresh OFF **3.014502 ms** and accepted-source REF **3.050521 ms**. The paired ON/OFF ratio is **1.427178 [1.398414, 1.443233]**, with **0/15 wins**. Every B-query context regresses against both controls. These are query timings, not full-proof timings; accepted full-proof 1×2 remains **7.857170 ms**.

Nineteen of 46 declared checks fail: twelve B nonregression checks, four gain checks and three RSS guards. All seven control calibrations, both unchanged-A checks and all twelve diagnostic-quality checks pass. The strong calibrated negative closes this fixed candidate without SDK follow-up or adoption. [Results](artifacts/analysis.json), [separate raw review](artifacts/review.json).

## Implementation and controls

The [native-qualified candidate](../2026-11-09-indexed-wide-native176/README.md) uses fixed width14, nine ordinary positions plus signed carry, two source partitions and thirty-two 256-bucket leaves. Each ordinary index groups signed source ordinals into four component/partition classes. Leaves jointly reduce those classes through the original complete tree, merge high parts before φ, merge low/high sums, then return a local weighted sum and total for the existing outer recurrence. Index construction finishes before leaf execution and both phases use existing resource admission. No persistent lookup table, transformed key, extra production pool, new field kernel or RAM cap is added.

The [preceding structural model](../2026-11-09-indexed-wide-bands176/README.md) counted about 5% fewer elementary products and one third fewer scalar inverse boundaries. It explicitly did not price native routing, group scanning, packing, memory or scheduling. This complete-consumer measurement rejects a speedup despite those count reductions.

REF reuses the unchanged accepted176 API-only core and driver from the earlier completed campaign after exact identity/source checks. ON reuses the qualified indexed-wide core, unchanged. Only three missing builds run: OFF core from the same qualified178 source with zopro_indexed_wide omitted, and OFF/ON driver links. All share field1200, original dependencies, byte-identical driver/perf-support sources and native/fatLTO/oneCGU consumer flags. No field, ON core, REF, native test or standard SDK rebuild is performed. [Build review](artifacts/build-review.json), [identities](artifacts/identities.json).

Grouping remains enabled. Primary has 30,089 terms; 2×2 has 52,274 raw rows and 28,852 grouped terms. Both B ON paths report route318, including the production-grouped branch. A and both control arms report zero. No raw route is forced. [Routing](artifacts/fixture-routing.json).

## Latency

Local Ryzen9950X, 16 physical/32 logical CPUs. Fifteen fixed matched blocks rotate/reverse seven cases and three arms. Each process has three primes, seventeen timed queries and one postcheck. Solo uses CPU2; other worker i uses CPUi, with 512 KiB driver stacks and coordinator affinity restricted to the case CPUs. Loading and pool creation are excluded. Scalar reset, grouping, both admissions, split, index construction/barrier, signed copies, all arithmetic, allocation/drop and final affine normalization are included.

Times are medians of process medians. Ratios pair blocks; bounds use fixed ranks4–12 of15, nominal96.484375% marginal median coverage under independent exchangeable blocks, without familywise coverage. Ratios need not equal displayed-median quotients. All observations remain.

|Case|REF ms|OFF ms|ON ms|ON/OFF [bounds]|Wins/15|
|---|---:|---:|---:|---|---:|
|1x2-A-w32|1.178421|1.185585|1.181307|0.994488 [0.979068, 1.004174]|9|
|1x2-B-w1|37.327352|37.129258|55.626235|1.498136 [1.495538, 1.500586]|0|
|1x2-B-w16|3.423133|3.428453|5.024762|1.465657 [1.443372, 1.480022]|0|
|1x2-B-w32|3.050521|3.014502|4.298202|1.427178 [1.398414, 1.443233]|0|
|2x2-B-w1|39.659208|39.511409|57.927562|1.464820 [1.463717, 1.466844]|0|
|2x2-B-w16|3.897277|3.926001|5.619022|1.430422 [1.410499, 1.433658]|0|
|2x2-B-w32|3.471604|3.468258|4.906779|1.412598 [1.392103, 1.439781]|0|

Primary ON loses every paired block at all three worker counts against both controls. Solo worsens from37.129258 to55.626235ms, so high-worker scaling alone does not explain the regression. Grouped2×2 at32 workers worsens from3.468258 to4.906779ms. The equal-log two-B-shape32 aggregate ON/OFF is1.424889[1.409532,1.433540],0/15wins.

The frozen gate requires all seven OFF/REF intervals within[1/1.03,1.03], fourteen ON/control nonregression upper bounds≤1.02, primary32 and equal-log B32 aggregate upper<1 and≥10/15wins against both controls, both unchanged-A intervals within[1/1.03,1.03], each ON maxRSS≤max(REF,OFF)×1.05+1MiB, and all twelve diagnostic checks. No minimum gain, exclusion, sample extension, reroll, parameter rescue or threshold change is used. Passing would permit later full-SDK qualification, not adoption. [Protocol](artifacts/protocol.md), [pre-clock review](artifacts/prospective-review.json).

Failed checks:

- `1x2-B-w1/ON/OFF/nonregression`
- `1x2-B-w1/ON/REF/nonregression`
- `1x2-B-w16/ON/OFF/nonregression`
- `1x2-B-w16/ON/REF/nonregression`
- `1x2-B-w16/RSS`
- `1x2-B-w32/ON/OFF/nonregression`
- `1x2-B-w32/ON/OFF/gain`
- `1x2-B-w32/ON/REF/nonregression`
- `1x2-B-w32/ON/REF/gain`
- `1x2-B-w32/RSS`
- `2x2-B-w1/ON/OFF/nonregression`
- `2x2-B-w1/ON/REF/nonregression`
- `2x2-B-w16/ON/OFF/nonregression`
- `2x2-B-w16/ON/REF/nonregression`
- `2x2-B-w16/RSS`
- `2x2-B-w32/ON/OFF/nonregression`
- `2x2-B-w32/ON/REF/nonregression`
- `B32-two-shape/ON/OFF/gain`
- `B32-two-shape/ON/REF/gain`

## CPU service and memory

CPU service is summed user+system time per query, then median process mean. RSS is maximum lifetime rusage among latency processes, including load/setup/primes. Faults/switches are timed-interval differences per query, then process medians. The unchanged driver exposes minor faults, not major faults. These are not minimum RAM, swap/pressure qualification or a rigorous live working-set bound.

|Case / arm|CPU ms/query|Peak RSS MiB|Minor faults/query|Voluntary switches/query|
|---|---:|---:|---:|---:|
|1x2-B-w1 / REF|37.327|15.50|0.118|2.000|
|1x2-B-w1 / OFF|37.125|15.50|0.118|2.000|
|1x2-B-w1 / ON|55.632|16.75|0.118|2.000|
|1x2-B-w16 / REF|45.203|27.75|43.588|120.235|
|1x2-B-w16 / OFF|44.879|28.25|39.235|118.765|
|1x2-B-w16 / ON|63.411|31.25|37.647|119.765|
|1x2-B-w32 / REF|70.698|35.00|84.000|196.529|
|1x2-B-w32 / OFF|70.173|33.00|61.471|194.471|
|1x2-B-w32 / ON|94.949|43.25|49.529|207.235|
|2x2-B-w1 / REF|39.676|27.25|0.118|2.000|
|2x2-B-w1 / OFF|39.515|27.25|0.118|2.000|
|2x2-B-w1 / ON|57.956|28.25|0.118|2.000|
|2x2-B-w16 / REF|48.422|43.75|29.412|107.824|
|2x2-B-w16 / OFF|48.376|43.25|25.176|108.118|
|2x2-B-w16 / ON|66.830|48.75|36.353|121.882|
|2x2-B-w32 / REF|75.944|60.75|39.941|189.176|
|2x2-B-w32 / OFF|75.697|60.25|39.882|185.529|
|2x2-B-w32 / ON|101.305|63.50|53.647|200.471|
|1x2-A-w32 / REF|24.693|21.75|68.941|75.176|
|1x2-A-w32 / OFF|24.729|23.25|67.353|77.176|
|1x2-A-w32 / ON|24.716|23.50|69.353|77.176|

Primary32 OFF→ON CPU service rises70.173→94.949ms and RSS33→43.25MiB, while minor faults fall61.471→49.529/query. Solo also slows substantially with the same0.118minor faults/query. More page faults alone cannot explain this result. Source capacity forecasts describe candidate kernel buffers, not whole-process RSS; their separate native qualification remains valid within that scope.

## Counters and automatic profiles

All315latency processes precede six grouped PMU panels and six separate cycles:u recordings, each200checkedqueries. FIFO excludes load/primes/postcheck; every recording finishes before decoding/analysis. Every grouped PMU event has100% scheduled coverage. These are individual diagnostic panels, not repeated-counter confidence intervals. Generic cache misses do not identify a cache level, DRAM traffic or a bandwidth bottleneck.

|Panel|Instructions M/query|Cycles M/query|Generic misses M/query|Branch misses/query|
|---|---:|---:|---:|---:|
|stat-1x2-B-w16-OFF|549.944346|222.723629|0.892397|270364.260|
|stat-1x2-B-w16-ON|693.663669|319.353641|3.039584|1460351.020|
|stat-1x2-B-w32-OFF|568.176601|318.959892|1.263943|316084.860|
|stat-1x2-B-w32-ON|708.897292|431.976047|3.710998|1518266.680|
|stat-2x2-B-w32-OFF|596.365463|346.142349|1.309535|317575.975|
|stat-2x2-B-w32-ON|732.965960|464.203391|3.822144|1473285.580|

Primary32 instructions rise568.177→708.897million, about24.8%, and cycles318.960→431.976million. Candidate generic misses and branch misses also rise. Fewer abstract products did not mean less executed work.

All six profiles qualify with26,235samples, zeroLOST/LOST_SAMPLES, exact16Mcycle period, allowedPID/TID/CPU ownership and matching executableMMAP/FORK metadata. Maximum unknown share is1.367%. Absolute coverage of the whole timed cycle budget is not validated.

|Profile|Samples|Unknown share|Qualified|
|---|---:|---:|---|
|cycles-1x2-B-w16-OFF|2778|0.7919%|true|
|cycles-1x2-B-w16-ON|3970|1.2091%|true|
|cycles-1x2-B-w32-OFF|3960|1.0606%|true|
|cycles-1x2-B-w32-ON|5419|1.3287%|true|
|cycles-2x2-B-w32-OFF|4328|0.9473%|true|
|cycles-2x2-B-w32-ON|5780|1.3668%|true|

Primary32 scalar-inverse samples decline303→236 while total samples rise3,960→5,419. reduce_layer samples rise630→2,321; the candidate leaf closure has956samples, index-construction closure189, local merge142, and g2_prefix rises82→568. This locates work in reduction and surrounding leaf processing, but flat symbols include inlining and do not isolate exclusive packing/routing costs or the critical path. Retained profiles can support an offline source/assembly audit; another timing run is unnecessary to establish this negative. [Full symbols](artifacts/symbols.json).

## Verification and disposition

All378capture-workflow commands,348callers and9,168complete-query invocations, including primes/postchecks, complete successfully once. Every result matches the retained independent Arkworks full-MSM hash, points remain unchanged, and route/placement receipts match. Oracle qualification is inherited and hash-bound, not replayed; these are not new Groth16 proofs.

The separate same-agent reader validates426pins, command/stream/managed-environment receipts, all raw latency ratios and46decisions,21resource cells and profile ownership. Ratio/selection reconstruction is independent; PMU/sample text-schema parsers are shared. The frozen analyzer and reader both pass first execution. No recording is repeated. Two pre-clock helper compile failures are retained: an extra delimiter in the offline build reader and an omitted demangle extern in the initial capture-helper compiler command. Neither changes scientific source/builds or observations.

Close this fixed width14 indexed geometry without SDK follow-up. Correctness and source-model identities remain valid; the modeled count saving did not produce a CPU or memory improvement. Before another proposal, use retained profiles and the whole-proof phase budget to identify a substantial reduction in actual consumer work. No replacement algorithm is selected here.

## Accepted CPU and Metal comparison, inherited

These are unchanged accepted local Ryzen9950X full-proof results with32workers and common fixed placement from the completed SDK campaign. They are not candidate query timings or new M5/AWS measurements. Fixed placement is not a production default.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.857170|37|24|4.709×|3.055×|3.7|
|2x2|11.052363|59|30|5.338×|2.714×|5.9|
|4x2|15.952018|109|48|6.833×|3.009×|10.9|
|8x2|24.080322|189|74|7.849×|3.073×|18.9|
|12x2|34.012417|284|101|8.350×|2.970×|28.4|
|16x2|40.128912|335|124|8.348×|3.090×|33.5|
|24x2|66.348981|528|176|7.958×|2.653×|52.8|
|merge-24x1|65.829291|507|175|7.702×|2.658×|50.7|

Goal active and unmet,0/8. Production176/standardSDK6d83 remain unchanged; canonical verified-proof ledger remains40,154. No new SDK requests, proofs, Gnark exports, AWS/Aeglos, mobile/ARM, huge-circuit, pressure, swap, minimum-RAM or throughput result follows. No cloud resource is owned.

[Continuation](artifacts/next.md), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS).
