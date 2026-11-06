# Lazy inverse-root codec: complete-caller comparison

**The candidate did not pass its performance gate and is not adopted.** This follows the [native codec qualification](../2026-11-06-inverse-root-codec176/README.md), which passed correctness and removed a redundant vector canonical comparison. The complete path did not establish gain across both curves and controls. No full-proof speedup is claimed.

The lazy inverse-root product candidate is closed without adoption or a full-SDK trial. Native correctness and code generation qualified, but the prospectively declared complete-caller performance gate failed24 of78 timing checks:5 calibration,13 nonregression and all6 aggregate gain checks. All1279 workflow commands and the independent reviewer succeeded on first execution; failed statistical checks are preserved as results, not command failures.

The equal-log all24-context paired median ON/OFF ratio is1.002256593 (about0.226% slower), interval[0.988659753,1.007889127],7/15 wins. ON/frozen is0.992846133 (about0.715% lower), interval[0.982834565,1.004109023],10/15 wins. G1/32 across all four sizes is1.031107247 against freshOFF, interval[0.948246840,1.057860040],6wins; against frozen0.983158686, interval[0.920408836,1.026736850],10wins. G2/32 is0.997840554 againstOFF, interval[0.984341053,1.004994082],9wins; against frozen1.006279755, interval[1.003245406,1.011713252],3wins. These are paired geometric aggregate summaries, not ratios of unpaired medians or proof times. Five32-worker fresh-control intervals fall outside the fixed calibration band. This limits attribution; it does not convert the result into a pass or prove the candidate slows every context.

Some scalar-tail point estimates improve: G2/201 is roughly1.00% lower solo and0.83% lower at16 workers againstOFF; G2/353 roughly0.57% and0.50%. They are retained numerically. The shared path did not establish gain across both curves, all batches/tails and both controls. There is no arbitrary positive-gain floor and no unchanged reroll, layout/attribute sweep or favorable-subset promotion.

All18 PMU panels and18 profiles qualify,119253 samples total, zero lost events and all samples on declared workers. Complete-caller instruction counts decrease only about0.0066–0.0271% againstOFF in the six diagnostic contexts. Cycles range from about0.106% lower to1.599% higher. This is materially smaller dynamic work removal than the24-static-row inverse reduction might suggest; static rows do not predict complete-caller speed. Small-count cache-miss ratios vary in both directions and do not establish a cache-level cause. Inverse_nonzero flat profile shares are only about0.6–2.8% in these callers, distinct from existing production-proof profiles and not a removable-wall-time bound.

All1188 callers pass independent generator-output checks, equal output hashes/capacities across all three arms and zero timed allocation. Capacity is exactly unchanged. Thirty-two-worker median busy-CPU observations are usually around31, with G1/200 ON28.76; minor faults are roughly0.000061–0.000183 per wave. There is no evidence here of a page-fault bottleneck. Peak caller RSS is setup-inclusive and not full-proof RAM or minimum RAM. No table, duplicate key/plan, production pool or heap buffer was introduced.

The qualification and its earlier scalar-range/half-root evidence remain intact. The native tests, field/core builds, final-consumer disassembly and successful capture/analyzer/reviewer are not rerun. Full raw profiles/decoded streams remain local with byte/SHA indexes; Git receives factual compact evidence and full latency outputs/receipts. No private key/image/witness/proof bytes or executable/library blobs are published.

Accepted production176/SDK6d83, local1×2 7.928660ms, inherited132.75MiB RSS and the30846-proof ledger are unchanged. No new AWS/Aeglos/Gnark/full-proof/huge/phone/pressure/swap/minimum-RAM evidence is added. All eight CPU10× requirements remain active/unmet0/8, with CPU andMetal comparisons retained. This experiment is not a breakthrough.


## Measurement boundary and frozen protocol

