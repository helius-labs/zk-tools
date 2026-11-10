# Bounded queue trees: first native qualification

**The private implementation passes curve, concurrency, fallback and capacity qualification. Performance is unmeasured and production is unchanged.** It replaces serial completion of a bounded collision queue with per-destination pair trees and one root attachment per bucket. This is the first native implementation of the [reviewed count model](../2026-11-10-ring-queue-tree176/README.md), not a timing retry.

Before native preparation or builds, the new [design](artifacts/design.md) explicitly permits this correctness/resource stage despite that model's failed componentwise screen. The old result remains **FAIL 2/14**: fused-product events increase 0.05095%/0.03146%, while inverse calls fall 783/1,252 and the worst modeled queue dependency changes from 199 serial attachments to eight tree levels plus one attachment. The exchange needs actual caller measurements. Neither the old screen nor its observations is rerun or relabeled. The previous complete-affine ring's [34% primary32 query slowdown and higher RSS](../2026-11-09-ring-affine-query176/README.md) also remain negative evidence. No native speedup follows from these counts.

The implementation sorts at most 199 local indices by destination, consumes signs while loading the original complete grouped affine reducer, and classifies root attachments for identity, doubling and cancellation before ordinary borrowed G2 arithmetic. Pending work completes first. Original batch200/queue199, unit phases, recoder/table, nineteen positions, two partitions, leading window, axes and weighting remain unchanged. The existing Window fields and original field/curve kernels are retained. Additional roots, borrowed attachment descriptors and fixed grouping metadata are explicit working storage. No retained key/point table, expanded key, solver plan, production pool, architectural memory cap, unsafe arithmetic or C/ASM rewrite is introduced.

Seven focused native tests pass on their first executions:

|Coverage|Evidence|
|---|---|
|Independent curve oracle|150 fixtures and 29,850 affine outputs against Arkworks; signs, identities, repeated/opposite points, cancellation, pending updates and queue lengths 0–199|
|Workers and shared pool|All 53 workers across 1/4/16/32-worker pools; 12 calls from four OS callers sharing eight workers; requested 512 KiB stacks|
|Changed phase/full-query behavior|Overflow and long collisions; complete 1,024/2,051-point queries at 1/4/16/32 workers with generator-weight oracles; inputs preserved|
|Capacity and fallback|20 forecast rows, arithmetic-overflow and denied-Option/mismatched-input fallback checks|
|Generic guard|NoASM/noIFMA core compiles with the selector present and native module excluded|

Three tests are new; four inherited tests are reapplied because their implementation changed. Unrelated tests are filtered from the 137-test binary. Unchanged field, table and model tests are not replayed. Worker oracle reuse is repeated execution coverage, not additional distinct mathematical fixtures or proofs. Generic compilation is not phone/ARM execution, and a requested stack size is not a measured stack peak.

The native source snapshot has 178 files: 171 accepted originals are exact, five private parent edits reverse to their original bytes, one new ring child and its inherited integer-table child are added. All 176 accepted production identities remain unchanged. The old ring's tail and table retain exact bytes; only completion, its associated capacity charge and tests differ. ON/OFF release cores use native/O3/one-CGU flags and the unchanged original field library1200. No downstream SDK is built. Read-only symbol checks find one ring-completion symbol and 71 ring-namespace lines in ON, none in OFF; these counts do not measure runtime or code size.

The grouped reducer has GROUP=256, so at most 199 output groups form one parallel chunk. Source inspection checks Rayon's minimum split length, which forbids splitting this length-one iterator. This supports charging the additional scratch per worker while its Window is live; it is not an arbitrary parallel-allocator or device proof. A separate reader checks source recovery, exact inherited fragments, resource composition, all command/stream receipts, oracle markers and routing. It shares the source formulas and qualified dependencies; it is not a second independent allocator implementation.

