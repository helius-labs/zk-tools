# Retained-capacity SIMD tails: native qualification

**The new original-tree tail path passes native correctness qualification. Speed is unmeasured; production is unchanged.** It sends an incomplete final batch through SIMD only when the existing pair-index and selected record buffers already have sufficient capacity. Otherwise it retains the original scalar tail. No table, new requested buffer capacity, key copy, solver plan, pool or RAM cap is added.

The original `reduce_layer` still selects its ordinary distinct-X pairs, prefix policy and tree topology. The private IFMA8 path appends copies of the first ordinary pair into spare descriptor capacity, computes the unchanged vector formula and inversion, scatters only real outputs, then truncates descriptors before the original stable compaction and complete exceptional formulas. A nonzero dummy denominator is required. Reverse order ensures duplicated inputs are read before their real earlier group writes; all reads precede writes when there is only one group. Original scalar arithmetic remains the capacity/ISA fallback.

This differs from the rejected [borrowed-bucket fill tree](../2026-11-07-borrowed-bucket-tree-pilot176/README.md), which changed routing and tree ownership, and [cold-tail outlining](../2026-10-30-g2-cold-tail175/README.md), which preserved scalar-tail arithmetic. Neither negative is rerun or relabeled. The current experiment changes the existing tree remainder without changing point routing, grouping, window geometry, streaming queues, field routines, solver, FFT or task scheduling.

All **130 native core tests** pass, including three new tests. There are **6,960 cases and 797,472 compacted point comparisons against independent Arkworks addition** across both curves. The actual reducer also matches an exact separately named original body over every input/output slot and descriptor/count result. Cases cover all remainder residues, sizes 0..64 and 22 larger boundaries through 1,025 pairs, fresh/partial/sufficient capacities, both inverse modes, identities, equal/opposite inputs and ragged buckets. Capacity-rich cases preserve all three vector capacities. Inputs remain unchanged.

**114 complete bucket trees** pass across 53 actual worker executions on 1/4/16/32-worker Rayon pools requesting 512 KiB stacks and four OS callers sharing an eight-worker pool. The changing-layer test observes 741 padded calls. These are correctness/concurrency executions, not latency, affinity, stack-peak or throughput measurements. The `no_ifma` core compile guard passes with the original native field library; it does not establish full generic-field, ARM or phone execution.

All 176 production files remain exact. The private snapshot has 175 byte-identical originals, one reversible parent and one test child. OFF compiles the exact original reducer body. The field library `1200` and its qualification are inherited by identity. Both fresh SDKs link coherently with original native/O3/fat-LTO/one-CGU consumer settings and the unchanged fixed-placement driver/support. No SDK preflight, timing, profile, new proof or Gnark call has run in this phase.

|Arm|SDK SHA256|Core SHA256|G1 / G2 reducer bytes in actual SDK|
|---|---|---|---:|
|OFF|`c6f0e32fcccb17d0bbb9bb8f8f5e65a11ed8dda653ac5a07c8297edb956b7262`|`907aeefd5c2420a33ae2334715234b48d58a4f1242df970b5656104d7f18d26c`|23599 / 81580|
|ON|`203086b54aed8184ca5dccfd2abb2d8f458bde2d16b755c64a6fe7c423ea51b8`|`7792ee244d256cb84a8e1a5de67dca9568e41c23acda3910f9944ee47dc7c863`|24092 / 81908|


The earlier accepted-source metadata records primary G1 tree 238,475 ordinary pairs/2,867 scalar-tail pairs and G2 tree 95,094/9,838, including 1,689 G2 layers below eight pairs. It is reused, not recaptured. A count-derived optimistic padding ledger is in the reader result, including dummy joins. It does not observe this candidate's capacity eligibility or price extra SIMD arithmetic, packing, branches, inversions or compiler effects. This is no forecast of a twofold proof gain.

One source-reader execution fails because it locates the earlier OFF copy's comment while checking candidate truncation. A separate reader changes only that comment ownership lookup and passes. Successful native tests/builds and the compile guard are retained. One direct preparer panics on a missing optional environment for symbol-list steps; the next preparer preserves the completed reader. The initial SDK plan then fails schema parsing on null environments before executing any SDK command. Its v2 plan defaults only those two environment maps to empty. All original versions and failures are preserved; no runtime code or timing gate changes for these repairs. Direct setup/compiler invocations do not all have xtask receipts.

There are **16 executed workflow commands: 15 successes and one retained reader failure**. Native builds/tests contribute four successes, the compile-guard/initial-reader phase one success/one failure, corrected reader build/execution two successes, and coherent SDK links/symbol lists eight successes. Closing review checks 802 pin occurrences and all argv/exit/stream hashes. Publication/checkpoint are separate. No successful native test, build, decoder, proof or recording is rerun.

Narrow primary-source searches revisited [SRE](https://arxiv.org/abs/2610.27452), [Hrabar's thesis](https://uwspace.uwaterloo.ca/items/43cc12b9-cb03-42bd-a01c-4b0ed5e12403) and [generalized precomputed buckets](https://eprint.iacr.org/2024/750). Those approaches already have local studies; no external implementation or reported speedup is transferred here. The selected change follows actual scalar-tail dispatch and retained capacity.

Accepted full-proof timings below are inherited from the [last matched SDK trial](../2026-11-10-simd-bucket-sdk176/README.md), on Ryzen 9950X with 16 physical/32 logical CPUs. Supplied M5 references use different hardware and are not remeasured. These are not timings of this candidate.

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10x target ms|
|---|---:|---:|---:|---:|
|1x2|7.770239|37 / 4.762x|24 / 3.089x|3.7|
|2x2|10.966498|59 / 5.380x|30 / 2.736x|5.9|
|4x2|15.761756|109 / 6.915x|48 / 3.045x|10.9|
|8x2|23.933998|189 / 7.897x|74 / 3.092x|18.9|
|12x2|33.607895|284 / 8.450x|101 / 3.005x|28.4|
|16x2|39.794609|335 / 8.418x|124 / 3.116x|33.5|
|24x2|66.142308|528 / 7.983x|176 / 2.661x|52.8|
|merge-24x1|65.271702|507 / 7.768x|175 / 2.681x|50.7|


Accepted primary remains **7.770239 ms**, already below 8 ms. Primary at 16 workers is 9.013311 ms and merge54 is 162.486028 ms without a supplied M5 reference. The original CPU 10× performance goal remains **active and unmet, 0/8**. The verified proof inventory remains **44,800**; no new independently exported proof is added. No new AWS/Aeglos, huge250/4999, pressure/swap/minimum-RAM, mobile/TEE or concurrent-throughput claim is made.

Next is a frozen complete-SDK accepted REF/fresh OFF/ON comparison over ALL8+54 and primary at 1/16/32 workers, charging every padding and fallback cost with automatic profiles/resources and independent fresh Gnark proofs. Native correctness is not adoption. [Continuation and autoresearch loop](artifacts/next.md).

[Qualification reader](artifacts/review-result.json), [source edits](artifacts/source-edits.json), [new tests](artifacts/tests.rs), [design](artifacts/design.md), [identities](artifacts/identities.json), [receipts](artifacts/receipts.json), [inherited CPU/Metal data](artifacts/fresh-comparison.json), [decision](artifacts/decision.json), [evidence archive](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). The UTF8 archive retains authored source/versions, plans/pins, streams and commands. Unchanged source snapshots, executables/libraries, private inputs/proofs, raw perf and full proof ledgers are excluded.
