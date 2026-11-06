# Partial compiler scheduling: complete-caller comparison

**Not adopted: the fixed gate fails 22 of 76 checks.** The all-context estimate is 0.38% faster than owned OFF, but its interval crosses parity. The required 32-worker gain is also unproven, and all eight 32-worker control calibrations fail. Scoped gains and calibrated regressions remain in the results. Production and accepted proof times are unchanged.

## Decision and controls

This tests the [qualified four-opcode compiler model](../2026-11-06-calibrated-compiler176/README.md), with unchanged Rust arithmetic and one shared final post-LTO module. ORIGINAL is the newly emitted Rust 1.98.1 caller; owned OFF is the separately built unmodified LLVM backend; ON uses the fixed partial scheduling correction. Raw files label ORIGINAL as `frozen`. Every executable receives arithmetic CLI arm OFF. There is no split-REDC arithmetic or historical word-export/SDK control here.

Failures are eight OFF/ORIGINAL calibrations, ten nonregression checks and four aggregate gain checks. Every calibration failure is at 32 workers, covering both curves and all sizes. This is not a globally calibrated campaign. An improvement over re-emitted OFF alone cannot establish an improvement over the accepted prover. No result, threshold, block or context is replaced.

| Declared aggregate | ON/control ratio [marginal interval], wins |
|---|---|
| ALL/OFF | 0.996166129 [0.991861167, 1.001994042], 10/15 |
| ALL/ORIGINAL | 0.999785604 [0.993514450, 1.008528481], 8/15 |
| LOGICAL32/OFF | 0.990028783 [0.974434370, 1.006047105], 11/15 |
| LOGICAL32/ORIGINAL | 0.997014608 [0.978058457, 1.019201850], 9/15 |

Positive calibrated observations are retained: G1-200-w16 improves 0.315% against owned OFF and 0.264% against ORIGINAL, with both intervals below parity and 14/13 wins respectively; G1-353-w1 also improves against both controls. G2-201-w1 and G2-201-w16 regress against both calibrated controls: ON/OFF ratios are 1.006101028 and 1.004555417, with intervals entirely above parity. Calibration failures elsewhere do not erase these losses.

Larger G2 32-worker comparisons show positive estimates against owned OFF, while their ORIGINAL comparisons remain near parity and their controls fail calibration. All estimates remain in the table; they do not select a successful subset. The candidate is closed without an SDK trial, post-result model/width/layout/annotation change, topology subset or unchanged retry.

The worker summaries below are descriptive equal-log aggregates of all eight curve/size contexts per worker count. They add no observations or gates.

| Descriptive worker aggregate | ON/control ratio [interval], wins |
|---|---|
| w1/ON/OFF | 1.001102307 [0.998226505, 1.002310780], 6/15 |
| w1/ON/ORIGINAL | 1.001613051 [1.000202342, 1.003253563], 2/15 |
| w16/ON/OFF | 1.000015590 [0.998420278, 1.000472068], 7/15 |
| w16/ON/ORIGINAL | 1.001822349 [1.000297198, 1.002501066], 3/15 |
| w32/ON/OFF | 0.990028783 [0.974434370, 1.006047105], 11/15 |
| w32/ON/ORIGINAL | 0.997014608 [0.978058457, 1.019201850], 9/15 |

## Complete timings

Medians are ns per complete batch wave, not proof latency or concurrent-proof throughput. Paired ratios use matched blocks and need not equal ratios of displayed medians. All 24 contexts remain.

