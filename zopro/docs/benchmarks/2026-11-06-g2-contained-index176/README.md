# G2 indexed packing within the original workspace

The contained packing candidate reduces CPU work and single-thread G2 query latency, but does not pass the declared complete-query selection gate. It is not adopted. Production source 176 and the accepted SDK remain unchanged. These are local component measurements; no new full proof or AWS measurement was made.

At 32 workers, the paired primary 1x2 query improvement is 0.652% with a nominal marginal median interval of 0.009%–1.232% improvement. The equal-log two-shape aggregate improves 0.387%, but its interval includes parity and it wins 9/15 blocks, below the declared 10/15 requirement. The unchanged A control also misses its stability bound. Retain the positive solo and primary observations without promoting a favorable subset or changing the criteria after capture.

## Change and resource policy

This uses the global 1024 point indexed packing from [the October 29 experiment](../2026-10-29-g2-packed-index175/README.md). That earlier full-SDK trial reduced CPU work but missed its latency gate. The packing mathematics, complete curve exception handling, global segment boundaries, disjoint recursive output slices, and final per-bucket grouping are inherited. This is not a new packing algorithm.

The new mechanism replaces the extra per-top optional OS-headroom reservation with a checked allocation-layout comparison against the existing whole-G2 workspace forecast. It restricts the route to the current native G2 signed 128 bin top and supported base lengths. It counts segment/tile/output metadata, heap local starts/counts, leaf scratch and outside-top arrays. Explicit `ZOPRO_CPU_PREPARE_MAX_BYTES` presence, unsupported geometry, checked overflow or failed containment falls back to the original reducer before packing allocations. No lookup table, duplicate key, retained plan or new pool is introduced.

The experiment compares original OFF against contained packing ON. It does **not** separately isolate the effect of removing the additional reservation from the effect of packing itself. It therefore cannot attribute the gains to OS-headroom checks alone. The native path is a private snapshot, not an unchecked production admission change.

## Preliminary layout decision

Before implementing this candidate, an original-bucket-boundary planner was tested against the original per-bin reduction and the earlier global packing. Three interval/planning tests passed. A census used pinned primary raw scalars and the prior qualified mathematical 2x2 grouping/Phi2 split. The grouped scalar digest matches the earlier independent fixture evidence; this census does not replay production resource admission or claim a new curve oracle.

| Population, four top components total | Original first-stage layer proxy | Preserved-boundary packing | Earlier global packing, including final layer proxy |
|---|---:|---:|---:|
|1x2|3289|462|441|
|2x2 grouped|3345|492|452|

All three layouts conserve total generic noncancelling addition count. The proposed preserved-boundary variant needs more modeled inversion layers and tails than the earlier global layout. It was closed before native curve implementation or timing. These proxies omit actual exceptional curve cases and are not measured inversion counts or latency predictions. The contained-workspace policy became the distinct implementation question.

Preserved preliminary failures: the first census compile had an ambiguous `.sum()` type; the next compile passed, but its run failed the grouped scalar digest check because the fixture's little-endian scalar serialization was hashed instead of the qualified big-endian representation. An initial digest repair helper also failed Rust ownership checking, followed by dependent commands against missing outputs. The corrected helper changes only digest serialization. Final compile and census pass; successful planner/compile stages were not rerun. Initial/fixed/final source, plans, pins and workflow receipts are in the preliminary archive.

## Measurement

The [prospective protocol](artifacts/protocol.md) was frozen before preflight/timing. Fifteen fixed rotated/reversed process blocks cover 1x2 and 2x2 B at 1/16/32 workers and unchanged primary A32, with three arms:

- FROZEN: retained optimized API caller `0ba4cd4e…`, a descriptive reference. It is not the accepted full SDK.
- OFF: fresh direct thin wrapper around the byte-exact original query.
- ON: the same executable and ABI, with contained packing. No TLS metadata path is used.

