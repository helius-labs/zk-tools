# Late-centered Montgomery arithmetic: prospective mathematical screen

Declared before builds/model execution. Previous goal turn PROGRESS: Winograd16
complete-caller rejection/report committed and pushed as9617fac5f7. Goal remains
verified CPU AVX single-proof >=10x supplied M5 CPU for ALL8, plus CPU/Metal
comparisons. Production176/SDK6d83/canonical35171 unchanged. No clocks selected.

Hypothesis: remove modular corrections from the linear stages of ordinary G1/G2
affine streaming by keeping signed numerical Montgomery representatives throughout
one complete prefix/reverse update. Host key and bucket coordinates stay canonical
R256. This is not a signed scalar recoding, the old pairwise-product factorization,
the local signed-complex square, or another R260 coordinate-isomorphism recipe.
No novelty claim. Positive-inverse/subtractive REDC is background, not imported
code or performance. The fixed implementation below uses additive cancellation
with a negative inverse, centering ONLY the final quotient digit.

For R=2^256 or2^260 use four52-bit steps then48/52. Let
q=(-T*p^-1) modR, q in[0,R). Replace q by q-R iff q>=R/2. The result
v=(T+q*p)/R satisfies |v| <= |T|/R+p/2. The first four ordinary unsigned
quotient digits remain unchanged; subtract p at the end iff the highest
quotient digit has its sign bit set. There is no centering at each row.
For either prime p/R<1/5, so |T|<=2p^2 implies |v|<p. No final canonical
correction is needed inside this bounded signed domain. Negative products and
fused differences use actual signed T, not a positive2p complement.

Storage for the prospective vector type: five radix52 digits, low four normalized
unsigned, signed top digit. Raw linear sums/differences may be unnormalized until
the next multiplication or export, but the model normalizes before every product.
No signed value may be passed to the existing Ifma8/get/compact/Fp contracts.
Stored host points retain their original32-byte field coordinates.

Native-shaped raw products MUST account for unsigned IFMA. For signed digits
a=u-s_a*B,b=v-s_b*B, B=2^52, the exact split is
lo=(u*v) modB, hi=floor(u*v/B)-s_a*v-s_b*u+s_a*s_b*B.
Only the top digits can be negative after normalization. Model every coefficient
and cancellation accumulator in signed64 range, both ordinary and fused raw sums/
differences, exact low cancellation, full-width quotient/reference and canonical
field meaning. A literal unfused raw product still needs50IFMAs plus signed-top
mask/subtract corrections; reduction needs50IFMAs including quotient generation.
Fused uses100raw+50reduction IFMAs. These counts do NOT establish speed.
The extra top-sign corrections, final quotient mask/subtract, signed carries,
normalization, register pressure and export must not be omitted from a later trial.

Fixed mathematical population: Fq/Fr xR256/R260;13signed edge representatives
[-p,-p+1,-p/2,-1,0,1,p/2,p-1,p] plus +/-2p,+/-5p (13total), their Cartesian
products,2048deterministic full-width signed tuples and256changing tuples;
ordinary/sum2/difference2 each. Edges wider than p test broader graph domains,
not the [-p,p] closure claim. Independent Arkworks modular arithmetic checks each
output; an extended-Euclid full-radix quotient independently checks each reduction.
One exhaustive toy p13,R256,two4-bit steps checks all729 signed input pairs and
three operation types. No population extension or oracle replay after results.

Complete scalar models of eight interleaved chains follow the current G1 prefix
and G2 norm-prefix algorithms, signed arrivals and reverse recovery. Fixed lengths
8/16/200, sign modes positive/negative/mixed and three changing passes, at both
radices. Expected full curve outputs come from independent Arkworks group addition,
generated once and reused across these new algorithm/radix cases. Inversion uses
the ordinary canonical boundary and independently checked Fermat inversion; this
does NOT qualify the actual eight-lane inversion bridge, tails, queues, exceptions
or native execution. Full-vector ordinary/nonzero-denominator scope only.

Conservative complete-circuit bounds, using p/R<1/5: chains/factors/norms and
slopes <p in absolute value; signed incoming Y difference <2p. G1 numerator<p,
square<p, X<3p, finishing delta<4p, final product<2p,Y<3p. G2 conjugate numerator
<2p, scaled slope<p, real/imaginary square<2p, X<4p, finishing delta<5p,
final complex product<3p,Y<4p. Canonical export is required after each update,
and its wider two-sided correction is a material cost, not a free boundary.

Continuation rule: require all exact identities, representative/range/coefficient
bounds, independent curve outputs and source/provenance checks. Passing only
selects explicit native tradeoff review/implementation design, NOT performance,
SDK integration/adoption or goal completion. If extra signed/normalization/export
work makes the complete proposal unconvincing, decline native work separately
without calling correct math a speedup or pretending a measured regression.

No production edits/key table/key copy/retained solver plan/new pool/CASM rewrite.
Model BigInt/Arkworks allocations are diagnostic storage, not proposed RAM.
Freeze source/compiler/dependency/176production pins and xtask build/run receipts.
Never rerun successes for bookkeeping; preserve failures and repair only failed/
unstarted work. No owned CPU work during any future recording. Publish/commit AND
push a finished stage politely. Keep full all8 CPU10x goal active unless verified.
