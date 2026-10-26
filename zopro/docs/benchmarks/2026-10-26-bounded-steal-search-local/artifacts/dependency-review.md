# Dependency review

The pristine Rayon-core 1.13.0 package archive has SHA256
22e18b0f0062d30d4230b2e85ff77fdfe4326feb054b9783a3460d8435c8ab91,
matching the accepted lockfile. Its recorded upstream commit is
7af20d7692d5decbcb4adcaa079cd607e3e50814. Both upstream licenses remain.

The 219-entry SDK lockfile changes only the Rayon-core package's registry
source/checksum for its private path. Every other package block is identical.
The isolated upstream-test lockfile uses exact SDK Crossbeam deque0.8.8,
epoch0.9.21 and utils0.8.23 entries, including checksums. Separate test-only
Loom dependencies do not enter the SDK graph.

The mechanism is motivated by [Tokio's throttling discussion](https://tokio.rs/blog/2019-10-scheduler#throttle-stealing)
and the cached [Tokio implementation](https://github.com/tokio-rs/tokio/blob/75fef53d0a8590c2d1dbb63672aa7b7d1ef51155/tokio/src/runtime/scheduler/multi_thread/idle.rs).
Tokio counts searching worker roles and coordinates wakeups with that state.
This experiment limits one finite victim sweep, releases its permit before
execution and retains Rayon's existing empty-scan count and parking checks.
The distinction is material: source similarity does not establish the same
progress properties or performance. No Tokio scheduling code is copied.

The added admission code uses safe Rust. Queue extraction, atomics and
reclamation are unchanged. A permit adds counter acquisition/release overhead;
the performance experiment must account for that cost and deferred yields.
Loom checks the actual admission module with instrumented atomics at a
two-preemption bound. It does not model the entire Rayon scheduler or prove
unbounded fairness. Native held-credit injection/broadcast, nested scopes,
concurrent callers, FIFO, panic, cross-pool and overloaded tests cover broader
integration behavior.
