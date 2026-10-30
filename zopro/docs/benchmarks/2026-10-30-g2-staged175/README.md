# Bounded operator stages in G2 reverse arithmetic

The candidate is rejected. Paired G2 query latency increases 4.58% with one worker, 8.17% with 16 workers, and 7.47% with 32 workers on 1×2; 2×2/32 increases 6.46%. It wins none of the 31 blocks in any context. All four control calibrations and the diagnostic checks pass. Instruction counts also increase about 2.3–2.6% in the threaded panels. These observations do not isolate a hardware cause. No full-proof improvement or new AWS measurement follows from this experiment.

Complete-query gate: **false**. Diagnostics: **true**. Eligible for a separate full SDK trial: **false**. Production remains unchanged; these are G2 query timings, not proof timings. Failed checks: `["1x2-w1/ON/OFF","1x2-w1/ON/FROZEN","1x2-w16/ON/OFF","1x2-w16/ON/FROZEN","1x2-w32/ON/OFF","1x2-w32/ON/FROZEN","2x2-w32/ON/OFF","2x2-w32/ON/FROZEN","two_shapes/ON/OFF/gain","two_shapes/ON/FROZEN/gain"]`.

The candidate evaluates unchanged reverse arithmetic in short operator loops across a fixed stack tile. Preparation packs original coordinates and computes the original reverse inverse recurrence. Separate noinline loops then compute slopes, new X coordinates, and Y/scatter. Calls span several SIMD groups instead of outlining one product at a time. All point operands, formulas, signs, window/partition geometry, grouping, scalar tails and forward norm/prefix construction remain original. This changes execution and live ranges, not the number of field products.

The tile occupies at most the existing 200-pair norm/prefix payload: four records, 14,336 bytes on this native layout, versus that payload's 16,000 bytes. This is additional bounded stack storage, not reuse of the prefix array or a whole-process memory bound. It adds no heap buffer, point-key table, copied persistent point array, plan or pool. Input points remain immutable. One local unsafe slice covers only fully initialized records in MaybeUninit storage; no trailing element is read and no reference escapes. Stack/RSS/pressure and unsupported-device behavior require further qualification before adoption.

Earlier owned-batch X/Y splitting, phased inverse materialization and single-vector fused-Fq outlining remain rejected under their original gates. This recipe changes several reverse stages over a bounded immutable-input tile; it does not retest those implementations or sweep their parameters. [History and structural research](artifacts/history-audit.md).

## Correctness and source

Fresh OFF and ON each pass all 126 native core and eight prover tests. Two new cases cover owned and signed borrowed batches, empty/sub-vector/full/partial tiles, 18 lengths through 513, both subtraction and inverse modes, source preservation, three-call reuse and independent complete generator sums. Concurrent pools at 1/4/16/32 workers use 512 KiB stacks. Runtime test hits require the new route under ON. Existing complete exceptional-point and MSM tests remain in the full suite; direct prefix tests retain their finite/nonzero-denominator input contract.

The private snapshot has 176 files: 171 of 175 originals are exact, four parent edits are reversible and one child is new. All 17 field sources remain exact; their 33-test qualification is inherited, not rerun. No field formula or representation changes. Reversing the edits recovers all accepted files. Original API-only FROZEN and its 124 core/eight prover qualification are reused after byte/flag/source checks. New standard SDK consumers link but are not executed here.

Two original fixtures and complete independent Arkworks MSM hashes are reused without a new solve, fixture generation or oracle call. Every preflight, prime and measured query agrees with the complete sum; point bytes and worker affinities remain checked. Raw point/scalar inputs and unblinded coordinates stay local. [Source audit](artifacts/source-audit.json), [exact source archive](artifacts/source.json.gz), [input reuse](artifacts/reused-inputs.md).

## Fixed complete-query comparison

Ryzen 9950X, 16 physical / 32 logical CPUs, shared host. Primary solo uses CPU 2; primary physical16 uses CPUs 0–15; primary32 and 2×2/32 use CPUs 0–31. Exact accepted API-only FROZEN, fresh OFF and ON share byte-identical driver/perf source. Each of 31 fixed rotated/reversed blocks runs 17 complete queries after three checked primes per fresh process. Grouping, every fill/flush/reduction/merge, reset copy, allocation/drop and final normalization are included; loading/parsing/witness solving are excluded. RSS is the process high-water maximum including loading/primes, not proof RSS or minimum RAM.

|Case|FROZEN ms|OFF ms|ON ms|ON/OFF [bounds]|Wins/31|
|---|---:|---:|---:|---|---:|
|1x2-w1|39.512235|40.480263|42.317556|1.045772821 [1.044626182, 1.046174069]|0|
|1x2-w16|3.716234|3.732295|4.027201|1.081653096 [1.068554131, 1.085596871]|0|
|1x2-w32|3.168281|3.197337|3.433853|1.074699392 [1.068089084, 1.081617988]|0|
|2x2-w32|3.632576|3.674726|3.882950|1.064608847 [1.051933124, 1.078711153]|0|

Primary32 and the equal-log two-shape32 aggregate require upper ON/control ratios below one and at least 21/31 wins versus both OFF and FROZEN. Other context upper ratios must be ≤1.02. All four OFF/FROZEN calibration intervals must fit factor 1.03. Ordered ranks 10–22/31 have nominal 97.055% individual coverage conditional on exchangeable blocks, without a familywise guarantee. No gain floor, excluded observation, sample extension, changed threshold, tile/annotation sweep or favorable subset promotion. Marginal medians need not reproduce paired ratios.

