# In-place inverse-hint reuse

The candidate passes correctness checks but fails the declared full-SDK latency screen and is **not adopted**. At 32 workers, paired ON/OFF estimates are 0.33% slower for 1x2 and 1.12% slower for 2x2. The latter interval lies entirely above parity. Both 32-worker OFF/accepted controls calibrate, so this result is not solely a rejection caused by the separate 16-worker calibration failures. No unchanged rerun, favorable-subset promotion or layout rescue follows.

## Implementation

The [semantic census](../2026-10-31-semantic-cse176/README.md) certified repeated inverse hints from their original linear inputs. This private implementation sorts normalized coefficient vectors, coalesces repeated wire IDs and cancellations, compares full vectors, and selects the earliest inverse operation in each class. For nonzero c, inv0(c*x)=inv0(x)/c, including x=0. Each alias copies an earlier inverse output and applies its constant scale. Every original constraint and validation step remains.

49 hints are replaced on 1x2, 72 on 2x2, exactly as independently predicted. Product reuse and static query folding from the earlier census are not implemented here. Original hint inputs and image kind 0 remain serialized, preserving the solver image bytes. Original inputs plus the alias source enter schedule dependencies, including the bounded builder's read-capacity forecast. Equality-group keys distinguish source and factor. Discovery requires definite assignment and an uncached schedule; unsupported or unadmitted preparation keeps the original operations.

Native Op remains 72 bytes/alignment 4; Hint uses 56 bytes within existing operation storage. No new retained vector, map, DAG, key/plan copy, lookup table or arbitrary RAM cap is added. Two temporary fallible arrays are sized from actual hint terms, charged through the current resource policy, checked against actual capacities, and dropped before scheduling. Their peak charges are 36,640 bytes on 1x2 and 64,132 on 2x2. These figures exclude the existing, separately charged definite-assignment check's temporary flags and do not establish process startup peak, stack use, minimum RAM or equal allocator behavior.

Source snapshots each contain 177 files: 173 unchanged originals, three edited originals and one child. All field sources are exact. The repaired snapshot changes only the new child's layout check/test and adds original-layout shadow types. Main source remains unchanged. Final native qualification passes 128 unfiltered core tests and 8 prover tests (2 unit+6 cubic). Added tests check coefficient cancellation/scaling, zero inverse behavior, unchanged images, forced budget fallback, invalid/incomplete plans, cached-schedule refusal, dependency readiness and real scheduled solves in 1/4/16/32-worker pools with 512 KiB stacks. No fresh field suite or portable qualification is claimed.

The initial qualification records 126 passes/2 failures: a hardcoded 76 byte layout guard was wrong because the compiler produces 72 bytes. The guard disabled discovery; tests requiring aliases failed. Original source and receipts are retained. The repair compares the candidate layout against original hint-layout types using the same Step. Its first helper also attempted UTF-8 decoding of binary source fixtures and stopped before creating a new plan; byte-preserving copying fixed preparation. An attempted workflow invocation with the absent intermediate plan stopped before tests. These failures are disclosed in the resource notes. Successful qualification/capture work was not repeated for bookkeeping.

## Measurement

Before any preflight/timing, the preliminary isolated-solver screen was replaced by a full-SDK screen. Public accepted APIs expose complete proof parts; adding a raw Plan diagnostic would change qualified source. This scope revision is retained. Ryzen 9950X,16 physical/32 logical CPUs, full-process masks 0-15/0-31. Accepted source 176/SDK 6d83 is the separate original control. Candidate OFF/ON use the same executable, final-O3/native/fat-LTO/one-codegen-unit links and coherent frozen libraries. Three SDK primes precede every process.

12 preflights verify proof/public bindings and exact alias counts before latency. 15 fixed rotated/reversed blocks cover four contexts and three arms,101 requests per process. All latency precedes 6 PMU and 6 cycle captures,251 requests each. Every process exports its last randomized proof for independent Gnark verification: 204 fresh exports, 21,816 total SDK requests, distinct-proof ledger 28,847→29,051. Export counts are not the number of all generated proofs. 439 workflow commands complete once; no owned build, helper, analyzer or other CPU workload overlaps performance capture. Exclusive host use is not established.

