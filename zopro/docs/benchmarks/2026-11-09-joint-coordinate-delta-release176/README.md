# Joint X/delta release caller comparison

**Rejected: jointly computing X and its Y delta does not improve the complete caller screen.** G2 regresses in calibrated solo and 16-worker contexts. Its 32-worker aggregate is **3.16% slower than matched OFF**, with one win in seven blocks. G1 improves about 1% solo, but the declared joint promotion checks fail. No optimization is adopted; accepted local full-proof 1×2 remains **7.857170 ms**.

This follows the [field/curve qualification](../2026-11-09-joint-coordinate-delta176/README.md) and [ordinary caller qualification](../2026-11-09-joint-coordinate-delta-callers176/README.md). The fixed proposal shares the uncarried X expression with an opaque, partially reduced delta, letting their carry chains proceed separately. Original multiply/cancellation bodies remain unchanged. The release comparison evaluates the complete signed borrowed G1 and G2 packet callers, including packing, prefix/inversion, finishing, writes and reuse.

## Qualification and release artifacts

The normal release field and matched OFF/ON cores explicitly disable debug assertions and overflow checks, and omit the diagnostic cfgs used in checked qualification. No TLS route symbols appear in the release consumers. Field arithmetic source is byte-exact to the checked parent. OFF and ON share source and the same new field library; only their core/driver selector differs. All 176 accepted sources remain recoverable; production sources stay exact. [Source audit](artifacts/source-audit.json), [release review](artifacts/review.json), [identities](artifacts/identities.json).

Each release arm passes 128 independent complete raw G1/G2 MSM comparisons over 53,680 input terms. Sizes are 0/1/7/8/9/255/1024/2051, with 1/4/16/32-worker pools and requested 512 KiB stacks. The oracle uses independent Arkworks coefficient sums and generator multiplication, including repeated/opposite/identity bases and zero/one/minus-one/full-width scalar cases. Inputs remain unchanged. These are synthetic complete MSM cases, not actual proving-key queries or proofs.

Thirteen successful build/oracle/symbol/decode commands are retained without repeating successful work. One initial external-oracle compile failed because Arkworks `ZERO` needed an `AdditiveGroup` import; the import-only [repair](artifacts/repair.json), original source and failed receipt remain. The first three successful builds were reused. An offline function reader also needed a [selector repair](artifacts/codegen-reader-repair.json) for Rust's inherent-method demangling. No build, test or decode was repeated for that repair.

The actual borrowed G1 kernel has 4,425/4,451 OFF/ON static instruction rows and 800/800 IFMA sites; G2 has 8,590/8,553 rows and 1,450/1,450 IFMA sites. Static stack-operand rows increase from 593 to 607 for G1 and 1,471 to 1,490 for G2. These counts do not measure spills, stack peaks or cycles. The unsigned G1 body has different inlining between builds and is outside the timed borrowed path. [Codegen review](artifacts/codegen-review.json).

## Frozen caller screen

The prospective screen includes both curves, 200/352 pairs and 1/16/32 workers: 12 contexts, exact frozen reference plus fresh matched OFF/ON, and seven rotated/reversed blocks. It retains every observation. There are 36 preflights, 252 latency processes and 24 diagnostic processes, totaling 312 checked caller processes and 50,455,296 worker batch calls including priming. These counts are not proofs. All 373 workflow commands succeed on their first execution.

Each latency process performs 4,096 rounds after three checked 128-round primes. Outputs and scratch capacities match across arms, with zero timed allocations. CPU placement and 512 KiB requested worker stacks are fixed. All recording finishes before offline decoding and analysis; no other owned work runs during recording. The original and matched controls are both required for promotion. [Selection](artifacts/selection.md), [prospective plan review](artifacts/capture-plan-review.json), [results](artifacts/capture-analysis.json), [separate final review](artifacts/capture-review.json).

Times below are microseconds per batch wave, with one fixture per worker. Ratios are medians of paired block ratios; they need not equal ratios of the separately reported median times. Brackets show the declared zero-based sorted ranks 1 and 5 of seven ratios: a descriptive screen interval, **not a 95% confidence interval**. Lower ratios are faster.

|Context|Reference µs|OFF µs|ON µs|Paired ON/OFF [ranks 1,5]|Wins /7|Control calibration|
|---|---:|---:|---:|---:|---:|---|
|G1-200-w1|4.215|4.213|4.174|0.990310 [0.988766, 0.991099]|7|pass|
|G1-200-w16|4.759|4.790|4.750|0.994711 [0.985183, 1.002909]|4|pass|
|G1-200-w32|8.079|6.865|6.970|1.015312 [0.998843, 1.033977]|2|FAIL|
|G1-352-w1|6.930|6.924|6.869|0.990067 [0.989697, 0.992201]|7|pass|
|G1-352-w16|7.929|7.925|7.906|0.997917 [0.993652, 0.998447]|6|pass|
|G1-352-w32|11.709|11.875|12.032|0.986278 [0.825613, 1.019516]|4|FAIL|
|G2-200-w1|7.655|7.668|7.929|1.034344 [1.033000, 1.035508]|0|pass|
|G2-200-w16|8.939|8.928|9.261|1.037216 [1.035959, 1.044774]|0|pass|
|G2-200-w32|14.319|14.536|15.234|1.050789 [1.029955, 1.068425]|1|pass|
|G2-352-w1|12.972|12.979|13.338|1.027589 [1.027249, 1.028075]|0|pass|
|G2-352-w16|15.179|15.277|15.624|1.025929 [1.022428, 1.026865]|0|pass|
|G2-352-w32|24.783|25.230|25.496|1.011573 [1.005755, 1.014651]|0|pass|

