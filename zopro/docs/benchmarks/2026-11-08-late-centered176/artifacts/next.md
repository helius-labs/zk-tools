# First unfinished work: native late-centered qualification

Both exact scalar stages are done; never rerun them or their original oracles for
bookkeeping. The failed first raw product and one-term repair remain explicit.
No native implementation, timing, SDK request or proof exists for this recipe.

Implement ONE private native type/kernel using the existing unsigned six-word
CIOS body and late sign compensation, keeping R256 for actual G1/G2 callers.
R260 is a mathematical cross-check, not a new coordinate scaling candidate.
Do not modify accepted Ifma8/Fp contracts, field library or production dispatch.
Keep low four digits normalized and top signed; linear operations can remain
loose but must normalize before unsigned encoding. Encode five52-bit digits
moduloS, retain sign masks, execute ordinary/fused unsigned CIOS without its old
<2p/fused correction assumptions, then compensate signs and center the last
quotient, carry and sign-extend. Difference2 must pay negation of its third input.
Use original100/150 source IFMA schedule, not the prior signed pair factorization.

Canonical host key and buckets remain untouched and compact. G1 scratch can keep
the same five vector fields; G2 retains its two-vector prefix records and original
coordinate rereads. No key/point copy, LUT, input-sized auxiliary array, solver
plan or pool. New native raw bounds must cover all52-bit unsigned operands, signed
compensation/normalization and mixed48 packing under64-bit vector arithmetic.
An integer-model pass is not an unsafe/native proof.

At the inverse bridge, signed roots <p normalize once before the unchanged
eight-lane product/scalar inverse; pay every conversion. Final coordinate export
is wider: G1 |X|/|Y|<3p and G2<4p. One fixed export can bias by4p, normalize to
[0,8p), use the existing one-digit reciprocal strategy then canonical correction,
and transpose to host words. That requires its OWN exact/native tests; do not
reuse get/compact outside their bounds or price export as the ordinary <2p case.
No coefficient/radix/layout/compiler sweep if the fixed caller loses.

New native tests need both primes/radices/operations, signed boundary and changing
outputs, complete G1/G2 full/tail/exception handling via original fallback,
key preservation, capacity/allocation behavior and1/4/16/32 shared-small-stack
workers. Inherit old unchanged field/core suites. Inspect actual final linked
G1/G2 caller codegen once, preserving sign/normalization/export costs. Only after
native qualification freeze matched original-source/freshOFF/ON complete-caller
timing/profile/resource gates. A component win then requires actual SDK primary
and ALL8+54/primary16,Gnark,portable/resource qualification before adoption.

The arithmetic count stays6M per G1 group and7M+5F per G2 group, plus the unchanged
inverse bridge. Potential savings are linear modular corrections; additional
masked compensation, shifts, signed normalization, quotient centering, exports,
live operands, compiler stack and branches can erase them. No speed forecast.
The all-eight CPU10x goal, especially primary3.7ms, remains required and unmet.