| Context | ORIGINAL ns/wave | OFF ns/wave | ON ns/wave | ON/OFF [interval], wins | ON/ORIGINAL [interval], wins | OFF/ORIGINAL [interval], wins |
|---|---:|---:|---:|---|---|---|
| G1-200-w1 | 4257.492 | 4262.446 | 4254.884 | 0.998210691 [0.996034783, 0.999449566], 12/15 | 0.999841696 [0.997648446, 1.000357352], 8/15 | 1.002022863 [0.998129160, 1.004202794], 5/15 |
| G1-200-w16 | 4847.204 | 4866.772 | 4832.095 | 0.996854586 [0.991904995, 0.998827370], 14/15 | 0.997355821 [0.994250116, 0.999386889], 13/15 | 1.001065426 [0.998119893, 1.005493740], 5/15 |
| G1-200-w32 | 7131.930 | 7900.563 | 6934.496 | 0.993456076 [0.872561922, 1.017923543], 9/15 | 0.999742351 [0.857034826, 1.006398205], 8/15 | 0.994273126 [0.952198316, 1.140762901], 8/15 |
| G1-201-w1 | 4383.306 | 4380.051 | 4371.149 | 0.999384390 [0.992987846, 1.001402301], 9/15 | 0.996265153 [0.994798595, 1.001066226], 9/15 | 1.000102607 [0.994353696, 1.003321886], 7/15 |
| G1-201-w16 | 5026.833 | 4969.363 | 4995.745 | 1.003317814 [0.998662617, 1.009440559], 6/15 | 0.996467191 [0.989681222, 1.005575883], 11/15 | 0.996379697 [0.986495498, 0.999004718], 12/15 |
| G1-201-w32 | 7980.151 | 7100.804 | 7219.373 | 1.018279690 [1.001353678, 1.177888570], 3/15 | 0.995267838 [0.889777036, 1.146173871], 9/15 | 0.993119065 [0.849683620, 1.003357808], 11/15 |
| G1-352-w1 | 7008.855 | 7000.559 | 7001.214 | 0.999572434 [0.996047850, 1.000757446], 9/15 | 0.999827640 [0.996214093, 1.001335692], 8/15 | 0.998225048 [0.997233498, 1.001555449], 10/15 |
| G1-352-w16 | 8016.485 | 7995.356 | 8008.350 | 0.999557932 [0.996892587, 1.002546075], 9/15 | 0.998420234 [0.997315078, 0.999201796], 12/15 | 1.000125559 [0.996187544, 1.001492735], 7/15 |
| G1-352-w32 | 11657.712 | 11810.215 | 11624.034 | 0.996602813 [0.979638366, 1.019839932], 9/15 | 0.999328784 [0.990094031, 1.013811096], 8/15 | 1.000041277 [0.996357460, 1.219391293], 7/15 |
| G1-353-w1 | 7109.284 | 7108.812 | 7097.303 | 0.997874277 [0.997032196, 0.998524839], 12/15 | 0.998462060 [0.995789824, 0.999484345], 12/15 | 1.000362758 [0.998400670, 1.001482705], 7/15 |
| G1-353-w16 | 8149.505 | 8134.902 | 8137.648 | 1.000189025 [0.995626924, 1.003084264], 7/15 | 0.998134733 [0.994773179, 1.000049378], 11/15 | 0.998931399 [0.996193598, 1.001148381], 10/15 |
| G1-353-w32 | 11781.446 | 11829.244 | 11799.353 | 1.002692192 [0.988047868, 1.018262593], 7/15 | 1.001273258 [0.992487360, 1.013350563], 7/15 | 1.003841618 [0.992201928, 1.202083284], 6/15 |
| G2-200-w1 | 7754.794 | 7749.456 | 7755.544 | 1.001193895 [0.999111439, 1.002692506], 5/15 | 1.000697589 [0.998874035, 1.001481172], 6/15 | 0.999823753 [0.997715562, 1.001065372], 8/15 |
| G2-200-w16 | 8980.498 | 9018.876 | 8965.111 | 0.996906730 [0.993726878, 0.999526742], 13/15 | 0.998688403 [0.997913342, 1.000490507], 11/15 | 1.003917018 [0.999201761, 1.005451971], 4/15 |
| G2-200-w32 | 14458.220 | 15103.920 | 14434.275 | 0.952272954 [0.942812875, 0.970203364], 13/15 | 0.995795385 [0.954815586, 1.000536884], 10/15 | 1.045703735 [0.989160835, 1.061020246], 5/15 |
| G2-201-w1 | 8127.271 | 8195.802 | 8244.449 | 1.006101028 [1.004051785, 1.007724648], 0/15 | 1.013041765 [1.012154660, 1.015667843], 0/15 | 1.008563407 [1.005691001, 1.009674990], 0/15 |
| G2-201-w16 | 9404.356 | 9505.745 | 9540.165 | 1.004555417 [1.001536275, 1.006800784], 1/15 | 1.015658356 [1.011502084, 1.016434895], 0/15 | 1.010273249 [1.007798886, 1.012216875], 0/15 |
| G2-201-w32 | 14950.771 | 15521.483 | 14978.221 | 0.970774901 [0.956353044, 1.007606096], 10/15 | 1.002926288 [0.995758783, 1.006630304], 4/15 | 1.031987903 [0.998963402, 1.043140935], 4/15 |
| G2-352-w1 | 13122.094 | 13117.306 | 13128.933 | 1.000539202 [0.999232318, 1.001258771], 6/15 | 1.000191184 [0.999504680, 1.002083223], 6/15 | 0.999888159 [0.998402636, 1.002037907], 8/15 |
| G2-352-w16 | 15246.394 | 15341.653 | 15254.466 | 0.994117417 [0.993197024, 0.996887972], 14/15 | 1.000186077 [0.998781258, 1.002975985], 7/15 | 1.006147021 [1.003948576, 1.007210067], 0/15 |
| G2-352-w32 | 24890.266 | 25520.471 | 24877.197 | 0.976287412 [0.962499792, 0.981277350], 14/15 | 0.999572963 [0.991031901, 1.003415578], 8/15 | 1.021472386 [1.014039393, 1.031174289], 0/15 |
| G2-353-w1 | 13476.582 | 13524.111 | 13587.769 | 1.003471105 [1.002652237, 1.005257758], 1/15 | 1.007288732 [1.005470846, 1.009225631], 0/15 | 1.004086981 [1.002081476, 1.005339432], 0/15 |
| G2-353-w16 | 15668.654 | 15757.832 | 15823.057 | 1.002878055 [1.001334280, 1.004768224], 1/15 | 1.009749182 [1.007881071, 1.010605537], 0/15 | 1.005616266 [1.004585094, 1.008038572], 0/15 |
| G2-353-w32 | 25466.104 | 26103.780 | 25497.120 | 0.975752231 [0.967130454, 0.989091833], 14/15 | 1.001554309 [0.994301875, 1.006642407], 6/15 | 1.026569625 [1.015349331, 1.030915085], 0/15 |

