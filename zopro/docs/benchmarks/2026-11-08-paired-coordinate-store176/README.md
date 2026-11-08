# Direct paired-coordinate stores in affine addition callers

**This fixed implementation is not adopted.** The all-24-context aggregate is 2.96% slower than fresh OFF, with zero wins out of 15. G1 regresses. G2's small gain against fresh OFF does not hold against the frozen control. Correctness, equal scratch capacities, zero timed allocations and profile quality pass. Production and accepted proof times remain unchanged.

The candidate removes an output-materialization boundary. It retains the original eight-lane mixed-R256 canonical reduction, word packing and transpose, then joins two fields into ordinary unaligned 64-byte stores. G1 writes x/y together; G2 writes c0/c1 together for x and then y. This uses no hardware scatter, new arithmetic/backend, lookup table, key copy, retained plan, scratch field or production thread pool. All eight destination indices are checked before writes; layout assertions and ordered stores preserve repeated-destination results. Scalar tails and unsupported targets retain their original paths.

Static disassembly confirms the actual linked stores. G1/G2 IFMA site counts remain 800/1,450. Whole-function instruction-site counts change from 4,425 to 4,454 for G1 and 8,590 to 8,512 for G2. These static counts do not establish dynamic copies, spills, stack peaks or speed. The formatter and bridge helpers inline into their consumers.

A prospectively declared early rejection screen ran first: both curves, 200 points, one/32 workers, seven rotated/reversed blocks and frozen/OFF/ON. Twelve preflights, 84 latency calls, four PMU panels, four profiles and offline checks total 125 commands. Correctness/quality pass and no early rejection rule triggers. Both 32-worker frozen-control comparisons fail early calibration and remain visible. Early results select this separate full matrix; they are not pooled or treated as a positive gain claim.

The full matrix uses Ryzen 9 9950X, 16 physical cores/32 logical CPUs on a shared host. It covers G1/G2 × 200/201/352/353 points × one/16/32 workers. The frozen accepted-source caller is not the production SDK. Both fresh arms share the exact new field library; all existing generated field arithmetic is byte-identical to production. Driver flags remain native O3, fat LTO, one codegen unit and debug1.

Each process uses changing public-generator coordinates with independent generator-sum output checks and 512 KiB diagnostic worker stacks. Solo uses CPU 2; 16/32 workers use CPUs 0–15/0–31. Three checked 128-round primes and a reset precede 8,192 timed rounds per worker. Timers include dispatch, borrowed-reference construction and complete additions/inversion. Output checks follow timing. These callers are not proof latency or arbitrary SDK input distributions.

Seventy-two preflights and a checker precede 15 fixed rotated/reversed blocks: 1,080 latency calls, then 18 PMU panels and 18 cycle profiles, then all ownership/decode/quality operations. Total: 1,279 commands and 1,188 checked caller processes. No owned build, helper, hash, analysis, report or push overlaps any recording, including preflights. No observation is excluded, extended or repeated; no candidate or compiler/layout/attribute changes follow clocks.

Paired ratios use sorted zero-based ranks 3/7/11 of 15. Every OFF/frozen interval must lie within [1/1.03, 1.03]; all candidate nonregression upper bounds must be ≤1.02. Each curve's four-size equal-log aggregate at 32 workers and the all-24 aggregate must have upper <1 and at least 10/15 wins against both controls. There is no minimum-gain floor. These marginal rank intervals are not simultaneous family-wise guarantees.

|Aggregate|Candidate / fresh OFF, fixed rank interval|Wins / 15|Candidate / frozen, fixed rank interval|Wins / 15|
|---|---:|---:|---:|---:|
|G1|1.107971480 [1.056499054, 1.141895772]|2|1.066323021 [1.010334671, 1.122879794]|3|
|G2|0.996255381 [0.990551804, 0.998430895]|12|1.004426877 [0.996156681, 1.008442423]|4|
|ALL|1.029625935 [1.015950388, 1.033712074]|0|1.021265557 [1.011087213, 1.031983370]|0|

