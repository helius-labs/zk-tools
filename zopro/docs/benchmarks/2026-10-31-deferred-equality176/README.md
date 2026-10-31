# Deferred equal-X detection in G1 bucket filling

The private candidate **fails the declared screen and is not adopted**. At 32 workers its two-shape ON/OFF paired estimate is about 3.0% slower, with an interval spanning parity and 4/15 winning blocks. Both 32-worker fresh-control calibration intervals also fail. These observations do not establish an exclusive causal slowdown. Solo ON/OFF improves about 1.1% on each shape with 15/15 wins; primary at 16 workers improves 2.58% with 12/15 wins. Those positive results are retained within their measured scope.

This is a complete ordinary-window filler experiment, not a new whole-proof measurement. Production, accepted SDK, proof ledger and all-eight goal remain unchanged. No unchanged retry, favorable worker-count promotion or code-layout rescue is selected.

## Mechanism

The original feeder checks finite bucket X against incoming X before every ordinary admission. The candidate retains the identity shortcut and busy-index uniqueness but defers equal-X detection to the central denominator product. Its checked inverse follows the original product-tree operation sequence and uses the scalar inverse's existing Option instead of unwrap. Over a field, that product is zero exactly when a denominator is zero. On zero, it returns before any tail or vector bucket write and performs complete signed curve additions to the batch's distinct destinations. Nonexception forward/reverse formulas, queue policy, batch size and final normalization are unchanged.

This removes per-arrival equality checks while adding a fallible batch return. It adds no field multiplication or input-sized buffer at source level. A zero batch pays the speculative forward pass plus full scalar additions; pathological valid inputs can therefore be slower. Exceptional intermediate states differ from the original immediate cancellation/deferred projective doubling representation. Complete outputs, rather than intermediate-state equality, are the contract.

No digit matrix, native bucket-column conversion, lookup table, copied proving-key points, duplicate solver plan, new thread pool, arbitrary RAM cap or C/assembly rewrite is introduced. The original functions remain verbatim. The copied original group-record unsafe writes are unchanged; no additional unsafe operation is introduced. Option layout, compiler frames and emitted code are not claimed memory neutral.

Source audit reverses the stream, checked inverse and Window edits to the accepted operations. Two measured snapshots have 179 files each: 172 original files exact, four append-only parents and three children; all 352 original copies recover. All 36 field-member copies are exact, including unchanged generated kernels. The failed initial 179-file candidate is separately retained, giving 537 source files in the archive. The prior 35 field-test qualification is inherited; zero fresh field tests are claimed.

## Complete-filler latency

Ryzen 9950X, 16 physical/32 logical CPUs. Original control has the new cfg absent; OFF and ON are the same final executable. Final links use opt-level 3, target-cpu=native, fat LTO and one codegen unit. Original streams in both binaries and the new checked stream contain IFMA.

The inherited raw pre-group A fixtures have 21,006 and 36,637 rows. Every pass fills all 21 ordinary width-12 windows. Admission, fresh Window allocation, queued/projective work, any checked fallback, final affine normalization and destruction are timed. The narrow top-window special case, weighted sums, final MSM combination, witness solving, proving and verification are excluded. These populations are not necessarily the final grouped queries in production.

15 fixed rotated/reversed blocks, both shapes at 1/16/32 workers, three checked prime passes and one checked post pass per caller. Every latency process times 20 complete passes. Whole-process affinity is CPU 2 solo, 0–15 physical, 0–31 logical; 512 KiB Rayon stacks, migration permitted within each mask. All latency precedes separate PMUs/profiles. No owned build, helper, analyzer, hash/archive work or second workload overlaps capture; exclusive host use is not established.

Ratios below are candidate/control; lower is faster. Times are medians of process averages; ratios and bounds are paired block statistics, so they need not equal ratios of displayed medians.

Two-shape 32-worker ON/frozen: **1.045024 [0.997471, 1.075812], 4/15**.

Two-shape 32-worker ON/OFF: **1.029831 [0.996204, 1.057970], 4/15**.

|Raw context|Original / OFF / ON ms per complete fill pass|ON/OFF [bounds], wins|ON/original [bounds], wins|OFF/original calibration|
|---|---:|---|---|---|
|1x2-w1|10.435574 / 10.531350 / 10.403507|0.988806 [0.985496, 0.990503], 15/15|0.996477 [0.994863, 0.999100], 13/15|1.007974 [1.005962, 1.011140], 0/15|
|1x2-w16|1.050173 / 1.067930 / 1.048909|0.974203 [0.937668, 0.995725], 12/15|0.991064 [0.950410, 1.004173], 10/15|0.995293 [0.979127, 1.028582], 8/15|
|1x2-w32|0.853507 / 0.861700 / 0.899305|1.031511 [0.979118, 1.111353], 5/15|1.027150 [1.019373, 1.085779], 2/15|1.011341 [0.985313, 1.045151], 6/15|
|2x2-w1|16.552033 / 16.675519 / 16.484777|0.989475 [0.987556, 0.989929], 15/15|0.995149 [0.994691, 0.997672], 14/15|1.007294 [1.005477, 1.008284], 0/15|
|2x2-w16|1.638685 / 1.653806 / 1.633261|0.984923 [0.978460, 1.006601], 10/15|0.999667 [0.986862, 1.032418], 8/15|1.008955 [0.995650, 1.022251], 5/15|
|2x2-w32|1.348751 / 1.349324 / 1.366647|1.007593 [0.977960, 1.044486], 6/15|1.017159 [0.993849, 1.049277], 4/15|1.012880 [0.976848, 1.041826], 6/15|


