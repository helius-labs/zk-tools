# Bounded task lifecycle recorder qualification

A private Rayon task recorder passes **118 tests**, including **eight new
recorder tests**; two upstream tests remain ignored. Compilation with the
recorder excluded also passes. No prover optimization is adopted, no SDK proof
or AWS measurement runs, and observer overhead has not yet been measured.

The [C7a worker-coverage audit](../2026-11-10-c7a-worker-residence176/README.md)
cannot distinguish temporary shortages of ready work from dispatch delays.
This component supplies publication and execution hooks for that question,
rather than another aggregation of the same CPU samples.

## Recorded events

The private Rayon core 1.13.0 copy brackets existing queue publication and
records job start/end plus whole victim-search invocation boundaries. It
covers local queues, external injection, broadcasts and FIFO indirection.
The join path that retrieves its own queued job and calls `run_inline` is
instrumented separately; it bypasses normal `JobRef::execute`.

Events contain clocks, kinds and opaque job/queue identifiers, without reading
job payloads. `JobRef` remains two pointer-sized words. Nine reversible
insertions affect four upstream files; two new modules hold recorder and tests.
Queue operations and memory ordering retain their original implementations.
The broadcast iterator additionally enumerates owner indices for tracing.

Each thread writes to its own preallocated, touched, bounded event vector.
An event is 64 bytes; reaching the declared limit increments an overflow count
instead of growing the vector. There is no shared append lock or hot-path
global event counter. A preparation lease allows concurrent worker setup while
excluding capture start. One capture owns the process-wide lease until draining
and validation finish. This is a single-request diagnostic, not attribution
for concurrent proofs or a product memory limit.

Publication-to-start time is an interval because another worker can begin
before the publisher records its post-push marker. It includes observation
cost and any delay after claiming a job; it is not exact queue residence.
Pointer/function keys can be reused, and FIFO wrapper keys can overlap.
Per-task analysis must retain ambiguous matches rather than assume uniqueness.
Job nesting denotes execution-stack context, not semantic proof ancestry or
exclusive useful arithmetic time.

Callbacks may signal completion before the outer execution guard drops, and
idle searches can cross the caller's stop marker. Those boundary tails are
retained and must be clipped for residence calculations. Other new work or
application phases after stop, unprepared participants, missing snapshots,
overflow and outstanding scope depth invalidate capture.

## Qualification

The eight tests exercise nested joins and reused keys on actual 1/4/16/32-worker
pools with requested 512 KiB stacks; FIFO/scoped jobs and broadcasts; callback
execution counts; panic unwinding; overflow without growth; capture/reset
ownership; unprepared participation and late application phases; disabled
hooks and event/job sizes; and capture exclusion during concurrent preparation.
Normal job and inline-job publication/start/end totals reconcile in the tests.
Their event-key matching checks aggregate counts and per-thread scope nesting;
they do not yet qualify a per-task dispatch-delay parser.

The final run passes 109 library tests and nine integration tests across eight
test suites. Two ignored stack-helper cases remain ignored in their parent
summary. The disabled-CFG compilation emits an unused owner-index warning.
There is no phone/ARM, memory-pressure, concurrent-proof or proving-speed claim.

The initial compile fails before tests because three public observer types lack
the `Debug` implementation required by the upstream crate's lint. That source
and failure are preserved. The repaired revision passes its tests and disabled
build. Review then identifies a setup-versus-capture race; a preparation lease
and concurrent-setup test are added, followed by fresh qualification of the
changed source. Earlier successes are not substituted for the final results.

Independent closeout checks 297 final input hashes, every command exit and
stream hash, all eight new successful test names, suite totals, both prior
source revisions, exact reversal to upstream and all 176 accepted production
identities. Five qualification commands are retained: four successful and the
initial failed compile. Helper compilation/review commands are outside that count.

The next stage must integrate the current accepted SDK, add coarse proof-phase
markers, validate complete exported traces and fresh proofs, and measure
accepted/OFF/ON observer perturbation before interpreting a C7a capture. The
recorder alone does not establish task readiness, dispatch delays or the cause
of the AWS scaling gap. No new runtime optimization follows from this stage.

Accepted [AWS CPU/Metal comparisons](../2026-11-10-accepted-current-c7a176/README.md)
and [latest local controls](../2026-11-10-weighted-owned-roots-sdk176/README.md)
remain unchanged. The full all-eight CPU 10× goal remains active and unmet,
**0/8**. The verified-export ledger remains **50,005**.

Evidence: [design](artifacts/design.md), [review](artifacts/review.json),
[source and receipts](artifacts/evidence.tar.gz), [publication](artifacts/publication.json),
[checksums](SHA256SUMS).
