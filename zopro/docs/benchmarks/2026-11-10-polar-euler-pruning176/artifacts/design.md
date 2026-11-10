# Polarized Euler joint-pruning screen

Fixed before execution: read cached 1x2 and 2x2 solver images, expand exact
formal linear expressions (including assigned terms) over independent original
wires, and derive ordered A, B, S=A+B, D=A-B support masks. No witness solving,
nonlinear simplification, reordered rows, or reconstructed histogram ordering.
This is new ordered metadata, not a replay of the earlier category census.

E=X d/dX is linear. Therefore
E(A)B+A E(B) = (S E(S)-D E(D))/2.
Equality at one domain point is not equality of derivatives. This identity
retains E(D), and only drops D E(D) at points where D is formally zero.

Model an unnormalized inverse radix-two DIF transform, multiplication by the
bit-reversed coefficient index, then forward DIT. Forward known-zero
propagation and backward demand are applied jointly. Baseline derivative
demands are support(B), support(A); polarized demands are support(S), support(D).
Padding is zero, cancellation inside butterflies is not assumed. Root-one
multiplications and zero/one coefficient weights are distinguished.

The fixed candidate keeps the current paired four-row/two-channel arrangement.
Count nonunit root product dependency sites per scalar channel, their union
across the pair, and groups of four adjacent butterfly sites per stage. These
are logical operation/occupancy counts, NOT literal emitted SIMD instruction
counts: production fuses stages, shuffles lanes and uses special small roots.
For native admission require a strict decrease in paired live root sites and
four-site groups on BOTH actual fixtures. Report scalar gains even if paired
work does not drop. No channel compaction, reordering, different layout, or
post-hoc fixture subset rescues this fixed screen. Preparation of S,D, masks,
normalization, scaling by 1/2, branches and extra memory are optimistically
unpriced; a passing screen would still require complete native measurements.
The screen is not a theorem that the transformation can never run faster.

Exact checks: all mask parsing must cover every row/sum/slot and consume every
byte. Small Fr domains use a separately constructed direct inverse DFT followed
by polynomial differentiation/evaluation, with explicit N scaling. Compare
both the full graph and demanded outputs of the pruned graph, then the complete
polarized expression. Include zeros, equality, opposition, sparse and dense
inputs and arbitrary output demand, across sizes 8..64.

A separate same-agent reader uses small prime 97 and a recursive demanded-node
dependency traversal, exhausts support/demand masks at N=8, and compares root
site sets to the producer graph. It also recomputes fixture decisions from the
published masks. It is an independent implementation, not an external audit.
All workflow sources, compiler, libraries, cached inputs and unchanged 176
production files are pinned before the serialized xtask workflow. No native
prover, timing capture, proof replay, AWS instance or production edit is part
of this feasibility screen. Full goal remains ALL8 verified CPU AVX >=10x M5
CPU, with CPU and Metal comparisons. Accepted SDK/canonical proofs unchanged.
