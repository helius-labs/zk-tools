# Paired Montgomery cancellation component

Two-digit lookahead gives small, context-dependent component gains. The fixed aggregate gate passes, but ordinary R256 multiplication regresses; **a blanket switch is not selected**. Winning contexts need validation inside their real callers and complete proofs. Production remains unchanged, with accepted local1x2 still8.324853ms at134.50MiB peak RSS.

The private Rust kernel forms two product rows, computes both Montgomery quotient digits from a104bit inverse, then cancels them together using the accepted exact carry identity. The second quotient no longer waits for the first cancellation. The template adds four IFMAs,104/154 versus100/150 for product/sum2, and uses one additional temporary coefficient plus a second quotient. Representation, Montgomery factors, output normalization and mixed fused correction remain. [Algebra and bounds](artifacts/bounds.md) state the assumptions. This differs from the older rejected inverse-radix residue-fold reduction.

## Fixed component results

Ryzen9950X, CPU2, one OS thread;31 fixed rotated/reversed blocks. Each process measures100000 iterations after3x2000 primes. The frozen ordinary executable was built before appending the kernel. Fresh ordinary and paired share one new executable and identical probe/oracle source. All input/output conversions and the scalar oracle are outside the timer. Each process returns a fixed array; there is no timed heap allocation.

Positive change means faster. Changes are medians of paired per-block ratios against fresh ordinary, not ratios of the displayed medians. Units are ns per field-lane update; four-stream counts divide by four.

| Case | Frozen | Fresh | Paired | Paired change |
|---|---:|---:|---:|---:|
| Fq R256 dependent | 3.004533 | 3.008615 | 3.057595 | -1.645% |
| Fq R256 four streams | 2.288685 | 2.292063 | 2.317596 | -1.139% |
| Fq R256 fused | 3.221668 | 3.221429 | 3.173000 | +1.498% |
| Fq R260 dependent | 2.875214 | 2.874888 | 2.834023 | +1.287% |
| Fq R260 four streams | 1.967995 | 1.972532 | 1.919441 | +2.725% |
| Fq R260 fused | 2.996091 | 2.980625 | 2.994601 | -0.219%, uncertain |
| Fr R256 dependent | 3.010544 | 3.009643 | 3.051746 | -1.518% |
| Fr R256 four streams | 2.290328 | 2.290995 | 2.318268 | -1.217% |
| Fr R256 fused | 3.221366 | 3.211585 | 3.174365 | +1.443% |
| Fr R260 dependent | 2.882151 | 2.871055 | 2.830404 | +1.414% |
| Fr R260 four streams | 1.967078 | 1.970249 | 1.921943 | +2.450% |
| Fr R260 fused | 2.989316 | 2.982690 | 2.993511 | -0.221%, uncertain |

All12 fresh/frozen calibrations pass factor1.03. Paired equal-weight geometric aggregates are0.994567 versus frozen (interval0.993885–0.995460) and0.995838 versus fresh (0.994780–0.996801),30/31 winning blocks each. Every context's upper nonregression bound is<=1.03. There is no minimum gain floor. The aggregate does not weight actual prover work and cannot override the measured R256 ordinary regressions.

Intervals use individual ranks10..22 of31, nominal97.055% conditional on independent/exchangeable blocks; they are not familywise guarantees. Every block/outlier is retained, including block14's larger aggregate difference. No exclusions, extensions or unchanged rerolls were used. These intervals describe the frozen component run, not full-proof speed.

## Validation and profiles

Three exact integer tests pass once: low-two-digit quotients with unnormalized carries, independent final48bit repacking and47232 full reductions across both fields/radices. The full model checks every block and quotient against an independent BigInt recurrence, final exact representatives, field interpretation, inclusive lazy corners, random inputs and changing chains. Two new native tests pass43136 result lanes against Arkworks, including fused products and inclusive inputs through2p. The33 unchanged native field tests are filtered out and inherited, not freshly claimed.

All1116 latency processes,12 preflights and four profiles check their returned fields after capture against an independent scalar affine-power formula. This checks final results, not every iteration. Chain/fused return duplicated vector slots;32 checked returned fields do not mean32 independent states. There are231600204 measured loop iterations and6792000 prime iterations; these are not proofs.

After all latency, four separate cycles:u profiles run30000000 iterations at199Hz. They contain150/150 chain samples and162/158 fused samples (fresh/paired),620 total, one PID each and zero lost records. Almost all sampled period is attributed to the matching loop. Profiles include setup/primes/oracle and are not warm-gated. Raw data remains local with hashes; decoded samples and loss dumps are archived. No owned build, helper/hash/archive or second workload overlaps capture.

Actual linked loops contain the predicted104/154 IFMAs. For Fq R256 fused, instruction sites rise344 to381, literal stack allocation448 to640bytes, and RSP operand lines44 to50. The faster component therefore does not establish that instruction count or stack traffic fell. These static values are neither executed counts nor measured spills/RSS. The original `analysis.json` field named `stack_subtractions` broadly matches subtract mnemonics mentioning RSP, including `vpsubq`; only scalar `sub $imm,%rsp` entries represent literal frame allocation. The separate hot-code audit filters that distinction explicitly.

One extra inverse digit costs eight logical constant bytes per field; vector constant materialization is additional. There are no point/key tables, heap buffers, image changes or retained plans. Whole-prover memory, resource admission, portable devices, concurrency and250/4999 behavior remain unqualified. All172 production hashes stay unchanged. Removing the one include from the private field module recovers its original bytes; other private copied sources recover exactly. Original controls, actual libraries, sources, observer and xtask are pinned before/after.

No SDK requests, new exports or verifier calls occur; the independent proof ledger remains12524. No fresh AWS or Aeglos comparison is added. The next candidate should target mixed fused arithmetic and uniform ordinary multiplication, retain the original mixed ordinary/uniform fused paths, and test the actual affine/FFT contexts before any proof-latency claim. No production adoption follows from this component report.

## Inherited CPU and Metal comparison

The [accepted all-eight report](../2026-10-27-clean-fft-adoption/README.md) remains authoritative, including its full-performance-gate failure and engineering exception. M5 figures are supplied historical references. Original all-eight CPU10x goal: active and unmet,0/8.

| Shape | Accepted ms | M5 CPU ms | CPU/accepted | M5 Metal ms | Metal/accepted |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37 | 4.445x | 24 | 2.883x |
| 2x2 | 11.460675 | 59 | 5.148x | 30 | 2.618x |
| 4x2 | 15.854365 | 109 | 6.875x | 48 | 3.028x |
| 8x2 | 24.084626 | 189 | 7.847x | 74 | 3.072x |
| 12x2 | 33.693397 | 284 | 8.429x | 101 | 2.998x |
| 16x2 | 39.981636 | 335 | 8.379x | 124 | 3.101x |
| 24x2 | 66.010670 | 528 | 7.999x | 176 | 2.666x |
| Merge24x1 | 65.086624 | 507 | 7.790x | 175 | 2.689x |

Accepted merge54 remains161.341835ms without a supplied M5 reference.

[Protocol](artifacts/component.md), [exact model](artifacts/model.rs), [native kernel](artifacts/paired.rs), [full results](artifacts/analysis.json), [hot-code audit](artifacts/hot-code.json), [source/library pins](artifacts/pins.json), [artifact hashes](artifact-index.json).
