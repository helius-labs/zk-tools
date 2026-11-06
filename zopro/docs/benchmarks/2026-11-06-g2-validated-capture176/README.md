# Once-validated G2 batches: complete-caller comparison

**Rejected: the candidate does not establish a complete-caller gain.** Its four-size 32-worker G2 aggregate is 0.457% slower than fresh OFF and 0.124% faster than the frozen accepted-source caller; both intervals include parity. Solo G2 regresses at every size. No SDK trial, proof-speed improvement or production change follows.

## Decision and limitations

The preregistered gate fails **35/76 timing checks**: 2 calibration, 29 nonregression and 4 gain checks. The G1-352-w32 and G2-200-w32 control-calibration failures remain in the result, so this is not a globally calibrated campaign. Stable solo G2 controls do calibrate, and ON loses all 15 paired blocks at every solo size against both controls. Small favorable estimates elsewhere remain published. No subset, extension, reroll or threshold change replaces the result.

The [qualified implementation](../2026-11-06-g2-validated-batch176/README.md) validates every masked destination once before writes, then uses fixed-size groups and justified unchecked accesses through the same field arithmetic. Validation stays **inside every timed kernel call**. Original signs, scalar tails, inversion, scratch capacities and input representation remain. No descriptor, point/key copy, table, plan or pool is added. Fewer compiled checks are not themselves a measured benefit.

## Timings

The table reports medians of 15 process observations, in ns per complete batch wave with all workers active. Ratios pair observations within each fixed block and need not equal ratios of displayed medians. These are component timings, not full-query/proof latency or concurrent-proof throughput.

