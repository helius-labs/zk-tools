# Next design to audit: deque buffer lifetime

This is a research direction, not an implemented or measured change.
Close this experiment and push it before starting another.

The pinned Crossbeam-deque 0.8.8 source pins an epoch guard before checking
whether a victim is empty. Pinning also supplies a required sequentially
consistent fence. Resizing retires the old allocation through that guard.
The previous C7a profile attributes roughly48.5% of primary user-cycle
weights at96 workers to epoch pin/advance; it does not establish a removable
wall-time fraction. Existing batched-guard, empty-GC, timed-search, static
St3 and finite-scan-admission results remain unchanged.

A distinct possibility is preserving the current FIFO/LIFO extraction and
CAS algorithm while retaining old deque buffers until the final shared
queue owner/stealer is dropped. If capacities only grow geometrically, old
array capacity sums to less than current capacity; current plus old array
storage stays below twice the high-water capacity, plus descriptor metadata.
Disabling shrink is necessary for that bound: retaining repeated growth and
shrink generations would be unbounded. This is a runtime-metadata tradeoff,
not a proving-key table, and it needs actual queue-capacity/drop measurements,
low-memory and large-circuit qualification. A pool retains its high-water
allocation until destruction, which is material for a long-lived server.

Retained ownership might permit an unprotected epoch guard on precisely
those worker-buffer accesses, eliminating repeated pinning. This is unsafe
unless every referenced allocation remains alive through all possible
stealer accesses. The required SeqCst fence MUST remain on every probe;
removing the guard must not remove the fence it previously supplied.
Mixed-mode batch stealing must protect both source and destination buffers.
Normal epoch mode and injector behavior should remain intact.

Before implementation, audit prior reports for this exact ownership design,
all pin/resize/shrink/deferred-destruction sites, final shared-owner drop,
FIFO/LIFO and batch semantics. Keep the exact upstream dependencies/licenses.
Before SDK timing, require upstream tests in both modes, growth/owner-drop/
stealer-drop/payload-drop tests, properly instrumented memory-lifetime models
and overloaded Rayon integration tests. Do not describe a partial model as
verification of the complete scheduler. Profile complete verified requests;
faster empty probes alone have already failed to predict SDK speed.

The current serial-FFT candidate remains preserved: every32-worker condition
passes, but its original overall screen fails physical16 OFF calibration.
Do not relabel that failure or repeat its unchanged timing matrix. Further
compatibility/resource qualification or a distinct diagnostic remains possible.
