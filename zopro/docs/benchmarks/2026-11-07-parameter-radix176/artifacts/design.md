# Curve-parameter radix Montgomery screen

Freeze before execution, 7 November 2026. Hypothesis: with u=4965661367192848881,
Fq=36u^4+36u^3+24u^2+6u+1 and Fr=36u^4+36u^3+18u^2+6u+1.
Both moduli have least radix-u digit 1. Five coefficient Montgomery rounds with
R=u^5 can therefore cancel using m=-c_i mod u, without a modular inverse word
multiply. The small modulus coefficients admit shifts/adds. This is distinct
from the existing six-coefficient PMNS and two-coordinate quadratic screens.

Use nonnegative normalized inputs below 2p, five radix-u digits. The first four
digits are below u (63 bits), the fifth below 128 (7 bits). Keep exact unsigned
128-bit convolution coefficients. For each of five low coefficients, choose m,
add m*p, divide that coefficient exactly by u, and carry to the next. Normalize
the remaining five coefficients with four quotient/remainder operations.
Check both moduli, ordinary multiplication, fused sum2, and difference via the
nonnegative complement 2p-c. Interpret output in Montgomery radix R, not R256.

Independent BigUint products and extended-Euclid radix inverses supply the
reference. Use boundary values around every radix power within [0,2p), fixed
full-width pseudorandom values, changing chains, and an exhaustive small-radix
control. Derive conservative coefficient bounds for ordinary/fused inputs and
check lazy output closure. Finite arithmetic checks are not SIMD qualification.

Count a fixed 52+11-bit decomposition of each 63-bit digit. For an IFMA-only
recipe, each partial product requires its low half and its high half when its
width exceeds 52. Also report a separate mixed recipe using VPMULLQ whenever
the full partial product fits 64 bits. Neither count prices dependencies,
shifts/adds, packing, reciprocal-based 128/u division, constants or spills.
Do not invent a scalar or vector division instruction. Report both recipes;
do not infer that IFMA and VPMULLQ have interchangeable cost.

Continuation condition: advance only if the complete explicit arithmetic gives
a credible ordinary/fused work reduction over 100/150 IFMAs, including a
specified vector normalization/reduction. Otherwise close this exact recipe
before native code, with no width/radix/shape rescue. This is not a lower bound
on other representations or a CPU timing experiment. Accepted source176 and
SDK6d83 stay unchanged; canonical proof ledger33949 and all8 CPU/Metal numbers
remain inherited. No key data or key-sized tables are needed.
