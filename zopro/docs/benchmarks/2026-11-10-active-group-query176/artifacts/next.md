# Continue CPU AVX prover research

Goal remains active and unmet: verified single-proof CPU AVX latency at least
10 times faster than the supplied M5 CPU reference for all eight supplied
shapes, reporting CPU and Metal comparisons. Targets met: 0/8. Do not redefine
success around a component, a favorable subset, throughput, or correctness.

The unfinished scalar-class scan experiment is closed without adoption.
Read analysis.json, review.json, protocol.md, source-audit.json, identities.json,
and the qualification in ../active-group-scan176/qualification-review.json.
Capture session 58838 terminated successfully: 440 commands, 400 caller
processes, 14,144 complete checked queries including primes/postchecks.
All 24 preflights and every timed output match retained independent Arkworks
complete-MSM references. No SDK calls, proof exports, or production changes.

Qualified mechanism: stable u16 active-class list, fixed 512-byte stack array
per active nonforking leaf, checked workspace charging. Point storage, arithmetic,
pair order, initial copies, grouping policy, and task geometry remain unchanged.
The original and selected layer arithmetic is byte-identical after reversing
the signature and two loop headers. Ten native/generic test functions passed.
Original two qualification compile failures and one helper dependency failure
are retained; successful stages were reused. No timing repeats or gate changes.

Actual 2x2-B grouping removes 21.608% of source-level descriptor visits
(109,187 to 85,594) but changes no pair-addition count. Dense size-two classes
increase visits by 33.16%. Primary 1x2-B main scalar grouping declines; other
indexed final reductions can still use the changed helper. Do not describe
that complete caller as guaranteed instruction-identical or causally unaffected.

Result: 16 of 54 prospective checks fail: one calibration, ten nonregression,
four gains, one RSS. Verify these counts from analysis rather than copying
unchecked prose. Grouped 2x2-B32 ON/OFF 1.017932 [1.010328,1.033712], 1/15 wins;
ON/REF 1.024289 [1.008300,1.031278], 0/15. REF/OFF/ON medians
3.768666/3.801789/3.870890 ms. Whole-process peak RSS 54/53.75/61.25 MiB;
RSS is not incremental scratch or minimum RAM. The grouped A/B32 aggregate
ON/OFF is 1.007957 [1.004333,1.027606], 3/15 wins. Calibration fails only
2x2-A32 (lower 0.960062), so do not claim globally qualified control agreement.
The calibrated B32 failure independently supports rejecting the fixed recipe.

All eight PMU panels and eight profiles qualify; 50,167 samples total, no lost
records. Candidate namespaces occur in grouped G1/G2 ON profiles. Single panels
show only ~0.086% fewer whole-query instructions for 2x2-B32 and ~0.274% for
2x2-A32. Do not turn flat sample shares into wall-critical fractions or claim
that 512 bytes caused the RSS difference. No threshold/curve/worker/layout
rescue or full-SDK trial for this rejected recipe.

Accepted executable is still ../busy-reset-sdk176/sdk-ON, SHA
7aff5f602a1e250a6b9b28889bf6278b51ec400af8399c7481c36248a8718d01.
Accepted176 manifest is the tracked 2026-11-09-busy-reset-adoption176 report.
Latest local all-eight 32-worker CPU ms:
7.829285503 / 10.979334806 / 15.803968871 / 23.944016903 /
33.703579161 / 39.858478065 / 66.186709387 / 65.489902226.
Primary RSS 133.50 MiB, 16-worker 9.192236934 ms. Earlier 7.803403940 ms
was the same accepted artifact in another campaign, not a superseded faster build.
Merge54 162.541995323 ms has no supplied M5 reference. No new AWS/Aeglos result.

Canonical proof inventory remains 43,730, authoritative path:
/home/ubuntu/src/zopro-perf-work/large-class-sdk176/proof-sha256-after-capture.json.
Every future proof-export campaign must seed this exact ledger, not an older one.

Autoresearch loop:
1. Inspect current authoritative source/status and exact live handle if any.
   Previous turn is progress: new native/generic qualification, actual descriptor
   census, complete real query comparison, automatic profiles, and negative result.
2. Pick a distinct mechanism that removes bulk field arithmetic or dependencies
   in accepted callers. Use these fresh real-input profiles and existing whole-field
   census as denominators. A descriptor percentage alone is inadequate evidence.
   Check only the relevant prior experiment before implementation; avoid another
   broad history survey. G1 GLV, indexed-wide geometry, sparse XYZZ, singleton
   initial-copy elision, class splitting, and this scan recipe remain closed.
3. Prototype privately, preserve a reversible source audit, original field1200,
   exact arithmetic or independent oracle tests, generic fallback, real small-stack
   workers/concurrent callers, and checked resource accounting. No giant tables,
   expanded key plans, architectural RAM cap, production pool, or C/ASM rewrite.
4. Freeze complete caller controls, sample/order/gates, oracles, source/compiler/
   libraries, full analysis and separate reader before preflight. Keep all positive
   and adverse results. No minimum positive gain, but require evidence it is real.
5. During preflight or recording ONLY poll the exact confirmed live workflow handle.
   No builds/helpers/hash/review/decoding/report/push overlap. Timeout/truncation
   is not terminal. Missing handle or authoritative terminal receipt is terminal.
6. Recompute gates from raw receipts. Failure closes the fixed recipe. Success
   permits full ALL8+54 SDK/Gnark/resource verification, not automatic adoption.
   Adoption requires the final full-proof artifact and complete validation.
7. Write a factual, polite report; commit AND push each completed experiment;
   verify exact remote HEAD. Update handoff/latest-status and continue. No questions
   or delegated agents unless explicitly authorized. Rust1.98.1 and serialized
   xtask, no new Python helpers. Keep goal active until the full objective is proved.
