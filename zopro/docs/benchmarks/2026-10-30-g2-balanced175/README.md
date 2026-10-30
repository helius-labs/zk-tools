# Balanced G2 windows: complete-query comparison

30 October 2026. Ryzen9 9950X,16physical/32logical CPUs,128GiB RAM, shared host. This is a complete G2 B-query measurement, not proof latency or concurrent proof throughput. Production remains unchanged.

Declared caller gate **true**; diagnostic qualification **true**; eligible for separate full SDK qualification **true**. Failed checks: `[]`. All observations and both controls remain. No adoption follows from this stage.

Primary32 improves **3.110% versus FROZEN**, ratio0.968899994 [0.962179370,0.975113506], and3.815% versus OFF; both have31/31wins. The equal-log two-shape aggregate improves **3.467% versus FROZEN** and3.745% versus OFF, also31/31wins. All calibration and nonregression checks pass. This selects complete SDK qualification; it is not a full-proof gain.

Solo primary improves12.72% versus FROZEN. At16workers the latency intervals span parity, so no clear gain is established there despite lower summed CPU time. Primary32 summed CPU falls72.817→59.818ms per query versus FROZEN, about17.85%. The smaller wall-time gain shows that reduced CPU work does not translate proportionally to this parallel query's critical path. The captures do not isolate an exclusive stage cost or causal scheduling explanation.

## Change and correctness

The candidate replaces uniform G2 Phi2 windows with eleven variable spans covering127bits: widths `[11,12,11,12,11,12,11,12,11,12,12]`, starts `[0,11,23,34,46,57,69,80,92,103,115]`. Every span uses the original borrowed ordinary Window, high component then phi then low. This removes the narrow indexed top's routing and point-tile reduction, preserving original field arithmetic, Phi split, two partitions, early-stopping LIFO queue,152/200batches, original point operands and exclusive scalar storage. The previous queue-scan candidate is absent. The exact accepted K/H offset reader avoids a recoded digit array.

Requested bucket outputs fall20,544→17,408slots per part,15.26%; actual affine/projective work increases in the preceding formal model. That screen checks all digits, buckets and weighted whole sums but omits actual indexed arithmetic/inversion costs and curve exceptions; it makes no speed claim. This trial measures the tradeoff. [Model, assumptions and history](../2026-10-30-g2-balanced-count175/README.md).

Source176 retains172of175 originals exactly; reverse edits recover all175. All17field sources remain exact and inherit qualification. Existing heap forecasts already charge eleven half2048/batch200 parents plus indexed workspace; newparents/leaves fit the envelope and add no scalar vector, point copy, key table, duplicate plan or pool. Eleven span descriptors request176stackbytes; compiler frames, minimum RAM and memory-pressure behavior are not proven. Unsupported sizes/targets and disabled borrowed-prefix consumers retain the original route.

FreshOFF/ON each pass127unfiltered core and8prover tests. Three new actual-curve tests compare complete MSMs with independent known-generator scalar sums over empty/full/tail inputs, zeros/identities/repeated/opposite/doubling/sparse values, and custom1/4/16/32thread pools; original point arrays remain unchanged. API-only FROZEN retains its previously passed124core/8prover qualification without rebuilding or retesting. Original SDK consumers link without proof execution here.

Two immutable local fixtures and successful independent Arkworks complete sums are reused without a new witness solve, fixture regeneration or oracle execution. The prior oracle checked every point on-curve/in-subgroup and scalar canonicality. All three variants' primes and every timed query output match the whole-sum hashes; input point bytes remain unchanged. Raw points/scalars and unblinded result coordinates remain local. [Reuse and encoding](artifacts/reused-inputs.md).

## Fixed protocol

FROZEN is the exact prior API-only driver; freshOFF and ON share its driver/perf source byte-for-byte and use new coherent libraries, flags and private source. All hashes and feature receipts are checked. Runtime PID, workerTID and individual CPU pins are retained; whole-process taskset also pins the coordinator. Cases are primary soloCPU2, primaryphysical16, primarylogical32 and2×2logical32.

