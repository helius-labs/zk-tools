# Point-bounded joint-top G2 marginal reduction

**Not qualified at the complete-query stage.** Primary32 ON/OFF ratio is 1.008493; 2×2/32 is 1.002113. The two-shape aggregate and all control/calibration limits determine SDK eligibility; no full-proof speedup or adoption is claimed. [Independent review](artifacts/postcapture-review.json).

1x2-w32 maximum query RSS MiB: OFF 36.50, PARENT 90.00, ON 58.00. These are process high-water maxima including load/primes, not full-proof or minimum RAM.

2x2-w32 maximum query RSS MiB: OFF 74.00, PARENT 114.50, ON 100.00. These are process high-water maxima including load/primes, not full-proof or minimum RAM.

Complete-query gate: **false**. Diagnostics: **true**. Eligible for a separate SDK trial: **false**. Production and full-proof measurements remain unchanged; this is not adoption.

Failed checks: `["1x2-w32/ON/OFF","1x2-w32/ON/FROZEN","two_shapes/ON/OFF/gain","two_shapes/ON/FROZEN/gain"]`. All observations and both required controls are retained.

## Change and prior result

The [previous joint-top layout](../2026-10-30-joint-top175/README.md) reduced counted work and improved primary query latency, but failed its two-shape gate and increased query RSS. That gate remains false. This revision changes only the marginal task decomposition and its checked capacity estimate; joint classes, first reduction, scalar/Phi/Booth geometry, ordinary windows, field arithmetic and input ownership stay unchanged. Six routing/math/first-stage/admission functions match the parent byte for byte.

A fixed component digit leaves at most 129 choices for the other joint digit. Each signed marginal bin therefore has at most 129 class sums. Greedy consecutive-bin groups contain at most the original G2 indexed-leaf tile size, 1024 points. Every nonlast range contains at least 896 points; at most 19 jobs suffice for 16640 joint classes. Each range calls the existing group_affine reducer, then 128 output bins are concatenated before the original sign/phi/component merges. This restores multiple independent marginal tasks without a width/core/fixture selector or parameter sweep. First-stage class reductions are unchanged; they are not claimed universally bounded by 1024.

## Correctness and capacity

Fresh OFF/ON each pass all 128 core and eight prover tests, unfiltered. Four child tests include the three parent complete-curve/input-preservation/custom-pool/denial checks and one new partition/bucket test. The latter checks 132 empty/skewed/dense/boundary count patterns, complete range coverage/order, 1024-point bounds, 896-point nonlast occupancy and every signed marginal bucket at 0/1/1023/1024/1025/16640 classes against known-generator sums. Repeated/opposite/identity points and signed extrema are retained. The unchanged 17 field files inherit 33 field tests; no new field formula is introduced.

Private source 176 retains 173 exact originals plus two reversible parents and one child; inverse edits recover all 175 accepted files. Admission precedes routing. The existing optional reservation now also charges the bounded range list, per-job Vec/result margins, the extra 128 output concatenation slots and all jobs' existing typed tree workspace. All point references are disjoint within a marginal; no input-sized persistent key copy, lookup table, new plan/pool or unsafe code is added. Denial falls back before construction. Full stack, allocation-failure, pressure, portable/phone/huge-key/concurrent-proof behavior still needs qualification before adoption.

The initial core compilation lacked an explicit Fq2 import and stopped before any test executed. Initial child/plan/pins/failed command receipts are retained. The one-line import repair changes no function body; dependency artifacts are reused and no successful test or workflow step is repeated. Both corrected native and query-link workflows finish before capture. [Source](artifacts/joint_top.rs), [recovery](artifacts/source-audit.json), [parent comparison](artifacts/parent-recovery.json).

## Complete-query measurements

