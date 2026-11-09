# Short-forward G2 maps in the complete SDK

The primary 32-worker proof improves by 0.255% against the fresh control and 0.328% against the accepted reference under the declared paired analysis. The complete selection gate fails two of 79 checks: control calibration for 2x2 and all-eight gain against the fresh control. The candidate is closed without adoption or an unchanged rerun. Production is unchanged.

REF uses the frozen original-field accepted SDK with the existing fixed-placement driver. OFF and ON use coherent newly built core/prover/zolana/SDK artifacts with the same qualified short-factor field; only ON enables three affine forward-map replacements in the actual G2 entry. The scalar split, component order, projective maps, windows, grouping, solver, FFT and resource policy remain original. There is no added key table, input-sized array, retained workspace, pool or memory cap. The fixed factor occupies 64 logical constant bytes, independent of circuit size. The earlier field and query qualification are inherited, not replayed.

The actual integration passes 187 new independent Arkworks entry checks, including all 53 workers across 1/4/16/32-worker pools with requested 512 KiB stacks, a generic no-ASM/no-IFMA guard build, coherent native links and 26 SDK/Gnark preflights. All 176 accepted source originals recover from the private 177-file snapshot. Test-route counters are absent from the release consumers. The generic result is a compile check, not ARM execution.

The performance workflow completes all 1,263 commands on first execution: 591 latency processes, nine grouped PMU panels, nine cycle-IP profiles, 45 offline commands, then 609 independent Gnark checks. Capture requests total 44,106 including primes; all 609 distinct final exports pass independent verification. With preflights, the SDK stage totals 44,210 requests and 635 independent exports. The canonical private ledger advances from 40,469 before this stage through 40,495 after preflight to 41,104 after capture. Requests and prime calls are not counted as independent exports.

Each timed request includes witness preparation/solving, fresh randomized Groth16 proving, internal verification and serialization. Loading, pool creation, three prime calls and final export are excluded. The primary at 16/32 workers has 31 matched blocks of 151 requests per process; the other nine conditions have 15 blocks of 31 requests, except solo primary has 11 requests. REF/OFF/ON execute in rotating and reversing order. Worker placement, default Rayon stacks and allocator settings match. No concurrent proof-throughput result follows from these single-request measurements.

Latencies below are medians of process means, computed from all emitted per-request samples rounded to integer nanoseconds. Paired geometric ratios use corresponding blocks and 20,000 log-bootstrap resamples with the declared seed. Lower ratios are faster. Displayed medians need not divide to the paired estimate. Intervals assume exchangeable blocks and do not provide a familywise guarantee. There are no exclusions, extensions, rerolls or changed thresholds.

|Case|REF ms|OFF ms|ON ms|Paired ON/OFF [95% interval]|Wins|
|---|---:|---:|---:|---|---:|
|1x2-w32|7.832708|7.834469|7.818977|0.997448 [0.995276, 0.999810]|23/31|
|1x2-w16|9.253306|9.216070|9.207626|1.000267 [0.998762, 1.001923]|15/31|
|1x2-w1|95.555016|94.643655|95.007670|1.004089 [1.001801, 1.006186]|3/15|
|2x2-w32|11.065989|10.944376|10.947052|0.999157 [0.996246, 1.002080]|8/15|
|4x2-w32|15.913588|15.779798|15.776978|1.000520 [0.990591, 1.010137]|6/15|
|8x2-w32|24.254933|23.851184|24.066852|1.010347 [1.004549, 1.017752]|1/15|
|12x2-w32|34.007120|33.722974|33.872272|1.005969 [0.998863, 1.014197]|6/15|
|16x2-w32|40.086324|39.822746|39.845148|1.000131 [0.993426, 1.006883]|7/15|
|24x2-w32|66.467587|66.101704|66.009278|0.998347 [0.993619, 1.002605]|8/15|
|merge-24x1-w32|65.569872|65.278995|65.152232|0.998991 [0.994840, 1.003520]|9/15|
|merge-54x1-w32|162.947719|162.013188|161.916019|1.002643 [0.997021, 1.008281]|7/15|

Primary32 ON/OFF is 0.997448 [0.995276, 0.999810], with 23/31 wins; ON/REF is 0.996719 [0.994668, 0.999109], with 26/31 wins. Both gain and win checks pass. This is a supported small primary improvement, not a new sub-8-ms milestone. The accepted prover was already below 8 ms in earlier campaigns. The prior 7.699867 ms statistic was a median of process medians from a different campaign; it is not a matched regression control for these process means. Primary16 ON/OFF is 1.000267 [0.998762, 1.001923], with 15/31 wins, so a gain there is not established.

The failed checks are:

