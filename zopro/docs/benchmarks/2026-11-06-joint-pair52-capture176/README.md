# Joint complex arithmetic: complete-caller comparison

**Rejected: the G2 caller is slower across all measured contexts.** The four-size 32-worker G2 aggregate takes **62.79% longer than fresh OFF** and 63.37% longer than the frozen accepted-source caller. Both aggregates have 0/15 winning blocks. This follows the [correctness/codegen qualification](../2026-11-06-joint-pair52-callers176/README.md); fewer static IFMAs did not yield faster execution. No proof speedup or production change is claimed.

## Results and limitations

The preregistered gate fails **39/76 checks**:4 calibration,31 nonregression and 4 gain checks. All 12 G2 OFF/frozen calibration intervals pass, and every G2 ON/OFF and ON/frozen context regresses. Four unchanged G1 32-worker calibration intervals fail; those failures remain in the result, so the campaign is not globally calibrated. These G1 failures do not explain the much larger G2 losses also observed against fresh OFF and in single-worker runs. No favorable subset, reroll or threshold change replaces the result.

Each row reports median elapsed ns per complete batch wave, with all workers active. It is neither aggregate throughput nor full-proof latency. Paired ratios need not equal ratios of the displayed medians.

|Context|Frozen ns/wave|OFF ns/wave|ON ns/wave|ON/OFF ratio [interval], wins|ON/frozen ratio [interval], wins|OFF/frozen ratio [interval], wins|
|---|---:|---:|---:|---|---|---|
|G1-200-w1|4257.080|4263.453|4257.060|1.000480429 [0.997428539, 1.001025655], 6/15|1.000916665 [0.998170244, 1.001620151], 6/15|1.000767987 [0.999125691, 1.002254472], 4/15|
|G1-200-w16|4842.504|4844.999|4846.267|1.000794194 [0.994994491, 1.002488401], 7/15|1.000492040 [0.995449289, 1.004008162], 7/15|0.999921254 [0.996163522, 1.003214318], 8/15|
|G1-200-w32|6952.609|6990.011|6991.340|0.993455728 [0.921323900, 1.002117327], 10/15|0.998417656 [0.950835630, 1.112608738], 8/15|1.007863533 [0.897598572, 1.193021321], 6/15|
|G1-201-w1|4381.782|4393.914|4378.481|0.997256971 [0.995240226, 1.000751871], 10/15|0.999321526 [0.997289208, 1.002637350], 8/15|1.001664711 [0.999408285, 1.005778459], 4/15|
|G1-201-w16|4972.977|4985.044|4980.069|0.999686550 [0.995057548, 1.011100817], 8/15|1.001501950 [0.996980090, 1.013419490], 6/15|1.002171994 [1.000481933, 1.004723939], 3/15|
|G1-201-w32|7116.146|8423.481|7126.348|0.929707083 [0.833958710, 1.002689561], 11/15|0.998726850 [0.952684877, 1.004047622], 8/15|1.014536418 [1.000685831, 1.198452058], 3/15|
|G1-352-w1|7011.650|7007.860|7003.966|0.999562422 [0.998247176, 1.001876992], 8/15|1.000332628 [0.997578050, 1.001379446], 6/15|0.999603356 [0.998066793, 1.001761857], 8/15|
|G1-352-w16|7993.994|8001.741|8025.125|1.001135294 [0.999293606, 1.004547399], 4/15|1.003550912 [1.000520582, 1.004332504], 2/15|1.001225155 [0.997692151, 1.002756307], 5/15|
|G1-352-w32|14598.042|11618.318|14378.932|1.210479781 [0.990427928, 1.250271836], 6/15|0.989562300 [0.976819937, 1.210097224], 10/15|0.987386233 [0.791412398, 1.005456481], 10/15|
|G1-353-w1|7118.369|7123.653|7116.116|0.998702250 [0.997923838, 1.000317302], 11/15|1.000098228 [0.998380679, 1.000983231], 7/15|1.000907154 [0.999503393, 1.002474690], 5/15|
|G1-353-w16|8121.378|8130.578|8122.012|0.998764643 [0.995741332, 1.000797451], 10/15|0.999591149 [0.997537819, 1.001558159], 8/15|1.002209792 [0.999210475, 1.003126498], 4/15|
|G1-353-w32|11850.387|11856.734|14592.007|1.000569310 [0.987300349, 1.224610617], 7/15|0.998358932 [0.988191502, 1.237731600], 8/15|0.998336486 [0.988409122, 1.249836122], 8/15|
|G2-200-w1|7753.161|7744.184|17292.854|2.233330288 [2.227496297, 2.235511290], 0/15|2.231644069 [2.228041014, 2.235245856], 0/15|0.998634177 [0.997518447, 1.001968600], 9/15|
|G2-200-w16|8969.286|8982.033|19267.537|2.143638952 [2.135738047, 2.148954287], 0/15|2.147916781 [2.145476362, 2.150233490], 0/15|1.001451060 [0.999023948, 1.003846633], 4/15|
|G2-200-w32|14419.307|14419.359|23679.944|1.641017503 [1.574177558, 1.663627117], 0/15|1.639686302 [1.582467193, 1.651149888], 0/15|1.000611027 [0.995582520, 1.005623548], 7/15|
|G2-201-w1|8191.798|8301.476|17599.467|2.120287350 [2.117836775, 2.122883398], 0/15|2.148833851 [2.139951112, 2.151908208], 0/15|1.014061044 [1.010913498, 1.015650600], 0/15|
|G2-201-w16|9476.986|9590.950|19591.593|2.040329146 [2.036225918, 2.045414522], 0/15|2.066648094 [2.061190803, 2.069598574], 0/15|1.012260371 [1.010843977, 1.014346232], 0/15|
|G2-201-w32|14969.829|15226.700|24061.469|1.580069182 [1.574216632, 1.596187464], 0/15|1.606653426 [1.541635471, 1.615214099], 0/15|1.012540067 [0.976369307, 1.022630002], 6/15|
|G2-352-w1|13124.975|13118.471|29968.995|2.285040674 [2.279597498, 2.288079662], 0/15|2.281239337 [2.277178095, 2.287782777], 0/15|0.999456770 [0.998013441, 1.000506878], 10/15|
|G2-352-w16|15259.792|15294.613|33618.062|2.197638394 [2.195078518, 2.199490825], 0/15|2.201978346 [2.199861442, 2.204985251], 0/15|1.002189807 [1.001231395, 1.003465204], 2/15|
|G2-352-w32|24782.468|24888.296|41222.545|1.654125917 [1.637538928, 1.666325741], 0/15|1.658639171 [1.639724116, 1.665964531], 0/15|0.999688200 [0.997287930, 1.004270284], 8/15|
|G2-353-w1|13518.054|13638.754|30208.355|2.214032163 [2.208868769, 2.215873903], 0/15|2.234486916 [2.232140927, 2.238214923], 0/15|1.010082262 [1.007833500, 1.011365958], 0/15|
|G2-353-w16|15712.770|15856.127|33653.083|2.122095271 [2.118347663, 2.123139010], 0/15|2.143121455 [2.139232545, 2.144902818], 0/15|1.009601122 [1.007747717, 1.011778194], 0/15|
|G2-353-w32|25490.174|25410.377|41796.192|1.641117647 [1.625963620, 1.645123480], 0/15|1.637983065 [1.627476881, 1.641742857], 0/15|0.995926483 [0.993150552, 1.002954634], 9/15|

