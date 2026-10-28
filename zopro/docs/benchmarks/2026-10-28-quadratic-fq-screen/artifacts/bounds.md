# Exact bounds and template work

Write w²+e*w+d=0 and pi=A+B*w, so N(pi)=A²-eAB+dB²=p. The two tested bases are (d,e)=(3,0), A=6u²+3u+1, B=u and (d,e)=(1,1), A=6u²+4u+1, B=2u. Both identities follow by expanding the BN254 prime polynomial. The root w=-A/B modulo p satisfies its defining equation. The conjugate is A-eB-B*w, so pi is invertible modulo any power-of-two radix R because its odd norm p is invertible there.

For T=x*y, compute m=-T/pi modulo R, center both coefficients of m in [-R/2,R/2), and set z=(T+m*pi)/R. Both numerator coefficients divide exactly by R. Evaluation at the field root gives z=T/R modulo p. This is a ring Montgomery identity. It is not a native coefficient implementation or an optimized conversion.

For all input coordinates bounded by H in absolute value, direct triangle bounds give

    |T.real| <= (1+d)H²
    |T.imag| <= (2+e)H²
    |z.real| <= ceil((1+d)H²/R) + ceil((|A|+d|B|)/2)
    |z.imag| <= ceil((2+e)H²/R) + ceil((|A-eB|+|B|)/2).

Here H=2^126+3B. These are sufficient rectangular-box bounds. Failure of an inequality does not prove a range failure; the archived sqrt(-3) R130 counterexample does. The fixed Eisenstein R130 samples provide no counterexample, and are not a proof of closure. Both R131 inequalities pass for both bases. Real/imag coordinates remain signed128, but each expands into three native radix52 vectors, six total.

The exact conversion oracle rounds the two coefficients of z*conjugate(pi)/p to nearest integers, then subtracts that multiple of pi. Its output coordinate bounds are (|A|+d|B|)/2 and (|A-eB|+|B|)/2. These fit inside H. Addition in the changing-chain model uses this exact oracle reduction; no efficient native normalizer or import/export path has been implemented.

The specified three-product row forms P=x0*y0, Q=x1*y1 and S=(x0+x1)*(y0+y1), then real=P-dQ, imag=S-P-(e+1)Q. Small integer factors require extra additions/shifts; those costs are not counted as IFMAs. Each row spans three coefficient digits, two full52-bit cancellation rounds and a final mixed-bit round. The modulus's second coordinate has only two nonzero52-bit digits, permitting that zero-constant product to be skipped. All three low cancellation multipliers are nonzero in the tested parameters.

For R130 the last divisor is2^26. Coefficient3 at2^156 becomes2^130 in the quotient, zero modulo R130, so its three raw and two nonzero-modulus high halves may be omitted. Counts: raw51 + modulus46 + cancellation9 =106.

For R131 the last divisor is2^27. The same coefficient becomes2^129 and survives modulo2^131. Retain the five high halves: raw54 + modulus48 + cancellation9 =111. Merely increasing the radix of the prior R130 kernel while retaining its dropped coefficient would be incorrect. The dedicated test checks that algebra independently.

The current ordinary five-limb CIOS cancellation-carry template uses five rows: 50 raw IFMAs, five low cancellation IFMAs and45 modulus IFMAs, total100. This is a source-template count, not an optimized machine-code count. It excludes input/output conversion, carries, normalization, copies, stack traffic, comparison and all memory effects. The quadratic counts exclude those costs too. Thus this study identifies no operation-count advantage for the specified construction; it does not prove that every quadratic implementation must be slower.

The earlier rejected Gaussian Fr design already had three coupled rounds and a small second modulus coordinate. Its scalar-field norm is (6u²+3u)²+(3u+1)². Those properties cannot be counted as a new advantage of this Fq proposal. A different reduction schedule or invariant would need an independently justified advantage before native work.
