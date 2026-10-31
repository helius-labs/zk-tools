# G1 denominator tree with anchor-slot reuse

The candidate **fails the declared caller screen and is not adopted**. At 32 workers the two-shape ON/OFF estimate is 0.81% slower, bounds spanning parity, with 4/15 winning blocks. Against the original build it is 0.31% slower, with bounds above parity and 3/15 wins. The required aggregate improvements are absent. Production remains unchanged; no whole-proof speedup is inferred.

Positive results remain recorded: solo improves 1.33% on 1x2 and 1.77% on 2x2 against OFF, each with 15/15 wins. At 16 workers, 2x2 improves 2.45% against OFF with 12/15 wins; primary's positive estimate has bounds spanning parity. A separate 2x2/16 control calibration and ON/original nonregression fail. All cases and blocks are retained, with no unchanged rerun, favorable-subset promotion or layout rescue.

## Algorithm and prior work

The original G1 stream scales signed dy by a serial denominator prefix and uses a reverse inverse recurrence. The candidate stores raw signed dy and builds a balanced denominator-product heap. Original AX record slots hold internal products; denominator slots hold leaves and later their inverses. After inverting the root with the original inverse bundle and scalar-tail code, downward propagation recovers independent leaf inverses. Finishing rereads and packs each bucket's original X, then uses the original slope/X/Y formulas.

For m full eight-lane groups, the prefix/numerator schedule changes from 4m vector products to 3(m−1)+m. At m=25/44, the reduction is three vector products per batch. Product dependencies become logarithmic, but reread packing, tree indexing, loads/stores and the loss of the original fused signed-dy/prefix preparation add work. Short, noncanonical or nonvector cases use the original stream. This preserves the original finite-point, nonzero-denominator and distinct-destination contract. Identity, equal-X, doubling, cancellation, queues, batch size, signed Booth digits and final normalization are handled by the unchanged feeder.

The [prior G2 denominator-tree experiment](../2026-10-31-g2-denominator-tree176/README.md) remains rejected. Its indexing is adapted here; its measurements are not rerun or relabeled. The G1-specific trade concerns the raw numerator, five-vector records, reused AX storage and anchor rereading. Earlier G1 multi-chain, prefix-only, unit-prefix, point-tree, column, triple and field variants remain closed. This is not a novelty claim or evidence of a twofold proof improvement.

There is no new scratch vector, input-sized buffer, key-point copy or table, solver plan, worker pool, arbitrary memory cap or C/assembly rewrite. Existing group-record unsafe writes are copied unchanged; no additional unsafe operation is introduced. Equal record capacities do not establish equal process RAM or stack usage.

Two private snapshots have 178 files each: 173 originals exact, three append-only parents and two children. Reversing parent suffixes recovers all 352 originals. Removing the feeder's backend-call substitution recovers the original Window. The scalar-tail/inverse body is exact after the documented compile-time signed substitution. All 36 field-member copies are exact; 35 field tests are inherited and zero fresh field tests are claimed. Generated field sources remain unchanged.

## Complete-filler latency

Ryzen 9950X, 16 physical/32 logical CPUs. Separate original control, same candidate executable OFF/ON. Actual final links use opt-level 3, target-cpu=native, fat LTO and one codegen unit. Both original streams and the new G1 kernel contain IFMA.

Inherited real raw pre-group A inputs have 21,006 and 36,637 rows. A timed pass fills all 21 ordinary width-12 windows, including allocation, feed, queues, tree/finishing, normalization and destruction. Narrow-top special handling, weighted sums, final MSM combination, solving, proof construction and verification are excluded. These raw populations are not necessarily the final grouped queries used in production.

Before any preflight or timing, the provisional design's 20 passes per latency process was revised to **20 solo / 200 threaded**, identically for both shapes and all arms. Earlier threaded 20-pass processes lasted about 20 ms. The change sought longer observations; it is not a validation of detector power or a claim that all noise was removed. Provisional helper sources and the separate revision are retained. Blocks, controls, compiler/runtime source and every acceptance gate stay unchanged. No measured candidate result informed this duration change.

15 fixed rotated/reversed blocks, three checked primes and one checked post per caller. Whole-process affinity CPU 2 solo, 0–15 physical, 0–31 logical; Rayon 512 KiB stacks, migration permitted within each mask. All latency precedes PMUs/profiles. No owned build, analyzer, helper, hash/archive or second workload overlaps capture; exclusive host use is not established.

Ratios below are candidate/control; lower is faster. Times are medians of process averages. Paired ratios need not equal ratios of displayed medians.

Two-shape 32-worker ON/frozen: **1.003149 [1.000780, 1.005930], 3/15**.

Two-shape 32-worker ON/OFF: **1.008060 [0.999553, 1.012526], 4/15**.

