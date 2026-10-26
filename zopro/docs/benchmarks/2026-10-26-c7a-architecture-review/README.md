# C7a small-proof architecture review

Aeglos's remaining advantage is concentrated in small proofs on the 96-physical-core server. The strongest explanation supported by the existing evidence is a combination of its prepared MSM algorithm and how that algorithm exposes parallel work. A generic claim that Zopro needs faster field multiplication, more RAM, or a different thread pool is insufficient.

This is a reanalysis of archived October 25 measurements, not a new benchmark. That server used 167 pinned Zopro source files; the current accepted build has 170. No fresh AWS, Aeglos, proof, memory-pressure or phone measurement is claimed. [analysis.json](analysis.json) records input hashes and derived values. [analyze.rs](analyze.rs) checks all 432 rows and the four previously pinned Aeglos files before calculating results.

## Where the advantage appears

| Workers | Zopro request ms | Aeglos Sorted request ms | Aeglos MSM ms | Aeglos request minus MSM ms |
|---|---:|---:|---:|---:|
| 16 | 19.809 | 19.291 | 15.238 | 4.042 |
| 32 | 13.627 | 11.606 | 8.116 | 3.498 |
| 64 | 14.354 | 8.602 | 4.943 | 3.645 |
| 96 | 16.052 | 7.966 | 3.853 | 4.003 |

Each value is a median of three process medians, with all 31 samples retained per process. The final column first subtracts MSM time from request time **within each request**, then computes those medians; it is not a subtraction of independent medians. Stage medians therefore need not add to request medians.

The small-proof difference at 16 workers is about 0.52 ms. Aeglos's MSM becomes 3.96 times faster from 16 to 96 workers; its full request becomes 2.42 times faster. Zopro's best tested point is 32 workers, followed by regression. We should explain this shape before attributing the difference to uniform arithmetic throughput. This does not establish equal arithmetic costs at 16 workers.

The [full comparison](inputs/2026-10-25-latest-c7a-aeglos.txt) also matters: Zopro wins six of nine circuits at their respective best tested counts. Aeglos wins 1x2, 2x2 and 4x2; Zopro wins from 8x2 onward, including merge54 at 129.985 versus 173.566 ms. That crossover constrains any general explanation.

## What Aeglos prepares

