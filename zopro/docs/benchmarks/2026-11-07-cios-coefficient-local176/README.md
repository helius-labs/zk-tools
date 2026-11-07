# Transient coefficient-local CIOS screen

The new recurrence passes exact arithmetic and range checks, but **fails its declared source-depth screen**. Fused source graphs improve while ordinary frontier depth increases. The fixed recipe closes before native implementation or timing; no source adoption follows.

## Arithmetic and resource tradeoff

For an interior coefficient, the candidate forms the input-product contribution in a transient U, forms the modulus contribution using the previous coefficient in V, then adds U+V. The low quotient frontier and canceled-low carry stay unchanged. This is applied to the first four radix52 rows; the fifth R256/R260 cancellation, packing, normalization and correction are retained.

Unlike the [rejected persistent split](../2026-11-06-split-redc-planes176/README.md), no second coefficient plane survives a row. The algebraic schedule introduces one transient product coefficient and sixteen extra vector additions across four rows. It retains the original field-product count. Compiler lifetime, registers, spills, stack and native capacity are unmeasured.

Every completed row coefficient equals accepted CIOS by regrouping the same nonnegative terms. The prefix coefficient bound is below32B=2^57, with B=2^52; U is below4B. See [equivalence and bounds](artifacts/bounds.md). This is a lane-arithmetic argument, not a prover RAM bound.

## Exact model checks

The new candidate executes **60,684 complete outputs** and **242,736 prefix-row comparisons**. Both BN254 primes, R256/R260, ordinary/sum2/complement-difference operations, inclusive[0,2p] boundaries, deterministic full-width random inputs and dependent chains are covered. The candidate matches accepted CIOS coefficientwise, a whole-BigUint recurrence, the full REDC quotient, canonical modular arithmetic and the lazy output bound. The common final step uses the unchanged accepted helper on each new prefix state. These are integer-model calls, not native SIMD executions or proofs.

Immutable prior Field/BigUint/accepted-row utilities are reused as oracles. The earlier persistent-split candidate and its study main are not called, and its old tests/model/capture are not replayed. New compile/model workflow commands pass on their first executions. The model also checks the conservative prefix-bound recurrence at52binary radices. These finite checks do not independently qualify future native code.

## Fixed source-dependency screen

This source graph uses an IFMA accumulator weight4, add2 and right52shift2. Multiply-source weights0 and4 are hypothetical endpoints from the [dependency audit](../2026-11-07-fq8-dependency176/README.md), not newly measured source latencies. The graph has no register allocation, memory or complete hardware schedule. It covers only the first four rows; it must not be compared directly with the full emitted-wrapper scores92/128 from that audit.

|Operation / hypothetical source weight|Accepted frontier / max|New frontier / max|Prefix IFMAs, both|Adds accepted→new|Strict screen|
|---|---:|---:|---:|---:|---|
|Ordinary /0|62 /62|64 /64|80|8→24|Fail|
|Ordinary /4|62 /62|64 /66|80|8→24|Fail|
|Fused /0|94 /94|80 /80|120|8→24|Pass|
|Fused /4|94 /94|80 /82|120|8→24|Pass|

The prospective rule requires both frontier and maximum depth to improve in all four comparisons. It fails in both ordinary cases. The fused positives remain evidence about this model; they do not waive the failed rule or establish a performance gain. No favorable native subset, threshold, compiler option or parameter rescue is selected.

## Next question and accepted state

The extra final U+V merge lies after the canceled-low carry in coefficient1. A distinct algebraic follow-up can put U into the previous coefficient before its modulus FMAs, keeping the final carry merge in its original place. That changes the dependency graph again; it requires a new declared screen and exact model rather than changing this result. No follow-up native candidate has been implemented here.

Accepted production remains unchanged. Latest matched local32-worker **1×2 is 7.915095 ms**, versus supplied M5 CPU37 ms and Metal24 ms. The all-eight verified CPU10× goal remains active and unmet, **0/8**; [full inherited comparisons](../2026-11-07-g2-real-norm-z176/README.md). Accepted merge54 remains161.718956 ms without a supplied M5 reference, and the distinct-proof ledger stays31,387. No new timing, proof, AWS/Aeglos, mobile, huge-circuit, pressure, RSS or minimum-RAM result is added.

[Results](artifacts/model.json), [new model](artifacts/model.rs), [declared screen](artifacts/design.md), [bounds](artifacts/bounds.md), [pins](artifacts/pins.json), [workflow](artifacts/plan.json) and exact receipts retain the study. The immutable included utility source is published with the linked persistent-split report and identified by hash. Local absolute include/dependency paths require adaptation elsewhere. No binary, library, proving key, witness or proof payload is published.