Failed checks: `["1x2-w32/OFF/frozen/calibration","1x2-w32/ON/frozen/nonregression","1x2-w32/ON/OFF/nonregression","2x2-w16/ON/frozen/nonregression","2x2-w32/OFF/frozen/calibration","2x2-w32/ON/frozen/nonregression","2x2-w32/ON/OFF/nonregression","two-shapes-w32/ON/frozen/gain","two-shapes-w32/ON/OFF/gain"]`.

Ordered ranks 4..12 of 15 give nominal 96.484375% marginal intervals under independent exchangeable blocks, without familywise guarantees. All OFF/original calibration intervals must fit [1/1.03,1.03]; every ON/original and ON/OFF upper bound must be <=1.03. Both equal-log two-shape 32-worker aggregates require upper <1 and >=10 wins. No gain floor, exclusion, extension or successful-capture repeat was applied. The 32-worker comparisons are variable, including large individual block excursions; all blocks remain present. These data do not justify blaming DRAM, caches, allocation, scheduler behavior or page faults exclusively.

## CPU and memory diagnostics

|Context|Original / OFF / ON CPU ms/pass|Original / OFF / ON max-RSS counter MiB (diagnostic)|Original / OFF / ON minor faults/pass|
|---|---:|---:|---:|
|1x2-w1|10.436 / 10.531 / 10.403|14.25 / 14.25 / 14.25|0.15 / 0.15 / 0.15|
|1x2-w16|11.756 / 11.751 / 11.728|18.75 / 18.75 / 18.00|8.90 / 12.40 / 11.30|
|1x2-w32|15.805 / 15.837 / 15.854|19.50 / 19.50 / 19.50|19.60 / 19.50 / 20.35|
|2x2-w1|16.552 / 16.675 / 16.485|17.00 / 17.00 / 17.00|0.10 / 0.10 / 0.10|
|2x2-w16|17.957 / 18.063 / 17.902|20.50 / 20.00 / 20.00|8.65 / 10.50 / 7.95|
|2x2-w32|23.641 / 23.864 / 23.647|18.75 / 19.25 / 19.25|16.80 / 16.90 / 16.80|


RSS counters are diagnostic because earlier work found undercounts; they are not precise lifetime peaks or minimum RAM. There is no fresh smaps, memory-pressure, concurrent-proof, phone, 250/4999 or swap qualification here. Minor faults count mapping/page-table work, not its elapsed cost or disk paging. Major faults are not exported by this driver.

At 32 workers, primary CPU service is effectively unchanged and 2x2 decreases about 0.9% versus OFF, despite the worse aggregate wall-time estimate. No input-sized allocation was added, but this does not prove unchanged stack or total process memory. The checked stream symbol is 30,726 bytes versus the original FAST signed stream's 26,179 bytes in both binaries. Static code size is not instruction-cache cost or dynamic spilling.

## Counters and profiles

|Stat panel|Cycles M/pass|Instructions M/pass|Generic cache misses M/pass|Branch misses M/pass|Profile samples|Unknown share|
|---|---:|---:|---:|---:|---:|---:|
|stat-1x2-w16-OFF|60.327|147.664|0.106927|0.115136|659|0.4552%|
|stat-1x2-w16-ON|60.148|145.539|0.108632|0.114631|640|1.2500%|
|stat-1x2-w16-frozen|60.042|147.653|0.106142|0.116411|634|0.1577%|
|stat-1x2-w32-OFF|76.347|151.730|0.117132|0.125924|717|0.5579%|
|stat-1x2-w32-ON|75.730|149.688|0.119121|0.123448|768|0.7812%|
|stat-1x2-w32-frozen|75.938|151.676|0.116735|0.126203|730|0.2740%|

|Workers|Descriptive ON/OFF cycles ratio|ON/OFF instructions ratio|
|---|---:|---:|
|16|0.997033|0.985609|
|32|0.991912|0.986538|


All six grouped PMU panels have 100% running time. Single-panel ON/OFF instruction counts decrease about 1.44% at 16 workers and 1.35% at 32; cycles decrease about 0.30% and 0.81%. These descriptive panels do not supply paired causal intervals or local CPI attribution. Fewer instructions do not establish a proof-latency gain.

