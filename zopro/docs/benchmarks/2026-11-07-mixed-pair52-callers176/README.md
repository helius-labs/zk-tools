# Hybrid R256 arithmetic: complete-caller comparison

**Closed without adoption: complete G1 and G2 callers are slower.** The aggregate across four sizes with 32 workers takes 10.45% longer for G1 and 23.79% longer for G2 than fresh OFF. The aggregate across all 24 contexts takes 10.42% longer. This is a component result; no new full-proof speed is measured. The 176 production source identities and accepted SDK `6d83` remain unchanged.

The candidate applies the exact previously qualified single-output signed-pair kernel to all eight-lane R256 ordinary/fused products in both fields, retaining CIOS for uniform R260 and other paths. This explicit hybrid trial was selected before builds/clocks after the earlier global trial failed; that old gate remains failed. The separate rejected joint-complex algorithm is different. Positive primitive cases with invariant second operands did not establish variable complete-caller performance.

The fixed screen fails 58/78 timing checks: 4 calibration, 48 nonregression and 6 gain checks. All 24 candidate/fresh-OFF paired median ratios exceed 1. Four 32-worker OFF/frozen calibration intervals fail and remain visible, so the complete campaign is not globally calibrated. Solo controls calibrate and also show regressions: roughly 2.2–2.3% for G1 and 9.5–11.6% for G2 versus fresh OFF. The rejection is supported by those comparisons as well as the broad losses; no failed calibration is discarded.

The [protocol](artifacts/protocol.md) fixes 24 contexts, 15 rotated/reversed matched blocks, 8,192 rounds per worker and both frozen/fresh controls. The frozen caller is an audited accepted-source diagnostic executable, distinct from the production SDK. Public-generator fixtures cover changing values, signs, tails and reuse; full-width/lazy field correctness is separately qualified. Timing includes dispatch and the existing borrowed-reference preparation. Each displayed time is ns per complete batch wave with the declared workers active, not throughput or proof latency. Paired ratios need not equal ratios of displayed medians.