|Aggregate|ON/control ratio [interval], wins|
|---|---|
|ALL/OFF|1.407892372 [1.390889497, 1.415743942], 0/15|
|ALL/frozen|1.409868134 [1.398883663, 1.418485668], 0/15|
|G2/OFF|1.627890593 [1.609790688, 1.637131772], 0/15|
|G2/frozen|1.633675969 [1.608578482, 1.638599870], 0/15|

All failed checks:

- `G1-200-w32/OFF/frozen/calibration`
- `G1-200-w32/ON/frozen/nonregression`
- `G1-201-w16/ON/frozen/nonregression`
- `G1-201-w16/ON/OFF/nonregression`
- `G1-201-w32/OFF/frozen/calibration`
- `G1-352-w32/OFF/frozen/calibration`
- `G1-352-w32/ON/frozen/nonregression`
- `G1-352-w32/ON/OFF/nonregression`
- `G1-353-w32/OFF/frozen/calibration`
- `G1-353-w32/ON/frozen/nonregression`
- `G1-353-w32/ON/OFF/nonregression`
- `G2-200-w1/ON/frozen/nonregression`
- `G2-200-w1/ON/OFF/nonregression`
- `G2-200-w16/ON/frozen/nonregression`
- `G2-200-w16/ON/OFF/nonregression`
- `G2-200-w32/ON/frozen/nonregression`
- `G2-200-w32/ON/OFF/nonregression`
- `G2-201-w1/ON/frozen/nonregression`
- `G2-201-w1/ON/OFF/nonregression`
- `G2-201-w16/ON/frozen/nonregression`
- `G2-201-w16/ON/OFF/nonregression`
- `G2-201-w32/ON/frozen/nonregression`
- `G2-201-w32/ON/OFF/nonregression`
- `G2-352-w1/ON/frozen/nonregression`
- `G2-352-w1/ON/OFF/nonregression`
- `G2-352-w16/ON/frozen/nonregression`
- `G2-352-w16/ON/OFF/nonregression`
- `G2-352-w32/ON/frozen/nonregression`
- `G2-352-w32/ON/OFF/nonregression`
- `G2-353-w1/ON/frozen/nonregression`
- `G2-353-w1/ON/OFF/nonregression`
- `G2-353-w16/ON/frozen/nonregression`
- `G2-353-w16/ON/OFF/nonregression`
- `G2-353-w32/ON/frozen/nonregression`
- `G2-353-w32/ON/OFF/nonregression`
- `G2-aggregate/ON/frozen/gain`
- `G2-aggregate/ON/OFF/gain`
- `ALL-aggregate/ON/frozen/gain`
- `ALL-aggregate/ON/OFF/gain`

