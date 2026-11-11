# CPU performance playbook

Keep useful improvements moving toward adoption while preserving correctness,
bounded memory and honest measurements. Follow the [xtask workflow](../xtask/README.md)
for execution. The [candidate register](cpu-performance-candidates.json) records
the current G2 scalar-grain candidates; it is manually maintained, not an
enforced xtask state machine. Historical reports retain their original protocols
and decisions.

## Keep the goal, experiment and candidate separate

The research goal remains verified single-proof CPU AVX latency at least 10×
faster than each supplied M5 CPU reference, with CPU and Metal comparisons for
all eight supplied shapes. Report merge 54×1 separately, without inventing an
M5 reference. A completed report or a failed experiment does not finish that goal.

Identify every candidate by its source snapshot, configuration, toolchain,
dependency/build receipts and measured SDK digest. Retain its binaries and
evidence. A cleanup, cfg removal, changed build configuration or relink that
produces a new artifact gets a separate record, linked to its parent. Equivalent
source operations do not establish equivalent performance.

Record four independent conclusions, each with its tested scope and evidence:

|Area|Record|
|:---|:---|
|Correctness|Passed, failed or untested; tests, proof bindings and external verification|
|Resources and compatibility|Memory, allocation, portability, concurrency and other applicable checks; inherited versus newly measured coverage|
|Performance|Passed declared criteria, failed declared criteria, inconclusive, or measured only as a control; identify host, shape, workers and comparator|
|Adoption|Pending with concrete remaining work, adopted with commit, or not adopted with an explicit reason|

Never compress these into an unexplained “candidate failed.” A performance
qualification failure does not imply an implementation bug. A bound that cannot
exclude a regression is inconclusive about that regression; it is not proof of
zero benefit or proof of a slowdown. It still fails a declared nonregression gate.

## Finish qualification of a promising candidate

1. Review existing source, profiles and experiment history. Write a distinct
   hypothesis and the whole-proof effect it could plausibly explain.
2. Implement the smallest Rust change. Check relevant correctness and resource
   behavior, including portable and concurrent behavior where affected. Reuse
   applicable qualification when its source and assumptions remain valid;
   identify what is inherited. Avoid tests that only mirror the implementation.
3. Build and pin coherent consumers. Use a short, verified SDK screen when useful.
   A microbenchmark or operation count alone does not establish a proof speedup.
4. When a candidate shows a useful gain, list its missing adoption requirements
   and finish those requirements on that candidate. Do not require a cosmetic
   cleanup or unrelated refactor before proceeding. If integration requires a
   changed build, track it separately and preserve the earlier candidate.
5. Freeze a suitably sized qualification protocol before its first preflight:
   actual consumers, inputs, comparisons, timing boundaries, worker counts,
   order, samples, uncertainty method, resource limits and decision criteria.
   Compare adoption performance against the accepted production baseline.
   State the role of additional controls explicitly; they do not automatically
   become adoption vetoes. If choosing between two candidates requires a
   head-to-head criterion, declare it prospectively.
6. Run the missing qualification, verify outputs and make an explicit adoption
   decision. Once the candidate meets its declared release requirements, apply
   the tested change, check that the production configuration matches, commit
   and push. Do not postpone a qualified improvement because it falls short of
   the broader research goal or because another cleanup might look nicer.

Retain qualified small improvements. There is no minimum percentage gain floor.
Report absolute milliseconds or microseconds saved, paired ratios, uncertainty,
CPU use and memory. A high-core gain does not automatically improve the best
worker-count latency; report both. Do not demand that a candidate beat Aeglos,
reach 2×, or meet the full 10× goal before adopting a smaller qualified gain.

## Handle a failed follow-up without losing its parent

A new variant's failure applies to that variant and campaign. Preserve the
parent's successful checks and its outstanding requirements. Decide separately
whether new evidence directly contradicts the parent's results, under matching
conditions, and record that evidence if it does. Do not silently transfer a
child's failure or success to its parent.

Use retained captures and receipts first. Rechecking existing data is distinct
from generating new proofs. A candidate measured as a control can supply useful
evidence, but a newly chosen comparison is retrospective analysis, not a newly
passed prospective gate. Do not silently add that comparison to the old protocol
or claim the control has completed full adoption qualification.