|Context|Frozen ns/wave|OFF ns/wave|ON ns/wave|ON/OFF ratio [interval], wins|ON/frozen ratio [interval], wins|OFF/frozen ratio [interval], wins|
|---|---:|---:|---:|---|---|---|
|G1-200-w1|4231.578|4234.486|4327.050|1.021810 [1.020801, 1.022451], 0/15|1.022558 [1.019648, 1.024737], 0/15|1.000138 [0.999023, 1.001564], 7/15|
|G1-200-w16|4804.552|4819.756|4962.068|1.029510 [1.027058, 1.036413], 0/15|1.031690 [1.027709, 1.036528], 0/15|1.002333 [0.995443, 1.005577], 5/15|
|G1-200-w32|6878.242|6959.005|7772.932|1.117451 [0.942092, 1.144845], 4/15|1.127852 [1.117252, 1.137703], 1/15|1.012879 [0.995315, 1.213812], 4/15|
|G1-201-w1|4354.240|4354.685|4450.104|1.021802 [1.020335, 1.023966], 0/15|1.022881 [1.019815, 1.024805], 0/15|0.999528 [0.998725, 1.001782], 8/15|
|G1-201-w16|4937.395|4972.307|5106.976|1.028303 [1.023400, 1.037191], 0/15|1.033414 [1.024174, 1.037555], 0/15|0.999534 [0.995335, 1.010475], 8/15|
|G1-201-w32|7131.187|7103.519|7943.852|1.120821 [1.101725, 1.166986], 1/15|1.114803 [1.100048, 1.128605], 1/15|1.001334 [0.991536, 1.014759], 7/15|
|G1-352-w1|6968.588|6960.017|7123.381|1.023040 [1.021136, 1.025072], 0/15|1.022195 [1.020733, 1.024042], 0/15|0.998674 [0.998083, 0.999441], 13/15|
|G1-352-w16|7960.005|7967.852|8233.233|1.035576 [1.030093, 1.036323], 0/15|1.034988 [1.032418, 1.036790], 0/15|1.001504 [0.998908, 1.003693], 6/15|
|G1-352-w32|11720.646|11700.324|13101.359|1.113656 [1.071645, 1.127898], 3/15|1.120529 [1.073356, 1.124262], 2/15|1.006171 [0.817525, 1.241319], 6/15|
|G1-353-w1|7075.803|7075.312|7232.851|1.023245 [1.021936, 1.023677], 0/15|1.022472 [1.021120, 1.023206], 0/15|0.998904 [0.998438, 1.000524], 11/15|
|G1-353-w16|8093.729|8094.143|8377.644|1.035128 [1.031871, 1.037692], 0/15|1.035080 [1.031311, 1.037272], 0/15|1.001191 [0.996418, 1.004288], 7/15|
|G1-353-w32|11778.668|11855.503|13371.381|1.120844 [1.075715, 1.134871], 3/15|1.131555 [1.123464, 1.335565], 2/15|1.010448 [0.997078, 1.256257], 6/15|
|G2-200-w1|7695.638|7701.052|8552.536|1.110690 [1.109295, 1.110837], 0/15|1.110901 [1.109946, 1.111960], 0/15|1.001011 [0.999330, 1.001405], 5/15|
|G2-200-w16|8947.746|8931.461|10156.226|1.138232 [1.132170, 1.139316], 0/15|1.134562 [1.131124, 1.137524], 0/15|0.998918 [0.996776, 1.000079], 11/15|
|G2-200-w32|14381.851|14499.047|18041.752|1.236692 [1.207107, 1.264538], 0/15|1.255598 [1.239879, 1.260781], 0/15|1.008357 [0.996592, 1.036363], 5/15|
|G2-201-w1|8129.073|8074.420|9017.183|1.116369 [1.115308, 1.117379], 0/15|1.108695 [1.108265, 1.109429], 0/15|0.993208 [0.992228, 0.993960], 15/15|
|G2-201-w16|9441.880|9382.063|10664.242|1.136710 [1.134997, 1.139165], 0/15|1.128875 [1.127410, 1.132543], 0/15|0.993857 [0.991442, 0.994945], 15/15|
|G2-201-w32|14864.804|15046.172|18658.910|1.229873 [1.206632, 1.266932], 0/15|1.256187 [1.196976, 1.288784], 0/15|1.014848 [0.975449, 1.025994], 5/15|
|G2-352-w1|13019.566|13018.792|14262.162|1.095316 [1.094877, 1.095966], 0/15|1.095254 [1.094658, 1.096113], 0/15|0.999903 [0.999201, 1.000434], 9/15|
|G2-352-w16|15223.964|15203.651|17310.488|1.139426 [1.136757, 1.140955], 0/15|1.136620 [1.134479, 1.138636], 0/15|0.997827 [0.994753, 0.999389], 13/15|
|G2-352-w32|24973.232|25064.592|31111.089|1.246013 [1.234146, 1.253212], 0/15|1.248538 [1.236827, 1.251204], 0/15|1.001556 [0.994949, 1.004694], 7/15|
|G2-353-w1|13418.579|13368.985|14719.582|1.100166 [1.099409, 1.101871], 0/15|1.096204 [1.095154, 1.097152], 0/15|0.996450 [0.995289, 0.997213], 15/15|
|G2-353-w16|15656.086|15621.152|17779.996|1.135955 [1.134320, 1.140567], 0/15|1.134576 [1.130210, 1.137839], 0/15|0.997921 [0.996821, 0.999317], 13/15|
|G2-353-w32|25494.469|25566.699|31742.336|1.245030 [1.238726, 1.246634], 0/15|1.246697 [1.232873, 1.264536], 0/15|1.003804 [0.992012, 1.018663], 6/15|

|Aggregate|ON/control ratio [interval], wins|
|---|---|
|ALL/OFF|1.104197 [1.098975, 1.109092], 0/15|
|ALL/frozen|1.114686 [1.101193, 1.117729], 0/15|
|G1/OFF|1.104533 [1.063254, 1.153304], 1/15|
|G1/frozen|1.158804 [1.075296, 1.177950], 0/15|
|G2/OFF|1.237858 [1.228018, 1.247037], 0/15|
|G2/frozen|1.246937 [1.235473, 1.254777], 0/15|

