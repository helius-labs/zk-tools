# Deferred bucket Y conversion

**Deferring conversion improves G2 at 32 workers by about 1.3%, but slows G1 by about 5.4%; the complete candidate fails 9/48 checks and is not adopted.** The equal-log G1/G2 result regresses against all three controls. All six calibrations, six RSS checks and twelve diagnostic-quality checks pass. Accepted local 1×2 full-proof latency remains **7.783594 ms at 32 workers**; the 16-worker observation is **9.016864 ms**.

The change stores original canonical Y with a format bit in the existing opaque bucket payload and delays modular quarter conversion until vector loading. Computed quarter-Y remains untagged and may be lazy below 2p. Since 2p<2²⁵⁵, the format bit does not enlarge the bucket. Loads clear it before arithmetic and blend converted lanes; scalar zero checks, tails and decoding honor both formats. Keys and solver plans are unchanged. There is no lookup table, bitmap, new pool or additional bucket allocation.

Generic raw packing now returns untyped words: numerical Wide3/Wide4 intermediates may use bit255 as data. Only proven <2p output becomes untagged Stored. Tagged or lazy words never enter canonical Fq. The arithmetic input bounds, queue, scheduling and normalization algorithms are otherwise unchanged. Three private Rust children change; all edits reverse exactly to the preceding 181-member snapshot, with 178 members byte-identical. All 176 production manifest members remain exact.

Qualification checks **4,610 scalar cases**, **131,072 mixed-format lanes** across every tag mask and 64 changing groups per mask, plus **1,024 raw Wide3/Wide4 roundtrips** including numerical bit255. Independent BigUint comparisons cover conversion, decoding, canonicalization and bounds. Four existing tests pass on the new coherent libraries: 1,695,744 independent Arkworks prefix point comparisons, 5,018,112 bucket comparisons and 220 threaded Windows, including 1/16/32 workers and twelve callers sharing one eight-worker pool. Portable fallback evidence is inherited from unchanged native-module guards and inactive paths.

Twenty-four conversion cases preserve pointer and capacity without allocation, reallocation or deallocation. Complete single-Window requested-live peaks are 191,360 bytes for G1 and 303,360 bytes for G2, equal to the original path in this allocation panel. Bucket sizes remain 64/128 bytes; record sizes remain 1,600/640 bytes. These counters delegate to the production allocator and do not measure allocator overhead, stack, RSS or minimum RAM.

|Native emitted probe|Previous quarter codec|Deferred conversion|
|---|---:|---:|
|Insertion instructions|63|6|
|Load/export probe instructions|57|103|
|Load conditional jumps|0|1|
|Calls in either probe|0|0|

The load probe includes exporting the loaded numeric words. Counts describe static emitted code, not dynamic cost or proof speed. The cheaper insertion moves work to another path.

The four arms retain the same design as the [previous trial](../2026-11-10-scalar-quarter-codec176/README.md): REF is the retained API-only accepted-source build; BASE is the original Window path in the new coherent build; OFF is the retained fixed-mask quarter candidate; ON is deferred conversion. BASE and ON share an executable. BASE/REF calibrates equivalent algorithms; OFF is a mechanism control and intentionally differs from the accepted baseline. Driver selectors are frozen separately from arm labels.

Ryzen 9950X, 16 physical/32 logical CPUs, shared host. Six contexts use 15,045 points, width12, column0, batch200 and 1/16/32 workers. Worker i is pinned to CPU i; solo uses CPU2; stacks request 512 KiB. Unchanged independent-Arkworks fixture bytes are reused. Every process performs three checked primes and seventeen timed waves. Each wave includes fresh Window construction, feeds, queues, scalar paths, normalization, complete output comparison, checksum and drop. Loading, pool setup and input hashing are excluded. **A wave runs one Window per worker; these are not complete MSM, proof or server-throughput times.**

Fifteen fixed rotated/reversed context/arm blocks are retained. Displayed times are medians of process medians. Ratios are paired by block and need not equal ratios of displayed times. Bounds are fixed ranks4–12 of15: nominal 96.484375% individual median coverage under independent exchangeable blocks, without a familywise guarantee.

|Context|REF ms|BASE ms|OFF ms|ON ms|ON/REF [bounds], wins|ON/OFF [bounds], wins|
|---|---:|---:|---:|---:|---|---|
|G1 / 1|0.328239|0.327588|0.307660|0.309985|0.945660 [0.941924,0.948166], 15/15|1.006136 [1.004651,1.010803], 1/15|
|G1 / 16|0.406326|0.402609|0.390145|0.421565|1.030636 [1.023727,1.043089], 1/15|1.073464 [1.066371,1.081330], 0/15|
|G1 / 32|0.591254|0.594300|0.595172|0.624076|1.054310 [1.043880,1.070642], 1/15|1.053518 [1.039337,1.069615], 0/15|
|G2 / 1|1.230209|1.229047|1.220381|1.207456|0.981021 [0.980295,0.981709], 15/15|0.988831 [0.984945,0.990684], 15/15|
|G2 / 16|1.498033|1.498735|1.498324|1.489668|0.994496 [0.981474,1.001703], 10/15|0.994260 [0.986435,0.997597], 12/15|
|G2 / 32|2.395906|2.399854|2.395776|2.360930|0.986843 [0.980690,0.991552], 15/15|0.987283 [0.981834,0.996961], 13/15|

