# G2 tree scaling on C7a

Both trees pass the declared C7a component gate, and stream calibration passes for every case. The prefix tree reduces primary G2 latency by 10.22% at 64 workers and 16.23% at 96 workers against the fresh stream, with seven of seven paired block wins against both stream controls at each count. No candidate is adopted. Raw G2 query latency excludes the rest of proving and cannot establish a whole-proof or M5 speedup. The [negative local result](../2026-10-26-g2-prefix-tree/README.md) remains unchanged.

The best tested primary stream is still faster: 4.464 ms at 32 workers versus the prefix tree's 4.595 ms at 64 workers. These are descriptive medians across different configurations. The tree improves scaling at high worker counts without establishing a new best primary component latency. For 2x2, the best tested prefix median is 6.325 ms at 64 workers versus the stream's 7.104 ms at 32 workers; this comparison is also descriptive. Both trees require more scratch and summed CPU time.

The hypothesis was that roughly22long stream fill jobs may limit high-core scaling while grouped trees expose more independent leaves. Both source snapshots and drivers are unchanged from their local studies, rebuilt for `znver4`. The old-source stream is a calibration control; fresh stream, generic tree and prefix tree share the new executable. This comparison changes target and available physical cores, with no new algorithm or key lookup table.

Dedicated c7a.24xlarge,96physical cores without SMT,30GiB container. Two actual fixture B2 queries; workers16/32/64/96 use compact firstN CPU masks. Seven rotated/reversed fresh-process blocks,17timed calls after2primes and1reference setup. All224latency processes finish before six199Hz gated profiles. Preparation/loading,witness solving,reference setup,scalar grouping,other queries,blinding,verification and serialization are excluded. Scalar clone,admission,decomposition,routing,allocations,all reductions,normalization,equality and temporary drops are timed. Allocator atomics are present in both drivers.

## Complete component results

Times are medians of process medians. Ratios use paired within-block geometric means and need not equal displayed median ratios. All observations remain included.

|Case|Old-source stream ms|Fresh stream ms|Generic tree ms|Prefix tree ms|Generic/fresh|Prefix/fresh|
|---|---:|---:|---:|---:|---:|---:|
|1x2-w16|7.124020|7.123510|7.896813|7.022339|1.108819|0.984579|
|1x2-w32|4.494419|4.463710|5.081261|4.685540|1.156333|1.061044|
|1x2-w64|5.152702|5.102303|4.838162|4.594779|0.942096|0.897831|
|1x2-w96|5.731314|5.649554|4.915912|4.743360|0.877987|0.837737|
|2x2-w16|11.716800|11.711970|12.119721|10.805598|1.034477|0.923247|
|2x2-w32|7.084101|7.104399|7.572872|7.005370|1.087115|0.982285|
|2x2-w64|7.985093|7.965735|7.061080|6.324996|0.878208|0.794288|
|2x2-w96|8.517296|8.425826|7.193880|6.650729|0.850278|0.783123|

## Memory observations

Requested peak is live caller-layout bytes above prepared inputs. Whole-process RSS includes loading,reference setup and primes; neither is minimum RAM. Forecasts are conservative typed admission bounds.

|Case|Stream / generic / prefix requested peak MiB|Stream / generic / prefix whole-process RSS MiB|
|---|---:|---:|
|1x2-w16|7.636 / 18.494 / 16.820|58.238 / 71.625 / 69.305|
|1x2-w32|9.206 / 28.135 / 25.551|57.992 / 80.398 / 71.348|
|1x2-w64|10.330 / 41.803 / 34.824|57.211 / 118.074 / 101.941|
|1x2-w96|10.778 / 55.102 / 47.054|58.664 / 125.301 / 111.797|
|2x2-w16|8.610 / 26.222 / 23.051|97.859 / 106.938 / 102.480|
|2x2-w32|9.736 / 42.174 / 36.012|91.277 / 123.070 / 115.621|
|2x2-w64|11.682 / 68.853 / 57.846|89.250 / 175.254 / 151.961|
|2x2-w96|13.593 / 93.303 / 75.065|89.676 / 196.289 / 164.984|

## CPU consumption

These are medians of per-process summed user and system CPU milliseconds divided by the 17 timed queries. They include coordination and are not arithmetic operation counts. A shorter request can consume more total CPU when it uses more workers.

