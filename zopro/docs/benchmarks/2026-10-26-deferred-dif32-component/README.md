# Deferred correction in five-stage FFT DIF tiles

**Component gate passed; full-proof integration is not yet measured.** The new tile takes 32.15% less time than the accepted-library reference and 32.14% less than the freshly compiled reference. All three cases improve in all 31 paired blocks against both controls. Accepted prover source and timings remain unchanged.

The routine defers modular correction inside the existing 32-vector, five-stage decimation-in-frequency tile. It keeps normalized radix-52 limbs, biases differences by known multiples of p, and uses the unchanged ordinary Montgomery multiplication. It restores the ordinary below-2p representation before returning. Final correction requires 31 conditional passes across the tile, rather than correcting each of its 80 low butterfly outputs. Positive biased differences also avoid the original subtraction's conditional borrow repair.

No lookup table, root preparation, retained cache, key data, heap allocation, field representation or scheduler change is added. Only a new experimental block in the four-lane field source and its generated eight-lane version changes. Removing each block recovers the complete original file byte for byte; the other 168 accepted source files are identical. This experiment does not dispatch the routine from the prover.

## Bounds and independent checks

The uniform radix is R=2^260. For either BN254 prime, 64p<R. Tile inputs are normalized below2p and every root is canonical belowp. At stage s, sum and biased difference are below2^(s+2)p, hence below64p. Multiplying the difference by its root has T<64p²<pR, so ordinary REDC returns below2p. The number of trailing low branches determines each final output bound; descending power-of-two modulus corrections restore below2p. [The complete local range argument](artifacts/range-argument.md) also bounds signed bias coefficients below2^58 and ordinary CIOS coefficients below32·2^52.