## Measurement boundary

The [protocol](artifacts/protocol.md), Rust campaign and independent analyzer were frozen before the first observation. Twenty-four contexts cover G1/G2, 200/201/352/353 points and 1/16/32 workers, with frozen/fresh OFF/ON executables. The frozen control is an accepted-source diagnostic caller, distinct from the production SDK. Fresh executables explicitly link their qualified coherent field/core libraries; qualification tests and builds were inherited without repetition.

Every worker uses a distinct public-generator fixture, seed19+worker. These are not arbitrary full-width coordinate fixtures; native field qualification separately covers full-width/lazy values. Three128-round priming/check passes and reset precede the clock. All 1188 callers pass independent generator oracles, cross-arm output hashes, equal scratch capacities and zero timed allocations. The diagnostic Rayon pool uses 512 KiB stacks, CPU2 solo, physical cores0–15 for 16 workers and CPUs0–31 for 32. Timing includes pool dispatch; final checks, digest and JSON are outside the clock. No production pool, lookup table, key copy or retained plan is added.

Fifteen fixed paired blocks rotate/reverse all contexts and arms; rounds per worker are32768 at 200/201 and 16384 at 352/353. All 1279 commands succeed first attempt:72 preflights,1 checker,1080 latency,18 PMU,18 cycles profiles and 90 offline commands. No competing owned compute overlaps capture. Frequency is not locked.

Intervals use sorted zero-based ranks3/7/11 of 15 paired ratios and are marginal, not simultaneous family-wise guarantees. All 24 calibration intervals must lie within[0.97, 1.03]; all 48 ON/control nonregression intervals require upper≤1.01. G2 logical32 across all four sizes and the all 24-context aggregate each require upper<1 and≥11/15 wins against both controls, without a positive-gain floor. Unchanged G1 requires nonregression. All 76 checks and their raw ratios are retained and independently reconstructed.

## CPU evidence

Complete G2 static IFMA sites fall 15.17%, but instruction rows rise 8.41% and vector stack-addressed rows rise 23.22%. Grouped counters show dynamic instructions rising 18.37–19.07% and cycles rising 80.12–129.25% in the G2 panels. Instructions per cycle therefore decline substantially. Larger live state, memory operations and serial carry/reduction dependencies are plausible contributors; flat profiles and these counters do not isolate their causal costs. Solo cache misses are very low in both builds, so a large cache-miss ratio alone would misdescribe this loss. G1 counter instructions are essentially unchanged.