Intervals use sorted zero-based ranks 3/7/11 of 15 and are marginal, without simultaneous family-wise coverage. All 24 control intervals must lie within [0.97, 1.03], all 48 candidate nonregression intervals require upper ≤1.01, and each G1/G2 aggregate with 32 workers plus the aggregate across all 24 contexts requires upper <1 and at least 11/15 wins against both controls. There is no positive-gain floor, subset promotion, extension, reroll or threshold amendment. All failures are retained in the results and independently reconstructed.

Native qualification passes 36 field and 126 core tests ON, two new OFF tests, raw dispatch comparisons and generic no-ASM/no-IFMA compilation. All 176 production identities remain exact; 172 private originals are exact and four modified parents recover exactly. The inherited runtime prefix matches the older kernel with 240,096 qualified outputs. Actual Cargo links verify coherent field/core/driver inputs under O3/native/fat-LTO/one-CGU driver flags. Old successful tests and frozen-reference decodes are inherited without repetition. Eight new actual-consumer decodes are recorded.

|Function|OFF/ON IFMA sites|OFF/ON instruction rows|OFF/ON vector stack-addressed rows|
|---|---:|---:|---:|
|G1|800 / 640|4425 / 4868|286 / 315|
|G2|1450 / 1110|8590 / 9938|603 / 1080|
|inverse|500 / 400|1139 / 1539|67 / 147|

Fewer multiply sites coexist with more total instructions and stack accesses. Grouped panels show dynamic instructions increasing 17–18% for G1 and 31–32% for G2; cycles also rise. Instructions/cycle can improve while elapsed time worsens because the instruction stream grows. Stack-addressed static rows are not dynamic spill counts. These measurements do not isolate causal costs for masks, carries, dependencies, spills or cache levels. Cache-miss ratios vary substantially between panels; neither those ratios nor the loss establishes a DRAM-capacity bottleneck.

|Context|ON/OFF cycles|ON/OFF instructions|OFF/ON cache misses per wave|
|---|---:|---:|---:|
|G1-200-w1|1.019170|1.170594|0.001278 / 0.001379|
|G1-200-w32|1.128954|1.170541|430.902405 / 530.158936|
|G1-352-w32|1.110660|1.181296|918.244446 / 35.949158|
|G2-200-w1|1.103342|1.307581|0.002073 / 0.002388|
|G2-200-w32|1.220221|1.307529|332.960571 / 27.085388|
|G2-352-w32|1.205524|1.317484|265.269836 / 305.984863|

All 18 profiles qualify: 125,426 samples, zero lost events, 100% assigned-worker ownership, at least 99.884% target-function attribution and at most 0.091% unknown. FIFO control brackets the interval; a period of 16,000,000, 512 mmap pages and grouped-counter running coverage ≥99.9% are checked. MMAP/FORK metadata verifies executable, PID/TID and CPU ownership. Flat function samples are not exclusive critical-path time. Each raw recording is decoded once per declared stream after all recordings finish.

All 1,188 caller processes pass independent generator oracles, cross-arm output hashes, equal scratch capacities and zero timed allocations. Existing diagnostic pools use 512 KiB stacks and 1/16/32 pinned workers; no production pool/table/key copy/retained plan is added. No owned build/helper/hash/analysis/report/push overlaps recording.

