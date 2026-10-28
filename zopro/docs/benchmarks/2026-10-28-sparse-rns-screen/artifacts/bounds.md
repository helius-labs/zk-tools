# Local derivation and scope

These are the assumptions and bounds used by the independent Rust model. They are not native-code or timing evidence.

## Residue arithmetic

Let B=2^52 and m=B-c, where c is zero, one or 2^t±1 with1<=t<=25. All operands to a modular multiplication are canonical in[0,m). Candidate moduli may be composite; the chosen ten are pairwise coprime and coprime to the BN254 prime. Exact inverses are checked.

For a raw product x*y<B², write it L+BH, 0<=L,H<B. For c=0 its remainder is L. Otherwise the first pseudo-Mersenne fold gives V=L+cH<(c+1)B, so its high radix-B word is at most c. The second fold gives V2<B+c². In this family c²+2c<B, so V2<2(B-c)=2m and one conditional subtraction gives the canonical result.

The model forms constant products by signed NAF terms. Each radix-52 low/high term is accumulated in signed64 words; the largest possible NAF weight of a nonnegative value <=2^52 is27. Each partial word has absolute value below27B<2^57, within the asserted2^59 bound. The final low carry normalizes the high word. Constants are positive before this split; a negative equivalent constant is applied by negating the canonical remainder afterwards. The second fold's small product is at most c²<B, so it fits a single64bit word. No native128bit product is assumed for the counted sparse recipe.

The hybrid alternative precomputes k'=floor(kB/m), k<m. With q=floor(xk'/B), q<=floor(xk/m) and floor(xk/m)-q<=1 because x/B<1. Therefore r=xk-qm is in[0,2m). Its low64 representation, formed by two wrapping low64 products and a subtraction, equals the actual small nonnegative r. One comparison and masked subtraction makes it canonical. The high52 product xk' is one IFMA site. For m=B, only low64 multiplication and a mask are needed. Zero/one constants are special cases.

## Exact mixed-radix conversion

For residues r_j modulo m_j, the triangular recurrence updates each later word by (r_j-a_i)*m_i^-1 modulo m_j, leaving a_i as the next mixed-radix digit. The model canonicalizes r_j+2m_j-a_i with two conditional subtractions. Since a_i<m_i<2m_j, this intermediate is nonnegative and below3m_j. The invariant is the standard successive division by the selected moduli, so the five digits reconstruct the unique input in[0,M).

Horner evaluation starts with the top digit, canonicalized to the destination modulus, then repeatedly evaluates a_i+(m_i mod n_j)*y. The product remainder is canonical and a_i<m_i<2n_j, so this sum is below3n_j and two conditional subtractions suffice. The final result equals the source integer modulo n_j. All modulus/coefficient reductions use the new exact word recipes. This spelling is deliberately explicit; constant/range-specific simplifications could reduce its count. It is not a lower bound or compiler schedule.

Each direction has10 triangular constant products and20 Horner constant products. Extraction has four successive digit rounds; Horner has four successive rounds per destination. Both conversions are required. No CRT cofactor weights, approximate quotients or correction table remains.

## Montgomery range

Let normal X be in[0,4p) and one linear operation produce an input in[0,8p). Set T=XY, XY+UV, or XY-UV+64p². All three are nonnegative and below128p². Compute the exact canonical Q=(-T/p) mod M in[0,M). Converting Q to the second basis preserves that exact integer. Z=(T+pQ)/M is integral and satisfies 0<=Z<128p²/M+p<3p, because the chosen M>64p. N>8p ensures the reverse conversion reconstructs the same Z. Thus outputs return to[0,4p), including both fused modes. The64p² difference offset is field zero. The global Montgomery factor is M, not the current binary radix; interoperability conversion is unimplemented and excluded from the ledger.

## Finite search guarantee

There are50 distinct offsets in the declared family. Every unordered coprime five-set is enumerated. A32-state subset DP finds the minimum declared paper-style proxy over all120 orders of each set. Its cost consists of inverse NAF weights, target reduction terms, source Horner terms for the first four positions and destination terms for the reverse conversion. These terms are additive across the pair of bases; there is no cross-pair term under this proxy. The pair search sorts bases by this cost, checks cross-coprimality, and stops only when remaining score sums cannot beat the best pair. Its minimum is510, over714520 five-sets and3992870 evaluated candidate pairs.

The proxy uses the lesser NAF weight of each canonical inverse and inverse-minus-modulus. It does not include all signs, shifts, word splitting or canonicalization required by the actual word recipe. The chosen pair minimizes this finite proxy, not the hybrid logical-operation count, native instructions or cycles. No claim of a best possible RNS implementation is supported.

## Work and storage

For each field, the hybrid product body has85 IFMA sites,140 low64 products and720 other logical arithmetic sites,945 total. The two conversions alone account for332+334=666. The all-sparse product body has3797 sites for Fq and3691 for Fr. Its low IFMA count is exchanged for shifts/additions and canonicalization work. All counts are parameter-dependent and checked identical across all input cases; they are not dynamic hardware instruction counts. Memory operations, register allocation/spills, input/output binary conversion and curve integration remain excluded. Fused-mode bodies are recorded separately.

The raw10 moduli,20 triangular inverses,40 Horner factors,15 field-specific factors and10 difference offsets occupy95 u64 words (760bytes). Precomputed Shoup factors and NAF descriptors require additional storage; no native layout or total footprint has been qualified. A residue state still has ten u64 words,80bytes compared with32bytes stored in the current binary field representation; eight-value working planes require640bytes versus320. These sizes do not establish proof RSS or minimum RAM.
