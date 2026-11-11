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
   locally. Push only with user authorization; publication is currently withheld.
   Do not postpone a qualified improvement because it falls short of
   the broader research goal or because another cleanup might look nicer.

Retain qualified small improvements. There is no minimum percentage gain floor.
Report absolute milliseconds or microseconds saved, paired ratios, uncertainty,
CPU use and memory. A high-core gain does not automatically improve the best
worker-count latency; report both. Do not demand that a candidate beat Aeglos,
reach 2×, or meet the full 10× goal before adopting a smaller qualified gain.

## Make rejection cheap and reserve qualification for promising changes

Do not start every candidate with the full all-shape performance campaign.
Correctness qualification, exploratory screening and release qualification answer
different questions. Use the smallest measurement that can answer the current
question; keep all eight shapes in the final goal and release requirements.

1. **Check the mechanism first.** State which work disappears, which cost is
   added, and what observation would refute the hypothesis. Inspect generated
   code when relevant. Measure the actual caller with representative inputs,
   including packing, scratch and reduction costs. A faster isolated field
   primitive is insufficient. For a scheduling change, inspect worker activity
   and the critical path; aggregate CPU samples are not a critical-path estimate.
2. **Run a short full-proof screen.** Freeze the exact consumers, a small fixed
   sample count, relevant worker counts and representative cases before capture.
   On this host retain all three predeclared executable backings and balance
   ordering. Include a memory-sensitive case when the change can affect resources.
   Verify exported proofs. This screen is for rejecting an expensive idea or
   choosing a follow-up question, not declaring adoption or a precise small gain.
   Preserve inconclusive results as inconclusive. Do not extend a screen until
   a favorable answer appears.
3. **Profile the surviving mechanism.** Collect counters or samples that can
   distinguish the proposed explanation from added work elsewhere, with observer
   checks. Reuse valid prior tests and retained captures. If the mechanism is
   refuted and whole-proof benefit is absent, close the candidate without a
   full all-shape timing run. A profile needed to test the original mechanism
   belongs in the first stage, not after expensive qualification.
4. **Qualify a promising exact build.** Run the missing all-shape correctness,
   performance, portability and resource checks against accepted production.
   Use a separately frozen, adequately powered protocol. Finish qualification
   before adoption; report CPU and Metal comparisons for all eight supplied
   shapes and merge 54×1 separately. An exploratory success does not relax these
   requirements.

Record preparation, capture, analysis and total elapsed time, plus the decision
each stage enabled. Prefer reusable xtask commands and existing readers over a
new campaign framework. A source change that increases instructions or stack
traffic may still help scheduling or instruction delivery, but it needs a short
test of that benefit before hundreds of full-proof processes.

## Prioritize changes that remove work

Before choosing the next experiment, make a compact cost model from the current
production implementation and profiles. Count field operations, point operations,
passes over memory and synchronization where feasible. Separate measured work
from hypotheses. Estimate how much complete-proof time the proposed change could
plausibly remove, and account for its added work and memory. Amdahl-style estimates
are upper bounds only when their critical-path assumptions hold.

Use this model to prioritize algorithmic changes: fewer scalar/point operations,
sharing work already repeated within a proof, less expensive representations or
fewer passes and synchronization boundaries. Check previous experiments and
primary papers/code before repeating an idea. Preserve Groth16 semantics,
fresh blinding, portable fallback and bounded memory; a genuinely different proof
system or protocol is a separate compatibility decision, not a drop-in speedup.

After a mechanism-directed variant fails, require new evidence before trying
another annotation, threshold or helper boundary in the same family. Document
why the next hypothesis differs and what a short experiment will resolve. This
is research prioritization, not a minimum gain floor: retain and finish adopting
small improvements that already have credible evidence. For an AWS discrepancy,
a local result screens an idea but cannot establish that it fixes C7a behavior.

## Explore beyond the current implementation

Maintain two tracks: finish candidates with credible gains, and test structural
alternatives. For the next three new exploratory decisions, devote at least two
to a different algorithm, representation or execution dependency, rather than
another parameter or helper variant. This is a research allocation rule, not an
adoption threshold; existing promising candidates still receive qualification.

Before building, write five short items in the existing experiment record:
the assumption being challenged, the predicted observable change, the cheapest
test that distinguishes explanations, the added memory/setup cost, and the
decision each possible outcome enables. Give the first screen a preparation and
measurement time allowance. If it overruns, simplify the question or explicitly
justify continuing. Elapsed time is not evidence that an idea failed.

After two unsuccessful variants in one mechanism family, choose a different
family unless new evidence changes the hypothesis. Consult retained negative
results first. A changed constant, language or function boundary alone is not a
new hypothesis. Do not require a speculative prototype to look like production
code before screening it. Keep it isolated, preserve output checks, and charge
its setup, conversion, merge and allocation costs at the actual caller boundary.

For the C7a gap, separate excess arithmetic work from poor parallel execution.
Use matched input and timing boundaries, worker-count curves, CPU time and
latency for both implementations. Select a targeted intervention that can
distinguish the remaining explanations. Local experiments can reject ideas;
they cannot establish a C7a fix. Lower operation counts are neither necessary
nor sufficient for lower latency, and a scheduler sample is not automatically
avoidable overhead. Reuse trustworthy captures before commissioning new ones.

Measure iteration by time to a resolved question and verified proof time saved,
not reports, tests or candidates produced. Reuse the Rust/xtask harness and
inherited qualification. Keep rejected screens to a short decision plus raw
evidence; reserve full campaigns and integration work for credible survivors.
At each three-experiment review, state what assumption changed. If none changed
and no candidate advanced, change the question before generating more variants.

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
- For small-change comparisons on this Ryzen host, distribute each consumer
  across a fixed, predeclared set of independently written, byte-identical
  executable files. Record complete hashes and device/inode identities; retain
  every copy and outcome. Pair or balance backing replicas in the analysis.
  Do not select the fastest copy or use copying to reopen a failed gate. The
  [full-SDK backing diagnostic](benchmarks/2026-11-11-gather-backing177/README.md)
  reproduced a large file-associated latency difference despite identical bytes.
  It does not establish a hardware mechanism or guarantee that any particular
  number of copies removes the effect. Diagnose large backing interactions
  before attributing their size to source changes.
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
Commit each completed experiment locally and update the candidate register and
current handoff. Push only when the user authorizes publication, then verify the
remote commit. Current authorization does not permit pushing. Preserve historical
report decisions.

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
