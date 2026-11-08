# Low-word Montgomery quotients: complete point callers

**This fixed masked-quotient implementation is slower and is not adopted.** The aggregate across all 24 contexts is **1.016617 versus fresh OFF** (1.014901–1.028111), with **0/15 wins**. G1 and G2 aggregates across four sizes at 32 workers are also slower. All 1,188 callers pass output and allocation checks; all 18 profiles qualify, and the independent reader passes. No full-proof trial is selected. Accepted production source 176 and SDK `6d83` remain unchanged.

The gate fails 46/78 timing checks: 5 control calibrations, 35 nonregressions and all 6 aggregate gains. Every context has an ON/OFF median above 1. All 16 solo and 16-worker contexts calibrate and lose every ON/OFF block; rejection therefore does not depend solely on noisy 32-worker data. Five 32-worker calibration failures remain visible: G1-200/352/353 and G2-200/201. No observation is excluded, extended or rerun, and no favorable subset is promoted.

## Implementation and qualification

The [native-qualified field candidate](../2026-11-08-quotient-mullo-native176/README.md) replaces five zero-accumulator IFMA quotient products per eight-lane CIOS product with VPMULLQ, retaining masks. Quotients, raw outputs, cancellation, all data/modulus products, packing, normalization and bounds are unchanged. Total multiply instructions do not decrease. Four-lane/generic field and production core runtime remain original; there is no new heap, table, key copy, record, retained plan or pool. Parent field qualification, 554,496 raw / 554,496 canonical / 328,400 quotient comparisons and twelve actual wrapper decodes are inherited, not replayed.

A new 177-file snapshot recovers all original core bodies and reuses the exact qualified field source/libraries. Only two diagnostic core exports and one fixture child are appended. G1/G2 fixture/driver/perf bytes match original recovered donors; closed phase-boundary/descriptor/tree/offset edits are absent. Both cores and drivers explicitly bind their corresponding qualified field and original matching dependencies. Field ON/OFF compile argv differ only by quotient selector/output; their diagnostic overflow-check flags are retained. Frozen-reference equivalence is checked by calibration, not assumed from identical whole-executable compilation settings.

Nine necessary build/test/consumer/generic commands succeed. Each fresh arm passes 127 unfiltered core tests, including 39,960 sequential + 162,816 threaded generator-point comparisons, signs, tails, reset, reuse, input immutability and all 53 actual workers in pools of 1/4/16/32 workers requesting 512 KiB stacks. The scalar-generator oracle uses unchanged scalar arithmetic, independent of the changed eight-lane quotient instruction; it is not a new independent proof verifier.

Actual new consumers are inspected once: two symbol listings/eight function decodes. Both run functions transfer to the real original production kernels.

|Actual kernel|IFMA sites OFF→ON|VPMULLQ sites OFF→ON|Vector AND sites OFF→ON|
|---|---:|---:|---:|
|G1 streaming|800→760|0→40|167→199|
|G2 borrowed|1450→1390|0→60|346→394|

The masks preserve quotient normalization after full 64-bit multiplication; the original IFMA output already truncates to 52 bits. G1 static instruction sites stay 4,425; G2 changes 8,590→8,596. RSP-operand sites change 591→593 and 1,469→1,481. These are static counts, not dynamic spills, complete stack bounds, latency or an isolated causal explanation. The cheaper-instruction premise alone did not predict complete point-addition performance.

## Fixed results

Local shared Ryzen 9950X, 16 physical / 32 logical CPUs; CPU 2 solo and CPUs 0–15/0–31 otherwise. G1/G2 × 200/201/352/353 × 1/16/32 gives 24 contexts. Fifteen rotated/reversed process blocks compare immutable original source 176 caller `f07229ce`, fresh OFF and ON. The frozen caller is not the production SDK. Three checked 128-round primes and a reset precede 8,192 changing batches per worker; setup/output checks are outside the clock, while complete arithmetic, inversion, dispatch and reference setup are included.

All 72 preflights, the checker, 1,080 latency processes, 18 PMU panels and 18 cycle profiles finish before 90 offline commands: 1,279 commands / 1,188 complete callers, 199,292,928 worker batch calls including primes. Counts are not proofs. Only the confirmed live handle was polled during recording; no owned helper/build/hash/analyzer/decoder/verifier/report/archive/push overlapped. Both analysis commands pass first execution.

