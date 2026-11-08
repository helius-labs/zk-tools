# Late-centered native arithmetic qualification

**The private AVX implementation passes four new native tests, including complete diagnostic G1/G2 streaming updates. Performance remains unmeasured and production is unchanged.** This selects a separate matched caller experiment, not an SDK integration or adoption. It follows the [exact signed-affine and late-compensation models](../2026-11-08-late-centered176/README.md).

The fixed recipe retains signed intermediates through the prefix/reverse circuit, performs ordinary unsigned CIOS, compensates input signs at the end, and centers only the final Montgomery quotient. Potential savings concern modular corrections between products. Sign compensation, mixed-radix shifts, normalization, inversion boundaries, wider export and compiler lifetimes remain costs. No speed estimate follows from correctness.

## Implementation and source scope

A private copy contains 18 original field files. Seventeen stay exact; the generated eight-lane parent gets one append-only child declaration. Removing that suffix recovers the original parent byte-for-byte, and the original generator/function bodies remain unchanged. All 176 production inputs still match the accepted manifest. The new child is guarded by the parent's existing AVX512IFMA/VL/DQ configuration. Production has no dispatch or API change.

Separate Signed and Loose types keep signed values out of existing Fp/Ifma8/get/compact contracts. Four low radix52 digits are normalized before multiplication; the top digit is signed. Raw fixture construction has explicit unsafe magnitude/digit preconditions, independently checked by the qualifier. These investigation types and diagnostic callers are not a new general-purpose production field API.

The unsigned six-coefficient CIOS schedule and canceled-low-product recurrence are retained. Difference2 pays third-input negation/normalization. Late input compensation, final quotient centering and signed carry normalization are explicit. Canonical host key and bucket coordinates retain their original compact R256 encoding. R260 is an arithmetic cross-check; actual G1/G2 diagnostic callers use R256.

Signed roots normalize before the original eight-lane product tree/scalar-root inverse, whose final lazy inverse also normalizes below p. Final coordinate export biases by 4p, normalizes, estimates one quotient on [0,8p), canonically reduces and transposes. R256 uses the original direct canonical transpose/store sequence. R260 uses its original radix-converting get after canonicalization; that extra correction is retained in the cross-check.

G1 records retain five 320-byte fields, with a Loose xsum; G2 prefix records retain two 320-byte fields and the original coordinate rereads. Scalar tails use unchanged scalar curve/field arithmetic. No new key table/copy, persistent recoded points, input-sized auxiliary cache, solver plan, production pool or C/ASM rewrite is introduced. Record payload sizes are not process RSS, live-memory or minimum-RAM bounds.

## Executed native checks

All four newly authored, unfiltered tests pass once. No existing field/core/prover test suite or old mathematical population is replayed.

|New coverage|Executed comparisons/cases|
|---|---:|
|Exact signed SIMD outputs, ordinary/sum/difference, Fq/Fr and R256/R260|77,664|
|Independent Arkworks field meanings of those outputs|77,664|
|Wide signed exports, strictly inside ±4p|8,608|
|Canonical imports|4,096|
|Signed root inverse outputs and canonical inverse boundary|4,096|
|Complete diagnostic G1/G2 point comparisons, including untouched slots|183,296|
|Additional threaded point outputs|85,012|
|Warm ordinary single-thread allocation/capacity cases|8|
|Diagnostic whole-call fallback cases|44|

Primitive cases cover all 169 pairings of 13 signed edges through ±5p, 512 full-width signed vector tuples and 128 output-fed vector steps per field/radix, all three operations and all eight lanes. Each native result matches an independent BigInt full-radix centered quotient and Arkworks field arithmetic. Low-digit normalization and exact signed representatives are checked, not congruence alone. Wide export cases include modulus aliases and values close to ±4p; imports and inverse boundaries check both radix tags.

Complete callers cover sizes 0/1/7/8/9/15/16/31/32/33/199/200/201/351/352/353/511, positive/negative/mixed arrivals and three changing passes. Independent Arkworks group addition checks full bucket arrays, including untouched destinations. Immutable incoming points are preserved. Counts include repeated identities, inherited fixtures within each new test, and comparisons across cases; they are not all unique random points.

The checked diagnostic adapter scans before publication and chooses unchanged scalar whole-call fallback for identity inputs, equal-X/doubling/opposite points or duplicate destinations. Its O(n²) duplicate scan is **not a proposed production Window guard**. These tests do not qualify the actual production Window/queue dispatch or arbitrary malformed input behavior. Ordinary raw caller entrypoints retain their finite, distinct-destination, nonzero-denominator preconditions.

Eight warm allocation cases cover G1/G2 at 200/201/352/353 points. After three warm updates, each observed ordinary call requests zero allocations/reallocations and retains exactly the same group/tail capacities; outputs are still independently checked. This does not establish zero allocations for a fresh proof, all fallback paths or all worker populations.

Shared immutable fixtures run on 1/4/16/32 scoped workers with 512 KiB requested stacks, each using 200/201-point callers and two changing passes. Each worker owns its bucket/scratch state. There are 53 worker executions in total and 85,012 additional touched-point checks. These are native concurrency/correctness tests, not Rayon scheduling, concurrent-proof throughput, measured stack peaks or a mobile-memory qualification.

