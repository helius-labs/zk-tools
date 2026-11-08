# Required complete-tradeoff review

Mathematical checks pass. Native implementation is NOT selected. The fixed
binary selected-power recipe is closed without a performance trial. This is
an engineering work-selection judgment, not a measured slowdown, universal
lower bound, or rejection of compressed cyclotomic arithmetic generally.

For every one of 256 ordinary mapped inputs, the accepted source schedule
counts 63 full cyclotomic squares and28 Fq12 products. The proposed schedule
has62 compressed squares,27 normal decompressions, one norm-batched scalar
inverse and27 Fq12 products. Removing the leading identity square/product
is already credited to the candidate. This does not select another schedule
or reopen the earlier signed-window-four full-SDK rejection.

After lowering through the accepted x86 scalar tower formulas:

|Semantic entry|Original|Candidate|Difference|
|---|---:|---:|---:|
|Fq multiply, including square/norm entries|756|847|+91|
|Fq fused-two-product entries|1386|1382|-4|
|Fq inverse|0|1|+1|
|Fq2 nonresidue linear maps|385|429|+44|

Only four fused entries disappear, with substantially more multiply, inverse
and linear-map work plus fixed scratch. No weighted cycle model is applied.
The narrow componentwise domination predicate is FALSE, because fused
entries decrease; the separately required full-tradeoff review declines
native work. Preserve raw count_dominated=false and
native_selection_requires_tradeoff_review=true. They are not acceptance flags.

The implementation's five fixed arrays occupy28,736 requested data bytes:
16,384 selected compressed powers,8,192 numerator/denominator pairs,
2,048 norms,2,048 prefixes,64 activity flags. They replace the design's
preliminary three-array norm forecast; fraction storage was additional.
Compiler frames, local scalar/Fq12 temporaries and analysis/oracle allocations
are extra. No runtime allocation, RSS, pressure, minimum-RAM, physical mobile
or actual native stack-bound qualification is claimed.

Coverage limitations remain explicit. The identity tuple recovers one;
257 easy maps and all parameter/power checks pass. No nonidentity g3=0
decompression case was observed, so its implemented formula remains untested
in that branch. The deliberately malformed g1=1,g2=g3=g5=0 tuple is classified
invalid by a direct assertion; the model does NOT execute/catch the rejecting
recovery panic. Zero raw Fq12 is separately rejected by the easy map. The
design phrase "zero invalid compressed tuple rejection" is imprecise: the
all-zero COMPRESSED tuple is the valid identity. Do not infer arbitrary tuple
validation, native exception qualification or proofs from these checks.

The original final exponentiation and verifier are untouched. This model
studies the parameter exponent only; it does not benchmark a complete final
exponentiation or prove its removable wall-time fraction. No SIMD kernel,
SDK request, proof, hardware profile or AWS resource is added. Source176,
acceptedSDK6d83, canonical35171 and inherited about8ms local1x2 stay exact.
The original all-eight CPU10x goal remains ACTIVE/UNMET0of8.

First compile and first model execution succeed, with an unused-import
warning retained. No successful command, reference or model is rerun for
bookkeeping. No signed schedule, block/layout/worker/host rescue follows.
