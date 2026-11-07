# Five-point rational addition: arithmetic work screen

The division-free construction correctly sums five arbitrary generic affine points with **one final inversion**, reducing the dependency depth from three binary inversion stages. Its explicit multiplication work is much larger than accepted binary addition. **The fixed recipe is closed without native implementation or timing.** This result does not rule out other rational-intersection or higher-arity constructions.

## Construction

Interpolate a rational cubic/linear function through five inputs on `y²=x³+b`. Intersecting it with the curve gives a sixth point; the desired sum is its negative. Shared complementary edge products clear interpolation denominators, and homogeneous final coordinates combine normalization into one inverse. The construction uses fixed local arrays, without a proving-key table, copy, retained plan or new pool.

The [derivation](artifacts/derivation.md) gives the polynomial identities and guards. The group-sum argument uses the standard principal-divisor sum relation described in [Ben Lynn's notes](https://crypto.stanford.edu/pbc/notes/elliptic/divisor.html). This is a local derivation, not a novelty claim or a performance claim imported from that reference. Earlier ternary H/dH and segmented common-Z trials are separate constructions and remain closed.

## Correctness

One Rust/xtask compile and model execution pass on their first executions:

- **512 fresh full-width BN254 sums:** 256 each in G1 and G2, checked against independent Arkworks complete group addition.
- **2,048 F43 tuples:** 1,121 accepted sums match a separately implemented integer group law; 725 equal-x, 26 zero-leading-coefficient and 176 pole cases decline.
- **Ten explicit BN254 guard checks:** identity classification, repeated x, opposite inputs, identity sum and final-pole degeneracy, in both groups.

Every accepted construction checks interpolation at all five inputs, cancellation of the two high polynomial coefficients, all seven coefficients of the intersection factorization, the curve equation and the independent final group sum. The F43 group oracle shares neither interpolation nor addition code with the construction. Infinity is classified before the finite-coordinate recipe; this model does not implement production exception fallback or arbitrary input validation.

An offline reviewer checks all 176 accepted source hashes, two exact command receipts/stream hashes, output-count accounting and all nine work rows. It reconstructs the arithmetic totals from separate stage sums and binds the binary comparison to the previously qualified source census. It does not rerun the model, native tests, proofs, prior census or verifier. Compiler warnings for two unused imports are preserved without a cleanup rebuild. [Review](artifacts/review.json).

## Work result

The explicit recipe takes **104 field multiplications, five squares and one inverse per five-point sum**. Complement products, their common denominator and weighted inputs already consume 29 multiplications before polynomial assembly. That preliminary work exceeds four generic G1 binary additions' variable 24-vector-multiply term. For G2 it likewise exceeds the conservative binary variable-work comparison. All these are counts for this fixed construction, not universal lower bounds.

For `g` independent eight-lane groups, the favorable native forecast batches all final inverses through the accepted bridge. Ordinary Fq square uses the accepted generic multiply; a complex product uses two fused-two-product operations, and a complex square uses two ordinary products.

|Source-work term|Accepted binary|Rational five-point|
|---|---:|---:|
|G1 vector multiplies|24g + 15|112g + 2|
|G2 vector multiplies|28g + 15|15g + 2|
|G2 fused-two-product operations|20g|209g|
|Scalar multiplication/inverse bridges|3|1|

Using nominal source proxies of 100 IFMA sites per ordinary vector multiply and 150 per fused operation:

|Eight-lane five-input groups|G1 binary / candidate proxy|G2 binary / candidate proxy|
|---|---:|---:|
|1|3,900 / 11,400|7,300 / 33,050|
|256|615,900 / 2,867,400|1,486,300 / 8,409,800|

The variable terms grow by about **4.67× for G1 and 5.66× for G2**. The necessary work gate fails for all nine declared group counts from 1 through 256, including the conservative G2 comparison that adds four vector multiplies per group to binary work. No percentage gain floor, size threshold or native layout rescue is used.

Scalar inversion prices, additions, packing, spills, scheduling and memory traffic remain outside these vector proxies. The inversion-depth improvement is real in the model and is retained as positive evidence; it does not establish a measured runtime benefit. No native memory peak, minimum RAM, mobile/huge-circuit feasibility or full-proof speedup is claimed.

## State and evidence

Production/source176 and accepted SDK remain unchanged. The latest accepted local 32-worker **1×2 remains 7.915095 ms**, versus supplied M5 CPU 37 ms and Metal 24 ms. The verified CPU10× goal remains active and unmet on all eight supplied shapes; [inherited full comparisons](../2026-11-07-g2-real-norm-z176/README.md). Merge54 remains 161.718956 ms. This study creates no proof or timing; the canonical proof ledger stays at **31,465**. No new AWS or Aeglos comparison is added.

[Model result](artifacts/analysis.json), [protocol](artifacts/design.md), [model](artifacts/model.rs), reviewer sources, input pins, workflow plans and exact receipts accompany this report. Keys, witnesses, scalars, proof payloads, libraries and binaries are not published. Private root: `/home/ubuntu/src/zopro-perf-work/five-point-rational176`.
