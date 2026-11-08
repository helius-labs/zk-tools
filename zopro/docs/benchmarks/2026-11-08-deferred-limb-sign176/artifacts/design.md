# Deferred limb-sign compensation: fixed feasibility design

Goal remains verifiedCPU AVX single-proof>=10x suppliedM5CPU ALL8; report CPU+Metal.
Accepted production176/SDK6d83/canonical35171 unchanged. Previous completed stage
late-centered callers2202b535ac is rejected; do not rerun or rescue its subsets.

Hypothesis: remove the data-dependent sign-orientation chain BEFORE each IFMA
cross product in the signed-pair52 convolution. This changes local signed limb
multiplication scheduling, not field representation, field-sign compensation or
Montgomery normalization. It does not reopen the old unchanged global recipe.
No speed forecast or 2x claim follows from the identity or instruction counts.

Let B=2^52, -B<x,y<B, u=x modB, v=y modB, sx=[x<0],sy=[y<0].
IFMA returns l=uv modB,h=floor(uv/B). Then
  xy = l+B*(h-sx*y-sy*u).
Proof: x=u-sx*B, y=v-sy*B; subtracting sx*y instead of sx*v
incorporates the sx*sy*B^2 cross term without a third correction.
Inputs x,y fit signed64, u,v<B. |xy|<B^2, l in[0,B), and the corrected
h equals floor(xy/B) in(-B,B). Sequential intermediate high corrections are
inside(-2B,2B); signed64 arithmetic is safe even with the existing bounded
coefficient accumulator. No full-word multiplication is added to the SIMD path.

Old per-pair dataflow:
 dx=ai-aj;dy=bj-bi;flip=dx<0;x=abs(dx);y=flip?-dy:dy;
 negative=y<0;lo+=IFMAlo(x,y);hi+=IFMAhi(x,y);hi-=negative?x:0.
New fixed dataflow:
 dx=ai-aj;dy=bj-bi;sx=dx<0;sy=dy<0;u=dx&(B-1);
 lo+=IFMAlo(dx,dy);hi+=IFMAhi(dx,dy);
 hi-=sx?dy:0;hi-=sy?u:0.
IFMA sees only low52bits, so dx/dy high twos-complement bits are ignored.
Both recipes use9 explicit vector/mask instructions per pair (including2IFMAs).
New IFMA operand paths need the initial differences only; sign masks/low-mask can
execute independently. There are two post-product high corrections versus one.
Actual CPU, compiler lifetimes, spills and carries may erase or reverse any gain.

The low word and signed high quotient are EXACTLY equal to the old pair recipe,
not merely congruent modulo p. Thus the existing diagonal convolution construction,
Montgomery rows, R256/R260 final packing and lazy output correction remain fixed.
Native source must replace ONLY the pair macro's implementation in a private new
child, preserve the prior child and every old parent body, and bind the delta.
No key/point table, retained plan/copy, new pool, representation or memory cap.

First qualification: standalone checked-i128 model of pair words with exhaustive
small radices,52bit boundaries/full-width deterministic tuples; independent exact
convolution at five normalized52-bit limbs, bothordinary andfused with changing
full input operands. Compare every coefficient and represented integer against
a schoolbook product, and compare newpair words against independent Euclidean
split of signed xy. General range arguments accompany finite checks. Do not
rerun the older signed-pair model/population or prior decoded binaries.

If algebra/ranges pass, build one fixed new native implementation and qualify
actual SIMD output pairs, fullfield mul/sum2/difference chains underFq/Fr and
R256/R260, canonical exports,strict2p closure and bothsign corrections against
independent BigInt/Arkworks references. Source/release/codegen inspection before
clock; no asm/C rewrite or compiler/layout/worker/host/operation-subset sweep.
Only then freeze fullmatched primitive andcompleteG1/G2 caller protocols with
original/freshOFF/ON,changingstates/oracles/zeroallocation/resources/calibration/
nonreg/gain/automaticprofilequality. Allrecordings before anyownedCPUwork.
Any performance qualification remains component evidence; actualWindow/core,
verifiedSDK/all8+54/primary16/Gnark/portable/resources required for adoption.

Prior evidence: signed-pair52 global primitive gate failed R260 despite R256 gains;
joint complex raw sharing lost with extra instructions/stack traffic; late-centered
fullcaller recipe lost3.2%aggregate vsfreshOFF. Those failures stay closed. This
hypothesis specifically removes abs/conditional-orientation before IFMA and adds
asymmetric compensation after it. The old nine-instruction recipe, convolution,
coefficient/range/REDC contracts are comparison inputs, not benchmark reruns.
