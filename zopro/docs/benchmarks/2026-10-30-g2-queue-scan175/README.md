# Bounded G2 collision-queue scan: complete-query comparison

30 October2026. Ryzen9 9950X,16physical/32logical CPUs,128GiB RAM, shared host. This is a complete G2 B-query experiment, not a proof-latency or concurrent-throughput measurement. Production remains unchanged.

Declared caller gate **false**; automatic diagnostic qualification **true**. Failed checks: `["two_shapes/ON/FROZEN/gain"]`. Eligible for a separate full SDK stage: **false**. No adoption follows from this stage.

Primary 1×2 at32workers improves **2.059% versus FROZEN** (27/31wins) and **3.843% versus OFF** (31/31wins). Gains also appear at1/16workers against both controls. The two-shape aggregate improves2.706% versus OFF, but its FROZEN comparison is **0.994737985 [0.986825015, 1.001848413],21/31wins**: that interval spans parity and fails the required gain check. All individual nonregression and control-calibration checks pass. The positive primary result is retained; the full gate does not qualify SDK advancement under this protocol. No unchanged reroll or primary-only promotion is selected.

Fresh OFF is systematically slower than the API-only FROZEN control, by about1.0–1.7% in paired median estimates, despite all calibration intervals fitting factor1.03. Consequently, candidate gains versus OFF should be read alongside the smaller FROZEN-relative estimates. These captures do not isolate the source of the control difference. The 2×2 ON/FROZEN interval spans parity within the nonregression tolerance; its upper endpoint is not an established slowdown.

## Mechanism and correctness

The borrowed G2 drain scans the existing bounded queue in descending order, swap-removing entries with free destinations while retaining busy entries. The control stops at the first busy tail. Busy bits only grow during draining, so an already inspected moved tail stays blocked. The change preserves original arithmetic, exceptional handling, width12, two partitions, batch200, feed/flush, allocation forecasts and worker policy. It adds no heap buffer, key table, point copy, plan, pool or persistent cache. Queue order changes. This differs from the earlier flush-queue candidate.

The preceding noncancelling count model reduces primary projective arrivals1178→617 and finish merges429→114, adding5 inversions and8779 checks. Only about0.10% of all primary arrivals change processing category; no 2× gain is inferred. [Count assumptions and per-window results](../2026-10-30-g2-queue-scan-count175/README.md).

Native API-only baseline124core/8prover and freshOFF/ON126core/8prover tests pass unfiltered. New tests exercise blocked-tail routing, actual bucket sums, full/tail cases, signs, identities, doubling, cancellation and input immutability. Both source trees reverse to all175 accepted files; all17field files are exact. Original SDK consumers link without running proofs.

Two local fixtures contain actual B key points and one freshly solved scalar population each,30089/52274 points. Independent Arkworks0.5 checks every point on the curve and in the prime subgroup, scalar canonicality, and complete MSM outputs before timing. All native process primes and every timed output match those independent complete sums; input points remain unchanged. Raw points/scalars and unblinded result coordinates remain local. Digests are published.

## Scope and fixed protocol

API-only FROZEN is accepted source plus the private complete-query export. FreshOFF/ON share that export, candidate guard and two common tests. Their coherent libraries, flags, source and executable hashes are pinned. FreshOFF/FROZEN calibration is required in all four contexts. The baseline is rebuilt once for this API and is not the old accepted SDK executable.

Whole-process taskset constrains coordinator and workers; each Rayon worker is individually pinned with TID/CPU receipts. Cases cover primaryCPU2solo, primaryphysical16, primarylogical32 and2×2logical32. Twelve preflights execute one query after three primes. Then31 fixed blocks rotate/reverse four cases and three variants; each process has three checked primes and17 timed queries. Displayed times are medians of31 process medians. Scalar-reset copy, grouping, point grouping, decomposition, window fills/flushes, top/weighted reductions, merging, allocation/drop and normalization are timed. Parsing, fresh solve and oracle checking are excluded. Lifetime RSS includes setup/fixture handling and is not proof workspace or minimum RAM.

