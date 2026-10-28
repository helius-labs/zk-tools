# Structured RNS bounds and accounting

This is a local mathematical construction. It is not an implementation or performance claim from the cited papers.

Let B=2^52 and m_i=B-c_i with c_i=3+2520i, i=0..9. All m_i are odd. Every pairwise difference is2520 times an integer≤9. All prime factors of1..9 divide2520, and gcd(B-3,2520)=1. Hence every m_i is coprime to every difference and all ten are pairwise coprime. The exact model also checks all45 gcds. Moduli need not be prime. The first/last five form products M/N; both products are large enough for the exact inequalities below for Fq and Fr.

For one field value X, retain residues in both bases. Normal0≤X<4p, and one addition or subtraction with a public4p offset can form an input0≤X<8p. For multiplication and a two-product sum, T is nonnegative and below128p². For a two-product difference, add the field-zero64p²; its integer numerator lies in the same interval. This is a range argument, not an implemented native addition API.

In the left basis let M_i=M/m_i and alpha_i=(-T*p^-1*M_i^-1) modm_i. Set S=sum alpha_i*M_i. The exact CRT quotient is k=floor(S/M), and S/M=sum alpha_i/m_i. Since

    0 <= sum alpha_i/m_i - sum alpha_i/B < sum c_i/B < 1,

k_low=floor(sum alpha_i/B) underestimates k by at most one. Thus Q=S-k_low*M is nonnegative and below2M. T+pQ is exactly divisible by M and

    0 <= Z=(T+pQ)/M < 128p²/M+2p < 4p

because M>64p. The result equals the exact canonical Montgomery result or that result plus p. This closes ordinary multiplication, square, sum2 and offset difference2 after the permitted single linear operation. More consecutive unreduced linear operations need a separate bound; they are not qualified here.

Reverse conversion uses beta_j=Z_j*(N/n_j)^-1 modn_j. For S'=sum beta_j*(N/n_j), S'/N=k+Z/N. With E_j=ceil(2^64/n_j), the upward estimate has error strictly below sum n_j/2^64. The exact checked inequality

    4p/N + sum n_j/2^64 < 1

therefore prevents crossing the next integer. Its floor is the exact k, in0..4. For these52bit near-power-of-two moduli every E_j equals4097. If s=sum beta_j<5B, the high64 bits of4097s equal

    (s>>52) + carry64(((s&(B-1))<<12)+s).

This needs only a sum, shifts and carry detection. It applies to this explicitly preweighted CRT input. In the inspected author's normalized CRT, the coefficients are different: actual canonical BN254 inputs produce54bit correction quotients. Dropping that algorithm's upper quotient products would be unjustified.

For structured conversion, evaluate P(t)=sum alpha_i*product_{j!=i}(t-j) on the other five indices. Because B=c_t modulo m_t, M_i modm_t=2520^4*product_{j!=i}(t-j). Thus a degree-four finite-difference evaluation replaces the dense unnormalized CRT sum. Its initial polynomial values are alpha_i times[24,-6,4,-6,24]. Ten difference-table subtractions and20 extrapolation additions produce the five outputs. Reverse conversion uses backward differences on indices5..9.

Split each52bit alpha/beta coefficient into two26bit halves. For one half, initial magnitudes are below24*2^26. The difference table is bounded by384*2^26. A deliberately loose bound of four sequential doublings at each of five extrapolation steps gives384*2^20*2^26<2^63, covering every signed intermediate. Target output coefficients are nonnegative and their absolute coefficient sum is at most11274; recombination can still require66bits and must not silently truncate to u64. The model uses exact u128 recombination. A native low/high fold is not implemented.

The dense-flow site ledger follows the inspected source at five channels per base:20 elementwise IFMAs,95 first-matrix IFMAs and85 second-matrix IFMAs per eight independent integers, total200. The source's residue-lane layout uses40 IFMA instructions for one integer. Neither is a timing or a comparison of optimized generated code. The current eight-instance CIOS source template uses100. Quotient estimation, rearrangement and conversion work are additional to these counts.

For the new structured recipe, a partial ledger includes10 variable modular products,25 fixed modular products after fusing output factors, and four26bit half-polynomial evaluations. Choosing a pseudo-Mersenne variable-product recipe and a Shoup fixed-product recipe gives65 IFMA,70 u64 products,10 u32 products and255 other vector operations:400 sites before recombination/folding, quotient/correction handling, load/store and input/output conversion. Strength reduction and a different spelling can change these numbers. This ledger is not a lower bound on RNS, compiler output or a cycle model. Saving IFMAs while adding this work does not establish a latency advantage.

A proposed optimized constant set has130 u64 entries (1040bytes), including400bytes for the universal five-choice correction constants. Those are modulus-only constants, not point/key tables. An unpadded field state has10 u64 residues (80bytes), versus32bytes for the current stored field, and eight-instance working residue planes occupy640 vector bytes versus320. These are representation sizes; native spills, point-workspace ownership, import/export cost, proof RSS and pressure admission remain unimplemented and unmeasured.
