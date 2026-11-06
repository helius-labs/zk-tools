# G1 binary tree-prefix complete-caller comparison

The native binary tree-prefix record route was compared with frozen and fresh controls in 27 unweighted/weighted contention contexts. **The predeclared gate did not pass: ten failed checks.** Large reused groups improved and used less scratch, but several small 32-worker contexts did not establish calibration or nonregression. No production change or full-proof speedup is claimed.

The binary tree-prefix record candidate is **not adopted**. Independent review reproduced all 87 timing checks and all ten failures from the frozen run. Every command and oracle completed successfully; all 15 counter panels and 15 profiles qualified.

The large, reused 35,973-point reduction improves substantially: ON/OFF is 0.437845 [0.431625, 0.442469] with 16 workers and 0.625835 [0.622544, 0.626724] with 32 workers, winning all 15 pairs in both cases. The four-main 32-worker geometric ratio is 0.877792 [0.860778, 0.885487], also 15 wins. These are complete reduction callers with duplicated public fixtures, not single-proof measurements.

Five failures are OFF/frozen calibration at 32 workers (tile257, tile384, tile511, weighted128 and weighted256). The other five are nonregression upper bounds for tile257 and tile384 against both controls and weighted256 against frozen. Their ON/control median ratios are below one, but their intervals extend above the declared 1.01 bound. This is insufficient evidence of nonregression, rather than a demonstrated median slowdown. No favorable subset, extra blocks or unchanged reroll is used to rescue the result.

Large-group retained capacity falls from 6,162,168 to 4,004,088 bytes per worker. Maximum observed process RSS falls from 178.25 to 123 MiB at 16 workers and from 355 to 243.5 MiB at 32. Both 32-worker arms use about 31 busy CPUs and incur a median 0.125 minor faults per wave. PMU cycles per wave fall 37.34%, cache misses fall 13.01%, and instructions increase 6.04%. The large gain is consistent with reduced scratch traffic and working set despite more instructions; this does not identify a particular cache level or prove DRAM bandwidth causality. It does not arise from reducing faults in this run.

The qualified H-only ternary experiment and this binary experiment use different arithmetic/record organizations and different harnesses; their historical ratios are not a paired ranking. Neither is adopted. The record approach does not add a key-dependent lookup table, duplicated proving key or execution plan.

Before the first performance capture, a source audit replaced a benchmark-only shared atomic allocation counter with per-worker TLS counters. The original preparation artifacts were retained; only changed consumers/harness were built again. All captured binaries use the TLS version. Counters sum worker allocation calls and exclude main/coordinator allocations; they do not measure allocation bytes or whole-process allocation count. Weighted production allocations are timed, but their dropped capacities are unknown, not zero. The existing typed source forecasts are reported separately and are not measured RAM.

No SDK, proof export, Gnark verification, AWS/Aeglos comparison, minimum-RAM, swap, phone/ARM/TEE or proof-throughput measurement was added. Accepted production sources, the accepted SDK, the 30,846-entry proof ledger and all eight accepted CPU/Metal comparisons remain unchanged. The full goal remains active and unmet for all eight shapes.


## Implementation and measurement boundary

The [native/portable qualification](../2026-11-06-g1-tree-records176/README.md) is inherited without rerunning its successful kernel suites. The generic binary addition formulas and checked batch inverse are unchanged. Native G1 stores prefix/norm values in two SIMD vectors for every full group, rereads coordinates during the reverse pass and keeps the old five-vector Group allocation empty. Exact reservation avoids the allocator's minimum record capacity. G2 retains its existing threshold/reservation; portable G1 retains its original route. This is a scratch lifetime change, not a key lookup table. It is distinct from the earlier streaming-prefix and ternary experiments.

A separate private caller snapshot adds a weighted adapter with prebuilt bases and stored projective output; normalization and oracle checks happen outside the clock. All 177 qualified sources are recovered exactly after removing two adapter appendices; all 176 accepted production sources remain exact. OFF and ON share that candidate snapshot and differ by the experimental cfg. Frozen uses accepted production bodies plus the same private adapters. Three dependent core libraries were needed for the new adapter; unchanged field/support libraries were reused. The original unweighted TreeCaller layout and body are exact. All final consumers use O3/native/fatLTO/one CGU.