Ryzen 9950X, 16 physical/32 logical CPUs, shared host. Four cases: primary solo on CPU 2, primary16 on CPUs 0–15, primary32 and 2x2/32 on CPUs 0–31. Thirty-one fixed rotated/reversed blocks compare exact accepted API-only FROZEN, fresh OFF, bounded ON and exact previous ON as PARENT diagnostic. Parent observations are controls for this changed layout; its historical failed gate is not revised. Every process measures 17 complete queries after three checked primes. Reset/grouping/decomposition/all fills/merges/reductions/allocation/drop/normalization are inside; parsing and witness solving are outside. Every output matches the inherited independent complete-Arkworks fixture hash, all point inputs stay unchanged and affinities are checked. No old oracle repeats.

|Case|FROZEN ms|OFF ms|PARENT ms|ON ms|ON/OFF ratio [bounds]|Wins/31|ON/FROZEN ratio [bounds]|Wins/31|
|---|---:|---:|---:|---:|---|---:|---|---:|
|1x2-w1|39.586041|39.583095|34.947821|35.791769|0.905093124 [0.903817888, 0.905956596]|31|0.903877128 [0.902894085, 0.905345595]|31|
|1x2-w16|3.735892|3.719892|3.736413|3.699844|0.991074097 [0.983924203, 1.004460248]|20|0.996717874 [0.985044714, 1.000304470]|20|
|1x2-w32|3.169656|3.173453|3.163084|3.190836|1.008492642 [1.001739242, 1.015471148]|7|1.008720639 [1.000240089, 1.015594392]|9|
|2x2-w32|3.670408|3.640732|3.753826|3.657755|1.002113044 [0.997026225, 1.010495011]|12|0.996293802 [0.988136790, 1.003756665]|19|

Primary32 and the equal-log two-shape32 aggregate require upper paired ratios<1 and at least 21/31 wins versus BOTH OFF/FROZEN. All other context upper ratios must be≤1.02; all OFF/FROZEN calibration intervals must fit factor 1.03. Sorted ranks 10–22/31 have nominal 97.055% individual coverage conditional on exchangeable blocks, without a familywise guarantee. PARENT ratios are diagnostic, not a changed adoption condition. There is no minimum gain floor, exclusion, sample extension, threshold change or favorable-subset promotion.

|Aggregate versus|Ratio [bounds]|Wins/31|Passed|
|---|---|---:|---|
|OFF|1.003777822 [0.997850368, 1.008683581]|11|false|
|FROZEN|1.002163759 [0.992691346, 1.008034273]|14|false|

## CPU service and process RSS

CPU is summed warm user+system ms/query, including runtime behavior. RSS is the maximum high-water mark across 31 fresh query processes, including fixture load/primes; it is not full-proof memory, minimum RAM or measured extra live payload. Do not add these differences to accepted SDK RSS. Marginal medians and paired ratios are distinct statistics.

|Case|FROZEN/OFF/PARENT/ON CPU ms per query|FROZEN/OFF/PARENT/ON maximum RSS MiB|ON/PARENT paired ratio [bounds]|
|---|---|---|---|
|1x2-w1|39.590/39.586/34.959/35.805|15.25/15.25/15.25/15.25|1.023892959 [1.023163020, 1.026061173]|
|1x2-w16|47.581/47.485/42.216/43.857|28.00/26.50/58.50/39.75|0.987853685 [0.980742598, 0.995023625]|
|1x2-w32|73.220/72.973/65.249/68.988|44.50/36.50/90.00/58.00|1.010426785 [1.000672923, 1.015799780]|
|2x2-w32|79.365/79.071/72.004/74.869|66.50/74.00/114.50/100.00|0.979624788 [0.966335595, 0.986844500]|

## Fixed counters and profiles

All latency precedes six acknowledged-FIFO stat panels (31 queries) and six fixed 16M cycles:u profiles (251 queries), three primes excluded, at primary16/32 and 2x2/32 OFF/ON. All captures finish before ownership changes, decoding, event/loss/mapping checks and analysis. Every helper is compiled/pinned beforehand; no independent owned build/helper/hash/archive/analyzer workload overlaps capture. Counter panels require ≥99.9% event running. Profiles require ≥1000 samples,≤5%unknown period, one PID, recorded worker-TID/CPU/time/mappings/fixed period and zero lost records. Positive joint_top samples are required in all three ON profiles. The inherited absolute-period coverage deficit remains unresolved; shares are not local CPI, exclusive costs or removable wall-time fractions.

