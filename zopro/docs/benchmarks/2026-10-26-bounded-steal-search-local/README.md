# Bounded simultaneous victim scans: local stage

The fixed local speed screen failed. Primary is 8.574 ms ON, 8.580 ms OFF
and 8.652 ms accepted, but the paired ON/OFF median is 1.001037 with only
three of seven wins. Merge-54 exceeds the 2% regression gate against accepted.
No production source is adopted. The separately declared C7a target stage
remains pending.

## Change

The private Rayon-core 1.13.0 prototype allows at most `ceil(workers/2)`
simultaneous victim sweeps per registry. A cache-padded counter and a scoped
permit cover one finite sweep, with the original victim order and queue
operations. The permit drops before job execution, injected-job checks,
parking or a retry sweep. Deferred attempts still check local/broadcast and
injected work, yield, and recheck the latch. They do not count as completed
empty searches toward the existing parking threshold.

`ZOPRO_CPU_BOUNDED_STEAL_SEARCH=1` is read once per registry. Omission and zero
select OFF. Cryptographic source, queues, fences, epoch reclamation, latches,
condition variables and worker counts retain their implementation. The policy
adds bounded registry metadata and stack state, with no new per-proof heap
buffer, retained key table, copied points, circuit plan or pool. Scheduling
can still change allocation lifetimes and fairness, so full resource and
concurrency qualification remains necessary before any adoption.

