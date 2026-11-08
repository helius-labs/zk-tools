# Native lowering refinement, declared after first mathematical model

The original signed affine model has completed successfully after the preserved
one-term raw-product correction. Do not rerun it. This file declares a NEW
algebraic lowering check before executing its helper, with no timing or gate edit.

Let S=2^260, R=2^256/2^260 and k=S/R=16/1. Write each signed input
a=A-s_a*S with A=a modS and s_a=[a<0], assuming |a|<S/2. For one product,
ab=AB-S(s_a*B+s_b*A)+s_a*s_b*S^2.
The unsigned and signed product therefore have exactly the same Montgomery
quotient moduloR. After unsigned REDC, compensate at the END by subtracting
k*(s_a*B+s_b*A). The remaining s_a*s_b*k*S is zero moduloS. Center the quotient
by subtracting p when its final digit's high bit is set, normalize low four52-bit
digits, then sign-extend the top52-bit digit. Provided the exact final result lies
in(-S/2,S/2), this modular signed decoding recovers that exact representative.

Fused sums use two unsigned products, with compensation for all four input signs.
A difference encodes -c as its signed third input. That negation/normalization is
an actual native cost; no free difference2 instruction or missing complement is
assumed. This keeps the original unsigned six-coefficient CIOS schedule; it does
not require an eleven-coefficient signed convolution in registers.

New helper models the accepted canceled-low-product CIOS recurrence literally:
four52-bit rows, final48/52-bit row, unsigned IFMA low/high pieces, ceil(t0/B)
carry, mixed48 repacking, then NEW late compensation/quotient centering/signed
normalization. It counts100/150 IFMA recipe sites (raw50/100 + reduction50),
and checks every coefficient state against signed64. This is an integer model,
NOT actual native instruction decoding or latency. The original signed-convolution
model is not executed again.

Fixed population: Fq/Fr xR256/R260 xordinary/sum2/difference2. Thirteen signed
edges as in the first design, all169 first-input pairs with deterministic other
edge choices, plus1024new signed random tuples per configuration (new RNGseed).
Check the full unsigned CIOS result against an independent full-radix Euclidean
quotient, every quotient digit, same unsigned/signed quotient, and the final exact
signed output against direct full-radix centered REDC of signed T. Check final
residue, signed magnitude and top-word decode. No old Arkworks/curve/oracle
population is rerun; those passed checks are inherited by source/result hashes.

Conservative bound for each tested tuple with |a|..|d|<=5p:
|out|<=50p^2/R+p/2<S/2 for both primes/radices. Model checks the exact integer
inequality. All raw unsigned inputs use full52-bit limbs; prior public Ifma8 input
contracts are NOT widened. A future implementation needs a private type/kernel
and new native proofs for this broader raw-input domain.

Literal additional work: up to10 masked limb subtractions per ordinary product
or20 per fused product, with one/16 scale (the latter needs shifts); five masked
modulus subtractions for centering; final signed top interpretation; signed
linear normalization/negation; wider final coordinate export. Input sign masks
and raw operands remain live. Existing instruction counts do not price any of
these costs. Source IFMA equality is neither a speedup nor sufficient selection.
Native review must retain full G1/G2 caller costs, both curves, tails/exception
fallback, shared-small-stack workers and no extra key/table/retained plan/pool.

Pass selects a private native implementation/codegen qualification, not adoption
or performance. A credible actual caller result still needs complete SDK/all8+
54/primary16/Gnark/resource/portable qualification. Original goal remains active.