Every B process uses 3 primes, 17 timed queries and 1 postcheck. A uses 47 timed queries to lengthen the negative-control window. Parsing is excluded; reset copy, grouping, Phi2, packing/reductions, allocation and normalization are included. Every prime/timed/final result matches the prior independently qualified Arkworks full-MSM oracle. Point inputs remain unchanged. Every ON B reports four completed packed top entries; every other arm reports the original route.

The necessary same-executable gate requires all six B nonregression upper bounds ≤1.02; primary B32 and the two-shape B32 aggregate upper bounds <1 and ≥10/15 wins; unchanged A32 interval inside [1/1.03, 1.03]. Ranks 4–12 of 15 give nominal 96.484375% marginal median intervals conditional on exchangeable blocks, without simultaneous coverage. No gain floor, exclusions, extensions or rerolls were used. FROZEN comparisons are all published and descriptive under this prospective design, because their cross-build gaps were already known and that caller is not the accepted proof binary. Favorable FROZEN rows cannot rescue failed ON/OFF gates.

## Complete-query results

Ratios are medians of within-block ON/OFF process-median ratios, which can differ from ratios of the displayed medians. Times are milliseconds per G2 query, **not proof times**.

|Query/workers|FROZEN ms|OFF ms|ON ms|Paired ON/OFF [lower, upper]|Wins/15|
|---|---:|---:|---:|---|---:|
|1x2 B /1|37.364762|37.475151|32.791511|0.876166 [0.874074, 0.877572]|15|
|1x2 B /16|3.624784|3.613162|3.601890|0.997073 [0.987409, 1.000327]|11|
|1x2 B /32|3.258974|3.216895|3.203630|0.993481 [0.987679, 0.999908]|12|
|2x2 B /1|39.980503|40.077236|35.566452|0.887667 [0.886688, 0.888504]|15|
|2x2 B /16|4.178608|4.172646|4.204427|1.002601 [0.995476, 1.008070]|6|
|2x2 B /32|3.821174|3.826374|3.813169|0.997939 [0.986562, 1.011836]|9|
|Unchanged 1x2 A /32|1.177700|1.172891|1.189583|1.015696 [0.995746, 1.035984]|6|

Two-shape B32 ON/OFF: **0.996134 [0.987120, 1.005992], 9/15 wins**. This fails both the upper-bound and win-count parts of the aggregate gain criterion. A's upper uncertainty bound exceeds 1.03; it is not evidence of a measured 3.598% slowdown. There are two failed check labels: aggregate gain and negative-control stability. All six B nonregression checks and the primary gain check pass.

The descriptive B32 aggregate ON/FROZEN is 0.987587 [0.984020, 0.998764], 12/15 wins. Primary 32 OFF/FROZEN and ON/FROZEN intervals extend below the symmetric 1.03 context range, so those context checks fail despite favorable direction. Full FROZEN ratios, raw clocks and all gate results are preserved in [analysis.json](artifacts/analysis.json). They do not change the decision.

## CPU work, profiles and memory

All 315 latency processes finished before 6 PMU callers of 200 queries and 6 separate cycles profiles of 1024 queries. The PMU groups ran 100% of their enabled time. All six profiles qualify, with 124386 samples in total and zero lost records. PID/TID/CPU, mappings, task events and the fixed 16 million-cycle period are checked. Source and actual optimized consumer argv are pinned; native compilation uses O3, native CPU, fat LTO and one codegen unit.

|32 worker B|OFF instructions/query|ON instructions/query|OFF cycles/query|ON cycles/query|OFF latency CPU ms/query|ON latency CPU ms/query|
|---|---:|---:|---:|---:|---:|---:|
|1x2|569.902M|520.188M|320.045M|287.212M|69.739|63.261|
|2x2|597.811M|548.777M|348.860M|316.644M|75.719|69.633|

The candidate removes about 8–9% of retired instructions and 9–10% of cycles in these warm PMU panels. This is useful work reduction, but the much smaller multithreaded latency change shows that it does not translate proportionally into the measured query completion time. These counters do not identify the exact critical path.

