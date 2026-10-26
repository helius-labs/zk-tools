# Deferred correction inside a five-stage DIF tile

Let B=2^52 and R=B^5. Both BN254 primes satisfy p<2^254, hence 64p<R.
Inputs have normalized digits and numerical values in [0,2p). Every twiddle
is a canonical R260 residue in [0,p), including cached roots constructed by
the existing Ifma::new conversion. These root bounds are an explicit precondition.

At stage s=0..4, both operands are bounded above by M=2^(s+1)*p.
The sum is below 2M. The biased difference M+a-b is nonnegative and below
2M, so its five radix52 digits can be normalized by one signed carry chain.
Twiddle multiplication has T<64p*p<pR and ordinary REDC is below2p.
There is no mixed-radix multiplication, fused sum2 or changed cancellation.

In five ordinary DIF stages, 80 low outputs each undergo a conditional
subtraction, while high-output subtraction also normalizes its conditional
borrow correction. The new schedule uses plain normalized sums and positive
biased differences until the tile ends. An output's trailing low branches
determine its final range: 2^(t+1)*p, where t=min(trailing_zeros(index),5).
Normalize it with conditional subtractions 2^t*p, ..., 2p. Across32 outputs
there are 8+8+6+4+5=31 such passes. All outputs then satisfy [0,2p),
so existing compact storage and host conversion retain their contracts.

The bias is represented as five coefficients (split(p)[j]<<shift), not a
truncated four-word integer. Each bias coefficient is below32B. Adding a
normalized input digit and subtracting another, plus the signed carry,
stays below34B in absolute value, hence below2^58. A correction subtraction
has the same conservative bound. Carry extraction uses signed shifts and
the final nonnegative value is belowR, so there is no remaining top carry.

Existing CIOS convolution accumulators still multiply normalized digits belowB;
their per-row additions are bounded by the existing full-digit operand argument.
At most five prior rows and the current product/cancellation contribute fewer
than32B to any coefficient, below2^57. There is no64-bit overflow. The looser
input value changes the output-value bound, not an IFMA multiplicand's52-bit
contract. The independent integer model and raw-limb oracle must check this
argument rather than relying only on modular output equality.

This is a local derivation inspired by the delayed-reduction principle in
[Scott, Slothful reduction](https://eprint.iacr.org/2017/437). Its abstract was
read, but the full PDF request returned403. No claimed benchmark percentage
is imported from that paper. No new performance result is established by these
bounds. No extra buffer, table, plan field or persistent representation follows.
