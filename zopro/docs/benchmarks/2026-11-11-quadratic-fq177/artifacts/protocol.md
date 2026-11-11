# Quadratic-ring Fq feasibility screen

Hypothesis: exploit p=a²+3u², a=6u²+3u+1, in the BN254 base field by working in
Z[t]/(t²+3, a+u*t). The modulus's second coefficient is only63bits. Store two
signed128-bit coefficients per field element, preserving32bytes rather than a
six-word PMNS representation. This is separate from the earlier Gaussian Fr FFT
trial and PMNSX^6−2 bridge; their negative findings remain valid context.

Before any native implementation, check exact integer arithmetic, full product
and fused-two-product bounds, a compact import/export map and a complete favorable
radix52 arithmetic-site ledger. Do not infer latency from multiplication rank.

Fix R=2^131 and H=2^127. Map(a+u*t)=0 by choosing t=−a/u modulo p. Compute centered
q=−T/(a+u*t) modR coefficientwise, then C=(T+q*(a+u*t))/R. Exact lattice rounding
re-encodes a scalar or recenters linear-operation results. The model may use
BigInt division; its cost is explicitly not free in a possible native design.

Validate all16 combinations of coefficient signs/boundaries for ordinary/fused
operations, additional boundary magnitudes and deterministic random full-width
signed coefficients. Include changing add/sub/mul/fused chains and Fq2 norm,
product and signed-difference compositions. Compare decoded results against
independent ordinary integer modular arithmetic; check both exact divisibility
and signed128 output bounds. No curve/proof/native SIMD claim.

Cost fixed three-product Karatsuba for raw quadratic multiplication and low
quotient multiplication, with unbalanced constant-modulus multiplication. Use
three radix52 limbs per ordinary coefficient and two for u. Omit known-zero high
halves and count truncated low131 multiplication explicitly. Report scalar fixed
factors, carries, signs, canonicalization, packing/import/export and inversion
as unpriced additional costs. Compare ordinary and fused2 with current100/150
IFMA templates; this is a recipe ledger, not an exhaustive lower bound. A recipe
already above those favorable arithmetic counts does not merit a native build
for the requested breakthrough without another work-removal mechanism.

One pinned optimized checked Rust build/run. Keep all failures. No SDK, proof,
CPU timing claim, AWS resource, production mutation, push or publication.
