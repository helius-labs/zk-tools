# Recursive signed scalar-window reuse

The fixed recursive construction removes **4.33% / 4.51%** of signed G2
digit arrivals beyond exact whole-scalar classes on the retained primary/raw2x2
inputs, after charging every new point formation. The earlier disjoint
construction removes2.08%/2.11% on those same classes. Recursive reuse is a
larger modeled opportunity; **no native implementation or proof speedup is
established**. Fused primary G1K/H adds only1.17% beyond exact classes.
Production remains the adopted paired-weighted233 implementation.

## Construction and scope

[SRE](https://arxiv.org/html/2610.27452v1), sections4-5, motivates looking for
scalar-window matches. We use a fixed signed construction that allows residual
parents and newly formed sums to participate again. Lexicographic columnpairs
are normalized by simultaneous sign, sorted by pairkey/nodeid and reduced
left-to-right within each class. A formation P_i+sign*P_j receives every
matching nonzero column, removed from its parents. Passes stop when none forms.
No order, round, window, threshold or result-based rescue is selected.

This differs from the [one-generation disjoint-point screen](../2026-11-08-global-window-pair-bound176/README.md).
Its old results and restricted upper-bound claims remain unchanged. No new
general optimum or recursive upper bound is claimed. Both constructions use
identical G2 inputs/recoding/counts. This is not a reproduction of the paper
and transfers no reported runtime improvement from it.

Each k-column merge reduces live arrivals byk and charges one point addition,
savingk-1. Sincek>=2, at mostinitialarrivals/2nodes can be created. The model
uses that data-derived termination/storage bound, not an architectural RAM cap.

|Retained input|Raw nonzero arrivals|After exact classes|New formations|Final arrivals|Additional net saved|Fraction of class arrivals|Maximum DAG depth|
|---|---:|---:|---:|---:|---:|---:|---:|
|1x2-B|613891|574356|24692|524810|24854|4.327%|5|
|2x2-B|1064904|633695|28339|576761|28595|4.512%|4|
|1x2-KH|1170733|1134318|13270|1107742|13306|1.173%|2|

Raw rows are30089/52274 forG2 and61451 forKH (30529K plus the exact30922
usedH prefix, not a padded32768-domain extraction). Zero/identity terms are
omitted; exact scalar equality is grouped first. G2 retains22 signed GLV
width12 positions;KH uses the original20 balanced12/13-bit windows.

Including hypothetical exact-class grouping, total raw-arrival savings after
formation costs are10.11%/41.24% forG2 and4.04% forKH. Raw2x2 already admits
runtime grouping; much of41.24% is therefore old work, not a new gain. Primary
runtime does not execute this optimistic whole-scalar stage. Arrivals, bucket
first assignments, affine/projective joins and CPU time are different quantities.
These counts do not justify claiming a comparable proof-time reduction.

Each input finishes after one productive columnpair sweep and a no-change
sweep. New nodes are used within the productive sweep; observed depths2-5
confirm that the construction exercises recursive sums. This neither maximizes
reuse nor proves another order or representation cannot do better.

## Resource and cost limits

|Input|Signature rows sorted|Hypothetical new point payload|Node header capacity|Node-code capacity|
|---|---:|---:|---:|---:|
|1x2-B|9259502|3160576B|3346176B|2236696B|
|2x2-B|10090940|3627392B|3693056B|2516404B|
|1x2-KH|20109529|849280B|7261440B|2800000B|

The diagnostic allocates no curve points. Point-payload figures assume64B/G1
or128B/G2 per formation; they omit point initialization, whole-class formation
state, original inputs, routing codes, scratch, inversions, normalization,
weighted finish, exceptions, allocator and stack costs. Actual diagnostic
header/code vectors also exclude many live arrays, including formal reviewers.
None is a native workspace/RSS/minimum-RAM/pressure bound. No production table,
key copy, expanded key, pool, memory cap or runtime allocation is introduced.

The three-case producer takes about1.18s total on this host, including hashing,
recoding, optimization, formal reverse checks and DAG export. That is neither
isolated search time nor a prospective native proof benchmark. Millions of
signature rows and new point routing remain costs a native implementation
would need to address. A count reduction alone is insufficient to select it
for adoption. This fixed opportunity screen does not select a native prototype;
any future construction requires a distinct cost/resource case and complete
caller screening. It must retain these results rather than reroll them.

## Validation and provenance

Two new tests pass unfiltered: all19683 three-point/three-column matrices over
{-1,0,1}, plus explicit deep recursion, residual reuse, signs and empty cases.
A forward formal-basis oracle checks every coefficient. The producer separately
propagates all output coefficients backward through the signed DAG.

The independent reader expands every live output down to original basis leaves,
checking2,344,468 original row/column coefficients exactly. Reverse local-state
checks also verify every one of66,301 merge edges, that all matching columns
move and that support/formations/counts reconcile. It does not replay optimizer
choice order. KH digit recoding is independently bit-based; GLV recoding and
its source-qualified reconstruction checks are shared inherited components.
This is a separate same-agent check, not an external audit or curve proof.

All188 production hashes and the62187-proof inventory remain unchanged. No
native field/core/prover suite, curve oracle, SDK capture, proof, profiler or
cloud resource is added. Existing cache/oracle provenance is inherited without
re-extraction. Private DAG/scalar bytes stay local and are hash-bound.

One helper compile fails because inherited argv omits sha2; the successful
engine-test compile is inherited. The continuation adds that extern to remaining
links and adjusts only pin/receipt paths, retaining pre-repair sources. The first
continuation launch then fails strict schema validation on an unsupported
top-level metadata field, before creating a run directory or executing commands.
A transport-only plan removes it; all scientific argv/timeouts/inputs and code
remain. Seven workflow commands execute: six pass, one failed original compile.
Tests, count and independent review succeed first execution. No successful
configuration is replayed. Initial publication then incorrectly requested a success-result file from the failed original build workflow; archiving stopped. Its partial archive remains private, and publication continues using the actual failure.json without changing scientific results. [Publication note](artifacts/publication-note.md). [Repair note](artifacts/repair-note.md),
[transport note](artifacts/transport-note.md), [receipts](artifacts/receipts.json).

Protocol preparation 266.74s, count command 1.18s and independent review 0.14s;
protocol-to-review 268.06s. Both allowances pass. Earlier research/history is
separate; these file/xtask spans are descriptive, not native prover timings.

## Unchanged complete-proof comparison

The adopted implementation's all-eight CPU/Metal results remain
[234](../2026-11-18-paired-weighted-all234/README.md), with native selector
provenance in[236](../2026-11-18-paired-weighted-adoption236/README.md).
Primary32 measured7.460937ms and128.75MiB peakRSS. These are inherited, not
measurements from the new count model. All-eight CPU10x remains active/unmet0/8.
Merge54 remains163.135118ms without a supplied M5 reference. No freshAWS/Aeglos,
mobile/ARM, pressure/swap, huge-circuit or concurrent-throughput claim follows.

[Protocol](artifacts/protocol.md), [counts](artifacts/analysis.json),
[independent review](artifacts/review.json), [prior comparison](artifacts/prior-comparison.json),
[timing](artifacts/timing.json), [sources/plans/receipts](artifacts/source-and-plans.tar.gz),
[checksums](SHA256SUMS). Private inputs, DAGs, full proof inventories and
executables are excluded. Evidence is committed locally; publication awaits
approval. Local root:`/home/ubuntu/src/zopro-perf-work/recursive-reuse237`.