The matrix fails 37 of 78 timing checks: six control calibrations, 26 nonregression checks and five aggregate gain checks. Calibration failures are all four G1 sizes and G2 200/201 at 32 workers. All raw observations remain in the evidence. Separately calibrated G1 solo and 16-worker rows also regress, so rejection does not depend only on uncalibrated 32-worker comparisons. G2/OFF alone passes its aggregate gain check; G2/frozen does not. This uniform implementation closes without selecting a favorable curve, worker subset or alternate store recipe.

|Context|OFF µs/wave|Candidate µs/wave|Candidate / OFF, fixed rank interval|Wins / 15|
|---|---:|---:|---:|---:|
|G1-200-w1|4.234|4.316|1.019091114 [1.018214368, 1.021006516]|0|
|G1-200-w16|4.807|4.944|1.023887882 [1.021394707, 1.036066523]|0|
|G1-200-w32|6.894|7.228|1.053485130 [1.045905533, 1.251790389]|2|
|G1-201-w1|4.348|4.465|1.027098375 [1.025011093, 1.030029971]|0|
|G1-201-w16|4.964|5.170|1.041960247 [1.031613346, 1.051871297]|0|
|G1-201-w32|7.099|7.443|1.044547910 [0.898845682, 1.306530477]|6|
|G1-352-w1|6.958|7.108|1.020673803 [1.019310457, 1.022311821]|0|
|G1-352-w16|7.949|8.208|1.031456033 [1.025818212, 1.033828299]|0|
|G1-352-w32|11.630|15.363|1.270728776 [1.051395127, 1.330817452]|2|
|G1-353-w1|7.080|7.254|1.024883781 [1.023749314, 1.025174029]|0|
|G1-353-w16|8.083|8.355|1.030595925 [1.027389543, 1.041049114]|0|
|G1-353-w32|11.831|12.392|1.057324569 [1.044416200, 1.068408374]|2|
|G2-200-w1|7.694|7.702|1.001497238 [1.000472671, 1.002386805]|1|
|G2-200-w16|8.932|8.936|1.000729457 [0.997024801, 1.002109349]|5|
|G2-200-w32|14.465|14.351|0.998165686 [0.989138446, 1.009515100]|9|
|G2-201-w1|8.066|8.170|1.011997130 [1.010852035, 1.014166197]|0|
|G2-201-w16|9.363|9.443|1.008783092 [1.005850343, 1.010702565]|0|
|G2-201-w32|14.962|14.940|1.000420112 [0.986655666, 1.009403980]|7|
|G2-352-w1|13.035|13.032|1.000213287 [0.999243567, 1.000780958]|7|
|G2-352-w16|15.216|15.242|1.001569608 [0.998812836, 1.002617161]|5|
|G2-352-w32|25.045|24.771|0.991361482 [0.984402382, 0.999194550]|12|
|G2-353-w1|13.384|13.476|1.007551439 [1.005249198, 1.008789938]|0|
|G2-353-w16|15.617|15.758|1.009278283 [1.007548085, 1.010736902]|0|
|G2-353-w32|25.570|25.400|0.993677096 [0.990230585, 0.999452539]|12|

All output oracles and cross-arm hashes match. Timed allocations are zero and retained scratch capacities match across all three arms, including tails and worker counts. Storage types and workspace forecasting are unchanged. This compares caller storage; it is not a minimum-RAM or memory-pressure result.

|Curve / points|Equal retained scratch bytes per worker|
|---|---:|
|G1-200|40000|
|G1-201|40640|
|G1-352|70400|
|G1-353|71040|
|G2-200|16000|
|G2-201|17280|
|G2-352|28160|
|G2-353|29440|

Maximum observed candidate caller RSS is 18.50 MiB, including setup. It is not prover peak RSS, phone feasibility, minimum RAM or concurrent-proof throughput.

|Diagnostic context|Instructions ON/OFF|Cycles ON/OFF|
|---|---:|---:|
|G1-200-w1|1.021267|1.018279|
|G1-200-w32|1.021279|1.040278|
|G1-352-w32|1.023186|1.077035|
|G2-200-w1|0.987872|1.000327|
|G2-200-w32|0.987862|1.005341|
|G2-352-w32|0.987400|0.999559|

