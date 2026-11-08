# Exact PMNS truncation width screen

Read primary https://arxiv.org/html/2610.09767v1 sections4 and5. Prior reports
already mention this paper, including query-control-codegen176; do not call
it a newly discovered technique. Earlier six-coefficient PMNS trial is closed.
New question: does this particular word-truncation recipe reduce the product
volume for BN254 ordinary and fused arithmetic at any coefficient geometry?

Use eight independent fields per ZMM as current point arithmetic. The paper's
n*ceil(n/8) expression instead packs polynomial coefficients into lanes and
does not compute eight independent field products in that count. Do not
compare those unmatched output populations. Count all n^2 coefficient pairs.

Each coefficient has s words of52bits. Enumerate literal low/high product
terms: raw AB 2s^2 (double for fused AB+CD), low quotient s^2, fullQM2s^2,
truncatedQM includes weights>=s-1. Saved terms=(s-1)^2 per coefficient pair.
Ignore additions/carries/permutations/conversion/final reductions, making
this an optimistic source-product screen, not cycle count. No parameter,
closure, signed-arithmetic or native PMNS qualification is implied.

Each prime has254bits, so n*s>=5 is necessary for full52bit coefficient-word
capacity. Check both actual prime constants. Enumerate n,s1..16 and both
ordinary/fused paths. Prove extension globally: t=ns>=6 gives truncated
ordinary>=4t^2>=144 and fused>=6t^2>=216. At t=5 only(1,5),(5,1) occur; exact
minima109/159 occur at degenerate n1. This is not a field feasibility proof.

Check truncated-word carry reconstruction with exact BigUint: full sums of
one/three/six products, coefficient words1..5, bases2^12/2^52,160 boundary/random
cases each. Keep terms at wordweight>=s-1. When omittedD<phi and C+full is
divisible byphi, ceil((C+kept)/phi) equals (C+full)/phi. Preserve explicit
insufficient-base counterexample where omittedD>=phi; do not overgeneralize
the shortcut. These are accumulated-word identities, not a PMNS parameter set.

One compile/execute Rust1.98.1 xtask workflow. Inherit exact accepted176 pins,
field and helper dependencies without replaying successful field/core/proof
tests. Separate reader checks source/receipt hashes and recomputes schedule
counts by independent closed forms. No native run selected unless a complete
work/resource case overturns the original CIOS cost. Existing Karatsuba/pairwise
PMNS, sparse-special-M and other representations are outside this screen;
their existing negative experiments are not rerun or declared impossible.

Goal remains verified singleproof CPUAVX>=10xM5CPU all8 withCPU/Metal comparison,
currently0/8. No new point/key LUT/copy, retainedplan, pool or arbitraryRAMcap.
Canonical37023proof ledger unchanged. Archive factual evidence, commitANDpush.
