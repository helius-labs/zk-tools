# Accepted prover dispatch-slot counters

Backend-blocked dispatch slots exceed frontend-empty slots in all three tested cases. Primary32 medians are 41.97% backend, 12.73% frontend and 22.70% SMT-denied. Merge54 medians are 74.16%, 4.64% and 11.12%. These are whole warm-SDK counter ratios, not exclusive bottlenecks, physical-core utilization, removable wall time or a speed prediction. Production and accepted latency results remain unchanged.

## Method

Ryzen 9950X, exact accepted SDK executable and linked libraries. Three rotated/reversed blocks measure three four-event panels on primary32/401 requests, primary16/201 requests on CPUs 0–15, and merge54/17 requests. All 27 fresh processes use three primes. The existing acknowledged FIFO enables perf stat after loading/priming and disables it before export. Measured SDK calls retain solving, fresh randomness, proving, verification and serialization. No owned build, archive, profile or second CPU benchmark overlaps capture. All independent Gnark checks run after the last capture.

All three event panels pass a separately recorded availability check before any SDK request. Every event runs 100% of its enabled time in availability and capture; no multiplexing or unsupported event occurs. Four-event groups share the same runtime within each capture. Counter ratios use events from their own panel.

The Linux [Zen5 decode table](https://raw.githubusercontent.com/torvalds/linux/master/tools/perf/pmu-events/arch/x86/amdzen5/decode.json), [execution table](https://raw.githubusercontent.com/torvalds/linux/master/tools/perf/pmu-events/arch/x86/amdzen5/execution.json), [instruction-cache table](https://raw.githubusercontent.com/torvalds/linux/master/tools/perf/pmu-events/arch/x86/amdzen5/inst-cache.json) and [pipeline metrics](https://raw.githubusercontent.com/torvalds/linux/master/tools/perf/pmu-events/arch/x86/amdzen5/pipeline.json) define the raw events and 8-slot normalization. Retrieved bytes and hashes are retained. No 0x18e instruction-cache tag event is used.

## Dispatch observations

Each cell is the median of three capture ratios. Independent panels do not form one jointly measured decomposition.

| Case | Frontend-empty slots % | Backend-blocked slots % | SMT-denied slots % | Retired macroop slots % | Dispatched minus retired slots % | Instructions/user cycle |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| primary32 | 12.729 | 41.969 | 22.696 | 20.345 | 2.026 | 1.615 |
| primary16 | 11.173 | 56.677 | 0.064 | 27.943 | 4.015 | 2.213 |
| merge54 | 4.640 | 74.155 | 11.125 | 9.738 | 0.361 | 0.781 |

Unused-slot ratios divide counts by8*counted user cycles. Retired/dispatched values count macroops/ops as documented by the source metrics. SMT-denied slots aggregate thread observations and do not establish physical-core idle time or a recoverable speedup. Backend blocking includes multiple resource/dependency limits; this experiment does not isolate stack spills, loads or field-multiply throughput.

| Case | At least8 frontend-empty slots: cycle % | I-cache lines filled from L2 /1000user cycles | I-cache lines filled from system /1000user cycles |
| --- | ---: | ---: | ---: |
| primary32 | 6.849 | 11.873845 | 0.351843 |
| primary16 | 5.401 | 11.215061 | 0.387505 |
| merge54 | 2.476 | 1.200143 | 0.148111 |

The cmask8 event counts cycles with at least eight empty frontend slots. Linux derives its frontend-latency metric from this condition. It is not a direct instruction-cache-stall timer. Instruction-cache fills count 64-byte lines, not cache accesses or a miss rate; system fills can come from another cache as well as RAM. No instruction-per-panel conversion is made for the fetch panel, which does not count retired instructions. The data provides no loop-specific attribution.

## Request accounting

The following are diagnostic measurements under perf, not replacement latency qualification. Each cell is the median of three process observations; resource fields are warm user+system CPU ms/request.

| Case | Slots panel request/CPU ms | Ops panel request/CPU ms | Fetch panel request/CPU ms |
| --- | ---: | ---: | ---: |
| primary32 | 8.600728 / 198.937 | 8.616087 / 198.866 | 8.618612 / 198.913 |
| primary16 | 10.844982 / 129.539 | 11.051222 / 130.467 | 10.976551 / 129.831 |
| merge54 | 177.104968 / 4853.044 | 175.398740 / 4839.135 | 174.732700 / 4803.233 |

## Decision and validation

The measurements weaken an explanation centered on frontend delivery alone. They support assessing dataflow/resource costs in actual arithmetic callers, while preserving the failed PGO, machine-splitting, G1 finish-phase and field-component results. They do not justify attributing the backend count entirely to register pressure, retesting rejected scheduler modes unchanged, or predicting a twofold speedup.

A proposed next design consumes full owned G2 batches: overwrite used prefix/norm slots with slope components, store original bucket x in dead incoming batch-x storage, then finish y in a separate pass. It reuses existing records and batch capacity, preserving arithmetic and key data. Extra stores and indexed passes may lose. This is unimplemented and requires caller ownership auditing, independent curve tests and whole-SDK timing; see [design notes](artifacts/next-design.md). It is distinct from the rejected G1 finish-phase experiment because G2's two-field prefix records lack the old-x slot and the proposal consumes the incoming batch to supply it.

All 5,652 SDK calls verify internally: 5,571 captured and 81 primes. All 27 distinct final exports independently pass Gnark. Their 54 proof/public files are hash-checked; proofs are 164 bytes, public hashes match accepted fixtures, and proof identities are disjoint from 4,804 prior identities. All 697 initial source/library/key/image/verifier/table pins match after execution. There is no production rebuild or source change.

The first analysis expected Gnark's entire stdout to be its summary. Debug and per-proof lines made that assertion fail after captures had already succeeded. The parser now checks the final summary line; successful command receipts, proof/public sizes, hashes, fixture binding and freshness remain mandatory. Initial parser source and the repair note are retained. No request, capture or verification was repeated.

No new Aeglos, AWS, portable, phone, memory-pressure, concurrency or huge-circuit measurement is claimed. Whole-process RSS in raw summaries includes loading/primes and does not establish minimum RAM. Accepted historical primary is 8.594267 ms and merge54 is 178.891799 ms. The unchanged [all-shape M5 CPU/Metal comparisons](../2026-10-25-grouping-module/artifacts/perf/report.md) remain authoritative; all eight CPU 10× targets remain unmet.

[Protocol](artifacts/experiment.json), [all observations](artifacts/rows.json), [analysis](artifacts/analysis.json), [source-defined events](artifacts/event-definitions.json), [availability](artifacts/availability-check.json), [capture order](artifacts/capture-order.json), [proof inventory](artifacts/proof-inventory.json), [closing pins](artifacts/closeout-pins.json), [artifact index](artifacts/artifact-index.json).