G1 executes about 2.1–2.3% more instructions in these panels; G2 executes about 1.2–1.3% fewer, without a consistent cycle improvement. Cache-miss changes vary by context. One diagnostic panel per arm/context does not establish a replicated gain, execution-port cause, critical path or DRAM-capacity limitation. The full latency comparison determines the decision.

All 18 full-matrix profiles qualify: 119,560 samples and zero LOST. The four early profiles also qualify, with 23,455 samples. Sampling uses cycles:u with fixed period 16,000,000, 512 mmap pages and acknowledged FIFO gating after primes. Assigned workers, PID/TID/CPU/MMAP, unknown ≤5%, assigned-worker/target shares ≥95%, at least 1,000 samples per profile and grouped-counter coverage ≥99.9% are checked. Complete raw symbols and PMU CSVs are retained.

Native qualification passes all 13 steps: generator check, two new field-format tests, OFF's focused new caller test, ON's 127 core tests, coherent caller links, and generic/noASM/noIFMA compilation. Field tests cover 12,288 guarded layouts across Fq/Fr, lazy boundary values, shuffled/repeated destinations, unaligned word offsets and sentinels. New core tests cover signs, tails, reuse, concurrent small stacks and invalid indices rejected before writes. Existing field math tests are inherited by byte-identical original function bodies; they are not replayed. Of 176 production files, 170 private copies are exact and six parents reverse to their originals; five child files are added. Both declarations and exports of the diagnostic adapter are native-gated before the first build. No ARM/mobile runtime claim follows from the portable compile check.

Both analyses and independent readers pass on first execution. Full review checks 412 pins, all 1,279 command receipts, 1,188 callers, 78 timing checks and all 18 profiles. Early review checks 404 pins and all 125 commands/104 callers. Successful tests, decoders and captures are not replayed for bookkeeping. A FIFO environment-path error in preparation was fixed before any recording.

This experiment produces no SDK requests, exported proofs or cloud resources. Canonical proof inventory remains 35,050. The proof comparisons below are entirely **inherited** from the [preceding SDK report](../2026-11-08-packed-miller-sdk176/README.md): latest local 1×2 and seven earlier full-SDK rows. Supplied M5 references use different hardware.

|Shape|Accepted local ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.995682|37.0|24.0|4.627×|3.002×|3.7|
|2x2|11.241463|59.0|30.0|5.248×|2.669×|5.9|
|4x2|15.933816|109.0|48.0|6.841×|3.012×|10.9|
|8x2|24.234311|189.0|74.0|7.799×|3.054×|18.9|
|12x2|33.855565|284.0|101.0|8.389×|2.983×|28.4|
|16x2|40.071563|335.0|124.0|8.360×|3.094×|33.5|
|24x2|65.917462|528.0|176.0|8.010×|2.670×|52.8|
|merge-24x1|65.305023|507.0|175.0|7.764×|2.680×|50.7|

The original verified CPU 10× goal remains active and unmet, **0/8**. Merge54 remains 162.240109 ms without an M5 reference. No new AWS/Aeglos, 250/4999, mobile/ARM/TEE, pressure/swap or minimum-RAM comparisons are made. This exact candidate does not select an SDK trial or adoption.

The [evidence archive](artifacts/evidence.json.gz) contains exact prospective plans/pins, helper and changed-source text, native/portable receipts, linked disassembly, raw latency outputs, full decoded profiles, PMU CSVs and full independently reviewed analyses. Decompression yields a JSON map of relative paths to {utf8,bytes,sha256}; every extracted payload is verified. Public analysis views omit only duplicated command/caller receipt maps; their complete original bytes remain in the archive. The publication manifest distinguishes the raw reviewed-analysis hashes from public-view hashes. Binaries/libraries, raw perf data, keys/images, witnesses/point payloads, private ledgers and full source copies are excluded; relevant identities are retained.
