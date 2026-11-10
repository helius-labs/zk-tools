# Bounded Rayon task lifecycle observer

Goal remains verified single-proof CPU AVX >=10x EACH supplied M5 CPU shape,
with CPU+Metal comparisons. Current accepted176 is unchanged and 0/8.
Prior turn made progress with independently reconciled C7a arithmetic-worker
coverage, but whole-recording concentration cannot measure task readiness.
No more flat-sample aggregation; this stage implements the missing observer.

Diagnostic-only private copy of exact Rayon core1.13.0, preserving JobRef size,
all original queue operations, fences, algorithms and scheduling decisions.
No new prover pool, key tables, solver image, production cap or arithmetic.
Record publication BEFORE and AFTER original queue pushes, JobRef start/end,
and the join fast path that pops its own StackJob and calls run_inline without
JobRef::execute. Include injector, broadcasts and FIFO indirection. Record full
victim-search invocation begin/end, not every victim or epoch pin.

Each thread has a preallocated bounded Vec of fixed64-byte events. Capacity is
chosen before a capture; push never grows after allocation and overflow marks
the whole capture incomplete. SDK protocol will freeze capacity before any
preflight. Native tests deliberately exercise tiny overflow separately.
No central hot-path trace counter or shared append lock. One process-wide
exclusive capture lease; ACTIVE is read-only during recording. Timestamps use
std Instant and per-thread buffers. TLS buffer initialized/touched before capture;
no field values, scalars, witness values or key data in events.

Use pointer+execute-function pair as an opaque job key; do not dereference it.
Keys may be reused after completion, and FIFO wrapper keys can overlap. Do not
pretend these are globally unique task IDs. Validate event matching, account
for repeated keys, and mark ambiguous per-task matches rather than choosing
the most favorable correspondence. Aggregate enqueue/start/end counts remain
available with retained ambiguity. Match FIFO inner and wrapper layers separately.

Publication interval brackets original push, so start may precede AFTER on a
fast thief. Publication-to-start delay is an interval with lower bound
max(0,start-AFTER) and upper bound max(0,start-BEFORE), including observer
cost. It is not exact queue residence or scheduler overhead. A job claimed but
not yet started remains in this dispatch-delay interval. Work after the end of
this interval may be a nested wait, not arithmetic. No claim of exclusive useful
CPU time or memory-stall cause from these events alone.

Per-thread search scopes and job scopes must unwind cleanly. Preserve explicit
previous context in a non-Send guard; do not infer semantic application ancestry
from a waiting sibling's TLS. Job-parent IDs are execution-stack context only.
Coarse phase markers can be emitted from any prepared thread; caller-to-worker
prover entry needs no TLS capture propagation because this recorder is explicitly
process-wide and single-request. This is not concurrent-proof attribution.

Preparation and draining require an inactive capture and quiescent scoped work.
A preparation lease count permits parallel worker setup and atomically excludes
capture start until every setup lease has ended. This synchronization is outside
recording; it adds no per-event shared-counter update.
Stop disables new events while retaining the capture lease until all snapshots
are validated. Job callbacks can signal their latch before the execution guard
drops, and idle workers can be mid-search when the caller stops. Retain their
END/search boundary tails, clipping residence calculations to the stop time;
do not confuse these tails with new proof work. Other new work or application
phase completion after stop invalidates capture. Outstanding scope depth at
draining, unprepared participation, overflow and reset/nested-session misuse
also invalidate capture.
No partial/incomplete trace may justify a causal or performance conclusion.

First stage: exact source recovery/pins and new recorder tests on real pools,
full upstream Rayon library/tests under --test-threads=1, disabled-CFG compilation,
repeated-key/FIFO/nestedjoins/unwind/overflow and exclusive-session checks.
Do not claim a new SDK, proof or AWS result at this stage. Follow-up integrates
the current176 prover and freezes observer OFF/ON/accepted calibration before
any performance capture. Local is correctness/perturbation qualification;
C7a96physical is the actual scaling discriminator. Every new proof must seed
the verified-export ledger50005, not the old AWS/audit ledger.
