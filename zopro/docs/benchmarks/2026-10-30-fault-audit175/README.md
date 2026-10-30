# Page-fault audit of saved threaded prover runs

30 October 2026. Reanalysis of completed measurements through commit `72eaef9a0c3ca265357addfb1423b20a6f32ce19`; no new benchmark, proof or AWS execution.

**The former 54→1 page-table fault problem is substantially reduced. The recent 32-worker arithmetic regression occurs with almost no faults in either variant.** Full-proof Zopro still records more minor faults than pinned Aeglos, so some VM work remains. Neither fault counts nor total kernel CPU alone identify its cost on the proof critical path.

## Definitions and scope

A minor fault is serviced without I/O; a major fault requires I/O. Minor faults are not evidence by themselves that RAM is exhausted. `RUSAGE_SELF` sums resource usage over the process's threads. [Linux getrusage documentation](https://man7.org/linux/man-pages/man2/getrusage.2.html).

The existing runners record `ru_minflt` but omit `ru_majflt`; major faults therefore remain **unreported**, not assumed zero. The SDK usage interval starts after three primes and excludes key loading. We divide each process's count by its timed request count, then take the median of those per-request averages across 15 processes. This is not a median of individually instrumented proof fault counts. CPU time is summed across threads; it is not wall latency. All comparisons below retain the pinned engine configurations and original measurement limits.

## AWS primary 1×2

c7a.24xlarge, 96 physical cores, no SMT; 101 timed requests per process after three primes. Accepted-source Zopro is labelled plain; the unadopted PGO candidate is excluded from this accepted-engine comparison. Aeglos uses the pinned Sorted engine.

|Workers|Zopro faults/request|Aeglos faults/request|Zopro kernel CPU ms/request|Aeglos kernel CPU ms/request|Zopro request ms|Aeglos request ms|
|---:|---:|---:|---:|---:|---:|---:|
|32|181.17|36.36|13.857|6.046|11.915591|11.867991|
|64|193.98|70.25|27.590|12.296|11.994779|8.773943|
|96|204.95|96.13|49.894|19.151|13.777708|7.959184|

Zopro's primary minor-fault count rises modestly with workers while kernel CPU grows much more. Aeglos's faults also rise as its request latency falls. These observations do not make faults costless, but they show that fault counts alone cannot explain the different scaling. Kernel CPU includes work besides fault handling; this audit does not attribute its remainder to allocation, futexes or scheduling.

## AWS merge 54→1

Seventeen timed requests per process after three primes; the same normalization applies.

|Workers|Zopro faults/request|Aeglos faults/request|Zopro kernel CPU ms/request|Aeglos kernel CPU ms/request|Zopro request ms|Aeglos request ms|
|---:|---:|---:|---:|---:|---:|---:|
|32|26352.00|2528.18|169.610|102.255|138.189311|347.611126|
|64|24422.29|2435.35|219.460|63.811|122.396711|207.563258|
|96|23952.24|3325.06|350.525|87.296|126.994039|173.236337|

The [October 23 study](../2026-10-23-aws-local-crosshost-aeglos/README.md) reported roughly 296,715 minor faults and 16,604 ms summed kernel CPU per 54→1 proof at 96 workers. Its kernel call-chain profile placed about 91% under anonymous-page handling and page-table locking. In the newer build the corresponding descriptive counts are roughly 23,952 faults and 350.5 ms kernel CPU, with 126.994 ms request latency. The old separate unchanged diagnostic measured 421.539 ms. These are different builds/runs; the full latency change cannot be attributed to fault reduction alone. No new kernel call-chain profile establishes the composition of the remaining 350.5 ms.

At 96 workers, Zopro still has about seven times Aeglos's minor-fault count, while proving 54→1 faster. The whole-workflow memory/OOM/swap-event deltas in the latest AWS record are zero. Those are workflow-wide observations, not per-request major-fault counts or a general memory-pressure qualification.

## Preallocated batch callers

The latest local R256-only arithmetic trial reuses scratch after primes, reports zero timed allocations and changes no heap capacities. Its OFF and ON medians both show **two minor faults per complete timed wave** for every 32-worker caller. Each wave contains 8,192 calls per worker, or 262,144 batch calls in total. These are not proof requests.

|32-worker caller|OFF median faults/wave|ON median faults/wave|OFF maximum|ON maximum|
|---|---:|---:|---:|---:|
|g1-owned/200|2|2|13|19|
|g1-owned/352|2|2|14|14|
|g1-borrowed-signed/200|2|2|22|26|
|g1-borrowed-signed/352|2|2|22|20|
|g2-borrowed-signed/200|2|2|13|16|
|g2-borrowed-signed/352|2|2|15|17|

All six 32-worker timing estimates regressed in that arithmetic experiment despite these low fault counts. This provides no evidence that page faults explain that regression. It also does not establish allocation-free end-to-end proving: the full SDK has different allocation and synchronization lifetimes. [Complete caller experiment](../2026-10-30-r256-truncated-caller175/README.md).

## Consequences for further optimization

The historical large-proof page-table contention should not be carried forward as the sole current bottleneck. The shared arithmetic still needs work that helps complete threaded callers. End-to-end VM and coordination costs remain separate targets; a new diagnostic should retain major faults and use phase/kernel evidence before attributing system CPU to fault handling. Any proposed workspace reuse or page-touch policy must preserve admission, release and peak-memory behavior, rather than retain unbounded state.

This audit reviews all 810 accepted-Zopro/Aeglos AWS process summaries across nine shapes and three worker counts, plus all 1,674 local caller timing records across 18 contexts and three variants. Count cardinality, priming/rounds, normalized CPU totals, process medians, zero timed allocations and all 175 production source hashes are checked. The machine-readable file includes every selected cell and normalized block; no successful test/capture/proof is repeated. Major counts cannot be reconstructed from these retained summaries.

Accepted single-proof results remain 1×2 **8.022165 ms / 134 MiB** locally and 54→1 **162.250464 ms / 1,831.87 MiB**. The original eight CPU 10× targets remain unmet. All nine local/AWS rows and eight supplied M5 CPU/Metal comparisons are preserved in [accepted rows](accepted-rows.json) and the [complete benchmark snapshot](../2026-10-30-current-bench-snapshot175/README.md). No new performance, minimum-RAM, pressure/swap, mobile/ARM/TEE, huge-circuit or concurrency result is claimed. Distinct-proof ledger remains 25,231.

[Data and normalization](data.json), [analysis source](analyze.rs), [input hashes](source-hashes.json), [checksums](SHA256SUMS), [original AWS benchmark](../2026-10-29-proof-pgo-c7a175/README.md).