Preparation completed 14 commands: three core/wrapper-oracle/driver sequences and two harness commands. Each arm passed 88 new weighted wrapper Arkworks cases including full 254-bit values, exceptions, multiple/partial chunks and repeated calls. The harness ran 16 contexts at 1/4/16/32 workers with 512 KiB stacks and zero timing observations. Its three tests comprise one new broadcast test and two existing FIFO tests. Before capture, source review found that a shared atomic allocation counter would introduce artificial contention. Original preparation artifacts were kept intact; five changed TLS consumer/harness commands passed. All performance captures use those final TLS binaries. No unchanged core or original kernel suite was rebuilt/rerun for this correction. The initial preparation reader's expected harness count was corrected from one to three; retained originals and the actual outputs are archived.

The benchmark-only Rayon pool uses broadcast callbacks with private fixtures pinned to workers. Public weighted_affine executes inside that pool. A coordinator and main thread control ready/start/finish/release barriers, and checks/serialization happen after release. This avoids routing foreign worker calls through a single global Rayon worker. No production pool is added. The allocation counter sums per-worker TLS deltas, excluding main/coordinator allocations.

## Frozen protocol and all timing results

The Rust/xtask plan was frozen before any new preflight timing. All 1,402 commands completed successfully once: 81 preflight callers and one checker; 1,215 latency callers (27 contexts × three arms × 15 paired blocks); 15 counter panels; 15 fixed-period profiles; and 75 offline ownership/decode/metadata/event/loss steps. Independent review reconstructed 1,326 caller outputs, 87 timing checks and 155,095 profile samples. No pilot, added blocks, favorable subset or unchanged reroll was used.

Every worker primes 24 calls. Tile257/384/511/2048 use fresh buffers; group35973/groups31x256/tails reuse buffers. Weighted128/256 call the actual public function with its allocations inside the clock, but base construction and normalization outside. Solo pins CPU2, 16 workers use physical cores 0–15 and 32 workers use all SMT threads 0–31. Workers duplicate complete public fixtures; ns/wave is elapsed time divided by rounds per worker, not aggregate calls or proof latency. This is contention evidence, not proof-level scaling or proof throughput.

Ratios are paired medians with intervals using sorted zero-based ranks 3/7/11 of 15 and wins. The intervals have marginal coverage, not simultaneous family-wise coverage. OFF/frozen calibration requires the whole interval in [0.97,1.03] in all 27 contexts. ON/OFF and ON/frozen nonregression require upper ≤1.01 in every context, including weighted. Large-group gains at both 16 and 32 workers and the four-main 32-worker geometric aggregate require upper <1 and at least 11 wins against both controls. There is no minimum gain floor.

