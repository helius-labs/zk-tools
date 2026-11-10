# C7a arithmetic worker coverage

All **96 registered Zopro workers** have G1-stream and borrowed-G2 arithmetic
samples in the saved 96-worker primary recording. Epoch samples are also spread
across all workers. This excludes a fixed worker subset doing all sampled
arithmetic throughout the recording. It does **not** establish that 96 arithmetic
tasks were simultaneously available, that individual proofs were balanced, or
that the workers were usefully occupied at every stage.

This descriptive follow-up uses the four existing recordings from the
[accepted C7a comparison](../2026-11-10-accepted-current-c7a176/README.md).
There are no new recordings, proofs, latency measurements, cloud resources or
production changes. The original all-eight CPU 10× objective remains unmet.

## Distribution across Zopro workers

Periods are summed over each whole 251-request recording. The external caller
is excluded below; all registered workers have samples and remain represented.
Effective thread count is `(sum periods)^2 / sum(periods^2)`. It measures
whole-recording concentration, **not instantaneous concurrency**. Arithmetic can
move between workers over successive proofs.

|Sampled work|Workers|Workers with samples|Effective thread count|Threads covering 90% of period|
|---|---:|---:|---:|---:|
|All sampled work|32|32|31.87|29|
|All sampled work|96|96|95.52|86|
|G1 stream|32|32|31.70|29|
|G1 stream|96|96|87.76|82|
|Borrowed G2 apply|32|32|31.54|28|
|Borrowed G2 apply|96|96|91.25|81|
|Crossbeam epoch|32|32|31.07|28|
|Crossbeam epoch|96|96|95.61|86|

The busiest worker accounts for 3.67% / 2.01% of G1-stream period at 32 / 96
workers, and 3.77% / 1.61% of borrowed-G2 period. Arithmetic is distributed
broadly over the recording rather than permanently confined to a few threads.
These aggregate numbers cannot separate temporary work shortages, stranded
ready tasks, or stalls in executing tasks.

Aeglos's noncaller sampled threads have effective counts of 32.99 and 94.55.
Their sampled thread counts are 40 and 104, including auxiliary threads whose
pool membership is not established by this audit. Rust-symbol categories are
not comparable categories for its C++ work. These distributions establish no
relative useful-utilization or idle-time result.

## Instruction cross-check and existing evidence

The [earlier coordination-site audit](../2026-11-10-aws-coordination-sites176/README.md)
already identifies epoch publication and participant scanning. This follow-up
cross-checks those locations from a new TIME/MMAP projection of the original
raw events; it is not a new discovery of those hotspots.

|Accepted ELF instruction|Context|32-worker period, billions|96-worker period, billions|
|---|---|---:|---:|
|`0x112732`|After global-epoch load, before pin-publication CAS|4.127|78.406|
|`0x112741`|Immediately after pin-publication CAS|7.969|123.223|
|`0x106527`|Immediately after participant-epoch load|3.057|85.880|

Together these three instruction sites account for 7.17% / 45.36% of total
sampled period. Sample skid, preceding instructions and coherence effects can
contribute. A `mov` or `xor` at the sampled IP is not necessarily the expensive
operation. This does not measure cache-line transfers, fence latency, task
readiness, idle time or removable proof wall time.

The pinned Crossbeam 0.9.21 source publishes a participant epoch when entering
an outer guard, and periodically scans participant epochs for reclamation.
Crossbeam deque 0.8.8 enters epoch protection before its empty-queue return.
Rayon core 1.13.0 may visit every other worker in a victim sweep. These mechanisms
provide a possible source of increasing coordination and coherence costs;
their presence and sample weights do not prove causation.

The [historical C7a phase diagnostic](../2026-10-25-phase-cpu-c7a/README.md)
located growing coordination work in solver and FFT phases as well as MSMs.
That used older source and is not a phase measurement of the current accepted
build. The [historical local readiness trace](../2026-10-25-current-readiness/README.md)
did not find a large isolated final query-callback tail. Existing
[scoped dataflow](../2026-10-25-proof-dataflow-c7a/README.md),
[worker groups](../2026-10-25-worker-groups-c7a/README.md),
[empty-victim probing](../2026-10-22-universal-empty-steal-probe/README.md),
[bounded stealing](../2026-10-26-bounded-steal-search-c7a/README.md), and
[ready-victim advertisement](../2026-11-10-ready-victims-sdk176/README.md)
retain their recorded results. Neither a new queue nor a generic task-graph
rewrite follows from the current evidence.

The remaining discriminating observation is **time-resolved ready and executing
work within current proof phases**, especially where a larger pool adds CPU
without shortening the phase. A follow-up observer needs bounded buffers,
overflow detection, verified proofs and an explicit perturbation check. It
must distinguish work not yet created from ready work awaiting dispatch, and
separate those from active tasks delayed by memory accesses. This audit does
not collect those observations or select an optimization. Further aggregate
epoch-share or whole-recording worker-balance audits would not resolve that
question.

## Validation and limits

All **97,089 samples** reconcile by the full PID/TID/period/IP multiset between
the original symbol streams and new raw-event projections. Exact profile
sample and period totals match the parent quality data. All seven projection
and ELF-inspection commands succeed on their first executions. Two subsequent
Rust analysis/review commands also succeed on their first executions, with
115 frozen input hashes.

The separate reader uses original mangled symbol tokens to recompute thread
bins and concentration statistics without importing the producer or demangler.
It checks raw-event reconciliation, monotonic sample spans, all 96 epoch
instruction sites, ELF load-bias arithmetic, and command stream hashes. The
producer additionally requires exact native symbol-range/name matches and
instruction-start matches for every selected epoch sample. Both use the same
saved event streams and native disassembly; neither is an independent PMU or
compiler verification.

Sampling is adaptive-period 199 Hz; absolute period coverage remains
unvalidated. TIME is recorded, CPU is not. Zopro worker identities come from
the parent's placement receipt; this audit makes no new placement claim.
There are no per-proof absolute boundaries or task lifecycle events in these
captures, so exact per-proof concurrency cannot be reconstructed from them.
The new scheduler grouping includes all symbols containing `rayon_core` or
`crossbeam_deque`; it is broader than the parent's frozen category rules and
is retained descriptively in JSON, not substituted for the parent's shares.

Accepted timing, RSS and M5 CPU/Metal comparisons remain in the
[full C7a report](../2026-11-10-accepted-current-c7a176/README.md) and
[latest local control measurements](../2026-11-10-weighted-owned-roots-sdk176/README.md).
This audit establishes no new speedup, minimum RAM, memory-pressure, mobile,
large-circuit or concurrent-throughput result. The verified-export ledger
remains **50,005**.

Evidence: [analysis](artifacts/analysis.json), [independent review](artifacts/review.json),
[design](artifacts/design.md), [source and receipts](artifacts/evidence.tar.gz),
[publication check](artifacts/publication.json), [checksums](SHA256SUMS).