|Aggregate|ON/OFF [fixed rank interval]|Wins/15|ON/frozen [fixed rank interval]|Wins/15|
|---|---:|---:|---:|---:|
|G1|1.022087055 [1.001241524, 1.083975971]|3|1.039660855 [0.990545968, 1.081734486]|4|
|G2|1.013271032 [1.003276261, 1.018095592]|3|1.006355738 [1.001210551, 1.015699840]|2|
|ALL|1.016616700 [1.014900998, 1.028111219]|0|1.019741455 [1.011985762, 1.025046001]|1|

G1/G2 aggregates combine four sizes at 32 workers; ALL combines all 24 contexts with equal log weights inside each block. Ratios pair observations by block; medians of raw times can have a different quotient. The fixed ranks 3/7/11 of 15 are marginal, not familywise intervals. Calibration requires a reciprocal factor of 1.03; all 48 nonregression uppers ≤1.02; all 6 aggregate gains require upper <1 and ≥10 wins against both controls. There is no minimum positive-gain floor.

|Context|OFF µs/wave|ON µs/wave|ON/OFF [interval]|Wins/15|Control calibrated|
|---|---:|---:|---:|---:|---|
|G1-200-w1|4.238|4.315|1.018017 [1.017289, 1.019695]|0|true|
|G1-200-w16|4.809|4.898|1.018758 [1.016016, 1.020522]|0|true|
|G1-200-w32|6.899|7.073|1.025874 [1.013571, 1.193370]|2|false|
|G1-201-w1|4.351|4.437|1.018571 [1.017301, 1.020481]|0|true|
|G1-201-w16|4.936|5.042|1.018163 [1.010812, 1.024253]|0|true|
|G1-201-w32|7.094|7.246|1.023463 [1.018290, 1.067941]|3|true|
|G1-352-w1|6.965|7.100|1.019923 [1.018112, 1.021410]|0|true|
|G1-352-w16|7.962|8.123|1.020709 [1.017834, 1.022161]|0|true|
|G1-352-w32|11.693|11.995|1.019690 [0.819233, 1.048930]|5|false|
|G1-353-w1|7.079|7.211|1.018204 [1.017350, 1.019547]|0|true|
|G1-353-w16|8.091|8.254|1.021379 [1.018717, 1.023337]|0|true|
|G1-353-w32|11.836|12.223|1.032535 [1.023393, 1.048827]|0|false|
|G2-200-w1|7.702|7.862|1.020541 [1.019679, 1.021111]|0|true|
|G2-200-w16|8.949|9.123|1.019741 [1.017457, 1.023676]|0|true|
|G2-200-w32|14.519|14.667|1.009373 [0.986371, 1.049251]|5|false|
|G2-201-w1|8.090|8.194|1.013237 [1.011845, 1.014181]|0|true|
|G2-201-w16|9.378|9.504|1.013644 [1.011558, 1.015738]|0|true|
|G2-201-w32|15.050|15.073|1.000904 [0.991000, 1.019327]|7|false|
|G2-352-w1|13.032|13.310|1.021483 [1.020009, 1.022353]|0|true|
|G2-352-w16|15.222|15.588|1.024426 [1.023127, 1.026157]|0|true|
|G2-352-w32|24.882|25.343|1.015713 [1.005891, 1.020069]|0|true|
|G2-353-w1|13.389|13.641|1.018719 [1.017295, 1.019477]|0|true|
|G2-353-w16|15.637|15.912|1.016843 [1.014654, 1.021492]|0|true|
|G2-353-w32|25.350|25.897|1.021592 [1.016495, 1.026884]|1|true|

## Automatic diagnostics and resource scope

All 18 profiles qualify: 120,086 samples, zero LOST, 100% assigned-worker samples, minimum target share 99.894%, maximum unknown 0.0784%, minimum 3,015 samples. All grouped PMU events run 100%. Each selected context/arm has one panel/profile, so uncertainty and causal stall attribution are not measured. These are flat user-cycle samples, not removable proof-wall shares.

