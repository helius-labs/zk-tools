# Extract scalar fallback from G2 prefix kernels

30 October 2026. Real-operand caller qualification. Production remains unchanged; this is not a proof-latency measurement or adoption.

Declared caller gate: **false**. Failed checks: `["1x2-dominant-w32/ON/OFF","1x2-dominant-w32/ON/FROZEN","1x2-short-tail-w32/OFF/FROZEN/calibration","merge-54x1-all-tails-w32/ON/OFF","original8/ON/OFF/gain","original8/ON/FROZEN/gain"]`. Primary32 ON/OFF ratio **1.010588058 [1.005824531, 1.015566998], 6/31 wins**; ON/FROZEN **1.009118060 [1.002744032, 1.012568595], 8/31 wins**. Lower is faster.

The candidate is rejected before full-SDK timing. Primary32 is **1.059% slower versus OFF and 0.912% slower versus FROZEN**, with both intervals above parity. The original-eight aggregate spans parity against both controls, so neither establishes a gain. The merge54 all-tail interval spans parity but its 1.02136 upper bound fails the 1.02 nonregression limit; that bound is not an established 2.14% slowdown.

The primary short-tail suite improves by **5.10% versus OFF and 2.52% versus FROZEN**, with 31/31 wins against each. This positive cell is retained. Its OFF/FROZEN calibration bounds are [1.025804167, 1.030894600], exceeding the declared 1.03 upper limit. The complete gate remains false; neither this cell nor a favorable subset is promoted.

## Change and qualification

Both G2 prefix functions move their scalar-tail/fallback inversion statements into private cold, never-inline helpers. Forward/reverse SIMD arithmetic, fast inversion, record order, tails, signs and field routines remain unchanged. The helper bodies match the original statements exactly and borrow existing chains, points, buckets and scratch. There is no new heap buffer, key table, point copy, plan, pool, width, readiness dependency or forecast. The [failed endpoint candidate](../2026-10-30-g2-endpoints175/README.md) is not included. Earlier whole-kernel specializations remain rejected. [Design](artifacts/design.md), [history](artifacts/history-audit.md).

The private source has 177 files: 171 of 175 originals are exact; reversing four parent edits and removing two children recovers all 175. All 17 field files remain byte-exact and inherit qualification. The common replay adapter is trace-free, with no recorder lock or capture hook. Its operational prefix and the replay driver match the qualified prior OFF implementation. One new test runs owned/borrowed, canonical/inverse modes and ten empty/full/tail sizes on a 512 KiB thread stack against known-generator sums. Native OFF/ON each pass 127 unfiltered core and eight prover tests. No portable build or minimum-stack claim is made.

Static implementation checks require smaller parent functions and calls to separately emitted fallback helpers. These verify the intended extraction, not its timing benefit. Combined helper/parent stack depth and total executable/RSS are not assumed smaller.

|Parent|OFF bytes|ON bytes|Cold call present|
|---|---:|---:|---|
|borrowed_true|51950|29641|true|
|owned_false|51371|29033|true|
|owned_true|50288|28241|true|

## Measurement scope

Ryzen 9 9950X, 16 physical cores and 32 logical CPUs, shared host. FROZEN is the exact previous trace-free OFF replay executable, with source recovery, common-adapter, flag and library hashes checked; it is reused without rebuilding. Fresh OFF and ON share source and flags apart from the private candidate cfg. Original SDK consumers are linked but not run.

Twenty-two cases include all nine real dominant 200-pair contexts at 32 workers, primary owned-512/32, primary dominant-200/16, all nine complete captured tail suites, and explicit signed-primary/unsigned-merge54 short-tail suites. Every captured tail fixture belongs to a timed tail suite. All 152 immutable fixtures preflight in all three variants, with outputs matching the inherited independent Arkworks oracles. Raw coordinates remain local; no old capture or oracle is repeated. [Fixture provenance](../2026-10-30-real-g2-replay175/README.md).

Thirty-one fixed blocks rotate/reverse case and variant order. Each worker performs three checked 256-round primes, then 2,048 rounds over the case’s fixture list. Fixture counts vary and actual call counts are retained. Compact replay preserves arithmetic inputs, signs, order and tails, but does not reproduce key/bucket locality or proof scheduling. Reset copies are timed; parsing, reference construction, checks and snapshots are outside every worker’s clock through barriers. Workers pin to individual CPUs and the whole process constrains the coordinator. All timed global allocations are zero; outputs and retained capacities match across variants.

## Caller results

Medians are microseconds per complete batch wave; ratios pair process means within blocks and need not equal ratios of displayed medians. Check types differ: primary32 requires gain; other cases use the nonregression tolerance.