|Raw context|Original / OFF / ON ms per complete fill pass|ON/OFF [bounds], wins|ON/original [bounds], wins|OFF/original calibration|
|---|---:|---|---|---|
|1x2-w1|10.613665 / 10.674286 / 10.498756|0.986688 [0.977546, 0.991818], 15/15|0.989978 [0.984548, 0.996169], 15/15|1.003864 [0.998567, 1.008739], 5/15|
|1x2-w16|1.076944 / 1.083893 / 1.076921|0.989793 [0.944788, 1.002986], 11/15|0.992606 [0.981133, 1.028302], 8/15|1.000251 [0.979463, 1.009115], 7/15|
|1x2-w32|0.874141 / 0.863761 / 0.875538|1.013506 [0.989717, 1.020382], 4/15|1.007629 [0.995216, 1.017582], 4/15|0.993040 [0.985921, 1.005692], 11/15|
|2x2-w1|16.775912 / 16.875415 / 16.544741|0.982286 [0.977931, 0.985662], 15/15|0.985229 [0.980599, 0.991307], 15/15|1.002635 [0.999267, 1.008912], 5/15|
|2x2-w16|1.700021 / 1.700001 / 1.667031|0.975487 [0.951104, 0.990900], 12/15|0.976804 [0.947420, 1.068970], 11/15|0.994805 [0.951678, 1.012897], 9/15|
|2x2-w32|1.374850 / 1.362175 / 1.368482|0.998482 [0.994383, 1.012643], 8/15|0.997075 [0.986543, 1.005196], 9/15|1.000540 [0.982759, 1.006655], 7/15|


Failed checks: `["2x2-w16/OFF/frozen/calibration","2x2-w16/ON/frozen/nonregression","two-shapes-w32/ON/frozen/gain","two-shapes-w32/ON/OFF/gain"]`.

All OFF/original calibration intervals must fit [1/1.03,1.03]; every ON/control upper bound must be <=1.03. Both equal-log two-shape 32-worker aggregates require upper <1 and >=10/15 wins. Ordered ranks 4..12 of 15 give nominal 96.484375% marginal coverage under independent exchangeable blocks, without familywise guarantees. No gain floor, exclusions, extensions, successful repeats, parameter sweep or changed gate. Failure establishes that the declared improvement was not shown; it does not isolate a hardware cause.

## CPU and memory diagnostics

|Context|Original / OFF / ON CPU ms/pass|Original / OFF / ON max-RSS counter MiB (diagnostic)|Original / OFF / ON minor faults/pass|
|---|---:|---:|---:|
|1x2-w1|10.614 / 10.674 / 10.499|14.25 / 14.25 / 14.25|0.15 / 0.15 / 0.15|
|1x2-w16|11.972 / 11.971 / 11.855|18.50 / 18.50 / 18.75|1.89 / 1.86 / 1.71|
|1x2-w32|16.193 / 16.206 / 16.347|20.00 / 20.25 / 19.75|2.71 / 2.88 / 2.54|
|2x2-w1|16.776 / 16.875 / 16.545|17.00 / 17.00 / 17.00|0.10 / 0.10 / 0.10|
|2x2-w16|18.419 / 18.437 / 18.071|20.50 / 20.75 / 20.50|1.63 / 1.55 / 1.61|
|2x2-w32|24.295 / 24.321 / 24.259|18.75 / 20.25 / 20.00|2.62 / 2.47 / 2.15|


Primary 32-worker CPU service increases about 0.87% versus OFF; 2x2 decreases about 0.26%. At 16 workers CPU decreases about 0.96% and 1.98%, respectively. These are measured caller comparisons, not proof-time gains. The same five-vector group record and original tail vector remain; packing AX again uses transient values, not a retained point/key copy. No new input-sized buffer is allocated.

RSS counters are diagnostic because previous work found undercounts. They are not precise lifetime peaks, minimum RAM or universal pressure bounds. No new smaps, stack-peak, phone, ARM/TEE, concurrent-proof, 250/4999, memory-pressure or swap qualification occurs here. Minor faults are counts, not their elapsed cost or evidence of disk paging. Longer latency processes also change the amortization of process-level effects, so fault rates should not be compared casually with prior short processes. Major faults are not exported by the driver.

## Counters and profiles

|Stat panel|Cycles M/pass|Instructions M/pass|Generic cache misses M/pass|Branch misses M/pass|Profile samples|Unknown share|
|---|---:|---:|---:|---:|---:|---:|
|stat-1x2-w16-OFF|60.398|147.616|0.106988|0.114441|676|0.5917%|
|stat-1x2-w16-ON|59.039|155.299|0.102187|0.110886|665|0.7519%|
|stat-1x2-w16-frozen|59.694|147.596|0.106951|0.118010|676|0.7396%|
|stat-1x2-w32-OFF|76.465|151.665|0.112616|0.124869|857|0.1167%|
|stat-1x2-w32-ON|76.347|159.204|0.114673|0.126009|782|0.6394%|
|stat-1x2-w32-frozen|76.168|151.499|0.111819|0.123466|786|0.1272%|

|Workers|Descriptive ON/OFF cycles ratio|ON/OFF instructions ratio|
|---|---:|---:|
|16|0.977514|1.052046|
|32|0.998458|1.049707|