All failed timing checks:

- `G1-200-w32/OFF/frozen/calibration`
- `G1-200-w32/ON/OFF/nonregression`
- `G1-201-w32/OFF/frozen/calibration`
- `G1-201-w32/ON/frozen/nonregression`
- `G1-201-w32/ON/OFF/nonregression`
- `G1-352-w32/OFF/frozen/calibration`
- `G1-352-w32/ON/frozen/nonregression`
- `G1-352-w32/ON/OFF/nonregression`
- `G1-353-w32/OFF/frozen/calibration`
- `G1-353-w32/ON/frozen/nonregression`
- `G1-353-w32/ON/OFF/nonregression`
- `G2-200-w32/OFF/frozen/calibration`
- `G2-201-w1/ON/frozen/nonregression`
- `G2-201-w16/ON/frozen/nonregression`
- `G2-201-w32/OFF/frozen/calibration`
- `G2-352-w32/OFF/frozen/calibration`
- `G2-353-w16/ON/frozen/nonregression`
- `G2-353-w32/OFF/frozen/calibration`
- `LOGICAL32-aggregate/ON/frozen/gain`
- `LOGICAL32-aggregate/ON/OFF/gain`
- `ALL-aggregate/ON/frozen/gain`
- `ALL-aggregate/ON/OFF/gain`

## Work, counters and profiles

The correction changes G1/G2 instruction ordering without changing selected static instruction, IFMA or vector-stack-operand counts. PMU retired work is also nearly identical between owned OFF and ON. These data do not establish removal of arithmetic or a particular port, branch, cache-level, spill, alignment, frequency or host-interference explanation. The separately compiled OFF control is not byte-identical to ORIGINAL; that distinction remains part of the result.

| Fixed PMU context | ON/OFF instructions | ON/OFF cycles |
|---|---:|---:|
| G1-200-w1 | 1.000000010 | 0.999480656 |
| G1-200-w32 | 0.999933437 | 0.998111302 |
| G1-352-w32 | 0.999993226 | 0.998288226 |
| G2-200-w1 | 1.000000012 | 1.000765670 |
| G2-200-w32 | 1.000009816 | 0.980874517 |
| G2-352-w32 | 1.000002246 | 0.993774342 |