|Context|Frozen ns/wave|OFF ns/wave|ON ns/wave|ON/OFF ratio [interval], wins|ON/frozen ratio [interval], wins|OFF/frozen ratio [interval], wins|
|---|---:|---:|---:|---|---|---|
|G1-200-w1|4255.209|4259.017|4258.573|0.999680703 [0.998327947, 1.003325722], 9/15|1.001406401 [0.999588598, 1.003343258], 5/15|1.000919311 [0.999436938, 1.004147474], 4/15|
|G1-200-w16|4842.600|4841.718|4890.922|1.009607949 [1.001635789, 1.012160223], 3/15|1.006960765 [1.003447530, 1.011084886], 2/15|1.001096216 [0.996382359, 1.003650275], 6/15|
|G1-200-w32|6953.781|6920.422|6972.815|1.002387420 [0.994641467, 1.016019295], 5/15|1.005634282 [0.993296623, 1.168005570], 4/15|0.999283219 [0.991348134, 1.014823270], 8/15|
|G1-201-w1|4385.303|4383.884|4382.451|1.001445487 [0.998314080, 1.003728206], 7/15|1.000365423 [0.996770147, 1.005328406], 7/15|0.998981968 [0.997108514, 1.002034606], 8/15|
|G1-201-w16|4972.808|4975.669|5028.104|1.004510421 [0.999934643, 1.012156940], 4/15|1.007662419 [0.999455580, 1.013979011], 4/15|1.000796424 [0.997817238, 1.004578345], 6/15|
|G1-201-w32|7142.599|7120.532|7118.616|0.999088537 [0.874792035, 1.011437091], 9/15|0.997025403 [0.860720332, 1.016821459], 10/15|0.997962650 [0.982518221, 1.004716458], 9/15|
|G1-352-w1|7009.178|7013.898|7015.724|1.000840686 [0.997446876, 1.002431669], 7/15|1.000072319 [0.998428112, 1.003628517], 7/15|1.000510940 [0.998107522, 1.002127169], 7/15|
|G1-352-w16|7994.303|7996.244|8038.365|1.003737087 [0.999506961, 1.006162660], 4/15|1.003918459 [1.000428892, 1.005673066], 2/15|1.000286298 [0.999076196, 1.001793784], 5/15|
|G1-352-w32|11630.117|11595.552|11727.352|1.003085992 [0.962908487, 1.035894632], 5/15|1.003481604 [0.795205452, 1.255704234], 7/15|0.990748072 [0.809165633, 1.262251684], 10/15|
|G1-353-w1|7113.036|7112.557|7119.540|1.000029551 [0.999534929, 1.002468208], 7/15|1.000954986 [0.999732298, 1.002496821], 5/15|1.000333243 [0.999775708, 1.001299417], 5/15|
|G1-353-w16|8118.374|8129.353|8166.936|1.004758195 [1.003259528, 1.006031002], 1/15|1.005701290 [1.001935794, 1.008108785], 2/15|1.000870407 [0.999734903, 1.002479101], 4/15|
|G1-353-w32|11911.137|11761.182|11774.650|0.998218558 [0.993159702, 1.015661138], 8/15|1.001026555 [0.983645989, 1.029055939], 7/15|0.995196286 [0.973127335, 1.020006281], 10/15|
|G2-200-w1|7749.714|7749.720|7945.341|1.025695235 [1.022413682, 1.026626079], 0/15|1.024889784 [1.022903864, 1.027207200], 0/15|1.000561171 [0.999214727, 1.001373883], 5/15|
|G2-200-w16|8978.310|8984.681|9138.551|1.017507909 [1.014532978, 1.018979212], 0/15|1.018309016 [1.015168196, 1.019247599], 0/15|1.000128412 [0.999097142, 1.002459305], 7/15|
|G2-200-w32|15009.676|14392.354|14605.893|1.011463545 [1.001847427, 1.023407481], 3/15|1.004725833 [0.968133486, 1.015077132], 6/15|0.994384513 [0.957419059, 1.001501153], 11/15|
|G2-201-w1|8186.494|8263.648|8357.250|1.010511010 [1.009362220, 1.013117342], 0/15|1.020938373 [1.018526381, 1.022148675], 0/15|1.009775499 [1.008093346, 1.011807607], 0/15|
|G2-201-w16|9475.623|9575.322|9611.515|1.004315246 [1.001358243, 1.006336353], 2/15|1.014659715 [1.013084343, 1.015548483], 0/15|1.009683846 [1.007512723, 1.012448329], 0/15|
|G2-201-w32|14951.377|15060.382|15067.867|0.999193147 [0.997110767, 1.005973705], 8/15|1.005951389 [0.977838120, 1.008792567], 5/15|1.006088625 [0.984308866, 1.009455336], 6/15|
|G2-352-w1|13112.022|13109.864|13435.653|1.024767194 [1.023504925, 1.026484581], 0/15|1.025059614 [1.023412557, 1.025896321], 0/15|0.999949992 [0.998663402, 1.001081296], 8/15|
|G2-352-w16|15259.911|15288.320|15601.805|1.020723733 [1.020308602, 1.021808290], 0/15|1.022432598 [1.021947098, 1.022981451], 0/15|1.001529671 [1.000610983, 1.002473946], 2/15|
|G2-352-w32|24886.628|24974.620|24879.238|0.998638929 [0.991633567, 1.009251492], 8/15|1.000335025 [0.991281474, 1.008211911], 7/15|1.000059842 [0.992666646, 1.013392263], 7/15|
|G2-353-w1|13508.117|13602.404|13865.262|1.018143918 [1.017011910, 1.019675930], 0/15|1.026439294 [1.025229660, 1.027444130], 0/15|1.008798307 [1.007101166, 1.009825244], 0/15|
|G2-353-w16|15699.846|15811.024|16056.684|1.015486775 [1.011761811, 1.017140972], 0/15|1.023369987 [1.022338736, 1.024017654], 0/15|1.007088791 [1.005729942, 1.012012719], 0/15|
|G2-353-w32|25413.302|25366.250|25374.790|0.999310310 [0.995690212, 1.006031101], 8/15|0.998791785 [0.992203737, 1.004842147], 9/15|0.997152923 [0.992409906, 1.008383011], 10/15|

|Aggregate|ON/control ratio [interval], wins|
|---|---|
|ALL/OFF|1.006693853 [0.999611520, 1.017141330], 4/15|
|ALL/frozen|1.008175921 [1.002886446, 1.016306265], 1/15|
|G2/OFF|1.004574572 [0.996387933, 1.009061024], 6/15|
|G2/frozen|0.998762155 [0.990911743, 1.006898006], 8/15|

All failed checks:

- `G1-200-w16/ON/frozen/nonregression`
- `G1-200-w16/ON/OFF/nonregression`
- `G1-200-w32/ON/frozen/nonregression`
- `G1-200-w32/ON/OFF/nonregression`
- `G1-201-w16/ON/frozen/nonregression`
- `G1-201-w16/ON/OFF/nonregression`
- `G1-201-w32/ON/frozen/nonregression`
- `G1-201-w32/ON/OFF/nonregression`
- `G1-352-w32/OFF/frozen/calibration`
- `G1-352-w32/ON/frozen/nonregression`
- `G1-352-w32/ON/OFF/nonregression`
- `G1-353-w32/ON/frozen/nonregression`
- `G1-353-w32/ON/OFF/nonregression`
- `G2-200-w1/ON/frozen/nonregression`
- `G2-200-w1/ON/OFF/nonregression`
- `G2-200-w16/ON/frozen/nonregression`
- `G2-200-w16/ON/OFF/nonregression`
- `G2-200-w32/OFF/frozen/calibration`
- `G2-200-w32/ON/frozen/nonregression`
- `G2-200-w32/ON/OFF/nonregression`
- `G2-201-w1/ON/frozen/nonregression`
- `G2-201-w1/ON/OFF/nonregression`
- `G2-201-w16/ON/frozen/nonregression`
- `G2-352-w1/ON/frozen/nonregression`
- `G2-352-w1/ON/OFF/nonregression`
- `G2-352-w16/ON/frozen/nonregression`
- `G2-352-w16/ON/OFF/nonregression`
- `G2-353-w1/ON/frozen/nonregression`
- `G2-353-w1/ON/OFF/nonregression`
- `G2-353-w16/ON/frozen/nonregression`
- `G2-353-w16/ON/OFF/nonregression`
- `G2-aggregate/ON/frozen/gain`
- `G2-aggregate/ON/OFF/gain`
- `ALL-aggregate/ON/frozen/gain`
- `ALL-aggregate/ON/OFF/gain`

## Measurement protocol

The [protocol](artifacts/protocol.md), campaign and independent analyzer were frozen before the first observation. All 24 contexts are G1/G2 × 200/201/352/353 points × 1/16/32 workers, using three immutable executables. Frozen389307 is the audited accepted-source caller, distinct from the production SDK; its original field-input provenance remains in the qualification note. Fresh OFF/ON explicitly link the original field1200 and their matching new cores. Existing native tests, models and builds are inherited, not repeated.

Each worker owns a public-generator fixture with seed19+worker. These are not arbitrary full-width curve coordinates; prior native field qualification covers full-width/lazy values. Three 128-round primes and reset precede timing. Every caller checks independent generator output, cross-arm hashes, equal retained capacities and zero timed allocations. Construction, reset and final checks are outside the timer. Timing includes dispatch and the complete kernel.

The diagnostic Rayon pool uses 512 KiB stacks and pins CPU2 solo, physical cores0–15 for 16 workers and CPUs0–31 for 32. Fixtures are dispatched through par_iter_mut and are not permanently bound to a worker TID. Fixed allocator environment is retained. Host topology, kernel, tools and governor observations are saved; frequency is not locked and exclusive host use is not established. No owned build, native test, second benchmark, verifier, packaging or large hash overlaps capture. Small checkpoint/log updates are bookkeeping.

Fifteen fixed blocks rotate/reverse contexts and arms. Rounds per worker are 32768 at 200/201 and 16384 at 352/353. Rank intervals use sorted zero-based ranks3/7/11 of 15 paired ratios; their coverage is marginal, not simultaneous family-wise. All 24 OFF/frozen intervals must fit[0.97,1.03]; all 48 ON/control upper nonregression bounds must be≤1.01. G2 logical32 across all four sizes and the all 24-context aggregate require upper<1 and≥11/15 wins against each control. There is no positive-gain floor.

All 1279 commands succeed once:72 preflights plus1 checker,1080 latency,18 PMU,18 cycles and 90 offline commands. All 1188 callers pass output/capacity/oracle/allocation guards. Independent review reconstructs argv, environments, exits/output hashes, all paired ratios,76 timing checks and their failures. No native/capture/profile/decode/analyzer stage is repeated for publication.

## CPU evidence

The final G2 function has3.62% fewer static instruction rows and 35 fewer bounds-panic call sites, plus an upfront assertion path; IFMA count remains 1450. Grouped PMU panels show about1.25–1.30% fewer executed G2 instructions, but higher cycle counts. The combined validation/iteration/access transformation therefore fails despite reducing instructions. The scan cost and other generated-code changes were not separately ablated; no individual causal cost is assigned. Generic cache-miss or fault counters do not establish a RAM-capacity bottleneck.