|Case|Workers|Frozen ns/wave|OFF ns/wave|ON ns/wave|ON/OFF [interval], wins|ON/frozen [interval], wins|OFF/frozen calibration|
|---|---:|---:|---:|---:|---|---|---|
|tile257|1|12737.60|12737.57|12576.59|0.992695 [0.980992, 0.997407], 12/15|0.990656 [0.977539, 0.997776], 12/15|0.996173 [0.989185, 1.009852], 9/15|
|tile257|16|13925.52|13905.77|13735.46|0.990367 [0.985411, 0.991779], 14/15|0.986130 [0.980668, 0.990805], 14/15|0.998580 [0.988797, 1.001801], 9/15|
|tile257|32|20130.84|19845.54|19578.26|0.997274 [0.981575, 1.014885], 9/15|0.980285 [0.961766, 1.012702], 8/15|0.978068 [0.955022, 1.022572], 10/15|
|tile384|1|16231.87|16243.33|16093.85|0.993845 [0.983488, 0.999020], 13/15|0.988056 [0.983753, 0.996644], 12/15|0.998215 [0.991409, 1.004892], 9/15|
|tile384|16|18018.40|17983.97|17822.29|0.988700 [0.979027, 0.992213], 13/15|0.991749 [0.984442, 0.997700], 13/15|1.000649 [0.994868, 1.013221], 5/15|
|tile384|32|25241.14|26004.51|25511.83|0.979319 [0.943166, 1.017131], 10/15|0.992723 [0.963553, 1.042898], 8/15|1.013006 [0.987432, 1.053334], 6/15|
|tile511|1|19304.43|19344.22|19194.30|0.991832 [0.987467, 0.998911], 12/15|0.989409 [0.983737, 0.998911], 13/15|0.996446 [0.994073, 1.007762], 9/15|
|tile511|16|21418.30|21420.08|21130.17|0.990474 [0.981440, 0.997108], 13/15|0.987125 [0.984726, 0.989603], 14/15|0.996353 [0.989304, 1.003989], 10/15|
|tile511|32|30579.70|30853.64|29867.98|0.970494 [0.955433, 0.985391], 14/15|0.972898 [0.951469, 0.994756], 12/15|1.002177 [0.992533, 1.031232], 6/15|
|tile2048|1|56195.43|56304.34|56152.22|0.998462 [0.992887, 1.004524], 8/15|1.000331 [0.991343, 1.002870], 7/15|0.997403 [0.995428, 1.004693], 9/15|
|tile2048|16|64370.19|64450.19|63626.17|0.986527 [0.980460, 0.991329], 14/15|0.990114 [0.982360, 0.998360], 12/15|1.000887 [0.995707, 1.012044], 6/15|
|tile2048|32|97388.98|97116.95|95433.22|0.983956 [0.961912, 1.006876], 11/15|0.981072 [0.970149, 0.998321], 12/15|1.003645 [0.978150, 1.020219], 6/15|
|group35973|1|935275.00|933718.38|908833.25|0.975832 [0.970465, 0.978219], 15/15|0.973086 [0.967675, 0.975507], 15/15|0.996683 [0.994989, 0.998695], 12/15|
|group35973|16|6506743.62|6477100.88|2828568.62|0.437845 [0.431625, 0.442469], 15/15|0.434386 [0.429961, 0.443175], 15/15|0.994044 [0.982805, 1.002698], 11/15|
|group35973|32|24765487.50|24679331.69|15438358.00|0.625835 [0.622544, 0.626724], 15/15|0.623352 [0.619548, 0.624756], 15/15|0.995756 [0.993954, 1.002740], 11/15|
|groups31x256|1|198501.02|198469.25|193649.20|0.976157 [0.970098, 0.977471], 15/15|0.973283 [0.969721, 0.975926], 15/15|0.996676 [0.993181, 1.002342], 9/15|
|groups31x256|16|228807.27|228485.41|220348.64|0.965742 [0.961231, 0.974229], 15/15|0.964755 [0.956185, 0.974667], 15/15|0.996906 [0.992645, 1.004178], 9/15|
|groups31x256|32|408691.73|409600.64|376634.98|0.918450 [0.900750, 0.925420], 15/15|0.919129 [0.889673, 0.934851], 15/15|0.998427 [0.990413, 1.017028], 8/15|
|tails|1|11867.22|11852.95|11679.08|0.986898 [0.976323, 0.996741], 12/15|0.982040 [0.976141, 0.998722], 14/15|0.991621 [0.987055, 1.007365], 9/15|
|tails|16|12942.08|12927.93|12786.12|0.989342 [0.988327, 0.991763], 15/15|0.987658 [0.986060, 0.989033], 14/15|0.998770 [0.995445, 1.000468], 8/15|
|tails|32|18465.23|18506.97|17759.43|0.956147 [0.933808, 0.988228], 14/15|0.964929 [0.940072, 0.972330], 15/15|1.003954 [0.973089, 1.029564], 7/15|
|weighted128|1|13340.58|13313.01|13267.19|0.996629 [0.990158, 1.004825], 10/15|0.996036 [0.985249, 1.006884], 10/15|0.997363 [0.988553, 1.006471], 8/15|
|weighted128|16|14663.79|14651.60|14605.76|0.997354 [0.994882, 0.999267], 13/15|0.996112 [0.994469, 1.002124], 11/15|0.999965 [0.998404, 1.006809], 8/15|
|weighted128|32|21049.50|21152.39|20822.16|0.988247 [0.963899, 0.996928], 12/15|0.989628 [0.969030, 1.002447], 11/15|1.005323 [0.977271, 1.033511], 7/15|
|weighted256|1|19204.63|19210.73|19174.74|0.998943 [0.992559, 1.004689], 9/15|0.995381 [0.987853, 1.004021], 10/15|0.996958 [0.989621, 1.008568], 9/15|
|weighted256|16|21396.66|21420.01|21383.94|1.001684 [0.997955, 1.008546], 5/15|0.999489 [0.992085, 1.001702], 8/15|0.997449 [0.992454, 1.002848], 11/15|
|weighted256|32|31327.81|32312.53|30513.96|0.966352 [0.930428, 1.004318], 11/15|0.985973 [0.935922, 1.018493], 11/15|1.004572 [0.985279, 1.036241], 7/15|

Four-main aggregate (tile384, tile511, tile2048 and group35973) at 32 workers:

- ON/OFF: 0.877792 [0.860778, 0.885487], 15/15.
- ON/frozen: 0.875627 [0.868495, 0.887224], 15/15.

All failed checks:

- `tile257-w32/OFF/frozen/calibration`
- `tile257-w32/ON/OFF/nonregression`
- `tile257-w32/ON/frozen/nonregression`
- `tile384-w32/OFF/frozen/calibration`
- `tile384-w32/ON/OFF/nonregression`
- `tile384-w32/ON/frozen/nonregression`
- `tile511-w32/OFF/frozen/calibration`
- `weighted128-w32/OFF/frozen/calibration`
- `weighted256-w32/OFF/frozen/calibration`
- `weighted256-w32/ON/frozen/nonregression`

## Resources

Unweighted capacity totals sum common points/counts/output, pair indices, legacy records, prefix records and fixed structs for one worker. All block/worker capacities match the preflights. Weighted production scratch is dropped inside the public call, so actual retained capacity is **unknown**, not zero; its unchanged typed source forecast is listed separately, not as measured memory. Reused cases perform zero measured worker allocations. Maximum RSS covers whole-process setup and workers across the 15 latency blocks and is not minimum RAM. Full three-arm capacities, CPU times, allocation counts, forecasts, RSS and faults are in the analysis.

|Case|Workers|OFF/ON capacity B per worker|OFF/ON source forecast B|OFF/ON worker allocations per round|OFF/ON max RSS MiB|OFF/ON busy CPUs|OFF/ON minor faults per wave|
|---|---:|---|---|---|---|---|---|
|tile257|1|44944 / 29584|— / —|11.000 / 11.000|2.500 / 2.250|1.000 / 1.000|0.003 / 0.003|
|tile257|16|44944 / 29584|— / —|11.000 / 11.000|2.250 / 2.250|15.467 / 15.502|0.002 / 0.002|
|tile257|32|44944 / 29584|— / —|11.000 / 11.000|2.500 / 2.250|29.843 / 30.185|0.002 / 0.002|
|tile384|1|66896 / 43856|— / —|12.000 / 12.000|2.500 / 2.500|1.000 / 1.000|0.003 / 0.003|
|tile384|16|66896 / 43856|— / —|12.000 / 12.000|2.250 / 2.500|15.490 / 15.491|0.002 / 0.002|
|tile384|32|66896 / 43856|— / —|12.000 / 12.000|3.000 / 2.750|29.726 / 30.256|0.002 / 0.002|
|tile511|1|86224 / 56464|— / —|12.000 / 12.000|2.500 / 2.500|1.000 / 1.000|0.002 / 0.003|
|tile511|16|86224 / 56464|— / —|12.000 / 12.000|2.500 / 2.250|15.380 / 15.454|0.002 / 0.002|
|tile511|32|86224 / 56464|— / —|12.000 / 12.000|3.250 / 3.000|30.059 / 30.671|0.002 / 0.002|
|tile2048|1|345936 / 223056|— / —|14.000 / 14.000|3.000 / 2.750|1.000 / 1.000|0.012 / 0.012|
|tile2048|16|345936 / 223056|— / —|14.000 / 14.000|10.250 / 10.250|15.364 / 15.478|0.008 / 0.008|
|tile2048|32|345936 / 223056|— / —|14.000 / 14.000|19.000 / 18.250|30.000 / 29.789|0.008 / 0.008|
|group35973|1|6162168 / 4004088|— / —|0.000 / 0.000|13.500 / 10.000|1.001 / 1.000|0.188 / 0.188|
|group35973|16|6162168 / 4004088|— / —|0.000 / 0.000|178.250 / 123.000|15.380 / 15.070|0.125 / 0.125|
|group35973|32|6162168 / 4004088|— / —|0.000 / 0.000|355.000 / 243.500|31.222 / 31.039|0.125 / 0.125|
|groups31x256|1|1327768 / 866968|— / —|0.000 / 0.000|4.750 / 4.000|1.001 / 1.000|0.047 / 0.047|
|groups31x256|16|1327768 / 866968|— / —|0.000 / 0.000|38.750 / 27.000|15.578 / 15.480|0.031 / 0.031|
|groups31x256|32|1327768 / 866968|— / —|0.000 / 0.000|75.250 / 52.250|29.955 / 29.657|0.031 / 0.031|
|tails|1|46856 / 31496|— / —|0.000 / 0.000|2.500 / 2.250|1.000 / 1.000|0.002 / 0.002|
|tails|16|46856 / 31496|— / —|0.000 / 0.000|2.250 / 2.250|15.420 / 15.374|0.002 / 0.002|
|tails|32|46856 / 31496|— / —|0.000 / 0.000|2.750 / 2.250|30.198 / 30.884|0.002 / 0.002|
|weighted128|1|unknown / unknown|30072 / 30072|9.000 / 9.000|2.250 / 2.250|1.000 / 1.000|0.002 / 0.002|
|weighted128|16|unknown / unknown|30072 / 30072|9.000 / 9.000|2.500 / 2.250|15.499 / 15.495|0.002 / 0.002|
|weighted128|32|unknown / unknown|30072 / 30072|9.000 / 9.000|2.250 / 2.250|30.311 / 30.579|0.002 / 0.002|
|weighted256|1|unknown / unknown|59768 / 59768|10.000 / 10.000|2.500 / 2.250|1.000 / 1.000|0.002 / 0.003|
|weighted256|16|unknown / unknown|59768 / 59768|10.000 / 10.000|2.250 / 2.250|15.499 / 15.445|0.002 / 0.002|
|weighted256|32|unknown / unknown|59768 / 59768|10.000 / 10.000|2.750 / 2.500|29.524 / 30.760|0.002 / 0.002|