In primary profiles, samples in symbols containing `reduce_layer` fall 2920→2007 and those containing `inverse` fall 1857→1023; packed-path symbols account for 610 samples. For 2x2 these counts are 3527→2478, 1912→1114 and 689 packed samples. These are descriptive fixed-workload samples, not exact field-operation counts or exclusive wall time. Kernel waiting/CPU helping and packing overhead require a distinct justified test before assigning a causal explanation.

Maximum whole-process RSS over the 15 latency processes, MiB:

|Query/workers|FROZEN|OFF|ON|
|---|---:|---:|---:|
|1x2 B /1|15.50|15.50|15.50|
|1x2 B /16|27.75|27.50|27.75|
|1x2 B /32|32.00|34.00|31.50|
|2x2 B /1|27.25|27.25|27.25|
|2x2 B /16|42.25|42.75|43.75|
|2x2 B /32|59.75|53.25|60.25|
|Unchanged A /32|27.75|27.25|25.50|

The longer profiles have different allocator histories: primary FROZEN/OFF/ON RSS 67.75/59.75/66.00 MiB; 2x2 167.75/109.75/69.25 MiB. All raw faults and switch counters are retained. RSS is not minimum RAM, allocation-lifetime proof, or a phone/pressure/concurrent-prover qualification. The explicit-budget fallback and every supported half-length's typed containment were tested, but no new device, swap or huge-circuit pressure run was made.

## Qualification and decision

The native workflow passes 128 unfiltered core tests, including three new tests over identities/signs/cancellation, tails, skewed bins, immutable inputs, whole-query grouping, explicit-budget fallback and 1/4/16/32 workers with small stacks. Every supported half-length 512..32768 passes the typed containment check. Existing field suites and prior independent fixture qualification were inherited, not rerun. The snapshot has 178 source files: 171 original files are exact, five parents contain append-only diagnostic additions, and removing those additions recovers all 176 originals. Five cloned bodies are inverse-audited and checked against actual source. The driver is also inverse-audited against its optimized predecessor.

A separately compiled checker reconstructs timing medians, bounds, wins, CPU/RSS, all declared gates, raw PMU counts and all 124386 profile samples. It verifies 461 pin occurrences, all actual command argv, the managed environment receipt projection, exact reused preflight argv/environment and six additional raw loss checks. The main native/capture workflows contain 375 successful commands: 3 native plus 372 capture. 348 caller rows represent 15462 complete queries including 35 queries in seven exact reused FROZEN preflights; 15427 queries are new. They are component calls, not proofs. Preliminary planner/census commands and their failures are separately archived and are not added to those totals.

No full-SDK follow-up, source adoption, new proof export, AWS allocation or unchanged reroll follows this failed screen. The canonical verified proof ledger remains 30846. The sole-thread reduction and smaller CPU-work demand are retained evidence for a future distinct scaling/critical-path question, not a qualified general proving improvement.

## Accepted end-to-end comparison remains unchanged

Historical local accepted source 176/SDK `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`, 32 workers. These values were **not remeasured** by this experiment. M5 CPU/Metal are the supplied references, not matched hardware-efficiency measurements.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU 10× limit ms|
|---|---:|---:|---:|---:|
|1x2|7.928660|37|24|3.7|
|2x2|11.151932|59|30|5.9|
|4x2|16.086386|109|48|10.9|
|8x2|24.179453|189|74|18.9|
|12x2|33.884455|284|101|28.4|
|16x2|40.077381|335|124|33.5|
|24x2|65.850777|528|176|52.8|
|merge24|65.060937|507|175|50.7|
|merge54|161.883899|—|—|—|

The all-eight CPU 10× goal remains active and unmet, 0/8. No new Aeglos comparison was performed; historical c7a results remain separate from these local component observations.

## Artifacts

[Review](artifacts/review.json), [design](artifacts/design.md), [source audit](artifacts/source-audit.json), [driver audit](artifacts/driver-audit.json), [protocol](artifacts/protocol.md) and [checksums](SHA256SUMS) accompany lossless source, UTF-8receipts/helpers, preliminary planner/census evidence and six raw perf archives. Archive round trips and member hashes are verified. Private point/scalar/key/witness arrays, executables and libraries are excluded. Source helpers retain exact experimental text for reproducibility.