Primary32 and equal-log two-shape32 aggregate require upper ON/control ratios below1 and at least21/31wins against both controls. Other contexts require upper≤1.02. Every OFF/FROZEN interval must fit factor1.03. Sorted ranks10–22 give nominal97.055% individual median coverage under independent/exchangeable blocks, without a familywise guarantee. No gain floor, exclusions, extension, threshold change, favorable subset or unchanged reroll. Ratios pair blocks and can differ from ratios of displayed medians. Passing selects separate all9+primary16 fullSDK/independent proof/resource/portable/final-artifact qualification.

## Complete-query results

Times are milliseconds per complete query, not per proof. Lower ratios are faster.

|Case|FROZEN ms|OFF ms|ON ms|FROZEN RSS MiB|OFF RSS MiB|ON RSS MiB|
|---|---:|---:|---:|---:|---:|---:|
|1x2-w1|39.645385|40.166891|38.987319|15.25|15.25|15.75|
|1x2-w16|3.689412|3.726081|3.622326|27.75|27.00|28.25|
|1x2-w32|3.152680|3.217071|3.096996|34.00|32.00|32.50|
|2x2-w32|3.617287|3.669935|3.626353|60.25|60.25|60.00|

|Comparison|Ratio|Bounds|Wins /31|Pass|
|---|---:|---|---:|---|
|1x2-w1/ON/FROZEN|0.982831495|[0.981848452, 0.984025223]|31|true|
|1x2-w1/ON/OFF|0.970117231|[0.969140043, 0.970747044]|31|true|
|1x2-w16/ON/FROZEN|0.980747899|[0.973785395, 0.988450010]|30|true|
|1x2-w16/ON/OFF|0.972578104|[0.966760061, 0.975715786]|31|true|
|1x2-w32/ON/FROZEN|0.979410323|[0.974169979, 0.993083924]|27|true|
|1x2-w32/ON/OFF|0.961570925|[0.955313634, 0.977715646]|31|true|
|2x2-w32/ON/FROZEN|1.011981226|[0.996051437, 1.016533791]|12|true|
|2x2-w32/ON/OFF|0.987963669|[0.981084743, 0.996696754]|23|true|
|two-shape/ON/FROZEN|0.994737985|[0.986825015, 1.001848413]|21|false|
|two-shape/ON/OFF|0.972938740|[0.969242405, 0.984675880]|29|true|

|OFF/FROZEN calibration|Ratio|Bounds|Pass|
|---|---:|---|---|
|1x2-w1|1.013624245|[1.011952951, 1.014510153]|true|
|1x2-w16|1.010366745|[1.006256807, 1.017903937]|true|
|1x2-w32|1.016591407|[1.007350572, 1.025048718]|true|
|2x2-w32|1.016051093|[1.010667055, 1.023561398]|true|

## Automatic counters and profiles

All latency precedes six FIFO-gated four-event PMU panels31queries+3primes and six fixed16-million cycles:u profiles251queries+3primes, primary16/32 and2×2/32 OFF/ON. Setup/primes and final output checks are outside FIFO gates. All captures precede owner transfer, decoding, audits, analysis and publication. All helpers, including this publisher and summary updater, were compiled/pinned before capture; no independent owned compiler/helper/hash/archive/analyzer work overlaps capture.

|Counter panel|Instructions/query|Cycles/query|Qualified|
|---|---:|---:|---|
|stat-1x2-w16-OFF|581745327|238935449|true|
|stat-1x2-w16-ON|577865251|231637953|true|
|stat-1x2-w32-OFF|601404263|339839235|true|
|stat-1x2-w32-ON|595439981|326737674|true|
|stat-2x2-w32-OFF|628371757|367242583|true|
|stat-2x2-w32-ON|630031350|361773859|true|

