# Native CIOS coefficient pre-add qualification

The fixed native implementation passes **37 field tests**, including two new tests with **147,468 native lane outputs** checked against Arkworks. The selected emitted Fq8 wrappers also retain shorter dependency-model chains without additional prefix stack-operand rows. This qualifies a matched caller experiment; **no timing, proof-speed gain or production adoption is established**.

## Implementation and correctness

The [qualified algebra](../2026-11-07-cios-preadd-local176/README.md) forms each coefficient's input-product contribution, adds it to the previous coefficient, then applies modulus FMAs. One transient contribution replaces the rejected persistent second plane. The native variant changes only the first four radix52 CIOS rows of eight-lane Fq/R256, under the new compile cfg and the existing canceled-carry cfg. Ordinary and fused operators both change, so both G1 and G2 point arithmetic can be affected. Fr, four-lane, R260, portable and final mixed48 selection remain original.

There is no new field product, function-call boundary, production unsafe block, key table/copy, heap buffer, retained plan, pool or representation. The new raw-lane transmute is test-only. The source schedule does not establish the compiler's live-state size in an actual inlined point caller.

The complete changed field build passes37tests with zero failures, ignored or filtered tests. Two new tests perform12,289 changing mixed mul/sum2/difference steps at four/eight lanes:49,156 and98,312 outputs. They include raw-lazy boundary resets, full-width random loose inputs, independent Arkworks results, canonical exports, normalized52-bit digits and strict2p bounds. The35 existing tests execute against this changed build; these are new executions needed for the changed arithmetic, not claims that the old accepted qualification was replayed for bookkeeping. No core/prover, full-point caller, portable or pressure test has yet run against this candidate.

All14generator/build/test/consumer/decode workflow commands succeed on their first executions. Actual consumer commands contain O3, native CPU, fatLTO and one codegen unit. The changed-field test uses the pinned native Rust flags and release profile. Cached accepted consumers/decodes are reused; fresh OFF/ON fields and consumers are coherent. The official generator changes only by adding the new reference-test name.

## Emitted code inspection

Scores use the [existing calibration and optimistic graph](../2026-11-07-fq8-dependency176/README.md): IFMA accumulator4, add/sub2, right52shift2; unmeasured forms/memory have zero model cost. Columns0/4 use hypothetical multiply-source weights, not new measurements. The ordinary wrapper is branch-free; fused scores stop before its conditional correction. These are model quantities, not measured cycles or proof predictions.

|Selected Fq8 wrapper|OFF→ON bytes|OFF→ON prefix instructions|IFMA sites, both|Dependency score0, OFF→ON|Score4, OFF→ON|Prefix stack-operand rows, both|
|---|---:|---:|---:|---:|---:|---:|
|Product|1,193→1,338|191→215|100|92→86|94→90|6|
|Fused sum|1,751→1,897|246→270|150|128→110|130→118|6|

Both scores improve for both operators, with unchanged IFMA counts and no calls. No selected prefix contains a ZMM stack store. Whole-function stack-operand counts stay6/12 for ordinary/fused. Counts are instruction locations, not dynamic traffic, spill costs, frame bounds or RAM peaks. More instructions and code may offset the shorter chains in real callers.

The Fq4 control is byte-identical between fresh OFF/ON. Fr8 retains its1,193-byte size and100IFMA sites, but its raw function bytes are not identical; no exact byte-equality claim is made. Ancillary four-lane graph values in the artifact are not calibrated or used for selection; the scored decision concerns the two Fq8 functions above. The graph's missing memory costs limit its interpretation. Actual point-caller code generation and runtime can differ under inlining.

The snapshot binds all176accepted parents:173exact, three changed field/generator parents. A separately generated patch reverses cleanly in a private copy, recovering all three originals byte-for-byte. Production files remain unchanged. The offline reader checks source identities, all14argv/exit/stream receipts, native-test output counts, ELF function sizes/hashes, controls and the four scored comparisons. Its compile/execution pass on their first attempts. Previously qualified graph/ELF helpers are reused without repeating their old toy tests, models or recordings.

## Next experiment and accepted state

Declare and run a matched actual caller experiment before any adoption claim. Because both G1 and G2 are affected, the next stage must check both and preserve fresh controls, full outputs, allocation/capacity limits and threaded behavior. A complete primary prover pilot after integration qualification can judge whether the field gain reaches the user-visible path sooner than an extensive primitive benchmark. No performance gate is declared or waived by this static inspection.

Accepted production remains unchanged: latest local32-worker **1×2 is 7.915095 ms**, versus supplied M5 CPU37ms and Metal24ms. The all-eight verified CPU10× goal remains active and unmet, **0/8**; [full inherited comparisons](../2026-11-07-g2-real-norm-z176/README.md). Merge54 remains161.718956ms without a supplied M5 reference. The distinct-proof ledger stays31,387. No new timing, proof, AWS/Aeglos, mobile, huge-circuit, pressure, RSS or minimum-RAM result is added.

[Native qualification/codegen result](artifacts/codegen.json), [reader](artifacts/codegen.rs), [source patch](artifacts/source.patch), [declared stage](artifacts/design.md), source/build pins, workflow plans and exact streams retain the evidence. Executables, libraries and the test binary remain local and are identified by hashes. No key, witness, proof or executable payload is published.
