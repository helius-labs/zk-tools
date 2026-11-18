# Current adopted SDK on c7a.24xlarge

Current Zopro 1×2 reaches **10.927 ms at 32 physical cores**, versus
**8.157 ms at 96 cores** for retained Aeglos Sorted. At these best observed
configurations, maximum lifetime RSS is **88.82 MiB / 462.84 MiB**, respectively.
Aeglos has lower latency; Zopro uses less reported CPU time and peak RAM.

Current Zopro improves **1.85% paired, with 9/9 wins**, over previous AWS-tested
build 194 at 32 workers, saving 207.92 µs. Its 96-worker comparison is inconclusive.
All 220 fresh remote exports pass server Gnark verification and host
reverification. The instance, EBS volumes and temporary S3 bucket are removed.
There is no production change or new adoption decision.

## Primary results

|Build|Physical workers|Median ms|CPU ms/request|Maximum RSS MiB|
|---|---:|---:|---:|---:|
|Current Zopro|32|10.927|228.6|88.82|
|Current Zopro|96|12.534|689.8|76.15|
|Previous Zopro194|32|11.139|238.7|80.55|
|Previous Zopro194|96|12.534|638.2|75.28|
|Retained Aeglos Sorted|32|11.929|327.5|417.83|
|Retained Aeglos Sorted|96|8.157|516.5|462.84|

Latency cells are medians of nine process means. CPU values are medians of the
harnesses' reported CPU time per timed request. RSS is the maximum lifetime
process peak, including loading, preparation and primes; it is not minimum RAM.

|Current/control|Workers|Paired wall ratio [bounds]|Wins/9|Paired CPU ratio|
|---|---:|---:|---:|---:|
|Current/previous194|32|0.981476 [0.974575,0.985245]|9|0.962054|
|Current/previous194|96|0.999262 [0.987990,1.014008]|5|1.075464|
|Current/Aeglos|32|0.916141 [0.864304,0.922909]|9|0.699259|
|Current/Aeglos|96|1.555017 [1.469301,1.578484]|0|1.337849|

Ratios below one favor current Zopro. At equal 32-worker counts, current Zopro
has 8.39% lower paired latency than this Aeglos artifact. At 96 workers, it has
55.50% higher latency. Comparing the lowest observed median for each build leaves
Aeglos ahead. Only 32 and 96 workers were measured. All three current/previous
32-worker backing medians are below one.

## Scaling conclusion

Current primary 96/32 wall ratio is 1.143153 [1.135511, 1.173226], with paired CPU
ratio 3.014018. Aeglos changes by 0.686051 [0.644706, 0.718348] in wall time and
1.568056 in CPU time. Increasing configured cores helps this Aeglos primary case
but worsens current Zopro primary latency.

Median reported CPU-time/wall-time ratios are 20.93/54.89 for current Zopro at
32/96 workers, versus 27.46/63.11 for Aeglos. These equal 65.4%/57.2% and
85.8%/65.7% of their respective CPU masks. They include synchronization and other
process CPU work; they do not measure useful-arithmetic utilization.

Against build 194, current paired CPU falls 3.79% at 32 workers but rises 7.55% at
96, where latency is inconclusive. The improvement at 32 workers does not resolve
the scaling gap. The next profiling question is how much of the current C7a
critical path comes from useful arithmetic, synchronization or insufficient
parallel work. CPU time alone establishes no scheduler, cache or allocation cause.

## All nine circuits

CPU and RSS pairs are for 32/96 workers. Scaling ratios use within-block pairs,
rather than ratios of displayed marginal medians.

