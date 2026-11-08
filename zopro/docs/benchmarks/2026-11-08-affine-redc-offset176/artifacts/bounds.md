# Deferred-offset range argument

Let B=2^52, R=2^256 and p be either pinned BN254 prime. Both have4p<R. Every product input has normalized limbs belowB and numerical value at most2p. The accepted difference complement2p-c may equal2p. All coordinate offsets x,y are canonical numerical R256 words, belowp.

For T=ab, choose q in[0,R) with T+qp divisible byR. Then U=(T+qp)/R satisfies0<=U<p+4p²/R<2p. For T=ab+cd, including the normalized negative complement, T<=8p² givesU<p+8p²/R<3p. The native prefix/cancellation/radix-conversion body is copied exactly from accepted CIOS, with its R256 branch fixed, so these are the same integerU before final normalization.

An interior CIOS coefficient receives at most two product halves per source product and two reduction halves per row. Carry from the canceled digit is a small integer plus one high product half. A conservative induction below64B covers all five rows (even the radix48 carry is below1025, far belowB). Every q/p/input IFMA factor is belowB; the final q<<4 is belowB becauseq<2^48. No negative coefficient enters IFMA.

The native radix conversion gives:
- u0=ceil(t0/2^48)+hi52((q<<4)*p0)+16*(t1 mod2^48), henceu0<2B+1025<3B.
- u1..u3=floor(ti/2^48)+16*(t(i+1) mod2^48), belowB+1024<2B.
- u4=floor(t4/2^48)+16*t5. All converted coefficients are nonnegative and their weighted sum isU<3p<2^256; thereforeu4<=U/B^4<2^48<B.

The conversion identity follows by splitting each ti into its low48bits and high quotient: B=16*2^48. It does not require normalizing the individual coefficients first.

For the five fixed operations, the biased whole result before normalization is:

|Operation|Whole integer|Bound|
|---|---|---|
|mul_minus|U+p-x|[0,3p)|
|mul_minus_sum|U+2p-x-y|[0,4p)|
|mul_double_minus_sum|2U+2p-x-y|[0,6p)|
|sum2_minus|U+p-x|[0,4p)|
|difference2_minus|U+p-x|[0,4p)|

Bias limbs are belowB, and each canonical offset limb is belowB. Initial signed coefficients lie strictly between-2B and7B; arithmetic carry normalization keeps them within8B, since incoming carries are small integers. Thus signed64 arithmetic has ample room:8B=2^55. Use arithmetic right shifts, not logical shifts, for negative coefficient carries. Low four limbs are masked after their carries; the high limb becomes the quotient of a nonnegative whole integer below6p byB^4, below2^49<B.

Ordinary and fused offset results are below4p, so one conditional subtraction of2p suffices. The doubled result is below6p: subtract4p when applicable, leaving a value below4p, then conditionally subtract2p. Both correction moduli fit four words because4p<R. Early rejection based on a strictly smaller high normalized limb is sound; equal high limbs must execute the complete borrow chain. The raw reducer returns arrays, so wider intermediate representatives are never exposed as normalized-lazy public arithmetic objects.

All additions ofp and subtractions of2p/4p preserve the field residue. Final limbs are normalized and the whole result is strictly below2p. A biasp can change the lazy representative by an odd multiple ofp; equality is required after canonical export, not necessarily modulo2p before export.

The exact interpreter independently checks every cancellation against whole-integer REDC and checks every signed carry and final residue. Actual native tests check canonical exports against Arkworks and the unchanged sequences. Finite populations supplement this range argument; they do not prove point/prefix/tail/exception integration or any speed/RAM result. The offset canonicality requirement must remain explicit in every future selected caller path.
