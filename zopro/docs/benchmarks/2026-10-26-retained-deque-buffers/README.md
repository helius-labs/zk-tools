# Retained worker-deque buffers

The fixed local speed screen fails. The candidate is not adopted and no unchanged timing rerun follows. All samples and failures are retained; production is unchanged. The all-eight tenfold CPU-reference goal remains unmet.

Primary ON is 8.607 ms versus accepted 8.593 ms. Summed CPU work falls from 199.884 to 194.323 ms/request, while voluntary switches rise from 414.5 to 688.4/request. The original-eight paired geometric result is 1.56% slower against accepted. Reducing the epoch category did not improve complete-SDK latency.

The private Crossbeam-deque 0.8.8 change keeps superseded worker-buffer
descriptors and arrays alive until the final worker/stealer `Arc` is dropped.
Retained queues only grow: the sum of old array capacities is below the current
capacity, so all arrays together occupy less than twice the high-water current
array storage. Descriptor and vector metadata are additional. The pool retains
that high-water allocation until destruction, a real memory-policy tradeoff.
There is no proving-key lookup table or additional copied point array.

Private cfg `zopro_retained_deque_buffers` and explicit
`ZOPRO_CPU_RETAINED_DEQUE_BUFFERS=1`, sampled once per process, select ON.
Absent/zero select OFF. Mode is immutable for each queue. Retained probes use
an unprotected guard while the shared queue handle keeps every old buffer alive.
The required SeqCst fence remains before the back-index load. Batch operations
pin normally if either source or destination uses ordinary epoch retirement.
Resizing still pins; only final shared-owner destruction converts retired
pointer handles to exclusive ownership and deallocates their arrays.

The source audit recovers byte-identical upstream single/batch extraction,
CAS and ordering bodies after removing the guard-selection block. Push and
reserve are unchanged; pop differs only at its two shrink-admission conditions.
The original generic slot-access protocol is preserved. Rayon scheduling,
injector behavior, arithmetic and proving-key preparation are unchanged.
This differs from the previously tested empty-victim precheck, bounded victim
search, batched epoch guards and St3 queue replacements.

## Fixed complete-SDK comparison

Ryzen 9950X; seven rotated/reversed process blocks compare frozen accepted,
the new executable OFF, and the same executable ON. All nine shapes use
32 workers on CPUs 0–31, 31 measured requests after three primes. The primary
16-worker diagnostic uses CPUs 0–15, one logical CPU per physical core, with
101 measured requests. Counts, order and gates were frozen before measurement.

Timed calls include fresh solving, randomness, proving, internal verification
and serialization. Loading and priming are excluded. All unprofiled latency
finishes before sampling. No owned build, other benchmark, profiler or bulk
audit overlaps latency. Times are medians of process medians; paired ratios
are medians of within-block ON/control ratios, and need not equal the ratio
of the displayed medians.

| Case | Accepted ms | OFF ms | ON ms | ON/accepted paired | ON/OFF paired |
|---|---:|---:|---:|---:|---:|
| 1x2-w32 | 8.592875 | 8.651315 | 8.606501 | 1.003733 | 0.997087 |
| 2x2-w32 | 11.715638 | 11.721300 | 12.003772 | 1.030985 | 1.023246 |
| 4x2-w32 | 16.020210 | 16.213257 | 16.683316 | 1.040544 | 1.022467 |
| 8x2-w32 | 24.680698 | 24.424749 | 24.560268 | 0.996458 | 1.007928 |
| 12x2-w32 | 34.479295 | 34.194708 | 34.748290 | 1.000074 | 1.009254 |
| 16x2-w32 | 41.279478 | 40.951905 | 40.939441 | 0.992713 | 0.999696 |
| 24x2-w32 | 68.300461 | 68.635936 | 69.981548 | 1.024418 | 1.018422 |
| merge-24x1-w32 | 67.342885 | 67.121031 | 69.397358 | 1.030771 | 1.033640 |
| merge-54x1-w32 | 176.647389 | 176.390000 | 178.577997 | 1.010678 | 1.011279 |
| 1x2-w16 | 10.522266 | 10.494102 | 11.132547 | 1.042800 | 1.058744 |

Original-eight paired geometric aggregate: ON/accepted **1.015613**, ON/OFF **1.015663**.

The declared gate requires primary and original-eight aggregate improvement against both controls, at least four of seven primary block wins, every case paired ratio ≤1.02, every block ratio ≤1.05, and OFF/accepted calibration within a factor of 1.03. There is no minimum improvement floor.

There are 32 failed primary, aggregate, case or block checks; the complete list is in [pilot-result.json](artifacts/pilot-result.json). OFF calibration fails for 0 cases. The observed speed failure is retained without exclusions or reruns.