All 18 profiles qualify with **119,382 samples**, zero lost events and all worker samples on assigned workers. Minimum native stream/inverse attribution is 99.9078%; maximum unknown attribution is 0.0922%. These are flat sampled locations with skid, not exclusive costs or removable wall time. PMU groups meet the fixed coverage requirement; complete events and coverage remain in the JSON. Generic cache ratios are not cache-level or RAM-capacity diagnoses.

## Protocol and correctness

All 1,207 workflow commands complete once: one checker reuses 72 already reviewed correctness receipts, followed by 1,080 latency processes in 15 paired blocks, 18 PMU calls, 18 profiles and 90 ownership/offline decode steps. All 1,116 newly executed callers pass complete generator-point hashes, unchanged capacities, worker mapping and zero-timed-allocation checks. The 72 reused calls are not new native executions. Their full process masks differ from later measurement masks; no timing or allocator inference is reused.

The protocol, three binary identities, 552 pins, campaign and independent reviewer are frozen before launch. Context and artifact order rotates/reverses by block. Sizes 200/201 use 32,768 rounds per worker; 352/353 use 16,384, after three checked 128-round primes. One worker/process uses CPU2, 16 use CPUs0–15 and 32 use0–31, with workers pinned and stacks configured to512KiB. The host is nonexclusive.

All latency precedes every PMU/profile recording; all recordings precede raw ownership/hash/decode work. Profiles use fixed 16,000,000-user-cycle periods and FIFO timed-region control. Exact executable/PID/TID/FORK/MMAP/CPU/period and zero-loss checks pass. No owned build, test, hash, archive, analyzer or second benchmark overlaps recording. One tiny live-handle JSON write immediately after launch was prospectively permitted and is disclosed. No successful stage is repeated.

Both analyzers independently reconstruct all 76 checks from retained observations: 24 control intervals inside[0.97,1.03], 48 nonregression upper bounds<=1.01, and LOGICAL32/ALL gains versus both controls with high<1 and>=11/15 wins. LOGICAL32 includes both curves and all four sizes. Sorted zero-based ranks3/7/11 of15 are marginal intervals without familywise coverage. There is no minimum gain floor. The independent review passes; the performance gate remains false.

The first capture-helper compilation stopped on a missing rustc_demangle extern before any successful setup command. Its source, failed workflow and logs are retained. A continuation adds only that dependency; all three remaining setup commands succeed once. No native qualification, successful helper build, recording or decode is rerun.

## Resources

Candidate maximum observed process RSS is **18.50 MiB** in this small caller matrix, including fixture setup and threads. It is not full-proof peak RAM or minimum RAM. Scratch capacities agree across artifacts and before/after timing; timed allocations remain zero. Full CPU-time, switch and minor-fault diagnostics remain in the resource JSON. Minor faults do not measure service time, disk paging or a RAM-limit explanation.

