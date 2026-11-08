# Fixed binary-power Karabina screen, before execution

Objective remains verified CPU AVX single-proof >=10x supplied M5 CPU on all
eight shapes, reporting CPU and Metal. This is one incremental verifier work
screen, not a replacement objective or a forecast of reaching the primary
3.7 ms target. Production176/SDK6d83 and canonical35171 remain unchanged.

History checked: signed-window-four parameter exponent, actual-library
diagnostic, static final-exponent selection, three-lane Miller/final exponent
and packed-Miller SDK all retain recorded outcomes. No identical exponent
schedule/compiler/layout/worker retry is selected. No Karabina/torus compressed
square implementation/report was found in the benchmark README history.

Primary references: Karabina ePrint2010/542 (abstract available; PDF403), and
installed gnark-crypto v0.22.0 internal/fptower/e12.go compressed-square and
decompression equations. We derive the compressed square by retaining four
coordinates of the exact accepted Granger-Scott square. We implement the
mathematical equations, without translating the upstream Go function body.
No upstream timing or novelty claim is imported. The decompression guard uses
the actual denominator coordinate g3=c1.c0. Source comments/guards in an
external implementation are not an oracle for that coordinate mapping.

Fixed recipe: for X=4965661367192848881, build successive powers f^(2^j),
j=1..62, keeping only g1=c0.c1,g2=c0.c2,g3=c1.c0,g5=c1.c2. Save the selected
nonzero-bit powers in a fixed [Compressed;64], without a key table or heap.
Decompress all selected powers using one scalar Fq inverse, batching norms of
Fq2 denominators. Start the final product at f (bit0=1). No signed recoding,
dictionary/addchain sweep, torus multiplier, quotient or layout rescue.

For g3!=0, g4=(xi*g5^2+3*g1^2-2*g2)/(4*g3). For g3=0,g2!=0,
g4=2*g1*g5/g2. For g3=g2=0, require all four stored coordinates zero and
recover one. Then g0=1+xi*(2*g4^2+g3*g5-3*g2*g1). Only cyclotomic-subgroup
inputs are supported; identity is covered. Rare finite nonidentity g3=0 cases
are counted if encountered, not claimed covered merely by a branch existing.

Compile once with optimized overflow checks, then execute once via xtask.
Use 256 fixed full-width generated nonzero Fq12 inputs plus identity. Apply the
easy final-exponent map, check that map against independent Arkworks, then
check each of 62 consecutive compressed squares and complete parameter
exponent against a source-bound original cyclotomic implementation and
independent Arkworks general powering. No proof timings or hardware recording.
Three separate focused reconstruction checks exercise the explicit identity,
zero invalid compressed tuple rejection and normal recovery. Input zero must
remain outside the easy map. Do not claim this qualifies new native SIMD or
arbitrary compressed tuples.

Count original binary exponent operations and candidate construction,
normal/degenerated reconstruction, norm batch inverse, and final products.
Report Fq2 square/multiply and Fq multiply/fused2/inverse entries separately.
The source lowering is Fq2S=2FqM, Fq2M=2FqFused2, Fq12M=18Fq2M; check source
bindings. Addition/negation/nonresidue work is additional, not free; record
nonresidue calls separately. Counts are source semantics, not instructions,
cycles, equal prices or removable wall fractions. The model's per-power
debug decompressions/oracles must not be counted as candidate runtime work.

Selection: mathematical mismatch blocks all native work. If candidate weakly
increases M,Fused2,inverse and nonresidue entries with one strict increase,
close as count-dominated. Otherwise perform an explicit complete-tradeoff
review before selecting any native implementation. No time weighting or
positive-gain floor is predeclared. An operation tradeoff or math pass alone
does not select a full-SDK trial. Any native selection would first require
actual whole-final-exponent callers and proofs, with fixed controls, resource
and portable qualification before adoption.

Fixed requested data payload: 64 four-coordinate compressed powers (16KiB),
three64-entry Fq arrays for norm/prefix/inverse (6KiB), and fixed scalar/local
Fq12 temporaries. The implementation may need fewer norm arrays; report actual
source sizes. Stack compiler frames and model/oracle analysis allocations are
additional. No RSS/minimum-RAM/device qualification is implied. No production
pool, input-sized cache, key copy or retained execution plan is added.

No owned build/helper/hash/report/push overlaps any performance recording.
This model has no performance recording. Never repeat successful builds,
model executions, references or tests for bookkeeping. Preserve failures and
repair only unstarted/failed steps. Finished work receives factual report,
verified evidence, commit AND push. Original all-eight goal remains active.
