# Grouped-level G2 reduction trees

The component does not pass its frozen speed gate. Production dispatch and accepted170source files remain unchanged. This is an isolated rawG2 caller study, not full-proof or M5 speed evidence.

Primary paired regressions are22.28% at16workers and12.01% at32, with zero of seven wins at both counts. Every measured case loses all seven blocks. At32workers primary requested peak above prepared inputs rises8.835→28.674MiB. This implementation does not justify SDK integration.

Ordinary12-bit component buckets are counted/scattered through signedu32 indices. Existing complete affine kernels batch pairs across256buckets at each tree level, instead of retaining one growing accumulator per bucket. G2phi2 split,two partitions,high-bucket phi,narrow top digit,part merging and weighted final reduction remain unchanged. No new pool,key table,field formula or SDK route. The tree introduces input-order changes,temporary point copies and a separate low/high merge. It differs from old tiny per-bucket trees and linear routed bucket rounds.

Typed conservative admission sums all possible suspended routing/group buffers above the original raw-query forecast. Explicit denial errors prevent a benchmark silently using another implementation. This is not a universal retained memory allocation or a process-RAM guarantee. No pressure/phone/TEE/large-circuit/concurrency qualification is claimed.

## Fixed caller comparison

Actual1x2/2x2B2 points and solved fixture scalars. Compressed key and current Euler point order agree exactly. Seven rotated/reversed fresh-process blocks,seven timed calls after two primes; each process also computes one stream reference. CPU0,w16CPUs0–15,w32CPUs0–31. Timing includes scalar clone,decomposition,admission,routing,allocations,fill/reduction,result normalization/equality check and temporary drops. It excludes key loading,witness solving,reference setup,scalar grouping,other queries,blinding,SDK verification and serialization. Both modes pay allocator accounting atomics; this can affect an allocation-heavy candidate and is not plain SDK timing.

Times are medians of process medians. Paired ratios are geometric means of within-block tree/stream ratios, so they may differ from ratios of the displayed medians.

|Case|Stream ms|Tree ms|Paired tree/stream|Wins/7|Stream/tree requested peak MiB above inputs|Stream/tree whole-process RSS MiB|
|---|---:|---:|---:|---:|---:|---:|
|1x2-w1|40.458085|52.784911|1.306266|0|1.556/2.318|60.50/60.50|
|1x2-w16|3.672836|4.047011|1.222850|0|7.716/18.529|63.50/76.93|
|1x2-w32|2.963398|3.325931|1.120072|0|8.835/28.674|64.12/86.62|
|2x2-w1|64.919663|82.152001|1.265427|0|2.252/3.521|94.53/94.53|
|2x2-w16|6.013500|6.366356|1.072160|0|8.530/26.339|100.67/108.05|
|2x2-w32|4.820640|5.317397|1.106767|0|9.746/41.689|99.84/127.86|

Gate: positive primary paired geometric ratio against stream at both16/32workers,at least4/7wins each,and2x2ratios<=1.02atboth. Workers1isdescriptive. No minimum positive gain floor or changed gate. Failures:

- 1x2/w16 ratio1.222849720341724 wins0/7
- 1x2/w32 ratio1.1200724419968542 wins0/7
- 2x2/w16 ratio1.07215986762248
- 2x2/w32 ratio1.1067669941986604

Requested peaks count caller layout bytes above prepared inputs,including the scalar clone and returned buffers,forwarding to the accepted allocator. They are not minimum RAM or an admission guarantee. Whole-process RSS includes loader/reference/primes. Every timed sample checks its peak against the typed forecast plus the input-scalar clone; all bounds pass. Full112nativecore,raw2/cubic6andgeneratorcheckpass;new3coretestscovergeneratororacles across workers,actual-size randomG2andcheckedforecasts. Field math is unchanged and its prior qualification is inherited.

All84latencies precede two primary32profiles,199Hz,64timedcalls after two primes. The existing FIFOs enable profiling after input/reference preparation. Profiles are user-cycle samples and not latency observations.

|Mode|Samples|Lost records|
|---|---:|---:|
|stream|1463|0|
|tree|1983|0|

Both profiles contain one sampled SDK process. The reports show total lost samples0. Stream samples emphasize the specialized G2 prefix-record kernel (38.49%); the tree emphasizes the existing generic `reduce_layer` (55.10%). These are descriptive sampled shares, not operation counts, exclusive stages or isolated causes. Rayon helper symbols can contain inlined arithmetic and point-copy work.

Source inspection reveals a coupled implementation difference worth isolating: the tree uses generic layer records containing nine Fq vector fields, while the current stream retains two Fq prefix/norm vectors and reloads its anchors. The current comparison therefore does not isolate tree topology from addition-kernel/storage choice. A follow-up could preserve the tree topology while using the current streaming batch-add kernel at each level, with complete-curve handling and a new typed forecast. That remains unimplemented and needs a separate declared experiment; the negative result here stands.

Across latency/profile processes:974raw-query executions,including86reference setups. There are no SDK proofs or Gnark exports;prior5958stays unchanged. This component cannot establish the original all-eightCPU10× goal. [Latest all-nine rejected-candidate comparison](../2026-10-26-prefix-pool/README.md) retains supplied CPU/Metal references; no whole-proof gain is inferred by dividing them by a component time. No new AWS or Aeglos measurement.

All helpers compiled before latency. No successful capture is repeated or excluded. Candidate source,protocol,rows,requested/RSS data and decoded profiles are retained in [analysis](artifacts/analysis.json),[design](artifacts/design.md),[source audit](artifacts/source-audit.json),[jobs](artifacts/jobs.json),[artifact index](artifacts/artifact-index.json). Rawperf,binaries and large inputs stay local with hashes.

Two workflow interruptions are retained. The initial compile had a visibility error, fixed before any capture ([build repair](artifacts/build-repair.md)). After all latency and the successful stream profile, perf's build-ID cache added a hard link to the executable, changing its ctime while preserving its SHA256. The runner stopped on that metadata change. An unchanged-protocol resume rehashed inputs and reused86successful command checkpoints, including all84latencies and that profile; no capture was repeated or excluded. [Metadata note](artifacts/profile-metadata-note.md) and [inode receipt](artifacts/profile-sdk-link-stat.txt) record the event. No guard or timing gate was relaxed.
