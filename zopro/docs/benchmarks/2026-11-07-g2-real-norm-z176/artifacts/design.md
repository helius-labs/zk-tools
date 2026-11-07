# Real-norm common-scale G2 algebra/work screen

Selected after closing the fixed G1 segmented implementation at b52a5499a8.
That implementation and its failed gate remain closed: no worker, selector,
layout, compiler, endpoint or SDK rescue is selected. This is a different
denominator construction that needs new algebra and operation/resource evidence
before any runtime candidate. It is not the prior G1 kernel relabeled as G2.

For G2 over Fq2, a naive common-Z lift uses D=product(dx_i) in Fq2 and complex
all-other cofactors. Instead use n_i=norm(dx_i) in Fq, D=product(n_i) in Fq,
q_i=product of the other norms, and u_i=dy_i*conjugate(dx_i)*q_i in Fq2.
Then u_i/D=dy_i/dx_i, so

    X'_i = u_i² - (X1_i + X2_i)*D²
    Y'_i = u_i*(X1_i*D² - X'_i) - Y1_i*D³
    Z'   = Z*D.

Starting from real Z=1 keeps Z in Fq. Cofactor products, D²/D³, carried-point
rescalings and final common normalization use Fq rather than Fq2. This changes
executed products and storage. Retain the real-norm formula at singleton levels
as well; do not introduce a favorable singleton/worker/size policy after counts.

The accepted G2 batch reducer already uses norms for ordinary inversions. That
fact alone is not a new optimization. The new question is retaining a common
real scale through multiple tree levels, without those intermediate inversions,
while comparing with that actual norm-based accepted recipe, including its fused
sum2/difference2 operations. A complex multiplication and a fused two-product
operation must not be priced as the same single ordinary Fq multiplication.

First implement only an independent algebra/count/resource model. Compare
real-norm lift, generic Fq2 common-product lift and accepted affine/norm recipe,
reporting elementary and fused units separately. Include denominator norms,
cofactors, all coordinates, odd/singleton carried groups, untouched survivor
rescaling, final normalization and complete exceptions. Derive peak live payload
from actual lifetimes. No constant table, duplicate key/plan, new production
pool, fixed RAM cap, C/ASM backend or field-library rebuild.

Qualify nontrivial Fq2 values, real common scales, zero-denominator/norm rejection,
odd/even and multiple groups, independent affine group outputs, input immutability
and finite-prime rational identities. Reuse accepted field qualification and
existing original-oracle identities where available; do not replay the completed
G1 or old native/core/prover suites. Count lengths2,3,4,7,8,15,16,31,32,63,64,
127,128,255,256,511,512,1024,4096. No latency, native benchmark, threshold sweep
or adoption is authorized by this algebra screen.

Initial history search found no report implementing the real-norm common scale;
broad regex matches included current G1 prose and old norm-based inversion/spill
studies. Inspect relevant original source/count reports before claiming new work.
The source of truth for current norms/products is msm.rs::BatchInverse for Fq2:
norm uses fused sum2, complex products use paired sum2/difference2 on native IFMA,
complex square uses two ordinary Fq products. Counts do not imply native speed.

Select native work only if this changes the useful work/memory decision against
the actual accepted caller. Otherwise record the screen and move to another
substantial main-MSM/prover idea. Full verified all-eight CPU10x objective remains
active/unmet0/8, with CPU/Metal comparisons; accepted1x2 stays7.915095ms and the
canonical distinct-proof ledger stays31387. No new proof or AWS resource.