Six separately FIFO-gated cycles:u profiles contain 4,148 samples, zero LOST, 634–768 samples each, unknown share at most 1.25%. They meet the declared 400-sample floor, but all are below 1,000 and provide thin descriptive context. PID, binary mmap, worker task records, CPU masks and fixed 16M period were checked. Full symbol bins are retained; no absolute-period coverage, removable critical-path share or dynamic-spill claim is made. Primes precede perf enable and postchecks follow disable.

## Qualification and provenance

All 128 native core tests pass: 125 original and three new tests for all 256 zero-lane masks, signed complete batches/tails and every exceptional destination of a 33-point batch, and full queue/exception windows against generator-coefficient oracles in 1/4/16/32-worker pools. The whole-filler preflight/prime/post outputs then match the inherited independently generated Arkworks reference for both real inputs. No fresh independent oracle generation, solve or SDK request is counted.

The initial native compile stopped before tests because the generated BatchAdd impl lacked its final brace. A separate candidate-fixed snapshot adds exactly one closing brace. Original failed source, helpers, plan/pins, Cargo log and structured receipts remain preserved. The repaired 128-test suite passes once in 36.95 s; neither successful tests nor measurements were repeated. Native and portable standalone prover suites were not run for this diagnostic boundary. Optional external-fixture core tests are not evidence for absent-fixture scenarios.

Before the first capture, a preparation-log filename collision overwrote the short native-plan preparer's stdout and caused its empty hash to be pinned while campaign preparation was still writing. The completed capture-preparation log and original pin map are retained. A repair requires that log to be the only mismatched pin, updates its completed hash, and pins the repair provenance. The earlier preparer's overwritten stdout is not claimed preserved. Plans, runtime code, all actual qualification receipts and every performance gate remain unchanged. No capture had started. See the linked repair note.

The 330 capture commands finish once: 18 preflights, 270 latency processes, 6 stat panels, 6 profiles and 30 ownership/decode/events/loss commands. All 300 callers check prime/post outputs: 9,018 complete passes including primes/post, 189,378 window fills and 51,609,600 bucket-output comparisons. These are not proof counts. The independent reviewer checks 682 pins, source recovery, all command exits/stream hashes, caller outputs, raw timing/CPU/fault/RSS statistics, paired gates and profile ownership/bins; it passes first execution. Floating round-trip comparisons use the inherited 1e-12 tolerance, with exact integer decisions. No SDK/Gnark export or ledger advance occurs; ledger remains 28,847.

The source search confirmed B1 omission and replacement quotient bases already exist in production; prior GLV assembly and SRE/Hrabar variants remain closed. This candidate does not relabel those techniques as new. No external implementation or benchmark speedup is imported.

## Inherited whole-proof CPU and Metal comparison

The following accepted local timings and supplied M5 references are inherited, not new candidate measurements. The accepted physical-16 calibration engineering exception and previous false full-performance gate remain recorded. The original verified single-proof CPU 10× objective is active and unmet on all eight shapes.

|Shape|Accepted whole-proof ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.928660|37.0|4.667×|24.0|3.027×|3.7|
|2x2|11.151932|59.0|5.291×|30.0|2.690×|5.9|
|4x2|16.086386|109.0|6.776×|48.0|2.984×|10.9|
|8x2|24.179453|189.0|7.817×|74.0|3.060×|18.9|
|12x2|33.884455|284.0|8.381×|101.0|2.981×|28.4|
|16x2|40.077381|335.0|8.359×|124.0|3.094×|33.5|
|24x2|65.850777|528.0|8.018×|176.0|2.673×|52.8|
|merge-24x1|65.060937|507.0|7.793×|175.0|2.690×|50.7|


Accepted merge54 remains 161.883899 ms, with no supplied M5 reference. The latest full-SDK experiment remains the separately rejected weighted-sdk176 study. The latest AWS comparison is historical source175 on c7a.24xlarge: Zopro 11.915591 ms at 32 workers versus Aeglos 7.959184 ms at 96; this experiment creates no AWS resource and supplies no fresh AWS/Aeglos result. Counter RSS observations above do not update accepted proof memory peaks.

[Protocol](artifacts/capture-protocol.md), [design](artifacts/design.md), [fallback argument](artifacts/fallback-proof.md), [analysis](artifacts/analysis.json), [independent review](artifacts/review.json), [source audit](artifacts/audit-v2.json), [codegen](artifacts/codegen.json), [full profile bins](artifacts/profile-symbols.json), [fixture bindings](artifacts/fixture-bindings.json), [native repair](artifacts/repair-notes.md), [preparation-log repair](artifacts/preparation-log-note.md), [source snapshots](artifacts/source.tar.gz), [workflow and helpers](artifacts/workflow.tar.gz), [all receipts](artifacts/receipts.tar.gz), [counter CSVs](artifacts/counters.tar.gz), [selected linked assembly](artifacts/codegen.tar.gz), [archive verification](artifacts/publication.json), [checksums](SHA256SUMS).
