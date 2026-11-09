# Borrowed-window busy bitmap completion

The private G1/G2 streaming change passes its targeted native qualification. It clears the existing busy bitmap in one pass when the number of bitmap words is no greater than the number of pending points; otherwise it retains the original indexed loop. **Performance is unmeasured, and production is unchanged.** The next stage is a matched complete-query/SDK comparison.

In both borrowed Windows, setting a busy bit appends an ordinary addition to the current pending batch. Queued collisions do not set bits. The batch completes before clearing or draining, so every set bit belongs to a completed pending destination. This permits a whole-bitmap reset under the existing ownership invariant. The chosen rule charges one branch per nonempty batch and avoids scanning a large table for a short sparse batch. The policy is fixed by these source schedules, without a tuning sweep.

For a full 200-point batch with 2048 buckets, the source reset changes from 200 indexed zero assignments to 32 word assignments; for 352 points and 4096 buckets, 352 becomes 64. These are source work counts, not dynamic store instructions, measured time or a full-proof speed estimate. Compiler lowering, vector stores, code size and a possible memset call remain costs to measure.

Arithmetic, inversion count, bucket geometry, task grain, queue/drain/exception order, configuration lifetime, allocation capacity and workspace forecasts remain original. There is no new buffer, point/key copy, retained plan/table, thread pool or memory cap. Only two private original files change behind `zopro_dense_busy_reset`; reversing their loop edits and stripping test suffixes recovers all 176 accepted sources. OFF retains the literal original loop without a new wrapper. The original field1200 library and native O3/one-codegen-unit core flags are reused. Earlier field/core/prover suites are inherited without replay.

Each curve passes 110 complete private-Window cases: 45 bucket/capacity combinations, one call on each of 53 actual workers across 1/4/16/32-worker pools, and 12 concurrent calls sharing an eight-worker pool. The 45 combinations cross bucket counts 1/63/64/65/127/128/2048/4096/16384 with capacities 8/32/200/352/512. Tests exercise mixed signs, identities, repeats, equal/opposite points, queued collisions, overflow, sparse and dense resets, empty completion and final canonical outputs. They are synthetic related cases, not cryptographic proofs or independent random populations.

|Qualified observations|G1|G2|
|---|---:|---:|
|Complete Window cases|110|110|
|Exact pending-set checks|31,180|31,180|
|Dense / sparse / empty checked executes|224 / 39 / 110|224 / 39 / 110|
|Queued arrivals / overflow flushes|18,467 / 244|18,467 / 244|
|Equal / opposite branches|118 / 111|118 / 111|
|Identity skips|2,038|2,038|

Before each checked execute and after each arrival, the bitmap equals the set reconstructed from pending destinations, with no duplicates. After execute it is zero, and its pointer, length and capacity remain stable. Every final bucket matches generator-coefficient arithmetic using the unchanged scalar/curve implementation. This is not a new independent Arkworks oracle. The separate reader also verifies 4096 subset/word reset cases; those finite cases supplement the source ownership argument. No RSS, minimum-RAM or stack-peak claim follows from unchanged vector capacity. Requested worker stacks are 512 KiB.

The initial combined test process passed G1 and failed G2 because the new shared fixture called G2 `finish()` with an undrained queue. G2's existing `feed()` empties its batch and queue before `finish()`; G1's `finish()` flushes for itself. A diagnostic checked conservation after each arrival. A separate control restores the byte-exact original G2 non-test body and confirms conservation through both batch completions, then reaches `finish()` with two queued arrivals and the same failure. Disabling direct batch arithmetic also retained the fixture failure. These observations establish fixture misuse, not a native kernel defect.

The correction adds only a test-side G2 queue flush and empty-state assertion before `finish()`. Both successful release libraries and G1's 110 cases are reused without rebuilding or rerunning them. The repaired G2-only test and previously unstarted generic no-ASM/no-IFMA compile pass. All four failed fixture-test exits, three diagnostic variants, source identities and command receipts remain visible. Twelve native workflow commands comprise eight successes and those four failures. Helper compiler/reviewer invocations have tool-transcript evidence rather than additional xtask receipts.

The final separate same-agent reader checks 738 repair pins plus earlier pin sets, all 12 command argv/exits/stream hashes, exact source recovery, the test-only repair, native/generic flags, original field identity and release-symbol absence of test helpers. Both native OFF/ON release cores remain frozen for the next stage. Generic compilation is not ARM execution. Mathematical/worker checks do not establish complete-SDK integration, proof correctness, concurrent proof throughput or performance.

The following full-proof timings are inherited from the [last unchanged SDK campaign](../2026-11-09-negative-phi-sdk176/README.md). The accepted local 1x2 result was already **7.832708 ms**. Historical 7.699867 ms used a different campaign and median of process medians; there is no new sub-8-ms result here. M5 CPU and Metal are supplied references on different hardware, not newly measured devices. All eight CPU 10× targets remain unmet.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.832708|37|24|4.724×|3.064×|3.7|
|2x2|11.065989|59|30|5.332×|2.711×|5.9|
|4x2|15.913588|109|48|6.849×|3.016×|10.9|
|8x2|24.254933|189|74|7.792×|3.051×|18.9|
|12x2|34.007120|284|101|8.351×|2.970×|28.4|
|16x2|40.086324|335|124|8.357×|3.093×|33.5|
|24x2|66.467587|528|176|7.944×|2.648×|52.8|
|merge-24x1|65.569872|507|175|7.732×|2.669×|50.7|

There are no new SDK requests, Gnark exports or proofs; the canonical ledger remains 41,104. No new AWS/Aeglos, ARM/phone/TEE, huge-circuit, pressure, swap, minimum-RAM or throughput observation is added. The change introduces no claim of a twofold improvement.

[Design](artifacts/design.md), [review](artifacts/review-result.json), [fixture repair](artifacts/test-repair.json), [decision](artifacts/decision.json), [continuation](artifacts/next.md), [sources and receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). The archive contains the changed source/test variants, helpers, plans, pins, original failures, successful native receipts and symbol summaries. Unchanged full source snapshots, executables, libraries, key/image/witness/proof bytes and the full private ledger are excluded.
