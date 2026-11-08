# Fixed deferred normalization of affine coordinate offsets

Use accepted eight-lane R256 CIOS arithmetic unchanged through final radix conversion, but retain its unnormalized five result coefficients. Subtract canonical coordinate limbs and add a fixed modulus bias before its final carry pass. Arithmetic right shifts propagate negative coefficient carries. No new input-sized state or persistent plan.

Five operations, both BN254 primes, normalized product operands in[0,2p] and canonical offsets in[0,p):
1 mul_minus: U+p-x, U=REDC_R256(ab)<2p; normalize once, reduce2p once.
2 mul_minus_sum: U+2p-x-y; normalize once, reduce2p once.
3 mul_double_minus_sum: 2U+2p-x-y<6p; normalize once, reduce4p then2p.
4 sum2_minus: U+p-x, U=REDC_R256(ab+cd)<3p; normalize once, reduce2p once.
5 difference2_minus: same but replace c by the accepted normalized2p-c complement before CIOS. U<3p; normalize once, reduce2p once.

All five return normalized raw R256 limbs strictly below2p, congruent to the desired operation modp. They need not equal the original lazy representative modulo2p because a biasp may change its parity multiple ofp. Never expose an invalid canonical Fp. Product inputs remain normalized IFMA operands. Inclusive2p strengthens the edge test; real ordinary product operands remain below2p, while complements may equal2p.

The accepted cfgzopro_cios_cancel_carry final48-bit step exposes coefficients with integer value U before final radix52 normalization. Low converted coefficients are below3B, B=2^52; the high coefficient is below B from U<3p<2^256. For doubling/offset variants initial coefficients lie within(-2B,7B). Signed normalization stays within8B plus small integer carries, far insidei64. The mathematical model must verify exact raw-state equality with independent whole-integer REDC, low cancellation, all normalized limb bounds, residue and final ranges. Analytic bounds are necessary; finite maxima are supplementary.

This differs from the October22 mul_sub trial: that normalizes2p-c first and injects/realigns it before final cancellation. This proposal neither forms that normalized complement for the coordinate offset nor shifts it into the product. It also differs from November8 typed uncarried Xsum subtraction: the new offset is inserted before product-output normalization. Earlier trials remain closed; no old captures or successful qualifications are rerun.

Fixed intended caller integration after native qualification: G1 X=mul_minus_sum(slope,slope,ax,bx) and Y=mul_minus(slope,ax-X,ay); G2 X.real=mul_minus_sum(s0+s1,s0-s1,ax0,bx0), X.imag=mul_double_minus_sum(s0,s1,ax1,bx1); G2 Y.real=difference2_minus(s0,dx0,s1,dx1,ay0), Y.imag=sum2_minus(s0,dx1,s1,dx0,ay1). Original signed numerator, norm, prefix/inverse, scalar tails, input/bucket layout and schedules stay original. No native caller or timing is claimed by this field qualification. All output offsets are canonical values packed from original affine buckets/inputs, unlike the lazy product factors.

No IFMA count saving is claimed. Necessary savings are removed normalization/materialization passes; extra raw-bias ALU, signed shifts, reduction branches, register lifetimes and actual caller code must be priced. A valid primitive does not establish proof speed or a2x forecast. All five operations must be qualified without favorable subset selection before complete changing G1/G2 callers at1/16/32 with accepted/fresh controls and automatic profiles. Full verified SDK/all8+54/primary16/Gnark/generic/resources is required before adoption.

Rust1.98.1/xtask; accepted production176/SDK6d83 untouched, canonical36477 unchanged. No huge lookup table, key/point copy, retained solver plan, new heap buffer, production pool, C/ASM rewrite or fixed application RAM cap. CommitANDpush each finished experiment. No owned helper/build/hash/analyzer/decoder/verifier/report/push during any recording, including preflight. No performance recording is selected yet.
