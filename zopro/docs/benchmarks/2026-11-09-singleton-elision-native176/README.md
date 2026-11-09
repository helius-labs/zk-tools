# Elide singleton copies in dense scalar grouping

The private candidate writes singleton scalar classes directly into the existing dense output instead of copying them through point-reduction scratch. Empty classes write identity; repeated classes retain the original reduction arithmetic, exceptional cases and output order. **Native qualification and coherent SDK linking pass. No performance result or adoption is established.** Accepted production is unchanged.

The change adds no heap vector, key-derived table, retained point/scalar copy, execution plan or pool. In each original 256-output tile, requested point capacity changes from the sum of every input count to the sum of counts greater than one. That quantity cannot increase. The original two fixed 256-element metadata arrays, scratch reuse lifetime and conservative admission envelope remain. This is a requested-capacity argument, not a measured RSS or compiler-stack bound. [Design](artifacts/design.md), [implementation and tests](artifacts/group_singletons.rs).

This differs from the earlier sparse grouped-G2 view: later MSM consumers still receive the same dense point array, and grouping cutoffs/admission remain unchanged. It removes copies and conversions, not curve additions. Primary main queries can decline dense grouping under existing cutoffs, so this does not justify forecasting a primary gain or a twofold proof improvement. Full-request coverage and performance still need measurement.

## Qualification

Four native workflow commands and three SDK build/link commands succeed on their first execution. Two new focused tests pass; 125 inherited tests are filtered and not counted as new executions. The tests cover both curves, empty/singleton/mixed classes, odd and partial tiles around 256 outputs, identities/repeated/opposite points, prepared-point indices, exactly one getter read per original input, retained point length/capacity, and scratch reuse. Threaded tests exercise all 53 actual workers across 1/4/16/32-worker pools with requested 512 KiB stacks, plus getter-panic propagation before output length publication. Expected panic messages in the successful receipt are intentional.

Generic no-ASM/no-IFMA core compilation and a native release core build pass. Generic compilation is not ARM/device execution. Field source and qualified field libraries are reused without another field suite. The native 177-file snapshot has 175 unchanged originals, one reversibly edited parent and one child; all 176 original hashes recover. A separately written same-agent reader checks the inverse, production hashes, 193 native pin entries and four command/stream receipts. The SDK review checks 199 pins and three command/stream receipts, binding coherent core/prover/zolana libraries and the unchanged benchmark driver. This is not an external audit. [Source recovery](artifacts/source-audit.json), [native review](artifacts/qualification-review.json), [SDK review](artifacts/sdk-build-review.json).

No SDK preflight, proof, independent verifier, latency observation or profile has run for this candidate. No successful test/build stage was repeated. The first publication checker attempted to validate the checksum link before creating the checksum file; a separate finalizer repairs that ordering and checks all links, checksums and the archive round trip. Native and link results are unchanged. The [allocation lifetime diagnostic](../2026-11-09-proof-lifetime-census186/README.md) is separately closed and pushed; its allocator instrumentation is not included here. The rejected prefix/short composition retains all nine failed checks.

## Next comparison

Freeze a complete-SDK comparison against existing accepted and fresh accepted controls before any preflight or clock. Include all eight supplied shapes, merge54 and primary16, with unchanged worker placement/environment and automatic PMU/profiles. Bind every input/source/library/driver and the 39,608-proof freshness ledger; retain calibration, nonregression, memory, failed observations and independent Gnark checks. Full correctness/resource/admission/portable/concurrency qualification remains required before adoption. No favorable-shape promotion, grouping-threshold change, exclusion or unchanged retry is selected. [Continuation loop](artifacts/next.md).

## Inherited accepted CPU and Metal comparison

These are earlier accepted-build measurements under common fixed worker placement, not measurements of this candidate. M5 CPU/Metal values are supplied references on different hardware.

|Shape|Accepted local ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.852740|37 / 4.712×|24 / 3.056×|3.7|
|2x2|11.026811|59 / 5.351×|30 / 2.721×|5.9|
|4x2|15.829917|109 / 6.886×|48 / 3.032×|10.9|
|8x2|24.163935|189 / 7.822×|74 / 3.062×|18.9|
|12x2|34.087049|284 / 8.332×|101 / 2.963×|28.4|
|16x2|40.010922|335 / 8.373×|124 / 3.099×|33.5|
|24x2|66.255362|528 / 7.969×|176 / 2.656×|52.8|
|merge-24x1|65.178861|507 / 7.779×|175 / 2.685×|50.7|

The all-eight CPU 10× goal remains active and unmet, **0/8**. Historical accepted standard 1×2 at 7.915095 ms and the latest fixed placement 7.852740 ms remain valid; the unadopted compound's 7.753327 ms is not a new production record. This implementation adds zero proofs; the canonical independent inventory stays 39,608. No new AWS/Aeglos, phone/ARM, huge circuit, pressure/swap, minimum-RAM or throughput claim follows.

[Exact text evidence](artifacts/evidence.json.gz) retains helpers, edited parent/child, pins and every native/link receipt; unchanged source files, libraries/executables and private key/witness/proof payloads stay local. [Publication checks](artifacts/publication.json), [checksums](SHA256SUMS). Private root: `/home/ubuntu/src/zopro-perf-work/singleton-elision176`.