G1/G2 ×200/201/352/353 points ×1/16/32 workers gives24 contexts, including full batches and scalar tails. Frozen is accepted source176 with the audited matching caller adapter, not the accepted proof SDK. Fresh OFF/ON share one qualified changed field library and matching cores; ON alone selects the lazy root product. Existing native tests, builds and codegen were inherited, not repeated. Binaries are immutable and pinned. No architecture table, heap buffer, key/plan copy or production pool is added.

Each worker fixture uses public generator points with seed19+worker; these are not arbitrary254-bit curve-coordinate fixtures. Independent generator output checks, hashes and capacities are checked after the clock and across every arm. Native field qualification separately covered full-width/lazy field inputs against Arkworks. Three128-round priming passes and reset precede timing. The benchmark Rayon pool pins CPU2 solo, physical cores0–15 for16 workers and CPUs0–31 for32, with512KiB stacks. Fixtures are dispatched through par_iter_mut inside pool.install; a fixture is not permanently bound to a particular TID. The clock includes install/scheduling. All timed calls allocate zero times.

The Rust/xtask protocol was frozen before any clock observation. All1279 commands succeeded once:72 preflights and1checker,1080 latency calls,18 grouped counter calls,18 fixed-period profiles and90 offline steps. All1188 callers and78 timing checks were reconstructed independently. No pilot, reroll, extra block, favorable subset or profile extension was used. No other owned compute/build/package/hash workload overlapped capture.

Fifteen paired blocks rotate/reverse contexts and arms. Latency rounds are32768 at200/201 and16384 at352/353 per worker. ns/wave is elapsed time divided by rounds, not aggregate calls or proof latency. Intervals use sorted zero-based ranks3/7/11 of15; their coverage is marginal, not a simultaneous family-wise guarantee. Calibration requires every OFF/frozen interval inside[0.97,1.03]. Every ON/OFF and ON/frozen interval must have upper≤1.01. Separate G1/G2 equal-log32-worker aggregates across all four sizes, and an equal-log aggregate of all24 contexts, require upper<1 and≥11wins against both controls. No minimum positive-gain floor is imposed. All checks and estimates are retained.

