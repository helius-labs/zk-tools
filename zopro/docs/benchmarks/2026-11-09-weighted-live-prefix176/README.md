# Initialized-prefix weighted reduction

**The candidate improves the 32-worker primary paired comparison by 0.42%, but remains unadopted because the 16-worker calibration and non-regression gates fail.** This retains a small supported signal without claiming broad qualification. All 231 fresh exported proofs verify independently with Gnark. The full all-eight CPU 10× objective remains active and unmet, 0/8.

The candidate builds each weighted tree as an initialized prefix using safe `Vec` operations. The existing implementation resizes the complete tree to infinity, replaces its leaves, and subsequently overwrites each internal slot before reading it. The candidate reserves the same capacity, appends the leaves, then appends saved even points before reducing each level. It keeps the arithmetic, reduction order, inverse modes, point representation and thread pool. Existing equal-size reuse already avoids some initialization; fewer source stores do not by themselves prove a speedup.

This private cfg changes only the initialization and dispatch in `weighted_affine`, with a child copy of the original weighted reducer taking a `Vec`. The original reducer and its other callers remain unchanged. Reversing the edits recovers all 176 production files exactly. No persistent table, key/witness/scalar/point-array copy, execution plan, production pool, C/assembly implementation or artificial memory cap is introduced. The maximum reserved tree payload is unchanged; process RSS is measured separately below.

## Complete-proof comparison

Local AMD Ryzen 9 9950X, Rust 1.98.1, native O3, fatLTO, one codegen unit. Accepted is the unchanged `6d83` SDK; OFF is a coherent fresh build with the selector disabled; ON is its enabled counterpart. Core, Prover, Zolana and SDK dependencies are linked coherently, reusing the exact accepted field library.

All shapes, merge 54→1 and primary at 16 workers get correctness preflights for all 3 arms. Timings cover 31 matched blocks×151 requests for 1×2 at 32 workers,15×151 at 16 workers, and15×51 for 4×2 at 32 workers. Every process has 3 priming requests, internal SDK proof verification and a final exported proof. Rotated/reversed arm/context order, both controls, bootstrap procedure, wins, resource gates and profiles were frozen before any preflight. There is no minimum percentage gain floor.

The following are medians of process means. Paired geometric ratios below use matched blocks, so ratios need not equal ratios of these medians.

|Context|Accepted ms|OFF ms|ON ms|ON/accepted paired gain|ON/OFF paired gain|
|---|---:|---:|---:|---:|---:|
|1x2-w16|10.339425|9.812820|9.912122|2.471%|1.111%|
|1x2-w32|8.061263|8.107382|8.018331|0.416%|1.014%|
|4x2-w32|16.054442|16.146161|16.086201|0.273%|0.805%|

At 32 workers, ON/accepted=0.995844(95% paired bootstrap 0.992669–0.999367), 23/31 wins. ON/OFF=0.989858(0.986828–0.992762), 26/31 wins. Both meet the declared primary gain gates; OFF/accepted calibration passes. The 4×2 calibration and both ≤1.02 non-regression upper bounds pass.

At 16 workers, OFF/accepted's interval 0.918150–1.053417 exceeds the declared [1/1.03,1.03] calibration range. ON/accepted and ON/OFF upper bounds 1.041650/1.051556 do not establish ≤1.02 non-regression. Their paired point estimates are favorable, but the uncertainty is too large. This is **incomplete qualification, not proof of a slowdown**. The original observations remain intact; no exclusion, extension, reroll, relaxed gate or production adoption follows this campaign. Topology confirms CPUs0–15 are 16 distinct physical cores; using 8 SMT pairs is not an explanation.

The first screen therefore does not select full adoption. Its 32-worker signal remains a candidate for further work. A new 16-worker control/stability question should first be grounded in the existing request distributions, CPU time, switches and profiles. Full all eight shapes plus merge 54→1 timings and admission/resource/portable/concurrent qualification remain required before adoption. This small gain also falls far short of the full 10× objective.

## Correctness and resources

The ON core suite passes 127 tests, OFF passes the 2 new shared tests, and the coherent ON Prover passes 2 raw plus 6 cubic tests. Both curves compare against the original reducer and independent generator-weighted sums across powers of two 1..1024, changing live lengths, pre-reserved reuse, full-width/equal/opposite/identity patterns and scalar/vector inversions. Each arm checks 224 fixture cases and 74,016 complete-tree limb points. The test checks actual 1/4/16/32-worker indices, 512 KiB requested stacks and nested joins: 53 actual workers and 212 curve calls per arm. Fresh parent calls also exercise tree allocation. Generic x86-64 noASM/noIFMA compiles; this is not phone/portable runtime qualification.

All 7 native and 12 coherent SDK workflow steps finish successfully on their first execution. Capture/offline has 507 successful commands: 231 recordings, 231 independent Gnark verifications and 45 profile ownership/decode/event/metadata/loss reads. All recordings precede external verification and offline work; during recordings only the confirmed-live handle is polled. The frozen analysis and later same-agent review each finish successfully. No unchanged old primitive/oracle, successful capture, decoder or verifier is replayed for bookkeeping.

