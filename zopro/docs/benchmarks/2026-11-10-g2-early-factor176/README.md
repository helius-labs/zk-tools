# Evaluate the G2 prefix factor before point packing

**G2 improves at every worker count, including 2.49% at 32 workers against the accepted-source control. All six primary/combined gain checks pass. The complete screen nevertheless fails 1/48 checks and the candidate is not adopted.** The sole failure is the unchanged G1 32-worker ON/OFF nonregression bound: 1.004260 [0.988901,1.020921], against a declared upper limit of 1.02. That interval includes parity; it does not establish a 2.09% slowdown. No threshold, block count or population changes after capture.

Accepted local 1×2 full-proof latency remains **7.783594 ms at 32 workers** and **9.016864 ms at 16 workers**. This experiment adds complete-Window measurements, not a new full-proof observation.

The change moves two independent products—`factor=record_prefix*inverse` and `inverse*=record_norm`—to the start of each G2 reverse eight-point group, before assembling and packing point coordinates. They depend only on existing records and inverse state. For valid batches, point arithmetic commutes with this calculation; bucket writes, prefix recurrence, record order, signs, tails and numeric bounds remain unchanged. There are no added arithmetic operations, calls, buffers, layouts, keys, tables, plans or pools.

Before implementation, a reload-based helper proposal was rejected: dropping and rebuilding the final coordinate inputs would repeat six field packs and 1,536 bytes of raw reads, with possible call-state transfers. These source-level counts are not measured spill costs. Prior whole-loop and per-product outlining remain negative evidence. The new statement ordering tests a different lifetime arrangement without those extra operations.

One private Rust child changes, and reversing the statement movement recovers all 181 parent snapshot members; 180 are byte-identical. All field sources and G1 statements are exact. The byte-exact qualified field library is reused, inheriting its 196,608 independent BigUint/old-path comparisons without a rebuild or repeat arithmetic test. All 176 production manifest members remain exact.

Four tests pass against the new coherent core: 1,695,744 independent Arkworks prefix point comparisons, 5,018,112 bucket comparisons and 220 threaded Windows including 1/16/32 workers and twelve callers sharing an eight-worker pool. Twenty-four conversion cases preserve pointer and capacity without allocation, reallocation or deallocation. Complete single-Window requested-live peaks remain 191,360/303,360 bytes for G1/G2. Buckets stay 64/128 bytes and records 1,600/640 bytes. Portable evidence is inherited from unchanged guarded field/native paths; no ARM execution is added.

|Linked G2Scratch::apply|Previous fused candidate|Early factor|
|---|---:|---:|
|Function bytes|51,415|50,359|
|Static instructions|8,625|8,522|
|Prologue `sub rsp` reservation, bytes|7,872|6,848|
|Explicit `rsp` memory sites|1,593|1,502|

The emitted mnemonic sequence changes. These static counts establish neither executed spill counts nor a causal speed estimate. Stack reservation is not full call depth or minimum stack/RAM.

Four arms: REF retains the API-only accepted-source build; BASE runs the original Window path in the new coherent build; OFF retains the fused-incoming-Y candidate; ON adds early factor evaluation. BASE/ON share one executable. BASE/REF calibrates equivalent original algorithms; OFF is a mechanism control and intentionally differs from the accepted baseline. Driver selectors and arm labels are frozen separately.

Ryzen 9950X, 16 physical/32 logical CPUs, shared host. Six contexts use G1/G2, 15,045 points, width12, column0, batch200 and 1/16/32 workers. Worker i pins to CPU i; solo uses CPU2; stacks request 512 KiB. Independent-Arkworks fixture bytes are reused unchanged. Three checked primes precede seventeen timed waves. Each wave includes fresh Window construction, feeds, queues, scalar paths, normalization, full output comparison, checksum and drop. Input loading/hashing and pool setup are excluded. **A wave runs one Window per worker, not a complete MSM, proof or server-throughput measurement.**

Fifteen fixed rotated/reversed context/arm blocks are retained. Displayed times are medians of process medians; paired ratios need not equal ratios of displayed times. Bounds use fixed ranks4–12 of15, with nominal 96.484375% individual median coverage under independent exchangeable blocks, without a familywise guarantee.

|Context|REF ms|BASE ms|OFF ms|ON ms|ON/REF [bounds], wins|ON/OFF [bounds], wins|
|---|---:|---:|---:|---:|---|---|
|G1 / 1|0.328409|0.327107|0.299364|0.299866|0.913353 [0.911001,0.915574], 15/15|1.001502 [1.000233,1.002883], 3/15|
|G1 / 16|0.405735|0.404613|0.385206|0.381960|0.936592 [0.931193,0.945354], 15/15|0.994782 [0.978322,1.002751], 11/15|
|G1 / 32|0.591085|0.593159|0.585985|0.587638|0.994164 [0.986599,1.006278], 9/15|1.004260 [0.988901,1.020921], 7/15|
|G2 / 1|1.233808|1.231524|1.226364|1.174465|0.952245 [0.950944,0.953144], 15/15|0.957952 [0.955617,0.958476], 15/15|
|G2 / 16|1.498407|1.501283|1.498457|1.445608|0.964344 [0.961387,0.968052], 15/15|0.964088 [0.957746,0.967556], 15/15|
|G2 / 32|2.397183|2.400609|2.408945|2.341547|0.975073 [0.971161,0.978133], 15/15|0.972092 [0.967393,0.977296], 15/15|

