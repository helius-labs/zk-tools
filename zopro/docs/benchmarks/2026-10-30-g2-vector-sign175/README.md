# Vector signs in borrowed G2 coordinate differences

The complete G2 query improves by a paired **5.36%** versus fresh OFF at32 workers on1×2 (30/31 wins) and **3.72%** on2×2 (29/31 wins). The two-shape aggregate improves **4.69%**, winning all31 blocks. The one-worker and16-worker contexts also improve. Every declared gain, nonregression, calibration and diagnostic check passes. This selects a separate full-SDK trial; it establishes no new proof latency or adoption.

Complete-query gate: **true**. Diagnostics: **true**. Eligible for a separate full SDK trial: **true**. Production remains unchanged; these are G2 query timings, not proof timings. Failed checks: `[]`.

The candidate packs original incoming Y coordinates and applies all eight incoming signs while subtracting bucket Y in SIMD. The accepted path negates each signed scalar Fq2 Y before packing. The new unscaled helper derives from the existing G1 signed-difference prelude and corrects negative zero before returning strict lazy values below2p. Positive lanes form p+by−ay; negative lanes form2p−by−ay. For by=0, the lazy representative may differ from the original by p, while canonical values and downstream products agree. No field multiplication/reduction body, norm, prefix chain, inverse, scratch record, scalar tail, point order, grouping, window geometry, pool or key representation changes. No new table, heap buffer or copied persistent points/plan. G1, owned G2 and unsupported paths retain their originals.

The older G1 endpoint-peeling, G2 signed raw-product and packed-subtraction experiments retain their failed gates. This changes incoming sign materialization in the actual current borrowed caller. It does not predict a twofold proof improvement. [History audit](artifacts/history-audit.md).

Fresh OFF/ON pass unfiltered33/35 field,124/125 core and eight prover tests each. Two new independent Arkworks field tests cover both primes, modulus/limb/full-width boundaries, all sign masks, strict lazy bounds, negative zero and following fused products. The added core test covers all256 eight-lane sign masks across12 lengths through64, three-call reuse and1/4/16/32-thread pools with512KiB stacks. Independent generator coefficients determine complete expected sums; input points stay immutable. Existing complete MSM/exception/tail tests remain included. Direct prefix tests retain their finite/nonzero-denominator input contract. This is native correctness, not portable or memory-pressure qualification.

The private snapshot has176 files:171/175 originals exact, four reversible parents and one new field child. All175 recover after removing the edits;16/17 original field sources are exact. Original API-only FROZEN reuses124core/eight-prover qualification after exact source/flags/fixture checks; its rejected queue-scan optimization is absent. Standard SDK consumers link but are not executed. No new fixture solve or oracle call is made. Every query agrees with the original independent complete Arkworks MSM hash. Raw points/scalars stay local. [Source audit](artifacts/source-audit.json), [full source](artifacts/source.json.gz), [reuse](artifacts/reused-inputs.md).

## Complete query comparison

Ryzen9950X,16physical/32logical CPUs. Primary solo usesCPU2, physical16 CPUs0–15, primary32 and2×2/32 CPUs0–31. Original FROZEN, fresh OFF and ON share byte-identical probe/perf sources. Each of31 fixed rotated/reversed blocks runs17 complete queries after three checked primes per fresh process. Scalar reset copy, grouping, all fill/flush/merge/reduction, allocation/drop and normalization are included. Parsing/loading/solver are excluded.

|Case|FROZEN ms|OFF ms|ON ms|Paired ON/OFF [bounds]|Wins/31|
|---|---:|---:|---:|---|---:|
|1x2-w1|39.483137|39.562035|37.182309|0.940266420 [0.939030954, 0.941558744]|31|
|1x2-w16|3.696769|3.690968|3.438272|0.928669788 [0.915948864, 0.939600379]|31|
|1x2-w32|3.178332|3.180326|3.014313|0.946438290 [0.935039495, 0.959110725]|30|
|2x2-w32|3.647006|3.650231|3.498446|0.962768827 [0.950853483, 0.967960157]|29|

Primary32 and the equal-log two-shape32 aggregate require upper ON/control ratios below1 and at least21/31wins versus both OFF and FROZEN. Other upper ratios must be<=1.02. All four OFF/FROZEN calibration intervals must fit factor1.03. Ordered ranks10–22/31 have nominal97.055% individual coverage conditional on exchangeable blocks, without a familywise guarantee. No gain floor, exclusion, extension, threshold change, unchanged rerun or subset promotion. Marginal medians need not reproduce paired ratios.

|Aggregate versus|Ratio [bounds]|Wins/31|Passed|
|---|---|---:|---|
|OFF|0.953063428 [0.948973346, 0.957252052]|31|true|
|FROZEN|0.954017117 [0.948083920, 0.959727501]|31|true|

|Case|CPU ms/query FROZEN/OFF/ON|Maximum process RSS MiB FROZEN/OFF/ON|
|---|---|---|
|1x2-w1|39.487/39.562/37.198|15.25/15.25/15.25|
|1x2-w16|47.207/47.279/44.746|27.00/28.25/26.75|
|1x2-w32|73.200/72.481/70.124|32.00/32.00/37.75|
|2x2-w32|79.262/78.389/75.992|67.00/66.75/66.75|

RSS includes loading/primes and is neither proof RSS nor minimum RAM. It cannot be added to accepted proof RSS as an estimated increment. Stack/admission/pressure behavior requires further qualification before adoption.

Primary32 query peak RSS rises from32.00 to37.75MiB in this run;2×2 remains66.75MiB versus OFF. The unchanged heap/scratch design does not establish unchanged process RSS. Full-proof resource measurements are still required.

