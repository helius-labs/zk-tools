# Initial-scale scalar inverse qualification

The arithmetic prototype passes; **performance is unmeasured and no production change is adopted**. Moving a fixed scale into the inverse's initial coefficient removes one field multiplication from its ordinary finish. Early returns instead need a correction multiplication. This supports a native performance screen, not a proof-speed claim.

For R=2²⁵⁶ and h=2⁶⁴, accepted modular coefficients start at (R²,0). The candidate starts at (hR² mod p,0). Every intervening modular coefficient update is linear, so the existing final radix64 cancellation now supplies the required scale directly. Both early returns that bypass that cancellation divide their result by h using a fixed Montgomery constant. Initial and updated coefficients remain canonical below p; existing signed reduction bounds still apply. Matrices, integer updates, Euclid divisions, field representation and zero handling are unchanged. No allocation, lookup table, key copy, execution plan or pool is added. [Derivation and prospective scope](artifacts/design.md).

One new instrumented Rust prototype uses unchanged accepted field arithmetic, native CPU compilation, fat LTO, one codegen unit, checked overflow and debug assertions. Independent BigUint modular inverses, the accepted inverse and product identities check complete outputs over both fields. Inputs include numerical boundaries, positive/negative raw powers, full-width deterministic values and changing inverse chains. Crafted coprime finish states independently establish the required coefficient invariant.

| Check | Fq | Fr |
|---|---:|---:|
| Complete nonzero inverse outputs | 7,168 | 7,168 |
| Ordinary finishes in those outputs | 6,659 | 6,659 |
| `b == 1` finishes in those outputs | 509 | 509 |
| Zero-input checks | 1 | 1 |
| Crafted finish states | 3,324 | 3,324 |
| Changing-chain steps, included above | 512 | 512 |

The outer GCD-exit branch was **not reached**. Its corrective expression is identical to the exercised `b == 1` expression, and the constant identity is checked; this is not runtime coverage of that outer branch. These synthetic branch proportions are not a production workload census or latency estimate.

Both workflow commands succeed on first execution. Review checks 186 pins, exact recovery of the original inverse, all 176 unchanged accepted production files, compiler flags, raw receipt hashes and reported branch/output totals. It does not rerun successful qualification. No old field/core suite, solver, SDK request, profile or Gnark export is repeated. The canonical independently verified proof ledger stays 32,637. [Qualification](artifacts/qualification.json), [source recovery](artifacts/source-audit.json), [review](artifacts/review.json), [commands](artifacts/plan.json).

Earlier direct-finish reduction, coprime termination, coefficient-fusion and quotient experiments remain separate, closed results. A bounded history search found no matching initial-scale placement; no exhaustive novelty claim is made. A read-only look at [AUCurves' pinned BN254 inverter](https://github.com/AU-COBRA/AUCurves/blob/e0e4be377623b4aca215e56d1f0f68eed6b6bbc6/curve25519-jasmin-rs/src/safegcd_bn254.rs) and its generic safegcd body found a fixed signed62 divstep schedule, not this coefficient placement. No external implementation is copied, built or benchmarked.

Next, declare a native complete-inverse/caller performance screen with accepted/fresh controls, both fields, actual code-generation checks, worker/resource coverage and unchanged fallback behavior. A gain in this scalar operation would still need complete independently verified SDK measurements. There is no 2× projection from removing one multiplication.

Accepted local **1×2 remains 7.928660 ms at adoption**, with later observations 7.915095 and 7.991694 ms of the same build. No proving time is remeasured here. The [accepted all-eight CPU and Metal comparisons](../2026-11-07-g1-merge-work176/README.md) remain unchanged; supplied CPU10× targets are unmet on all eight shapes. No AWS/Aeglos, mobile, huge-circuit, pressure, swap, peak-RAM or minimum-RAM qualification is added.

Exact source and raw command receipts are public; compiled binaries/libraries and private proving material are excluded. Absolute dependency paths in the plan require adaptation outside this workspace. Private root: `/home/ubuntu/src/zopro-perf-work/inverse-seed-scale176`.