G2 32-worker ON/BASE is 0.983414 [0.979068,0.993291], 15/15 wins; all three primary G2 gain checks pass. Equal-log G1/G2 32-worker ON/BASE is **1.022856 [1.010626,1.026423]**, 1/15 wins; ON/OFF **1.018250 [1.012658,1.028555]**, 0/15; ON/REF **1.021052 [1.015094,1.031801]**, 1/15. The nine failures are G1 16/32-worker nonregression against all three controls, and all three aggregate gains. No favorable group, worker or parameter subset is promoted. The fixed recipe closes without an unchanged rerun or SDK trial.

|32-worker context / arm|Maximum process RSS MiB|Median CPU ms/Window|Minor faults/Window|
|---|---:|---:|---:|
|G1 BASE|25.25|0.598239|0.086397|
|G1 OFF|25.75|0.604296|0.077206|
|G1 ON|25.50|0.636858|0.069853|
|G2 BASE|36.00|2.361711|0.272059|
|G2 OFF|36.25|2.360068|0.248162|
|G2 ON|36.00|2.322248|0.270221|

Major-fault medians are zero. Short process high-water observations include fixtures and mappings; they establish neither minimum RAM nor pressure/swap behavior or a full-proof memory saving.

After all latency runs, six separate PMU panels and six separate cycle profiles run 2,048 waves each behind FIFO warm gates. All grouped counters report 100% scheduled coverage. Profiles use a 16-million-cycle period and 512 pages; all pass PID/TID/CPU/executable-MMAP checks, zero LOST and sample/unknown limits. They contain **156,848 samples**, with unknown shares below 0.46%.

|Per complete Window, 32 workers|G1 BASE|G1 OFF|G1 ON|G2 BASE|G2 OFF|G2 ON|
|---|---:|---:|---:|---:|---:|---:|
|Instructions|4,681,093|4,804,069|4,813,101|19,222,377|19,341,212|19,225,966|
|User cycles|2,613,547|2,616,625|2,755,979|10,236,890|10,136,002|10,029,561|
|Branch misses|4,724|5,723|6,329|7,241|9,223|9,624|
|Generic cache misses|4,341|3,821|4,564|21,942|25,588|24,417|

The complete G1 caller executes slightly more instructions and substantially more cycles than OFF; G2 executes fewer instructions and cycles. This supports testing conversion fusion instead of inferring speed from insertion alone. These are single diagnostic panels, not replicated causal estimates or identification of a particular cache/DRAM bottleneck.

All **426 capture commands succeed once**, with **396 caller processes and 906,752 oracle-checked Window invocations**. Frozen producer and separate same-agent raw reader pass first execution, reconstructing all48 decisions and resource rows; PMU/sample schema parsers are shared. All610 capture pins remain exact. Across native, corrected allocation, consumer/tool, capture and post workflows, **450/451 commands succeed**. The failed allocation execution omitted the required environment and was retried alone with that environment; successful builds/tests were retained. One earlier setup preparer also failed outside these workflow receipts by reading a binary fixture as UTF-8, and was corrected to copy unchanged bytes. Both failures are retained. Recording was serialized with no owned helper/build/report work overlapping it.

No new SDK request, Gnark export, proof, AWS/Aeglos comparison, ARM/mobile measurement, huge-circuit or pressure result is added. Canonical proof inventory remains44,265. Accepted local32 full-proof observations and supplied M5 references below are inherited from the [latest SDK campaign](../2026-11-10-frontend-overlap176/README.md).

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.783594|37 /4.754×|24 /3.083×|3.7|
|2×2|10.932731|59 /5.397×|30 /2.744×|5.9|
|4×2|15.757088|109 /6.918×|48 /3.046×|10.9|
|8×2|23.973295|189 /7.884×|74 /3.087×|18.9|
|12×2|33.827269|284 /8.396×|101 /2.986×|28.4|
|16×2|39.900917|335 /8.396×|124 /3.108×|33.5|
|24×2|66.041083|528 /7.995×|176 /2.665×|52.8|
|merge-24×1|65.717994|507 /7.715×|175 /2.663×|50.7|

Full verified CPU10× goal remains active and unmet, **0/8**. Merge54 remains162.170158 ms without a supplied M5 reference. The next distinct hypothesis is to combine original/converted Y selection, signed difference and modular quartering into one bounded vector operation, eliminating the separate conditional conversion. It is proposed and unmeasured; all correction, carry and scalar costs still require qualification and complete-caller pricing.

[Analysis](artifacts/analysis.json), [raw reader](artifacts/review.json), [decision](artifacts/decision.json), [protocol](artifacts/protocol.md), [format qualification](artifacts/format-qualification.json), [allocation qualification](artifacts/allocations.json), [structural qualification](artifacts/structural.json), [format methods](artifacts/format.rs), [vector load](artifacts/load.rs), [prospective freeze](artifacts/prospective-review.json), [identities](artifacts/identities.json), [source audit](artifacts/source-audit.json), [commands](artifacts/receipts.json), [symbols](artifacts/symbols.json), [failures](artifacts/failures.md), [continuation](artifacts/next.md), [public bindings](artifacts/public-views.json), [evidence archive](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Raw rows, full commands and source edits remain in the indexed UTF-8 archive; private fixtures, binaries, raw perf payloads, unchanged snapshots and full proof ledgers are excluded. Reproduction requires the pinned private artifacts.