Times are medians of per-process request medians. Paired ratios need not equal ratios of displayed medians. Ratios are candidate/control, lower is faster. The publisher appends the measured tables below.

|Context|Accepted / OFF / ON ms|ON/OFF [bounds], wins|ON/accepted [bounds], wins|OFF/accepted calibration|
|---|---:|---|---|---|
|1x2-w16|9.505696 / 9.569427 / 9.587529|1.007931 [0.984311, 1.128027], 6/15|1.022573 [0.948531, 1.174428], 6/15|0.994414 [0.956160, 1.259384], 8/15|
|1x2-w32|7.906321 / 7.914847 / 7.975131|1.003319 [0.998348, 1.015529], 4/15|1.003442 [1.000786, 1.013848], 2/15|1.003363 [0.994009, 1.006191], 6/15|
|2x2-w16|13.085022 / 12.973001 / 13.082427|1.004114 [0.976828, 1.024673], 7/15|1.006182 [0.967550, 1.027516], 7/15|0.995762 [0.963586, 1.040643], 8/15|
|2x2-w32|11.095611 / 11.111322 / 11.237650|1.011165 [1.007602, 1.014941], 2/15|1.013257 [0.999129, 1.018474], 4/15|1.002668 [0.991591, 1.008629], 6/15|

Four-context equal-log ON/OFF: **1.006068 [0.996626, 1.042236], 4/15**, required gain absent.

Four-context equal-log ON/accepted: **1.016076 [0.982702, 1.042600], 7/15**, required gain absent.

All declared failures: `["1x2-w16/OFF/accepted/calibration","1x2-w16/ON/accepted/nonregression","1x2-w16/ON/OFF/nonregression","1x2-w32/ON/accepted/32-worker-upper-bound","1x2-w32/ON/OFF/32-worker-upper-bound","2x2-w16/OFF/accepted/calibration","2x2-w32/ON/accepted/32-worker-upper-bound","2x2-w32/ON/OFF/32-worker-upper-bound","four-contexts/ON/accepted/gain","four-contexts/ON/OFF/gain"]`.

All calibration intervals must fit[1/1.03,1.03]; each ON/control upper bound must be<=1.03, and each32-worker shape must have upper<=1. Both equal-log four-context aggregates require upper<1 and>=10/15wins. Ordered ranks4..12 of 15 provide nominal96.484375% marginal coverage under independent exchangeable blocks, without familywise guarantees. No gain floor or measurement-informed exclusions/extensions. Physical16 bounds are wide; the screen does not establish a precise16-worker effect. The32-worker primary ON/accepted interval is above parity and 2x2 ON/OFF is above parity; the failed aggregate is not merely a control-calibration issue.

## CPU, memory and faults

|Context|Accepted / OFF / ON CPU ms/request|Accepted / OFF / ON max-RSS counter MiB|Accepted / OFF / ON minor faults/request|
|---|---:|---:|---:|
|1x2-w16|116.405 / 117.574 / 117.155|83.50 / 85.50 / 85.00|79.48 / 85.39 / 83.68|
|1x2-w32|179.304 / 179.901 / 179.550|127.25 / 127.50 / 126.25|183.08 / 185.97 / 179.75|
|2x2-w16|162.928 / 162.544 / 163.491|190.75 / 192.50 / 192.50|228.81 / 228.99 / 233.62|
|2x2-w32|257.541 / 257.902 / 258.123|304.25 / 307.00 / 306.50|500.55 / 510.83 / 508.99|

At 32 workers1x2 CPU service is approximately flat versus OFF;2x2 slightly increases. RSS counters remain diagnostic due to earlier observed undercounts: they are not precise lifetime peaks, minimum RAM, pressure or swap bounds. Minor-fault counts do not measure fault service time or establish disk paging. Major faults are not exported. The small count differences do not explain a latency gain; none was established.

All 6 PMU groups report full qualification. ON/OFF retired instructions increase about0.043% at 16 workers and 0.044% at 32; cycles increase about0.23%in both one-shot primary panels. These descriptive panels are not paired causal confidence intervals.

