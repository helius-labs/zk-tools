# Phased quadratic-extension multiplication

The component fails its fixed speed gate and is not adopted. Its paired aggregate is 12.47% slower than accepted ordinary arithmetic, 12.40% slower than fresh ordinary arithmetic, and 31.28% slower than the earlier shared-CIOS component. None of the 31 block aggregates wins against any control. The smaller standalone stack frame does not translate into a speed improvement.

## Arithmetic and scope

For exact integer products A=a0*b0, B=a1*b1 and C=(a0+a1)*(b0+b1), the prototype first forms D=A−B and S=−(A+B). It completes the real Montgomery reduction before forming the exact input sums and accumulating C into S for the imaginary reduction. This uses three raw products and two reductions, as the earlier [shared-product experiment](../2026-10-25-shared-complex-products/README.md) did, but replaces simultaneous row CIOS with phased Comba convolutions. Logical phase boundaries do not force LLVM register lifetimes.

Inputs have normalized radix-52 limbs and values at most 2p, with p < 2^254. Exact sums fit below 2^256. Signed carries preserve the real difference; biasing its reduced value by p and conditionally subtracting 2p gives an output below 2p. The imaginary cross-product is nonnegative and its reduced result also needs at most one 2p correction. The [range argument](artifacts/range-argument.md) covers signed coefficients and mixed-radix packing.

Only two native field files gain methods. Removing the additions recovers both original files byte for byte; the other 168 accepted source files match. There is no core dispatch, scheduler, key table, transformed key, persistent cache or heap buffer. This experiment does not measure actual G2 callers, complete curve operations, proof latency or minimum RAM.

## Fixed component comparison

Ryzen 9950X, one thread pinned to CPU 2, 31 rotated/reversed blocks, four patterns and four variants. Every process runs three 2,000-iteration primes and 100,000 measured iterations. All 496 observations and 49.6 million measured iterations are retained; no other owned build, profile or archive overlaps latency. Packing and canonical conversion occur outside timing, and an optimizer barrier consumes state before the end timestamp. Numbers below are ns per eight-lane complex element per iteration, additionally divided by four for independent streams.

| Pattern | Accepted | Fresh ordinary | Shared CIOS | Phased Comba | Phased/fresh | Phased/shared |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| chain | 5.644715 | 5.650552 | 4.516361 | 7.012672 | 1.242448 | 1.553493 |
| streams4 | 5.458633 | 5.473374 | 4.264539 | 6.261561 | 1.147329 | 1.467670 |
| scaled | 9.074889 | 9.081038 | 8.088015 | 9.501508 | 1.046229 | 1.175019 |
| affine-expression | 21.340025 | 21.366951 | 20.607430 | 22.855023 | 1.070129 | 1.108662 |

Ratios are medians of block-paired ratios; times are medians of process observations. The gate requires aggregate improvement with at least 21/31 winning blocks against each of three controls, each pattern within 3% regression, and fresh/accepted calibration within a factor of 1.03. All 12 pattern checks and three aggregate checks fail. Calibration passes for every pattern, with maximum factor 1.002394263. Every pattern loses all 31 blocks against all three controls. All 41,664 final canonical word comparisons agree. No samples are excluded and no unchanged trial is repeated.

## Generated code and profiles

These are exact linked standalone wrappers, not inlined prover callers. Literal stack subtractions exclude alignment, pushes and callees; RSP operand lines are not measured spill counts.

| Wrapper | Bytes | Instructions | Static IFMA sites | Literal stack subtraction bytes | RSP operand lines |
| --- | ---: | ---: | ---: | ---: | ---: |
| accepted | 3646 | 569 | 300 | 248 | 40 |
| fresh | 3646 | 569 | 300 | 248 | 40 |
| shared | 6717 | 969 | 250 | 6008 | 210 |
| phased | 4984 | 803 | 260 | 312 | 38 |

