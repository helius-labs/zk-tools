# Window batch bounds: native qualification

**Correctness and emitted-work qualification pass; performance is not measured.** The candidate uses bounds already checked by the window producer, avoiding repeated kernel checks without adding another scan. Production and accepted proof times are unchanged. This selects a separately frozen complete-caller comparison, not adoption.

## Mechanism and safety contract

The original G1/G2 Window.add indexes its affine array before publishing each pending batch entry. Queued entries reach the batch only through add. The array length and published batch remain fixed until execute returns. Two private unsafe kernel entries use that existing guarantee; the original arbitrary-batch APIs retain checked accesses. No additional validation pass, descriptor, point/key copy, table, execution plan, scratch buffer or pool is added. Original field expressions, queue policy, exceptions, grouping, widths and partitions remain.

The closed [once-validated G2 trial](../2026-11-06-g2-validated-capture176/README.md) added a batch scan and found no gain. Its observations remain closed. This changes the producer/consumer contract and includes G1, but does not establish that the scan caused the old loss. The new implementation needs fresh complete-caller and actual-window/full-proof evidence. Existing producer checks stay inside actual window execution; the synthetic fixture's immutable range-correct indices are constructed identically outside all arms.

Review recovers every accepted source file: 172 originals are exact; four original parents have reversible changes and four private child modules give 180 files. The original add bodies remain byte-exact, each has one batch-publication site, and inspected source contains no intervening affine-length mutation. These source invariants and finite tests are not a formal unsafe-code proof.

## Native checks

Eight native build/test/link commands succeed. ON passes all 129 core tests, including four new producer cases. OFF runs those four cases with 125 existing tests filtered. The four populations are reapplied across compiler arms, not counted as eight distinct cases. They cover signed/tail/reuse lengths 0 through 511, identities/doubling/opposites, queued and projective-overflow arrivals, rejection before publication and the original checked API. Independent window states also run on every worker of 1/4/16/32-worker pools with configured 512 KiB stacks. Generator-coefficient sums use the unchanged accepted curve arithmetic; they are not a fresh independent Arkworks implementation.

The original accepted field library is reused coherently and not rebuilt/retested. Both complete-caller drivers link with native targeting, O, fat LTO and one codegen unit; they have not been executed in this stage. No SDK/prover suite, independent Gnark proof, portable/device, pressure or concurrency-throughput qualification is claimed.

## Actual final consumers

These are static whole-function counts, not cycles, dynamic checks, measured spills or RAM peaks. ON G1 is inlined into G1Caller.run. Its comparison therefore uses the complete run envelope: OFF run plus its separate signed kernel, versus ON run. This envelope also includes the unsigned fallback. No attribute or forced inlining change is made.

| Final function | Arm | Instruction rows | IFMA sites | Bounds-panic sites | Vector stack-operand rows |
|---|---|---:|---:|---:|---:|
| G1 | OFF | 4425 | 800 | 24 | 286 |
| G1-caller | OFF | 4458 | 800 | 28 | 278 |
| G2 | OFF | 8590 | 1450 | 37 | 603 |
| inverse | OFF | 1139 | 500 | 0 | 67 |
| G1-caller | ON | 8596 | 1600 | 32 | 541 |
| G2 | ON | 8268 | 1450 | 2 | 606 |
| inverse | ON | 1139 | 500 | 0 | 67 |

The G1 envelope falls 8,883→8,596 rows and 52→32 bounds-panic sites, retaining 1,600 IFMA sites. G2 falls 8,590→8,268 rows and 37→2 bounds-panic sites, retaining 1,450 IFMA sites; its vector stack-operand rows increase 603→606. The shared inverse retains 1,139 rows and 500 IFMA sites. These observations support testing the fixed implementation; fewer checks do not prove a speedup. No allocation, peak RSS or minimum-RAM result is measured here.

## Preserved preparation and evidence

The first ON compile stopped on an inner documentation comment following the G1 function. Changing it to an ordinary comment repairs only the unfinished ON stage; successful OFF builds/tests are retained because that compile flag excludes the child. Three reviewer attempts then stopped on Rust symbol spelling or the actual G1 inlining boundary. All failures and sources remain. Symbol listings and the three already decoded OFF functions are reused; seven unique final functions are decoded exactly once in total. No successful native stage or performance capture is repeated.

[Qualification](artifacts/qualification.json), [source recovery](artifacts/source-audit.json), [fixed design](artifacts/design.md) and [preparation details](artifacts/preparation.md) describe scope and limitations. The UTF-8 archive includes changed children, reversible deltas, plans/pins, receipts, helper sources and disassembly. Its exact roundtrip and every member hash are checked. Dependencies, native executables/libraries, keys, images, points, scalars and witnesses are excluded. Local root: `/home/ubuntu/src/zopro-perf-work/window-batch-contract176`.

The selection review also built a navigation index of 698 direct benchmark reports, excluding embedded copies; its 79 language matches are hints, not authoritative pending states or successful candidates. No prior result is reopened merely because the index labels it.

## Accepted CPU and Metal comparison, inherited

These retained local 32-worker timings and supplied M5 references are unchanged, not measured here. All eight CPU10× targets remain unmet.

| Shape | Accepted CPU ms | M5 CPU ms | M5 Metal ms | CPU10× target ms |
|---|---:|---:|---:|---:|
| 1×2 | 7.928660 | 37 | 24 | 3.7 |
| 2×2 | 11.151932 | 59 | 30 | 5.9 |
| 4×2 | 16.086386 | 109 | 48 | 10.9 |
| 8×2 | 24.179453 | 189 | 74 | 18.9 |
| 12×2 | 33.884455 | 284 | 101 | 28.4 |
| 16×2 | 40.077381 | 335 | 124 | 33.5 |
| 24×2 | 65.850777 | 528 | 176 | 52.8 |
| merge24 | 65.060937 | 507 | 175 | 50.7 |

Merge54 remains 161.883899 ms without a supplied M5 reference. Accepted source176/SDK6d83 and the 30,846-proof ledger remain unchanged. The inherited 1×2 RSS is 132.75 MiB, not a new or minimum-RAM result. Zero new SDK/Gnark proofs, latency/profile captures or AWS/Aeglos/device measurements. No cloud resources are created.

Next: freeze and execute the complete changing-generator caller comparison at all four sizes and 1/16/32 workers, with both controls, allocation/resource checks and automatic profiles. Only a qualified result selects all-eight plus merge54 actual SDK, independent proof, portable and resource qualification. The full CPU10× goal remains active and unmet (0/8).
