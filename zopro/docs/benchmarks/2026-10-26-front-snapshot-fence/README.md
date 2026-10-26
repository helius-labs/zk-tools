# One common fence after bounded victim-front snapshots

The fixed local speed screen fails. The candidate is not adopted and no unchanged timing rerun follows. All samples and failures are preserved.

Primary accepted/OFF/ON is **8.644460/8.557766/8.601689 ms**. Summed CPU time is **200.668/199.431/192.661 ms/request**. Original-eight paired geometric ON/accepted is **1.018644**, ON/OFF **1.014440**. Production remains unchanged; the all-eight tenfold CPU-reference goal remains unmet.

## Change and ordering

The private Crossbeam API pins one ordinary epoch guard, saves up to eight
victim-front indices with Acquire, issues one explicit SeqCst fence after all
snapshots, then tries the original back-load/buffer-read/pointer-check/front-CAS
body per victim. Every victim retains front -> fence -> back ordering. It returns
the first task, otherwise Retry if any probe raced, otherwise Empty. Guard and
snapshots end before returning; no task execution or parking occurs inside them.

Rayon preserves the first scalar victim probe, then groups later victims into
ordered tiles of up to eight references. The random circular victim order, self
skipping, aggregate Retry and complete-pass rescan on a race are retained. The
original OFF steal body is unchanged after removing the experimental dispatch.
The complete upstream deque source is a byte-identical prefix of the candidate;
new methods are appended. Source audit compares the extraction body byte-for-byte.
Injector, wakeup, arithmetic, proving-key preparation and job graph are unchanged.

This differs from [the earlier guard-reuse trial](../2026-10-25-batched-steal/README.md),
which retained a separate fence for every victim. It does not compose
[retained worker buffers](../2026-10-26-retained-deque-buffers/README.md).
Normal queue shrinking and deferred reclamation remain. The guard's own
publication barrier is retained before snapshots; the common explicit fence
after snapshots is also retained. No SC access is assumed to replace that fence.

Private cfg `zopro_front_snapshot_fence` and explicit
`ZOPRO_CPU_FRONT_SNAPSHOT_FENCE=1`, read once per Rayon registry, select ON.
Absent/zero select OFF. Scratch uses at most eight references and eight
WorkerIndex values on stack, plus ordinary call frames. One immutable registry
boolean is added. There is no new queue field, retained array, proving-key lookup
table, copied point buffer or per-proof heap allocation in this path. Earlier
snapshots and fewer pins can change retries, scheduling and collection cadence;
bounded scratch does not establish a whole-process memory bound.

The [ordering review](artifacts/ordering-review.md) links the primary paper and
Rust fence contract. Applying one common fence across these victims is our
inference; the cited paper does not prove this API. Native and bounded model
tests support the screen, without establishing full scheduler verification.

## Complete SDK latency

Ryzen 9950X; seven rotated/reversed fresh-process blocks compare frozen
accepted, the new executable OFF, and that same executable ON. All nine shapes
use 32 workers on CPUs 0–31 and 31 measured requests after 3 primes. Primary with
16 workers uses CPUs 0–15, one logical CPU per physical core, and 101 measured requests. Loading
and primes are excluded; fresh solving, randomness, proving, internal verification
and serialization are included. All latency finishes before CPU sampling. No
owned build, second benchmark, profiler or bulk audit overlaps latency.

Times are medians of process medians. Paired ratios are medians of within-block
ON/control ratios and need not equal the ratio of displayed medians.

| Case | Accepted ms | OFF ms | ON ms | ON/accepted paired | ON/OFF paired |
|---|---:|---:|---:|---:|---:|
| 1x2-w32 | 8.644460 | 8.557766 | 8.601689 | 1.000669 | 1.007332 |
| 2x2-w32 | 11.688494 | 11.796527 | 11.924308 | 1.021176 | 1.010403 |
| 4x2-w32 | 16.102491 | 16.227988 | 16.621601 | 1.031144 | 1.024450 |
| 8x2-w32 | 25.046416 | 24.775105 | 24.715732 | 1.010002 | 1.005036 |
| 12x2-w32 | 34.412207 | 34.678349 | 35.347171 | 1.023193 | 1.013989 |
| 16x2-w32 | 40.845555 | 41.163765 | 41.432863 | 1.002319 | 1.000633 |
| 24x2-w32 | 68.399372 | 68.324091 | 69.800836 | 1.017285 | 1.018891 |
| merge-24x1-w32 | 67.311198 | 68.276640 | 70.131049 | 1.046633 | 1.036669 |
| merge-54x1-w32 | 176.331010 | 180.022115 | 181.316627 | 1.025179 | 1.006651 |
| 1x2-w16 | 10.739513 | 10.458873 | 10.515269 | 0.985665 | 1.014900 |