|Context|ON/OFF cycles|ON/OFF instructions|ON/OFF cache misses|ON/OFF branch misses|
|---|---:|---:|---:|---:|
|G1-200-w1|1.001466|1.000000|0.485685|1.076032|
|G1-200-w32|1.011709|0.999928|31.303053|1.013403|
|G1-352-w32|0.999579|1.000002|1.236705|0.981968|
|G2-200-w1|2.292477|1.183747|0.855556|1.064612|
|G2-200-w32|1.801197|1.183734|1.258574|1.035346|
|G2-352-w32|1.819490|1.190672|1.759997|0.998390|

Diagnostics follow all latency runs. FIFO control brackets counters and cycles:u profiles at fixed period 16000000, 512 mmap pages; solo profiles use2097152 rounds and 32-worker profiles65536, with counters at one quarter. All grouped counter events have≥90% coverage. All 18 profiles qualify with **143325 samples**, zero loss,100% on assigned workers,≥99.81% in native kernel/inverse functions and≤0.104% unknown. MMAP/FORK metadata validates executable, PID/TID and CPU ownership. Flat sampled periods are not exclusive cycles or removable proof time. Recordings are not extended. Exact owned raw paths are chowned before decoding with hashes unchanged; each sample/metadata/attribute/loss decode runs once.

|Profile|Samples|Unknown share|Worker share|Target kernel/inverse share|
|---|---:|---:|---:|---:|
|cycles-G1-200-w1-OFF|3015|0.00000%|100.00000%|100.00000%|
|cycles-G1-200-w1-ON|3014|0.00000%|100.00000%|100.00000%|
|cycles-G1-200-w1-frozen|3016|0.03316%|100.00000%|99.90053%|
|cycles-G1-200-w32-OFF|3862|0.10357%|100.00000%|99.81875%|
|cycles-G1-200-w32-ON|3921|0.00000%|100.00000%|99.97450%|
|cycles-G1-200-w32-frozen|3882|0.07728%|100.00000%|99.87120%|
|cycles-G1-352-w32-OFF|6463|0.06189%|100.00000%|99.92264%|
|cycles-G1-352-w32-ON|6457|0.03097%|100.00000%|99.93805%|
|cycles-G1-352-w32-frozen|6371|0.06278%|100.00000%|99.93722%|
|cycles-G2-200-w1-OFF|5432|0.05523%|100.00000%|99.94477%|
|cycles-G2-200-w1-ON|12410|0.03223%|100.00000%|99.96777%|
|cycles-G2-200-w1-frozen|5423|0.00000%|100.00000%|100.00000%|
|cycles-G2-200-w32-OFF|7704|0.05192%|100.00000%|99.94808%|
|cycles-G2-200-w32-ON|13986|0.05720%|100.00000%|99.94280%|
|cycles-G2-200-w32-frozen|7704|0.03894%|100.00000%|99.96106%|
|cycles-G2-352-w32-OFF|13271|0.04521%|100.00000%|99.95479%|
|cycles-G2-352-w32-ON|24129|0.04144%|100.00000%|99.95441%|
|cycles-G2-352-w32-frozen|13265|0.07539%|100.00000%|99.90954%|

## Resources

Scratch capacities are identical across arms after every timed call. G2 capacity per worker is16000 bytes at 200 points and 28160 at 352, plus 1280 for the201/353 scalar tails. Maximum observed candidate process RSS is18.5 MiB across this small caller matrix, including setup and threads. It is not a proof memory peak or minimum RAM. G2 busy CPU counts are near 1 solo and 31 at 32 workers. Timed minor faults remain very small; this experiment does not establish a RAM-capacity or page-fault bottleneck.

