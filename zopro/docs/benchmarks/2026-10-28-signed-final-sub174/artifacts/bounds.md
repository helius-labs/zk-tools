# Final REDC carry folded with subtraction

Let B=2^52, p be either BN254 prime, and R=2^256 or2^260. All product input
digits are normalized and a,b<2p; c has normalized digits and0<=c<=2p. Ordinary
Montgomery REDC gives r congruent to a*b/R modulo p, with0<=r<2p. This
experiment leaves all product/cancellation and mixed-radix packing instructions
unchanged. The final five not-yet-normalized coefficients t represent EXACTLY r.

The existing CIOS bound is below31B per coefficient before packing. The mixed
packing has a conservative upper bound512B=2^61, including propagated partial
limbs. Uniform radix is smaller. Subtracting a digit c_j<B and adding the signed
carry therefore remains strictly within signed64 range. An arithmetic right
shift by52 computes floor(value/B), including negative values; masking produces
its Euclidean residue in[0,B). The carry identity v=(v mod B)+B*floor(v/B)
preserves the represented integer at every step.

After five digits the carry is0 or-1, since r-c lies in[-2p,2p) and2p<B^5.
The low digits represent (r-c) mod B^5. If negative, add2p, normalize and discard
the B^5 carry. This yields r-c+2p in[0,2p); otherwise r-c is already in[0,2p).
The exact c=2p case is allowed, including r=0, and returns zero. Results are
bit-for-bit equal to the existing normalized-multiply-then-subtract sequence,
not merely field-congruent. All outgoing digits and lazy bounds are unchanged.

Compared with22Oct mul-sub, this does NOT normalize2p-c or inject it before final
Montgomery cancellation. Compared with24Oct canonical-Y, subtraction occurs
before output normalization. It eliminates the standalone product's four carry
links, retaining the existing subtraction and conditional negative repair.
These are source/dataflow facts, not a measured saving. Holding c live through
CIOS may spill. Compiler scheduling and real callers must decide performance.

Two tests check coefficient renormalization identities and full native
mul-sub outputs. The latter compares exact lazy digits with the old sequence
and field values with independent Arkworks arithmetic, across both fields,
both radices, boundaries, c=2p, full-width loose inputs and dependent chains.
Runtime test-only coefficient assertions check the conservative2^61 limit.
This derivation plus finite checks is not a formal machine-code proof.
