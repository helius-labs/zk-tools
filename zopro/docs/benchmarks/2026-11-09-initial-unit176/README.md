# Initial-affine accumulator state qualification

**The private candidate passes native and generic qualification, and matching SDKs are built. Speed is unmeasured; production is unchanged.** The accepted build's latest matched 1×2 measurement remains **7.803149 ms** with 32 workers. The earlier [global unit-Z method](../2026-11-09-unit-z-sdk176/README.md) remains rejected.

This candidate records that an exceptional projective accumulator was initialized from a nonidentity affine point. It uses a spare bit in its existing sparse index and consumes that information on its first update or finish join. The specialized complete addition needs no dynamic Z comparison. Subsequent updates retain the accepted field method. G2's endomorphism preserves Z and therefore this initial state.

The accepted field source and original field library remain unchanged in both arms. All 176 accepted original files recover exactly: 173 are byte-exact and three contain reversible private edits. A private helper module is added. No Window fields, vector dimensions/capacities, table, key copy, plan, pool, allocator, forecast or memory policy changes. Bucket domains below 2^31 permit tagged indices while preserving the empty sentinel; larger domains retain the original full-width representation. Synthetic boundary checks do not establish physical 32-bit or ARM execution.

At most one ordinary mixed addition per eligible accumulator saves three field multiplications and two squares. This must pay for index masking, domain/flag checks, clearing stores and a new finish lookup. No proof-wide eligibility fraction or latency gain is established.

The targeted core tests exercise actual index state, identity/equal/opposite additions, queue flush, finish, endomorphism and independent Arkworks bucket results. The helper additionally matches the accepted raw Jacobian outputs and independent Arkworks results. Each Window test uses 53 actual broadcast workers across pools of 1, 4, 16 and 32 workers with requested 512 KiB stacks, followed by four callers sharing an eight-worker pool.

|Arm|Tests passed|Window case executions|Helper output checks|Synthetic encoding checks|
|---|---:|---:|---:|---:|
|Native OFF|4|138|2048|60|
|Native ON|4|138|2048|60|
|Generic ON|3|69|2048|60|

Fixtures are reused across arms; execution totals are not distinct proof counts. Generic execution covers the G1 Window and both-curve helper; the native-only G2 Window is unavailable with `no_ifma`. Native ON per curve logs 1,608 tagged updates, 469 tagged finish joins and 4,036 other recorded updates. Those are fixture observations, not proof-workload savings. All 345 Window cases pass. Older unchanged field and bitmap-reset suites are not replayed.

Two native core libraries, three targeted test binaries/runs and six downstream builds complete: 14 successful workflow commands. One initial test compile fails because the Arkworks G2 oracle cannot multiply-assign Fq2 by Fq; the test expression is corrected componentwise. Both completed runtime libraries are reused, with no runtime source change. The failed receipt and original test source are retained. A repair helper has a borrow-check compile fix, and the review reader is corrected to handle unchanged binary source assets without UTF-8 decoding. These manual helper attempts lack complete xtask receipts; no successful scientific work is replayed for them.

Source/receipt/link reviews bind both core/prover/zolana/SDK chains to the same original field and the unchanged driver with fixed worker placement, native O3, fat LTO and one codegen unit. Release binaries contain no qualification helpers. No SDK request, timed proof, PMU recording or profile is run in this qualification stage. New complete-proof correctness and performance remain to be measured before any adoption.

The following are the **previously measured accepted REF observations**, carried forward from the linked completed campaign. M5 CPU and Metal are supplied historical figures from different hardware; neither is remeasured here. This candidate has no timing column because it has not been timed.

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.803149|37.0 /4.742x|24.0 /3.076x|3.7|
|2x2|10.960964|59.0 /5.383x|30.0 /2.737x|5.9|
|4x2|15.869955|109.0 /6.868x|48.0 /3.025x|10.9|
|8x2|24.036033|189.0 /7.863x|74.0 /3.079x|18.9|
|12x2|33.735567|284.0 /8.418x|101.0 /2.994x|28.4|
|16x2|39.937712|335.0 /8.388x|124.0 /3.105x|33.5|
|24x2|66.114430|528.0 /7.986x|176.0 /2.662x|52.8|
|merge-24x1|65.391550|507.0 /7.753x|175.0 /2.676x|50.7|

The full all-eight goal remains active and unmet, **0/8**. Next is the frozen complete SDK comparison with all eight shapes, merge 54, primary single/16/32-worker conditions, automatic PMU/profiles and fresh Gnark verification. Canonical proof inventory remains 42,438; this stage adds zero exported proofs. No new AWS/Aeglos, ARM/mobile, huge-circuit, memory-pressure, swap, minimum-RAM or throughput result is claimed.

[Design](artifacts/design.md), [qualification review](artifacts/review-result.json), [link review](artifacts/link-review.json), [source recovery](artifacts/source-audit.json), [test correction](artifacts/test-repair.json), [continuation](artifacts/next.md), [sources and receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS).
