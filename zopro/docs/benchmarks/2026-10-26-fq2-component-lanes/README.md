# Real and imaginary components in one SIMD vector

Rejected by the fixed arithmetic-component test. Packing four Fq2 elements into one eight-lane vector is **42.4% slower across the four workloads** and **2.14× slower in the affine expression**. Every workload loses all 31 paired blocks. No SDK trial or production change follows.

The candidate packs four real components in the lower half and their four imaginary components in the upper half. Two such vectors produce the same eight complex results as the reference's separate real/imaginary vectors. Existing half extraction/join, negation and fused Montgomery operations compute both outputs together. The field representation and kernels are unchanged. This tests register layout, without a new table, heap buffer, assembly, key copy or retained preparation.

Both tests and measurements explicitly link the qualified BN254 library, SHA-256 `680805fb88bce25c54240d20ca88dd455b80aea1c4c2c496e11ecf66a32200b9`. The standalone probe SHA-256 is `f70de4c244e374162ec06fee09f0d6ff5ec368f4b5a94d64c120e97524a81ed9`. The 170 accepted source files and qualified SDK remain unchanged.

## Component measurements

AMD Ryzen 9 9950X, one OS thread pinned to CPU 2. The fixed protocol rotates four workloads and alternates mode order over 31 fresh-process blocks. Each process performs three 2,000-iteration primes and 1,000,000 measured iterations. Both modes calculate eight Fq2 elements per iteration; the independent-stream workload calculates four such states. Initial input preparation/packing and final canonical conversion are outside timing. Arithmetic and a final optimizer barrier are inside. No other owned compilation, profile or benchmark overlaps latency.

| Workload | Reference ns/Fq2 | Interleaved ns/Fq2 | Paired ratio | Wins |
| --- | ---: | ---: | ---: | ---: |
| Dependent chain | 6.529232 | 8.447954 | 1.294656 | 0/31 |
| Four independent streams | 6.037774 | 7.929789 | 1.312907 | 0/31 |
| Product and real scaling | 11.663345 | 13.203625 | 1.131838 | 0/31 |
| Norm and affine expression | 29.761568 | 63.625878 | 2.137762 | 0/31 |

The aggregate is the geometric mean of the four paired median ratios: **1.424075201**, with **0/31** winning block aggregates. The prospective gate requires an improving affine expression and aggregate, each with at least 21/31 wins, plus every workload within 3%. It fails all four nonregression checks and both improvement checks. There is no positive-gain floor, discarded sample, threshold change or unchanged reroll.

All 248 measured processes complete 248 million iterations and 1,488,000 priming iterations. Every corresponding final canonical word agrees: 16,864 64-bit word comparisons across all states, including updated real scales. This is deterministic arithmetic, not fresh proof generation.

The affine workload evaluates denominator norms and the ordinary G2 reverse-pass coordinate formulas with a changing state and supplied real factor. It excludes scalar inversion, prefix-chain construction, streaming allocation, bucket collisions and full proving. Its state need not remain a curve point between iterations. Separate oracle tests below use valid curve points and actual inverse norms.

## Linked code and profiles

The exact outer functions and their arithmetic callees are retained. LLVM outlines array construction in both layouts. For example, the reference affine outer routine has 21,025 bytes and 1,050 static IFMA sites; the candidate has 18,124 bytes and 705 sites. Those counts exclude callees. The reference scaling helper has 100 IFMA sites and the candidate multiplication helper has 150, each traversing two outputs. Static outer counts therefore do not establish fewer executed operations or lower register traffic.

The reference copies the accepted field formulas but uses an array-map wrapper for real scaling; the production helper spells out two products. This is a standalone expression comparator, not the actual linked SDK kernel. The candidate uses half-width helpers for norms and some carries, as documented in the source. These differences, extraction/join operations, stack traffic and call boundaries remain combined in the timing result; no causal share is isolated.

After all latency measurements, one 20-million-iteration affine-expression capture per mode records user cycles at 999 Hz. The complete process is captured, including small setup, primes and output; there is no SDK warm-interval claim.

| Profile | Samples | Profiled ns/Fq2 | Outer-function period share | Outlined arithmetic helper share |
| --- | ---: | ---: | ---: | ---: |
| reference | 4791 | 29.878648 | 87.926% | 11.676% |
| interleaved | 10186 | 63.670650 | 87.839% | 12.043% |

All 14,977 decoded samples match raw sample-event counts, each capture has one process ID, and both have zero lost-event records. All 128 final profiled output words also agree. Period shares are sampled user-cycle attribution, not wall-time fractions or removable costs. The profiles support keeping the negative result, without attributing the slowdown solely to a call, spill or permutation.

## Validation and retained failures

Four new tests pass against the frozen accepted field library: 196,608 complex result-lane checks and 16,384 norm checks against independent Arkworks arithmetic, 864 modulus-corner/half-order checks, 384 ordinary G2 additions against Arkworks, and 20 short changing-state comparisons. Inputs include normalized lazy values on both sides of p and below 2p, signs, zeros and eight-step chains. The ordinary curve formula requires distinct x coordinates; this component does not implement bucket exception handling for identity, doubling or cancellation.

The initial oracle compilation omitted Arkworks' `AdditiveGroup` trait for its zero constant. It failed before any test or measurement. The import-only repair and original source/logs are archived. Cargo builds the oracle dependencies, while explicit Rust links select the accepted BN254 library for the tests and timed probe.

The first profile command exits successfully, then the workflow rejects changed executable inode metadata. Inspection finds the actual hard link created by perf's build-ID cache; contents and mtime are unchanged. All 200 content pins match. The successful reference capture is retained, then decoded without recapture; the candidate capture disables build-ID cache updates. Both the rejection and repaired workflow are archived. All 206 final content pins and the accepted source hashes pass the closing audit.

The arithmetic uses fixed-size vector states. This experiment adds no key table or persistent data, but supplies no process-memory, stack-bound, pressure, concurrent-caller, portable-device or minimum-RAM qualification. There are **zero new SDK proofs and zero Gnark exports**. No AWS, Aeglos, phone or large-circuit measurement is added.

## Accepted prover context

These historical accepted local full-SDK timings are unchanged by this component. CPU and Metal references are the supplied M5 figures, not fresh Apple measurements. The all-eight 10× CPU goal remains unmet.

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

Accepted merge-54 is 178.891799 ms; no corresponding M5 reference was supplied. The [accepted grouping-module qualification](../2026-10-25-grouping-module/README.md) and [prior shared-products integration](../2026-10-25-shared-complex-products/README.md) provide the surrounding context. This new result rejects the tested layout, without claiming that every possible interleaved representation is slower.

[Protocol](artifacts/experiment.json), [component results](artifacts/component-result.json), [all process receipts](artifacts/timing), [probe source](artifacts/probe.rs), [native tests](artifacts/build-v2/oracle-tests/stdout.log), [linked outer routines](artifacts/codegen.json), [outlined helpers](artifacts/helper-codegen.json), [profile analysis](artifacts/profile-analysis.json), [closing audit](artifacts/closeout.json), [artifact hashes](artifacts/SHA256SUMS). Raw captures and expanded decode/dump streams remain in the local archive with hashes and are omitted from Git.
