# Finite-field split-radix screen, before execution

Original goal remains verified CPU AVX single-proof latency >=10x supplied M5
CPU on all eight shapes, with CPU/Metal comparisons. This algebra/work screen
does not replace that goal. Production176/SDK6d83 and canonical35171 unchanged.

Study ordinary split-radix over BN254 Fr. Divide an N-point transform into even
N/2, odd-one N/4 and odd-three N/4 transforms; recombine with w^k,w^(3k),iota.
Here iota=w^(N/4) is an ordinary dense Fr element, NOT a free complex sign/swap.
Unit factors are skipped consistently in both scalar models; +/-iota products
are charged as full field multiplies. Do not transfer complex flop counts.

Primary reading: Johnson/Frigo 2007, https://fftw.org/newsplit.pdf, sections I-II
and Algorithm1 (conjugate-pair variant, not copied). FLINT's generic-ring DFT
documentation describes ordinary split-radix; its current documentation warns
parts were LLM-generated. Treat it as a lead, not a trusted operation-count
oracle. Independently derive recurrences and execute the field identities.
No published speed figure or source implementation is imported.

Accepted source bindings: fft.rs Plan.euler_product_values calls
pair::euler_product_study under native guards. That function packs A/B into
lower/upper lanes, inverse transforms, weights coefficient k, forward transforms,
then returns N*(E(A)B+A E(B)) in natural order. Four scalar transforms are packed
into two paired passes. fft/pair/leaf.rs retains bounded in-place radix2 kernels;
stored columns fuse five layers and terminal kernels shuffle within lanes.
The innermost unit-twiddle layer is already addition-only. Existing prefix
pruning, CRT-Euler, fixed-root/column-preparation and fusion studies stay closed.

Write a scalar natural-order radix2 reference and ordinary split-radix, using
one borrowed root table. Root preparation, exponentiation and model allocation
are outside arithmetic counters. Count data add/sub, unit skips, nonunit root
multiplies and fourth-root multiplies. The latter are a SUBSET of nonunit
products, not an extra count. Count k-weight and final combination separately.
These models do NOT execute or estimate literal compiled SIMD instructions.

Prospective recurrences, bases M1=M2=0,A1=0,A2=2:
radix2 M(N)=2M(N/2)+N/2-1; A(N)=2A(N/2)+N.
split M(N)=M(N/2)+2M(N/4)+3N/4-2;
split A(N)=A(N/2)+2A(N/4)+3N/2.
If derived correctly both give M=N log2(N)/2-N+1 and A=N log2(N).
Check recurrence results through N=2^28 with exact integer arithmetic. This is
a claim about these fixed scalar circuits, NOT a universal lower bound or
a proof that all transform organizations perform identically.

Validation: domains2..256 (powers of2), roots with exponents1,3,-1, five input
families (zero,unit,alternating signs,fullwidth fixed random,last basis):120
transforms per scheme. Compare each output to independent Arkworks direct DFT,
not merely a roundtrip. Check counts against closed forms and separate recursive
enumeration. Then50 complete Euler operators at N8/16/32/64/128, roots1/3,
five length pairs including empty/partial/full, against independent Arkworks
coefficient differentiation. Finally two N32768 full-domain sparse-polynomial
Euler cases checked against independent analytic Arkworks derivatives. Do not
call them proof fixtures or native full-FFT tests.

Reuse all nine saved geometries from prefix-pruning176 without new key loading,
witness solving or geometry capture. Report full/prefix count interpretation:
this recipe computes full transforms and consumes the original prefix; it does
not implement a new pruned split graph. Charge all four transforms plus unchanged
coefficient weights and output combination for each geometry. No inference from
discarded output fraction to skipped products.

Necessary arithmetic selection: only a strict reduction in complete-operator
nonunit products with no extra add/sub selects further native investigation.
No positive-percentage floor. If counts equal, close this work-removal recipe
without a native layout/worker/host/compiler sweep; no measured slowdown or
universal impossibility claim. Do not market model Vec allocations/root table
as proposed mobile/prover storage or memory peaks. Any positive model would
still require a separate packed layout/permutation/resource/caller design and
actual verified SDK tests before adoption. No new pool, expanded key, retained
solver plan, multigigabyte lookup table, C/ASM rewrite or production changes.

One optimized checked build and one model execution via serialized xtask. An
offline reader reconciles counts, source/receipt hashes, geometry identities,
formula identities and selection without repeating model/oracles. No owned CPU
work overlaps any performance recording; this study has none. Never repeat
successful tests, captures, builds or verifiers for bookkeeping. Preserve any
failed stage, repair only failed/unstarted work. Commit AND push finished report.
