# Three-row quotient prefix

Let B=2^52, p odd, I=-p^-1 modB^3, with normalized digits I0/I1/I2. Form just the first three input product rows T=A*(b0+B*b1+B²*b2), plus the corresponding fused C/D rows when enabled. This is not the complete five-row input convolution.

The first three normalized low digits are U0=t0 modB, V1=t1+floor(t0/B), U1=V1 modB, U2=(t2+floor(V1/B))modB. IFMA may consume U2 unmasked because only its low half is used. With lo/hi denoting exact52bitproduct halves:

q0=lo(U0,I0)
v1=hi(U0,I0)+lo(U0,I1)+lo(U1,I0)
q1=v1 modB
v2=hi(U0,I1)+hi(U1,I0)+lo(U0,I2)+lo(U1,I1)+lo(U2,I0)+floor(v1/B)
q2=v2 modB

Then Q=q0+B*q1+B²*q2=(T*I)modB³, so T+Qp is exactly divisible byB³. The carry from v1 into the next inverse-product digit is essential. Bounds v1<3B and v2<6B hold. Individual native low/high contributions fit unsigned64; the complete model checks both digit/carry recurrences against an independent BigUint inverse/product.

Apply q0/q1/q2 with the accepted cancelled-low identity, one offset at a time: (t0+q*p0)/B=ceil(t0/B)+floor(q*p0/B). The future quotient digits no longer depend on the preceding cancellation's output. The returned six coefficients represent (T+Qp)/B³ exactly. Original row3 and row4 are then unchanged, including final48-bit cancellation/packing for R256 and52-bit for R260, final normalization and fused correction. Each input row beyond the prefix is divisible byB³, so withholding those rows cannot change the low156-bit quotient.

Both primes<2^254, multiplicands<=2p imply topdigits<2^48. At most100raw and45modulus half contributions occur in a complete fused product. Ignoring discarded coefficients gives a conservative prepacking sum below146B including five small carries, closing a256B envelope. First-prefix inverse temporaries add only quotient-computation terms, v1<3B,v2<6B; they do not enter state coefficients except through normalized q. Mixed final packing masks48bits beforeleftshift4; its high t5<3*2^44 term is safe. Allworkingcoefficients remain below2^61. No overflow/rare-carry assumption.

For full input X<=4p² ordinary or8p² fused, exact REDC=(X+Qtotal*p)/R<p+X/R. This is<2p for ordinary, <3p for mixed fused (existing2pcorrection), and<2p for uniform fused. Output digits, numerical Montgomery radix and exact lazy representative match ordinary CIOS. Native tests compare raw digits as well as canonical values and bounds.

First-prefix quotient work costs9IFMAs instead of3. Total106ordinary/156fused versus100/150accepted and104/154paired2prefix. It uses8partialcoefficientvectors duringprefix, then6fororiginalremainingrows, plus3quotientdigits/inversewords. Two extra logical inverse digits cost16bytes per field (excluding vector constant materialization). Constant generation is exact bit lifting at compile time, tested against independent model constants. There are no per-key tables, heap buffers, duplicated plans or host point conversions. Register pressure can lose more than shorter dependencies gain.
