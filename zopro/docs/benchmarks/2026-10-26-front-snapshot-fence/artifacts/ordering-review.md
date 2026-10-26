# Front-snapshot ordering review

The private API keeps a normal epoch guard across every buffer read. It records
up to eight front indices with Acquire, then issues one explicit SeqCst fence,
then tries the original back Acquire/read/pointer-check/SeqCst front CAS for each
victim. No pointer or guard leaves the call. Original queue allocation, shrinking,
retirement, generic slot accesses and single-steal method remain byte-identical.

For a particular victim, the earlier front observation still precedes the fence
and back observation. Intervening loads concern other queues. Successful removal
still requires the unchanged front CAS and buffer identity check; an obsolete
front or generation can cause Retry. The inference is that a common fence keeps
the required per-victim relation, although observations across queues occur at
different times and the scan does not promise an atomic snapshot of all queues.

The baseline follows [Lê, Pop, Cohen and Zappa Nardelli, PPoPP2013](https://fzn.fr/readings/ppopp13.pdf),
Figure1 and Section5: their portable deque uses a sequentially consistent fence
between the thief's top/bottom observations, with a corresponding owner barrier.
That analysis explains why simply deleting the barrier is invalid. It does not
prove this multi-victim API; the common-fence argument above is our inference.

[Rust's fence contract](https://doc.rust-lang.org/std/sync/atomic/fn.fence.html)
distinguishes atomic/fence synchronization and SC ordering. We retain an explicit
SC fence after the snapshots, rather than assuming the pin's SC access before
the snapshots supplies that ordering. The pin's own publication barrier remains.

The five Loom tests use actual Loom Arc/atomic/fence/thread types and a declared
three-preemption bound. They check a two-pair store-buffering litmus, detect a
missing-common-fence negative control with the expected failure message, and
cover bounded two-thief/owner-last-pop, release publication and generation
changes. Slots use atomics and both model generations remain allocated. Thus
these tests cover modified ordering/control; they do not verify generic volatile
payload accesses, wrapping counters, the epoch collector or the full scheduler.
Upstream/native tests and complete verified SDK runs remain separate requirements.

Normal guard reuse changes collection cadence. Native tests observe retired
array deallocation after guards drop, but neither these tests nor source-level
bounded scratch storage establish whole-process or pressure behavior. No speed
claim follows from a barrier-count reduction or a bounded model passing.
