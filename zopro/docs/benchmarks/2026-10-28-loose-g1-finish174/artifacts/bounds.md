# Bounded loose coordinate finish

Let R=2^256. Both supported BN254 primes satisfy 5p<R; checked compile-time
word additions prove this without floating-point approximations. Inputs are
normalized Montgomery integers 0<=s<2p, 0<=ax,ay<p and 0<=S<2p.

Unchanged nonfused CIOS computes REDC(a*b)=(a*b+q*p)/R, 0<=q<R.
Consequently 0<=u=REDC(s*s)<p+4p²/R<1.8p<2p.
Keep X0=2p+u-S in [0,4p) and D0=2p+ax+S-u in (0,5p).
All values fit below R and each normalized digit fits IFMA's 52-bit input.
Using the unchanged nonfused CIOS recurrence on s*D0 gives
0<=V<p+10p²/R<3p. Set Y0=p+V-ay in (0,4p).
These satisfy X0=u-S (mod p), D0=ax-X0 (mod p) and Y0=s*D0-ay
in the same Montgomery representation. This does not permit passing wide
values to arbitrary existing field operations: D0 is consumed locally by
one nonfused multiply, and X0/Y0 go directly to the local export.

For any Z in [0,4p), one conditional subtraction of 2p followed by one
conditional subtraction of p returns its canonical residue. Raw export then
packs normalized digits into host Montgomery words without another reduction.
Three signed carry passes normalize X0,D0,Y0. Each coefficient is bounded by
four radix-52 digits in magnitude, well below signed64; sign extension is
required. Ordinary CIOS coefficients with 52-bit inputs retain their original
accumulation bound; the wider whole integer changes output range, not per-digit
product bounds. Product nonfused has no final 2p correction or bound-dependent
branch. The final 48-bit cancellation remains exact because all operands<R.

Only native eight-lane FAST G1 streaming uses this primitive. Bucket coordinates
are canonical because ordinary scalar/Fp operations and every stream export
produce canonical host coordinates. ax+bx uses the existing canonical-input
addition and lies below 2p. Slope is an ordinary nonfused product, below 2p.
Scalar tails, general non-FAST streams, all G2, trees, signed numerator,
inversion, scratch, query geometry, solver and scheduler remain original.

This differs from the closed weighted-Y anchor (which changes a forward
record), lazy-linear quotient estimator (which preserves <2p at each operation),
and post-REDC mul-sub fusion (which returns the original lazy representative).
The current method uses local loose bounds and canonical output equivalence;
no standalone primitive or earlier component speed is used to predict latency.
