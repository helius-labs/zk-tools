# Exact diagonal recurrence and bounds

Let B=2^52 and split each input/modulus into five B-digits. Inputs are normalized
and at most2p; p is BN254 Fq/Fr. Each a_i*b_j adds its low52 and high52 halves
to the anti-diagonal accumulators L,H. For fused products also add c_i*d_j.
On low diagonal k add the preceding q_i*p_(k-i), i<k, plus prior carry to L.
Set q_k=(L*INV) mod2^bits, bits52 except final mixed step48.

The canceled coefficient carry is
ceil(L/2^bits)+floor(q_k*p0/2^bits)+H*2^(52-bits).
The identity follows because L+q_k*p0 is divisible by2^bits. For48bits,
q_k<2^48, so IFMA-high52(q_k<<4,p0) computes the quotient term exactly.
This includes all of diagonal4's high half; do not add it again.

For high diagonals k=5..8, add all raw and reduction products with i+j=k.
Mixed: W=carry+16L; output_(k-5)=W modB; carry=floor(W/B)+16H.
Uniform control: omit both factors16. Final output4 is carry. Reconstruction
is exactly U*R=T+Q*p, not just equality modulo p. The independent full-integer
reference performs successive conventional four52+one48/52 REDC steps.

Before the final cancellation, at most10 raw plus4 reduction products occur.
For52-bit low steps, carry<16B and L<30B,H<14B are conservative inductive
bounds. At the final48-bit step, carry<226B. The next diagonal has at most12
total products, so W<418B<2^61 and its outgoing carry<193B. Later diagonals
have at most9,6,3 products and smaller bounds. Every intermediate sum/shift
fits u64; IFMA operands remain<2^52. The model checks these bounds for its
fixtures, in addition to this conservative argument.

The usual Montgomery bound U<p+T/R gives U<2p for ordinary inputs<=2p,
and U<3p for fused mixed products. Existing fused subtraction of2p brings
the result below2p; all other paths already do. Both primes have8p<2^256.
Therefore final output4 also fits52bits. Reuse the existing correction;
do not substitute a canonical correction or change lazy representation.

Model outcome:38176 complete prime/radix/product cases and32 separate
carry-identity cases pass. Corners include exact2p and power-boundaries;
4096 full-width deterministic random tuples per prime. These are arithmetic
checks, not SDK/proof or performance measurements.
