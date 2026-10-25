# Local CPU accounting by request phase

This private diagnostic records process CPU time around sequential request
phases. It changes neither arithmetic nor scheduling policy. The accepted
production implementation and its 8.594267 ms local primary qualification
remain unchanged.

The fixed Linux recorder holds 32 stamps and permits one active diagnostic
request. Each stamp brackets `getrusage(RUSAGE_SELF)` with `CLOCK_MONOTONIC`
and records the calling TID. Accounting includes all process threads, so it
follows the request when execution moves onto a Rayon worker. Request phases
are nonoverlapping; overlapping query callbacks are not summed as CPU costs.

Removing the diagnostic insertions recovers three production call-site files
byte for byte. The native recorder passes ownership, disabled-operation,
worker-handoff, monotonic-accounting, overflow, unwind and reuse checks. Full
native validation passes 110 core tests, two raw-prover tests and six cubic
tests. No production file receives these hooks.

## Fixed comparison

AMD Ryzen 9 9950X, 16 physical cores / 32 logical CPUs. Three blocks compare
accepted, diagnostic OFF and ON for primary at 16 pinned physical cores and
32 logical CPUs, plus merge-54 at 32 workers. Latency processes finish before
27 warm profiles. Including preflight, 2,456 SDK requests produce 56 distinct
exports independently verified by Gnark.

| Instrumented profile phase | CPU ms/proof, 16 workers | CPU ms/proof, 32 workers |
| --- | ---: | ---: |
| Solver | 4.872 | 8.861 |
| FFT product | 8.003 | 19.233 |
| MSM queries | 113.115 | 163.568 |

Values are medians of process means. They describe CPU work, including work
search; they are not additive critical-path costs. The 16-worker profile has
substantial block-to-block wall-time variation. The supplied M5 references
belong to accepted full-SDK qualifications, not these instrumented spans.

Primary produces the expected 16 fine stamps. Merge-54 uses another solving
path and produces nine coarse stamps. The original all-case fine-coverage
check failed; its solver/FFT/MSM split remains unavailable. The original
failure and complete observations are retained.

## Clock validation

The initial `perf` clock differs from `CLOCK_MONOTONIC` by about 28.78 seconds.
All 205,901 ON samples therefore fall outside the recorded request intervals.
The process CPU deltas remain valid, but these samples support no phase
attribution. The first reader also lacked an attribution-coverage check;
its complete status does not validate clock alignment. No inferred constant
offset is applied to the captures.

The Rust task now has an explicit `monotonic_clock` option, passing
`--clockid mono` to `perf record`. A separate clock qualification completes
232 SDK requests and eight independently verified exports, with two warm
profiles. Its ON capture has 16,336 samples and 99.9279% of weighted periods
inside known request interiors. Eighteen boundary-ambiguous and five
outside-request samples remain separate. The corrected reader requires at
least 90% weighted coverage and passes five parser/accounting tests.

| Phase in the clock-qualified capture | CPU ms/proof | Wall ms | Epoch + scheduler samples |
| --- | ---: | ---: | ---: |
| Solver | 9.199 | 1.233 | 38.92% |
| FFT product | 18.905 | 0.607 | 47.46% |
| MSM queries | 162.424 | 5.805 | 6.38% |

Sample shares are weighted user-cycle attribution, with inlining limitations.
They are not removable wall-time fractions. The fixed clock qualification
checks instrumentation; it makes no optimization adoption or faster-latency
claim. Total local phase work: 2,688 SDK requests, 64 distinct Gnark-verified
exports and 128 proof/public files. The later container/AWS campaign is
reported separately.

The Rust workflow passes 23 unit tests, formatting and Clippy. The Cargo alias
uses a release build for efficient large-image hashing. A copied capture's
ownership initially prevented root `perf script` from reading it; that reader
failure and subsequent decoding attempts are retained. Completed proof and
test processes were reused.

[Original protocol](artifacts/experiment.json),
[clock audit](artifacts/clock-audit.json),
[clock-qualified result](artifacts/clock-analysis/result.json),
[original phase observations](artifacts/analysis-v2/observations.json),
[diagnostic source](artifacts/overlay/crates/zopro-core/src/phase_cpu.rs),
[source recovery audit](artifacts/audit.rs),
[complete artifact hashes](SHA256SUMS.json).

Numeric results, protocols, proof exports, source and command receipts are
published here. Raw captures and expanded sample streams remain in the local
`artifacts` archive; the hash index covers that complete archive. No AWS,
phone, large-circuit or minimum-RAM conclusion follows from this local test.
The original all-eight tenfold CPU objective remains unmet.
