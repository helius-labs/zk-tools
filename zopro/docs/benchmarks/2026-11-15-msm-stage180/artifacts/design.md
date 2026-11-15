# Bounded Aeglos MSM phase observer

Implement a private diagnostic on the retained Aeglos0290613 Sorted/Windowed
source. This is instrumentation of the comparison engine, not a C++ rewrite of
Zopro, a new prover/pool or an optimization. Current production stays unchanged.
First step toward the selected matched current-Zopro/Aeglos C7a MSM comparison.

Instrument original serial MSM phase boundaries only: begin/setup, recode, count,
place, scatter, accumulate, each original merge-plan/merge round, sums and finish.
Record elapsed monotonic time and counts already computed for each pass. Do not
call steps() a second time (it can allocate or mutate work). No per-point, field,
worker, steal or task hooks. No phase order, grain, arithmetic, key or pool change.
Observer keeps at most64 records and16 job-count entries per record on its caller
stack, no dynamic allocation. Explicit overflow/incomplete flags make an invalid
trace rejectable; never silently drop records or override a prover error. No key,
scalar, point, witness, pointer or proof values are logged. Each record describes
scheduled items, not equal-size arithmetic operations or ready concurrency.

OFF/ON use the same private binary, envAEGLOS_BENCH_MSM_PHASES0/1. One bounded
line is emitted after the phase run. Output and all observer setup/clocks remain
inside full-request timing and must pass the later overhead comparison. Reported
phase intervals exclude row-copy/format/output costs; total/request residual must
remain visible. A fresh original-source build with the same harness is also
required before claiming calibration to the retained Aeglos reference executable.
Do not rerun historical profiles or inherited proof verification.

Before full proofs: fake-clock tests for exact spans, disabled observer, overflow,
job bounds, incomplete runs, zero-work phases and concurrent independent buffers;
compile the native archive, and run existing native arithmetic tests when available.
Capture and validate emitted JSON from test processes. No performance claim from
unit tests. Full-SDK integration, external proof verification, current-Zopro phase
observer and frozen32/96AWSmatrix remain subsequent requirements. Keep all source
edits reversible and hash the retained originals. No push without approval.