|Shape|32-core ms|96-core ms|Paired96/32 [bounds]|CPU ms32/96|Max RSS MiB32/96|
|---|---:|---:|---:|---:|---:|
|1×2|10.927|12.534|1.1432 [1.1355,1.1732]|228.6 /689.8|88.8 /76.2|
|2×2|14.610|16.759|1.1484 [1.1369,1.1604]|318.0 /975.7|217.1 /211.7|
|4×2|19.440|20.906|1.0733 [1.0581,1.0920]|489.8 /1441.8|276.5 /262.8|
|8×2|26.888|28.266|1.0499 [1.0268,1.0791]|734.0 /2116.5|455.8 /455.9|
|12×2|35.562|35.600|0.9986 [0.9907,1.0121]|984.8 /2588.6|643.1 /633.9|
|16×2|41.164|41.031|0.9997 [0.9890,1.0114]|1169.3 /2835.5|780.6 /783.7|
|24×2|60.018|56.805|0.9498 [0.9167,0.9575]|1725.1 /3751.3|1239.6 /1215.3|
|Merge24×1|61.226|61.177|0.9965 [0.9693,1.0276]|1671.9 /3846.2|1118.4 /1155.5|
|Merge54×1|140.713|125.368|0.8909 [0.8705,0.9062]|3568.0 /6420.5|2081.6 /1999.6|

The 96-core configuration helps 24×2 and merge54. Smaller cases regress and
middle cases are nearly flat. The effect depends on the circuit.

## Supplied M5 references

These are supplied historical CPU/Metal timings; no Apple machine was remeasured.
Speedup pairs are for 32/96 AWS workers. **All 16 context checks fail the CPU 10×
upper-bound criterion; the objective remains unmet for all eight shapes.**
Merge54 has no supplied M5 reference.

|Shape|M5 CPU/Metal ms|CPU speedup32/96|Metal speedup32/96|CPU10× limit ms|
|---|---:|---:|---:|---:|
|1×2|37 /24|3.39 /2.95|2.20 /1.91|3.7|
|2×2|59 /30|4.04 /3.52|2.05 /1.79|5.9|
|4×2|109 /48|5.61 /5.21|2.47 /2.30|10.9|
|8×2|189 /74|7.03 /6.69|2.75 /2.62|18.9|
|12×2|284 /101|7.99 /7.98|2.84 /2.84|28.4|
|16×2|335 /124|8.14 /8.16|3.01 /3.02|33.5|
|24×2|528 /176|8.80 /9.29|2.93 /3.10|52.8|
|Merge24×1|507 /175|8.28 /8.29|2.86 /2.86|50.7|

## Artifacts and method

Current SDK SHA256:
`d523f49e8548620c4373a1ab4be2712d705709fdd7a590c5ed73e9aff97424e3`.
This is the exact experiment-233 forced-cfg SDK qualified in 234/235, with the
188-file implementation adopted at `b164b9776b`. The normal native selector
integration in 236 was separately validated; its normally selected SDK has not
been newly linked or timed.

Previous build-194 SDK:
`bcc8e0775bb5bc6c09ad86b1987ca45eebac62617340911266c84aa601fa05b1`.
This comparison covers aggregate changes from 194 to 233, including arithmetic
and weighted reductions. It does not isolate the final small reducer.

Retained Aeglos Sorted:
`eab9fbfd4b94aeef9ced3ce3e3ede21a37ab70ad38b217720f1afd06fceba99b`,
source `0290613e8eeca17a4ce88f6f2d726e8e06a5385d`. Later upstream changes are
outside this measurement. Both Zopro arms enable certified output, fused
preparation and indexed tile bundles. Aeglos uses its existing `verified_ms`
boundary.

One eu-north-1 c7a.24xlarge ran AMD EPYC 9R14, with 96 physical cores, one thread
per core and Linux 7.0.0-1014-aws. Fixed process masks, individual Zopro worker
affinity, default SDK stacks and identical allocator policy were retained. Each
engine had three independently written, byte-identical backings; all were used.
No owned CPU workload overlapped remote captures.

There were 22 preflight captures, followed by 22 Gnark verifications, semantic
checks and the separate reader, before 198 main captures. Nine matched,
rotated/reversed blocks covered 22 contexts; main processes had three primes and
17 timed fresh requests. All main captures preceded their 198 server verifications.
Totals: **4,048 remote SDK calls, 3,388 timed records and 220 exports**. Prior local
preparation adds 16 calls and four exports, totaling 4,064 calls and 224 distinct
new proofs. Host reverification adds no distinct proofs.