|Panel|Instructions/query M|Cycles/query M|Qualified|
|---|---:|---:|---|
|stat-1x2-w16-OFF|582.000|234.534|true|
|stat-1x2-w16-ON|529.616|216.203|true|
|stat-1x2-w32-OFF|600.455|334.606|true|
|stat-1x2-w32-ON|549.407|307.498|true|
|stat-2x2-w32-OFF|628.395|360.170|true|
|stat-2x2-w32-ON|582.125|340.386|true|

|Profile|Samples|Unknown period fraction|Attribution qualified|
|---|---:|---:|---|
|cycles-1x2-w16-OFF|3669|0.008722|true|
|cycles-1x2-w16-ON|3389|0.007967|true|
|cycles-1x2-w32-OFF|5203|0.010955|true|
|cycles-1x2-w32-ON|4835|0.010134|true|
|cycles-2x2-w32-OFF|5642|0.009926|true|
|cycles-2x2-w32-ON|5333|0.009001|true|

These fixed diagnostic panels are observations, not repeated-trial confidence intervals or a causal decomposition. Generic cache misses do not identify a cache level or bandwidth limit. Fewer instructions or smaller leaves alone do not establish a faster complete proof.

## Accepted proof results, unchanged

The original all-eight CPU 10× goal remains 0/8. Inherited local 32-worker full-proof measurements use supplied M5 CPU/Metal references, not new Apple measurements or candidate proof timings. The old accepted physical16 calibration exception remains documented and does not transfer to this gate.

|Shape|Accepted ms|Peak RSS MiB|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|
|1x2|8.022165|134.00|37 / 4.61×|24 / 2.99×|3.7|
|2x2|11.335969|275.00|59 / 5.20×|30 / 2.65×|5.9|
|4x2|15.945544|315.00|109 / 6.84×|48 / 3.01×|10.9|
|8x2|24.090485|512.50|189 / 7.85×|74 / 3.07×|18.9|
|12x2|33.844855|707.25|284 / 8.39×|101 / 2.98×|28.4|
|16x2|39.931557|826.25|335 / 8.39×|124 / 3.11×|33.5|
|24x2|66.233309|1328.75|528 / 7.97×|176 / 2.66×|52.8|
|merge-24x1|65.273224|1175.50|507 / 7.77×|175 / 2.68×|50.7|
|merge-54x1|162.250464|1831.87|—|—|—|

Latest retained C7a primary is Zopro 11.915591 ms at 32 workers versus Aeglos 7.959184 ms at 96, best tested configurations in that campaign. All nine local/AWS rows are retained exactly in [accepted rows](artifacts/accepted-rows.json) and the [current snapshot](../2026-10-30-current-bench-snapshot175/README.md). No new AWS/Aeglos, full-SDK proof, independent export, phone/ARM/TEE, forester 250/4999, pressure/swap/minimum-RAM or concurrent-throughput result follows. Ledger remains 25,231. A query/diagnostic pass can only select separate all-nine SDK/Gnark/resource/portable/final-artifact qualification before adoption.

All 554 workflow steps complete: 524 query processes, 11712 complete queries including primes. Native 10 and query-link 2 steps complete separately. Source, helpers, libraries, fixtures, oracle receipts and exact parent input pins close. Full source and every command/observation are retained; archives round-trip byte-exactly. Raw private fixture/query values, binaries/libraries and raw perf files stay local. No successful native test, old oracle or performance capture repeats.

[Declared design](artifacts/design.md), [all results](artifacts/analysis.json), [every observation](artifacts/rows.json), [parent diagnostic](artifacts/parent-pairs.json), [new source](artifacts/joint_top.rs), [source recovery](artifacts/source-audit.json), [exact source archive](artifacts/source.json.gz), [helpers/protocols](artifacts/helpers.json.gz), [receipts](artifacts/receipts.json.gz), [archive review](artifacts/archive-review.json), [checksums](SHA256SUMS).
