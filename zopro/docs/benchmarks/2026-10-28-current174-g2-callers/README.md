# G2 preparation before raw MSM

The G2 caller starts within about four microseconds of the shared join. Its grouping attempt accounts for most of the previously observed delay before raw MSM: **1.32–1.62 ms at32workers** across three process medians. No grouped-point reduction runs before raw MSM in these requests; the caller takes the retained ungrouped path. This localizes the interval to optional grouping preparation rather than late task dispatch. It does not establish a specific hash/allocator/syscall cause or a speedup from removing it.

The same adopted174-source SDK81033 runs without rebuilding. Three fixed alternating blocks use primary16/32,3primes+31requests each. Coarse probes record every entry/return; cycles sample at199Hz inside acknowledged warm-loop FIFO gates. Timings are instrumented diagnostics, not a replacement for accepted8.366931ms.16-worker variation remains visible. No source, dependency, global kernel policy or clock setting changes; all16owned probes are removed.

| Process | SDK ms | Join→G2 caller ms | Caller→raw ms | Grouping ms | Admission ms (inside grouping) | Headroom ms (inside admission) | Raw ms |
|---|---:|---:|---:|---:|---:|---:|---:|
| b0-1x2-w16 | 15.030602 | 0.003026 | 0.776525 | 0.773158 | 0.174710 | 0.082476 | 10.978638 |
| b0-1x2-w32 | 8.464025 | 0.003607 | 1.323536 | 1.319249 | 0.281822 | 0.116670 | 4.108109 |
| b1-1x2-w16 | 15.118388 | 0.003105 | 1.090868 | 1.087631 | 0.189607 | 0.076855 | 11.249921 |
| b1-1x2-w32 | 8.253779 | 0.003827 | 1.434436 | 1.430398 | 0.275971 | 0.117141 | 3.878285 |
| b2-1x2-w16 | 11.265920 | 0.002806 | 0.803897 | 0.801471 | 0.170421 | 0.074891 | 7.821313 |
| b2-1x2-w32 | 8.483231 | 0.003957 | 1.627229 | 1.622912 | 0.283585 | 0.143831 | 3.924803 |

Memory admission accounts for about0.276–0.284ms at32workers; headroom about0.117–0.144ms. These nested values must not be added to grouping. Shared mutex waiting, user execution and kernel work are not separated by elapsed spans. Thread-matched cycle samples inside grouping are sparse (zero or one per process), so they do not provide a calibrated CPU-time estimate or exclusive cause. Grouping returns without a grouped point array; this tracer does not expose its return reason and therefore does not prove a full hash-map build or cardinality rejection occurred.

All204SDK calls verify internally;6fresh164-byte exports independently pass Gnark, fixture public hashes and uniqueness beyond15053prior identities. Ledger15059. Six recordings contain9210cyclesamples, onePID each, no lost records, and validated16traceperiod1+cyclesfreq199+metadata-dummy attributes. Existing4parser tests plus2caller tests passed before capture; an added seventh test qualifies a post-capture scope correction. Raw MSM itself invokes the same point-grouping symbol for bucket reduction, so caller preparation now includes only completed calls before raw entry on the caller TID. The original analyzer failure/source remain; recordings, verification and successful capture helpers were not repeated or changed. All original pins pass.

Next: compare the existing ZOPRO_CPU_DEDUP=1 and0 settings on this exact SDK in a fixed unprofiled protocol. This changes optional grouping across all participating queries and also its workspace forecast; it cannot isolate only B2 or justify a global default change from a primary result. Retain existing B-first, grouping-prefix, bitmap, borrowed-map and parallel-grouping negative results. No new optimization is adopted here.

Inherited all-eight results from the accepted source follow, compared with supplied historical M5 references. This study does not remeasure other shapes or AWS/Aeglos. No new mobile/ARM/TEE, giant, minimum-RAM, swap or pressure qualification. The original10×CPU goal remains active and unmet0/8.

| Shape | Accepted ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |
|---|---:|---:|---:|---:|---:|
| 12x2 | 34.858408 | 284.0 | 8.147× | 101.0 | 2.897× |
| 16x2 | 41.377043 | 335.0 | 8.096× | 124.0 | 2.997× |
| 1x2 | 8.366931 | 37.0 | 4.422× | 24.0 | 2.868× |
| 24x2 | 67.764001 | 528.0 | 7.792× | 176.0 | 2.597× |
| 2x2 | 11.539335 | 59.0 | 5.113× | 30.0 | 2.600× |
| 4x2 | 16.540233 | 109.0 | 6.590× | 48.0 | 2.902× |
| 8x2 | 25.136722 | 189.0 | 7.519× | 74.0 | 2.944× |
| merge-24x1 | 67.096516 | 507.0 | 7.556× | 175.0 | 2.608× |

[Protocol](artifacts/design.md), [every request and result](artifacts/analysis.json), [captured observer](artifacts/run.rs), [scope-corrected analyzer](artifacts/analyze.rs), [source pins](artifacts/pins.json), [probes](artifacts/probes.json), [cleanup](artifacts/cleanup.json). Raw perf data stay local with hashes.