[Scott's Slothful reduction](https://eprint.iacr.org/2017/437) motivates delaying correction when intermediate ranges permit it. Its primary abstract and indexed excerpts were consulted; the full PDF request returned403. The BN254/R260 tile schedule, range proof and implementation here are independently derived, with no imported performance estimate. This differs from the prior R260 bucket, bounded-subtraction, prepared-root and FFT scheduling trials.

All 33 existing native field tests and generator consistency pass. Six new tests check four/eight-lane implementations:

- 49,152 raw output residues match an independent arbitrary-precision Montgomery/schedule model exactly, before host conversion. All output digits are normalized and values below2p; compact/expand roundtrips agree.
- 122,880 modeled Montgomery products match an independently computed full-radix REDC identity. The model also checks ordinary CIOS coefficients; its largest observed coefficient is 47,239,727,476,272,382, below the conservative 144,115,188,075,855,872 limit. The existing four-lane implementation uses diagonal multiplication; its raw tile outputs are checked by the same exact-value oracle.
- 5,760 field outputs match direct 32-point Fourier sums computed in Arkworks, with nonzero column twists, zeros, ones, impulses and signed/full-width values.
- Twenty repeated eight-lane tiles check 5,120 outputs against the existing field implementation. Mixed R256 invocation fails with the expected assertion.

The exact model reaches 260-bit intermediates and covers inputs near2p, canonical roots nearp, zero/one roots and random normalized values. These are deterministic tests plus the range argument, not exhaustive enumeration of all field values. The twiddle-belowp precondition must be discharged for actual FFT call sites before integration.

## Fixed component comparison

Ryzen 9950X, one OS thread pinned to CPU2. Thirty-one rotating/reversing fresh-process blocks compare the exact accepted BN254 library with fresh-reference and deferred modes from one new executable. The reference implements the current five-stage column arithmetic. All modes perform three2,000-tile primes and100,000 measured tiles per process, with changing state, then an optimizer barrier. Each tile processes32 vectors/eight lanes. Initial packing, root construction, output conversion and reporting are outside timing. No owned build, profile, archive or other benchmark overlaps unprofiled timing.

The q/column cases supply synthetic canonical roots for independent32-point transforms and twists. They are arithmetic component geometries, not complete packed prover FFTs or real-key SDK requests.

| q / column | Accepted ns/output | Fresh reference ns/output | Deferred ns/output | Paired / accepted | Paired / fresh | Wins against each |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 / 0 | 7.260469 | 7.258834 | 4.926847 | 0.678508226 | 0.678533532 | 31/31 |
| 256 / 173 | 7.261798 | 7.260251 | 4.925845 | 0.678464968 | 0.678481994 | 31/31 |
| 16384 / 8191 | 7.259544 | 7.257019 | 4.924104 | 0.678539073 | 0.678670062 | 31/31 |

The geometric means of paired case medians are 0.678504088 against accepted and 0.678561858 against fresh, both with31/31 winning block aggregates. Every case passes the3% nonregression and fresh-calibration conditions. The primary q256 case and aggregate must improve against both controls with at least21/31 wins. There is no minimum positive-gain floor, omitted sample, gate change or unchanged timing retry.

The279 measured processes complete27,900,000 timed tiles and1,674,000 prime tiles. Every final canonical word agrees across modes:190,464 64-bit word comparisons against the two controls, including all256 outputs per process. This is arithmetic verification, not proof verification.

## Linked code and profiles

| Linked repeated-tile function | Bytes | Instructions | Static IFMA sites | RSP operand lines |
| --- | ---: | ---: | ---: | ---: |
| accepted | 2223 | 422 | 100 | 39 |
| fresh | 2223 | 422 | 100 | 39 |
| deferred | 2347 | 457 | 100 | 26 |

The new function is slightly larger; both loops retain100 static IFMA sites. Source-level multiplication count is unchanged at80 vector products per tile. Fewer static stack references are not a measurement of spill cost or memory traffic. Full disassemblies and call sites are retained.

After all latency measurements, one q256/column173 capture per variant records5,000,000 tiles at999Hz user-cycle sampling with build-ID cache updates disabled. The complete component process is captured, including setup, primes and output.

| Capture | Samples | Profiled ns/output | Period share in repeated-tile routine |
| --- | ---: | ---: | ---: |
| accepted | 9302 | 7.262153 | 99.9131% |
| fresh | 9326 | 7.276945 | 99.9671% |
| deferred | 6343 | 4.942959 | 99.9681% |

All24,971 decoded samples match raw sample-event counts, with one process ID per capture and zero lost-event records. All2,048 final profiled word comparisons against both controls agree. The sample shares locate the work; they are not removable SDK latency fractions or a causal decomposition.

## Scope and retained harness failures

The first oracle compilation failed on test-only BigUint/reference/index type inference. Those annotations were repaired without changing field arithmetic. The six oracle tests then passed, but the workflow's required-name reader included libtest's `- should panic` suffix while the requested name omitted it. The successful exit, six cases, unfiltered summary and exact output hashes were independently audited and reused; no test or timing was repeated. Both failed workflow records, original source and repairs remain archived.

All214 final input pins and the170 accepted production hashes pass the closing audit. This stage generates zero SDK proofs and zero Gnark exports. No full FFT, SDK, pressure, stack-peak, minimum-RAM, concurrent-caller, portable-device, AWS, Aeglos or mobile result is established. The routine uses fixed-size temporaries and adds no heap storage; that is not a whole-process memory bound.

The next experiment must use this byte-pinned arithmetic in the real FFT, verify cached/uncached roots and storage boundaries, and measure all nine shapes plus the physical-core control with fresh SDK proofs and independent Gnark verification. The component pass is insufficient for adoption or for the original10× goal.

## Accepted full-proof context

Historical accepted local timings below are unchanged. M5 CPU/Metal figures are supplied references, not fresh Apple measurements. None of the eight CPU cases reaches10×.

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

Accepted merge-54 is178.891799ms; no M5 reference was supplied for it. The [accepted grouping-module report](../2026-10-25-grouping-module/README.md) remains the production checkpoint.

[Protocol](artifacts/experiment.json), [results](artifacts/component-result.json), [all timing receipts](artifacts/timing), [range derivation](artifacts/range-argument.md), [new arithmetic](artifacts/field-block.rs.txt), [oracles and driver](artifacts/probe.rs), [oracle qualification](artifacts/oracle-qualification.json), [source recovery audit](artifacts/source-audit.json), [linked code](artifacts/codegen.json), [profiles](artifacts/profile-analysis.json), [closing audit](artifacts/closeout.json), [artifact hashes](artifacts/SHA256SUMS). Raw captures and six expanded profile streams remain hash-indexed in the local archive and are omitted from Git.