|Context|Frozen ns/wave|OFF ns/wave|ON ns/wave|ON/OFF [interval], wins|ON/frozen [interval], wins|OFF/frozen [interval], wins|
|---|---:|---:|---:|---|---|---|
|G1-200-w1|4261.431|4254.476|4260.121|0.999770304 [0.998275802, 1.001760578], 9/15|0.999166373 [0.996795930, 1.001546961], 11/15|0.999478138 [0.997547230, 1.001657826], 9/15|
|G1-200-w16|4840.131|4844.203|4838.539|0.997584097 [0.995482402, 1.000867564], 9/15|0.999226897 [0.996626146, 1.003634314], 10/15|1.001646779 [0.996765312, 1.007036736], 6/15|
|G1-200-w32|8210.755|7034.755|7624.050|1.005702499 [0.965580734, 1.052328395], 6/15|0.993139028 [0.920941226, 1.023581465], 10/15|0.978141583 [0.856773186, 1.008394952], 10/15|
|G1-201-w1|4378.570|4388.267|4383.092|0.997798681 [0.992967802, 1.002117330], 9/15|1.000943926 [0.995658437, 1.003622085], 7/15|1.002214634 [0.998022813, 1.005725172], 6/15|
|G1-201-w16|4973.779|5026.922|4975.551|0.997229353 [0.990320237, 1.000786136], 10/15|1.000583576 [0.997542148, 1.011855940], 5/15|1.004978713 [1.000230137, 1.014391586], 3/15|
|G1-201-w32|7128.682|7129.637|7276.773|1.002407876 [0.997274254, 1.022150076], 7/15|0.999767249 [0.993557837, 1.164679171], 8/15|1.000403020 [0.997197413, 1.002342410], 7/15|
|G1-352-w1|7010.546|7009.851|7014.489|1.000176718 [0.995664463, 1.002444602], 7/15|0.999867157 [0.996994346, 1.001550660], 8/15|0.999257256 [0.996931996, 1.002527408], 9/15|
|G1-352-w16|7985.465|7994.553|7993.551|0.999874701 [0.997112480, 1.002289103], 9/15|1.002180580 [1.000106535, 1.003633253], 3/15|1.001386405 [1.000071900, 1.003974923], 3/15|
|G1-352-w32|11721.120|11574.407|11673.076|1.003365441 [0.990016572, 1.257298305], 6/15|0.986315752 [0.982217261, 1.018072703], 10/15|0.993347755 [0.781845756, 1.001209379], 11/15|
|G1-353-w1|7109.816|7121.972|7107.457|0.998441205 [0.996454317, 0.999361921], 13/15|0.999665526 [0.998154276, 1.002307054], 8/15|1.001803343 [0.999721139, 1.002745469], 4/15|
|G1-353-w16|8123.202|8135.513|8122.356|0.998656640 [0.995619763, 1.002516930], 9/15|1.000647742 [0.998484408, 1.001532506], 5/15|1.001599047 [0.998594288, 1.002618304], 5/15|
|G1-353-w32|14552.210|11780.710|11759.165|0.996553203 [0.965782626, 1.004072578], 9/15|0.807235421 [0.793200932, 0.999417014], 12/15|0.994609761 [0.794928522, 1.004918436], 10/15|
|G2-200-w1|7751.533|7745.975|7751.087|1.000098643 [0.998348757, 1.001455469], 7/15|0.999583975 [0.997595608, 1.001341868], 9/15|0.999543856 [0.997511077, 1.002096575], 9/15|
|G2-200-w16|8963.851|8967.017|8971.273|0.999742980 [0.998360381, 1.001229732], 9/15|1.000698480 [0.999581869, 1.003595572], 5/15|1.000700504 [0.999491276, 1.005575422], 6/15|
|G2-200-w32|14964.109|14454.100|14515.755|1.004078462 [0.958817860, 1.060627061], 5/15|1.005628854 [0.967930776, 1.016506189], 4/15|1.006736781 [0.959550485, 1.022270220], 5/15|
|G2-201-w1|8185.467|8200.127|8126.834|0.989963815 [0.988350107, 0.992414102], 15/15|0.992150087 [0.990682016, 0.996296337], 15/15|1.002239991 [0.999733968, 1.004156909], 4/15|
|G2-201-w16|9471.482|9497.923|9416.756|0.991728484 [0.988999443, 0.992875455], 15/15|0.994073755 [0.992624070, 0.995098900], 13/15|1.003323323 [1.001870382, 1.005521666], 2/15|
|G2-201-w32|14903.229|14926.543|14972.098|0.997955010 [0.954253467, 1.007660806], 8/15|1.003673445 [1.000106232, 1.014153901], 3/15|1.006304419 [0.998055283, 1.061702005], 5/15|
|G2-352-w1|13123.190|13130.453|13123.465|0.999915825 [0.998177934, 1.001957644], 8/15|0.999578053 [0.997878802, 1.001566785], 8/15|0.998799946 [0.998192056, 1.001517792], 9/15|
|G2-352-w16|15259.853|15230.524|15229.336|0.999164366 [0.994068333, 1.000771580], 8/15|0.998172599 [0.996643621, 0.999252427], 13/15|0.998476677 [0.997186076, 1.003834564], 10/15|
|G2-352-w32|24888.369|25040.949|24986.570|0.997583360 [0.990005109, 1.001203289], 9/15|1.004155596 [0.994932162, 1.010348170], 5/15|1.008111529 [0.997589642, 1.014293347], 4/15|
|G2-353-w1|13508.570|13525.530|13440.310|0.994312437 [0.992588098, 0.994741176], 15/15|0.995037217 [0.994289448, 0.996510283], 15/15|1.001691549 [1.000479200, 1.003150475], 3/15|
|G2-353-w16|15706.652|15717.281|15632.739|0.994954437 [0.993709653, 0.995596789], 15/15|0.996139429 [0.995123991, 0.996967947], 15/15|1.001139922 [1.000144956, 1.001733355], 3/15|
|G2-353-w32|25466.131|25593.408|25539.066|1.000118980 [0.991961986, 1.011115809], 7/15|1.003888410 [0.997694143, 1.015989940], 5/15|1.004723387 [0.997530664, 1.008407623], 4/15|