|Diagnostic context|Retired instructions ON/OFF|Cycles ON/OFF|
|---|---:|---:|
|G1-200-w1|0.997578|1.017782|
|G1-200-w32|0.997579|1.034074|
|G1-352-w32|0.997420|1.040328|
|G2-200-w1|1.002672|1.020254|
|G2-200-w32|1.002671|1.012055|
|G2-352-w32|1.002784|1.019334|

G1 retires about 0.24–0.26% fewer instructions while using 1.78–4.03% more cycles; G2 retires about 0.27–0.28% more instructions and uses 1.21–2.03% more cycles. Cache/branch changes differ by context. Additional masks, dependency costs and register allocation are visible leads, not uniquely isolated causes; this does not establish a DRAM or RAM-capacity bottleneck.

Timed allocations are zero and all primed retained capacities match. Maximum ON diagnostic process RSS is 18.5 MiB, excluding proving keys; this is not whole-prover peak or minimum RAM, mobile, concurrent-proof or pressure evidence. Requested worker stack size is not a measured stack bound.

Preparatory helper compilation first needed Value conversion/borrow fixes; a separate preparation premise then assumed the old driver already explicitly named a field library. It stopped before any build plan after copying source. Adding explicit qualified field bindings fixed that premise; source variants and failed snapshot remain retained. Runtime/fixtures/tests did not change. Original core-OFF and test-build-OFF succeeded, then the runner rejected duplicated --nocapture before executing any test. A new recovery plan removes only the final duplicate flag and runs the seven remaining commands once. The two successful builds are reused; the failed invocation/receipt and original plan/pins remain immutable. No successful test/build/decoder/recording was replayed. Helper compiles/preparation/prospective plan-reader/publication lack complete workflow receipts; scientific build/test/decoder/capture/analysis steps retain exact command/exit/streams.

This fixed masked-quotient implementation closes without SDK timing/adoption. The saved-code mask counts are a post-capture observation, not a retroactive rule or an unmasked-quotient result. A distinct truncation/range proof would be needed before changing normalized quotient handling; neither its correctness nor a speedup is established here. Production target policy and generated-source integration also remain unqualified, as in the native parent. No universal impossibility claim about VPMULLQ or field arithmetic follows.

## Accepted CPU and Metal comparison, inherited

No SDK request or Gnark proof is created. The canonical ledger remains at 37,023. The [latest full SDK campaign](../2026-11-08-tree-policy-sdk176/README.md) supplies these local 32-worker accepted-build medians; supplied M5 CPU/Metal values use different hardware.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1×2|8.040649|37|24|4.602×|2.985×|3.7|
|2×2|11.149097|59|30|5.292×|2.691×|5.9|
|4×2|16.069415|109|48|6.783×|2.987×|10.9|
|8×2|24.131178|189|74|7.832×|3.067×|18.9|
|12×2|33.940628|284|101|8.368×|2.976×|28.4|
|16×2|40.164083|335|124|8.341×|3.087×|33.5|
|24×2|66.187704|528|176|7.977×|2.659×|52.8|
|merge24×1|65.361571|507|175|7.757×|2.677×|50.7|

Merge54×1 remains 160.890372 ms without a supplied M5 reference. Historical same-build 7.915095 ms remains valid; later 8.04 ms does not establish an adopted regression or a new record. The full verified single-proof CPU AVX 10× goal remains active and unmet, 0/8. No new AWS/Aeglos, giant-circuit, mobile/ARM/TEE, swap/pressure or concurrent-throughput comparison is added.

[All contexts](artifacts/analysis.json), [independent review](artifacts/review.json), [actual code](artifacts/codegen.json), [source audit](artifacts/source-audit.json), [protocol](artifacts/protocol.md), [full source/receipt/observation evidence](artifacts/evidence.json.gz), [member index](artifacts/archive-index.json), [checksums](SHA256SUMS). Public analysis omits only duplicate full caller/command receipt maps; its exact original bytes and all receipts remain archived and hash-bound. Private keys/images/witnesses/point/scalar/proof arrays, executables/libraries, full unchanged source copies and raw perf data are excluded.
