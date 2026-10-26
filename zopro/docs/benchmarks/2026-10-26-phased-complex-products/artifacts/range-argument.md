# Phased complex multiplication

This is a dataflow alternative to the earlier simultaneous signed CIOS routine.
It uses the same three-product identity, rather than claiming a new bilinear
multiplication count. Write B=2^52, R=B^5 (uniform) or 2^256 (mixed).

For 0<=ai,bi<=2p and p<2^254, exact sums are <=4p<2^256.
A=a0*b0 and B1=a1*b1 lie in [0,4p²]. D=A-B1 lies in [-4p²,4p²].
Set S=-(A+B1) while constructing D. Finish signed REDC(D) first, then bias
by p. The low-digit cancellation chooses m in [0,R), so REDC(T)=(T+mp)/R.
Since 4p/R<1, REDC(D)+p lies strictly between zero and3p. Subtracting2p
conditionally gives a normalized representative below2p.

After the real result is complete, form exact a0+a1,b0+b1 and accumulate
C=(a0+a1)*(b0+b1) into S. Its final integer value is the nonnegative cross
product a0*b1+a1*b0 <=8p². As8p/R<2, REDC(S)<3p. The same single2p
correction suffices. Signed intermediate accumulators use arithmetic carries.
IFMA multiplicands are normalized nonnegative52-bit digits throughout.

Each raw convolution coefficient sums at most nine low/high products and is
<9B. Coefficientwise D is in(-9B,9B); S in(-18B,0]. Adding C before reduction
leaves each coefficient in(-18B,9B). Montgomery cancellation adds at most
five low and five high terms to a coefficient (<10B in total). A processed
coefficient's arithmetic carry has magnitude at most32. Each receiving
coefficient receives at most one such carry per step (five steps), so32B
is conservative throughout ordinary cancellation and normalization. For
mixed radix, the final48-bit division packs up to16 times a coefficient,
plus an arithmetic carry and the p bias; the resulting coefficient fits
below2^62 in magnitude. Values ultimately fit five normalized radix52 digits.
An exact integer oracle and adversarial SIMD tests check the implementation;
finite tests supplement, rather than replace, this argument.

The two wide accumulators occupy20 logical vectors while forming A/B. During
real reduction S remains ten vectors. During the third product, the reduced
real result is five vectors and the imaginary accumulator ten vectors. This
is a logical dataflow description, not a register-allocation or peak-stack
bound. LLVM may retain or reload source operands; disassembly is required.
There is no heap, lookup table, transformed key, persistent cache, scheduler
change or global allocator/registry modification.