|Aggregate|Paired ON/control ratio [interval], wins|
|---|---|
|ALL/OFF|1.002256593 [0.988659753, 1.007889127], 7/15|
|ALL/frozen|0.992846133 [0.982834565, 1.004109023], 10/15|
|G1/OFF|1.031107247 [0.948246840, 1.057860040], 6/15|
|G1/frozen|0.983158686 [0.920408836, 1.026736850], 10/15|
|G2/OFF|0.997840554 [0.984341053, 1.004994082], 9/15|
|G2/frozen|1.006279755 [1.003245406, 1.011713252], 3/15|

All failed checks (5calibration,13nonregression,6gain):

- `G1-200-w32/OFF/frozen/calibration`
- `G1-200-w32/ON/frozen/nonregression`
- `G1-200-w32/ON/OFF/nonregression`
- `G1-201-w16/ON/frozen/nonregression`
- `G1-201-w32/ON/frozen/nonregression`
- `G1-201-w32/ON/OFF/nonregression`
- `G1-352-w32/OFF/frozen/calibration`
- `G1-352-w32/ON/frozen/nonregression`
- `G1-352-w32/ON/OFF/nonregression`
- `G1-353-w32/OFF/frozen/calibration`
- `G2-200-w32/OFF/frozen/calibration`
- `G2-200-w32/ON/frozen/nonregression`
- `G2-200-w32/ON/OFF/nonregression`
- `G2-201-w32/OFF/frozen/calibration`
- `G2-201-w32/ON/frozen/nonregression`
- `G2-352-w32/ON/frozen/nonregression`
- `G2-353-w32/ON/frozen/nonregression`
- `G2-353-w32/ON/OFF/nonregression`
- `G1-aggregate/ON/frozen/gain`
- `G1-aggregate/ON/OFF/gain`
- `G2-aggregate/ON/frozen/gain`
- `G2-aggregate/ON/OFF/gain`
- `ALL-aggregate/ON/frozen/gain`
- `ALL-aggregate/ON/OFF/gain`

## Resources

Capacities are actual retained caller vectors per worker, identical in all arms and after every timed call. RSS is the maximum observed process peak across15 latency blocks, including setup and threads. It is not a full-proof peak or minimum RAM. CPU/fault observations cover the timed region; busy CPUs are median process CPU-time/wall-time ratios. No page-fault bottleneck is established.

