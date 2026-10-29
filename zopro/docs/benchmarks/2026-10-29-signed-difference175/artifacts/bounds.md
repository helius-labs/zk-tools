# Direct signed product difference bounds

For normalized stored a,b,c,d <=2p and R=2^256 or2^260, both BN254 primes
satisfy4p<R. Set Z=ab-cd. Then -4p²<=Z<=4p². Choose the usual Montgomery
quotient q=(-Z/p) modR in[0,R). T=(Z+qp)/R lies strictly between -p and2p.
Thus adding p only when T<0 yields a normalized representative in[0,2p).
This does not need the existing positive two-product REDC's2p subtraction.

The private helper subtracts the IFMA halves of c*d directly from six signed
CIOS coefficients. Low-digit cancellation uses the exact signed identity
ceil(t0/2^k)+floor(m*p0/2^k), k=52 or48. Low quotient digits still depend only
on the low k bits, including negative coefficients. Arithmetic shifts preserve
signed carries and mixed-radix repacking. The optimized cancelled-low identity
is always selected in this helper, independently of the parent's cfg switch.

Each coefficient receives bounded product/reduction halves per row. A loose
31*2^52 absolute envelope covers five ordinary rows and small propagated
carries; mixed repacking multiplies that envelope by at most16, below2^61.
Two's-complement vector add/sub is therefore an exact signed representation,
not a saturation or probability assumption. After the final arithmetic ripple,
T<0 iff its top coefficient is negative. Add masked modulus digits, ripple
again, and return. Lazy representatives can differ from the original method.

This is distinct from the old complement seed/correction and shared three-raw-
product Gauss schemes: no extra Rd term, complementary operand, inter-output
sharing or new representation. It retains four raw products/two reductions
per complex multiplication. Removing operand negation/positive fused correction
adds per-half vector subtraction and signed correction; it may be slower.
No lookup table, heap storage, per-key preparation or proof buffer is added.

The independent BigInt model compares quotient, signed integer CIOS state,
repacking and exact corrected representative across both primes/radices,
inclusive2p boundaries, full-width inputs and changing chains. Separate native
Arkworks lane tests exercise actual machine operations and output bounds.