The pinned source is [0290613e](https://github.com/helius-labs/aeglos/tree/0290613e8eeca17a4ce88f6f2d726e8e06a5385d/cpu). Its Sorted engine prepares shifted bases and their endomorphism images. Online it scatters contributions from different scalar windows into **one bucket set**. Within a block, it repeatedly pairs entries from each bucket, batches independent additions, and carries odd entries to the next round. The bucket-local pairing depth is logarithmic in its occupancy, although batches, inversions, sorting and final reductions add real dependencies and costs. It is not a claim that total MSM latency is logarithmic.

Its runner batches multiple MSM calls into shared, nonnested recode/count/scatter/accumulate/sum passes. Bucket blocks interleave low bucket indices to distribute crowded buckets. Thread-local arenas retain intermediate pair results. The pool drains a common task index and has explicit SMT-dependent parking behavior. These source observations identify coupled choices; none is individually established as the source of the speedup.

Zopro's ordinary unshifted streaming filler has separate window states, mutable bucket accumulators, a queue for colliding bucket updates, and batched inversions. Repeated updates to one accumulator depend on its previous value. Query and reduction parallelism exist, but the available independent work differs from Sorted's shifted-window bucket blocks. This is an algorithm-and-dataflow distinction, not a C++ versus Rust distinction.

The same-server Aeglos Windowed control is important: its best 1x2 result is 26.809 ms versus Sorted's 7.966 ms. It retains the same general pool while changing the engine and its preparation. Therefore the pool alone cannot explain Sorted's result. Conversely, this coupled engine change is not a clean experiment isolating tables, reduction trees, or memory layout.

## What the profiles establish, and what they do not

The [warm diagnostic](inputs/2026-10-25-c7a-warm-profile.txt) finds similar summed primary CPU time at 32 workers: 317.4 ms for Zopro and 329.8 ms for Aeglos. At 96, Zopro reaches 1020.4 versus 538.0 ms, with 1811 versus 102 voluntary switches per proof and IPC of 0.626 versus 1.251. This supports a coordination/work-distribution problem as worker count grows.

Summed CPU time includes spinning, scheduler work and kernel execution. It is **not** an arithmetic-operation count. Average active cores likewise includes overhead. Cycle profiles attribute aggregate cost across threads; they do not identify the dependency chain determining request completion. Prominent epoch samples may be a symptom of workers searching for useful work, as well as a source of overhead. Removing a prominent symbol does not guarantee shorter proofs.

Previous interventions already limit the scheduler explanation. [Balanced worker groups](inputs/2026-10-25-worker-groups-c7a.txt) improve 13.304 ms to 11.441 ms but leave a 3.353 ms difference from Aeglos's 8.088 ms in that run. The [scoped proof graph](inputs/2026-10-25-proof-dataflow-c7a.txt) regresses. Pool partitioning and coarse dependency restructuring have therefore been tested; proposing them again is insufficient.

Cache and memory-access behavior remain plausible contributors to the host reversal. [Local IBS](inputs/2026-10-23-local-ibs-memory.txt) finds a larger sampled memory-access cost for Aeglos, concentrated in point loading. The [host inventory](inputs/2026-10-23-aws-local-crosshost-aeglos.txt) records 64 MiB L3 in two instances locally versus 384 MiB in twelve on C7a, with 16 physical cores plus SMT locally versus 96 physical cores without SMT on C7a. Aeglos's retained native bytes increase from about 254.65 MiB for 1x2 to 445.46 MiB for 2x2 and 802.84 MiB for 4x2.

Those figures do **not** establish that a 255 MiB table fits a single shared server cache. The caches are distributed; access placement, replication, per-task footprint and memory-level parallelism matter. Retained native bytes are also not exclusively a table-size measurement. We do not yet have a matched server memory-access experiment isolating cache locality from physical-core scaling. Treat that explanation as a hypothesis.

## The missing measurement and the next decision

We lack a current-build measurement distinguishing **insufficient independent arithmetic work** from **ready work that is expensive to dispatch or execute**. Existing inclusive query timelines and join counts cannot make that distinction: callbacks may execute other queries while waiting, and counts omit dependency duration. This is the main gap in the investigation.

The next diagnostic should observe the actual arithmetic leaves and their dependency boundaries in the current Zopro and pinned Aeglos paths, before another candidate:

1. On matched small-proof inputs, record task-ready/start/finish boundaries for window or bucket-block arithmetic, inversion batches and reduction levels. Separate executor search/parking from mathematical work. Use bounded diagnostic storage and qualify tracing overhead against OFF; do not interpret perturbed latency as accepted performance.
2. Compare 16/32/96 physical workers on C7a, keeping core placement and source versions explicit. Include a circuit beyond the observed crossover. Capture memory-access evidence only if the host exposes suitable events; record availability failures rather than substituting unsupported cache conclusions.
3. Ask whether the late part of each proof has too few runnable arithmetic tasks, has ready tasks stranded behind coordination, or has active tasks stalled on memory. That result chooses between changing MSM decomposition, dispatch, or locality. It prevents another queue or field-kernel edit based only on aggregate samples.

This is a diagnostic design, not a claim that those traces have been collected. It does not authorize a universal table-based implementation. Any future candidate must retain a bounded low-memory path and be judged on full verified request latency and memory. A server-specific strategy can be selected from actual resources without forcing phones or giant circuits to retain shifted tables.

The obvious algorithm transfers have already had negative results: [unshifted indexed G1 bucket trees](inputs/2026-10-22-universal-indexed-g1-tree.txt), [one-half shifted bases](inputs/2026-10-23-shift128-g1-component.txt), and [cache-group query pools](inputs/2026-10-22-universal-cache-islands.txt). A new proposal must account for why it differs from those experiments; neither “use trees” nor “use a small table” is sufficient.

Finally, Aeglos's unchanged non-MSM remainder is about 4.00 ms at 96 workers. Removing its MSM interval entirely would still miss our 3.7 ms primary target in that configuration. This is a counterfactual for that measured schedule, not a fundamental lower bound: solver, FFT, assembly, verification and overlap can change. It shows why an MSM-only research program cannot be assumed to meet the full target.

No production change or new speed improvement results from this review. The tenfold goal remains unmet.

## Reproduction

The input directory includes the exact archived 432-row matrix, source pins and verbatim prior report summaries. The summaries are evidence snapshots; their original relative artifact links refer to their original archive locations and are not reproduced here. They are not new measurements.

Compile analyze.rs with the repository's existing serde_json and sha2 dependencies. Run it with repository root, output JSON path and an Aeglos checkout root at the pinned commit as its three arguments. It checks source hashes, complete matrix cardinality, repetition identities, sample counts and every process median, then derives results from all retained samples. It does not launch either prover.
