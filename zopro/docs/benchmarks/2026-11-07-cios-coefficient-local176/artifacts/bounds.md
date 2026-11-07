# Prefix equivalence and range

Let B=2^52 and every normalized input/modulus digit lie in[0,B). Before a row, accepted coefficients t0..t5 are nonnegative. The input-product contribution to interior coefficient j is lo(aj*bi)+hi(a(j-1)*bi), plus the two analogous terms in fused mode. Calling this Uj only changes the parenthesization of a nonnegative integer sum.

The low frontier z is exactly the accepted t0 after its input low-half contributions. q=−z*p0^-1 modB is therefore identical. Since z+q*p0 is divisible byB,

    (z+q*p0)/B = ceil(z/B) + floor(q*p0/B).

The candidate coefficient1 adds this quotient, lo(q*p1), t1 and U1. Each other interior coefficient adds its accepted input terms, lo(q*pj), hi(q*p(j−1)) and prior tj. The top coefficient has only input high halves and hi(q*p4). After shifting, every coefficient equals accepted CIOS exactly. Induction proves equality after four rows. The fifth cancellation, radix48 packing when needed, normalization and fused correction are unchanged; they receive the identical coefficient state.

For a conservative bound, let Mr bound every coefficient before row r and M0=0. At most four input half-products and two modulus half-products reach a coefficient in fused mode. The extra low-frontier carry is bounded by ceil((Mr+2(B−1))/B). Thus

    M(r+1) = Mr + 6(B−1) + ceil((Mr+2(B−1))/B)

is a valid common upper bound. For B≥4, successive upper bounds are6B−4,12B−2,18B+6,24B+21. The last is below32B. B=2 has direct recurrence bound58<64. The model checks this recurrence at all52binary radices; this finite check supplements the inequalities rather than proving all symbolic inputs by sampling.

Every partial sum in the new nonnegative prefix is no larger than its completed coefficient; therefore it is also below32B=2^57. Uj alone is below4B. All52×52products fit u128, and z*inverse52 is below32B²=2^109. Candidate integer operations fit the declared interpreter widths and eventual64-bit lane accumulators. This is a prefix arithmetic bound, not a native stack/RSS/pressure bound. The unchanged final-row state has exactly the inherited accepted bounds.

The model compares each completed new row coefficientwise with the accepted product/reduction helpers and also with an independent whole-BigUint row recurrence. Full outputs agree with full-width quotient/REDC and modular references for both primes/radices and all three operations, including the inclusive2p complement edge. The old persistent-plane model and study main are not invoked.
