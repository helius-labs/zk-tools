# Combined prefixes and X-sum carry removal: full-shape qualification

The original eight-shape aggregate favors the candidate by **0.94% versus accepted** and **1.03% versus fresh OFF**, with both stratified intervals below parity. The primary32 pilot remains **7.869400 ms**. All 32-worker calibration, nonregression and RSS checks pass. The complete gate remains **failed**, with two failures at primary16: OFF/accepted calibration and candidate/accepted nonregression uncertainty. **No source is adopted.** The original CPU 10× goal remains active and unmet, **0/8**.

This is the first latency qualification of the combined candidate on the other seven supplied shapes, merge54 and primary16. It reuses the three immutable [pilot SDKs](../2026-11-08-uniform-carry-sdk176/README.md); no native build, successful preflight, test, old disassembly, recording, proof verification or decoder is repeated. The successful primary32 pilot remains unchanged and is not rerun. Earlier uniform-only **7.859756 ms** and accepted **7.915095 ms** observations remain valid history: this is not a new absolute record or an established improvement over that earlier candidate.

## Complete SDK latency and memory

Ryzen 9 9950X. Fifteen fixed rotated/reversed matched blocks cover nine new contexts. Each process runs three untimed primes, then 51 timed requests for the nonprimary shapes or 151 for primary16. Solving, fresh randomized proving, internal verification and serialization are timed; image loading, primes and proof export are excluded. All observations are retained. CPU affinity is 0–31 for 32 workers and 0–15 for 16 workers.

|Context|Accepted ms|OFF ms|Candidate ms|Max RSS MiB, accepted / OFF / candidate|
|---|---:|---:|---:|---:|
|1x2,32 workers, inherited pilot|7.988685|8.031154|7.869400|135.75 / 133.25 / 135.00|
|2x2-w32|11.170223|11.233644|11.018191|293.75 / 296.25 / 299.25|
|4x2-w32|15.957173|15.951837|15.922784|333.75 / 336.50 / 334.50|
|8x2-w32|24.241904|24.042195|24.003699|542.75 / 534.25 / 544.75|
|12x2-w32|33.728675|33.862138|33.729083|748.75 / 758.25 / 750.50|
|16x2-w32|40.047534|40.127204|39.706864|882.00 / 882.00 / 884.75|
|24x2-w32|65.881596|66.226747|65.792319|1427.18 / 1355.00 / 1376.69|
|merge-24x1-w32|65.325410|65.148691|64.546908|1210.21 / 1203.75 / 1225.48|
|merge-54x1-w32|161.772979|161.831985|161.197772|1992.88 / 1974.80 / 1926.49|
|1x2-w16|9.484109|10.222249|9.433169|90.25 / 91.00 / 88.75|

Times are medians of process means. RSS is the largest process `ru_maxrss` during loading/primes/timing, not minimum RAM or a pressure/device qualification. The per-context RSS screen is candidate ≤ larger control ×1.05 +1024 KiB; it is not a universal architecture cap. Memory observations are mixed, and all nine screens pass. No new table, buffer, key copy, solver plan or pool is added.

|New context|Candidate / accepted, paired GM [95% bootstrap]|Wins /15|Candidate / OFF, paired GM [95% bootstrap]|Wins /15|
|---|---:|---:|---:|---:|
|2x2-w32|0.983536 [0.978485, 0.988072]|15|0.977748 [0.968532, 0.984002]|15|
|4x2-w32|0.996452 [0.989540, 1.003777]|9|0.995267 [0.985044, 1.005364]|6|
|8x2-w32|0.982563 [0.972276, 0.992309]|12|0.995032 [0.987505, 1.004175]|11|
|12x2-w32|0.999108 [0.992525, 1.006325]|9|0.992534 [0.985426, 0.998973]|11|
|16x2-w32|0.989900 [0.984153, 0.995609]|12|0.989433 [0.985864, 0.993378]|14|
|24x2-w32|1.002480 [0.995665, 1.009282]|5|0.993905 [0.988935, 0.998548]|12|
|merge-24x1-w32|0.987741 [0.982105, 0.993063]|14|0.993115 [0.989564, 0.997101]|14|
|merge-54x1-w32|0.997416 [0.992759, 1.002113]|9|0.997052 [0.992183, 1.002122]|9|
|1x2-w16|1.019581 [0.972046, 1.077498]|9|0.952370 [0.899786, 1.011871]|10|

