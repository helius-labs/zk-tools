# Continuation and autoresearch loop

The original goal remains active and unmet: independently verified CPU AVX single-proof latency at least 10 times faster than every supplied M5 CPU timing across ALL8. Report both CPU and Metal comparisons. Both accepted REF and this rejected candidate meet 0/8 targets. There is no token budget or genuine blocker recorded here.

This eight-update SIMD bucket-admission recipe is CLOSED: FAIL 12/77, not adopted. Read README.md, analysis.json, review-result.json and diagnostic-summary.json. Primary32 REF/OFF/ON = 7.770238516 / 7.784087968 / 7.903203710 ms. Paired ON/OFF = 1.020248572 [1.010576591,1.023997665], 0/15 wins. ALL8 ON/OFF = 1.006421726 [1.005047318,1.009719220], 0/15. All control calibrations, RSS guards and diagnostic quality checks pass. Instructions, cycles and branch misses increase in all three diagnostic panels. The fixed recipe has no net saving. Do not rerun it, adjust packet width/masks or select a favorable subset to rescue it.

Current production is unchanged current176. Accepted SDK:
/home/ubuntu/src/zopro-perf-work/busy-reset-sdk176/sdk-ON
SHA256 7aff5f602a1e250a6b9b28889bf6278b51ec400af8399c7481c36248a8718d01
Original field1200 and --cfg zopro_dense_busy_reset remain required. Source manifest: /home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-09-busy-reset-adoption176/artifacts/production-source-manifest.json. Its 176 originals include a binary fixture: preserve bytes. Do not enable this SIMD candidate in production.

Fresh accepted local32 ALL8 ms, in 1x2/2x2/4x2/8x2/12x2/16x2/24x2/merge24 order:
7.770238516 / 10.966498484 / 15.761755968 / 23.933997645 / 33.607895129 / 39.794608516 / 66.142308129 / 65.271702355.
Primary16 = 9.013310903 ms, solo = 94.257636774 ms, merge54 = 162.486027742 ms (no supplied M5 reference). The accepted 7.77 ms is the same executable previously observed at 7.783593581 ms, not a new adoption or new sub-8-ms breakthrough. These are Ryzen9950X observations, not fresh AWS comparisons.

M5 CPU = 37/59/109/189/284/335/528/507 ms; Metal = 24/30/48/74/101/124/176/175 ms. CPU10x targets = 3.7/5.9/10.9/18.9/28.4/33.5/52.8/50.7 ms. Do not confuse locally supplied cross-hardware ratios with equal hardware efficiency or Aeglos comparison.

Canonical independent proof inventory is now 44,800. Seed EVERY future new proof export from:
/home/ubuntu/src/zopro-perf-work/simd-bucket-sdk176/proof-sha256-after-capture.json
This stage added 535 unique fresh exports, each independently Gnark verified, and 19,510 internally verified SDK requests. Never seed from the older 44,287 preflight or 44,265 parent ledger. Private input/output payloads and full canonical ledgers stay local.

All work is terminal: preflight44296, freeze39258, capture83034, post37165, fixedhelpers32987 exited0. Capture ran 1,071 commands once, then separate producer and raw-reader analyses passed on first execution. Native qualification's130 tests and coherent SDKs were inherited without rebuilding. One setup compile failure before requests is retained alongside the separately fixed campaign source. Through report derivation there are1,127 executed workflow commands:1,126 successes and that one setup failure. Publication/checkpoint are separate. No owned cloud resources or live recording.

Continue autonomously without questions:

1. Check the exact pushed report/checkpoint and existing closed-study evidence before selecting a new mechanism. Do not repeat an unchanged negative or qualification for bookkeeping.
2. Select a materially different way to remove main CPU work or shorten a real dependency. Use actual complete-consumer profiles and mathematical operation counts; a symbol's samples are not exclusive removable cost. Review prior work before claiming novelty. SIMD predicate dispatch added work here. No new algorithm is selected merely by closing this trial.
3. Read relevant primary papers and other implementations when they inform a concrete hypothesis. Distinguish applicable algorithms from superficially similar titles. Research alone is not a measured improvement.
4. State the mechanism, expected saved work, resource cost, correctness obligations and rejection criteria before implementation. Keep bounded transient storage; no multi-GB LUT, expanded key/solver plan, new production pool, artificial architectural memory cap, Python, C/ASM rewrite or unauthorized subagents.
5. Implement a minimal Rust change with meaningful mathematical/native tests and coherent optimized complete-consumer links. Freeze producer AND separate raw reader, exact sources/binaries, ordering, gates and independent proof checks prospectively. Rust1.98.1 + serialized xtask is the established flow.
6. During ANY functional preflight or performance recording, ONLY poll the exact confirmed live handle until terminal. Timeout/truncation is not terminal. Never overlap an owned helper, build, hash, analysis or report with recording. Do not reroll a successful recording or modify declared thresholds after seeing results.
7. Analyze complete SDK latency, resources and automatic profiles. Retain ALL8+54 and primary1/16/32, actual fresh control calibration, real proof bindings, exact original/fresh controls, and independent fresh Gnark verifications. No arbitrary minimum improvement floor; require evidence of a positive result without material regressions.
8. Close each experiment honestly, save a short factual polite report with evidence, commit AND push, verify exact remote HEAD, and update /home/ubuntu/src/zopro-cpu-perf-handoff.md and latest-status.json. Do not publish private payloads or full proof ledgers. Reject a negative without modifying production; promote only a genuinely qualified improvement.
9. Keep the original full goal active until ALL8 actually passes. Do not invent a blocker or declare success on a subset, a microbenchmark, native correctness or a supplied cross-hardware estimate. Keep concise user progress updates, and answer interruptions before resuming work.
