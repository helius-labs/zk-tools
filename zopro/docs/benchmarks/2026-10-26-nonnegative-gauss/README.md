# Nonnegative Gauss quadratic-extension product

The fixed component gate passes against accepted and fresh ordinary arithmetic: paired aggregate ratios are 0.963646405 and 0.963722514, with 31/31 aggregate block wins against both controls. The affine-expression pattern regresses 2.8%, and the prior streamed signed method remains faster in isolation. This is not a verified prover gain. Production is unchanged.

## Arithmetic

For normalized inputs in [0,2p], form exact raw arrays S=a0+a1, N=2p-a1 and stream T=b0+b1, D=b1+2p-b0 one radix-52 digit per row. Share P=S*b0 between X=P+N*T and Y=P+a0*D. These satisfy X=real+2p*T and Y=imag+2p*a0. Their nonnegative accumulators are each at most 12p².

R256>4p gives REDC(X),REDC(Y)<4p, so one conditional subtraction of 2p suffices. R260>64p gives output<1.1875p, so no final correction is required. Prepared arrays can exceed 2p and remain raw limbs rather than public field objects. Final 48-bit mixed-radix cancellation masks are preserved. The shared raw-product phase uses four vector additions per limb pair instead of eight in the previous signed method; preparation costs and generated caller code may offset that saving.

Only ifma.rs and generated ifma8.rs gain additive methods, including the exact prior streamed method as a diagnostic control. Removing additions and their two separator blank lines recovers the original whole files exactly; all other 168 accepted source files match. No core dispatch, scheduler, table, buffer, heap allocation, barrier or assembly changes.

## Component measurements

Ryzen 9950X, one thread on CPU 2, 31 rotated/reversed blocks, four patterns and four variants, three 2,000-iteration primes and 100,000 measured iterations per process. All 496 processes and 49.6 million measured iterations are retained. Packing and conversion are outside timing. No owned build, profile or archive overlaps latency. Times are ns per eight-lane complex element per iteration, additionally divided by four for streams4. Ratios are medians of block-paired ratios.

| Pattern | Accepted | Fresh ordinary | Prior streamed | Positive Gauss | Gauss/fresh | Gauss/streamed |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| chain | 5.667560 | 5.672155 | 4.552886 | 5.308596 | 0.935354 | 1.165355 |
| streams4 | 5.483704 | 5.490357 | 4.304778 | 5.152512 | 0.939032 | 1.196839 |
| scaled | 9.139999 | 9.124106 | 8.129357 | 8.706206 | 0.955243 | 1.072746 |
| affine-expression | 21.424669 | 21.435464 | 20.034163 | 22.017509 | 1.028107 | 1.099226 |

The declared gate requires aggregate improvement and at least 21/31 block wins against accepted and fresh ordinary, each pattern within 3% regression, and fresh/accepted calibration within a factor of 1.03. The prior streamed method is diagnostic only, as frozen before timing; Gauss is 13.245% slower against it in aggregate. All required gates pass, including affine regression just below the declared limit. No minimum gain floor, sample exclusion or unchanged retry. All 41,664 canonical word comparisons agree. This result supports inspecting actual caller code; it does not justify adoption or imply an end-to-end improvement. The [prior streamed SDK trial](../2026-10-26-streamed-complex-sdk/README.md) remains rejected.

## Linked code and profiles

These are actual linked component wrappers and full repeat functions, not prover G2 callers. Literal stack subtractions omit alignment, pushes and callees; RSP operand counts are not dynamic spill or traffic counts.

| Variant and function | Bytes | Instructions | IFMA sites | Stack subtraction bytes | RSP operands |
| --- | ---: | ---: | ---: | ---: | ---: |
| accepted wrapper | 3646 | 569 | 300 | 248 | 40 |
| accepted repeat | 28581 | 4533 | 2100 | 11584 | 586 |
| fresh wrapper | 3646 | 569 | 300 | 248 | 40 |
| fresh repeat | 29390 | 4535 | 2100 | 15232 | 608 |
| streamed wrapper | 5570 | 870 | 250 | 1144 | 86 |
| streamed repeat | 40224 | 6171 | 1850 | 22016 | 1030 |
| gauss wrapper | 5603 | 807 | 250 | 4088 | 155 |
| gauss repeat | 37507 | 5677 | 1850 | 19520 | 1204 |

