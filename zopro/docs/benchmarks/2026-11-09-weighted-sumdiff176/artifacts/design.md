# Shared sum/difference weighted fold

Question: can sharing one denominator for P+Q and P-Q make a different weighted
bucket tree cheaper, without a point table, key expansion or new production pool?
This is a local algebra/work screen, not a native implementation or timing run.
The global unit-Z, initial-unit tags, ordinary symmetric square, joint coordinate
endpoint and previously measured prefix-tree recipes remain closed.

For n=2h, split points into A_i and B_i, i=1..h. Let S_i=A_i+B_i,
D_i=A_i-B_i, and let T(X)=sum X_i, W(X)=sum i*X_i. For h even:

    W(A,B) = W(S) + (h/2)*(T(S)-T(D))
    T(A,B) = T(S)

The base cases n=1,2 retain the accepted weighted-tree reconstruction. For n>=4,
the pair outputs share the inverse of dx=Qx-Px. Ordinary pair work with supplied
inverse is four multiplications and two squares; complete identity/equal/opposite
guards handle exceptional inputs. D totals reduce in place; S recurses. A model
uses one n-point slice, so the topology alone fits within the accepted 2n-1 point
envelope. This is not a production scratch, stack, allocation forecast or RSS
qualification; returned totals and recursive field temporaries still need storage.

The accepted source-bound control reproduces weighted_sum's side copies,
simultaneous reduction levels and final projective double/subtract ladder.
Independent expected W and T use known full-width generator coefficients rather
than either tree's recurrence. Both curves and complete exceptional populations
are checked. The ordinary shared-denominator pair is independently checked against
Arkworks complete group additions. Arkworks is used for this scalar model; this
does not execute the production SIMD kernel or establish native performance.

For dense ordinary G1 inputs, charge the existing amortized batch-inversion work:
one ordinary affine addition is5M+S; one shared pair is7M+2S, allowing the optimistic
same3M prefix/reverse overhead for one denominator. Count final projective adds
and doubles separately; do not assign them zero real cost or claim the proxy is
a machine lower bound. Shared-pair packets might incur additional packing, tails
and inversion bridges, omitted here in the candidate's favor. G2 arithmetic uses
a norm-based Fq prefix, so the G1 proxy is not relabeled as a G2 native cost.

Prospective necessary work gate: for n=32,64,128,256 (the accepted group size is
256), candidate dense G1 variable M+S proxy must be strictly below the accepted
topology and neither final projective-add nor double count may increase. Failure
closes this exact topology/formula without a native trial, timing, threshold,
curve/shape/worker selection or a revision disguised as the same experiment.
Passing would only justify native qualification, not any proof speed claim.

The current canonical proof ledger remains43073. Accepted SDK/production remain
unchanged, latest matched primary7.803403940ms. Full ALL8 CPU10x goal remains
active/unmet0/8, with supplied CPU and Metal references retained in the report.