G2 32-worker ON/BASE is 0.974609 [0.971008,0.976613], with 15/15 wins. All primary G2 comparisons improve. Equal-log G1/G2 32-worker ON/BASE is **0.985714 [0.974180,0.991646]**, ON/OFF **0.990686 [0.978397,0.996703]**, and ON/REF **0.984183 [0.981782,0.989922]**, each with 13/15 wins. All aggregate gain checks pass.

All six calibrations, seventeen other nonregressions, six RSS guards and twelve diagnostic-quality checks pass. The failed G1 bound exceeds its limit by 0.000921, or 0.0921 percentage points; the bound is an uncertainty limit, not a measured slowdown. G1 source is unchanged from OFF, but different linked executables and a shared host still permit timing differences. The fixed recipe closes without adoption, SDK trial, unchanged rerun or favorable subset promotion. The G2 and aggregate improvements remain evidence for further work.

|32-worker context / arm|Maximum process RSS MiB|Median CPU ms/Window|Minor faults/Window|
|---|---:|---:|---:|
|G1 BASE|25.75|0.599039|0.080882|
|G1 OFF|25.50|0.605781|0.084559|
|G1 ON|25.25|0.606353|0.097426|
|G2 BASE|36.25|2.354645|0.277574|
|G2 OFF|36.25|2.368906|0.246324|
|G2 ON|36.00|2.297265|0.264706|

Major-fault medians are zero. Short process high-water observations include fixtures and mappings; they do not establish minimum RAM, pressure/swap behavior or a full-proof memory saving.

After all latency runs, six separate PMU panels and six separate cycle profiles perform 2,048 waves behind FIFO warm gates. Grouped counters run at 100% scheduled coverage. All profiles pass PID/TID/CPU/executable-MMAP checks, zero LOST and sample/unknown limits. Period is16 million cycles, buffer512 pages. Profiles contain **155,920 samples**, with unknown shares below0.42%.

|Per complete Window, 32 workers|G1 BASE|G1 OFF|G1 ON|G2 BASE|G2 OFF|G2 ON|
|---|---:|---:|---:|---:|---:|---:|
|Instructions|4,665,651|4,769,402|4,768,439|19,239,258|19,158,440|18,874,869|
|User cycles|2,608,146|2,565,418|2,565,931|10,217,146|10,285,361|9,935,278|
|Branch misses|4,812|5,734|5,661|7,274|9,125|9,183|
|Generic cache misses|4,160|3,833|3,735|22,601|25,666|25,260|

The G2 panel uses fewer instructions and cycles than OFF; G1 counters are close. This supports retaining the early-evaluation mechanism, without identifying particular spill, dependency or port costs as the cause. These are single diagnostic panels, not replicated causal estimates or removable full-proof wall-time fractions.

All **443 workflow commands succeed once**: eight native/core, seven consumer/emitted-audit/analysis-tool, 426 capture and two post-analysis commands. Capture contains 24 preflights, 360 latency processes, six PMU panels, six profiles and thirty offline steps. All 396 callers and **906,752 Window invocations** match the independent oracle. Frozen producer and separate same-agent raw reader pass first execution, reconstructing all48 decisions and resource rows; PMU/sample schema parsers are shared. All606 capture pins remain exact. No owned helper/build/hash/analysis/report work overlaps recording. No failures, exclusions or successful scientific replays occur.

No new SDK request, Gnark export, proof, AWS/Aeglos comparison, ARM/mobile, huge-circuit, pressure/swap or minimum-RAM result is added. Canonical proof inventory remains44,265. Accepted local32 full-proof observations and supplied M5 references below are inherited from the [latest SDK campaign](../2026-11-10-frontend-overlap176/README.md).

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.783594|37 / 4.754×|24 / 3.083×|3.7|
|2×2|10.932731|59 / 5.397×|30 / 2.744×|5.9|
|4×2|15.757088|109 / 6.918×|48 / 3.046×|10.9|
|8×2|23.973295|189 / 7.884×|74 / 3.087×|18.9|
|12×2|33.827269|284 / 8.396×|101 / 2.986×|28.4|
|16×2|39.900917|335 / 8.396×|124 / 3.108×|33.5|
|24×2|66.041083|528 / 7.995×|176 / 2.665×|52.8|
|merge-24×1|65.717994|507 / 7.715×|175 / 2.663×|50.7|

The verified CPU10× objective stays active and unmet, **0/8**. Merge54 remains162.170158 ms without a supplied M5 reference. A next distinct hypothesis is to evaluate the two products sharing inverse state together, exposing independent CIOS chains and shared input loads. This is not implemented or measured here; extra live accumulators could make it slower. It requires new arithmetic bounds/independent checks and complete-caller evaluation, with no change to this trial's threshold or rejected status.

[Analysis](artifacts/analysis.json), [raw reader](artifacts/review.json), [decision](artifacts/decision.json), [protocol](artifacts/protocol.md), [inherited field](artifacts/inherited-field.json), [allocation checks](artifacts/allocations.json), [structural checks](artifacts/structural.json), [selected G2 assembly](artifacts/g2-selected.asm), [prospective freeze](artifacts/prospective-review.json), [identities](artifacts/identities.json), [source audit](artifacts/source-audit.json), [commands](artifacts/receipts.json), [symbols](artifacts/symbols.json), [failure record](artifacts/failures.md), [continuation](artifacts/next.md), [public bindings](artifacts/public-views.json), [evidence archive](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Full raw rows/commands/source edits remain in the indexed UTF-8 archive. Private fixtures, binaries, raw perf payloads, unchanged snapshots and proof ledgers are excluded; reproduction needs the pinned private artifacts.