Zopro timing includes solving, quotient construction, fresh blinding, proof
construction, serialization and SDK verification. Mapping, prepared-key setup,
pool creation, primes and export are outside latency and inside lifetime RSS.
The retained Aeglos timing contract is preserved. Cold starts and concurrent
server throughput were not measured. Image/request/public bindings match the
experiment-234 fixtures.

Intervals use zero-based sorted ranks 1/7 from nine block values: nominal
96.09375% marginal median coverage under independent continuous-block assumptions,
with no simultaneous or familywise guarantee. Best-count summaries are descriptive
among the two declared counts. There were no exclusions, extensions, worker
changes or new adoption vetoes.

## Resources and verification limits

The service had 30 GiB/no-swap infrastructure headroom. Recorded unit/system.slice
memory.high/max/oom counters did not increase. All 200 remote Zopro processes
report zero measured-loop major faults. The 20 Aeglos processes lack that field;
absence is not zero. Primary minor faults per timed request were 516.7/497.8 for
current Zopro at 32/96 workers and 104.1/287.8 for Aeglos. Minor fault counts alone
do not establish disk I/O, physical pressure or a cause.

`cpu.max` and `cpuset.cpus.effective` were unavailable at the recorded cgroup
levels. Read errors and ancestor observations are preserved; missing files do
not prove unlimited CPU or absence of host scheduling effects.

The terminal systemd summary reported 512 KiB peak memory, inconsistent with the
retained unit `memory.peak` of 1,779,085,312 bytes. Tables use SDK per-process
maximum RSS. Controller observations are retained separately.

The server reader checked all 220 exports and raw records. After retrieval, all
220 host Gnark verifications and the frozen host analysis passed. A second offline
reader reconstructed 3,388 samples, 660 capture/server-verifier/host-verifier
receipts, 1,320 stream hashes, all 18 current latency/CPU/RSS/M5 rows, four primary
comparisons, backing medians and nine scaling rows. This was a separate same-agent
review. No new PMU, ARM, huge-circuit, physical-pressure or minimum-RAM
qualification was added.

All captures, proof verifiers and original result readers succeeded on their
first execution. The new offline statistics helper initially failed compilation
on a redundant JSON value block. Its repair changed no sample, protocol or
statistic; no successful scientific work was replayed. Earlier preparation
failures remain in [preparation](preparation.md).

## Execution and saved results

AWS service execution: November 18, 2026, 11:34:24–11:39:07 UTC, lasting
4 min 43.775 s. Serial capture commands sum to 229.971 s, server verifiers to
44.242 s and host verifiers to 24.133 s. These are summed command durations,
not exclusive CPU or critical-path fractions. The existing package was reused
after renewed SSO; prior login waiting is outside this interval.

One instance was launched and its termination verified. Tagged EBS volumes are
absent, temporary objects and their bucket are deleted, and secret transport
parameter files are removed. Cleanup preceded report generation. No ECR image
was published.

Canonical inventory: **62,411**, at
`/home/ubuntu/src/zopro-perf-work/adopted-c7a239/host-remote/proof-sha256.json`,
SHA256 `17f43bcd6b9a2731c5464bfdc62b4fb26948e86b9ea67452269126cc022d0d7b`.
Complete private results: `/home/ubuntu/src/zopro-perf-work/adopted-c7a239`.

[Results](artifacts/results243.json), [sanitized samples/processes](artifacts/processes243.json),
[CPU/resources/scaling](artifacts/supplement243.json),
[independent review](artifacts/independent243.json),
[host verification](artifacts/host-review243.json), [receipts](artifacts/receipts243.json),
[source/readers](artifacts/post-sources243.tar.gz),
[frozen preparation sources](artifacts/sources-and-plans.tar.gz).
Private keys/requests, proof/public bytes, full inventories, executables and raw
PID/TID/address streams are excluded. The readiness snapshot remains historical.
Results are committed locally; no push is performed.
