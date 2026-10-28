# Signed final-carry subtraction in G1 streaming

The complete caller gate passes. G1 additions improve **1.39–2.00%**, with unchanged scratch capacities and zero allocation calls in measured loops. Both borrowed signed cases win **31/31** blocks. This qualifies a controlled SDK experiment; production remains unchanged and no proving-time gain is claimed. [Decision](artifacts/interpretation.md).

The candidate folds subtraction into the final carry pass after the accepted CIOS Montgomery reduction and mixed-radix packing. The multiplication prefix stays unchanged. Arithmetic right shifts normalize signed coefficients; the existing conditional negative repair restores the same below-2p representation. Outputs match the original multiply-then-subtract sequence bit for bit. Only the native eight-lane G1 streaming X/Y output expressions use it.

Fused mul-sub itself is an older, unsuccessful experiment. The 22 October version normalized and injected `(2p-c)*R` before final cancellation; the 24 October canonical-Y version operated after normalized multiplication. This trial tests a distinct dataflow with neither preliminary delta normalization nor injection. Their failed gates remain unchanged. [History audit](artifacts/history-audit.md), [bounds](artifacts/bounds.md).

## Complete caller results

Ryzen 9950X, CPU 2, one Rayon worker. Fresh disabled/enabled builds use coherent frozen libraries and the same existing caller adapter. Each process performs three checked eight-call primes, resets outside timing, times 4,096 calls at 200 entries or 2,048 at 352, then checks the final output. The caller includes production dispatch, signed/borrowed handling, packing, inversion, records, tails and writes. Points and destinations are modeled; these are the actual batch sizes identified by prior request diagnostics, not a captured witness replay or complete MSM.

There are 31 rotated/reversed process blocks, 372 latency processes and 12 preflights. All observations remain. Paired change uses the median within-block enabled/disabled ratio, rather than the ratio of displayed medians. Intervals use ranks 10..22 of 31, nominal 97.055% under independent/exchangeable blocks, without a familywise guarantee.

| Caller | OFF µs | ON µs | Paired improvement | Ratio interval | Wins |
|---|---:|---:|---:|---:|---:|
| G1 owned, 200 | 3.964 | 3.907 | 1.388% | 0.985605–0.986506 | 25/31 |
| G1 owned, 352 | 6.473 | 6.375 | 1.502% | 0.984833–0.985489 | 24/31 |
| G1 borrowed signed, 200 | 4.170 | 4.091 | 1.865% | 0.980777–0.981686 | 31/31 |
| G1 borrowed signed, 352 | 6.844 | 6.706 | 2.000% | 0.979827–0.980317 | 31/31 |
| G2 prefix, 200, unchanged control | 7.549 | 7.541 | 0.083% | 0.998696–0.999531 | 26/31 |
| G2 prefix, 352, unchanged control | 12.747 | 12.738 | 0.075% | 0.998621–0.999722 | 27/31 |

The declared condition requires all four G1 upper bounds below one and at least 21 wins each; G2 paired medians must stay within 3% of parity. It passes without a minimum gain floor or threshold change. The small G2 differences are calibration observations, not a claim of a G2 arithmetic improvement.

## Validation and memory

OFF passes 33 field and 125 core tests; ON passes 35 field and 125 core tests, all unfiltered. Two new field tests cover coefficient renormalization identities and 185,760 lane outputs against independent Arkworks values, including both fields/radices, loose boundaries, exact 2p subtrahends and dependent chains. They also require exact lazy digits to match the original routine and preserve normalized below-2p bounds. Test-only assertions check the conservative coefficient range.

The reused caller's three tests cover 13 lengths from zero through 512, signed and unsigned operands, partial lanes, scratch reuse, resets and repeated calls against generator scalar sums. Every captured output passes its scalar-group oracle and matches between builds. These are curve/arithmetic checks; no new SDK or Gnark proof is counted.

