# Semantic expression and deterministic-hint census

The previous goal turn made progress: the G1 denominator tree was qualified,
measured, rejected and pushed as e0c541135d. Its source-count/dependency savings
did not improve the target threaded caller. That exact experiment remains
closed, along with earlier kernel/layout/heap/queue variants.

This is a distinct read-only circuit analysis, not a new prover runtime. The
previous affine-closure176 reader made each nonlinear assignment and unknown
hint output a fresh opaque atom. It therefore did not test whether two such
operations denote the same function. Reuse its exact image-v2 parser and affine
linear forms, but intern semantic nonlinear atoms instead of fresh wire atoms.

Proposed rules, all over Fr and independent of any fixture witness values:

- Multiplication is commutative. Normalize nonconstant affine operands by their
  first nonzero atom coefficient, factor the units outside, and intern the
  ordered pair of monic forms. Existing constant/linear propagation remains.
- InvZero hints are deterministic, inv0(c*x)=inv0(x)/c for nonzero c, including
  x=0; inv0(inv0(x))=x. Preserve zero semantics rather than canceling x/x to one.
- bits.nBits outputs are bits of the canonical field representative. Equal
  normalized input functions have the same bit at index i regardless of the
  number of requested outputs. Bits at indices >=254 are zero.
- Only actual Bits-hint atoms are certified Boolean. For a certified b, use
  b*b=b, b*(b-1)=0 and (b-1)^2=-(b-1), including nonzero scalar factors.
  InvZero/scalar Bits of these two-valued forms can also be evaluated exactly.
- Unknown-divisor assignments keep a typed guarded function: output0 when
  the other side is0 and the required output is0; otherwise quotient minus
  the known assigned-side form. Simultaneous unit scaling of divisor and
  numerator is safe. Do not simplify unknown proportional division to1.

No deductions from constraint-only rows, general polynomial ideal solving,
unknown Boolean assumptions, witness-specific facts, expression-size budget,
hash-only equality or truncation are permitted. The abstraction remains
incomplete. Constraints/validation must remain if a future runtime is built;
aliases describe successful solver results, not permission to accept an
unsatisfied assignment. The current work does not change solver behavior.

Count exact and proportional wire/query equivalence classes, repeated node
kinds and rule hits on the two already retained primary/2x2 solver programs.
Only wire0 is constant1; all other inputs remain arbitrary. Reuse original
private solver images with exact hashes, no new export/solve/proof/timing.

Any proportional query classes could permit replacing corresponding key terms
once, rather than paying repeated point grouping in every proof. This is only
an opportunity census: no key transformation, retained symbolic DAG/table,
second solver plan or runtime cache is proposed. All analysis structures are
disposable offline objects. A positive result requires independent semantic
checks and actual bounded-memory proof implementation/qualification before a
speed or RAM claim. A zero result closes only these specified rules and inputs.

Before reading real census outputs, test exact product/unit/commutativity,
deterministic hint/involution/bit-prefix rules, Boolean identities, unknown
and zero divisor semantics, and synthetic complete images with known aliases.
Retain the original parser/linear tests. Reuse exact accepted native field
libraries; no fresh field/core/prover suite is claimed for unchanged code.
Use Rust and structured xtask receipts/pins. Publish a factual report with
all eight inherited CPU/Metal references and commit/push either outcome.

The full objective remains verified single-proof CPU AVX latency >=10x the
supplied M5 CPU times on all eight shapes, also reporting Metal comparisons.
Production176/SDK6d83, accepted primary7.928660ms, ledger28847 and all reference
tables remain unchanged. No performance improvement is established by this plan.