|Case|FROZEN µs|OFF µs|ON µs|ON/OFF ratio|Bounds|Wins /31|Check|
|---|---:|---:|---:|---:|---|---:|---|
|12x2-all-tails-w32|27.294936|27.194183|27.410357|1.003249803|[0.991932657, 1.013778392]|14|true|
|12x2-dominant-w32|15.870682|15.776991|15.747039|0.999308265|[0.995564116, 1.007802375]|16|true|
|16x2-all-tails-w32|28.948754|29.275813|29.150942|0.991236521|[0.983895561, 1.003184152]|19|true|
|16x2-dominant-w32|15.618694|15.628767|15.675660|1.005942179|[0.992976356, 1.013428075]|12|true|
|1x2-all-tails-w32|4.947340|5.001310|4.954210|0.987496501|[0.972942082, 0.999086640]|23|true|
|1x2-dominant-w16|10.123587|10.090443|10.228520|1.013017751|[1.009110679, 1.016598593]|2|true|
|1x2-dominant-w32|16.694836|16.665158|16.823371|1.010588058|[1.005824531, 1.015566998]|6|false|
|1x2-owned512-w32|40.271124|40.373635|40.367965|1.002137424|[0.996387321, 1.005470285]|14|true|
|1x2-short-tail-w32|3.485998|3.581715|3.398435|0.948979153|[0.944187445, 0.952873476]|31|true|
|24x2-all-tails-w32|18.082927|18.097725|18.038112|0.992472478|[0.982604397, 0.999986670]|22|true|
|24x2-dominant-w32|15.803260|15.783972|15.766167|0.996861775|[0.991737691, 1.005619724]|16|true|
|2x2-all-tails-w32|3.722695|3.754180|3.619969|0.960215575|[0.956302575, 0.965922906]|29|true|
|2x2-dominant-w32|16.674880|16.641392|16.775707|1.008292085|[1.005366841, 1.012635131]|6|true|
|4x2-all-tails-w32|22.783451|22.923927|22.578985|0.983637396|[0.981278271, 0.996912584]|24|true|
|4x2-dominant-w32|15.837679|15.855735|15.774971|0.998160960|[0.990858456, 1.005484528]|18|true|
|8x2-all-tails-w32|23.902170|24.101806|24.007214|0.993246681|[0.986164834, 1.012212695]|17|true|
|8x2-dominant-w32|15.727161|15.741241|15.733604|0.995593435|[0.991477620, 1.002784026]|21|true|
|merge-24x1-all-tails-w32|28.843200|28.832416|28.702057|0.993934264|[0.988441265, 1.004015554]|19|true|
|merge-24x1-dominant-w32|15.684615|15.700121|15.784440|1.003068358|[0.997389553, 1.007371153]|13|true|
|merge-54x1-all-tails-w32|22.494382|22.488473|22.690788|1.006487729|[0.986557788, 1.021362945]|12|false|
|merge-54x1-dominant-w32|15.742137|15.745530|15.735450|1.001560681|[0.993005915, 1.006683880]|14|true|
|merge-54x1-short-tail-w32|3.426400|3.437294|3.177495|0.925557793|[0.918159311, 0.933931930]|31|true|