Busy CPUs are the ratio of median process CPU time to median wall time. Minor faults are mappings resolved without disk reads. Neither establishes swap behavior or cache-level causality. Weighted forecasts are unchanged and cannot be interpreted as actual allocation bytes. No low-memory-device bound or universal cap is inferred.

## CPU counters and profiles

All latency capture finished before counter/profile capture. Four grouped user events require ≥90% coverage. Profiles use FIFO-controlled measured regions and a fixed 16,000,000 user-cycle period. Qualification requires ≥1,000 samples, unknown ≤5%, no lost events and ≥95% samples on the declared workers. Every PID/TID is checked against binary MMAP ownership and FORK metadata; worker CPU assignments are exact, with other owned main/coordinator samples separately counted. Raw ownership changes preserve hashes. All 15 panels and all 15 profiles qualified; no capture was extended.

|Panel|Cycles/wave|Instructions/wave|Cache misses/wave|Branch misses/wave|Qualified|
|---|---:|---:|---:|---:|:---:|
|stat-group35973-w1-OFF|5031719.8|11008629.2|94863.2|90.1|true|
|stat-group35973-w1-ON|4872782.8|11673266.2|86652.5|75.7|true|
|stat-group35973-w1-frozen|5030638.6|11008629.2|94045.6|81.4|true|
|stat-group35973-w32-OFF|3995534123.8|352277300.9|9889648.1|5163.2|true|
|stat-group35973-w32-ON|2503639065.3|373545520.8|8602877.1|3508.3|true|
|stat-group35973-w32-frozen|4022996010.1|352277172.2|9929471.6|4446.0|true|
|stat-tile384-w1-OFF|86676.8|225744.1|0.0|15.3|true|
|stat-tile384-w1-ON|85864.6|232640.1|0.0|12.9|true|
|stat-tile384-w1-frozen|86541.2|225744.1|0.0|11.8|true|
|stat-tile384-w32-OFF|3442439.0|7220610.7|42.3|434.3|true|
|stat-tile384-w32-ON|3441744.8|7441006.6|40.1|349.7|true|
|stat-tile384-w32-frozen|3437450.4|7220609.3|43.3|426.5|true|
|stat-weighted256-w32-OFF|4193301.1|8663793.3|41.7|567.4|true|
|stat-weighted256-w32-ON|4112599.3|8956685.0|38.3|489.6|true|
|stat-weighted256-w32-frozen|4191656.9|8663791.7|42.1|548.4|true|

|Profile|Samples|Unknown share|Worker share|Other owned samples|Qualified|
|---|---:|---:|---:|---:|:---:|
|cycles-group35973-w1-OFF|5153|0.0194%|100.0000%|0|true|
|cycles-group35973-w1-ON|5022|0.0398%|100.0000%|0|true|
|cycles-group35973-w1-frozen|5139|0.0389%|100.0000%|0|true|
|cycles-group35973-w32-OFF|32217|0.2421%|100.0000%|0|true|
|cycles-group35973-w32-ON|19987|0.0901%|100.0000%|0|true|
|cycles-group35973-w32-frozen|32511|0.2307%|100.0000%|0|true|
|cycles-tile384-w1-OFF|2843|0.6683%|100.0000%|0|true|
|cycles-tile384-w1-ON|2807|0.7125%|100.0000%|0|true|
|cycles-tile384-w1-frozen|2838|0.7047%|100.0000%|0|true|
|cycles-tile384-w32-OFF|7016|0.9835%|100.0000%|0|true|
|cycles-tile384-w32-ON|7035|1.2793%|100.0000%|0|true|
|cycles-tile384-w32-frozen|7019|1.0543%|100.0000%|0|true|
|cycles-weighted256-w32-OFF|8551|1.2864%|100.0000%|0|true|
|cycles-weighted256-w32-ON|8402|1.4877%|100.0000%|0|true|
|cycles-weighted256-w32-frozen|8555|1.1105%|100.0000%|0|true|