Keep failed protocols closed. Do not extend their samples, loosen thresholds,
drop inconvenient contexts, change comparators, or repeatedly rerun unchanged
screens until they pass. If required evidence remains inconclusive, state the
missing question and declare a separate, adequately sized follow-up before
collecting data. Reuse completed tests and captures; additional work must resolve
that question, rather than rebuild the entire campaign for bookkeeping.

Plan uncertainty and sample sizes with the number of required comparisons in
mind. Report marginal intervals as marginal; do not imply joint confidence.
Avoid an unnecessarily large set of underpowered vetoes. This guides future
protocol design and does not relax any completed experiment's declared rules.

## Preserve measurement and resource discipline

- Use the Rust xtask workflow and exact build receipts. Keep portable and native
  builds separate. Resolve actual Cargo libraries and proc-macro artifacts, and
  use the pinned toolchain for SDK linking. Never nest xtask under itself.
- During any SDK preflight or recording, only poll the confirmed live handle
  until authoritative completion. No owned build, helper, hashing, analysis,
  reporting or push may overlap it. A timeout or truncated output is not
  evidence of completion. Run isolated latency and profiling phases separately.
- Use automatic profiles when needed to answer the hypothesis. Validate observer
  overhead before treating instrumented behavior as representative. Sample shares,
  job counts, faults and context switches are not removable wall time or syscall
  counts. Reject causal claims the evidence cannot support.
- Independently verify fresh exported proofs, public-input bindings and sample
  accounting. Extend the current verified-export ledger; transport rechecks do
  not add distinct proofs. Preserve failures and resume completed work without
  replaying successful science.
- Preserve universal bounded-memory behavior. Avoid giant lookup tables, expanded
  key plans, new production pools and artificial architectural RAM caps. Record
  benchmark container limits separately from prover requirements. Peak RSS is
  not minimum RAM. Do not infer phone, ARM, pressure, huge-circuit or concurrent
  throughput results from unrelated qualification.
- On AWS, use only scoped owned benchmark resources, bounded lifetimes and
  adequate credentials. Retrieve and verify results, then promptly remove and
  check the owned VM, volume, temporary image tag and transfer objects. Do not
  act on historical resource IDs.

## Close an experiment and continue the goal

Publish a short factual report with exact candidate identities, results and
limitations. Preserve raw evidence, and use respectful comparisons with Gnark,
Aeglos and other projects. Label historical comparisons and supplied references.
Commit and push each completed experiment, verify the remote commit, and update
the candidate register and current handoff. Preserve historical report decisions.

The handoff must identify the production baseline, each pending candidate and its
remaining requirements, the current proof ledger, live handles/resources, actual
external blockers and the next concrete action. Close a pending promising
candidate's qualification or record why it is deferred before replacing it with
another variant or speculative optimization. Reporting is a checkpoint in the
research loop, not its endpoint.

Report a stale blocked goal status separately from an actual external blocker.
After the user resumes work, continue authorized independent work and reset the
blocker audit. Do not invent an authentication, approval or budget obstacle, or
mark the full goal achieved because an experiment passed. If the available goal
tools cannot reactivate its recorded state, say so; do not create a replacement
goal or claim that the metadata was repaired.

## Current example: G2 scalar grain

The [experimental build](benchmarks/2026-11-11-phi-grain-c7a176/README.md), SDK
`c88a3528…`, passed all twelve primary checks at 32/96 C7a workers, including
the 96-worker improvement. It was retained as PARENT in the later all-shape
run, and those control measurements were reviewed retrospectively. A separate
[31-block prospective qualification](benchmarks/2026-11-11-phi-grain-parent176/README.md)
then passed all forty adoption checks. The exact experimental runtime and a
separately validated native build selector are now adopted. No cleanup build
was substituted for the measured SDK.

The [default build](benchmarks/2026-11-11-phi-grain-clean176/README.md), SDK
`96915d56…`, passed correctness, exported-proof verification and all thirteen
RSS guards, but passed only 58 of its 66 performance/resource checks. Its
performance qualification failed and it is not adopted. This does not reject
the experimental build. Its failed protocol remains unchanged. The adoption
record preserves the distinction between retrospective control review, fresh
prospective qualification, default-selection evidence and production source
mapping. Future experiments start from the newly adopted baseline.
