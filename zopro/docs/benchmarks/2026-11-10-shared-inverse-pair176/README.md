# Shared-inverse R260 Montgomery products

**Interleaving the two shared-inverse products adds no established 32-worker G2 gain over early factor evaluation. The complete candidate fails 4/48 checks and is not adopted.** G2 ON/OFF is 0.999404 [0.996552,1.005104], with 9/15 wins. Solo and 16-worker G2 improve modestly; those results are retained without promoting a subset. Accepted local 1×2 full-proof latency remains **7.783594 ms at 32 workers**, and **9.016864 ms at 16 workers**.

The native private `Wide2::mul_pair` interleaves two independent five-row R260 CIOS products sharing inverse state. Each stream retains the ordinary coefficient recurrence, cancellation carry and normalization. G2's early reverse group uses the pair for its factor and next inverse. Inputs and outputs stay below 2p; REDC is below p+4p²/2²⁶⁰<2p. All five cancellation rows use 52 bits. This adds no lookup table, retained operand, key copy, solver plan, heap/record buffer, new layout or pool. Two six-vector accumulators can increase register pressure.

Prior [dual-CIOS work](../2026-10-25-dual-cios-component/README.md) selected interleaving only for native eight-lane R256; its R260 cases retained ordinary multiplication. Its isolated gain did not become a qualified [full-SDK improvement](../2026-10-25-dual-cios-sdk/README.md). This trial applies the independent-stream schedule to R260 and the new early factor/inverse pair; it does not import the earlier speed estimates.

New native qualification passes **297,536 output comparisons** against both exact old representatives and independent BigUint REDC quotient/field-modulus oracles. It covers 102 boundary values, all 10,404 boundary-pair groups, 4,096 random groups and 4,096 output-fed groups, with two outputs of eight lanes per group. It checks lazy values through 2p−1, limb/carry boundaries, output bounds and input preservation. Field and core changes reverse exactly to the 181-member parent snapshot; 179 members are byte-identical. G1, generic paths and native guards are unchanged. All 176 production manifest members remain exact.

Four tests pass on new coherent field/core libraries: 1,695,744 independent Arkworks prefix point comparisons, 5,018,112 bucket comparisons and 220 threaded Windows, including 1/16/32 workers and twelve callers through one eight-worker pool. Twenty-four conversion cases preserve pointer/capacity without allocation, reallocation or deallocation. Complete single-Window requested-live peaks remain 191,360/303,360 bytes for G1/G2. Buckets remain 64/128 bytes and records 1,600/640 bytes. Portable fallback evidence is inherited from unchanged guards and inactive code; no ARM execution is added.

|Native emitted observation|Separate products / early factor|Interleaved pair|
|---|---:|---:|
|Pair probe instructions|329|319|
|Pair probe IFMAs / calls|200 / 0|200 / 0|
|Pair probe explicit `rsp` sites|12|0|
|Full G2 function bytes|50,359|50,523|
|Full G2 static instructions|8,522|8,523|
|Full G2 prologue `sub rsp` reservation, bytes|6,848|6,528|
|Full G2 explicit `rsp` sites|1,502|1,485|

The probe's smaller instruction count does not transfer directly to the full caller. Static stack sites are not executed spill counts, and prologue reservation is not full call depth or minimum RAM. No arithmetic operation-count reduction is claimed.

Four arms: REF retains the API-only accepted-source build; BASE runs the original Window path in the new coherent build; OFF retains early factor evaluation; ON adds paired products. BASE/ON share an executable. BASE/REF calibrates equivalent original algorithms; OFF is a mechanism control and intentionally differs from the accepted baseline. Driver selectors and labels are frozen separately.

Ryzen 9950X, 16 physical/32 logical CPUs, shared host. Six contexts use G1/G2, 15,045 points, width12, column0, batch200 and 1/16/32 workers. Worker i pins to CPU i; solo uses CPU2; stacks request 512 KiB. Independent-Arkworks fixture bytes are reused. Three checked primes precede seventeen timed waves. Fresh Window construction, feeds, queues, scalar paths, normalization, complete output comparison, checksum and drop are timed; input loading/hashing and pool setup are excluded. **A wave performs one Window per worker, not a complete MSM, proof or server-throughput measurement.**

Fifteen fixed rotated/reversed context/arm blocks are retained. Times are medians of process medians; paired ratios need not equal ratios of displayed times. Bounds are fixed ranks4–12 of15, with nominal 96.484375% individual median coverage under independent exchangeable blocks, without a familywise guarantee.

|Context|REF ms|BASE ms|OFF ms|ON ms|ON/REF [bounds], wins|ON/OFF [bounds], wins|
|---|---:|---:|---:|---:|---|---|
|G1-w1|0.328390|0.327568|0.299876|0.300197|0.913887 [0.911782,0.915989], 15/15|1.001608 [0.999093,1.003175], 5/15|
|G1-w16|0.405625|0.400405|0.383192|0.383243|0.951592 [0.941888,0.953508], 15/15|1.000260 [0.990815,1.013366], 7/15|
|G1-w32|0.599961|0.592898|0.589221|0.586265|0.975910 [0.971862,0.995259], 12/15|0.999372 [0.988111,1.021289], 8/15|
|G2-w1|1.232274|1.229609|1.175467|1.171068|0.949226 [0.948596,0.950771], 15/15|0.995190 [0.994452,0.998615], 14/15|
|G2-w16|1.497274|1.498847|1.447180|1.443091|0.962812 [0.958761,0.966231], 15/15|0.995666 [0.994018,0.999277], 12/15|
|G2-w32|2.397781|2.393463|2.346786|2.348619|0.976338 [0.972495,0.982523], 14/15|0.999404 [0.996552,1.005104], 9/15|