All six grouped panels have 100% running time. In the single primary panels, ON/OFF instructions rise about 5.20% at 16 workers and 4.97% at 32. Cycles decrease about 2.25% and 0.15%, respectively. The shorter dependency schedule can accompany better solo timing while executing more instructions. These observations do not establish that one factor caused the wall-time results, or that mathematical dependency depth equals CPU critical-path time. No cache-level, DRAM-bandwidth, local-CPI, scheduler or dynamic-spill attribution is made.

Six FIFO-gated cycles:u recordings contain 4,442 samples, zero LOST, 665–857 per profile and unknown share at most 0.752%. All meet the declared 400-sample floor, but all are below 1,000 and provide thin descriptive context. Expected PID, binary mappings, task/FORK records, CPU masks and fixed 16M period are checked. Full symbol bins remain available; no absolute-period coverage or removable proof-wall share is claimed. Primes precede enable and checks follow disable.

The new apply symbol is 26,359 bytes; the original FAST signed stream is 26,179 bytes in both builds. Static code size is not executed instruction count, total stack usage or cache cost. Thirty selected linked contexts are archived.

## Validation and provenance

All 128 core tests pass in the first native execution: 125 original plus three new tests. Heap sizes 0..65 check root products and every leaf inverse against independent scalar arithmetic, including irregular trees. Signed generator-batch tests cover 25 lengths through 512, scalar tails and three changing reuse passes. Full queue/exception windows match generator-coefficient references in 1/4/16/32-worker pools with 512 KiB stacks. Field qualification is inherited by exact source identity; no standalone prover or portable suite is counted. Optional original external-fixture tests are not proof of absent-fixture cases.

The native suite reports 35.42 s, not a benchmark claim. Both final driver builds then succeed. A generic Borrow bound was retained before qualification so the original scalar-tail body remains unchanged. The archived setup already incorporates that bound; its prequalification adaptation helper is historical and should not be rerun after the current setup. There is no failed native build/test/capture or successful rerun in this study. Helper compile warnings for unused import/mutability are retained; they are not test failures.

All 330 capture commands complete once: 18 preflights, 270 latency processes, six PMU panels, six profiles and 30 ownership/decode/events/loss commands. Across 300 callers, 41,418 complete passes include 869,778 window fills and 51,609,600 checked prime/post bucket outputs. The latency subset times 37,800 passes. All real outputs match the existing independently generated Arkworks oracles; four original input/oracle hashes match their prior pins. No new solve or independent oracle generation is counted.

The independent reviewer passes on its first execution: 482 pins, original-source recovery, every command/exit/stream hash, raw caller values, per-context timing/CPU/fault/RSS, paired ratios, aggregate gates and every profile bin/ownership check. Floating serialization comparisons use the inherited 1e-12 tolerance; integer gate decisions are exact. Review does not rerun successful tests, captures or checks. Five archives verify exact member sets and decompressed file hashes. Executables, libraries, raw perf recordings and private inputs remain local with hashes.

No SDK proof, independent Gnark export or ledger advance occurs; ledger remains 28,847. No production source or accepted SDK changes. The all-eight target remains active and unmet.

## Inherited whole-proof CPU and Metal comparison

Accepted local timings and supplied M5 references below are inherited, not new candidate proof measurements. The accepted physical-16 calibration engineering exception and prior false full-performance gate remain recorded. No supplied shape reaches CPU 10× yet.

|Shape|Accepted whole-proof ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.928660|37.0|4.667×|24.0|3.027×|3.7|
|2x2|11.151932|59.0|5.291×|30.0|2.690×|5.9|
|4x2|16.086386|109.0|6.776×|48.0|2.984×|10.9|
|8x2|24.179453|189.0|7.817×|74.0|3.060×|18.9|
|12x2|33.884455|284.0|8.381×|101.0|2.981×|28.4|
|16x2|40.077381|335.0|8.359×|124.0|3.094×|33.5|
|24x2|65.850777|528.0|8.018×|176.0|2.673×|52.8|
|merge-24x1|65.060937|507.0|7.793×|175.0|2.690×|50.7|


Accepted merge54 remains 161.883899 ms without a supplied M5 reference. Latest full-SDK candidate evidence remains the separate rejected weighted-sdk176 report. Latest AWS evidence is historical source175 on c7a.24xlarge: Zopro 11.915591 ms at 32 workers versus Aeglos 7.959184 ms at 96. No new AWS resource, AWS/Aeglos comparison or proof-memory peak is claimed here.

[Design](artifacts/design.md), [final capture protocol](artifacts/capture-protocol.md), [results](artifacts/analysis.json), [independent review](artifacts/review.json), [source audit](artifacts/audit.json), [codegen](artifacts/codegen.json), [profile bins](artifacts/profile-symbols.json), [fixture bindings](artifacts/fixture-bindings.json), [runtime sources](artifacts/source.tar.gz), [workflow and helpers](artifacts/workflow.tar.gz), [all receipts](artifacts/receipts.tar.gz), [PMU CSVs](artifacts/counters.tar.gz), [linked assembly](artifacts/codegen.tar.gz), [archive checks](artifacts/publication.json), [checksums](SHA256SUMS).
