# Trace-one Fq2 basis arithmetic screen

Declared before native build or timing, 29 October 2026.

Accepted production is the 175-file balanced-K/H snapshot, SDK 20fde19f.
No adoption or prior failed gate is changed. The preceding report-only turn
completed requested documentation and is progress; the all-eight M5 CPU 10x
objective remains unmet. This screen is not a proof-latency qualification.

Use j=(1+c*i)/2 with c^2=-5 modulo BN254 Fq and i^2=-1. Then j^2=j+1.
Encoding x+y*i gives (x-y/c, 2*y/c). Decoding (a,b) gives
(a+b/2, c*b/2). Verify the isomorphism with independent BigInt arithmetic.

The native multiplication uses S=a+b, D=c+2p-d, shared P=S*d,
X=P+a*D and Y=P+b*c. Inputs are normalized inclusive [0,2p]. Raw S,D
are at most 4p; X,Y at most 12p^2. Two unsigned CIOS reductions share
three raw products, for 250 IFMA sites instead of 300. This differs from
the already negative positive-Gauss caller: the polynomial basis changes,
removing two of its four operand preparations. This is not an unchanged
Gauss rerun or a claim that fewer IFMAs proves a runtime gain.

Square uses P=a*(a-2b), Q=(a+b)*(a+3b), v=(Q-P)/3, output (P+v,v).
Division by three works on the integer Montgomery representative without
changing its scale: choose k in {0,1,2} with z+k*p divisible by three and
divide that exact integer. Because p=1 mod3 and B=2^52=1 mod3, residue
selection can use a digit sum and an IFMA reciprocal; exact digit division
then runs low to high with five IFMA-low operations. This recipe is fixed
before timing; do not select a better square after inspecting the results.
Norm is a*(a+b)-b*b, conjugate (a+b,-b). All costs are included.

First exact models check basis roundtrip/products/norm/square, unsigned
CIOS cancellation/packing/ranges and divide-by-three digits, including
inclusive 2p, full-width random values and dependent outputs. Only a passing
model permits native implementation. Native tests compare against Arkworks
and check internal normalized bounds; run the unfiltered field suite once.

Then one common fat-LTO component harness compares ordinary and new-basis
operations, with separate exact accepted/fresh baseline controls if native
implementation proceeds. Six fixed contexts: dependent multiplication,
four independent streams, scaled multiplication, square recurrence,
norm recurrence and affine-expression recurrence including norm. CPU 2,
31 fixed rotated/reversed blocks, three 2000-iteration primes and 100000
timed iterations per process. Convert operands before timing and outputs
afterwards; scalar/Ark oracles are outside timing. This omission makes the
screen an optimistic arithmetic-only bound, not a key-format, conversion,
curve, inversion, memory-pressure or SDK performance result.

Each candidate/control upper paired ratio must be <=1.03; equal-log
six-context aggregate and norm-affine must have upper<1 and >=21/31 wins
against both controls. OFF/accepted intervals must lie within [1/1.03,1.03].
Ranks 10 through 22 of 31 define individual nominal 97.055% median bounds
(compute exact binomial coverage in analysis), not a familywise statement.
Retain all observations, including small positive estimates. No extensions,
exclusions, layout/seed fishing, unchanged reruns or changed thresholds.

All helpers, analyzers and publishers must be compiled before capture.
No owned compiler, hash/archive/helper or second CPU workload may overlap
capture. Successful work is retained if a later publication repair is needed.
Use Rust/xtask, no new Python. Commit and push a completed negative result
as well as a positive one. No AWS resources or per-key lookup tables.