The motivation is [Tokio's scheduler design](https://tokio.rs/blog/2019-10-scheduler#throttle-stealing)
and the current C7a profile: epoch pin/advance accounts for about 48.5% of
sampled user cycles at 96 workers, versus about 9% locally. Tokio limits
searching worker roles; this prototype limits a finite sweep and preserves
Rayon's parking checks. No Tokio scheduling code is copied. See the
[dependency review](artifacts/dependency-review.md).

## Complete SDK results

Ryzen 9950X, seven rotated/reversed fresh-process blocks, 31 measured requests
after three primes per process. The 32-worker cases use CPUs 0–31; 16 workers
use CPUs 0–15, one logical CPU per physical core. Loading is excluded. Timed
work includes fresh solving, randomness, proving, internal verification and
serialization. All latency finishes before the separate profiles.

| Case | Accepted ms | OFF ms | ON ms | Paired ON/accepted | Paired ON/OFF |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1x2 / 32 | 8.651936 | 8.580181 | 8.573519 | 0.990936 | 1.001037 |
| 2x2 / 32 | 11.746021 | 11.727406 | 11.660761 | 0.992741 | 0.995116 |
| merge-54x1 / 32 | 176.465048 | 179.715708 | 179.972472 | 1.022119 | 1.000957 |
| 1x2 / 16 | 11.107698 | 11.323835 | 11.228726 | 1.006878 | 0.980468 |

Cells are medians of process medians. Ratios pair corresponding blocks, so
ratios of pooled medians differ. Primary wins 7/7 against accepted but 3/7
against the same binary switched OFF. This does not isolate a speedup from
admission. The 16-worker OFF calibration factor is 1.05448 and fails the
fixed 1.03 bound; its variable observations remain in the result.

The original screen requires positive primary and four-case geometric
results against both controls, at least four primary wins, at most 2% paired
median and 5% per-block regression, and OFF calibration within 1.03. There
is no minimum gain floor or unchanged local rerun. Four-case paired
geometric ratios are 1.002926 against accepted and 0.994200 against OFF.
Every failed check is retained in [the result](artifacts/pilot-result.json).

| Primary / 32, unprofiled | Accepted | OFF | ON |
| --- | ---: | ---: | ---: |
| CPU ms per request | 199.172 | 196.961 | 201.009 |
| Voluntary switches per request | 417.81 | 423.35 | 359.45 |
| Maximum process RSS, MiB | 113.25 | 113.00 | 116.00 |

RSS includes loading and priming and establishes neither minimum RAM nor
pressure feasibility. No additional owned build, profiler, CPU benchmark or
bulk audit overlaps latency.

Nine separate 999 Hz captures contain 243,749 samples, one process per
capture and no lost samples. Primary epoch share falls from 9.38% OFF to
6.14% ON, while scheduler share rises from 4.35% to 8.14%. Profiled CPU time
rises from 199.50 to 202.35 ms per request. Lower epoch share is offset by
other work locally. These are sampled CPU shares, not removable wall time;
inlining limits attribution. Profiles use an explicit monotonic clock.

## Validation and provenance

The SDK lock retains all 219 package entries except Rayon-core's private
path source/checksum. The vendored source and upstream MIT/Apache licenses
are pinned to the original package archive. Upstream tests use the exact
SDK Crossbeam deque 0.8.8, epoch 0.9.21 and utils 0.8.23 package entries.
Loom and its dependencies belong to a separate test-only workspace.

Both OFF and ON pass 104 Rayon library and 11 integration tests, with the
two upstream ignored tests retained. Added coverage includes held-permit
injection/broadcast progress, complete credit return, concurrent nested
callers, FIFO, panic, pool reuse and cross-pool calls. Two integration tests
also pass with up to 32 workers restricted to one physical core. Two Loom
tests verify actual instrumented atomics and admission/release interleavings
at a two-preemption bound. This models the admission primitive, not the
whole Rayon scheduler or unbounded fairness. Native proving passes 109 core,
two raw-image and six cubic tests. Field arithmetic is unchanged; no new
native field-suite execution is claimed.

Preflights cover all nine real SDK shapes at four workers and primary,
2x2 and merge-54 at 32 workers, both OFF and ON. They complete 96 requests
and 24 independent Gnark exports. Timing adds 2,856 requests and 84 exports;
profiles add 660 requests and nine exports. Total: **3,612 SDK requests,
117 distinct independently verified exports and 234 proof/public files**.
Fixture bindings and freshness pass before timing. The inventory validator
runs in the preflight workflow and again as a mandatory perf check. Source,
linked library and executable hashes are checked around evaluation.

The initial ON unit suite passed 103 tests and failed one test that assumed
a credit would be immediately available after release. Idle workers could
acquire it first. The corrected test reclaims the entire budget within a
five-second deadline before and after forced deferral. Only `cfg(test)` code
changes; runtime scheduling code is identical. Original source, failure logs
and the failed test executable's hash are retained. Final suites run from a
new source snapshot and workflow, preserving the first result. No SDK timing
preceded the correction. A read-only full-workspace metadata query also failed
offline on an uncached optional Vulkan dependency; actual CPU builds passed
locked and offline without changing that dependency.

## Target stage and limits

C7a measurement was explicitly declared before compilation regardless of the
local speed sign because measured epoch overhead differs substantially at
96 physical cores. This local failure remains a failure. It supports no
local/default adoption or server speed claim. Next: build this exact source
for `znver4`, test the target executable, then run the frozen 32/64/96-worker
C7a comparison with fresh Aeglos timing and independent verification.
The Aeglos branch head was checked on October 26 and still matches the
pinned `0290613` Sorted build. No new AWS resource was created for this local
stage. Full-nine latency, portable, pressure, concurrent SDK and large-circuit
qualification remain necessary if target results are positive.

ON primary is 4.32 times the supplied 37 ms M5 CPU reference and 2.80 times
the 24 ms Metal reference; 2x2 is 5.06/2.57 times its 59/30 ms references.
These are supplied historical references, not matched-machine measurements.
No merge-54 M5 reference was supplied. The original all-eight 10x CPU goal
remains unmet; accepted production still uses the
[qualified manifest](artifacts/accepted-manifest.json).

[Protocol](artifacts/experiment.json), [local conclusion](artifacts/local-stage.json),
[observations](artifacts/perf/rows.json), [profiles](artifacts/perf/report.md),
[source change](artifacts/source.patch),
[admission module](artifacts/overlay/vendor/rayon-core/src/search.rs),
[final build](artifacts/native-v2/result.json),
[first test failure](artifacts/failure-v1/review.json),
[proof inventory](artifacts/perf/proof-inventory.json).

Full local experiment: `/tmp/zopro-bounded-steal-search/`.
Raw captures and expanded symbol streams are retained locally and omitted
from Git. The hash index covers those archival copies; executables and
compiler caches are represented by their pinned build/link receipts.
