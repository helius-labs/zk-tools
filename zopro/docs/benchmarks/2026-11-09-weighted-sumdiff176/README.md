# Shared sum/difference weighted fold

**The algebra passes, but this topology fails its work screen and is not selected for native implementation.** At 256 buckets, its optimistic G1 variable multiplication/square proxy rises from **3,012 to 3,774 (+25.30%)**, final projective-add calls rise from **8 to 15**, and doubles from **8 to 22**. Production and accepted performance are unchanged. This is a count result, not a measured slowdown.

The local question was whether sharing a denominator for P+Q and P−Q could improve a different weighted-bucket reduction. For n=2h, split inputs into halves A and B. Set S=A+B and D=A−B elementwise. With W(X)=Σ iXᵢ and T(X)=Σ Xᵢ, for even h:

```
W(A,B) = W(S) + (h/2)·(T(S) − T(D))
T(A,B) = T(S)
```

The candidate reduces D, recurses on S and retains the accepted reconstruction for sizes one and two. Ordinary P±Q uses one shared denominator inverse, four multiplications and two squares; complete identity/equal/opposite guards cover exceptional inputs. The accepted control reproduces the actual `msm.rs::weighted_sum` side copies, simultaneous reduction levels and final double/subtract ladder. No mathematical novelty is claimed.

Two serialized Rust 1.98.1/xtask commands build and execute the checked optimized model successfully on first execution. Across both curves, nine sizes from 1 to 256 and six input patterns produce **108 cases**. Independent generator-coefficient oracles check 432 weighted/total outputs; separate complete-group comparisons check 4,096 paired outputs. All **4,528 output comparisons** pass. Patterns include dense full-width coefficients, identities, equal points, alternating opposites, matching halves and opposite halves. The same populations are reused across cases; these are not proof counts.

The candidate model reuses one n-point slice and checks stable pointer/capacity and immutable inputs. Its recursion introduces no input-sized point buffer, whereas the control models 2n−1 point slots. These are scalar-model slots, not a native byte layout, complete allocation forecast, stack/RSS peak or minimum-RAM result. No production table, key expansion, retained plan, pool or memory cap is introduced.

For dense ordinary G1, the declared proxy charges 5M+S per ordinary affine addition and 7M+2S per shared pair, optimistically amortizing one denominator at 3M. Fixed inversion bridges, packing, tails and projective costs are not folded into this proxy; projective calls/doubles remain separate. It is not a hardware latency model or universal algebraic lower bound. G2 correctness is checked, but its native norm-prefix arithmetic is not assigned these G1 costs.

|Buckets|Accepted M+S|Candidate M+S|Change|Projective adds old/new|Doubles old/new|
|---|---:|---:|---:|---:|---:|
|32|342|432|+26.32%|5 /9|5 /7|
|64|720|906|+25.83%|6 /11|6 /11|
|128|1482|1860|+25.51%|7 /13|7 /16|
|256|3012|3774|+25.30%|8 /15|8 /22|

All four prospective work-admission checks fail. The separate reader derives closed counts independently: accepted variable work is 12n−6log₂n−12; candidate work is 15n−6log₂n−18, an excess of 3(n−2). It also verifies 508 linear coefficients of the weighted identity, complete case cardinalities, both command/exit/stream receipts, the actual accepted source binding and all 176 unchanged production-source hashes. The reader imports no model or curve code, but does not independently rerun the curve model or constitute an external audit.

This closes the exact topology/formula without a native trial, timing run or a threshold/curve/shape/worker rescue. The denominator saving loses to the extra reduction work under the declared admission proxy. It does not prove that shared sum/difference formulas cannot benefit a different algorithm, nor that every scalar operation count predicts CPU latency.

A narrow literature/history check revisited [SRE](https://arxiv.org/abs/2610.27452) and the [Waterloo Pippenger thesis](https://uwspace.uwaterloo.ca/items/43cc12b9-cb03-42bd-a01c-4b0ed5e12403). Their scalar-pattern reuse mechanisms already have local studies; no reported speedup or implementation is transferred here. Previous symmetric-square, joint-coordinate endpoint, prefix-tree, global unit-Z and initial-unit-state results remain unchanged.

No native SIMD qualification, SDK request, proof export, timing or hardware profile is added. The canonical verified-proof inventory stays **43,073**. Preparation/review/publication helper invocations are manual and do not all have xtask receipts. No successful old model, native suite, proof, capture or decoder is replayed.

The following are inherited accepted observations from the [latest complete SDK comparison](../2026-11-09-initial-unit-sdk176/README.md), with supplied historical M5 references on different hardware. They are not measurements of this model.

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.803404|37.0 /4.742x|24.0 /3.076x|3.7|
|2x2|10.940319|59.0 /5.393x|30.0 /2.742x|5.9|
|4x2|15.809849|109.0 /6.894x|48.0 /3.036x|10.9|
|8x2|24.043743|189.0 /7.861x|74.0 /3.078x|18.9|
|12x2|33.720355|284.0 /8.422x|101.0 /2.995x|28.4|
|16x2|39.753010|335.0 /8.427x|124.0 /3.119x|33.5|
|24x2|65.983137|528.0 /8.002x|176.0 /2.667x|52.8|
|merge-24x1|65.378103|507.0 /7.755x|175.0 /2.677x|50.7|

The full CPU 10× goal remains active and unmet, **0/8**. Accepted primary peak RSS remains 132.50 MiB; merge 54 remains 162.189200 ms without a supplied M5 reference. No new AWS/Aeglos, ARM/mobile, huge-circuit, pressure/swap/minimum-RAM or throughput result is claimed.

[Model](artifacts/model.rs), [design](artifacts/design.md), [results](artifacts/result.json), [separate review](artifacts/review-result.json), [source binding](artifacts/source-binding.json), [decision](artifacts/decision.json), [continuation](artifacts/next.md), [sources and receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS).