The expected 250 standalone IFMA sites are retained. Fewer instructions than the prior signed method coexist with a larger standalone frame and more RSP operands in the repeat. Static counts do not establish a performance cause. Accepted/fresh repeat contexts differ while calibration remains within the fixed gate.

After all latency observations, four 999 Hz cycles:u profiles run 20 million affine-expression iterations each. They include setup, primes and output, rather than SDK warm intervals.

| Profile | Samples | ns/element | Leading repeat-symbol period % | Lost records |
| --- | ---: | ---: | ---: | ---: |
| accepted | 3442 | 21.492934 | 99.969 | 0 |
| fresh | 3448 | 21.469311 | 99.966 | 0 |
| streamed | 3237 | 20.157257 | 99.820 | 0 |
| gauss | 3548 | 22.147624 | 99.970 | 0 |

All 13675 samples match raw event counts, with one PID/TID per capture, zero lost records, and all 192 profile output-word comparisons equal. Symbol attribution does not isolate opcode costs or removable wall time.

## Validation and limitations

All 33 existing native field tests and generator consistency pass. Four independent oracle tests cover Fq/Fr, R256/R260, four/eight lanes, lazy and limb boundaries, all 16 combinations of zero/2p, and eight-step chains. There are 91,392 exact raw output comparisons against BigInt REDC and an independent unsigned scalar CIOS model, plus 731,136 chained Arkworks output checks. Prepared sums/differences and final-zero carries are reconstructed independently. The largest modeled coefficient is 69,178,680,307,411,985, below the conservative 144,115,188,075,855,872 bound. Mixed packing is below 2^62; output digits are normalized and values below 2p.

The first oracle build failed to infer BigInt/BigUint shift types. Explicit integer types and usize shift widths repaired test code before any oracle execution or timing; kernel arithmetic and runtime probe code were unchanged. Initial test sources, failed receipts and all 187 original pins are retained and audited via exact source relocation. The corrected 190 and final 212 pins match. An earlier inverse-source audit expected different separator blank lines; only that audit was repaired before build pinning. Both implementation-helper versions are retained.

No new SDK proofs, Gnark exports, actual G2 caller timing, mobile, portable, concurrent, pressure, large-circuit, AWS, Aeglos or minimum-RAM result is claimed. A separate actual-caller/full-prover qualification is required before adoption.

## Inherited accepted prover comparison

These unchanged accepted measurements are compared against supplied historical M5 references. They are not timings from this component and not a same-host comparison.

| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1x2 | 8.594267 | 37.0 | 4.305× | 24.0 | 2.793× |
| 2x2 | 11.697247 | 59.0 | 5.044× | 30.0 | 2.565× |
| 4x2 | 16.158262 | 109.0 | 6.746× | 48.0 | 2.971× |
| 8x2 | 24.477005 | 189.0 | 7.722× | 74.0 | 3.023× |
| 12x2 | 34.386319 | 284.0 | 8.259× | 101.0 | 2.937× |
| 16x2 | 40.867383 | 335.0 | 8.197× | 124.0 | 3.034× |
| 24x2 | 69.265160 | 528.0 | 7.623× | 176.0 | 2.541× |
| merge-24x1 | 68.150148 | 507.0 | 7.439× | 175.0 | 2.568× |

Accepted merge54 remains 178.891799 ms with no supplied M5 reference. The all-eight CPU 10× objective remains active and unmet (0/8).

[Protocol](artifacts/experiment.json), [paired results](artifacts/component-result.json), [codegen](artifacts/codegen.json), [profiles](artifacts/profile-analysis.json), [source audit](artifacts/source-audit.json), [harness repair](artifacts/harness-repair.json), [closing audit](artifacts/closeout.json), [artifact index](artifacts/artifact-index.json). Four raw captures and eight expanded streams are retained locally with hashes and excluded from Git. Executables and caches are represented by pinned identities and sources.