All timed loops make zero allocation calls. G1 retained scratch is 40,000/70,400 bytes for 200/352 entries; borrowed references separately use 3,200/5,632 bytes. G2 prefix scratch is 16,000/28,160 bytes. Corresponding capacities match exactly across builds. These are caller-specific endpoint capacities, excluding common fixtures and stack; they are **not peak RSS or minimum RAM**. No new production heap scratch, lookup table or prepared-key structure is added. G2, weighted reductions, solver, keys, scheduler and resource forecasts remain unchanged.

## Profiles and reproducibility

All latency captures finish before four whole-process cycles:u profiles at 199 Hz, covering both borrowed signed sizes and both builds, with 1,048,576 calls each. There are 4,615 samples, one PID per recording and zero lost records. Setup, primes and final checks are included. Weighted symbol periods are retained; sampling does not establish a causal wall-time fraction or exact executed instruction count.

Corresponding G1 bodies retain 800 static IFMA sites. The borrowed signed FAST body has 4,423/4,397 instruction sites OFF/ON, with 591/593 RSP-operand lines. Static RSP operands include ordinary stack accesses; they are not spill counts, stack bytes or a proof-speed estimate. The timing result includes the compiler's actual scheduling and register allocation.

The first OFF caller link failed because the standard eight-library SDK linker omitted the diagnostic driver's direct sha2 dependency. Original protocol, command and error are retained in native-initial. A separate repair links the exact sha2 artifact from the frozen dependency set. Four successful OFF steps are reused and six remaining steps finish once; source and driver bytes remain unchanged. No successful test or measurement is repeated. [Repair](artifacts/harness-repair.json), [combined build receipt](artifacts/native-completion.json).

The private snapshot has 176 files. Of 174 originals, 170 match byte for byte, and reversing the four parent-file edits recovers all originals. The original CIOS body is untouched; a new field child derives its prefix and changes only the final carry/subtraction. The other new module is the unchanged diagnostic caller adapter. Production remains unchanged. Source, library, driver, tool, repair and input pins match before and after capture. No owned build, analysis, hash, archive or second benchmark overlaps capture.

Complete source, command/test logs, raw timing results, decoded profiles and selected assembly are archived. Executables, targets, raw perf data and hexadecimal dumps remain local with hashes. No new AWS, Aeglos, mobile/ARM, portable, pressure, giant-circuit or concurrent-proof qualification is claimed. The independent-proof ledger remains **15,553**.

## Inherited CPU and Metal comparison

The [accepted source174 report](../2026-10-28-g2-borrowed-adoption/README.md), including its failed performance gate and explicit engineering exception, remains authoritative. These timings are inherited; the M5 values are supplied historical references.

| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup | Required ms for CPU 10× |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 8.366931 | 37 | 4.422× | 24 | 2.868× | 3.7 |
| 2x2 | 11.539335 | 59 | 5.113× | 30 | 2.600× | 5.9 |
| 4x2 | 16.540233 | 109 | 6.590× | 48 | 2.902× | 10.9 |
| 8x2 | 25.136722 | 189 | 7.519× | 74 | 2.944× | 18.9 |
| 12x2 | 34.858408 | 284 | 8.147× | 101 | 2.897× | 28.4 |
| 16x2 | 41.377043 | 335 | 8.096× | 124 | 2.997× | 33.5 |
| 24x2 | 67.764001 | 528 | 7.792× | 176 | 2.597× | 52.8 |
| Merge24x1 | 67.096516 | 507 | 7.556× | 175 | 2.608× | 50.7 |

Accepted Merge54×1 remains 164.388184 ms without a supplied M5 reference. Accepted primary RSS median/max remains 131.25/135.00 MiB. This component establishes no new proof latency or process memory result. The all-eight CPU 10× goal remains **active and unmet, 0/8**.

[Protocol](artifacts/design.md), [analysis](artifacts/analysis.json), [generated field method](artifacts/source/crates/zopro-bn254/src/ifma8/signed_final.rs), [field tests](artifacts/native/zopro-bn254-candidate/stdout.log), [core tests](artifacts/native/zopro-core-candidate/stdout.log), [source audit](artifacts/source-audit.json), [pins](artifacts/pins.json), [workflow completion](artifacts/runs/result.json), [artifact index](artifact-index.json).