|Case|ON/FROZEN ratio|Bounds|Wins /31|Check|OFF/FROZEN calibration bounds|Calibration|
|---|---:|---|---:|---|---|---|
|12x2-all-tails-w32|0.993370036|[0.990070230, 1.010257539]|18|true|[0.982961009, 1.007072254]|true|
|12x2-dominant-w32|0.995056761|[0.989321457, 1.002962367]|20|true|[0.986806444, 1.001808851]|true|
|16x2-all-tails-w32|1.006083981|[0.990850332, 1.019849734]|12|true|[1.001986082, 1.017521706]|true|
|16x2-dominant-w32|1.000885349|[0.989607620, 1.009231701]|14|true|[0.993491911, 1.001526435]|true|
|1x2-all-tails-w32|0.994277058|[0.982487422, 1.010272814]|18|true|[1.005543611, 1.013367757]|true|
|1x2-dominant-w16|1.011725945|[1.009275445, 1.015178163]|3|true|[0.994757317, 0.999958669]|true|
|1x2-dominant-w32|1.009118060|[1.002744032, 1.012568595]|8|false|[0.993012698, 1.001447771]|true|
|1x2-owned512-w32|1.001422988|[0.995383915, 1.006675388]|15|true|[0.993321148, 1.004758134]|true|
|1x2-short-tail-w32|0.974798071|[0.969546458, 0.979540276]|31|true|[1.025804167, 1.030894600]|false|
|24x2-all-tails-w32|0.999871523|[0.990088890, 1.008201128]|17|true|[0.995557698, 1.012160855]|true|
|24x2-dominant-w32|1.000294846|[0.988875371, 1.004885043]|15|true|[0.992832551, 1.003518157]|true|
|2x2-all-tails-w32|0.971563771|[0.968260968, 0.977703587]|29|true|[1.007145077, 1.014071165]|true|
|2x2-dominant-w32|1.007244041|[1.003934968, 1.010791046]|4|true|[0.994939113, 1.000790905]|true|
|4x2-all-tails-w32|0.991183299|[0.986719000, 0.994434390]|25|true|[0.997438408, 1.010408742]|true|
|4x2-dominant-w32|1.000317375|[0.993381126, 1.004142562]|15|true|[0.992066600, 1.007240902]|true|
|8x2-all-tails-w32|0.994962955|[0.982995584, 1.012789056]|17|true|[0.989709870, 1.016969796]|true|
|8x2-dominant-w32|1.000355768|[0.991537480, 1.005282669]|15|true|[0.995035182, 1.008733352]|true|
|merge-24x1-all-tails-w32|0.996945074|[0.988106676, 0.999984353]|22|true|[0.994528042, 1.009051841]|true|
|merge-24x1-dominant-w32|1.005902878|[0.998104350, 1.009683903]|12|true|[0.997467128, 1.007879995]|true|
|merge-54x1-all-tails-w32|1.007782130|[0.994388093, 1.015112099]|11|true|[0.977151667, 1.018818208]|true|
|merge-54x1-dominant-w32|1.003262545|[0.993557849, 1.005822119]|14|true|[0.992787303, 1.005612640]|true|
|merge-54x1-short-tail-w32|0.927268829|[0.919077012, 0.940508990]|31|true|[0.997358133, 1.008304315]|true|

|Original-eight dominant aggregate versus|Ratio|Bounds|Wins /31|Check|
|---|---:|---|---:|---|
|OFF|1.000623238|[0.997951791, 1.004992662]|14|false|
|FROZEN|1.000744035|[0.997881824, 1.004042098]|11|false|

Primary dominant-200/32 and both equal-log original-eight dominant-caller aggregates require upper ratios below one and at least 21/31 wins against both controls. Every other case, including all tail suites, requires upper ON/control ratios ≤1.02. All 22 OFF/FROZEN calibration intervals must fit factor 1.03. Sorted ranks 10–22 of 31 have nominal 97.055% individual median coverage under independent/exchangeable blocks, without a familywise guarantee. There is no gain floor, exclusion, extension, threshold change, favorable subset or unchanged reroll. A pass permits a distinct accepted/OFF/ON full-SDK experiment and further resource/portable/final-artifact qualification; it is not adoption.

## Counters and automatic profiles

All latency precedes eight FIFO-gated four-event counter panels and eight fixed-period cycles:u profiles: primary dominant-200 at 16/32 workers, merge54 dominant-200/32 and primary short-tail/32, OFF/ON. Each profile has 65,536 rounds after primes. All captures precede owner transfer, decode, event/loss/mapping audits and analysis. No independent owned compiler/helper/hash/archive/analyzer workload overlaps capture.

Counter values are per executed batch across workers, with ≥99.9% event running coverage required. Each panel is one diagnostic observation, not a repeated-trial confidence interval or isolated cause.

|Panel|Instructions/batch|Cycles/batch|Qualified|
|---|---:|---:|---|
|stat-1x2-dominant-w16-OFF|123397.848|44559.916|true|
|stat-1x2-dominant-w16-ON|122651.848|45173.840|true|
|stat-1x2-dominant-w32-OFF|123397.868|63587.775|true|
|stat-1x2-dominant-w32-ON|122651.883|64275.115|true|
|stat-1x2-short-tail-w32-OFF|31898.091|16313.223|true|
|stat-1x2-short-tail-w32-ON|31514.593|15397.181|true|
|stat-merge-54x1-dominant-w32-OFF|108903.136|59917.120|true|
|stat-merge-54x1-dominant-w32-ON|109437.156|59943.677|true|

Profiles use a fixed period of 16 million, explicit CPU/time/period, executable/task mappings, one PID, zero lost records and worker-TID/CPU checks. Attribution requires ≥1,000 samples and ≤5% unknown period. The prior absolute-period accounting deficit remains unresolved; shares are qualitative, not exclusive costs, local CPI or removable proof wall time. Helper-name context is not an exclusive timing measurement, and zero samples do not establish zero execution.

