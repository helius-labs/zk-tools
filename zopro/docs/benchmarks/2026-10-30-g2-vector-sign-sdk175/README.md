# Vectorized G2 signs in full SDK proofs

The declared full-SDK gate is **false**; diagnostics are **true**. Failed checks: `["1x2-w16/OFF/accepted"]`. Production remains unchanged. These measurements include solving, fresh randomized proving, internal verification and serialization. The passed [complete-query trial](../2026-10-30-g2-vector-sign175/README.md) remains separate evidence.

The candidate improves primary 32-worker latency to **7.887963 ms**, a paired **1.92% gain** versus accepted production, winning all 31 blocks. Its paired ratio is 0.980799995, with bounds [0.979909727, 0.985355230]. All candidate latency and memory checks pass, including the original-eight aggregate and the 16-worker comparison. The sole failed check is the fresh OFF control's 16-worker calibration against the accepted binary: ratio 1.005104955, with bounds [0.985550576, 1.036044283]. That interval exceeds the declared 3% calibration range. It does not show a candidate regression, but the full qualification gate remains failed; this report does not waive or change it.

The candidate packs original incoming G2 Y coordinates and applies eight signs while subtracting the bucket anchor in SIMD. This removes per-point scalar negation before packing. Positive lanes form p+by−ay; negative lanes form2p−by−ay, with negative-zero correction before the value escapes. The exact passed field helper and callsite are retained. Field multiplication/reduction bodies, norms, prefix/inversion, scratch, tails, grouping, geometry, solver, proof distribution, scheduler and key representation stay unchanged. No heap buffer, per-key table, copied persistent point array or second plan is added. No change is adopted from an isolated arithmetic count.

The pure SDK source removes only the diagnostic query export from the parent. It contains176files:172of175 originals exact, three reversible parents and one new field child; all175 recover. Sixteen original field files remain exact. Parent33/35 field tests, including two independent Arkworks tests, are inherited after exact helper/source checks. Fresh native OFF/ON pass124/125core and eight prover tests each; noIFMA portable candidate passes101core/eight prover. The new core case covers every eight-lane sign mask, tails, reuse and1/4/16/32 pools with512KiB stacks. These checks do not establish mobile, huge-circuit or memory-pressure qualification. The cfg include in generated ifma8.rs still needs generator-consistency/final-artifact qualification if this gate passes. [Pure-source review](artifacts/final-source-review.json).

## Full proof comparison

Ryzen9950X,16physical/32logical CPUs,128GiB RAM. All nine shapes use32 workers onCPUs0–31; primary also uses16 workers onCPUs0–15. Exact accepted SDK20fde, fresh OFF and ON use identical fixtures and runtime policy. Thirty-one fixed rotated/reversed blocks;151requests after3primes for primary,31after3 for other shapes. Loading/preparation is outside latency and inside lifetime peak RSS. Every request solves and produces a fresh randomized verified/serialized proof. No owned build/helper/hash/archive/analyzer or second workload overlaps captures.

|Case|Accepted ms|OFF ms|ON ms|Paired ON/OFF [bounds]|Wins/31|
|---|---:|---:|---:|---|---:|
|1x2-w32|8.037767|8.081259|7.887963|0.977547973 [0.973839999, 0.981044782]|31|
|2x2-w32|11.292426|11.330254|11.157129|0.983605126 [0.980383571, 0.990532649]|30|
|4x2-w32|16.032120|16.132934|16.055974|0.999161751 [0.992775927, 1.009208039]|18|
|8x2-w32|24.108688|24.225798|24.206045|1.001500519 [0.991726076, 1.009902066]|15|
|12x2-w32|33.880905|33.968638|33.770306|0.996882360 [0.988043911, 1.000160990]|21|
|16x2-w32|40.098195|40.170942|40.063214|0.995942254 [0.990706472, 1.001659965]|20|
|24x2-w32|66.178195|66.137069|66.218548|1.002372017 [0.992850645, 1.004215717]|15|
|merge-24x1-w32|65.341462|65.383867|65.295067|0.997725148 [0.993834807, 1.002395753]|19|
|merge-54x1-w32|161.608820|162.012725|161.880641|0.997334048 [0.991149201, 1.004579431]|17|
|1x2-w16|9.694021|9.751337|9.510192|0.974762755 [0.952363938, 0.992665743]|23|

Primary32 and the equal-log original-eight aggregate require upper paired ratios<1 and at least21/31wins against BOTHaccepted and OFF. All other contexts, including primary16 and54→1, require upper<=1.02. All10 OFF/accepted calibration intervals must fit factor1.03. Every case's maximum ON process RSS must be<=1.05of both controls. Ranks10–22/31 have nominal97.055% individual median coverage conditional on exchangeable blocks, without a familywise guarantee. No minimum gain floor, exclusions, extension, changed threshold, unchanged rerun or favorable-subset promotion. The accepted historical16-worker exception does not transfer. Marginal medians need not reproduce paired ratios.

|Original-eight aggregate versus|Ratio [bounds]|Wins/31|Passed|
|---|---|---:|---|
|accepted|0.996264886 [0.995218390, 0.998101851]|26|true|
|OFF|0.993757820 [0.990590765, 0.995537830]|30|true|

## CPU and memory

Process RSS includes loading/primes. It is an observed high-water mark, not minimum RAM or a bound under pressure. No new heap/scratch design does not imply unchanged RSS. The parent query's primary32 RSS increased32→37.75MiB; these full-proof observations are measured separately.