The checked optional forecast adds **99,728 bytes per worker**, conservatively summing roots, descriptors, grouping scratch and known fixed arrays even where lifetimes do not overlap. The original forecast still charges all suspended ring parents, leading-window parents and codes independently of worker count. For 30,089 primary rows:

|Workers|New checked optional capacity bytes|
|---:|---:|
|1|49586276|
|4|80956964|
|16|206439716|
|32|373750052|

The 32-worker forecast is about 356.44 MiB and includes conservative overcharges inherited from the leading-window model. It is not allocated-byte measurement, RSS, minimum RAM or a compiler-stack bound. Admission checks the expanded forecast before allocating codes/buckets; unsupported, denied and overflowing paths preserve the original fallback. Actual pressure, RSS and device qualification are still required before any adoption.

All nine native workflow commands succeed once. Initial offline review compiles then stops on a leading-comment boundary mismatch; its repaired source checks the complete inserted fragment and passes without rebuilding or retesting the candidate. Across native and reader workflows, **13 commands comprise 12 successes and one retained reader failure**. The separate source/receipt review checks 382 native pins; terminal publication verifies all 399 repaired-reader pins and all 13 command/stream receipts. A preparer-only resource-log parser initially finds 19/20 rows because the first marker follows a test-name prefix; parsing from the marker fixes it before any native workflow. Failed sources, initial plans/pins and failed reader logs remain archived. Setup helpers have transcript evidence rather than complete workflow receipts. The first publisher also checks the checksum link before creating its file; its retained partial output and an ordering-only repair do not rerun scientific work.

This stage creates zero SDK requests, Gnark exports, timing captures or profiles. Canonical proof inventory remains **44,265**. There is no new AWS/Aeglos, mobile/ARM/TEE, 250/4999, pressure/swap, minimum-RAM or concurrent-throughput result. The next stage is a prospectively frozen complete-query comparison with accepted REF and fresh OFF, including grouping/admission/recoding, all windows/merges/weighting, normalization, allocation/drop, resources and automatic profiles. Native correctness alone does not select production.

The following accepted local 32-worker full-proof observations and supplied historical M5 references are inherited from the [latest SDK campaign](../2026-11-10-frontend-overlap176/README.md), not measured by this native qualification.

|Shape|Accepted CPU ms|M5 CPU ms /speedup|M5 Metal ms /speedup|CPU 10x target ms|
|---|---:|---:|---:|---:|
|1x2|7.783594|37 /4.754x|24 /3.083x|3.7|
|2x2|10.932731|59 /5.397x|30 /2.744x|5.9|
|4x2|15.757088|109 /6.918x|48 /3.046x|10.9|
|8x2|23.973295|189 /7.884x|74 /3.087x|18.9|
|12x2|33.827269|284 /8.396x|101 /2.986x|28.4|
|16x2|39.900917|335 /8.396x|124 /3.108x|33.5|
|24x2|66.041083|528 /7.995x|176 /2.665x|52.8|
|merge-24x1|65.717994|507 /7.715x|175 /2.663x|50.7|

The full verified CPU 10x goal remains active and unmet, **0/8**. Primary16 is 9.016864 ms and merge54 is 162.170158 ms without a supplied M5 reference. Inherited primary32 RSS 111.25 MiB belongs to the short 31-request campaign; older 133.50 MiB observations remain valid in a different campaign. No memory saving is inferred.

[Native review](artifacts/review.json), [decision](artifacts/decision.json), [13 workflow receipts](artifacts/receipts.json), [completion kernel](artifacts/queue-complete.rs), [Arkworks/worker tests](artifacts/queue-tests.rs), [extra capacity](artifacts/extra-workspace.rs), [source audit](artifacts/source-audit.json), [preserved failures](artifacts/failures.md), [continuation/autoresearch loop](artifacts/next.md), [source/receipt archive](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Executables/libraries, unchanged snapshots/table payloads, private keys/points/scalars/witnesses/proofs and full ledgers are excluded.