## Automatic profiles

Six full-SDK cycle profiles conserve63,190 samples, zeroLOST,8,697–12,335 samples each, unknown share 1.20–1.89%. PID-specific binary mappings, thread forks, CPU masks and 16M fixed periods are checked. All profiles pass the inherited>=1,000 sample/<=5% unknown attribution checks. Flat user-cycle periods do not validate absolute period coverage, kernel fault time, local CPI, dynamic spills or a critical path.

Named solver frames account for roughly1.4–2.0%of sampled periods. MSM/curve frames account for roughly67.7–76.3%; generic inversion frames account for6.9–8.6%, mixing callers rather than isolating these Fr hints. This reinforces the limited scope of hint reuse relative to dominant curve work. The profiles do not prove that the added source dependency caused the regression. Solver scheduling, compiler context and runtime effects are not isolated by this trial. Full symbol/category totals are retained.

## Accepted proof comparisons, unchanged

The following are inherited accepted local32-worker observations and supplied M5 references. Fresh controls/candidate measurements appear above; the other six shapes were not remeasured after this initial screen failed. Production still uses the accepted176-file source and SDK SHA-256 `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`. All eight verified CPU 10x targets remain unmet.

|Shape|Accepted ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10x target ms|
|---|---:|---:|---:|---:|
|1x2|7.928660|37 / 4.667x|24 / 3.027x|3.7|
|2x2|11.151932|59 / 5.291x|30 / 2.690x|5.9|
|4x2|16.086386|109 / 6.776x|48 / 2.984x|10.9|
|8x2|24.179453|189 / 7.817x|74 / 3.060x|18.9|
|12x2|33.884455|284 / 8.381x|101 / 2.981x|28.4|
|16x2|40.077381|335 / 8.359x|124 / 3.094x|33.5|
|24x2|65.850777|528 / 8.018x|176 / 2.673x|52.8|
|merge-24x1|65.060937|507 / 7.793x|175 / 2.690x|50.7|

Accepted merge 54 is161.883899 ms without a supplied M5 reference. The accepted physical16 calibration exception remains in its adoption report. Latest retained AWS source 175 comparison on c7a.24xlarge remains historical: Zopro11.915591 ms at 32 workers and Aeglos7.959184 ms at96, each campaign's best tested configuration. This trial adds no AWS/Aeglos, portable, phone, ARM/TEE,250/4999, concurrent-throughput, pressure or swap qualification.

The full inverse-alias implementation is closed negative for this declared screen. Do not reroll it through layout/attribute changes or promote favorable individual blocks. The small structural opportunities remain real, but a future materially different approach needs its own evidence; no proof gain transfers from a census. The next substantial reduction must address dominant curve/field work while preserving parallelism and bounded memory.

## Artifacts and review

[Design](artifacts/design.md), [premeasurement scope](artifacts/capture-scope.md), [resource/failure notes](artifacts/resource-notes.md), [all results](artifacts/analysis.json), [raw rows](artifacts/rows.json), [independent review](artifacts/review.json), [review source](artifacts/review.rs), [profile symbols](artifacts/profile-symbols.json), [proof inventory](artifacts/proof-inventory.json), [source snapshots](artifacts/source.tar.gz), [raw receipts](artifacts/receipts.json.gz), [source audit](artifacts/source-audit.json), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [identities](artifacts/identities.json), [accepted manifest](artifacts/accepted-manifest.json), [publisher](artifacts/publish.rs), and [checksums](SHA256SUMS).

The independent Rust reader checks801 pin entries,445 successful commands and their streams, source-repair scope, exact128+8 test counts, raw SDK samples and paired gates, all 204 proof bindings and ledger union, and profile-sample conservation. It shares SHA-256/JSON infrastructure, not the campaign's analysis code. The source archive contains354 regular files with exact member/hash roundtrip; receipt gzip decompression matches its original bytes. Private inputs, aliases, keys, raw perf captures, executables and libraries remain local. Source qualification failures are preserved, not relabeled as successful work.