There are 28,644 complete SDK requests, including priming, and 231 fresh unique final exports. Every export's image/VK/public/proof hashes agree with its independent Gnark result. Canonical inventory advances 37,633→37,864. The separately written same-agent reader reconstructs raw means, schedule, resources, control/gain gates, export bindings and all eight reference comparisons. Bootstrap, binding and detailed PMU parsers are shared; this is not an external audit. Floating comparisons retain 1e−12 tolerance; integer/hash/count checks are exact.

One setup-only helper attempt used the wrong filename for the preceding study's analysis. It failed before clocks; correcting that path and permitting exact-equality resume of already-written setup artifacts completed preparation. The first source/binary are retained. No test, arithmetic, runtime, gate or measured observation changed.

|Context|Accepted / OFF / ON peak process MiB|Accepted / OFF / ON minor faults per request|Timed major faults OFF / ON|
|---|---:|---:|---:|
|1x2-w16|89.50 / 89.75 / 88.50|59.79 / 59.44 / 58.69|0 / 0|
|1x2-w32|135.75 / 135.75 / 137.00|128.74 / 128.46 / 130.61|0 / 0|
|4x2-w32|333.50 / 337.00 / 338.00|814.22 / 787.29 / 796.20|0 / 0|

All RSS screens pass. Peak process RSS includes loading and allocator/worker state; it is not the tree payload, minimum RAM, a memory-pressure limit or evidence of phone/swap/TEE operation. The accepted SDK does not report major faults; its value is unavailable, not inferred zero.

## Automatic CPU profiles

All 9 profiles pass: 308,328 samples total, 100% assigned to the recorded process/worker cohort, 1.34–2.72% unknown, zero LOST. All 36 grouped counter event values run 100% of enabled time. Profiles use cycles:u, period 16,000,000, 512 mmap pages and 501 timed requests per process. PMU uses 31 requests; FIFO gates exclude priming from both. Both weighted curve reducers appear in the candidate's native symbols and actual profile samples.

|Context / arm|Instructions per proof, billions|Cycles per proof, millions|Cache misses per proof, millions|
|---|---:|---:|---:|
|1x2-w16-OFF|1.348090|576.122|1.874|
|1x2-w16-ON|1.347432|562.796|1.839|
|1x2-w16-accepted|1.347070|556.070|1.841|
|1x2-w32-OFF|1.386536|825.899|2.832|
|1x2-w32-ON|1.387964|813.571|2.701|
|1x2-w32-accepted|1.388566|820.124|2.952|
|4x2-w32-OFF|2.364554|1928.484|7.095|
|4x2-w32-ON|2.366011|1904.094|7.112|
|4x2-w32-accepted|2.362590|1907.703|7.126|

The 32-worker candidate has approximately unchanged instructions versus both controls. A single PMU panel per arm cannot establish a cache/DRAM/store/fault cause, repeated-counter uncertainty, or an exclusive critical-path fraction. Fewer source initialization writes are a mechanism under test, not a measured large instruction reduction. The [prior query-work sensitivity study](../2026-11-09-query-neutralization-sdk176/README.md) still motivates larger G2/KH work reductions; cached same-key/witness query outputs are diagnostic and are not adopted.

## Accepted CPU / Metal comparison

Primary and 4×2 accepted observations are fresh in this screen; the other 6 are inherited from the prior full-shape campaign. These compare different hardware against the supplied M5 references. They are accepted-build observations, not a claim that the unadopted candidate has qualified all eight shapes.

|Shape|Accepted local ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× limit ms|
|---|---:|---:|---:|---:|
|1x2|8.061263|37 / 4.590×|24 / 2.977×|3.7|
|2x2|11.149097|59 / 5.292×|30 / 2.691×|5.9|
|4x2|16.054442|109 / 6.789×|48 / 2.990×|10.9|
|8x2|24.131178|189 / 7.832×|74 / 3.067×|18.9|
|12x2|33.940628|284 / 8.368×|101 / 2.976×|28.4|
|16x2|40.164083|335 / 8.341×|124 / 3.087×|33.5|
|24x2|66.187704|528 / 7.977×|176 / 2.659×|52.8|
|merge-24x1|65.361571|507 / 7.757×|175 / 2.677×|50.7|

Merge 54's inherited accepted timing remains 160.890372 ms without an M5 reference; this screen adds correctness, not a fresh merge 54 timing campaign. Historical same-build primary 7.915095 ms remains valid; private 7.859756/7.869400 ms candidates were unadopted. Fresh 8.061263 ms does not establish an adopted regression or its cause. Accepted primary remains roughly 8 ms. No fresh AWS/Aeglos/Gnark latency comparison or cloud resource is introduced. The verified all eight CPU 10× goal remains unmet, 0/8.

[Timing analysis](artifacts/latency-analysis.json), [profiles/counters](artifacts/diagnostic-analysis.json), [review](artifacts/review.json), [native qualification](artifacts/qualification.json), [source audit](artifacts/source-audit.json), [frozen protocol](artifacts/protocol.json), [design](artifacts/design.md), [candidate and tests](artifacts/weighted_live_prefix.rs), [continuation loop](artifacts/next.md), [source/logs/receipts](artifacts/evidence.json.gz), [checksums](SHA256SUMS). The archive includes UTF8-member SHA256 hashes and verified gzip roundtrip. It excludes binaries/libraries, perf.data and key/witness/proof/ledger payloads; proof hashes and verifier receipts remain. Reproduction needs the pinned inputs/dependencies and adapted absolute paths.
