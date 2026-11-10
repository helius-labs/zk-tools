# Persistent quarter-scaled coordinates: native feasibility

Prospective stage, 10 November 2026. Accepted production is revision176,
HEAD12513a7dc3208ecde74c3cb27e18fad7abd2d3c0. The last turn only clarified
timings (no optimization progress). No live owned recording/build is inherited.
The full verified CPU10x ALL8 goal remains active/unmet0/8.

Use the curve isomorphism c=1/4: X=x/16,Y=y/64, with uniform Montgomery
R=2^260=16H. Numerical X words equal the original xH words. Numerical Y
words equal yH/4 modulo p. Retain canonical X<p and opaque Y<2p between
updates. Original key points remain canonical H, with incoming Y quartered
on demand. Buckets remain64B(G1)/128B(G2), no persistent converted key,
lookup table, second solver plan, or new pool. This is distinct from prior
c=1/2 per-batch codecs, unscaled persistent R260 buckets/preconverted keys,
and original-H lazy-Y buckets. No algorithmic novelty is claimed.

All bounds below are strict except explicitly inclusive3p. Since64p<R:
dx=p+bx-ax<2p. For incoming quarter-Y by<p and bucket ay<2p,
D=2p+by-ay (positive) or3p-by-ay (negative), with0<=D<=3p.
Norm T<=8p^2; numerator T<=12p^2; ordinary square T<=16p^2;
all REDC results <2p. Conjugate numerator uses3p-D0, not2p-D0.
X=2p+slope^2-ax-bx<4p: reduce2p thenp before retaining canonical X.
Finishing dx=p+ax-X<2p. Complex finishing products have T<=8p^2.
Y=2p+product-ay<4p: reduce2p before raw storage. No noncanonical Fp
objects are constructed. Temporary4p values fit four words;3p inclusive
fits normalized radix52 digits. Normalized product operands have at most
4p each. CIOS coefficient growth is bounded conservatively below2^59:
five rows, at most eight52-bit units/column/row, plus final carry/packing;
the new path is uniform52 and has no radix48 final repacking.

Reuse original private uniform CIOS, existing shuffle/inverse tree and
raw packing. Inverse bridges must call the uniform get/new (charged);
interpreting raw roots as H would be incorrect. Quarter import chooses
k=(raw mod4), since p=3mod4, then(raw+k*p)/4<p. Scalar boundary codecs
must canonicalize storedY before doubling twice; direct8p may overflow256.

This stage qualifies actual native arithmetic, not Window timing or proofs.
Fixed tests: 4096 deterministic random tuples plus all288 Cartesian signed
boundary combinations (by4,ay6,dx6,sign2), additional inclusive3p case;
512 output-fed arithmetic groups; all256 sign masks over4 changing valid
G1/G2 point groups; tails0/1/7/8/9/15/16/31/32/33 and identity/double/opposite
cases via complete scalar fallback. Independent BigUint full-radix REDC
checks numerical representatives, canonical ranges, signed deltas, norms,
numerators and finishing operations; Arkworks generator/scalar results check
decoded complete point updates. Include512KiB stacks at1/16/32workers.
No replay of old successful tests/captures or timing is needed.

Before any latency claims, a complete Window integration must charge incoming
quarter conversion, scalar tails/exceptions, final decode, uniform inverse
get/new, all prefix records, scratch and allocation coexistence. Original
accepted IFMA counts are not reduced by this representation. A feasibility
pass only permits that next investigation; it does not establish a speedup.
Do not repair this fixed arithmetic after a numerical counterexample by
quietly changing bounds/formulas. Preserve failures and close or explicitly
qualify a distinct correction. Commit/push completed stage, polite report,
CPU+Metal ALL8 comparisons inherited and labeled; canonical44265 unchanged.
