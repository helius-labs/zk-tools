# CIOS coefficient pre-add: fixed follow-up

The completed post-add coefficient-local recipe f364bf2ded remains CLOSED under its declared failed screen. Its exact arithmetic/range evidence is retained, including fused positives; no gate is waived or favorable operation subset promoted. This new recipe changes the association/order of the coefficient calculation for both ordinary and fused operations.

Change U+(modulus terms accumulated into tj) into modulus terms accumulated into(tj+U). Coefficient1 retains its final add of the canceled-low quotient in the original place. The previous recipe put one additional merge after that carry, increasing ordinary frontier depth. This pre-add removes that exposed merge from the carry path. The quotient frontier and all product/modulus terms stay unchanged.

For j=1..4:

```
Uj = lo(aj*bi)+hi(a(j-1)*bi)
     [+lo(cj*di)+hi(c(j-1)*di)]
Vj = tj + Uj
Vj = madd52lo(Vj,q,pj)
t'0 = V1 + canceled_low_quotient
t'(j-1) = madd52hi(Vj,q,p(j-1)), j=2..4
```

The same transient U is consumed within each coefficient; no persistent second plane, new field product, key table/copy, heap buffer, retained plan or production pool is introduced. The first row's four tj+U additions fold away because those tj are known zero. Thus the source graph has12additional adds across four rows, versus16 in the closed post-add recipe; actual compiler lifetimes/instructions remain unknown. This is an arithmetic dependency change, not a compiler/width/layout/threshold/parameter rescue.

Use the SAME strict screen as before: first-four-row source frontier AND max-coefficient depth must both improve for ALL four ordinary/fused × hypothetical multiply-source0/4 comparisons; IFMA counts unchanged. No subset or changed gate. Exact coefficient/state equivalence and prefix bounds must also pass. If any fails, close before native implementation.

Arithmetic equivalence follows from associativity of the nonnegative coefficient sums and the same frontier/q. Every new partial sum is below its completed coefficient. Inherit the previous proven32B prefix bound and52radix bound rows without rerunning that unchanged bound computation. Reuse immutable accepted-row/BigUint utilities on NEW candidate executions; do not invoke either old study main. New fixed random seed0x7ac083d69215f4b9,1357random plus193dependent tuples per context; retain the full8boundary values needed to qualify the changed recurrence. Cover Fq/Fr,R256/R260,ordinary/sum2/complementdifference, inclusive[0,2p]. Changed-candidate arithmetic checks are not reported as an old model replay or native qualification.

If the unchanged strict screen passes, the next stage is a single native implementation for eight-lane Fq/R256 first-four-row CIOS, preserving four-lane/Fr/portable and final-row selection. Qualify new native arithmetic/independent oracles and inspect actual emitted consumers before declaring a fixed matched caller/profile protocol. A source graph/math pass is not a native speed win or adoption. Preserve all-eight verified CPU10x goal, CPU/Metal comparisons, bounded memory, Rust/xtask, no questions/agents, commit AND push completed stages.
