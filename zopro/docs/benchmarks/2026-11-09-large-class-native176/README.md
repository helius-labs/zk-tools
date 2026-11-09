# Oversized scalar-class splitting: native qualification

**The private implementation passes native correctness and fallback checks. Performance is unmeasured; production is unchanged.** It splits oversized repeated-scalar classes across existing point tiles, then combines their partial roots through the original complete affine kernel. This stage does not establish a faster complete query or proof.

Original grouping assigns up to 256 classes to a job, so one unusually large class remains serial inside that job. The retained [all-eight grouping audit](../2026-11-07-g1-large-coverage176/README.md) observed admitted A classes up to 15,072 points. Those populations do not identify the current proof's critical path. Primary A grouping declines, so this mechanism alone is not a route to the primary 3.7 ms target.

Only classes larger than the original indexed-reducer tile—2,048 points for G1 or 1,024 for G2—split into concurrent unchanged pair trees. A second pass retains 256 output classes per job, reading partial roots for split classes and original inputs for all other classes. It builds no duplicate input-order array and creates no task per singleton. It preserves both ordinary and prepared-point getters. The [Barretenberg batched-affine documentation](https://barretenberg.aztec.network/api/classbb_1_1_batched_affine_addition) also describes splitting long addition sequences among threads; this independently written implementation uses Zopro's existing tile geometry.

The native snapshot has 178 files: 175 accepted originals remain exact, one parent recovers through the recorded hook and module suffix, and two children contain the reducer and tests. Original field library1200, point formulas, grouping admission, scalar decomposition, windows, solver/key code, allocator and pool remain unchanged. The OFF and ON optimized cores use native/O3/one-CGU flags and the accepted bitmap reset. Release symbol checks find the candidate namespace only in ON and no test namespace in either. No downstream SDK is built here.

Before allocation, checked optional admission charges task descriptors, partial roots, one class-prefix array, vector overhead and the entire first-pass worker-scratch bound, conservatively on top of the old consumer forecast. First-pass scratch drops before the output pass starts. No-split, denied, overflow, disabled and generic paths retain the original reducer. This introduces no retained point table, expanded key, new pool or artificial global RAM limit. The forecast is not allocated-byte measurement, emitted stack peak, RSS or minimum RAM. Extra partial roots, inversion calls and a second parallel pass may outweigh the additional parallelism.

Three new focused tests pass; 127 inherited tests are filtered, not replayed.

|New checks|Coverage|
|---|---|
|Boundary fixtures|194 G1/G2 cases; empty, singleton, odd, tile and 256-class edges; identities, repeated/opposite points and cancellation|
|Independent results|Arkworks generator-weight sums from full-width coefficients; ordinary/prepared access and exactly one getter read per original input|
|Worker execution|All 53 actual workers across 1/4/16/32-worker pools requesting 512 KiB stacks|
|Concurrent callers|Four OS callers share one eight-worker pool; worker and concurrent phases total 162 curve cases|
|Resource and panic behavior|15 checked-cost/overflow/denial cases; denied admission invokes no getter; original fallback and intentional getter-panic recovery pass|
|Generic guard|NoASM/noIFMA core compiles with the candidate cfg present and native module excluded|

These counts describe constructed correctness fixtures, not unique witnesses, production proofs or throughput measurements. Ordinary and prepared calls reuse each fixture. The independent group oracle does not share the native addition formulas. Complete query/proof, ARM/mobile and pressure/swap behavior remain unqualified.

All five native Rust1.98.1/xtask commands succeed first execution. A separate Rust reader checks 376 pins, exact argv and stream hashes, source recovery, test populations, release identities and 18 scalar join-topology cases. The latter show that split and final reductions preserve n−1 generic joins per nonempty class; they do not price inversions or exceptions. Three reader-invoked nm checks and setup/review/publication helpers are outside the five-command workflow. All 176 production hashes and canonical 43,083-proof ledger remain unchanged. There are zero new SDK requests, Gnark exports, latency captures, PMU panels or profiles.

Next is a prospectively frozen complete-consumer comparison using the exact qualified ON core, fresh OFF and accepted REF, with relevant large-class coverage, unchanged primary controls, resources and automatic profiles. Any later adoption still needs coherent all-eight plus merge54 SDK/Gnark qualification. Native correctness alone selects no optimization.

The full CPU10× goal remains active and unmet, **0/8**. Accepted local 32-worker full-proof observations and supplied M5 references below are inherited from the latest unchanged SDK comparison. Cross-machine ratios do not establish equal hardware efficiency.

|Shape|Accepted CPU ms|M5 CPU ms /speedup|M5 Metal ms /speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.803404|37.0 /4.742×|24.0 /3.076×|3.7|
|2x2|10.940319|59.0 /5.393×|30.0 /2.742×|5.9|
|4x2|15.809849|109.0 /6.894×|48.0 /3.036×|10.9|
|8x2|24.043743|189.0 /7.861×|74.0 /3.078×|18.9|
|12x2|33.720355|284.0 /8.422×|101.0 /2.995×|28.4|
|16x2|39.753010|335.0 /8.427×|124.0 /3.119×|33.5|
|24x2|65.983137|528.0 /8.002×|176.0 /2.667×|52.8|
|merge-24x1|65.378103|507.0 /7.755×|175.0 /2.677×|50.7|

Accepted primary peak RSS remains 132.50 MiB; merge54 remains 162.189200 ms without a supplied M5 reference. No new AWS/Aeglos, phone/ARM, huge-circuit, pressure/swap, minimum-RAM or concurrent-throughput evidence is added.

[Design](artifacts/design.md), [native reducer](artifacts/large_class_tiles.rs), [tests](artifacts/large_class_tests.rs), [source recovery](artifacts/source-audit.json), [independent review](artifacts/review-result.json), [release identities](artifacts/identities.json), [receipts](artifacts/receipts.json), [decision](artifacts/decision.json), [continuation](artifacts/next.md), [sources and receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Executables/libraries, full unchanged source copies, keys/images/points/scalars/witnesses/proofs and the full ledger remain local. Private root: /home/ubuntu/src/zopro-perf-work/large-class-tiles176.