G2 at32 workers still improves against REF and BASE, but that includes the inherited quarter/fusion/early-factor changes. The paired routine alone does not establish a gain against OFF. Equal-log G1/G2 32-worker ratios:

- ON/BASE: 0.984617 [0.977026,0.988929], 12/15.
- ON/OFF: 0.995670 [0.993932,1.010479], 9/15.
- ON/REF: 0.981092 [0.975691,0.987919], 13/15.

Four failures are one G1 calibration, one G1 nonregression, primary G2 ON/OFF gain and aggregate ON/OFF gain. G1-w32 BASE/REF is0.993378 [0.966922,1.013385], outside factor1.03 at its lower bound; this limits attributing that context's differences solely to the new arithmetic. G1 ON/OFF is0.999372 [0.988111,1.021289], so its bound fails the1.02 limit without establishing a slowdown. Five other calibrations, seventeen nonregressions, six RSS guards and all twelve diagnostic-quality checks pass. No threshold, exclusion, block extension or unchanged rerun follows. The fixed recipe closes without SDK trial or adoption.

|32-worker context / arm|Maximum process RSS MiB|Median CPU ms/Window|Minor faults/Window|
|---|---:|---:|---:|
|G1-w32/BASE|26.00|0.599324|0.084559|
|G1-w32/OFF|25.75|0.602675|0.086397|
|G1-w32/ON|25.75|0.603798|0.082721|
|G2-w32/BASE|35.75|2.351092|0.277574|
|G2-w32/OFF|36.00|2.305934|0.270221|
|G2-w32/ON|36.00|2.308449|0.272059|

Major-fault medians are zero. Short process high-water measurements include fixtures/mappings and establish neither minimum RAM nor pressure/swap behavior or full-proof savings.

After all latency runs, six separate PMU panels and six separate profiles perform2,048waves behind FIFO warm gates. Grouped counters run at100% scheduled coverage. Profiles use period16million cycles and512pages; all pass PID/TID/CPU/executable-MMAP, zeroLOST and sample/unknown limits. They contain **154522 samples**, with unknown shares below0.43%.

|Per complete Window,32workers|G1 BASE|G1 OFF|G1 ON|G2 BASE|G2 OFF|G2 ON|
|---|---:|---:|---:|---:|---:|---:|
|Instructions|4668678|4770801|4768023|19231418|18874578|18822994|
|User cycles|2617055|2571220|2557052|10224603|9918053|9979655|
|Branch misses|4743|5670|5710|7305|9194|9173|
|Generic cache misses|4187|3737|3717|23474|24675|23278|

G2 has slightly fewer instructions but more cycles than OFF in this panel. These are separate single diagnostics, not replicated causal estimates, exclusive instruction costs or identification of a spill/cache/port bottleneck.

All426capture commands succeed once:24preflights,360latency processes,6PMU panels,6profiles and30offline steps. All396callers perform906,752oracle-checked Windows. Frozen producer and separate same-agent raw reader pass first execution, reconstructing all48 decisions/resources; PMU/sample schema parsers are shared. All617capture pins remain exact. Across native, repaired continuation, consumer/tool, capture and post workflows, **450/451 commands succeed**. The failed oracle build needed only an explicit usize shift-index annotation; its source and failure remain, and the three successful field/probe builds are reused. No successful scientific step is replayed and no owned helper/build/hash/analysis/report overlaps recording.

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

The full verified CPU10× goal remains active and unmet, **0/8**. Merge54 remains162.170158ms without a supplied M5 reference. Next: revisit complete-query operation counts and memory-bounded MSM strategies before choosing another arithmetic micro-optimization. New algorithmic work must retain the original full-proof/all-shape goal and be checked against prior experiments.

[Analysis](artifacts/analysis.json), [raw reader](artifacts/review.json), [decision](artifacts/decision.json), [protocol](artifacts/protocol.md), [arithmetic](artifacts/arithmetic.json), [pair routine](artifacts/pair.rs), [allocations](artifacts/allocations.json), [structural checks](artifacts/structural.json), [selected G2 assembly](artifacts/g2-selected.asm), [prospective freeze](artifacts/prospective-review.json), [identities](artifacts/identities.json), [source audit](artifacts/source-audit.json), [commands](artifacts/receipts.json), [symbols](artifacts/symbols.json), [failure record](artifacts/failures.md), [continuation](artifacts/next.md), [public bindings](artifacts/public-views.json), [evidence archive](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Raw rows/full commands/source edits remain in the indexed UTF-8 archive. Private fixtures, binaries, raw perf payloads, unchanged snapshots and proof ledgers are excluded; reproduction needs the pinned private artifacts.
