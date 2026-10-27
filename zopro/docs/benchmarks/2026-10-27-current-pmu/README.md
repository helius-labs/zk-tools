# Accepted-source warm hardware counters

The unchanged accepted SDK was measured in three fixed blocks at primary16, primary32 and merge54/32, using separate branch and cache panels. These are diagnostic counters around warm requests, not acceptance timings or a candidate speedup. The existing 172-file source and SDK14da8231 remain unchanged.

At primary, 32 workers retire about 3% more instructions than 16 but accumulate about 43% more user cycles. IPC falls from about 2.30 to 1.66. Increased summed CPU consumption therefore cannot be read directly as an equal increase in executed instructions. This comparison does not isolate SMT, clocks, kernel geometry, memory or synchronization. C7a has different hardware and no SMT in the previous runs, so this local result does not explain its scaling reversal.

Each panel counts four events together, with actual running percentages validated. A six-event capability-only group was unschedulable and is retained as not-counted evidence, not interpreted. The existing SDK FIFO/ack protocol enables counters after loading and three primes, then disables them before export. No build/helper/hash/archive/second owned CPU overlaps requests. Required independent verifiers run serially between captures.

## Counter medians

Each cell is the median of three process observations. Cycles/instructions are per timed request, IPC is the median of per-process ratios. Different panels are separate processes. All event scopes are userland and include useful arithmetic, polling and coordination; rusage CPU also includes kernel time. Generic cache events do not measure DRAM bytes, bandwidth or load latency.

|Case|Panel|Cycles/request million|Instructions/request million|IPC|CPU ms/request|Request ms|
|---|---|---:|---:|---:|---:|---:|
|1x2-w16|branch|607.940|1397.785|2.299|123.841|10.022|
|1x2-w16|cache|604.393|1397.282|2.312|123.520|10.422|
|1x2-w32|branch|865.635|1439.325|1.662|190.483|8.452|
|1x2-w32|cache|865.939|1439.416|1.663|190.573|8.451|
|merge-54x1-w32|branch|20802.321|18009.544|0.866|4400.677|161.336|
|merge-54x1-w32|cache|20734.834|18010.322|0.869|4402.044|161.186|

These counters do not isolate SMT, memory latency or scheduling causes. Worker count can change algorithm geometry, coordination and frequency. Three diagnostic blocks do not establish broad confidence bounds. Raw branch/cache counts, runtime percentages, SDK samples and commands remain in analysis and the archive.

|Case|Panel|Branches or generic cache references/request, million|Corresponding misses/request, million|
|---|---|---:|---:|
|1x2-w16|branch|74.789|1.092|
|1x2-w32|branch|83.994|1.221|
|merge-54x1-w32|branch|1001.909|6.653|
|1x2-w16|cache|60.934|1.743|
|1x2-w32|cache|80.712|2.700|
|merge-54x1-w32|cache|1159.850|80.318|

The cache counter names are retained as Linux generic events. These are not RAM-served load counts or measured bandwidth. The large circuit has a different instruction mix, so its lower IPC is not a directly recoverable performance fraction.

## Exploratory instruction audit

After the census, offline annotation reused the exact accepted SDK's existing primary profile from the [packed-G1 experiment](../2026-10-27-g1-raw-scratch/README.md). No new capture or proof is counted. The borrowed signed G1 stream has 771 samples. Annotation periods sum exactly to the archived symbol total; the [audit](artifacts/instruction-audit.json) records raw-profile, annotation and SDK hashes.

|Sampled opcode class|Share of this symbol's weighted period|
|---|---:|
|IFMA|43.96%|
|Other vector arithmetic, masks and shifts|41.69%|
|Vector moves|8.90%|
|Vector shuffles|1.89%|
|Scalar, control and calls|3.56%|

Instructions with explicit RSP/RBP address expressions carry 2.95% of the symbol's weighted period. That is not a count of all spills, stack traffic or avoidable stalls. Cycle sampling can skid; opcode position does not establish precise instruction latency or causality. These observations provide no evidence for a large gain from another record-copy rewrite. They motivate examining arithmetic work and dependencies while retaining the negative packing, square, lazy-linear and scheduling experiments.

The next useful crosshost diagnostic would compare instruction/cycle changes with worker count on C7a itself. This report does not contain that measurement or select another queue, lane-width or layout variant.

## Validation and scope

All 6,252 SDK calls internally verify; all 18 exported proofs independently pass Gnark, are 164 bytes, have exact fixture bindings and are fresh against 12,125 prior/new identities. Ledger **12,125→12,143**, 36 proof/public members. All frozen source/library/input/helper pins are rechecked. No native rebuild, new native/portable suite or new performance qualification is claimed. No AWS/global perf setting/production resource changes occur.

The [accepted CPU/Metal comparison](../2026-10-27-clean-fft-adoption/README.md) retains all eight supplied historical M5 references; none is remeasured here. No 54 reference was supplied. The original CPU 10× objective remains active and unmet, 0/8. No hardware counter ratio is treated as a speedup or removable wall time.

[Design](artifacts/design.md), [all rows](artifacts/analysis.json), [proof inventory](artifacts/proof-inventory.json), [workflow](artifacts/workflow/result.json), [artifact hashes](artifact-index.json).

The [perf-stat manual](https://man7.org/linux/man-pages/man1/perf-stat.1.html) documents FIFO acknowledgement, inherited counting, grouping and output fields. Existing [IBS observations](../2026-10-23-local-ibs-memory/README.md) and previous negative experiments remain separate evidence.
