# Indexed wide-band G2: native correctness and resource qualification

**The private implementation passes five focused native tests, a generic build guard, and a release build. Performance has not been measured.** Production remains unchanged; accepted local 1×2 full-proof latency is **7.857170 ms** under the retained fixed CPU placement. The all-eight CPU10× goal remains unmet.

The candidate implements the fixed [structural proposal](../2026-11-09-indexed-wide-bands176/README.md), keeping the original field library and complete addition/tree/weighted kernels. It changes G2 accumulation and scheduling: nine width14 ordinary windows build signed point indices, then 256-bucket leaves reduce four component/partition classes jointly. High-part sums receive φ after merging. Each leaf returns a weighted sum and an ordinary total; the existing outer weighted identity combines those pairs. The tenth window uses the existing signed indexed carry path.

No full 8,192-bucket coordinate array, persistent point table, setup cache, field rewrite or additional production thread pool is introduced. Index construction, signed point copies, extra projective doubling, and the construction-to-leaf barrier are real costs still to be measured. The earlier count reduction is not a latency prediction. [Design](artifacts/design.md), [source audit](artifacts/source-audit.json), [review](artifacts/review.json).

## Admission and memory scope

Two existing analysis reservations cover the new temporary buffers. The first reserves all nine retained indices, their construction scratch, leading-window parents and result vectors. After parallel index construction releases count/position scratch, the starts determine the largest actual cell. A second reservation covers the largest nonforking leaf scratch times the existing worker count. Either denial releases candidate buffers and continues the original code with its already-split scalars. There is no second split or new fixed RAM cap.

Joint reduction buffers drop before local merging; merge/output buffers drop before weighting. The forecast includes their actual overlap, capacity growth, vector overheads, conservative inline structures and leading-window scratch. The original process-wide policy remains in use. Skewed cells receive larger charges; they are not discarded behind a guessed cell-size cutoff.

For primary's retained model population, n=30,089 and largest cell=2,359, the checked x86_64 requested-capacity forecast is:

|Workers|Index/construction/parent charge B|Leaf charge B|Total candidate charge B|
|---:|---:|---:|---:|
|1|5,347,164|893,696|6,240,860|
|16|5,347,164|14,299,136|19,646,300|
|32|5,347,164|28,598,272|33,945,436|

The 32-worker total is about **32.4 MiB**. These are conservative requested-capacity charges, **not measured allocations, RSS, full-process peak, minimum RAM, or phone/pressure qualification**. Source inputs, keys, other prover phases, runtime internals, allocator state and worker stacks are outside this kernel-buffer subtotal. An adversarial all-in-one-cell bound can be much larger: n=65,535 with 32 workers forecasts 1,338,130,156 bytes and remains subject to admission/fallback. This is scratch, not a retained lookup table.

## Validation

All six serialized Rust1.98.1/xtask commands succeed on their first native-workflow execution. Five new tests cover:

- Signed four-class weighted sums and totals against known-generator scalar algebra, including identities, equal/opposite points, cancellations, lane/power boundaries, and nonzero index offsets.
- Hot cells up to 4,097 rows, beyond primary's 2,359-row modeled maximum, with deep reduction and unchanged input buffers.
- Complete raw and production-grouped query results at 1/4/16/32 workers with 512 KiB test stacks. The grouped fixture has 1,051 distinct scalars; route318 is asserted in both query branches.
- Both forced admission denials through the actual raw entry point: original fallback produces the expected result and leaves exactly once-split words. Admission succeeds afterward.
- Checked requested-capacity/overflow behavior over six sizes, four worker counts and three multiplicities. The separate reader recomposes all 72 emitted rows from the original type layouts and formulas.

The generic noASM/noIFMA configuration compiles; this is not an ARM or mobile execution test. The optimized native core builds with original field1200. All 176 production files remain exact. The private snapshot has 178 source entries: 172 original entries exact, four modified parents reversible to their originals, and two new children. The hook is after the original split in hb_raw; its original remainder is byte-exact. No original arithmetic kernel is edited.

The separate same-agent reader verifies 201 pins, all six command/exit/stream receipts, the five required test passes, source recovery, unchanged production/field identities and the 72 capacity rows. Known-generator tests are not a substitute for the actual-key independent MSM oracle or Gnark proof verification required later.

Two tooling errors are retained: source preparation initially tried to decode an existing binary fixture as UTF8; bytewise comparison repaired that before any native build. The initial offline reader compared xtask's timeout-wrapped command directly with its unwrapped protocol; it was repaired to validate the wrapper and command separately. Original helper sources/failure records are archived. No native source, successful build or test was repeated to repair either issue.

There is no new latency/profile/SDK/Gnark/proof result and no adoption. The canonical verified-proof ledger remains 40,154. Next is a frozen complete-query comparison that includes all construction and arithmetic, with accepted-source REF and fresh coherent OFF/ON controls, actual-key oracle preflight, automatic profiles, and resource/nonregression checks. Full all-eight plus merge54 SDK/Gnark/resource/portable/concurrency validation remains required before adoption.

## Accepted CPU and Metal comparison, inherited

These are the accepted local Ryzen9950X full-proof measurements from the completed SDK campaign, using 32 workers and common fixed placement. They are not new candidate, AWS or M5 measurements; fixed placement is not a production default. [Accepted campaign](../2026-11-09-singleton-elision-sdk176/README.md).

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.857170|37|24|4.709×|3.055×|3.7|
|2x2|11.052363|59|30|5.338×|2.714×|5.9|
|4x2|15.952018|109|48|6.833×|3.009×|10.9|
|8x2|24.080322|189|74|7.849×|3.073×|18.9|
|12x2|34.012417|284|101|8.350×|2.970×|28.4|
|16x2|40.128912|335|124|8.348×|3.090×|33.5|
|24x2|66.348981|528|176|7.958×|2.653×|52.8|
|merge-24x1|65.829291|507|175|7.702×|2.658×|50.7|

Goal active and unmet, **0/8**. Earlier accepted 7.915095 ms remains valid; the separate 8.075143 ms campaign did not establish a code regression. No new AWS/Aeglos, huge-circuit, pressure, swap, minimum-RAM or throughput claim follows.

[Continuation](artifacts/next.md), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS).
