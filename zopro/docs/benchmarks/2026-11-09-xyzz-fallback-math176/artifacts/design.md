# XYZZ for sparse fallback accumulators: arithmetic qualification

The current accepted Window keeps only exceptional/collision contributions in
sparse Jacobian accumulators. This new hypothesis changes that sparse point
representation to X,Y,ZZ,ZZZ with x=X/ZZ, y=Y/ZZZ and ZZ^3=ZZZ^2. It does not
replace the ordinary shared-inversion affine stream. The earlier triple-affine
review mentioned XYZZ versus affine, but did not implement the sparse fallback
replacement. No previously rejected full-projective stream, initial-unit tag,
global unit-Z method, queue policy or large-class split is reopened.

Use the EFD Sutherland2008 mixed-add/double/normalization formulas, independently
written in Rust against the exact original field1200. Source:
https://www.hyperelliptic.org/EFD/g1p/auto-shortw-xyzz.html
Mixed addition is8M+2S versus current Jacobian7M+4S. Generic scalar Fq square
currently dispatches the same multiply routine as multiplication, making this
a plausible small work saving. Fq2 has different relative costs. Doubling specialized to a=0 is
6M+3S versus the existing2M+5S and can be worse; retain this tradeoff explicitly. The general EFD doubling
count includes an extra square for the a-dependent term, absent here.
Identity/equal/opposite/doubling cases must be complete. No special unit-Z
branch, precomputation, pool or assembly is added.

Normalize using inverse(ZZZ), B=(ZZ/ZZZ)^2, x=X*B, y=Y/ZZZ. Batch inversion of
nonzero ZZZ preserves the existing one-inverse organization and does not need
a second inverse. Every point now holds four base fields instead of three:
G1128vs96bytes,G2256vs192bytes. Any future Window integration must charge all
additional sparse capacities/normalization buffers in the shared checked
forecast or admit optional scratch before allocation, retaining the old memory
fallback. This stage has no Window/prover integration or memory-peak claim.

Independent Arkworks expected sums/raw-coordinate equations must cover both
curves, full-width generator coefficients, variable projective scaling,
identities/repeats/opposites/cancellation/doubling, changing chains, final
ordinary-affine merge and batch normalization. No standalone timing follows
from arithmetic counts or correctness. Reuse original native and generic field
libraries; compile and run focused new tests only, without inherited suites.
Prospectively freeze native/portable plans and input/source pins before running.

Accepted production176/SDK and canonical43730 ledger must remain unchanged.
No new proof/export/latency/PMU/profile is planned at this stage. Full goal stays
ACTIVE: verified single-proof CPU AVX>=10x suppliedM5CPU onALL8 and CPU/Metal
comparisons. This small fallback opportunity is not a standalone route to2x.
After verification, save a polite factual report, commit AND push, and continue.