Two G1 32-worker OFF/reference calibrations fail the declared factor-1.05 range. Those rows are retained and cannot establish a reliable accepted-reference gain. The calibrated G2 solo and 16-worker regressions independently reject the proposal. Ten of twelve calibrations pass. There are 21 failed promotion checks and nine clear-regression checks. The fixed experiment is closed without rerolls, extension, a favorable curve/size subset, or an SDK followup.

The 32-worker G2 aggregate ON/OFF ratio is 1.031580 [1.022080, 1.036617], with 1/7 wins; against the frozen reference it is 1.045163 [1.011748, 1.052534], with 0/7 wins. G1's 32-worker matched aggregate is 1.000690 [0.921555, 1.012762], with 3/7 wins, subject to its failed control calibrations. The all-context 32-worker matched aggregate is 1.001219 [0.977394, 1.021658], with 3/7 wins. No aggregate gain qualifies.

## CPU work and memory

All counter panels run at 100%. All twelve profiles pass their declared quality checks: 94,820 samples in total, no LOST events, unknown symbols below 0.091%, with PID/TID/CPU/MMAP and 16-million-cycle period checks. The following single-panel comparisons describe work per 32-worker batch wave; they are not repeated counter estimates or causal attribution.

|32-worker context|Instructions/wave OFF → ON|Cycles/wave OFF → ON|
|---|---:|---:|
|G1-200-w32|1678136 → 1684254|956197 → 950401|
|G1-352-w32|2725810 → 2736241|1588649 → 1579327|
|G2-200-w32|3557268 → 3522085|1894562 → 1988353|
|G2-352-w32|6033154 → 5971299|3249569 → 3363037|

G2 executes fewer measured instructions but takes more measured cycles. The regression also appears solo, with equal scratch capacities and the same 2.75/3 MiB process RSS for the 200/352 solo fixtures. A scaling-only or larger-RAM-only explanation is insufficient. The G2-200 generic cache-miss panel decreases from about 324 to 38 per wave while cycles increase; G2-352 misses increase from about 209 to 234. These counters do not identify a cache level, DRAM cause, spill bottleneck, clock change or critical path.

Flat profiles put G2-200's main prefix kernel at 7,119/7,471 OFF/ON samples and G2-352's at 12,696/13,109. Inlining and sampling skid limit attribution. Fewer static rows or retired instructions do not guarantee fewer cycles. [Full diagnostics](artifacts/diagnostic-analysis.json).

The G1 record remains five fields and 1,600 bytes; the helper adds no production buffer, table, copied key, retained plan, pool or RAM cap. Maximum whole-process RSS is 14.75 MiB across latency processes and 15.25 MiB across all caller processes including diagnostics. These are small caller drivers, not full-proof peaks or minimum-RAM requirements. All per-context CPU service, RSS and capacity observations remain in the results. Prior qualification facade allocations are outside the changed kernel; the measured driver itself verifies zero timed allocations.

The next investigation needs a distinct material reduction of complete-consumer arithmetic or machine work. This local normalization/dependency rearrangement has now been tested through complete callers and is closed. Its retained code and profiles can support offline analysis, but they do not justify rescuing the same fixed proposal. The CPU 10× objective still requires a 52.91% wall reduction for primary 1×2 and 46.62% for 2×2, as well as gains on larger shapes. [Continuation](artifacts/next.md).

## Accepted CPU and Metal comparison, inherited

These unchanged local Ryzen 9950X full-proof results use 32 workers and common fixed placement. Supplied M5 CPU/Metal references are on different hardware. The caller times above are not proof times or new accepted baseline measurements.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.857170|37|24|4.709×|3.055×|3.7|
|2x2|11.052363|59|30|5.338×|2.714×|5.9|
|4x2|15.952018|109|48|6.833×|3.009×|10.9|
|8x2|24.080322|189|74|7.849×|3.073×|18.9|
|12x2|34.012417|284|101|8.350×|2.970×|28.4|
|16x2|40.128912|335|124|8.348×|3.090×|33.5|
|24x2|66.348981|528|176|7.958×|2.653×|52.8|
|merge-24x1|65.829291|507|175|7.702×|2.658×|50.7|

Production176/SDK6d83 and the 40,154-verified-proof ledger are unchanged. Goal active and unmet, 0/8. No new SDK request, proof, Gnark export, AWS/Aeglos, mobile/ARM, huge-circuit, memory-pressure, swap, minimum-RAM or throughput result is added. No job or owned cloud resource remains live.

[Source and receipt archive](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS).