## Counters, profiles and provenance

Six FIFO-gated four-event panels31queries and six separate fixed16M cycles:u profiles251queries follow all latency, with three primes, at primary16/32 and2×2/32 OFF/ON. Gates exclude setup/primes/output checks. All recordings finish before transfer/decode/hash/archive. No owned build/helper/analyzer/second workload overlaps capture; all original helpers are compiled and pinned beforehand.

Counters require>=99.9% running. Profiles require>=1000samples, unknown<=5%, matching PID/workerTID/CPU/time/mappings/fixed period and zeroLOST. The signed helper is inline; symbol shares cannot isolate it. Source/cfg/library provenance and changed actual borrowed-G2 machine code are checked before capture. Static code sizes/IFMA sites are not dynamic counts or exclusive costs. Absolute period coverage, CPI/stalls and removable wall fractions remain unvalidated.

|Panel|Instructions/query M|Cycles/query M|Qualified|
|---|---:|---:|---|
|stat-1x2-w16-OFF|582.270|234.860|true|
|stat-1x2-w16-ON|549.993|221.677|true|
|stat-1x2-w32-OFF|600.578|331.892|true|
|stat-1x2-w32-ON|567.763|317.934|true|
|stat-2x2-w32-OFF|629.534|359.883|true|
|stat-2x2-w32-ON|596.950|350.780|true|

|Profile|Samples|Unknown period share|Qualified|
|---|---:|---:|---|
|cycles-1x2-w16-OFF|3675|0.008163|true|
|cycles-1x2-w16-ON|3470|0.007493|true|
|cycles-1x2-w32-OFF|5169|0.009673|true|
|cycles-1x2-w32-ON|4968|0.009259|true|
|cycles-2x2-w32-OFF|5616|0.007301|true|
|cycles-2x2-w32-ON|5412|0.008500|true|

All426 capture steps finish:12preflights,372latency processes,six stat panels,six profiles and30 postcapture commands. There are396 query processes and9,216complete queries including primes. Twelve native steps and two links finish separately:440successful workflow commands. Every command stream and full source round-trip byte-exactly in archives. Raw perf and executables/libraries stay local with hashes. [Changed caller code](artifacts/linked-borrowed-code.json), [receipts](artifacts/receipts.json.gz).

The initial OFF field suite passed33 tests. OFF core compilation stopped before tests on an unused denominator declaration in the generated OFF-only scope whose array length was not inferable. The used denominator already existed outside that scope; only the unused declaration was removed. The first repair guard matched both forward and unused reverse declarations and stopped before editing source; the corrected guard selects only the unused declaration. Two fixed-workflow invocations stopped before commands because the plan was not yet present or its output directory already existed. Initial sources/plans/pins/failure/repair receipts remain. The successful field suite is reused exactly, with only the remaining11native steps executed and composed into12. No successful tests or performance captures repeat. [Repair record](artifacts/repair-note.md).

## Accepted CPU and Metal comparisons

Candidate field compilation also stopped before tests because the included child started with an inner doc comment. Changing only that comment to a regular comment fixed compilation; all successful OFF steps were retained, and the six remaining candidate steps completed separately. The final12-step native receipt composes the checked successes. [Second compile repair](artifacts/repair-v2-note.md).

The independently compiled reviewer verifies all14 paired distributions/gate decisions from raw integer query durations, all440 successful command streams,671 capture pins,2,533 archived members, source reversals, accepted SDK hash and all nine inherited comparison rows. It was compiled before capture and executed afterward; no test, oracle or timing rerun. [Review](artifacts/postcapture-review.json), [source](artifacts/postcapture-review.rs).

Inherited full-proof timings and supplied historical M5 references, unchanged. Original all-eight CPU10x goal remains active/unmet0/8. Primary accepted peak RSS remains134MiB. The historical physical16 calibration engineering exception and separately qualified accepted artifact remain documented.

|Shape|Accepted ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10x target ms|
|---|---:|---:|---:|---:|
|1x2|8.022165|37 / 4.612×|24 / 2.992×|3.7|
|2x2|11.335969|59 / 5.205×|30 / 2.646×|5.9|
|4x2|15.945544|109 / 6.836×|48 / 3.010×|10.9|
|8x2|24.090485|189 / 7.845×|74 / 3.072×|18.9|
|12x2|33.844855|284 / 8.391×|101 / 2.984×|28.4|
|16x2|39.931557|335 / 8.389×|124 / 3.105×|33.5|
|24x2|66.233309|528 / 7.972×|176 / 2.657×|52.8|
|merge-24x1|65.273224|507 / 7.767×|175 / 2.681×|50.7|

Accepted54→1 remains162.250464ms without suppliedM5 reference. Latest C7a primary remains Zopro11.915591ms/32workers versus Aeglos7.959184ms/96workers, best tested configurations in that campaign. No AWS result is refreshed. [Nine exact local/AWS rows](accepted-rows.json). Ledger25231 unchanged. No new SDK proof, solver call, phone/ARM/TEE, forester250/4999, pressure/swap/minimumRAM or concurrent-throughput result. A passing query trial selects separate all-nine SDK/independent-proof/resource/portable/final-artifact checks before adoption.

[Protocol](artifacts/design.md), [analysis](artifacts/analysis.json), [all observations](artifacts/observations.json.gz), [source](artifacts/source.json.gz), [helpers](artifacts/helpers.json.gz), [archive review](artifacts/archive-review.json), [checksums](SHA256SUMS).