The phased implementation uses full low-digit cancellation, accounting for ten more static IFMA sites than the earlier shared-CIOS wrapper. Its stack allocation is much smaller, but it still loses every timing comparison. This does not isolate a causal cost for cancellation, dependency chains, loads or spills. The actual repeat functions and disassemblies are archived too; the phased repeat function has 16,320 bytes of literal stack subtractions versus 21,696 for shared CIOS.

After all latency, four declared 999 Hz user-cycle captures run the affine expression for 20 million iterations each. These include setup, primes and output, rather than SDK warm intervals. All profile output words agree.

| Profile | ns/complex element | Samples | Lost records |
| --- | ---: | ---: | ---: |
| accepted | 21.415376 | 3430 | 0 |
| fresh | 21.475990 | 3441 | 0 |
| shared | 20.856912 | 3349 | 0 |
| phased | 23.002364 | 3685 | 0 |

The 13905 samples match raw sample-event counts, with one PID per capture and zero lost records. Symbol-period attribution is retained without treating it as removable wall time or a causal model.

## Validation and decision

All 33 existing native field tests and the generator consistency check pass. Four new tests cover Fq/Fr, R256/R260 and four/eight SIMD lanes, including zero, p, 2p, limb boundaries, lazy inputs and eight-step chains. They check 729,088 result lanes against Arkworks and 91,136 raw representatives against independent exact-integer REDC and signed-coefficient models. Outputs have normalized limbs and remain strictly below 2p. The maximum modeled prepacking coefficient is 51,068,970,446,924,509, below the conservative 144,115,188,075,855,872 bound. Test-only aligned representation access is documented in the source.

The first oracle build failed on an ambiguous BigUint shift type. One explicit test-variable type annotation repaired it before any oracle execution or timing. The failed receipt and initial source remain; successful native field tests were reused after verifying exit and stream hashes. Arithmetic, timing harness and gates were unchanged. All 213 final pins and the accepted 170 source files plus qualified SDK pass the closing hash audit.

There are no new benchmark SDK requests, proof exports or Gnark verifications in this component experiment. Two fixed source proof/public fixtures are separate. No actual G2 caller, portable device, concurrent SDK, memory-pressure, large-circuit, mobile, AWS or Aeglos qualification is claimed. The component is rejected before SDK integration. A new experiment needs a distinct arithmetic argument supported by generated code and profiles; reducing stack use alone is insufficient evidence.

## Accepted prover context

The following accepted results are inherited from the [production checkpoint](../2026-10-25-grouping-module/README.md), not measurements made by this component experiment. The CPU and Metal references were supplied historical M5 measurements.

| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1x2 | 8.594267 | 37 | 4.31× | 24 | 2.79× |
| 2x2 | 11.697247 | 59 | 5.04× | 30 | 2.56× |
| 4x2 | 16.158262 | 109 | 6.75× | 48 | 2.97× |
| 8x2 | 24.477005 | 189 | 7.72× | 74 | 3.02× |
| 12x2 | 34.386319 | 284 | 8.26× | 101 | 2.94× |
| 16x2 | 40.867383 | 335 | 8.20× | 124 | 3.03× |
| 24x2 | 69.265160 | 528 | 7.62× | 176 | 2.54× |
| merge-24x1 | 68.150148 | 507 | 7.44× | 175 | 2.57× |

Accepted merge54 remains 178.891799 ms; no M5 reference was supplied for it. The all-eight 10× CPU goal remains unmet.

[Protocol](artifacts/experiment.json), [decision and all block ratios](artifacts/component-result.json), [order](artifacts/timing-order.json), [source audit](artifacts/source-audit.json), [generated code](artifacts/codegen.json), [profiles](artifacts/profile-analysis.json), [closing audit](artifacts/closeout.json), [artifact hashes](artifacts/SHA256SUMS). Four raw captures and eight expanded profile streams remain hash-indexed locally and omitted from Git.
