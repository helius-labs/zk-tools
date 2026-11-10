# Montgomery reduction with a friendly multiple

Investigate M=k*p with low radix52 digit +1 or -1. For each BN254 prime,
choose the unique smallest positive k in[1,2^52) giving each sign. No parameter
sweep/choice after results; both signs/fields are retained. This is an independently
derived feasibility model, not a claim of novelty or external speedup.

For +1 choose q=-t0 modulo B, B=2^52. For -1 choose q=t0. Exact low cancellation
then shifts52bits. Since p divides M, projection of (T+Q*M)/R modulo p has the
desired Montgomery residue. M is generally composite: it is NOT a replacement
prime field, and inversion/zero tests still require reduction modulo p.

Two fixed complete recipes:
1.Canonical-state R260: inputs remain below2p. Five cancellation rows with the
  enlarged M produce a wider U, which MUST project back modulo p after every
  operation. Charge a full one-digit quotient-times-p projection (two IFMA halves
  per p digit), optimistically pricing quotient selection/import/export at zero.
  Verify the exact upper bound makes the projection quotient fit52bits.
2.Widened-state R312: six-digit values below2M survive between operations.
  Six cancellation rows, six-digit raw products, and the exact <2M output bound.
  Project modulo p for exports/zero tests and original-field inversion bridges.
  No new key/point format or input-sized transformed array is proposed here.

Count the explicit schoolbook raw-product and higher-modulus-digit IFMA tape,
giving the unit low-digit cancellations and quotient selection ZERO IFMAs. Do
not hide the mandatory canonical projection or use five-digit products for six-
digit state. Unit/zero higher digits are simplified explicitly. Native ALUs,
quotient-estimation/division, normalization, register allocation, scalar bridges,
conversions, exceptions and scheduling are unpriced. Counts are not universal
lower bounds, native timing or a hardware dependency schedule.

Prospective advancement: a fixed recipe must use strictly fewer modeled IFMAs
than current100/150 for both ordinary and fused operations for both primes/signs.
This is our work-removal screen, not a theorem that more IFMAs cannot be faster.
Failure closes the two fixed recipes before native implementation/timing, without
subsets, larger constants, altered radices, quotient estimator or table rescue.

Exact BigUint cheap-digit recurrence must match a separately constructed full-R
quotient and independent Arkworks field projection. Both signs/fields/recipes,
zero/inclusive lazy bounds, all corner tuples, deterministic random operands and
output-fed chains. Separately verify small-radix instances with u128/brute-force
full inverses, no BigUint/Newton helper. Bind source/deps and command receipts.
Rust1.98.1/serializedxtask only; preserve failures. No native/performance/proof
replay, giant LUT, copied key/solver plan, new pool or architectural RAM cap.

Full ALL8 verified CPU10x plus CPU/Metal reporting stays active/unmet0/8. A model
cannot satisfy it. Production176/acceptedSDK/canonical43730 remain unchanged.