At large-group/32 workers the reduction kernel receives 25,865 OFF and 14,006 ON fixed-period samples; outer Work::reduce receives 6,191 and 5,903. Absolute sampled periods suggest that most of the saving occurs inside reduce_layer, while outer copying/reduction work persists. Profiles are statistical observations, not exact instruction or memory-service attribution. Static final-consumer IFMA sites (OFF 600, ON 1,200) include both generic branches and are not executed-work counts; PMU instructions are the dynamic evidence.

## Accepted CPU and Metal comparisons, unchanged

Inherited local 32-worker full-proof observations and supplied historical M5 references follow. They are not new measurements.

|Shape|Zopro ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× limit ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37 / 4.667×|24 / 3.027×|3.7|
|2×2|11.151932|59 / 5.291×|30 / 2.690×|5.9|
|4×2|16.086386|109 / 6.776×|48 / 2.984×|10.9|
|8×2|24.179453|189 / 7.817×|74 / 3.060×|18.9|
|12×2|33.884455|284 / 8.381×|101 / 2.981×|28.4|
|16×2|40.077381|335 / 8.359×|124 / 3.094×|33.5|
|24×2|65.850777|528 / 8.018×|176 / 2.673×|52.8|
|merge24|65.060937|507 / 7.793×|175 / 2.690×|50.7|

Merge54 remains 161.883899 ms without an M5 reference. Primary RSS132.75 MiB is inherited. Source176/SDK6d83 and the 30,846-proof ledger are unchanged. The all-eight CPU10× objective remains active/unmet, **0/8**. No new AWS/Aeglos/Gnark, phone, huge-circuit, pressure, swap, minimum-RAM or concurrent-throughput result is added.




## Storage and continuation

Raw profiles (about 8.9 MiB) and all original workflow files (about 84 MiB) remain at `/home/ubuntu/src/zopro-perf-work/g1-tree-callers176/perf/` and `runs/`, indexed by byte size and SHA256. Git includes all latency outputs, full analysis, commands/exits, source deltas, symbols and counter CSVs. Only the 30 normal/metadata decode stdout streams are omitted from the receipt archive and indexed locally. No capture or decode was repeated for packaging.

The [next stage](artifacts/next.md) starts with actual production call coverage before selecting a genuinely different work/lifetime mechanism. The current failed gate does not select production adoption or a full-SDK candidate. No new proof, AWS/Aeglos, phone/ARM/TEE, pressure/swap, minimum-RAM or proof-throughput result is added. All eight CPU 10× requirements remain active and unmet.

[Analysis](artifacts/analysis.json), [independent review](artifacts/review.json), [decision](artifacts/decision.md), [design](artifacts/design.md), [TLS correction](artifacts/design-tls-addendum.md), [profile ownership declaration](artifacts/profile-thread-addendum.md), [declaration](artifacts/declaration.json), [final identities](artifacts/identities-tls.json), [source audit](artifacts/source-audit.json), [driver](artifacts/driver-tls.rs), [weighted adapter](artifacts/weighted.rs), [capture plan](artifacts/plan.json), [pins](artifacts/pins.json), [symbols](artifacts/profile-symbols.json), [source deltas](artifacts/source-deltas.json.gz), [helpers and receipts](artifacts/helpers-and-receipts.json.gz), [raw index](artifacts/raw-profile-index.json), [decoded-stream index](artifacts/profile-stream-index.json), [publication](artifacts/publication.json), [checksums](SHA256SUMS). Source deltas reconstruct all 178 files of each snapshot using qualified/accepted base manifests; base paths/hashes and inherited qualification are bound in the archived source audits. Full snapshots/binaries remain local. No private key, witness, image or proof bytes, external paper text or executable/library blobs are embedded.