| Context | OFF / ON max latency-process RSS MiB | OFF / ON median busy CPUs | OFF / ON minor faults per wave |
|---|---|---|---|
| G1-200-w1 | 2.75 / 2.75 | 1.0000 / 1.0000 | 0.000061035 / 0.000061035 |
| G1-200-w16 | 3.75 / 3.75 | 15.4230 / 15.5172 | 0.000091553 / 0.000061035 |
| G1-200-w32 | 5.00 / 5.25 | 27.7736 / 31.3146 | 0.000122070 / 0.000091553 |
| G1-201-w1 | 2.75 / 2.75 | 1.0000 / 1.0000 | 0.000061035 / 0.000061035 |
| G1-201-w16 | 3.75 / 3.75 | 15.5222 / 15.3811 | 0.000061035 / 0.000061035 |
| G1-201-w32 | 5.00 / 5.25 | 31.5240 / 30.9656 | 0.000061035 / 0.000061035 |
| G1-352-w1 | 2.75 / 2.75 | 1.0000 / 1.0000 | 0.000122070 / 0.000122070 |
| G1-352-w16 | 5.75 / 5.50 | 15.5337 / 15.4859 | 0.000183105 / 0.000183105 |
| G1-352-w32 | 8.50 / 8.25 | 30.8690 / 31.1252 | 0.000183105 / 0.000183105 |
| G1-353-w1 | 3.00 / 3.00 | 1.0000 / 1.0000 | 0.000122070 / 0.000122070 |
| G1-353-w16 | 5.75 / 5.75 | 15.5116 / 15.4572 | 0.000122070 / 0.000122070 |
| G1-353-w32 | 8.50 / 8.50 | 31.3483 / 31.3508 | 0.000122070 / 0.000122070 |
| G2-200-w1 | 2.75 / 2.75 | 1.0000 / 1.0000 | 0.000061035 / 0.000061035 |
| G2-200-w16 | 5.00 / 5.00 | 15.5919 / 15.6513 | 0.000061035 / 0.000061035 |
| G2-200-w32 | 7.75 / 7.75 | 30.3644 / 31.6007 | 0.000061035 / 0.000061035 |
| G2-201-w1 | 2.75 / 2.75 | 1.0000 / 1.0000 | 0.000061035 / 0.000061035 |
| G2-201-w16 | 5.25 / 5.00 | 15.5957 / 15.6164 | 0.000061035 / 0.000061035 |
| G2-201-w32 | 7.75 / 7.75 | 30.5864 / 31.5513 | 0.000061035 / 0.000061035 |
| G2-352-w1 | 3.00 / 3.00 | 1.0000 / 1.0000 | 0.000122070 / 0.000122070 |
| G2-352-w16 | 9.25 / 9.50 | 15.5731 / 15.6596 | 0.000122070 / 0.000122070 |
| G2-352-w32 | 15.50 / 15.00 | 30.7781 / 31.4012 | 0.000122070 / 0.000122070 |
| G2-353-w1 | 3.00 / 3.00 | 1.0000 / 1.0000 | 0.000122070 / 0.000122070 |
| G2-353-w16 | 10.75 / 10.50 | 15.6382 / 15.6079 | 0.000122070 / 0.000122070 |
| G2-353-w32 | 18.50 / 18.50 | 30.7180 / 31.2700 | 0.000122070 / 0.000122070 |

## Accepted proof baseline, unchanged

These are inherited accepted local32-worker proof times and supplied M5 references, not this compiler candidate's proof results. The accepted physical16 calibration engineering exception and prior false full gates remain documented. SDK6d83 and proof ledger30,846 stay current. No new SDK/Gnark, AWS/Aeglos, phone/TEE, huge-circuit, pressure/swap, portability, throughput or minimum-RAM qualification is added.

| Shape | Accepted ms | M5 CPU ms / speedup | M5 Metal ms / speedup | CPU10x target ms |
|---|---:|---:|---:|---:|
| 1x2 | 7.928660 | 37 / 4.667x | 24 / 3.027x | 3.7 |
| 2x2 | 11.151932 | 59 / 5.291x | 30 / 2.690x | 5.9 |
| 4x2 | 16.086386 | 109 / 6.776x | 48 / 2.984x | 10.9 |
| 8x2 | 24.179453 | 189 / 7.817x | 74 / 3.060x | 18.9 |
| 12x2 | 33.884455 | 284 / 8.381x | 101 / 2.981x | 28.4 |
| 16x2 | 40.077381 | 335 / 8.359x | 124 / 3.094x | 33.5 |
| 24x2 | 65.850777 | 528 / 8.018x | 176 / 2.673x | 52.8 |
| merge24 | 65.060937 | 507 / 7.793x | 175 / 2.690x | 50.7 |

Merge54 remains161.883899ms without a supplied M5 reference. Inherited primary132.75MiB RSS is not a new peak or minimum RAM. The all-eight verified CPU10x objective remains **active and unmet,0/8**. The fixed compiler correction is closed; the next research step needs a distinct work-removal mechanism, not a post-result parameter rescue.

[Protocol](artifacts/protocol.md), [decision](artifacts/decision.md), [next boundary](artifacts/next.md), [analysis](artifacts/analysis.json), [independent review](artifacts/review.json), [resource diagnostics](artifacts/resources.json), [descriptive worker summaries](artifacts/worker-topology-descriptive.json), [raw/bulk local index](artifacts/raw-and-bulk-index.json), [helpers and receipts](artifacts/helpers-and-receipts.json.gz), [archive index](artifacts/archive-index.json), and [checksums](SHA256SUMS). Bulk sample/meta streams and raw perf data remain local with hashes. Compiler source/archive, IR, objects, executables/libraries, private inputs, keys and witnesses are excluded. The preceding qualification report retains compiler source identities and all nine existing function decodes.
