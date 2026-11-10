# Compact task recorder passes local perturbation checks

The interval recorder passes all six declared local calibration/overhead checks.
At 32 workers, recording adds a paired median **3.18%** versus OFF, with a
**4.24% upper bound**; at 16 workers the estimate is **1.04%**. This qualifies
the observer for the next C7a diagnostic stage. It is not a prover speedup or
an AWS result. Production remains unchanged and the full CPU 10× goal is unmet.

The [previous observer](../2026-11-10-task-lifecycle-sdk176/README.md) failed its
32-worker overhead screen. This revision reserves one bounded thread-local row
at an operation's start and fills its end timestamp in place. Each 56-byte row
retains both publication or execution endpoints, job/function keys and queue
ownership. Unused execution-parent pointer metadata is removed. Queue operations,
fences, inline joins, FIFO layers, clocks and observed operations are preserved.
Buffer preparation explicitly touches pages before recording.

## Fixed SDK comparison

Ryzen 9950X, 16 physical/32 logical CPUs. Nine rotated/reversed blocks per
worker count, accepted REF and one diagnostic executable recording OFF/ON,
31 timed requests after three excluded primes. Timers include fresh solving,
randomness, proving, internal verification and serialization. Observer setup,
draining and checking are outside the SDK timer but can still affect caches.

|Workers|REF ms|OFF ms|ON ms|OFF/REF [bounds]|ON/OFF [bounds]|ON/REF [bounds]|
|---:|---:|---:|---:|---|---|---|
|16|8.969657|8.949371|9.065228|0.999548 [0.992467,1.013993]|1.010382 [0.997218,1.018684]|1.012604 [0.996315,1.020937]|
|32|7.739810|7.708036|7.965508|0.997738 [0.988235,1.020081]|1.031824 [1.016154,1.042378]|1.029062 [1.025561,1.036559]|

Times are medians of process arithmetic means of integer SDK nanoseconds.
Paired median bounds use sorted zero-based ranks 1 and 7 of nine blocks,
nominal 96.09375% marginal coverage under independent exchangeable blocks,
without familywise guarantees. OFF/REF must fit [1/1.03,1.03]; both ON/control
upper bounds must be at most 1.05 in both contexts. All pass without exclusions,
extensions, threshold changes or subset promotion. These overall bounds do not
establish that each internal phase has the same perturbation bound.

|Workers|Mode|Whole-process peak RSS MiB|Whole-loop CPU ms/request|
|---:|---|---:|---:|
|16|REF|78.75|115.323|
|16|OFF|79.50|115.088|
|16|ON|120.50|118.877|
|32|REF|114.25|177.632|
|32|OFF|108.50|177.368|
|32|ON|191.75|193.536|

RSS is the maximum whole-process high water across nine repetitions. CPU is
the median summed whole-loop user+system time per request and includes observer
work between requests. Neither isolates SDK-only CPU work or minimum RAM.
The frozen 8,192-row capacity forecasts 14.4375 MiB of payload at 32 workers;
metadata, retained graphs and output add memory. This is diagnostic storage,
not a production prover RAM limit. Comparisons with the earlier observer are
across campaigns, not a factorial isolation of the representation change.

## Local task lifecycle observations

All 560 enabled requests validate in the driver; 56 retained graphs receive
independent event-by-event checks. A separate descriptive aggregation uses all
54 retained latency graphs, leaving two preflight graphs as functional evidence.
Every queued-job occurrence in those 54 graphs has an unambiguous matched key
generation. The reader retains ambiguity when present; it does not assume
pointer/function keys are globally unique.

|Phase|16-worker median wall ms / published jobs|32-worker median wall ms / published jobs|
|---|---:|---:|
|Solver|0.988 / 183|1.096 / 183|
|Scalar preparation|0.140 / 2,839|0.244 / 10,988|
|Product FFT|0.512 / 3,979|0.617 / 8,125|
|MSM queries|6.639 / 1,101|5.137 / 1,299|
|Internal verification|0.403 / 3|0.437 / 3|

These are medians over 27 instrumented graphs per worker count. Independent
medians need not sum. They expose a scalar-preparation granularity lead: twice
the workers produce roughly four times as many queued jobs and longer elapsed
time in that phase. MSM queries remain the largest measured phase. Neither
observation establishes the cause of the C7a gap.

The [phase summary](artifacts/phase-summary.json) also retains publication-to-start
intervals, time with known published but not started jobs, and worker residence
inside victim-search invocations. Ready-work counts include claimed jobs that
have not crossed the execution-start hook. Job spans can include nested waits;
search wall time is not CPU idle time. No pending queued jobs does not imply no
arithmetic work executing inline. The next discriminating measurement is the
same current-source trace at 32/96 physical C7a workers, with target-side
observer controls before causal interpretation.

## Validation and provenance

The interval component passes 119 tests, including nine recorder tests on real
1/4/16/32-worker pools, plus disabled-CFG compilation; two upstream tests remain
ignored. SDK integration passes 127 core, two prover and six cubic tests, and
generic no-ASM/no-IFMA compilation. Seven independent parser models cover
dispatch bounds, fast thieves, reused/overlapping keys and interval expansion.
These are not ARM or phone execution claims.

Twelve preflight processes cover all nine circuits OFF, accepted primary, and
enabled primary at 16/32 workers. The fixed 54-process timing matrix follows.
All **66 new 164-byte exports** independently pass pinned Gnark verification,
image/public binding and freshness checks. There are 1,884 SDK calls including
primes and 1,686 timed samples. Both readers, capacities, inputs, order and
independent executable copies are frozen before any SDK preflight. All 135
capture commands complete once; only the confirmed handle is polled during
recording. No build/helper/hash/second benchmark overlaps SDK recording.

The build supervisor disappears after successful test stages. Cargo's retained
library output ends with `build-finished: success=true`, and stderr reports
completion, but that command's process-exit receipt is missing. This gap is
preserved explicitly; no exit code is invented. The existing artifacts are
frozen and linked, then generic compilation finishes under a new supervisor.
Successful tests and the completed library build are not rerun for bookkeeping.
The missing receipt limits command-level provenance; subsequent SDK proofs and
all source/library hashes are independently checked.

All 176 accepted originals recover exactly after the five diagnostic Cargo/
phase-hook edits; the new qualified recorder is reused byte-exact. The ledger
advances **50,071 → 50,083 → 50,137**. Current private ledger:
`task-interval-sdk176/proof-sha256-after-capture.json`, SHA-256
`0202a88b8b682abfe8b9282aff086522bae23147b6dcaf21cc2b0227cd5d6469`.
Fresh primary REF is 4.780× supplied M5 CPU and 3.101× Metal; the complete
[AWS CPU/Metal table](../2026-11-10-accepted-current-c7a176/README.md) and
[prior local all-shape controls](../2026-11-10-weighted-owned-roots-sdk176/README.md)
remain separate observations. No new AWS, huge-circuit, pressure/swap,
minimum-RAM or concurrent-throughput result is claimed.

Evidence: [analysis](artifacts/analysis.json), [independent review](artifacts/review.json),
[phase summary](artifacts/phase-summary.json), [resources](artifacts/resources.json),
[design](artifacts/design.md), [source and receipts](artifacts/evidence.tar.gz),
[retained traces and exports](artifacts/traces.tar.zst), [publication](artifacts/publication.json),
[checksums](SHA256SUMS).
