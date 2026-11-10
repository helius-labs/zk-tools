# SDK task lifecycle observer: perturbation screen fails

The observer produces complete task traces and verified proofs, but **fails
the declared overhead screen** at 32 workers. Recording adds a paired median
5.37% versus OFF, with a 6.71% upper bound. Production is unchanged. These
instrumented observations are not qualified as a low-overhead explanation
of the C7a scaling gap, and no AWS capture or optimization is selected.

The diagnostic integrates the [qualified recorder](../2026-11-10-task-lifecycle176/README.md)
with the current accepted SDK. All 176 originals recover exactly after five
reversible Cargo/source edits. Field, solver, FFT and MSM algorithms retain
their source; coarse phase hooks and a direct private Rayon-core dependency
provide observation. The vendored recorder remains byte-exact to its qualified
revision. Libraries and SDK link coherently with accepted native flags,
including dense busy reset, O3, fat LTO and one codegen unit.

## Fixed local comparison

Ryzen 9950X, 16 physical cores/32 logical CPUs. Nine fixed rotated/reversed
blocks compare unchanged accepted REF and one diagnostic executable recording
OFF/ON. Each process primes three requests and times 31 complete fresh SDK
requests, including solving, blinding, proving, internal verification and
serialization. Loading, observer setup, draining and graph checks are outside
the SDK timer; those operations can still affect caches and frequency.

|Workers|REF ms|OFF ms|ON ms|OFF/REF [bounds]|ON/OFF [bounds]|ON/REF [bounds]|
|---:|---:|---:|---:|---|---|---|
|16|8.960540|9.021668|9.164301|1.004507 [0.971903,1.015645]|1.021235 [1.008250,1.031931]|1.020258 [0.999769,1.030432]|
|32|7.730773|7.781514|8.204030|0.999406 [0.983273,1.018426]|1.053650 [1.048563,1.067126]|1.058688 [1.037667,1.070508]|

Times are medians of nine process means; ratios pair corresponding blocks.
Bounds use zero-based sorted ranks 1 and 7, nominal 96.09375% marginal median
coverage under independent exchangeable blocks, without familywise guarantees.
OFF/REF intervals must fit [1/1.03,1.03]; both ON/control upper bounds must be
at most 1.05 in both contexts. Four of six checks pass. Both 32-worker overhead
checks fail; the overall qualification fails. The 16-worker results are retained
without subset promotion, threshold changes, exclusions or unchanged recapture.

|Workers|Mode|Whole-process peak RSS MiB|Whole-loop CPU ms/request|
|---:|---|---:|---:|
|16|REF|79.00|115.006|
|16|OFF|80.50|114.983|
|16|ON|163.75|123.111|
|32|REF|112.50|177.209|
|32|OFF|111.50|176.870|
|32|ON|276.50|215.217|

RSS is the maximum process high water across nine repetitions, including loading
and observer state. CPU is the median whole-loop user+system time divided by
requests; ON includes between-request buffer setup, graph checking and retained
graph construction. It is not SDK-only CPU cost or a minimum-RAM measurement.
Each thread reserves at most 16,384 64-byte rows before a recorded request;
output and temporary JSON storage add memory outside that payload forecast.
These allocations belong to the diagnostic, not the production prover.

## Trace coverage and next step

All 560 enabled requests pass in-driver participant, overflow, scope-nesting,
publication/start/end accounting and required phase checks. Solver, scalar
preparation, product FFT, queries, A/KH/G2 and internal verification appear on
the actual primary path, including its existing worker-entry hop. Fifty-six
retained graphs contain **4,988,226 events** and pass a separate raw reader.
Only those retained graphs receive independent event-by-event review; the
remaining requests rely on the in-driver checker.

The first enabled preflight graph contains 23,758 queued-job occurrences:
95,032 publication/start/end rows, 40,000 search rows and 18 phase rows.
This is event-volume accounting in an instrumented request, not an unprofiled
task census or causal phase attribution. It identifies a substantial recording
cost worth reducing. The current representation writes separate full records
for both ends of publications and scopes and carries metadata that the dispatch
reader does not use.

A distinct next observer can store interval endpoints in one reserved row and
retain the publication brackets, execution boundaries, overflow detection and
reused-key ambiguity checks. That requires fresh native qualification and a new
fixed overhead comparison. No such revision or speed improvement is measured
here. Further C7a interpretation waits for a qualified observer.

The dispatch reader accepts separate key generations only when their complete
publication/execution intervals do not overlap. It retains other occurrences
as ambiguous. Four synthetic tests cover ordinary intervals, a fast thief,
safe key reuse and overlapping keys. Job intervals include nested waits and
do not identify exclusive useful arithmetic, idle CPU time or memory stalls.
Boundary END/search events are retained after stop and require clipping.

## Validation and provenance

New integration tests pass 127 core, two prover and six cubic cases; generic
no-ASM/no-IFMA compilation passes. These do not establish ARM or phone execution.
Twelve preflight processes cover all nine circuits with OFF, accepted primary,
and enabled primary at 16/32 workers. The 54-process timing matrix follows.
All 66 final exports are 164 bytes, independently verified by pinned Gnark,
bound to expected image/public inputs and fresh against the prior ledger.
There are 1,884 SDK requests including primes and 1,686 timed samples.

Both readers, order, inputs, capacities, gates and independent executable copies
are frozen before any SDK preflight. The 135-step workflow succeeds once;
only its confirmed tool handle is polled during recording. All main captures
finish before their independent export verification and offline analysis.
No build, hash helper, archive or second benchmark overlaps SDK recording.

Preparation preserves two earlier helper failures: an absent export anchor and
attempted UTF-8 decoding of a binary source-manifest member. No native command
had run. An unsupported optional xtask plan field is converted before command
execution. After three successful native build/test commands, SDK link setup
cannot find a Cargo-reported root library path; the completed artifacts are
resolved through their dependency metadata and frozen. Only freeze/link/generic
steps then run. Successful native tests and builds are not replayed for these
repairs. Sources, partial workspaces, plans and successful receipts are retained.

The verified-export ledger advances **50,005 → 50,017 → 50,071**. Its current
private path is `task-lifecycle-sdk176/proof-sha256-after-capture.json`, SHA-256
`35caebea2ce8fdb41aa63938ac2b960b2879156b2bfa6b20bdd0e3e377983e3a`.
The full all-eight CPU 10× goal remains unmet. Fresh primary REF corresponds
to 4.786× the supplied 37 ms M5 CPU and 3.104× its 24 ms Metal reference;
these are historical different-hardware comparisons. Complete all-shape
[AWS CPU/Metal results](../2026-11-10-accepted-current-c7a176/README.md) and
[prior local controls](../2026-11-10-weighted-owned-roots-sdk176/README.md)
remain separate observations. No new AWS, huge-circuit, pressure/swap,
minimum-RAM or concurrent-throughput claim is made.

Evidence: [analysis](artifacts/analysis.json), [independent review](artifacts/review.json),
[event composition](artifacts/event-composition.json), [design](artifacts/design.md),
[source and receipts](artifacts/evidence.tar.gz), [retained traces and exports](artifacts/traces.tar.zst),
[publication checks](artifacts/publication.json), [checksums](SHA256SUMS).