|Context|ON/OFF cycles|ON/OFF instructions|ON/OFF cache misses|ON/OFF branch misses|
|---|---:|---:|---:|---:|
|G1-200-w1|0.997626|1.000000|0.564373|0.928933|
|G1-200-w32|0.994832|0.999990|0.750751|0.992445|
|G1-352-w32|0.995695|0.999988|0.048076|0.976571|
|G2-200-w1|1.025692|0.987464|0.973514|1.078658|
|G2-200-w32|1.015441|0.987457|1.024805|1.079068|
|G2-352-w32|1.009748|0.986964|0.963915|1.061538|

Diagnostics follow all latency. Fixed panels cover each curve at 200w1,200w32,352w32, all three arms. Profiles run 2097152 rounds solo and 65536 at 32 workers; PMU uses one quarter. FIFO enable/disable brackets the timed interval. Counters are grouped user cycles,instructions,cache misses,branch misses, all at≥90% coverage. Profiles use cycles:u period 16000000 with 512 mmap pages.

All **18 profiles qualify with 119638 samples**, zero loss and all samples on assigned workers. At least 99.82% resolve to the measured native kernel or vector/scalar field inverse, with≤0.103% unknown. MMAP/FORK metadata verifies executable/PID/TID ownership and CPU assignments. Exact owned raw paths are chowned after recordings with hashes unchanged, before once-only decoding. Flat samples are not exclusive instruction cost, spill latency, memory-level attribution or removable proof time. Inadequate recordings would be retained rather than extended.

|Profile|Samples|Unknown share|Worker share|Native kernel/inverse share|
|---|---:|---:|---:|---:|
|cycles-G1-200-w1-OFF|3016|0.03316%|100.00000%|99.96684%|
|cycles-G1-200-w1-ON|3016|0.03316%|100.00000%|99.96684%|
|cycles-G1-200-w1-frozen|3013|0.06638%|100.00000%|99.90043%|
|cycles-G1-200-w32-OFF|3915|0.05109%|100.00000%|99.94891%|
|cycles-G1-200-w32-ON|3915|0.02554%|100.00000%|99.94891%|
|cycles-G1-200-w32-frozen|3920|0.10204%|100.00000%|99.82143%|
|cycles-G1-352-w32-OFF|6378|0.03136%|100.00000%|99.92161%|
|cycles-G1-352-w32-ON|6378|0.01568%|100.00000%|99.95296%|
|cycles-G1-352-w32-frozen|6452|0.03100%|100.00000%|99.96900%|
|cycles-G2-200-w1-OFF|5423|0.03688%|100.00000%|99.96312%|
|cycles-G2-200-w1-ON|5560|0.00000%|100.00000%|100.00000%|
|cycles-G2-200-w1-frozen|5426|0.03686%|100.00000%|99.96314%|
|cycles-G2-200-w32-OFF|7747|0.03872%|100.00000%|99.96128%|
|cycles-G2-200-w32-ON|7849|0.05096%|100.00000%|99.94904%|
|cycles-G2-200-w32-frozen|7702|0.03895%|100.00000%|99.93508%|
|cycles-G2-352-w32-OFF|13278|0.05272%|100.00000%|99.94728%|
|cycles-G2-352-w32-ON|13392|0.05227%|100.00000%|99.94773%|
|cycles-G2-352-w32-frozen|13258|0.02263%|100.00000%|99.97737%|

## Resources

All scratch capacities match across arms and before/after timing. G2 per-worker capacities are16000 bytes at 200 points and 28160 at 352, plus1280 bytes for 201/353 tails. Candidate maximum observed process RSS across this small caller matrix is18.75 MiB, including setup and threads. It is not full-proof peak RAM or minimum RAM. CPU-time/wall ratios and timed minor faults remain diagnostics.

