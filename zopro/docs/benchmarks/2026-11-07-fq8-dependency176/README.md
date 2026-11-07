# Accepted Fq8 dependency audit

The saved ordinary and fused Fq8 wrappers contain long accumulator chains even when all unmeasured latencies are optimistically assigned zero. Their model completion scores are **92 and 128 cycles**, versus separate nominal IFMA resource floors of **50 and 75**. This supports testing a different accumulation recurrence; it does not measure kernel latency or prove a removable fraction of proof time. Production is unchanged.

This follows the earlier [partial caller-chain analysis](../2026-11-06-split-redc-planes176/README.md). That analysis already identified long IFMA chains and led to a rejected persistent product/reduction split. The present audit neither rediscovers that mechanism nor reopens it. It uses the [cached accepted wrapper decodes](../2026-11-07-cios-outer-loop176/README.md) and [existing hardware calibration](../2026-11-06-instruction-calibration176/README.md), with no native call, recording, decode, arithmetic test, proof or timing replay.

## Model and result

The graph tracks vector register definitions, XMM/YMM/ZMM aliases and zeroing idioms. Register renaming removes false write-after-write and write-after-read dependencies. A copy propagates its source value. The analyzed prefix stops at the first branch; the fused conditional correction is excluded. The ordinary prefix covers its complete branch-free wrapper.

Rounded measured weights are four cycles for an IFMA accumulator edge, two for vector add/subtract and two for right shifts by 52. The older combined accumulator/source-alias recurrence does not isolate multiply-input latency. Loads/stores, memory dependencies, address generation, masks, permutations, 48-bit right shifts, four-bit left shifts and other uncalibrated operations contribute zero in this optimistic model. They remain unknown costs, not free hardware operations. Entry operands are available at model time zero.

|Function|Prefix instructions|IFMA sites|Nominal IFMA resource floor|Dependency score, unknown source weight 0|Hypothetical source weights 1 / 2 / 3 / 4|
|---|---:|---:|---:|---:|---|
|Fq8 product|191|100|50|92|92 / 92 / 92 / 94|
|Fq8 fused sum|246|150|75|128|128 / 128 / 128 / 130|

These are simplified calibration-weighted graph quantities, not guaranteed architectural lower bounds, a calibrated whole-core model or measured runtime. Hypothetical source weights are sensitivity values, not measurements or bounds on those unknown latencies. The resource column uses an idealized two register IFMAs per cycle, rounded from the measured throughput; it excludes all other resource use. Do not sum the columns or scale them into SDK, AWS or M5 timing predictions.

One retained ordinary critical chain has seventeen accumulator-weighted IFMAs and twelve measured add/shift edges: 68 + 24 = 92 model cycles. The graph and critical paths are retained for inspection. Modeled insensitivity to source weights 0–3 means this result does not select another source-latency capture. It does not establish whether another unmeasured effect dominates the real caller.

## Next arithmetic question

Screen a **transient coefficient contribution within each CIOS row**. For an interior coefficient, form the input-product terms in a temporary independent of the previous coefficient, form the modulus terms using the previous coefficient, then add the results. The quotient frontier and canceled-low carry remain the accepted recurrence. This changes accumulation dependencies while retaining the field-product count.

Unlike the rejected persistent split, the proposed recipe retains no second coefficient plane across rows. It adds vector additions and a temporary whose real compiler lifetime still requires qualification. A full low/high persistent split was considered and not implemented because it preserves the earlier state-pressure concern. Exact arithmetic/range and work screening must precede native code; no timing, adoption or twofold speedup is claimed for the proposal.

## Validation and accepted state

The new Rust analyzer checks pinned inputs, the existing calibration identities and rounded weights, hand-derived toy chains for alias/zero/copy/known-versus-unknown shift behavior, and a secondary all-edge relaxation of each internal graph. It checks critical-path weight sums across all ten model rows. The relaxation shares the graph and cannot independently establish instruction semantics. Compile and analysis commands pass on their first executions. No production field/core/prover test is reported as newly run.

Latest accepted local 32-worker **1×2 remains 7.915095 ms**, versus supplied M5 CPU **37 ms** and Metal **24 ms**. The all-eight verified CPU10× goal remains active and unmet, **0/8**; [full inherited comparisons](../2026-11-07-g2-real-norm-z176/README.md). Merge54 remains 161.718956 ms without a supplied M5 reference. The distinct-proof ledger stays at 31,387. No new AWS/Aeglos, RAM/pressure, huge-circuit or mobile result is added.

[Results and graph](artifacts/analysis.json), [analyzer](artifacts/audit.rs), [declared model](artifacts/design.md), [history correction and decision](artifacts/history-addendum.md), [pins](artifacts/pins.json), [workflow](artifacts/plan.json) and receipts retain the evidence. Source identities and raw decodes are inherited from the linked codegen report. No executable, library, key, witness or proof payload is published.
