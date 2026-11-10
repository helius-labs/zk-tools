# Existing C7a worker and instruction residence

Read-only follow-up to accepted-current-c7a176. No new build of the prover,
recording, SDK request, proof, cloud resource, timing comparison or adoption.
The previous strategic review yielded a work-availability hypothesis, not a
causal finding. Inspect current authoritative inputs before selecting a change.

Use all four original cycles profiles (accepted/Aeglos, 32/96 workers), with
their successful original symbol projections, receipts and quality totals.
Project TIME and MMAP records from the exact original perf.data files for a
new question. This is not a replay of an old decoder for bookkeeping. Retain
every positive-period sample, including startup/ramp samples and unknowns.
Reconcile PID/TID/period/IP against the original projections as multisets;
timestamps may reorder equal-time records. Require exact sample and period
totals against original quality.json, no lost events, and successful commands.

Describe whole-recording per-thread period distributions: all work, epoch,
scheduler, G1 stream and borrowed G2; arithmetic can migrate across workers
between proofs. Effective count = squared sum / sum of squares, not simultaneous
concurrency. Report worker coverage and concentration; do not treat counts as a
measure of a single proof's balance or derive idle time from missing samples.
Keep external caller and auxiliary Aeglos threads separate where identities
are established. Do not infer per-thread CPU placement (CPU was not sampled).

Use original resolved names, native ELF program headers and MMAP file offsets
to map accepted epoch IPs to instructions in the exact accepted executable.
Validate address-to-symbol ranges. Distinguish TLS/guard, pin fence, participant
scan and other locations descriptively, retaining unclassified IPs. Instruction
residence includes skid and stalls; sampled instruction is not a latency cause.
No posthoc percentage is a removable exclusive wall-time fraction.

Sampling is frequency 199Hz (adaptive periods), not fixed 16M. No absolute-period
coverage validation exists. TIME is monotonic; no absolute per-proof boundaries
were recorded. Therefore do not claim exact stage concurrency, individual task
durations, task readiness or a complete proof critical-path reconstruction.

Publish sources, hashes, command receipts, derived per-thread/IP data and a
concise factual conclusion. Reuse raw capture from the parent report; do not
duplicate it. Verify reconciliation with a separate raw reader. No performance
gate or candidate is selected by this descriptive audit. Review old epoch,
steal and task-decomposition trials before proposing any implementation.