The declared gate requires primary and original-eight aggregate improvement against both controls, primary wins in at least four of seven blocks, every case paired ratio ≤1.02, every individual block ratio ≤1.05, and OFF/accepted calibration within a factor of 1.03. There is no minimum gain floor, exclusion or unchanged rerun.

Speed gate: **fail**. Failed checks: **20**; OFF calibration fails for **0** cases. [Exact checks and every block ratio](artifacts/pilot-result.json) are retained.

## CPU work and process memory

Unprofiled CPU time is user+system time per measured request, median across processes. RSS is maximum whole-process high-water observation, including loading/primes; it is not minimum RAM or query-only memory.

| Case | Variant | CPU ms/request | Maximum RSS MiB | Voluntary switches/request |
|---|---|---:|---:|---:|
| 1x2-w32 | accepted | 200.668 | 113.25 | 423.7 |
| 1x2-w32 | off | 199.431 | 111.50 | 415.5 |
| 1x2-w32 | on | 192.661 | 111.50 | 576.2 |
| 12x2-w32 | accepted | 956.729 | 715.25 | 874.5 |
| 12x2-w32 | off | 957.843 | 732.75 | 880.6 |
| 12x2-w32 | on | 964.575 | 717.75 | 1757.5 |
| merge-54x1-w32 | accepted | 4841.853 | 1937.43 | 1817.9 |
| merge-54x1-w32 | off | 4938.846 | 1942.80 | 1816.5 |
| merge-54x1-w32 | on | 4930.285 | 1815.33 | 4010.0 |
| 1x2-w16 | accepted | 130.637 | 88.50 | 266.3 |
| 1x2-w16 | off | 129.551 | 86.25 | 269.5 |
| 1x2-w16 | on | 128.162 | 88.50 | 331.2 |

## Separate profiles

Nine profiles at 999 Hz follow all latency runs and use the monotonic clock. Shares are exclusive sampled CPU-period categories with symbol/inlining limits; they are not removable wall-time fractions. Zero means no attributed samples. Barrier work may move between epoch and inlined scheduler frames. Profiled CPU time is a separate observation.

| Case | Variant | CPU ms/request | Epoch % | Scheduler % | FFT % | Voluntary switches/request |
|---|---|---:|---:|---:|---:|---:|
| 1x2-w32 | accepted | 199.330 | 9.398 | 3.956 | 5.082 | 428.2 |
| 1x2-w32 | off | 200.071 | 10.008 | 4.165 | 4.788 | 417.7 |
| 1x2-w32 | on | 192.760 | 3.821 | 5.749 | 5.205 | 619.3 |
| 12x2-w32 | off | 957.889 | 5.313 | 4.215 | 26.828 | 932.8 |
| 12x2-w32 | on | 951.736 | 2.313 | 5.473 | 27.491 | 1821.5 |
| 12x2-w32 | accepted | 950.398 | 5.234 | 3.799 | 27.615 | 877.5 |
| merge-54x1-w32 | on | 4844.698 | 1.061 | 2.886 | 42.819 | 4196.1 |
| merge-54x1-w32 | accepted | 4873.120 | 1.918 | 2.320 | 42.004 | 1772.1 |
| merge-54x1-w32 | off | 4897.789 | 2.180 | 2.102 | 41.543 | 1879.7 |

299776 samples, zero lost records and one sampled PID per capture. Raw captures and expanded symbols are retained locally with hashes.

## Supplied M5 references

Historical supplied CPU/Metal timings, not fresh matched M5 measurements. Merge54 has no supplied reference.

