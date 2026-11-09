# Joint X/delta caller qualification

**The new arithmetic passes ordinary G1/G2 caller qualification. Speed remains unmeasured.** The actual forward/reverse G1 stream and owned/borrowed G2 prefix paths match independent Arkworks additions across **842,568 point comparisons**. The accepted local full-proof 1×2 baseline remains **7.857170 ms**. No optimization is adopted here.

This extends the [joint endpoint qualification](../2026-11-09-joint-coordinate-delta176/README.md), which computes X and an opaque delta for Y together from the squared slope. It reuses the exact qualified checked field library. Original multiply/cancellation code and generic lazy lane helpers remain unchanged. See the [design](artifacts/design.md), [source audit](artifacts/source-audit.json), and [separate review](artifacts/review.json).

## Integration and memory

G1 retains its five-field, 1,600-byte, 64-byte-aligned record. With the new selector, its third field holds canonical Bx for `FAST=true`, or the original X-sum for `FAST=false`. Both record stores initialize that field: ordinary vector push and the existing uninitialized-pointer store. The reverse pass computes the joint X/delta pair and consumes the opaque delta directly in the Y product.

G2 retains its original square, prefix records, inversion, and final coordinate subtraction. Both owned and signed borrowed paths use the new finish for `FAST=true`. The `FAST=false` formulas, scalar tails and exception scheduling retain their original arithmetic. Opposite guards retain the original OFF statements verbatim.

No kernel record field, heap buffer, lookup table, copied key, retained plan, pool or RAM cap is added. Qualification facades allocate reference/owned vectors for test convenience; this does **not** establish an allocation-free complete API. Separate diagnostic TLS counters prove path execution and must be omitted from performance artifacts. Record payload and requested worker-stack size are not RSS, stack-peak or minimum-RAM measurements.

## Independent caller checks

Four serialized Rust 1.98.1/xtask commands pass on their first execution: optimized checked core build, external oracle link, three new unfiltered tests, and generic noASM/noIFMA core compilation. The already-qualified native and generic field libraries are reused unchanged. No inherited test suite is replayed.

|New population|Independent point comparisons|Scope|
|---|---:|---|
|Changing packets|293,040|All eight scalar tails, signs, both G1 stores, owned/borrowed paths, both inverse modes|
|`FAST=false` controls|40,728|Original formulas; no candidate route executions|
|Small-stack workers|508,800|53 actual worker executions across 1/4/16/32-worker pools|
|Total|842,568|Ordinary stream inputs|

Arkworks constructs full-width-scalar points and computes expected additions independently. The fixture reuses 64 points per curve; comparison counts are not unique-point or proof counts. Inputs and signs change between rounds, including zero-delta cases `P + (−2P) = −P`. Every native call checks its exact route increment and output points. The changing-packet population records 17,856 G1, 4,464 borrowed G2, and 4,464 owned G2 candidate full-vector groups. Input immutability and stable same-sized scratch capacity are checked.

Rayon `broadcast` makes every actual worker execute the caller checks, using a requested 512 KiB stack per worker. The generic build checks selector guards only; it is not ARM or mobile execution.

All stream inputs satisfy the existing nonidentity, distinct-X and distinct-bucket preconditions. These checks cover the full ordinary packet prefix/inversion/finish/tail flow. **They do not qualify complete exceptional MSM dispatch, a complete key query, or a proof.** Exceptional pairs must continue through their existing scheduling paths.

## Review and next step

The separate reader verifies 382 source/artifact pins, all four command/exit/stream receipts, source recovery, test names and unfiltered summaries. It independently recomputes population and route cardinalities. The private snapshot has 178 source entries, with 170 exact accepted originals; reversing five core-parent edits and removing the qualified field suffix recovers all 176 accepted sources. All production files remain exact.

The reader initially compared test-name arrays in declaration order, while the runner sorted passed names. The [repair record](artifacts/reader-repair.json) retains that failure and the original reader. Sorting both vectors fixes the offline comparison while preserving exact count and unfiltered checks. Scientific sources, pins, commands and successful native results are unchanged and not repeated.

Correctness does not establish performance. Earlier checked endpoint codegen was mixed: G1 grew from 359 to 395 static instruction rows, while G2 fell from 987 to 919; IFMA site counts were unchanged. G1 requires an extra correction after its wider-delta product. No favorable curve subset or timing run is selected by these static numbers.

Next, prepare and qualify normal release artifacts, inspect actual complete consumer code, and decide whether the fixed proposal warrants a prospectively frozen matched caller trial. Accepted reference plus fresh OFF/ON controls must expose compiler/build differences. Full actual MSM exception/query checks and all-eight plus merge54 SDK/Gnark/resource/portable/concurrency validation remain necessary before adoption. [Continuation](artifacts/next.md).

## Accepted CPU and Metal comparison, inherited

The following results are unchanged local Ryzen 9950X full-proof measurements with 32 workers and common fixed placement, compared with the supplied M5 references on different hardware. They are not caller timings or new measurements.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.857170|37|24|4.709×|3.055×|3.7|
|2x2|11.052363|59|30|5.338×|2.714×|5.9|
|4x2|15.952018|109|48|6.833×|3.009×|10.9|
|8x2|24.080322|189|74|7.849×|3.073×|18.9|
|12x2|34.012417|284|101|8.350×|2.970×|28.4|
|16x2|40.128912|335|124|8.348×|3.090×|33.5|
|24x2|66.348981|528|176|7.958×|2.653×|52.8|
|merge-24x1|65.829291|507|175|7.702×|2.658×|50.7|

Production176/SDK6d83 and the 40,154-verified-proof ledger remain unchanged. Goal active and unmet, 0/8. No new timing, profile, SDK request, proof, Gnark export, AWS/Aeglos, mobile/ARM, huge-circuit, memory-pressure, swap, minimum-RAM or throughput result is added. No job or owned cloud resource remains live.

[Source/receipt archive](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS).
