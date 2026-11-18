# Why the 96-core 1×2 times diverge on C7a

Aeglos’s MSM stage accounted for 4.48 ms of savings when moving from 32 to 96 cores. Its complete 1×2 proof saved 4.09 ms because its witness solve and several other stages became slower. Zopro’s complete proof took 1.60 ms longer on the same machine. Zopro’s 1.03 ms lead at 32 cores therefore became a 4.66 ms deficit at 96 cores.

These are new additive calculations from the unprofiled requests of the [matched accepted-build run](../2026-11-18-c7a-scaling245/README.md). All three blocks and all 251 timed requests per process enter arithmetic means, allowing the stage changes to add up. The original published medians use a different statistic.

|1×2 stage or engine|32 cores, ms|96 cores, ms|Saving, ms|
|---|---:|---:|---:|
|Aeglos witness|1.277|1.614|−0.337|
|Aeglos scalar preparation|0.003|0.003|<0.001|
|Aeglos FFT|0.898|0.857|0.041|
|Aeglos MSM|8.322|3.847|4.475|
|Aeglos assembly|0.271|0.302|−0.031|
|Aeglos residual inside engine|0.019|0.018|0.002|
|Aeglos outside engine|1.141|1.202|−0.060|
|**Aeglos verified request**|**11.931**|**7.841**|**4.090**|
|**Zopro verified request**|**10.901**|**12.502**|**−1.601**|

## Does this generalize?

The complete [catalog capture](../2026-11-18-full-catalog-c7a244/README.md) supplies a second, independent run. MSM was Aeglos’s largest source of mean elapsed-time savings in every one of the 41 shapes. It sped up in **369 of 369 paired blocks**. Some larger shapes still favor Zopro at 96 cores. Scaling and absolute latency answer different questions.

|Shape|Aeglos MSM 32 ms|Aeglos MSM 96 ms|Aeglos full saving ms|Zopro full change 96−32 ms|96-core Zopro−Aeglos ms|
|---|---:|---:|---:|---:|---:|
|12x2|66.86|35.07|37.37|0.11|-12.94|
|16x2|81.88|42.09|46.28|0.32|-15.08|
|1x2|8.65|4.29|3.84|1.81|4.34|
|24x2|130.50|63.55|83.31|-3.46|-30.50|
|2x2|14.96|7.18|7.64|2.50|5.16|
|4x2|24.44|12.10|12.48|1.79|3.06|
|8x2|43.55|22.83|23.62|2.36|-2.01|
|merge-24x1|126.44|61.52|81.43|-0.77|-22.49|
|merge-54x1|266.86|127.02|174.72|-17.07|-48.16|

The Rust reader checked original stdout hashes against successful process receipts, sample ordinals, request counts, warm cache hits and per-request stage accounting. The catalog analysis covered 10,332 timed samples over both engines, alongside 3,012 primary samples in the focused run. Both original runs had already independently verified their exported proofs. This analysis made zero new proof calls.

## What the mechanism evidence supports

In the previous dedicated [C7a phase study](../2026-11-15-msm-c7a180/README.md), Aeglos Sorted’s instrumented bucket accumulation fell from 6.90 to 2.98 ms. It dispatched 1,280 and 2,560 accumulation items, respectively, and performed zero merge passes. Its Windowed path used the same worker pool but scaled poorly. The fresh instrumented Sorted build missed one 96-core equivalence bound against the retained binary, so those substage times describe that build. Current matched uninstrumented MSM times above independently show the direction and size of the overall MSM change.

In that earlier Zopro build, solver, preparation and FFT together became 1.94 ms slower at 96 cores. The query stage improved only 0.37 ms. A recent 96-core profile of the current accepted build attributes around 56% of sampled cycles to Crossbeam epoch bookkeeping, compared with about 18% of sampled instructions. Summed CPU time rose from 230 to 698 ms per request while wall time increased. This supports investigating how workers obtain useful work during each phase. The profiles cannot assign 56% of elapsed latency to bookkeeping, distinguish an empty ready queue from a worker looking in the wrong place, or isolate DRAM bandwidth. The earlier [worker-search study](../2026-11-12-search-overlap178/README.md) also observed search during intervals with no pending jobs under its instrumented workload.

The [new solver comparison](../2026-11-18-counted-ready-c7a248/README.md) lowered 96-core 1×2 latency by a paired 1.80% against its same-binary control and raised CPU time 3.80%. Its corrected construction reservation has a clear memory-accounting benefit. An improvement of that size leaves the primary MSM scaling gap open.

[Per-shape and per-stage figures](artifacts/analysis.json.gz) · [Reader](artifacts/analyze.rs)

## Three hot instruction addresses

Mapping the stored samples to the exact executable narrows the epoch observation. In each 96-core recording, 46–48% of **all** cycle samples landed on three instructions. In each 32-core recording the same three accounted for approximately 7–8%. The 32-core profiler exceeded its observer tolerance, so this contrast is descriptive. Each independently backed executable had the same hash. The reader checks each decoded stream hash, its executable mapping and the ELF segment offset.

|Instruction address|What precedes it in the executable|96-core share of all cycle samples across the three blocks|
|---|---|---:|
|0x114c81|A locked compare-and-exchange publishes the current participant’s pinned epoch|19.5–20.4%|
|0x108a67|The epoch-advance traversal loads a participant’s epoch|14.1–14.8%|
|0x114c72|The pin path loads the current global epoch|12.2–12.8%|

These are sampled instruction pointers. Imprecise PMU sampling can place a sample after the instruction that incurred a stall. The clustering identifies participant publication, global-epoch reading and participant scanning as sites to investigate. It does not identify a cache level, prove false sharing, or quantify latency recoverable by changing reclamation.

[Address counts](artifacts/ips.json) · [ELF and sample reader](artifacts/epoch_ips.rs) · [Pin disassembly](artifacts/pin.disasm) · [Advance disassembly](artifacts/advance.disasm)