## Builds, repair and generated code

The private field build succeeds first. The initial test link then fails because the selected BigInt archive lacks the bitcode needed for fat LTO. The preserved continuation changes only that direct dependency to its coherent cached LTO artifact; field/kernel/caller/test sources and compiler settings stay unchanged. It reuses the completed field library and runs only the failed link and unstarted tests/generic compile. The repaired link, four-test execution and generic noASM/noIFMA field compile pass once. The generic check proves native-child gating at the field crate, not full-prover portability or ARM execution.

There are four successful command receipts and one retained failed link. Builds use Rust 1.98.1, -O, native CPU, embedded bitcode and one codegen unit. The native consumer has fat LTO, overflow checks and debug assertions enabled. Inherited unused-import/dead-code warnings and one unused test import remain; no cleanup rebuild occurs. These correctness consumers are not accepted release SDK artifacts.

Two actual native caller functions are decoded once from that final test executable. Whole-function counts include inverse branches, tails and other paths; backward spans overlap and are all preserved in [qualification](artifacts/qualification.json). Static stack references are not dynamic spill counts or cycle attribution.

|Diagnostic caller|Bytes|Instruction rows|Whole-function IFMA sites|Stack-addressed rows|
|---|---:|---:|---:|---:|
|g1|37109|6120|1602|873|
|g2|67484|11054|2454|2014|

The G1 forward/reverse spans contain 200/402 IFMA sites; G2 spans contain 250/1,204. Their totals are **602 and 1,454**: the ordinary point-product recipe is still 600/1,450, with two/four additional IFMAs for wider export quotient estimates. This cost is explicit. Outer spans and inverse branches must not be added as if they were disjoint dynamic work. No matched original consumer, CPU profile or clock result is supplied by these two decodes.

The offline reader checks 221 pins, four successful/one failed command and exact streams, the dependency-only repair, all 176 production identities, all 18 copied field originals/suffix recovery, the exact child copy, test names/counts, native flags and the two new decodes/binary hashes. It does not rerun builds, tests, old models or prior decoders. The local native host was confirmed as Ryzen 9 9950X, 16 physical/32 logical CPUs; the test does not assert exclusive host use or fixed worker affinity.

## Next measurement and unchanged proof comparison

Build actual original-source/fresh OFF/ON performance consumers with consistent release settings and independent changing-state output checks. Preserve this fixed arithmetic and charge every sign/normalization/export/inverse/borrowed-reference cost. Freeze complete G1/G2 full/tail and 1/16/32-worker calibration, nonregression, gain, allocation/resource and automatic PMU/profile quality rules before any preflight. Record all observations before offline work. A caller win still needs actual Window/core integration, verified SDK primary, all8+54/primary16, Gnark, portable and resource qualification before adoption. [Next stage](artifacts/next.md).

This stage adds no SDK request, proof, timing, profiler capture, RSS peak or minimum-RAM result. Model/fixture/qualifier storage is diagnostic. No AWS/Aeglos, huge-circuit, pressure/swap, mobile/ARM/TEE or concurrent-proof result is added. Production source 176, accepted SDK 6d83 and canonical 35,171 are unchanged.

The following proof timings are inherited from the [preceding mathematical report](../2026-11-08-late-centered176/README.md): latest primary plus seven explicitly older full-campaign rows. Supplied M5 reference timings use different hardware.

|Shape|Accepted local ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.002770|37.0|24.0|4.623×|2.999×|3.7|
|2x2|11.241463|59.0|30.0|5.248×|2.669×|5.9|
|4x2|15.933816|109.0|48.0|6.841×|3.012×|10.9|
|8x2|24.234311|189.0|74.0|7.799×|3.054×|18.9|
|12x2|33.855565|284.0|101.0|8.389×|2.983×|28.4|
|16x2|40.071563|335.0|124.0|8.360×|3.094×|33.5|
|24x2|65.917462|528.0|176.0|8.010×|2.670×|52.8|
|merge-24x1|65.305023|507.0|175.0|7.764×|2.680×|50.7|

Accepted local 1×2 remains **about 8 ms**: the same binary previously measured 7.915095/7.928660/7.995682 ms; latest 8.002770 ms. The variation's cause is unconfirmed, without an adopted regression. Merge54 remains inherited 162.240109 ms without a supplied M5 reference. The full all-eight verified CPU 10× goal remains active/unmet, **0/8**.

[Native kernel](artifacts/kernel.rs), [diagnostic callers](artifacts/callers.rs), [new qualifier tests](artifacts/native-tests.rs), [qualification/codegen](artifacts/qualification.json), [declared design](artifacts/design.md), [dependency repair](artifacts/link-repair.json) and [UTF8 evidence archive](artifacts/evidence.json.gz). Every archived {utf8,bytes,sha256} payload and decompression roundtrip is verified. Exact helpers, sources, raw receipts, plans/pins and both disassemblies remain. Unchanged field source copies, libraries/executables, keys/witness/proof payloads, private ledgers and external full papers/source bodies are excluded. Public prose trims only trailing blank lines; frozen archived bytes remain exact. Source/result copies are byte-exact.