The primary32 pilot uses its original fixed-rank median-ratio bounds, not these bootstrap GM bounds. Full-scope requirements were fixed before clocks: OFF/accepted intervals within factor1.03; candidate/both-control upper bounds ≤1.02 for every new context; original-eight equal-log stratified gain upper<1 against both controls; inherited primary gain, RSS and diagnostic quality. There is no minimum positive gain floor. **38/40 checks pass.** The two failed checks remain failed; there is no exclusion, extra block, unchanged reroll or favorable subset promotion.

|Original-eight aggregate|Equal-log paired GM|Stratified 95% bootstrap|
|---|---:|---:|
|Candidate / accepted|0.990633372|[0.988334433, 0.992956119]|
|Candidate / OFF|0.989660396|[0.987257479, 0.992003732]|

Each of the eight supplied shapes has equal log weight. The unchanged primary32 stratum has31blocks; the seven new shape strata have15each. Each stratum resamples its own matched ratios, using20,000 fixed-seed resamples and the byte-exact previously qualified procedure. Separately captured shapes are not treated as synchronized blocks. Exchangeability and independent-stratum assumptions are not validated against shared-host drift or cross-shape correlation; these are marginal intervals, not a familywise guarantee. Merge54 and primary16 are outside the eight-shape aggregate.

## What failed at16workers

OFF/accepted has paired GM **1.070572**, interval **[1.025966,1.119093]**, outside the declared calibration range. Candidate/accepted has paired GM **1.019581**, interval **[0.972046,1.077498]**, crossing parity and exceeding the1.02 nonregression bound. Its marginal median is lower (9.433169 versus9.484109ms), but that does not override the paired comparison: some slow processes affect the full matched estimate. Candidate/OFF is0.952370 [0.899786,1.011871]. No reliable16worker speedup or slowdown is established.

All raw16worker observations remain. A post-study order audit below only describes recorded samples; it does not change a gate, remove a process, select a new subset or establish a cause. It reports each arm's first/middle/last position in the matched triplet and measured CPU/wall ratios. Lower CPU occupancy in a slow process can reflect waiting or interference and does not identify CPU frequency, allocator, scheduler, cache or arithmetic as the cause. No such tracing was collected.

|16-worker arm and position|Processes|Median process mean ms|Median occupied CPU equivalents|
|---|---:|---:|---:|
|accepted, first|5|10.004167|11.638|
|accepted, middle|5|9.484109|12.302|
|accepted, last|5|9.375040|12.494|
|candidate, first|5|10.691521|10.494|
|candidate, middle|5|9.246784|12.434|
|candidate, last|5|9.194423|12.483|
|control, first|5|10.656095|10.898|
|control, middle|5|9.963949|11.760|
|control, last|5|9.534449|12.252|

## Automatic CPU diagnostics

All405latency processes precede12diagnostic processes. Primary16 has31counter/251profile requests per arm; merge54 has7/17, each after3primes. FIFO-gated user cycles use period16million and512mmap pages. All six profiles qualify, with zeroLOST records, ≥95% assigned SDK-main/worker ownership, valid actual PID/TID/CPU/MMAP/header/period bindings and FFT-leaf samples. All four grouped PMU events report100%running. No owned build/helper/hash/analysis/decoder/verifier/report/push overlaps any recording. Exclusive host use is not established.

|Context and arm|Profile samples|Unknown symbols %|
|---|---:|---:|
|1x2-w16-accepted|8636|1.355|
|1x2-w16-candidate|8597|1.605|
|1x2-w16-control|8500|1.388|
|merge-54x1-w32-accepted|21994|0.673|
|merge-54x1-w32-candidate|21631|0.758|
|merge-54x1-w32-control|21386|0.599|