|Context|OFF/ON process peak RSS MiB|OFF/ON median busy CPUs|OFF/ON minor faults per wave|
|---|---|---|---|
|G1-200-w1|2.75 / 2.75|1.0000 / 1.0000|0.000061035 / 0.000061035|
|G1-200-w16|3.75 / 3.75|15.5251 / 15.4213|0.000091553 / 0.000091553|
|G1-200-w32|5.0 / 5.0|31.5656 / 31.2925|0.000091553 / 0.000122070|
|G1-201-w1|2.75 / 2.75|1.0000 / 1.0000|0.000061035 / 0.000061035|
|G1-201-w16|3.75 / 3.75|15.5119 / 15.3870|0.000061035 / 0.000061035|
|G1-201-w32|5.0 / 5.0|31.5993 / 31.5133|0.000061035 / 0.000061035|
|G1-352-w1|2.5 / 2.75|1.0000 / 1.0000|0.000122070 / 0.000122070|
|G1-352-w16|5.25 / 5.5|15.5291 / 15.4913|0.000183105 / 0.000122070|
|G1-352-w32|8.5 / 8.5|31.4928 / 31.1245|0.000244141 / 0.000183105|
|G1-353-w1|3.0 / 3.0|1.0000 / 1.0000|0.000122070 / 0.000122070|
|G1-353-w16|5.5 / 5.5|15.5207 / 15.4647|0.000122070 / 0.000122070|
|G1-353-w32|8.75 / 8.75|31.4907 / 31.4691|0.000122070 / 0.000122070|
|G2-200-w1|2.75 / 2.75|1.0000 / 1.0000|0.000061035 / 0.000061035|
|G2-200-w16|5.0 / 5.0|15.6602 / 15.6212|0.000061035 / 0.000061035|
|G2-200-w32|7.5 / 7.75|31.6950 / 31.4261|0.000061035 / 0.000061035|
|G2-201-w1|2.75 / 2.75|1.0000 / 1.0000|0.000061035 / 0.000061035|
|G2-201-w16|5.25 / 5.0|15.6338 / 15.6156|0.000061035 / 0.000061035|
|G2-201-w32|7.75 / 7.75|31.5451 / 31.5168|0.000061035 / 0.000061035|
|G2-352-w1|3.0 / 3.0|1.0000 / 1.0000|0.000122070 / 0.000122070|
|G2-352-w16|9.0 / 9.25|15.6273 / 15.6067|0.000122070 / 0.000122070|
|G2-352-w32|14.75 / 15.0|31.2444 / 31.5442|0.000122070 / 0.000122070|
|G2-353-w1|3.0 / 3.0|1.0000 / 1.0000|0.000122070 / 0.000122070|
|G2-353-w16|10.5 / 10.5|15.6386 / 15.6208|0.000122070 / 0.000122070|
|G2-353-w32|18.75 / 18.75|31.5007 / 31.5274|0.000122070 / 0.000122070|

## Accepted proof baseline, unchanged

Inherited local 32-worker full-proof timings and supplied M5 references follow. No new proof/SDK/AWS/Aeglos comparison is added. SDK6d83/source176 and ledger30846 remain current. Inherited 132.75 MiB RSS is neither a new peak nor minimum RAM.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU 10× limit ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37|24|3.7|
|2×2|11.151932|59|30|5.9|
|4×2|16.086386|109|48|10.9|
|8×2|24.179453|189|74|18.9|
|12×2|33.884455|284|101|28.4|
|16×2|40.077381|335|124|33.5|
|24×2|65.850777|528|176|52.8|
|merge24|65.060937|507|175|50.7|

Accepted merge54 remains 161.883899 ms without a supplied M5 reference. The all-eight CPU 10× goal remains active and unmet (0/8). This stage adds no portable/device, huge-circuit, forced-pressure, swap, minimum-RAM or concurrent-proof qualification.

## Artifacts and continuation

The [decision](artifacts/decision.md) closes this implementation. The [next step](artifacts/next.md) calls for whole-path dependency/resource accounting before another candidate, using saved evidence without an unchanged retry.

[Analysis](artifacts/analysis.json), [independent review](artifacts/review.json), [protocol](artifacts/protocol.md), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [identities](artifacts/identities.json), [symbols](artifacts/profile-symbols.json), [host](artifacts/host-public.json), [helpers/receipts](artifacts/helpers-and-receipts.json.gz), [raw index](artifacts/raw-profile-index.json), [decoded-stream index](artifacts/profile-stream-index.json), [publication](artifacts/publication.json) and [checksums](SHA256SUMS) preserve all arms and failures. Full recordings/decoded streams remain local under `/home/ubuntu/src/zopro-perf-work/g2-validated-capture176/`. The selected UTF-8 archive excludes executable/library/key/witness/proof blobs; host-public omits the unrelated process list while the original remains locally pinned.
