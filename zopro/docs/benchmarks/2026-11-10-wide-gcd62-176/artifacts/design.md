# 62-bit approximate GCD blocks

Replace two 31-bit approximate binary-GCD blocks with one 62-bit block using
u128 approximations: 66 leading bits and 62 low bits. Packed rows use
E=f+2^64*g modulo2^128, with L1 norm <=2^62, so both decoded coefficients fit
i64 without the old 31-bit pure-row ambiguity. Preserve positive absolute
difference/zero-run decisions; no signed small-quotient method or lookup table.

Apply the resulting matrix once to the full integers, dividing exactly by
2^62 and normalizing whole-row signs. Reuse the accepted signed-linear,
62-bit modular coefficient update and exact one-word Euclid finish byte for
byte. The coefficient invariant starts with R2 and zero as before. No pending
matrix/composition is needed. This saves the intermediate full-integer update
and extraction; it adds wider comparisons/subtractions, encoded-row arithmetic
and possibly longer zero runs. Counts alone do not establish a speedup.

This derives a wider instantiation of the production Pornin-style algorithm
(https://eprint.iacr.org/2020/972). No novelty or external speed claim, and no
external implementation copied. Narrow history checks found prior scalar
packed31, signed-jump30, signed62 safegcd, active-word and Lehmer studies;
none is this positive approximate62 recurrence. SRE/Hrabar/memory-adaptive
bucket approaches already have prior local evidence and are not rerun.

Initial qualification: independent step-by-step u128 matrices, signed BigInt
full-integer divisibility/sign/range, arbitrary odd-second operands, pure
2^62 matrix-row boundaries, and complete Fq/Fr inversion against Arkworks and
x*inverse=1. Include raw Montgomery zero, one, p-1, powers/neighbors, fixed
full-width random values and output-fed chains. Native field/core suites and
generic field suites must pass before timing; source recovery must reconstruct
all176 accepted files. Fixed stack state only; no heap, tables, point/key/solver
copies, pool or architectural RAM cap in the candidate.

If qualified, compare complete checked G1/G2 query owners, REF/freshOFF/ON,
primary1x2andgrouped2x2 at1/16/32workers with unchanged A32 negative controls,
prospective fixed blocks/gates and automated PMU/profiles. Exact timing plan,
sources, identities and raw reader must be frozen before preflight. No isolated
inverse microbenchmark may select adoption. No posthoc block, worker, fixture
or width rescue of a failed fixed candidate. Passing query callers only selects
ALL8+54 SDK/Gnark/resource/concurrency qualification, not adoption/completion.
Goal remains ALL8 verified CPUAVX >=10x supplied M5CPU, reporting CPU and Metal.

During any preflight or recording, only poll its exact confirmed live handle
to terminal; no overlapping helper/build/hash/report/push. Retain failures and
all outcomes. Commit and push each completed experiment and update handoff.