| Shape | CPU reference ms | Metal reference ms | ON ms | CPU/ON | Metal/ON |
|---|---:|---:|---:|---:|---:|
| 1x2 | 37 | 24 | 8.601689 | 4.30× | 2.79× |
| 2x2 | 59 | 30 | 11.924308 | 4.95× | 2.52× |
| 4x2 | 109 | 48 | 16.621601 | 6.56× | 2.89× |
| 8x2 | 189 | 74 | 24.715732 | 7.65× | 2.99× |
| 12x2 | 284 | 101 | 35.347171 | 8.03× | 2.86× |
| 16x2 | 335 | 124 | 41.432863 | 8.09× | 2.99× |
| 24x2 | 528 | 176 | 69.800836 | 7.56× | 2.52× |
| merge-24x1 | 507 | 175 | 70.131049 | 7.23× | 2.50× |

## Validation and provenance

The runtime lock preserves 219 packages: 217 entries are byte-identical; deque and
Rayon-core lose only registry source/checksum for exact private path copies.
Their original licenses and upstream revisions are retained. Development locks
are separate and retain matching epoch 0.9.21 and utils 0.8.23. The four relevant
registry src/build/manifest trees are checked against pristine crate archives.
Actual scheduler/collector libraries and eight directly linked libraries are
pinned with the executable, source, compiler, helpers, inputs and verifier.

Each mode passes 44 deque tests (38 upstream plus 6 added), 113 Rayon tests
(two upstream tests remain ignored), and 109 core tests. Raw/cubic tests pass 2/6
with ON enabled.
Field kernels are unchanged; inherited 33-test field qualification is not claimed
as a new execution. Added checks cover ordered/mixed-flavor tiles, duplicate
references and bounds, stale front/retry aggregation, owner/stealer destruction,
concurrent growth/pop/stealing, nested guards, and actual deferred array release.
Five retired arrays remain allocated under an externally held guard and reclaim
after it drops. The allocation hook exists only in the unit-test executable.
Overloaded nested FIFO/cross-pool/panic/reuse tests also check jobs execute with
no steal guard held. First runtime compilation/tests pass; an unused test import
warning is preserved without changing frozen source.

The linked containing `wait_until_cold` routine grows from 1,661 to 5,757 bytes;
the scalar steal routine remains 356 bytes in both executables. The snapshot
helper is inlined into the scheduler routine. Exact linked disassembly is in
[codegen-review.json](artifacts/codegen-review.json) and [codegen.txt](artifacts/codegen.txt).
This static code-size difference does not establish cache misses or explain the
latency regression; no outlining or tile-size timing search follows this result.

Five instrumented Loom tests with a three-preemption bound check two-pair
store-buffering, a missing-common-fence negative control with the expected
failure message, owner-last-pop/two-thief uniqueness, publication and generation
identity. Atomic slots and retained model generations bound their scope. They
do not verify the original generic slot protocol, wrapping indices, epoch
collector or complete scheduler. The original queue protocol remains unchanged.

All nine shapes pass OFF/ON preflight with four workers before measurement. Every export
is independently Gnark-verified; fixture public bindings, uniqueness, 164-byte
compressed proof format and freshness against previous exports are audited.
All exit/stdout/stderr receipts, successful timing samples and source guards
are preserved.

No AWS resource is created and no new C7a/Aeglos comparison is claimed. Portable,
whole-SDK concurrency, pressure and very-large-circuit qualification remain
outstanding. Neither small stack arrays nor observed GC progress proves a
universal memory bound. Production is unchanged.

Total: **9192 complete SDK requests, 237 distinct independently Gnark-verified exports, 474 proof/public members**, plus two separately counted fixed source-fixture members. 609 input pins protect performance; 3537 prior identities are rejected before preflight.

See [declaration](artifacts/experiment.json), [source audit](artifacts/source-audit.json), [speed checks](artifacts/pilot-result.json), [closeout](artifacts/closeout.json), and [artifact index](artifacts/artifact-index.json). Nine raw perf captures and nine expanded streams are local with hashes and omitted from Git. The research PDF/full HTML remain local with links/hashes; executable/compiler caches are represented by exact build/link receipts.