|Context, ON / OFF|User cycles|Instructions|Generic cache misses|Branch misses|
|---|---:|---:|---:|---:|
|1x2-w16|0.993554|0.980962|1.006032|1.001392|
|merge-54x1-w32|1.012542|0.983581|1.008786|1.016213|

The candidate reduces counted instructions in both panels, while cycles and cache misses are mixed. A single panel is not a repeated-counter interval or a causal cache/DRAM/spill diagnosis. Flat profile samples are not exclusive wall-time fractions. The inherited primary32 diagnostics remain in the pilot report.

## Verification and provenance

All864 workflow commands complete:405latency +12diagnostic processes, then417independent Gnark verifiers and30offline ownership/decoder/event/metadata/loss commands. There are **27,324new SDKrequests** including primes and **417fresh independently verified exports**, each164bytes with44public-input bytes, bound to image/public/VK and distinct from prior inventory and one another. Canonical inventory advances **35,446→35,863**. All timed requests also perform internal verification. Unexported requests are not counted as independent Gnark exports.

The independent reader checks pins, command/stream receipts, every fresh proof binding/inventory union and reconstructs raw means, ratios, wins, RSS, aggregate point estimates and selection. Exact qualified bootstrap functions and detailed diagnostic parsing are inherited rather than independently reimplemented. Its initial final pass failed an exact JSON-array comparison at about one ULP; a new reader preserves exact counts and uses the existing numerical tolerance for reconstructed ratios. The original source/executable/pins and repair note remain. No capture, proof, verifier, decoder, statistical analysis or threshold changes or repeats.

Native182-file source composition, all176production hashes, eight-library OFF/ON identities, coupled native correctness, generic compilation and all9+primary1/16 correctness preflight inherit the successful pilot. No new native or portable runtime qualification is claimed. Private generated-field integration remains pending and no source promotion occurs. Actual keys/images/requests/witness/proof/public payloads, executables/libraries, rawperf and full ledgers stay private; public evidence retains hashes, sources/helpers, plans/pins, command receipts, raw samples and decoded diagnostics.

## All supplied M5 comparisons

These are candidate local CPU observations against supplied references on different hardware. Primary32 is the unchanged pilot; all other shapes are fresh here. The CPU10× requirement is not met for any supplied shape.

|Shape|Local candidate CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.869400|37 / 4.702×|24 / 3.050×|3.7|
|2x2|11.018191|59 / 5.355×|30 / 2.723×|5.9|
|4x2|15.922784|109 / 6.846×|48 / 3.015×|10.9|
|8x2|24.003699|189 / 7.874×|74 / 3.083×|18.9|
|12x2|33.729083|284 / 8.420×|101 / 2.994×|28.4|
|16x2|39.706864|335 / 8.437×|124 / 3.123×|33.5|
|24x2|65.792319|528 / 8.025×|176 / 2.675×|52.8|
|merge-24x1|64.546908|507 / 7.855×|175 / 2.711×|50.7|

Merge54 candidate161.197772ms has no supplied M5 reference. The measured approximately1% aggregate gain is useful positive evidence, well short of the requested2× jump or original10× goal. There is no new AWS/Aeglos, huge-circuit, mobile/ARM/TEE, pressure/swap, minimum-RAM or concurrent-proof result. Next work should explain the saved16worker tails/control separation and review exact source/resource compatibility before any explicit adoption decision; this failed full gate cannot be called a pass. No unchanged timing reroll is selected.

[Design](artifacts/design.md), [protocol](artifacts/protocol.json), [latency](artifacts/latency-analysis.json), [diagnostics](artifacts/diagnostic-analysis.json), [descriptive order audit](artifacts/order-audit.json), [reader repair](artifacts/review-repair.md), [review](artifacts/review.json), [publication](artifacts/publication.json), [evidence archive](artifacts/evidence.json.gz), [checksums](SHA256SUMS).