|Case|CPU ms/request accepted/OFF/ON|Maximum process RSS MiB accepted/OFF/ON|
|---|---|---|
|1x2-w32|181.908/182.047/178.887|134.00/135.00/133.00|
|2x2-w32|262.265/262.069/258.430|273.00/279.25/275.00|
|4x2-w32|421.886/422.426/420.532|318.00/317.75/327.25|
|8x2-w32|667.320/667.317/670.738|512.00/517.50/510.00|
|12x2-w32|938.550/941.444/938.370|712.25/731.00/721.25|
|16x2-w32|1125.051/1127.966/1123.921|847.25/837.00/837.75|
|24x2-w32|1895.115/1891.957/1893.666|1290.12/1301.00/1314.00|
|merge-24x1-w32|1815.012/1813.204/1808.905|1198.50/1173.75/1213.36|
|merge-54x1-w32|4434.231/4445.599/4447.580|1865.01/1872.66/1869.20|
|1x2-w16|119.799/119.910/116.842|92.00/91.00/91.00|

## Supplied M5 CPU and Metal references

Historical user-supplied M5 timings, not matched-machine Apple measurements. The ON rows below are this unadopted candidate; the accepted production rows remain preserved separately. Completing the original all-eight10xCPU objective still requires a fully qualified adopted artifact.

|Shape|ON ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10x target ms|Target reached by ON timing|
|---|---:|---:|---:|---:|---|
|1x2|7.887963|37 / 4.691×|24 / 3.043×|3.7|false|
|2x2|11.157129|59 / 5.288×|30 / 2.689×|5.9|false|
|4x2|16.055974|109 / 6.789×|48 / 2.990×|10.9|false|
|8x2|24.206045|189 / 7.808×|74 / 3.057×|18.9|false|
|12x2|33.770306|284 / 8.410×|101 / 2.991×|28.4|false|
|16x2|40.063214|335 / 8.362×|124 / 3.095×|33.5|false|
|24x2|66.218548|528 / 7.974×|176 / 2.658×|52.8|false|
|merge-24x1|65.295067|507 / 7.765×|175 / 2.680×|50.7|false|

Candidate timing targets reached: **0/8**, without an adoption claim.54→1 has no supplied M5 reference. All exact accepted local/AWS timings and RSS remain in [accepted rows](accepted-rows.json). Accepted production primary remains8.022165ms/134MiB, with0/8CPU10x targets; the documented historical physical16 calibration exception and separate final-artifact qualification remain unchanged. Latest retained C7a primary is Zopro11.915591ms/32workers versus Aeglos7.959184ms/96workers, best tested configurations in that campaign. No new AWS comparison.

## Verification and profiles

All30preflight exports independently pass Gnark before latency. Remaining latency/diagnostic verification follows every performance capture. The campaign makes55,788internally verified SDK requests and exports972fresh164-byte proofs, each independently verified and bound to its exact image/public input/key. Every proof hash is distinct from the prior25,231 and each other; the ledger advances to26,203 only after these checks. All1,944proof/public members are retained. Sources, commands, outputs and hashes close.

Six FIFO-gated four-event panels31+3requests and six separate fixed16Mcycles:u profiles251+3requests cover primary16/32 all3arms after latency. All recordings finish before verification/decode/archive. Counters require>=99.9% running. Profiles require>=1000samples, unknown<=5%, sole binary-owner PID, recorded thread membership, CPUs inside the requested whole-process set, fixed period/mappings/time and zero lost records. Workers inherit the allowed CPU set; they are not each pinned to one numbered CPU. The inline helper has no exclusive symbol share. Absolute period coverage, CPI/stalls and removable proof-wall fractions remain unvalidated.

|Counter panel|Instructions/request M|Cycles/request M|Qualified|
|---|---:|---:|---|
|stat-1x2-w16-OFF|1381.856|587.540|true|
|stat-1x2-w16-ON|1348.029|564.414|true|
|stat-1x2-w16-accepted|1381.121|582.598|true|
|stat-1x2-w32-OFF|1419.515|824.608|true|
|stat-1x2-w32-ON|1389.961|808.376|true|
|stat-1x2-w32-accepted|1421.406|815.971|true|

|Profile|Samples|Unknown period share|Qualified|
|---|---:|---:|---|
|cycles-1x2-w16-OFF|9023|0.014629|true|
|cycles-1x2-w16-ON|8784|0.016052|true|
|cycles-1x2-w16-accepted|8957|0.012616|true|
|cycles-1x2-w32-OFF|12489|0.016975|true|
|cycles-1x2-w32-ON|12193|0.015091|true|
|cycles-1x2-w32-accepted|12491|0.016572|true|

All1,974capture/verification/postprocessing steps and12native steps finish once:1,986successful commands. Three metadata parser tests pass before capture. Original measurement/analyzer/publisher helpers are compiled and pinned beforehand. The publisher independently recomputes32paired distributions and64gate decisions from raw SAMPLE/usage records, checks all command streams, source/helper pins, unique independently verified exports and byte-exact archive round-trips. Float serialization consistency checks use relative1e-12; performance thresholds remain unchanged. No successful native test, proof capture or verification is repeated.

This trial adds no phone/ARM/TEE, huge forester250/4999, memory-pressure/swap/minimumRAM or concurrent-throughput qualification. A passing SDK gate selects resource/fallback/generator/final-artifact work before adoption; it is not completion of the original latency goal. Raw perf/executables/libraries remain local with hashes. Full source/helpers/receipts/proofs/observations/symbol aggregates are archived.

[Protocol](artifacts/design.md), [all results](artifacts/analysis.json), [independent review](artifacts/review.json), [observations](artifacts/observations.json.gz), [source](artifacts/source.json.gz), [helpers](artifacts/helpers.json.gz), [receipts](artifacts/receipts.json.gz), [proofs](artifacts/proofs.json.gz), [symbols](artifacts/symbols.json.gz), [archive review](artifacts/archive-review.json), [checksums](SHA256SUMS).