Twelve preflights use one query after three checked primes. Then31fixed blocks rotate/reverse cases and variants, with17timed queries after three primes per process. Per-query clocks include scalar-reset copy, grouping, point grouping, decomposition, fills/flushes, merges, weighted reductions, allocation/drop and final normalization. Loading/parse/solve/oracle checks are excluded. Displayed medians are medians of31process medians; paired ratios need not equal ratios of displayed medians. Lifetime RSS includes setup/fixture handling, not minimum RAM or proof workspace.

Primary32 and equal-log two-shape32 aggregate require upper ON/control ratios below1 and≥21/31wins versus BOTHcontrols. All other contexts require upper≤1.02; all four OFF/FROZENcalibration intervals must fitfactor1.03. Sorted ranks10–22 give nominal97.055%individual median coverage under independent/exchangeable blocks, nofamilywiseguarantee. No gain floor, exclusion, extension, threshold change, favorable subset or unchanged reroll. Passing selects all9+primary16fullSDK/independent-proof/resource/portable/final-artifact qualification, not adoption.

## Complete-query results

Milliseconds per complete query, not per proof.

|Case|FROZEN ms|OFF ms|ON ms|FROZEN RSS MiB|OFF RSS MiB|ON RSS MiB|
|---|---:|---:|---:|---:|---:|---:|
|1x2-w1|39.154019|39.601339|34.179514|15.25|15.25|15.25|
|1x2-w16|3.682639|3.675185|3.680655|27.25|27.25|26.50|
|1x2-w32|3.149615|3.172658|3.050047|32.00|31.25|29.50|
|2x2-w32|3.611615|3.625802|3.479647|67.00|60.25|60.25|

|Comparison|Ratio|Bounds|Wins /31|Pass|
|---|---:|---|---:|---|
|1x2-w1/ON/FROZEN|0.872777639|[0.871853844, 0.873815881]|31|true|
|1x2-w1/ON/OFF|0.863422056|[0.862541647, 0.864515060]|31|true|
|1x2-w16/ON/FROZEN|0.998033329|[0.991543543, 1.000433770]|20|true|
|1x2-w16/ON/OFF|0.997140325|[0.990969100, 1.004040467]|20|true|
|1x2-w32/ON/FROZEN|0.968899994|[0.962179370, 0.975113506]|31|true|
|1x2-w32/ON/OFF|0.961854183|[0.956868652, 0.969791791]|31|true|
|2x2-w32/ON/FROZEN|0.964876984|[0.958067185, 0.969171776]|31|true|
|2x2-w32/ON/OFF|0.958493838|[0.951343231, 0.968128176]|31|true|
|two-shape/ON/FROZEN|0.965331922|[0.960408940, 0.967504305]|31|true|
|two-shape/ON/OFF|0.962551838|[0.959789236, 0.966594881]|31|true|

|OFF/FROZEN calibration|Ratio|Bounds|Pass|
|---|---:|---|---|
|1x2-w1|1.010685895|[1.009622644, 1.012336552]|true|
|1x2-w16|1.001374584|[0.996592302, 1.007759990]|true|
|1x2-w32|1.007325238|[1.000059992, 1.010740233]|true|
|2x2-w32|1.002915336|[0.995168320, 1.011184642]|true|

## Automatic counters and profiles

All latency precedes six FIFO-gated grouped PMU panels31queries+3primes and six fixed16-million cycles:u profiles251queries+3primes, primary16/32 and2×2/32 OFF/ON. Setup/primes/final oracles are excluded from FIFO capture. All captures precede owner/decode/event/loss/mapping audits, analysis and publication. All helpers including analyzer/publisher/summary are compiled and pinned before capture. No independent owned compiler/helper/hash/archive/analyzer workload overlaps capture.