|Aggregate versus|Ratio [bounds]|Wins/31|Passed|
|---|---|---:|---|
|OFF|1.067959895 [1.059677540, 1.076251642]|0|false|
|FROZEN|1.075431095 [1.070243748, 1.082147625]|0|false|

## CPU service and query RSS

Summed CPU milliseconds per query include runtime work. RSS maxima cover fresh query processes and cannot be added to accepted proof RSS as an estimated increment. No minimum-RAM, memory-pressure or full-proof-memory saving is established.

|Case|CPU ms FROZEN/OFF/ON|Maximum RSS MiB FROZEN/OFF/ON|
|---|---|---|
|1x2-w1|39.511/40.478/42.313|15.25/15.25/15.25|
|1x2-w16|47.305/48.162/50.284|26.75/27.50/27.50|
|1x2-w32|72.921/73.110/77.428|32.50/32.50/32.00|
|2x2-w32|79.200/79.327/83.552|82.25/66.25/67.00|

## Fixed counters and profiles

All latency precedes six FIFO-gated four-event stat panels (31 queries) and six separate fixed 16M cycles:u profiles (251 queries), after three primes, at primary16/32 and 2×2/32 OFF/ON. Gates exclude loading/primes and output checks. All recordings finish before ownership transfer, decoding, hashing and archives. Every capture helper and original analyzer is compiled/pinned beforehand; no owned build/helper/hash/archive/analyzer or independent workload overlaps captures.

Counter running time must reach 99.9%. Each profile requires at least 1,000 samples, at most 5% unknown period, one matching PID, checked worker TID/CPU, time/mappings/fixed period and zero lost records. All three stage functions must have positive periods in every ON recording and none in OFF; static linked names are also checked before capture. Absolute period coverage remains unvalidated; flat shares and counters do not identify exclusive stalls, CPI or removable wall-time fractions.

|Panel|Instructions/query M|Cycles/query M|Qualified|
|---|---:|---:|---|
|stat-1x2-w16-OFF|582.692|240.442|true|
|stat-1x2-w16-ON|596.182|250.700|true|
|stat-1x2-w32-OFF|599.308|334.431|true|
|stat-1x2-w32-ON|614.825|348.624|true|
|stat-2x2-w32-OFF|629.384|363.808|true|
|stat-2x2-w32-ON|645.953|382.841|true|

|Profile|Samples|Unknown period share|Stage activation|Qualified|
|---|---:|---:|---|---|
|cycles-1x2-w16-OFF|3741|0.008821|true|true|
|cycles-1x2-w16-ON|3925|0.008408|true|true|
|cycles-1x2-w32-OFF|5238|0.008973|true|true|
|cycles-1x2-w32-ON|5490|0.010200|true|true|
|cycles-2x2-w32-OFF|5689|0.010195|true|true|
|cycles-2x2-w32-ON|5935|0.008593|true|true|

All 426 capture-workflow steps finish: 12 preflights, 372 latency processes, six stat panels, six profiles and 30 postcapture commands; 396 query processes and 9,216 complete queries including primes. Ten native and two query-link steps finish separately. Sources, helpers, flags, libraries, inputs and receipts close. Original raw stat CSVs, every command stream and full source round-trip byte-exactly in archives. Raw perf and executable/library files remain local with hashes. No successful test, old oracle or performance capture repeats.

A separate [postcapture check](artifacts/postcapture-review.json) recomputes all 14 paired distributions and gate decisions from integer query durations, verifies all 438 command receipts, 592 capture pins, 2,452 archive members, the source reversals, the accepted SDK hash and all nine inherited comparison rows. Its [source](artifacts/postcapture-review.rs) was compiled after capture; it adds no test, oracle or timing rerun. Floating-point serialization comparisons use a relative tolerance of 1e-12; the performance thresholds remain unchanged.

## Accepted CPU and Metal comparisons

Inherited accepted full-proof timings and supplied historical M5 references, unchanged by this query trial. The original all-eight CPU 10× goal remains active and unmet, 0/8. Primary accepted peak RSS remains 134 MiB. The historical physical16 calibration engineering exception and separately qualified final artifact remain documented.

|Shape|Accepted ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|8.022165|37 / 4.612×|24 / 2.992×|3.7|
|2x2|11.335969|59 / 5.205×|30 / 2.646×|5.9|
|4x2|15.945544|109 / 6.836×|48 / 3.010×|10.9|
|8x2|24.090485|189 / 7.845×|74 / 3.072×|18.9|
|12x2|33.844855|284 / 8.391×|101 / 2.984×|28.4|
|16x2|39.931557|335 / 8.389×|124 / 3.105×|33.5|
|24x2|66.233309|528 / 7.972×|176 / 2.657×|52.8|
|merge-24x1|65.273224|507 / 7.767×|175 / 2.681×|50.7|

Accepted 54→1 remains 162.250464 ms with no supplied M5 reference. Latest retained C7a primary remains Zopro 11.915591 ms at 32 workers versus Aeglos 7.959184 ms at 96, best tested configurations in that campaign; no new AWS result follows. [All nine exact local/AWS rows](accepted-rows.json). Ledger remains 25,231. No new solver, SDK request, Gnark export, phone/ARM/TEE, forester 250/4999, pressure/swap, minimum-RAM or concurrent-throughput result is established. A passing query gate selects separate all-nine full-SDK, independent-proof, resource/portable and final-artifact qualification before adoption.

[Protocol](artifacts/design.md), [analysis](artifacts/analysis.json), [all observations](artifacts/observations.json.gz), [profiles](artifacts/profile-symbols.json), [source](artifacts/source.json.gz), [helpers](artifacts/helpers.json.gz), [receipts](artifacts/receipts.json.gz), [archive review](artifacts/archive-review.json), [checksums](SHA256SUMS).