|Context|OFF/ON process peak RSS MiB|OFF/ON median busy CPUs|OFF/ON minor faults per wave|
|---|---|---|---|
|G1-200-w1|2.75 / 2.75|1.0000 / 1.0000|0.000061035 / 0.000061035|
|G1-200-w16|3.75 / 3.75|15.5227 / 15.5374|0.000061035 / 0.000061035|
|G1-200-w32|5.0 / 5.0|31.3168 / 31.2659|0.000091553 / 0.000091553|
|G1-201-w1|2.75 / 2.75|1.0000 / 1.0000|0.000061035 / 0.000061035|
|G1-201-w16|3.75 / 3.75|15.5241 / 15.5109|0.000061035 / 0.000061035|
|G1-201-w32|5.0 / 5.0|26.8624 / 31.5842|0.000061035 / 0.000061035|
|G1-352-w1|2.75 / 2.75|1.0000 / 1.0000|0.000122070 / 0.000122070|
|G1-352-w16|5.5 / 5.5|15.5369 / 15.4906|0.000122070 / 0.000122070|
|G1-352-w32|8.5 / 8.25|31.3435 / 25.4817|0.000183105 / 0.000183105|
|G1-353-w1|3.0 / 3.0|1.0000 / 1.0000|0.000122070 / 0.000122070|
|G1-353-w16|5.5 / 5.75|15.5393 / 15.5329|0.000122070 / 0.000122070|
|G1-353-w32|8.25 / 8.75|31.2511 / 25.6431|0.000122070 / 0.000122070|
|G2-200-w1|2.75 / 2.75|1.0000 / 1.0000|0.000061035 / 0.000061035|
|G2-200-w16|5.0 / 5.0|15.6535 / 15.5207|0.000061035 / 0.000061035|
|G2-200-w32|7.75 / 7.75|31.6618 / 31.2811|0.000061035 / 0.000122070|
|G2-201-w1|2.75 / 2.75|1.0000 / 1.0000|0.000061035 / 0.000061035|
|G2-201-w16|5.0 / 5.0|15.6179 / 15.5191|0.000061035 / 0.000061035|
|G2-201-w32|7.75 / 7.75|31.6940 / 31.3594|0.000061035 / 0.000061035|
|G2-352-w1|3.0 / 3.0|1.0000 / 1.0000|0.000122070 / 0.000122070|
|G2-352-w16|9.25 / 9.75|15.6215 / 15.4110|0.000122070 / 0.000122070|
|G2-352-w32|15.25 / 17.0|31.4380 / 30.9821|0.000122070 / 0.000244141|
|G2-353-w1|3.0 / 3.0|1.0000 / 1.0000|0.000122070 / 0.000122070|
|G2-353-w16|10.5 / 10.5|15.6326 / 15.5240|0.000122070 / 0.000122070|
|G2-353-w32|18.75 / 18.5|31.7831 / 30.9387|0.000122070 / 0.000122070|

## Accepted proof baseline, unchanged

These are inherited local 32-worker full-proof timings, not new observations. SDK6d83/source176 and ledger30846 remain unchanged. Inherited132.75 MiB RSS is neither a new peak nor minimum RAM. No new SDK, proof, Gnark, AWS, Aeglos, large-circuit, phone, pressure or swap run is selected by this rejected candidate.

|Shape|M5 CPU ms|M5 Metal ms|Accepted local CPU ms|CPU 10× limit ms|
|---|---:|---:|---:|---:|
|1×2|37|24|7.928660|3.7|
|2×2|59|30|11.151932|5.9|
|4×2|109|48|16.086386|10.9|
|8×2|189|74|24.179453|18.9|
|12×2|284|101|33.884455|28.4|
|16×2|335|124|40.077381|33.5|
|24×2|528|176|65.850777|52.8|
|merge24|507|175|65.060937|50.7|

Accepted merge54 remains 161.883899 ms, without a supplied M5 reference. The all-eight CPU 10× goal remains active and unmet (0/8).

## Decision and artifacts

The [decision](artifacts/decision.md) closes this implementation. The [next step](artifacts/next.md) uses saved instruction-address samples and decoded consumers to study the IPC regression before selecting a genuinely different whole-path hypothesis. No successful capture, decode, native suite, build or model is repeated for analysis or publication.

[Analysis](artifacts/analysis.json), [independent review](artifacts/review.json), [protocol](artifacts/protocol.md), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [identities](artifacts/identities.json), [source audit](artifacts/qualification-source-audit.json), [symbols](artifacts/profile-symbols.json), [helpers/receipts](artifacts/helpers-and-receipts.json.gz), [raw index](artifacts/raw-profile-index.json), [decoded-stream index](artifacts/profile-stream-index.json) and [publication](artifacts/publication.json) preserve all arms and failures. Full raw/decoded streams remain local under `/home/ubuntu/src/zopro-perf-work/joint-pair52-capture176/`. The archive contains selected UTF-8 sources and receipts, excluding executable/library/key/witness/proof blobs. Host-public omits the unrelated process list; original host data remains pinned locally.