|Context|OFF/ON capacity B per worker|OFF/ON peak RSS MiB|OFF/ON median busy CPUs|OFF/ON minor faults per wave|
|---|---|---|---|---|
|G1-200-w1|[40000,0] / [40000,0]|2.75 / 2.75|1.0000 / 1.0000|0.000061035 / 0.000061035|
|G1-200-w16|[40000,0] / [40000,0]|3.75 / 3.75|15.5083 / 15.5183|0.000061035 / 0.000091553|
|G1-200-w32|[40000,0] / [40000,0]|5.25 / 5.0|31.1711 / 28.7568|0.000091553 / 0.000122070|
|G1-201-w1|[40000,640] / [40000,640]|2.75 / 2.75|1.0000 / 1.0000|0.000061035 / 0.000061035|
|G1-201-w16|[40000,640] / [40000,640]|3.75 / 3.5|15.4037 / 15.4942|0.000061035 / 0.000061035|
|G1-201-w32|[40000,640] / [40000,640]|5.0 / 5.0|31.5104 / 31.0100|0.000061035 / 0.000061035|
|G1-352-w1|[70400,0] / [70400,0]|2.75 / 2.75|1.0000 / 1.0000|0.000122070 / 0.000122070|
|G1-352-w16|[70400,0] / [70400,0]|5.5 / 5.5|15.5317 / 15.5315|0.000183105 / 0.000122070|
|G1-352-w32|[70400,0] / [70400,0]|8.25 / 8.5|31.4953 / 31.2747|0.000183105 / 0.000183105|
|G1-353-w1|[70400,640] / [70400,640]|3.0 / 3.0|1.0000 / 1.0000|0.000122070 / 0.000122070|
|G1-353-w16|[70400,640] / [70400,640]|5.5 / 5.25|15.5252 / 15.5326|0.000122070 / 0.000122070|
|G1-353-w32|[70400,640] / [70400,640]|8.5 / 8.5|31.4960 / 31.5666|0.000122070 / 0.000122070|
|G2-200-w1|[16000,0] / [16000,0]|2.75 / 2.75|1.0000 / 1.0000|0.000061035 / 0.000061035|
|G2-200-w16|[16000,0] / [16000,0]|5.0 / 5.25|15.6548 / 15.6586|0.000061035 / 0.000061035|
|G2-200-w32|[16000,0] / [16000,0]|7.5 / 7.5|31.6347 / 31.4784|0.000061035 / 0.000061035|
|G2-201-w1|[16000,1280] / [16000,1280]|2.75 / 2.75|1.0000 / 1.0000|0.000061035 / 0.000061035|
|G2-201-w16|[16000,1280] / [16000,1280]|5.0 / 5.25|15.6289 / 15.6279|0.000061035 / 0.000061035|
|G2-201-w32|[16000,1280] / [16000,1280]|7.75 / 7.75|31.5360 / 31.5317|0.000061035 / 0.000061035|
|G2-352-w1|[28160,0] / [28160,0]|3.0 / 3.0|1.0000 / 1.0000|0.000122070 / 0.000122070|
|G2-352-w16|[28160,0] / [28160,0]|9.0 / 9.25|15.6696 / 15.6743|0.000122070 / 0.000122070|
|G2-352-w32|[28160,0] / [28160,0]|15.25 / 15.0|31.2245 / 31.2481|0.000122070 / 0.000122070|
|G2-353-w1|[28160,1280] / [28160,1280]|3.0 / 3.0|1.0000 / 1.0000|0.000122070 / 0.000122070|
|G2-353-w16|[28160,1280] / [28160,1280]|10.5 / 10.5|15.6570 / 15.6506|0.000122070 / 0.000122070|
|G2-353-w32|[28160,1280] / [28160,1280]|18.5 / 18.5|31.0920 / 31.1973|0.000122070 / 0.000122070|

## CPU counters and profiles

All latency finished before diagnostics. Fixed panels: for each curve200w1,200w32,352w32 and all three arms. Solo profiles run2097152rounds;32-worker profiles65536; counters use one quarter of profile rounds. Duration was fixed from prior controls before observing the candidate. FIFO control brackets the measured region. Each of four grouped user events requires≥90% coverage. Profiles use period16000000cycles:u, at least1000samples,≤5%unknown, zero loss and≥95%worker samples. Executable MMAP/FORK metadata checks PID/TID ownership and each assigned worker CPU. All18 panels and18 profiles qualify;119253samples, all on declared workers.

Complete-caller instruction savings against freshOFF are only0.0066–0.0271%, despite24fewer static inverse rows. Counters do not show a consistent cycle gain. Cache-miss ratios fluctuate and are not a causal cache-level attribution. Flat inverse sample shares below are not a removable-wall-time bound.

