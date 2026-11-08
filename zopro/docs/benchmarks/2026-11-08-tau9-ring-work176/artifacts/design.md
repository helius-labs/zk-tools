# Fixed tau^9 quotient-ring MSM work screen

Frozen before any new model execution. Goal remains verified CPUAVX single
proof >=10x supplied M5 CPU for all8 shapes, reporting CPU and Metal. No source
adoption, native MSM, performance timing or proof belongs to this stage.

The preceding local-tile recipe is closed negative. Its faster schoolbook
control includes the known mixed-radix packing identity and old paired-row
idea. Packing passed older small SDK timing but failed a later4999 declared
criterion; the row-pair/fused SDK trials also failed. Do not rename/retry them.

Primary research: https://eprint.iacr.org/2024/1906 describes tau=1-omega for
prime-field single-scalar multiplication. The current abstract was read;
the PDF fetch returned403. No paper runtime or projective operation cost is
transferred to this MSM or G2/Fq2 implementation. The local adaptation below
is independently derived, with no novelty claim. SRE2610.27452 and memory-
constrained2026/1316 were rechecked but have prior local screens and are not
reimplemented. Binary-ring G1/G2 native failures remain closed.

Use Z[omega], omega^2+omega+1=0, tau=1-omega, and FIXED rho=tau^9=81+162omega,
Norm(rho)=19683. This differs from the previous binary radix128 ring ideal.
There is one geometry, no width/table/layout/host sweep or favorable subset.
Residue classes are (a mod81,b mod81,(floor(a/81)+floor(b/81)) mod3).
Find bounded representatives by exact nearest-quotient enumeration, quotient
coordinates(-a+2b)/243,(-2a+b)/243, testing nine neighboring rounded integers.
Combine six units into three signed point planes. Include nontrivial unit
stabilizers: one nonzero orbit has two classes, remaining orbits six.

The finite table must cover all19683 residue classes exactly. Every chosen
digit has norm<=81^2. Avoid wide signed overflow by dividing coordinates by81
before coefficient combinations. Recode the existing signed Phi2 pair, whose
coordinates are strictly below2^126, in eighteen rho-digits. Independent BigInt
reconstruction and Fr eigenvalue reconstruction must agree. Check all classes,
signed edge pairs and fullwidth deterministic signed/canonical random cases.
Prove termination using |rho|>140 and digit radius<=81:
139*2^127 <58*140^18 implies |q18|<1 and therefore q18=0.

Apply the new recoder to the two existing immutable B fixtures, keeping the
old scalar/identity filtering and two point parts. Reuse the exact original
split helper and inherited baseline/binary-ring dense counts without rerunning
their old models. For each of18positions, model ordinary fill, part/unit-plane
merges, both coordinate grouping axes, dense consecutive weighted sums and
one axis-result combination. Check every window against two independent
integer-module dot products. Keep unit maps and rho-Horner operations separate.
Raw2x2 is diagnostic because production admits whole-scalar grouping there.

Necessary work gate requires the primary total strictly below BOTH inherited
accepted dense634405 and binary-ring dense573432, under their SAME cost
definition. No percentage floor. A count pass only permits a later exact
queue/tree/tail/resource census; it cannot select native timing or adoption.
If it fails, close this fixed geometry without a native implementation or
unchanged reroll. The prior binary native60% regression is not erased.

Candidate counts omit recoding, scheduling, inversions, native field costs,
exceptions and normalization. All these exclusions remain explicit. Fixed
curve-only digit metadata is not a proving-key table; no shifted point table,
key copy, retained solver plan or production pool is created. The model's
host allocations are not a native/prover memory estimate. Future native
admission must charge every plane, suspended parent, axes, digits and scratch;
larger bucket arrays and costlier Horner work may erase any count saving.

Run a new Rust/xtask compile and new model exactly once, preserve failures.
Then separately verify hashes/receipts/algebra/counts and write/push a factual
positive or negative report. No old tests/models/profiles/proofs are replayed.