|Case|Stream CPU ms/query|Prefix CPU ms/query|
|---|---:|---:|
|1x2-w16|92.275|101.152|
|1x2-w32|104.975|123.524|
|1x2-w64|181.949|210.323|
|1x2-w96|313.028|340.596|
|2x2-w16|144.359|155.867|
|2x2-w32|160.867|182.228|
|2x2-w64|249.535|296.694|
|2x2-w96|393.296|464.352|

At primary96, prefix requested scratch rises from 10.778 to 47.054 MiB and summed CPU rises from 313.028 to 340.596 ms/query. This is a latency/resource tradeoff, not a demonstrated reduction in total CPU work. No universal device default follows from it.

## Qualification

The frozen calibration requires fresh/old-source stream ratios within factor1.05eachblock and factor1.03pairedmedian for all eight cases. Both tree variants are evaluated independently against both stream controls: primary64and96 must improve with at least4/7wins each;2x2at64and96 must remain within1.02. Workers16/32 are descriptive. There is no minimum gain floor or posthoc variant choice. Failed calibration withholds fine timing qualification.

Calibration failures:

- None.

tree speed/nonregression failures:

- None.

prefix speed/nonregression failures:

- None.

The baseline112andcandidate117unfiltered native core tests pass, plus raw2/cubic6andgeneratedIFMA checks for each source. Unchanged field source inherits the pinned33-test `znver4` qualification. Every timed result equals its stream reference,point order agrees with the compressed key,and requested peak stays within the typed bound plus scalar clone. One output hint per shape agrees across all local and remote processes.

Exact-container local preflight runs8processes/32rawqueries before publication. Remote runs230processes/4882rawqueries including all reference setups, for4914combined calls. No SDK/Gnark proofs; prior5958unchanged. All helper binaries are built before capture. No successful capture is excluded or repeated.

## Profiles and machine state

Six primary profiles follow all latency processes: fresh stream,generic,prefix at32and96workers,64timed calls after2primes and1reference,cycles:u at199Hz. Recording is gated after preparation. Direct pinned perf uses `--no-buildid-cache`. Aggregate samples are descriptive,not exclusive-stage CPU or causal wall-time evidence.

|Profile|Samples|Lost records|Sampled process count|
|---|---:|---:|---:|
|w32-prefix|2036|0|1|
|w32-stream|2548|0|1|
|w32-tree|1954|0|1|
|w96-prefix|8785|0|1|
|w96-stream|12110|0|1|
|w96-tree|9249|0|1|

Topology,CPU flags,and cgroup CPU/memory/swap snapshots are retained before the workflow,after latency,and after profiles. These counters cover entire stages and are not attributed to individual queries. The container records no CPU throttling, OOM events or swap use. Profiles and their raw hashes are retained locally; decoded reports are archived. All six profiles contain 36,682 samples in total, with zero lost records and one sampled process each.

At96 workers, the two prominent epoch symbols (`with_handle` and `try_advance`) occupy 44.85% of stream samples and 41.87% of prefix samples. The prefix profile also exercises `group_g2_prefix` and the current G2 prefix kernel. These aggregate shares do not isolate removable wall time or establish that scheduler overhead explains the latency difference. The latency CPU observations above also do not show a reduction in total CPU consumption.

## Scope and next decision

The component gate passes, so a bounded, opt-in SDK integration at high worker counts is now justified as an experiment. It must preserve the existing stream for small pools, unsupported query sizes and insufficient memory, then pass all-nine verified SDK, memory and consumer qualification. The primary comparison must include the accepted 32-worker configuration: beating a degraded 96-worker stream alone would not establish a better prover. This result does not authorize promoting either locally slower tree or claiming a full-proof gain.

This study does not measure Aeglos,concurrent throughput,phones,TEE,memory pressure,swap or giant circuits. [Latest C7a whole-proof comparison](../2026-10-26-lower-bound-c7a/README.md) and [latest local all-nine SDK table](../2026-10-26-prefix-pool/README.md) retain supplied M5 CPU/Metal references. The all-eightCPU10× objective remains active and unmet.

The owned instance is terminated; its EBS volume,temporary ECR tag,private S3 object and bucket are absent,and the presigned URL parameter file is deleted. No production networking or IAM changes. [Frozen design](artifacts/design.md),[analysis](artifacts/remote-results/output/measure/analysis.json),[cleanup](artifacts/cleanup-complete.json),and[artifact index](artifacts/artifact-index.json) retain the evidence.
