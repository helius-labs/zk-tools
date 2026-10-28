# Source work ledger after exact model

This is an operation-graph review, not native code, emitted instructions, timing or a memory measurement. Accepted production remains byte-exact. Counts refer to one eight-point group of the G2 prefix path, excluding the batch-wide central inverse.

The accepted path is `crates/zopro-core/src/msm/g2_prefix.rs::apply`, calling the Fq2 lane helpers in `msm.rs`, FqBatch in `simd.rs`, and mixed Ifma8 in the field crate. The candidate keeps two fused products for complex multiplication, not reduced Gauss.

| Arithmetic node | Original IFMA sites | Proposed IFMA sites |
|---|---:|---:|
| Denominator norm | 150 | 150 |
| Forward prefix multiply | 100 | 100 |
| Reverse factor and inverse update | 200 | 200 |
| Conjugate numerator | 300 | 300 |
| Real-factor scaling | 200 | 200 |
| Complex square | 200 | 200 |
| Finishing complex product | 300 | 300 |
| Total | 1450 | 1450 |

Original source-graph costs that wider bounds could remove or simplify:

- Eleven general linear add/sub operations after slope construction: three in complex square, two in Ax+Bx, two in square-(Ax+Bx), two in Ax-X, and two in product-Ay. These are eleven operations, not eleven identical conditional reductions. The current generic add carries then conditionally subtracts 2p; generic sub propagates borrow and conditionally adds 2p. A wide expression still needs additions, carry/borrow propagation, constants, live registers and final packing.
- Five mixed fused-product final corrections: norm once, conjugate numerator twice, finishing complex multiplication twice. Uniform R260 avoids the mixed correction. The norm must explicitly allow output below 3p; other product outputs stay below 2p.

Costs that must be retained in any native trial:

- Twelve imported base-field coordinate vectors per group: four X in the forward scan, then four X and four Y in reverse. Multiplication by 4/2 is an integer shift with carry and radix packing, not zero work. Do not add a persistent recoded key or bucket array.
- Four exported vectors: two X modular quarters and two Y modular halves. X can exceed 256 bits before division; divide in the five-digit representation before compact packing. Each export needs conditional subtraction of 2p then p, versus one canonical correction in the old bounded export: eight final correction stages versus four.
- Existing twelve input and four output packing operations remain. Whether shifts or division can be fused into these requires actual code generation and measurements.
- The batch-wide vector inverse currently executes five vector Montgomery products, one scalar product and one scalar inverse. The uniform representation must charge actual R260-to-host conversion of the root quads and host-to-R260 import of the scalar inverse, preserving representation and lazy-range contracts. The model includes its algebra, not its native conversion cost.
- The new wide representation needs its own bounded operations. Ordinary compact/get/difference2 APIs are not valid for every wider temporary; especially conjugate negation uses 4p-dy0, not 2p-dy0. No public lazy contract may be silently widened.
- Register pressure, spills, stack, scratch, exceptional inputs, tails and complete caller overhead remain unmeasured. Existing host-coordinate key storage alone does not prove unchanged peak RSS, stack size or minimum RAM.

Decision: the exact screen and work ledger permit a native prototype, but do not select production or establish speed. IFMA work is unchanged; any benefit must come from less linear/correction work after charging codecs and wider live values. A complete concurrent point caller is required before any SDK trial.

A separate, smaller source opportunity was observed: the prefix reverse loop calls generic Fq2::lanes_add for canonical Ax and Bx. The other G2 streaming implementation already calls canonical_add::<FAST>. Substituting that existing helper only for FAST inputs could remove two redundant generic sum corrections without R260 changes. No prior report matching xsum/canonical or add_canonical was found in the README history search. This observation is not a measured gain and is not part of this model's results.
