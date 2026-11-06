# Recurrence and coefficient bounds

Let B=2^52, with normalized five-digit inputs in [0,2p]. The first four rows maintain the original nonnegative raw coefficient state as T_j=P_j+Q_j. Product halves update only P; cancellation contributions update only Q. Before cancellation the low integer coefficient is z=P_0+Q_0. For m=(-z/p_0) modB, z+m*p_0 is divisible by B and

    (z+m*p_0)/B = ceil(z/B) + floor(m*p_0/B).

The identity also holds when z modB=0: then m=0. Put that carry in Q_1, add m*p_j's low/high halves to Q_j/Q_(j+1) for j=1..4, then discard P_0,Q_0 and shift both arrays. Coefficient-wise addition and the carry identity recover exactly the accepted row's unnormalized coefficients. Induction starts from both zero arrays. Merge P+Q before the fifth row; the original final52/mixed48 packing, normalization and correction can be reused.

Each split product contributes less than B in each half. A fused product row therefore adds at most4(B-1) to an arbitrary coefficient; reduction adds at most2(B-1), plus a cancelled-low carry to coefficient1. If E bounds the previous raw coefficients, a conservative next-row bound is

    E' = E + 6(B-1) + ceil((E+4(B-1))/B).

Starting with E=0 and taking four steps stays below25B. Product/reduction in the final row and a mixed48 carry, bounded by ceil(z/2^48)+B, keep the checked coefficient state below32B; the implementation conservatively checks strict64B=2^58, far below u64 overflow. Nonnegative P and Q are separately no larger than their sum. These bounds cover arbitrary normalized full52-bit digits for the first four rows, not just sampled coefficients.

For mixed48 packing, the final highest input digits are below2^47 and the modulus digit below2^46. The highest raw coefficient t5 is therefore below3*2^42 for two products plus reduction. Its left shift by4, plus bounded carry, stays below2^48. Other packed low coefficients are below2B+1024 before the four ordinary normalization transfers. The final raw representative is exactly U=(ab+cd+Qp)/R with 0<=Q<R. Ordinary U<2p because4p<R. Fused R260 also has U<2p because8p<R260; fused R256 has U<3p because4p<R256. The original conditional subtraction of2p then returns [0,2p). Difference uses cd=(2p-c)*d exactly as accepted, so the same bound holds.

The model checks80,844 reduced outputs and404,220 row states with an independent BigUint whole-state recurrence and full-R quotient inverse, plus2,796,192 reduced-radix carry/cancellation cases. These finite tests support the explicit algebra/range argument; they are not a formal proof of future native code.

The result key max_plane is a historical helper name: its checker also sees merged/product coefficients. The reported value is the maximum over those checked lanes, not a claim about a separately measured P-only maximum. max_sum records pre-cancellation P+Q coefficients in the first four rows. All such maxima are observations below the conservative bound.

Preparation repairs before the first model execution: the coefficient-bound text was corrected from the false inequality64B<2^58 to64B=2^58, and a JSON-macro indexed array required parentheses. The first compilation failed before execution; a subsequently attempted executable invocation found no binary. The failed source/stderr are retained in preexecution-json-correction. The corrected build and full mathematical execution each succeeded once. No mathematical failure, extra favorable case selection or native timing rerun occurred.