## CPU work and process memory

CPU milliseconds are summed user+system time per measured request from unprofiled runs, shown as medians across processes. RSS is the maximum whole-process high-water observation, including loading and primes; it is not a minimum RAM requirement or a proof-buffer-only peak.

| Case | Variant | CPU ms/request | Maximum RSS MiB | Voluntary switches/request |
|---|---|---:|---:|---:|
| 1x2-w32 | accepted | 199.884 | 114.00 | 414.5 |
| 1x2-w32 | off | 200.013 | 114.00 | 424.8 |
| 1x2-w32 | on | 194.323 | 113.50 | 688.4 |
| 12x2-w32 | accepted | 962.398 | 714.50 | 875.6 |
| 12x2-w32 | off | 950.809 | 700.50 | 864.7 |
| 12x2-w32 | on | 948.640 | 714.25 | 2266.2 |
| merge-54x1-w32 | accepted | 4843.269 | 1929.89 | 1766.2 |
| merge-54x1-w32 | off | 4848.268 | 1866.83 | 1771.9 |
| merge-54x1-w32 | on | 4864.705 | 1788.96 | 5319.0 |
| 1x2-w16 | accepted | 131.243 | 88.25 | 270.6 |
| 1x2-w16 | off | 131.375 | 87.50 | 272.1 |
| 1x2-w16 | on | 128.725 | 88.00 | 348.1 |

## Separate CPU profiles

The nine profiles follow all latency runs, use the monotonic clock and 999 Hz sampling, and cover primary, 12x2 and merge54 for all three variants. Percentages are exclusive sampled CPU-period shares with symbol/inlining limits, not removable wall-time fractions. Profiled CPU times are separate observations.

| Case | Variant | CPU ms/request | Epoch % | Scheduler % | FFT % | Voluntary switches/request |
|---|---|---:|---:|---:|---:|---:|
| 1x2-w32 | accepted | 199.876 | 9.767 | 3.829 | 4.695 | 426.0 |
| 1x2-w32 | off | 200.738 | 9.585 | 3.931 | 4.501 | 451.3 |
| 1x2-w32 | on | 195.493 | 0.000 | 9.723 | 4.969 | 704.7 |
| 12x2-w32 | off | 959.788 | 5.193 | 3.819 | 26.546 | 929.2 |
| 12x2-w32 | on | 951.055 | 0.000 | 7.537 | 27.133 | 2455.6 |
| 12x2-w32 | accepted | 960.032 | 5.699 | 4.047 | 26.976 | 937.0 |
| merge-54x1-w32 | on | 4995.006 | 0.000 | 3.751 | 40.687 | 5283.2 |
| merge-54x1-w32 | accepted | 4786.932 | 2.214 | 2.212 | 42.507 | 1866.0 |
| merge-54x1-w32 | off | 4926.849 | 2.061 | 2.049 | 41.223 | 1833.0 |

302154 samples; zero lost records; exactly one sampled PID per capture. Direct and expanded symbol streams and their hashes are preserved. Zero category percentage means no attributed samples. The required fence remains in the steal body: the linked native ON branch lowers it to a locked OR on stack storage before loading the back index, as preserved in [disassembly](artifacts/steal-disassembly.txt). Some attribution can therefore move from epoch helpers into scheduler frames; sample-category removal does not establish eliminated synchronization cost.

## Worker-buffer capacity observations

After performance measurement, the same frozen source is rebuilt with a separate trace cfg. Constructor/resize/final-drop events record actual generic payload size and queue capacity; no trace code is compiled into the latency executable. All nine shapes run OFF and ON at 32 workers, four complete SDK requests each including primes, with independent exported-proof verification.

The table shows ON only. Bytes sum per-queue observed maxima, which may occur at different times. Descriptor bytes include current/retired descriptors and retired-vector capacity. They exclude pointed-to jobs, fixed Inner/Arc/thread storage and all prover allocations. OFF traces omit deferred epoch allocations and cannot provide a physical-memory baseline. Tracing changes scheduling, so these observations do not establish a universal or uninstrumented peak.

| Shape | Queues | Largest final capacity | Retired capacity sum | Observed array bytes | Descriptor/vector bytes | Growth events |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 64 | 64 | 0 | 65536 | 1024 | 0 |
| 2x2 | 64 | 64 | 0 | 65536 | 1024 | 0 |
| 4x2 | 64 | 64 | 0 | 65536 | 1024 | 0 |
| 8x2 | 64 | 64 | 0 | 65536 | 1024 | 0 |
| 12x2 | 64 | 64 | 0 | 65536 | 1024 | 0 |
| 16x2 | 64 | 64 | 0 | 65536 | 1024 | 0 |
| 24x2 | 64 | 64 | 0 | 65536 | 1024 | 0 |
| merge-24x1 | 64 | 64 | 0 | 65536 | 1024 | 0 |
| merge-54x1 | 64 | 64 | 0 | 65536 | 1024 | 0 |