|Profile|Samples|Unknown period share|Attribution qualified|Lost|
|---|---:|---:|---|---:|
|cycles-1x2-w16-OFF|3747|0.007473|true|0|
|cycles-1x2-w16-ON|3628|0.008820|true|0|
|cycles-1x2-w32-OFF|5269|0.009300|true|0|
|cycles-1x2-w32-ON|5148|0.010684|true|0|
|cycles-2x2-w32-OFF|5730|0.010820|true|0|
|cycles-2x2-w32-ON|5637|0.007451|true|0|

PMU requires≥99.9% running coverage. Profiles require at least1000 samples,≤5%unknown period, the runtime PID, known workerTID/CPU mapping, whole-process affinity, fixed period, exec/task metadata and zero lost records. Previous absolute sample-period accounting deficits remain unresolved. Profile categories are qualitative, not exclusive costs, local CPI or removable proof wall time. Single counter/profile panels do not establish cause or repeated-trial confidence.

## Repairs and retained evidence

The initial fixture launch was rejected by the host lock before any step ran. Once native work completed, fixture preparation succeeded with exactly two fresh solves. The first independent checker rejected the first point because its decoder assumed Montgomery little-endian words; public decompression actually returns canonical big-endian Gnark x.c1,x.c0,y.c1,y.c0. Both fixtures were retained unchanged. A corrected independent checker completed both sums in a separate two-step workflow. Before any driver execution, source review found decode_g2 also expected kernel Montgomery bytes; all drivers were relinked with explicit canonical coordinate parsing and component ordering. All first-link binaries/receipts remain local/hash-indexed. Original fixture metadata/source/failure, corrected encoding receipt and successful oracles remain in the archives. A link-helper ownership compile error was corrected before execution. No successful runtime preflight, native test, oracle or performance capture was repeated. [Encoding correction](artifacts/fixture-encoding-repair.md).

The controller's first compilation also detected a duplicate median helper imported from the inherited sampling parser. Removing that duplicate before controller execution changed no parser, gate or workload. Initial source and compiler log are retained. The native fixture-preparer's earlier array-method compile repair is retained from the preceding preparation stage. Capture, analysis and publication then complete on their first executions. [Additional build and publication records](artifacts/offline-review.json).

The426-step workflow completes396 processes and9216complete queries including primes, with0SDK requests and0new independent proof exports. Retained proof ledger23971. These results do not establish ARM/mobile/TEE, huge-circuit, memory-pressure/swap, minimum RAM, concurrent throughput or forester250/4999 behavior.

## Accepted proof comparisons

The following accepted measurements and supplied historical M5 references are inherited; no fresh proof or Apple measurement is added. All eight CPU10× targets remain unmet.

|Shape|Accepted local ms|Peak RSS MiB|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× target ms|
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

Latest AWS best-count1×2 remains Zopro11.915591ms at32workers versus pinned Aeglos Sorted7.959184ms at96workers. Zopro leads6/9best-countlatencies and9/9RSS; no new AWS measurement. The accepted build's physical16 calibration exception remains documented and is not inherited here. [Accepted AWS matrix](../2026-10-29-proof-pgo-c7a175/README.md), [benchmark status snapshot](../2026-10-30-benchmark-status-report/README.md).

[Full analysis](artifacts/analysis.json), [accepted rows](accepted-rows.json), [design](artifacts/design.md), [diagnostic counts](artifacts/capture-addendum.md), [source recovery](artifacts/source-audit.json), [native tests](artifacts/queue-tests.rs), [driver](artifacts/probe.rs), [capture pins](artifacts/pins.json), [sources](artifacts/source.json.gz), [receipts](artifacts/receipts.json.gz), [observations/plan](artifacts/observations-and-plan.json.gz), [exact inherited summary](artifacts/inherited-summary.json.gz), [archive checks](artifacts/archive-review.json), [local raw hashes](artifacts/raw-local-pins.json).