- `2x2-w32/OFF/REF/calibration`: ratio 0.975903 [0.942944, 0.995072], outside the declared interval-containment requirement [1/1.03, 1.03]. Block02 has REF/OFF/ON means 14.338358/11.136399/11.037644 ms. This favorable-to-ON reference observation remains included; no cause is established here.
- `ALL8/ON/OFF/gain`: equal-log ratio 1.001355 [0.999256, 1.003516]. The point estimate is 0.1355% slower, but neither gain nor regression is established. ALL8 ON/REF improves at 0.992197 [0.987000, 0.995939], with the calibration issue above retained. Shapes are resampled independently within their 31/15 blocks; unrelated shape blocks are not treated as synchronized observations.

All other 77 numeric checks pass, including all 22 nonregression and 22 RSS guards, both primary gain/win comparisons, ten of eleven calibrations and all 18 diagnostic quality checks. Source identities and proof correctness pass separately. The prospective protocol closes this fixed implementation on failure and authorizes no additional engineering exception. No candidate-only subset is promoted and no minimum gain floor is introduced.

|Case|REF peak MiB|OFF peak MiB|ON peak MiB|
|---|---:|---:|---:|
|1x2-w32|133.250|134.000|134.750|
|1x2-w16|87.750|88.250|89.750|
|1x2-w1|34.500|34.000|34.250|
|2x2-w32|277.750|273.500|269.250|
|4x2-w32|319.500|311.250|312.750|
|8x2-w32|507.250|509.750|508.500|
|12x2-w32|704.000|729.750|718.000|
|16x2-w32|826.000|813.750|826.250|
|24x2-w32|1292.637|1308.492|1308.625|
|merge-24x1-w32|1180.566|1172.656|1161.031|
|merge-54x1-w32|1997.043|1991.746|1941.500|

These are maximum whole-process RSS observations across each arm's latency processes, including image loading, primes and allocator state. They are not incremental scratch sizes, hard bounds, minimum RAM measurements or pressure/swap/mobile qualification. No very-large-circuit behavior is inferred.

All nine cycle profiles qualify: 120,193 samples, zero lost records, fixed 16M user-cycle periods, verified binary/PID/TID/CPU ownership and at most 2.184% unknown symbols. Absolute cycle-period coverage is not validated. All 36 PMU events have 100% reported running coverage. The single panels show ON/OFF instructions down 0.850% for primary32 and 0.696% for 2x2; user cycles down 0.406% and 0.774%. These diagnostics support lower CPU work but do not prove a wall-time critical-path saving.

The fresh original REF32 profile attributes 20.79% of samples to `stream_g1_impl`, 15.52% to ordinary borrowed G2 application, 7.44% to base-field inversion, 5.78% to G2 layer reduction, 4.72% to balanced G1 feed and 4.56% to G1 layer reduction. Crossbeam pinning accounts for 8.28% of aggregate samples; that is not evidence that its removal would save the same fraction of proof latency. The next distinct study should address larger useful-work consumers rather than repeat the three-map recipe. No replacement is selected by this report.

The reviewer independently reconstructs raw sample means, 79 gate decisions, RSS, proof freshness and source recovery. It is a separate reader run by the same agent, and shares the qualified bootstrap and diagnostic parsers with the analyzer. It is not an independent implementation of those parsers. All 307 capture pins are checked again for publication. No successful scientific command is repeated for publication.

The following comparison uses this campaign's unchanged accepted REF at 32 workers. M5 CPU and Metal are supplied reference timings, not newly measured devices. None of the eight CPU10x targets is reached. Merge54 has no supplied M5 reference.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU10x target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.832708|37|24|4.724x|3.064x|3.7|
|2x2|11.065989|59|30|5.332x|2.711x|5.9|
|4x2|15.913588|109|48|6.849x|3.016x|10.9|
|8x2|24.254933|189|74|7.792x|3.051x|18.9|
|12x2|34.007120|284|101|8.351x|2.970x|28.4|
|16x2|40.086324|335|124|8.357x|3.093x|33.5|
|24x2|66.467587|528|176|7.944x|2.648x|52.8|
|merge-24x1|65.569872|507|175|7.732x|2.669x|50.7|

[Results](artifacts/analysis.json), [review](artifacts/review-result.json), [protocol](artifacts/protocol.md), [prospective review](artifacts/prospective-review.json), [source audit](artifacts/source-audit.json), [decision](artifacts/decision.json), [proof inventory](artifacts/proof-inventory.json), [profile symbols](artifacts/profile-symbols.json), [CPU/Metal data](artifacts/fresh-accepted-comparison.json), [continuation](artifacts/next.md), [source and receipt archive](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Raw proofs, private input images, binary libraries, raw perf recordings and full proof ledgers remain private. This experiment establishes no new AWS, Aeglos, ARM, mobile, huge-circuit, minimum-RAM or concurrent-throughput result.