|Counter panel|Instructions/query|Cycles/query|Qualified|
|---|---:|---:|---|
|stat-1x2-w16-OFF|581847133|235633808|true|
|stat-1x2-w16-ON|520443319|200901095|true|
|stat-1x2-w32-OFF|600701557|335567394|true|
|stat-1x2-w32-ON|535388589|279529006|true|
|stat-2x2-w32-OFF|630376331|364465281|true|
|stat-2x2-w32-ON|563452315|308932773|true|

|Profile|Samples|Unknown period share|Attribution qualified|Lost|
|---|---:|---:|---|---:|
|cycles-1x2-w16-OFF|3685|0.008412|true|0|
|cycles-1x2-w16-ON|3150|0.006032|true|0|
|cycles-1x2-w32-OFF|5190|0.005395|true|0|
|cycles-1x2-w32-ON|4386|0.006840|true|0|
|cycles-2x2-w32-OFF|5627|0.007642|true|0|
|cycles-2x2-w32-ON|4755|0.006099|true|0|

PMU requires≥99.9%running coverage. Profiles require runtimePID=sole sampledPID, workerTID/CPU mapping, correct affinity/exec/task metadata/fixedCPUtimeperiod, zero loss,≥1000samples and≤5%unknown period. Profile-symbol records retain namespace attribution for the new route. Known absolute-period accounting deficits remain unresolved; categories are qualitative, not exclusive costs, local CPI or removable proof wall time. Single panels do not isolate causes.

ON profiles contain342/469/486samples in the new balanced namespace at primary16/primary32/2×2, versus zero in OFF. Static linked symbols also retain23new-namespace entries in ON and zero in OFF. Primary32 retired instructions fall600.702M→535.389M in the single diagnostic panels. This supports less CPU work in the measured query without assigning the entire change to one operation.

The workflow completes426steps,396processes and9216complete queries including primes, zero new solves/oracle executions/SDK requests/Gnark exports. Retained proof ledger23,971. No mobile/ARM/TEE, huge-circuit, pressure/swap, minimum-RAM or concurrent-throughput result is established.

## Accepted proof comparisons

Inherited local proof measurements and supplied historical M5 CPU/Metal references; all eight CPU10× targets remain unmet.

|Shape|Accepted local ms|RSS MiB|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|134.00|37.0|4.612×|24.0|2.992×|3.7|
|2x2|11.335969|275.00|59.0|5.205×|30.0|2.646×|5.9|
|4x2|15.945544|315.00|109.0|6.836×|48.0|3.010×|10.9|
|8x2|24.090485|512.50|189.0|7.845×|74.0|3.072×|18.9|
|12x2|33.844855|707.25|284.0|8.391×|101.0|2.984×|28.4|
|16x2|39.931557|826.25|335.0|8.389×|124.0|3.105×|33.5|
|24x2|66.233309|1328.75|528.0|7.972×|176.0|2.657×|52.8|
|merge-24x1|65.273224|1175.50|507.0|7.767×|175.0|2.681×|50.7|
|merge-54x1|162.250464|1831.87|—|—|—|—|—|

Latest AWS primary remains Zopro11.915591ms at32workers versus pinned Aeglos Sorted7.959184ms at96; no new AWS run. Zopro leads6/9best-countlatencies and9/9RSS in that matrix. Accepted physical16 calibration exception remains recorded and does not transfer to this trial. [Current comparison](../2026-10-30-benchmark-status-report/README.md).

[Full analysis](artifacts/analysis.json), [accepted rows](accepted-rows.json), [design](artifacts/design.md), [source recovery](artifacts/source-audit.json), [native implementation](artifacts/balanced.rs), [native tests](artifacts/native-tests.rs), [driver](artifacts/probe.rs), [pins](artifacts/pins.json), [sources](artifacts/source.json.gz), [receipts](artifacts/receipts.json.gz), [observations/plan](artifacts/observations-and-plan.json.gz), [inherited summary](artifacts/inherited-summary.json.gz), [archive checks](artifacts/archive-review.json), [local raw hashes](artifacts/raw-local-pins.json).