|Context|ON/OFF cycles|ON/OFF instructions|ON/OFF cache misses|ON/OFF branch misses|
|---|---:|---:|---:|---:|
|G1-200-w1|0.999227148|0.999751996|2.145610278|0.998017901|
|G1-200-w32|1.003430351|0.999728742|0.331178777|1.024443405|
|G1-352-w32|1.015992304|0.999890912|0.746094349|1.012837312|
|G2-200-w1|0.999765721|0.999883047|1.397245763|1.031251082|
|G2-200-w32|1.009847344|0.999920778|1.437559560|1.015038036|
|G2-352-w32|0.998943170|0.999933613|1.419110417|1.007555136|

|Profile|Samples|Unknown share|Worker share|Flat inverse share|Qualified|
|---|---:|---:|---:|---:|:---:|
|cycles-G1-200-w1-OFF|3014|0.03318%|100.00000%|2.68746%|true|
|cycles-G1-200-w1-ON|3015|0.06633%|100.00000%|2.81924%|true|
|cycles-G1-200-w1-frozen|3013|0.03319%|100.00000%|2.85430%|true|
|cycles-G1-200-w32-OFF|3874|0.10325%|100.00000%|2.52969%|true|
|cycles-G1-200-w32-ON|3912|0.00000%|100.00000%|2.65849%|true|
|cycles-G1-200-w32-frozen|3916|0.07661%|100.00000%|2.60470%|true|
|cycles-G1-352-w32-OFF|6444|0.01552%|100.00000%|1.56735%|true|
|cycles-G1-352-w32-ON|6392|0.06258%|100.00000%|1.40801%|true|
|cycles-G1-352-w32-frozen|6378|0.01568%|100.00000%|1.69332%|true|
|cycles-G2-200-w1-OFF|5424|0.01844%|100.00000%|1.54867%|true|
|cycles-G2-200-w1-ON|5424|0.01844%|100.00000%|1.58555%|true|
|cycles-G2-200-w1-frozen|5425|0.05530%|100.00000%|1.16129%|true|
|cycles-G2-200-w32-OFF|7708|0.02595%|100.00000%|1.31033%|true|
|cycles-G2-200-w32-ON|7776|0.03858%|100.00000%|1.23457%|true|
|cycles-G2-200-w32-frozen|7734|0.03879%|100.00000%|1.02146%|true|
|cycles-G2-352-w32-OFF|13261|0.03016%|100.00000%|0.59573%|true|
|cycles-G2-352-w32-ON|13279|0.00753%|100.00000%|0.69282%|true|
|cycles-G2-352-w32-frozen|13264|0.05277%|100.00000%|0.79162%|true|

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





## Artifacts and continuation

Full raw profiles and all original workflow files remain under `/home/ubuntu/src/zopro-perf-work/inverse-root-callers176/perf/` and `runs/`. Raw recordings and36 normal/metadata decode stdout streams are indexed by bytes andSHA256. Other workflow files, including every latency output, command, exit and counter CSV, are in the checked UTF8 archive. No capture, decode, native suite or qualified build was repeated for packaging. Host-public omits the unrelated process listing; the immutable original host file remains local and pinned. No private key/image/witness/proof bytes or binary/library blobs are embedded.

The [next stage](artifacts/next.md) is a work-count/algebra hypothesis on the dominant production streaming updates, using existing production evidence. This failed candidate does not select a full-SDK trial. There is no new AWS/Aeglos/Gnark/proof/huge/phone/pressure/swap/minimum-RAM result. All eight CPU10× requirements remain active and unmet.

[Analysis](artifacts/analysis.json), [independent review](artifacts/review.json), [protocol](artifacts/protocol.md), [decision](artifacts/decision.md), [next](artifacts/next.md), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [caller identities](artifacts/identities.json), [source qualification](artifacts/qualification-source-audit.json), [symbols](artifacts/profile-symbols.json), [helpers and receipts](artifacts/helpers-and-receipts.json.gz), [raw index](artifacts/raw-profile-index.json), [decoded-stream index](artifacts/profile-stream-index.json), [publication](artifacts/publication.json), [checksums](SHA256SUMS).
