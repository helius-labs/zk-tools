# Fq diagonal Montgomery experiment

Declared before model/native implementation or timing. Accepted production is
source175/a1585b86 and SDK20fde; its qualification and physical16 calibration
exception remain unchanged. Goal: all eight supplied CPU shapes at least10x M5;
0/8 currently. No budget limit or goal completion claim.

Hypothesis: product-scanning anti-diagonals interleaved with cancellation may
schedule better than accepted eight-lane row-scanning CIOS. It does not reduce
the nominal IFMA count: ordinary100/fused150. More live q digits or serial
dependencies may lose. Only eight-lane Fq/R256 ordinary and fused products are
eligible; four-lane, Fr/R260, representation and corrections stay unchanged.
No LUT, heap allocation, copied points/key, pool, solver, or resource-policy change.

Inspiration: ZKPU section2/3.4, https://arxiv.org/html/2610.27530v1.
It describes hardware FIPS. No CPU performance extrapolation or source copied.
The mixed-radix recurrence and CPU kernel here are independently derived.

Before native code: Rust checked-u128 digit interpreter versus independent
full-BigUint Montgomery reduction for both primes/radices, ordinary/fused,
corners and4096 random tuples per prime. Require exact U*R=T+Q*p, matching q,
normalized limbs, correction/range and cancellation-carry identity. Derive
conservative u64 bounds. These mathematical controls do not change native scope.

If valid, generate a fully unrolled native implementation, same inline policy,
and official ifma8 generator. OFF original CIOS body must remain exact. Add
candidate-only Arkworks comparisons and chains to the existing shared carry
test; retain all33 field tests and124 core/8 prover tests both builds. Inspect
actual linked SDK G1/G2 callers; no forced outline or assembly variant.

If native qualification passes, prospectively use the existing fixed primary
SDK protocol: accepted A, fresh OFF B, candidate C;15 rotated/reversed paired
blocks151 timed requests+3 primes each, all9-shape preflight proofs,3 stat and
3 profile jobs.7902 complete SDK requests/78 independent Gnark exports,
ledger17317->17395 only on successful close. All helper builds, PMU feasibility,
protocol/source/binary pins before captures; no owned competing work during
capture. No selective reruns, exclusions, extensions or threshold changes.
Use both paired upper confidence bounds<1 and>=10/15 wins against A and B;
OFF/A calibration within factor1.03. A failure closes this exact kernel.
A pass permits separate all8+54/physical16/portable/resource qualification;
it does not authorize automatic adoption or claim the overall goal achieved.

Commit and push the completed experiment, including negative results. Preserve
raw outputs and source provenance. No fresh AWS/Aeglos/mobile/giant-circuit claim.
