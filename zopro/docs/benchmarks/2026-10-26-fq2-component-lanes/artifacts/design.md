# Real and imaginary components in one vector

For four Fq2 values, let a=[a0,a1] and b=[b0,b1], each half holding
four independent Fq values. Existing half extraction/join operations form
A=[a0,a0], C=[-a1,a1] and D=[b1,b0]. Then the existing fused operation
A*b+C*D returns [a0*b0-a1*b1, a0*b1+a1*b0]. Each value retains the
accepted mixed R256 Montgomery domain and the below-2p lazy contract.

The lower half of C is negated using the accepted four-lane zero.sub(ai).
Its zero input and lazy ai are within that operation's existing bounds;
no unsigned negative limb is passed to IFMA. Joining and extracting halves
changes neither limbs nor radix. Squaring packs the accepted expressions
(a0+a1)*(a0-a1) and a0*a1 into one eight-lane product, then doubles the
imaginary half. Norm and real-factor helpers are component operations too.

This layout computes four Fq2 elements per vector, rather than eight
elements in two vectors. Two packed states provide the exact same eight
results as the reference. It does not halve the total data or point work.
The intended opportunity is register allocation in the complex expression;
additional permutations, extraction and four-lane carries may outweigh it.
The prior four-lane whole-MSM, three-product raw Karatsuba and dual-CIOS
experiments do not use this layout and remain rejected.

No cryptographic security claim or production change follows from a field
component. Complete streaming inversions, bucket scheduling, packing,
memory admission, full SDK verification and portable behavior are outside
these timed expressions. The candidate must first pass independent field
and complete-curve oracle checks and the frozen component gate.
