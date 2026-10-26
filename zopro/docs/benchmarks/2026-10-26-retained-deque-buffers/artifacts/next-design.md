# Next audit: front snapshots with a common fence

This is an unimplemented research direction. Close and push the retained-buffer
experiment before starting another. Its failed speed gate remains failed.

The completed trial removes attributed epoch samples but retains a full barrier
on every victim probe. The exact linked native Stealer ON branch contains a
locked OR on stack storage before the back-index load. It is not appropriate
to infer mfence instructions from the Rust source or to equate removal of the
epoch category with removal of that synchronization. Primary CPU work falls
about 2.8%, but switches rise and the original-eight latency aggregate regresses
about 1.6%. Queue arrays stay at64KiB with no retired generations in all traced
shapes; this observation is not a universal memory bound.

The prior10/25 bounded guard-reuse experiment shares one guard across eight
victims but retains each victim's separate SeqCst fence. Its negative result
does not test taking several front-index snapshots before one common fence.
README/design searches found no exact common-fence/snapshot design, but a
complete source/handoff history audit is still required before declaring one.

A candidate design to prove or reject:

- Keep ordinary epoch-protected buffers, shrinking and reclamation. Do not
  compose this failed retained-buffer mode or change the payload-slot protocol.
- Pin one ordinary guard for a small bounded victim tile. Load each victim's
  front index first, execute one explicit SeqCst fence, then perform the original
  back load, buffer read, pointer check and front CAS per victim using its saved
  front. Every individual victim still has front-load -> fence -> back-load.
- The common guard must protect every buffer read; end it before executing a
  job or parking. Do not rely on an SC access being interchangeable with an
  SC fence. Do not weaken/elide the explicit common fence.
- Use bounded stack snapshots/references, no heap table or proving-key cache.
  Eight victims is an initial design point, not a search result. Earlier front
  snapshots may produce more retries, and speculative front loads can waste
  work when the first victim succeeds. Audit existing victim/census evidence
  before deciding whether to preserve an initial scalar probe; freeze one
  concrete design and protocol before implementation/timing.
- Preserve victim ordering, self skipping, retry handling, single-job delivery,
  injector, wakeup and worker-entry semantics. A new safe dependency API needs
  an ordering argument and actual instrumented models, not an exposed integer
  snapshot that callers could use without the fence or guard.

The ordering argument is a hypothesis until audited against the single-victim
proof and bounded concurrent models (owner pop of the last task, publication,
multiple thieves, generation changes, both FIFO/LIFO). Tests should include a
broken-ordering negative control with real modelled synchronization, plus
upstream tests, overloaded scheduler integration and independently verified
complete SDK requests. Do not call a partial model full scheduler verification.

Guard reuse changes collection cadence, which needs resize/reclamation and
pressure checks even with ordinary buffers. Static instruction counts, faster
empty probes or fewer barriers are not complete-proof speed evidence. Use the
same frozen accepted/OFF/ON controls and full-shape checks; preserve negative
results. C7a remains a separate later protocol with tested target binaries and
resource cleanup, never a claim based on these local results.

Actual structure layout must be measured: Rust representation can reorder
fields. The trace Inner is256bytes; source field placement is not evidence of
an added cache line. Do not chase a hypothetical layout regression.
