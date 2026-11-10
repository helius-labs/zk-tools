# Parallel roots before weighted SIMD folds

**The new integration passes native and SDK-link qualification. Performance is unmeasured; production remains unchanged.** It restores one root-building work item per 256-bucket group, then folds up to eight groups at a time using the same qualified double-add arithmetic. The previous grouped-root implementation's [failed complete-proof result](../2026-11-10-weighted-double-sdk176/README.md) remains closed.

At 2,048/4,096/16,384 buckets, root work items increase from 1/2/8 to 8/16/64; fold work items remain 1/2/8. A completed parallel root pass precedes the fold pass. This exposes more work; it does not establish simultaneous worker use, prove task starvation caused the old result, or predict a twofold gain. Sibling windows may already supply sufficient parallelism.

The root array holds nine affine points per group: 576 bytes per group plus the existing vector forecast's 64-byte allowance. Its forecast is 4,672/9,280/36,928 bytes at those three sizes. Existing `Reservation::analysis` checks the entire array cost before candidate allocation/work and holds the token through the final fold. Overflow, denied admission, disabled policy, short inputs and G2 retain the accepted original route. The actual denied-entry test uses 2,048 power-of-two buckets; it does not newly qualify arbitrary non-power-of-two input on the original fallback.

The original forecast's 6,176 logical extra bytes per G1 leaf remains for fixed fold objects. The new root array is separately charged; it is not hidden in a per-worker maximum while several window parents are suspended. Each root job reuses the original 511-slot tree and scratch, released before the fold barrier. The original `roots()` and double-add kernel are byte-exact; weighted and unweighted outputs and final offset corrections remain preserved. There is no retained key table, expanded plan, new production pool or architectural RAM cap. Logical forecasts are not emitted stack, RSS or minimum-RAM bounds.

Full timing must include the extra allocation, initialization, root stores/loads, second parallel pass, drop and admission. The admission check reads current OS headroom/policy on each weighted call; its filesystem/lock cost is not assumed free. No syscall reduction is claimed. This local weighted path alone cannot establish the required full-proof improvement.

Four serialized native commands pass on first execution: ON release core, test compilation, all **131 unfiltered core tests**, and portable-disabled compilation. The new integration checks the inherited 175-case weighted/sum family against independent Arkworks oracles, sparse/identity/opposite points, long tails, 53 actual workers across 1/4/16/32 pools requesting 512 KiB stacks, and 12 concurrent calls sharing an eight-worker pool. The additional test checks phase ordering, root/fold work counts, overflow, actual zero-budget entry fallback and later readmission. Standalone old arithmetic qualification, field tests and native OFF are inherited without replay. Portable compilation is not ARM/mobile execution or a server-throughput benchmark.

Four subsequent commands coherently link the new ON prover, Zolana and SDK and inspect final symbols. All eight scientific commands succeed once. The final SDK contains weighted-double symbols and no test route counters. Dynamic SDK execution is not yet measured. Accepted REF and qualified OFF are reused byte-exact, original field `1200` and all accepted cfgs including dense busy reset remain unchanged. The private snapshot has 174 exact originals, two reversibly hooked parents and three children; reversing the hooks recovers all 176 production hashes.

The native and SDK readers independently check command argv, terminal receipts, stream hashes, 201/210 pin occurrences, source recovery, roots/kernel equality, test markers, coherent link inputs and exact final consumer identities. They are separate readers by the same agent, not an external cryptographic audit. No SDK request, preflight, proof or Gnark export is added; the checked canonical inventory remains **48,721**.

Preparation exception: the first SDK plan selector matches names ending in `-ON` and misses the `-ON-symbols` command. It stops on a three-versus-four assertion before writing a plan. A subsequent xtask invocation has no plan and starts no child. The selector is repaired, original source/streams retained, and the four-command plan then executes once. Native sources, mathematics and successful build/test commands are not replayed.

Next is a prospectively frozen complete-SDK ALL8 plus merge54 comparison at 32 workers, primary also at 1/16, with exact REF/OFF/ON, independent fresh Gnark verification, automatic profiles, resource guards and both raw readers fixed before preflight. It must charge the complete caller and interaction with sibling queries. No component, favorable worker subset or build qualification completes the goal.

The following are the previous complete campaign's fresh accepted REF observations, unchanged here. Supplied M5 CPU/Metal are historical measurements on different hardware and were not remeasured. There is no new candidate timing column.

| Shape | Accepted REF ms | M5 CPU ms / speedup | M5 Metal ms / speedup | CPU10× limit ms |
|---|---:|---:|---:|---:|
|1x2|7.843523|37 / 4.717×|24 / 3.060×|3.7|
|2x2|10.999178|59 / 5.364×|30 / 2.727×|5.9|
|4x2|15.904657|109 / 6.853×|48 / 3.018×|10.9|
|8x2|24.339150|189 / 7.765×|74 / 3.040×|18.9|
|12x2|34.042084|284 / 8.343×|101 / 2.967×|28.4|
|16x2|40.074491|335 / 8.359×|124 / 3.094×|33.5|
|24x2|66.240170|528 / 7.971×|176 / 2.657×|52.8|
|merge-24x1|65.627663|507 / 7.725×|175 / 2.667×|50.7|

Accepted primary remains 7.843523 ms /132.5 MiB in that campaign; primary16 is 9.200161 ms and merge54 163.098343 ms /1905.28 MiB without an M5 reference. The CPU AVX 10× ALL8 goal remains active and unmet, **0/8**. No AWS/Aeglos, ARM/phone/TEE, 250/4999, pressure/swap, minimum-RAM or concurrent-proof-throughput result is added; no cloud resource is created.

[Native reader](artifacts/review-result.json), [SDK reader](artifacts/sdk-review-result.json), [consumer identities](artifacts/identities.json), [source and exact receipts](artifacts/evidence.tar.gz), [archive member hashes](artifacts/public-members.json), [checksums](SHA256SUMS). The archive contains the changed implementation/tests, reversible parents, design, next-agent autoresearch loop, plans/pins and every native/link receipt. Native executables/libraries, unchanged full trees, key/request/witness/proof payloads and the full canonical ledger remain local and hash-bound.
