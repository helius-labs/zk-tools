# Aeglos MSM phase observer qualification

A bounded observer is implemented and locally qualified on the retained Aeglos
MSM source. All seven existing MSM test families pass against Gnark, with 227
passing test/subtest entries across scalar, AVX-512, IFMA256 and IFMA512 paths.
Both Sorted and Windowed are covered. The reader validates 1,014 emitted records.
**This is diagnostic preparation, not a performance result or an optimization.**
Current Zopro production and accepted timings are unchanged.

The [retained C7a stage decomposition](../2026-11-12-scaling-divergence180/README.md)
located Sorted's scaling gain in MSM. The next measurement needs to distinguish
its substage timings and work partitioning from the shared worker pool. This
observer provides the Aeglos side of that comparison; the matched full-SDK
experiment and current Zopro observer are not yet implemented or measured.

## What it records

The observer surrounds the existing serial phase runner: initialization, recode,
count, place, scatter, accumulate, each merge-plan/merge round, weighted sums and
finish. It preserves original phase order, arithmetic, job partitioning and pool
calls. It records each phase's elapsed interval and the already computed number
of scheduled items for each query. It never calls `steps()` a second time: that
method can allocate or mutate work.

There are no per-point, field-operation, task, worker or steal hooks. Scheduled
items are not equal-cost arithmetic operations or measured simultaneous readiness.
The merge-plan count represents the original pair count; it is not interchangeable
with the merge pass's scheduled-item count. The records contain no scalar, point,
witness, proof or pointer values.

Each call owns a fixed 10,808-byte recorder with at most 64 rows and 16 job counts
per row. Formatting uses a separate 32,768-byte caller-stack buffer and performs
one stdio write per record. These are diagnostic stack costs, not a production
workspace policy or a minimum RAM claim. No dynamic observer allocation or
retained key table is added. Overflow, malformed nesting/counts and incomplete
runs are explicit and must reject a performance trace. A failed proving operation
is not converted into success by the recorder.

OFF/ON share a private binary, selected by `AEGLOS_BENCH_MSM_PHASES=0/1`.
Observer setup, clocks, record construction and output remain inside the full
request's elapsed time. Recorded phase intervals exclude post-interval count
copying and final formatting/output; total/request residuals must remain visible.
Emission happens after the original phase sequence, before the MSM function
returns. Passing functional tests does not establish acceptable perturbation.

## Validation

Ten fake-clock/functional cases cover disabled operation, exact elapsed spans and
per-job counts, empty work, row overflow, job overflow, bad counts, incomplete
runs, unfinished spans, maximum-size encoding and concurrent independent buffers.
Sixteen emitted test records parse: eleven are valid and five carry the expected
invalid/incomplete flags. The disabled case makes no clock call and emits nothing.

The native archive builds with the inherited Makefile and g++. Existing native
MSM tests cover independent Gnark comparisons, thread counts, zero-length input,
shared scalars, limited memory, repeated plans and native wire views. Observed
worker counts are 1/4/32 and job counts are 1/2/3/9. None of the seven test families
is skipped. Actual records use at most eleven rows, have no overflow or malformed
flag, follow the expected phase sequence, and conserve per-job pass counts.
Intervals are nonoverlapping and contained in their enclosing run. Test durations
are **not** used as benchmark data. There is no 96-core qualification yet.

The private tree matches all 157 retained candidate source files before editing;
that reference already contains the historical Sorted/Windowed Go switch. One
additional header implements the observer. Eight reversible edits to
`cpu/msm_impl.hpp` recover its exact original SHA-256
`6a57cf5dd2312a1bcbadefa6b409c600dc33dd38cc138d11e3849b23d99aac85`.
The retained reference is Aeglos commit
`0290613e8eeca17a4ce88f6f2d726e8e06a5385d`; it is not claimed to be latest upstream.

One initial unit compile stopped on misleading indentation in the test and a
nonliteral no-argument format call. Both are fixed before the first successful
unit run or native build; the pre-fix sources are retained. The final compile
uses `-Wall -Wextra -Werror`. No successful native test is repeated. Six xtask
commands pass across four workflows: unit run, unit-record review, native build,
Go test compilation, native MSM tests and native-record review. Native build takes
21.27 seconds, Go test compilation 6.16 seconds, and MSM tests 45.58 seconds.
Preparation/compiler invocations outside those workflows remain separate.

## Remaining experiment work

Before interpreting performance, integrate this observer with the full-request
harness and establish exact key/request/engine identities and fresh external
proof verification. A fresh unmodified-source build with the same harness and
the retained reference executable must remain distinguishable from diagnostic
OFF. Calibration against the retained binary is untested.

Add only the missing coarse Zopro boundaries, preserving its existing overlapping
query execution. Compare input readiness, preparation and all-query completion;
do not add overlapping query spans as exclusive CPU time. Freeze a bounded
32/96-worker C7a matrix with disabled/enabled observer controls before capture.
Report both the overhead and any phase redistribution. The older per-task trace's
failed overhead check remains a failure, and no joint-window queue retry follows
from this preparation. No new optimization is selected.

There are zero new SDK calls, exported proofs or AWS resources. The canonical
verified-export ledger remains 55,209. The all-eight CPU10x objective remains
unmet (0/8). No GitHub publication is authorized.

Private root: `/home/ubuntu/src/zopro-perf-work/msm-stage180`.
[Observer checks](artifacts/observer-review.json), [native checks](artifacts/native-review.json),
[observer source](artifacts/bench_msm_phases.hpp), [source edits](artifacts/source-edits.json),
[plans, pinned inputs and receipts](artifacts/qualification-records.tar.gz).
Native libraries, executables and the complete third-party tree stay private.