|Context|OFF/ON process max RSS MiB|OFF/ON busy CPUs|OFF/ON minor faults per wave|
|---|---:|---:|---:|
|G1-200-w1|2.75 / 2.75|1.000 / 1.000|0.00024414 / 0.00024414|
|G1-200-w16|3.75 / 3.75|15.390 / 15.488|0.00036621 / 0.00024414|
|G1-200-w32|5.0 / 5.25|31.252 / 31.371|0.00036621 / 0.00024414|
|G1-201-w1|2.75 / 2.75|1.000 / 1.000|0.00024414 / 0.00024414|
|G1-201-w16|3.75 / 3.75|15.327 / 15.490|0.00024414 / 0.00024414|
|G1-201-w32|5.25 / 5.25|31.240 / 31.350|0.00024414 / 0.00036621|
|G1-352-w1|2.75 / 2.75|1.000 / 1.000|0.00024414 / 0.00024414|
|G1-352-w16|5.25 / 5.5|15.489 / 15.526|0.00036621 / 0.00024414|
|G1-352-w32|8.5 / 9.0|31.035 / 31.386|0.00036621 / 0.00024414|
|G1-353-w1|3.0 / 3.0|1.000 / 1.000|0.00024414 / 0.00024414|
|G1-353-w16|5.5 / 5.75|15.496 / 15.523|0.00024414 / 0.00024414|
|G1-353-w32|8.25 / 8.5|31.172 / 31.268|0.00024414 / 0.00036621|
|G2-200-w1|2.75 / 2.75|1.000 / 1.000|0.00024414 / 0.00024414|
|G2-200-w16|5.0 / 5.0|15.575 / 15.651|0.00024414 / 0.00024414|
|G2-200-w32|7.75 / 7.75|31.359 / 31.389|0.00024414 / 0.00024414|
|G2-201-w1|2.75 / 2.75|1.000 / 1.000|0.00024414 / 0.00024414|
|G2-201-w16|5.25 / 5.0|15.552 / 15.624|0.00024414 / 0.00024414|
|G2-201-w32|8.0 / 8.0|31.286 / 31.273|0.00024414 / 0.00024414|
|G2-352-w1|3.0 / 3.0|1.000 / 1.000|0.00024414 / 0.00024414|
|G2-352-w16|9.25 / 10.5|15.599 / 15.504|0.00024414 / 0.00024414|
|G2-352-w32|15.25 / 18.75|31.097 / 31.092|0.00024414 / 0.00024414|
|G2-353-w1|3.0 / 3.0|1.000 / 1.000|0.00024414 / 0.00024414|
|G2-353-w16|10.5 / 10.5|15.597 / 15.561|0.00024414 / 0.00024414|
|G2-353-w32|18.75 / 19.0|31.049 / 30.952|0.00024414 / 0.00024414|

Maximum candidate process RSS is 19 MiB in this small caller matrix. It includes setup/thread/allocator history and is neither a full-proof peak nor minimum RAM. Minor-fault counts do not measure fault service time or disk paging.

The following full-proof comparison is inherited from the [latest SDK trial](../2026-11-07-fused-epoch-sdk176/README.md), not measured here. Earlier local 1×2 results of 7.915095/7.928660 ms and the latest 8.018344 ms use the same accepted SDK `6d83`; the inter-run cause is unconfirmed and no slower replacement was adopted. Historical supplied M5 timings use different hardware.

|Shape|Accepted local ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU10x limit ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.018344|37|24|4.614x|2.993x|3.7|
|2x2|11.241463|59|30|5.248x|2.669x|5.9|
|4x2|15.933816|109|48|6.841x|3.012x|10.9|
|8x2|24.234311|189|74|7.799x|3.054x|18.9|
|12x2|33.855565|284|101|8.389x|2.983x|28.4|
|16x2|40.071563|335|124|8.360x|3.094x|33.5|
|24x2|65.917462|528|176|8.010x|2.670x|52.8|
|merge-24x1|65.305023|507|175|7.764x|2.680x|50.7|

Accepted merge54 is 162.240109 ms without a supplied M5 reference. The verified CPU 10× goal remains active and unmet on all eight shapes. The canonical count of 34,929 proofs and the latest full SDK remain unchanged. No new SDK/Gnark proof, AWS/Aeglos, mobile/ARM, huge-circuit, pressure/swap, concurrent-proof or minimum-RAM measurement is added.

This exact hybrid is closed. No favorable curve/worker/layout/compiler/inlining rescue or full SDK candidate is selected. The next hypothesis must reduce the complete variable-operand arithmetic/dependency/lifetime path.

[Results](artifacts/results.json), [independent review](artifacts/review.json), [protocol](artifacts/protocol.md), [native qualification](artifacts/native-qualification.json), [codegen](artifacts/codegen.json), [decision](artifacts/decision.md) and [full UTF8 evidence archive](artifacts/evidence.json.gz) retain all arms and failures. The archive contains selected sources, plans, pins, receipts and all decoded streams; raw perf recordings and binaries/libraries remain local with hashes. No keys, witness/proof payloads or private ledgers are published. Host-public omits unrelated process arguments.