Every traced run constructs 64 worker deques: 32 ordinary queues and 32 broadcast queues in the pinned Rayon registry. None grows beyond its initial 64 entries. Observed array storage is 64 KiB in total, with no retired generations. This is runtime queue storage rather than a proving-key cache.

The geometric bound is checked at every retained event. Unit tests additionally force repeated 1,024-entry bursts and multiple retained generations, verify array deallocation after the last stealer drops, and check that pending payloads are dropped once. These observations do not replace long-lived-pool, low-memory, concurrency or very-large-circuit qualification.

## Supplied M5 references

These are the supplied historical CPU/Metal timings, not fresh matched M5 runs. Merge54 has no supplied reference.

| Shape | CPU reference ms | Metal reference ms | ON ms | CPU/ON | Metal/ON |
|---|---:|---:|---:|---:|---:|
| 1x2 | 37 | 24 | 8.606501 | 4.30× | 2.79× |
| 2x2 | 59 | 30 | 12.003772 | 4.92× | 2.50× |
| 4x2 | 109 | 48 | 16.683316 | 6.53× | 2.88× |
| 8x2 | 189 | 74 | 24.560268 | 7.70× | 3.01× |
| 12x2 | 284 | 101 | 34.748290 | 8.17× | 2.91× |
| 16x2 | 335 | 124 | 40.939441 | 8.18× | 3.03× |
| 24x2 | 528 | 176 | 69.981548 | 7.54× | 2.51× |
| merge-24x1 | 507 | 175 | 69.397358 | 7.31× | 2.52× |

## Validation and provenance

The private runtime lock retains all 219 package entries, changing only
Crossbeam-deque to a private path. The other 218 entries remain byte-identical.
Upstream tests use separate development locks with matching deque/epoch/utils
runtime versions. Actual registry source trees, build scripts and manifests
for deque/epoch/utils/Rayon-core are byte-checked against pristine crate archives.
The directly linked libraries and actual scheduler/collector library files
are pinned along with the complete SDK executable.

Both modes pass 45 Crossbeam tests (38 upstream plus seven added lifetime,
growth, destruction, mixed-batch and concurrency tests), 112 Rayon tests
(two upstream tests remain ignored), and 109 native core tests. Raw/cubic
prover tests pass 2/6. Field kernels are unchanged; inherited 33-test field
qualification is not represented as a new run. The two instrumented Loom
tests cover retention/publication/final shared-handle destruction, including
a premature-reclamation negative control. They do not verify extraction,
the original generic slot protocol or the complete scheduler.

Initial compilation failures are retained: the private deque additions needed
explicit std Vec/macro imports in the upstream no_std crate, and the model
needed an explicit borrow through a raw pointer. Separate v2 snapshots fix
those imports/borrows and register private cfg names without changing the
queue algorithm. No SDK timing preceded those fixes.

The initial closeout helper expected a 256-byte raw-coordinate proof. The SDK
exports the 164-byte compressed Gnark representation documented by
`Proof::to_gnark`; the audit was corrected and rerun without rerunning any SDK
measurement. The initial audit source and failing log are retained.

Preflight checks all nine shapes OFF/ON at four workers before performance.
Fixture public-input hashes, proof uniqueness and freshness are mandatory
before timing and independently audited again on closeout. Every exported
proof is checked with the pinned Gnark verifier. Exact commands, source and
binary hashes, test/exit/stdout/stderr receipts, all samples, proof bytes and
profiles are preserved. No AWS resource is created by this local experiment;
there is no new Aeglos or C7a measurement here.

No portable, whole-SDK concurrent, memory-pressure or very-large-circuit
qualification is claimed. Production remains unchanged pending qualification.

Total: **9264 complete SDK requests, 255 distinct independently Gnark-verified exports, 510 proof/public members**; 9 captures. These counts include preflight, performance and separate capacity tracing, and exclude two fixed source-fixture members. Performance uses 847 pinned inputs and rejects 3282 prior proof identities before preflight.

Start with [declaration](artifacts/experiment.json), [source audit](artifacts/source-audit.json), [full speed checks](artifacts/pilot-result.json), [capacity observations](artifacts/trace-result.json), [independent closeout](artifacts/closeout.json), and [artifact index](artifacts/artifact-index.json). Nine raw perf captures and nine expanded streams are kept locally with hashes and omitted from Git; executable/compiler caches are represented by exact build/link hashes.
