# Six-coefficient Toeplitz feasibility screen

Question frozen before execution: does a division-free 2-by-3 Toeplitz
schedule provide a new usable coefficient-product/range case for BN254?
This is distinct from the rejected 21-product pairwise PMNS kernel and
the dense schoolbook multiword-truncation screen. Neither is rerun.

Use a two-way block Toeplitz identity (three 3x3 products), each evaluated
with a six-product identity. This gives18 scalar coefficient products per
6x6 Toeplitz matvec. Check every66 independent matrix/vector basis pair,
then 8192 full-width signed random matvecs and boundary cases against a
separately organized dense matrix oracle. Emit the operation schedule.

For E=X^6-lambda, polynomial multiplication is Toeplitz. For the existing
Fq E=X^6-X-1, reconstruct a Toeplitz matrix plus one six-product row
correction; qualify every36 polynomial basis pair for both geometries.
Thus the optimistic nominal one-word ordinary/fused IFMA recipes are
90/126 for binomial18-product stages,120/168 for the trinomial24-product
stages, before sign/carry/range/conversion/linear operations. Low-only
internal Q products count once; raw and external full products count twice.
Do not call these native instruction counts or measured work reductions.

Reuse the old public parameter JSON ONLY as input. With both actual old
Fq/Fr parameters, execute the NEW factored polynomial products in all three
PMNS stages. For ordinary/sum/difference, check coefficient-corner inputs,
4096 deterministic signed random tuples and1024 output-fed updates against
independent dense convolution and BigUint field identities. Prove the
normalized input-box rho closure when its conservative inequality holds;
record wider2rho certificate failures rather than pretending they pass.
Do not rebuild, retime or replay the old native PMNS implementations.

Fixed binomial scan: lambda=-64..64 excluding0, both exact BN254 primes.
Check sixth-power-residue eligibility. Flag obvious rational-square/cube
factor cases; no general irreducibility or parameter qualification claim.
For any valid PMNS M, its multiplication matrix has nonzero odd determinant
(inverse mod2^52) divisible by p (evaluation at gamma). Hadamard gives
S^6>=p for its max absolute column sum S. With B=2^52, the fixed sufficient
closure certificate for all signed inputs <=2rho and fused two products is
8*w*rho^2+(B-1)*S<=B*rho, w=1+5*abs(lambda).
A real rho can satisfy it only if32*w*(B-1)*S<=B^2. Combine with
S>=ceil(p^(1/6)). This excludes this particular conservative certificate,
not all possible PMNS invariants or actual arithmetic closure.

List all surviving non-obvious-factor geometries; do not select a favorable
timed subset. A surviving geometry still needs actual gamma/M/N, invertibility,
tighter complete range bounds, signed52/53-bit product lowering, import/export,
point consumer and native code qualification before any performance capture.
If none survives, close this fixed route without native timing. No larger
lambda, different width or closure-certificate rescue belongs to this trial.

Retain full source, fixed inputs, plans/pins/receipts and an independent
result reader. Commit AND push the completed screen. No production edit,
key/point copy, LUT, retained solver plan, pool or runtime buffer is added.
No full-proof/clock/profile/memory/SDK/Gnark execution is selected here.
Canonical proof inventory37023 unchanged. CPU AVX verified single-proof
>=10xM5CPU for all8+CPU/Metal comparisons remains active0/8.
