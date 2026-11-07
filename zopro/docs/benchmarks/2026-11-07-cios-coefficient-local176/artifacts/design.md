# Transient coefficient-local CIOS contribution

The new accepted-wrapper dependency audit b0b3012117 finds an optimistic accumulator-weighted score92/128 versus nominal IFMA resource50/75cycles. Hypothetical unknown multiply-source weights0..3 do not change those scores. Existing split-redc-planes176 already tried persistent product/reduction planes, with six extra live coefficients and failed native callers. Full persistent low/high planes are not selected either.

New fixed recurrence: form each interior coefficient's input-product contribution U in one transient vector; accumulate modulus terms into the prior coefficient V, then add U+V. Finish that coefficient before moving to the next. No second plane survives the row. The quotient frontier and canceled-low carry are unchanged. This differs from persistent split planes, row-quotient preparation (extra IFMAs/prepared vectors), FIPS/full convolution and dual-CIOS overlap. A targeted row-local/coefficient-local/transient/balanced history search found no identical recipe.

For row i, B=2^52 and lo/hi product halves:

```
z = t0 + lo(a0*bi) [+ lo(c0*di)]
q = lo(z*inverse) mod B
carry = ceil(z/B) + hi(q*p0)
Uj = lo(aj*bi) + hi(a(j-1)*bi)
     [+ lo(cj*di) + hi(c(j-1)*di)]
V1 = t1 + lo(q*p1) + carry
Vj = tj + lo(q*pj) + hi(q*p(j-1)), j=2..4
t'(j-1) = Uj + Vj
t'4 = t5 + hi(a4*bi) [+ hi(c4*di)] + hi(q*p4)
t'5 = 0
```

Use this for first four radix52 rows. Keep the fifth R256/R260 cancellation, normalization and fused correction unchanged. Four extra vector adds per row (sixteen total), unchanged100/150IFMA whole-product counts. One new transient product coefficient in the algebraic schedule, not six persistent coefficients. The compiler may extend lifetimes, hoist work or add spills; do not claim a native workspace/speed benefit before inspecting emitted code.

First screen: independent checked-u128 recurrence against accepted coefficient-wise CIOS and whole-BigUint row/full REDC references, Fq/Fr,R256/R260,ordinary/sum2/complementdifference, inclusive[0,2p], boundary/random/dependent inputs. Reuse immutable prior Field/BigUint/accepted-row utilities as oracles on NEW candidate executions; never invoke the prior model main, persistent-split candidate or its old study. Prove conservative nonnegative first-four-row coefficient bounds below32B (2^57); common final row unchanged. Include exact output/radix/correction and range checks.

Dependency/work comparison: reconstruct accepted and new first-four-row SOURCE graphs with accumulator4/add2/right52shift2 and hypothetical multiply-source0/4, excluding zero additions. Retain coefficient/frontier/max depths, IFMA/add counts and scope. This is neither emitted code nor a hardware schedule. Continue to native only if all four ordinary/fused × source0/4 comparisons reduce both max merged-coefficient and next-frontier model depths, IFMA counts are unchanged and the fixed prefix range proof holds. No threshold/parameter/subset/compiler rescue if it fails.

If mathematical screen qualifies, freeze a single native eight-lane Fq/R256-only implementation, independently qualify changed native arithmetic and inspect emitted actual caller/consumer flags before a prospective fixed matched caller/profile campaign. No native speed or source adoption follows from this math screen alone. No key/point copy, LUT, heap buffer, retained plan, production pool or representation change. Preserve universal large/low-RAM behavior. Goal remains independently verified CPU10x ALL8 with CPU/Metal comparisons, active/unmet0/8; commit AND push finished experiments.