|Profile|Samples|Unknown share|Cold-helper context share|Attribution qualified|Lost|
|---|---:|---:|---:|---|---:|
|cycles-1x2-dominant-w16-OFF|11672|0.021419|0.000000|true|0|
|cycles-1x2-dominant-w16-ON|11838|0.025511|0.000000|true|0|
|cycles-1x2-dominant-w32-OFF|33292|0.025412|0.000000|true|0|
|cycles-1x2-dominant-w32-ON|33664|0.023556|0.000000|true|0|
|cycles-1x2-short-tail-w32-OFF|8525|0.006569|0.000000|true|0|
|cycles-1x2-short-tail-w32-ON|8084|0.006061|0.567170|true|0|
|cycles-merge-54x1-dominant-w32-OFF|31350|0.028389|0.000000|true|0|
|cycles-merge-54x1-dominant-w32-ON|31379|0.027853|0.000000|true|0|

The three parent functions shrink by approximately 43%, but that static change does not produce a primary caller gain. In the primary32 counter panels, instructions fall from 123,397.868 to 122,651.883 per batch (about 0.605%), while cycles rise from 63,587.775 to 64,275.115 (about 1.081%). These single diagnostic panels do not isolate instruction-cache, register-allocation, branch or frequency effects. Zero full-batch cold-helper samples do not explain the regression or prove zero helper execution. The positive short-tail result is consistent with a different code-generation tradeoff in that context; no causal mechanism is established.

All eight counter panels and all eight profile-attribution panels pass their declared checks. All original helper hashes remain unchanged. The original finish and publisher complete on their first executions; no offline analyzer repair or capture repeat is needed. [Offline publication notes](artifacts/offline-report-notes.json).

## Records and limitations

All 2,129 workflow steps complete: 27 all-fixture preflights, 2,046 latency processes, 16 diagnostic processes and 40 postcapture ownership/audit steps. Total **1,059,202,848 actual kernel calls**, including primes/profiles. No new SDK request, proof export or Gnark verification occurs; the canonical distinct-proof ledger remains **23,971**. All original pins close. Native tests, captures and oracles are not repeated.

Archives retain every command/stdout/stderr/exit receipt, source byte and derived observation, with byte-exact gzip reconstruction checks. The observations archive has one `rows.json` member; archive records store text bytes with their original hash and length. Raw perf/disassemblies and raw coordinates remain local with hashes; coordinates are not copied into this report.

[Full analysis](artifacts/analysis.json), [all observations archive](artifacts/rows.json.gz), [source recovery](artifacts/source-audit.json), [static extraction checks](artifacts/static-review.json), [exact helpers](artifacts/g2_tail.rs), [small-stack test](artifacts/stack-test.rs), [complete private source](artifacts/source.json.gz), [command receipts](artifacts/receipts.json.gz), [archive review](artifacts/archive-review.json), [capture plan](artifacts/plan.json.gz), [job definitions](artifacts/jobs.json.gz). Plan and job files retain their exact original bytes after gzip decompression; the original pinned publisher is unchanged.

Caller RSS/capacities are retained in analysis and establish neither prover memory nor minimum RAM. No new AWS/Aeglos, ARM/mobile/TEE, forester250/4999, swap/pressure or concurrent-throughput qualification follows.

## CPU and Metal comparison

Accepted local measurements and supplied historical M5 references below are inherited, not candidate proof timings. All eight CPU 10× targets remain unmet. Accepted local 1×2 is 8.022165 ms / 134 MiB; AWS 1×2 remains Zopro 11.915591 ms at 32 workers versus pinned Aeglos Sorted 7.959184 ms at 96. The accepted physical-16 calibration engineering exception remains recorded and is not inherited by this gate.

|Shape|Accepted local ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|37.0|4.612×|24.0|2.992×|3.7|
|2x2|11.335969|59.0|5.205×|30.0|2.646×|5.9|
|4x2|15.945544|109.0|6.836×|48.0|3.010×|10.9|
|8x2|24.090485|189.0|7.845×|74.0|3.072×|18.9|
|12x2|33.844855|284.0|8.391×|101.0|2.984×|28.4|
|16x2|39.931557|335.0|8.389×|124.0|3.105×|33.5|
|24x2|66.233309|528.0|7.972×|176.0|2.657×|52.8|
|merge-24x1|65.273224|507.0|7.767×|175.0|2.681×|50.7|

Accepted 54→1 remains 162.250464 ms / 1831.87 MiB with no supplied M5 reference. [Current summary](../2026-10-29-current-cpu-summary/README.md), [dated consolidated report](../2026-10-30-cpu-benchmark-report/README.md).
