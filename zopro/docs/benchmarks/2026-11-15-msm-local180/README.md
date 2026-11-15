# Local full-request MSM phase comparison

The new coarse observers pass all 16 local calibration and overhead checks.
On 1x2, accepted Zopro measures **7.711 ms at 32 threads**, versus **12.793 ms
for the retained Aeglos Sorted reference**. Zopro's query stage shortens with SMT;
Sorted's MSM does not. This is diagnostic progress, not a new optimization or
evidence that the C7a gap has closed. Production Zopro is unchanged.

## Full-request measurements

This host has 16 physical cores and 32 logical CPUs. Compare one thread per core
(CPU0–15) with both SMT threads (CPU0–31). Each of nine blocks contains every
engine, control and worker count, with deterministic rotation and reversal.
Each process performs three primes and 51 timed, sequential verified requests.
The following rows use the exact retained reference binaries with tracing off.
Latency is the median of nine process medians; CPU is the mean process CPU per
request; RSS is the largest whole-process peak across the nine processes.

| Reference, 1x2 | Threads | Latency, ms | CPU, ms/request | Peak RSS, MiB |
|---|---:|---:|---:|---:|
| Zopro accepted | 16 | 9.022752 | 113.06 | 84.50 |
| Zopro accepted | 32 | 7.710818 | 173.30 | 115.00 |
| Aeglos Sorted | 16 | 12.848314 | 192.37 | 421.12 |
| Aeglos Sorted | 32 | 12.792590 | 377.56 | 427.45 |
| Aeglos Windowed | 16 | 16.622220 | 233.43 | 437.34 |
| Aeglos Windowed | 32 | 16.551417 | 472.54 | 472.46 |

Zopro improves in all nine paired blocks; the median 32/16 ratio is 0.855721.
Sorted's ratio is 0.999196, with five improving blocks. Windowed's is 0.997107,
with seven improving blocks. Small differences in the last two are not presented
as confirmed improvements. CPU consumption increases substantially for all three.
The reference RSS values include loading and preparation; they are not minimum
RAM requirements, incremental observer memory or memory-pressure measurements.

Zopro's fresh primary observation is 4.80× the supplied M5 CPU reference of37ms
and 3.11× its Metal reference of24ms. The CPU10x target is3.7ms and remains unmet.
These M5 timings are historical supplied references, not matched new measurements.
The other seven shapes were not rerun; their current accepted comparisons remain
in the [all-shape adoption report](../2026-11-12-bundle-adoption178/README.md).
The full goal remains unmet,0/8.

## What the phase records show

The following are means over all459 timed ON requests per context, after excluding
primes. Full-request controls above remain separate. All applicable local
calibration gates passed before interpreting these records.

| Instrumented phase | 16 threads, ms | 32 threads, ms |
|---|---:|---:|
| Zopro solve | 1.017169 | 1.103019 |
| Zopro scalar/query preparation | 0.123609 | 0.190642 |
| Zopro FFT | 0.481206 | 0.541762 |
| Zopro complete query span | 6.605582 | 5.086817 |
| Zopro output | 0.003844 | 0.004879 |
| Zopro unassigned inside observer | 0.001205 | 0.001249 |
| Zopro request outside observer | 0.751565 | 0.801956 |
| **Zopro complete ON request** | **8.984179** | **7.730325** |
| Aeglos Sorted accumulate | 8.560652 | 8.822815 |
| Aeglos Sorted complete MSM observer | 10.215081 | 10.254003 |
| Aeglos Windowed accumulate | 8.535181 | 7.996189 |
| Aeglos Windowed merge planning | 1.698213 | 2.300496 |
| Aeglos Windowed merge execution | 1.761865 | 1.910616 |
| Aeglos Windowed weighted sums | 0.812133 | 0.380536 |
| Aeglos Windowed complete MSM observer | 13.807916 | 13.658367 |

Zopro saves1.519ms in its query span, while solve, preparation, FFT and remaining
request time become slightly more expensive. Sorted's dominant accumulation phase
does not improve with SMT. Windowed's accumulation and weighted-sum savings are
largely offset by increased merge planning and merge execution. Its merge planning
occurs in the existing serial phase runner. These observations locate costs; they
do not identify a removable amount of CPU time or establish a causal optimization.

Sorted schedules640 accumulation items at16 threads and1280 at32, with no merge
execution in either context. Windowed schedules354 and652 accumulation items and
executes three and four merge rounds respectively. Its window/chunk geometry also
changes. Counts are scheduled items, not equal-cost arithmetic operations, total
field work, or a measured ready queue. Complete per-job counts and every phase,
including residuals, are preserved in the numerical artifacts.

Zopro's A, fused KH and G2 spans overlap. At32 threads, their mean spans are4.833,
4.994 and5.073ms; their completion tails to the end of all queries are0.253,0.091
and0.013ms. These callbacks can help other tasks or wait. Their durations cannot
be added or used alone to conclude that G2 arithmetic is the bottleneck.

The fresh local evidence therefore does not reproduce the C7a scaling reversal.
The [retained C7a decomposition](../2026-11-12-scaling-divergence180/README.md)
showed Sorted MSM improving from8.338 to3.901ms at32→96 physical cores. Here it
stays near10.2ms at16→32 SMT threads. Core topology and the measured phase behavior
must remain distinct. This does not show that RAM capacity, page faults, language,
or a particular pool alone explains the difference.

## Calibration and bounded instrumentation

Zopro has exact accepted REF and one diagnostic binary with OFF/ON controls.
Aeglos has retained REF, fresh unmodified native source with the matched Go
harness (FRESH), and the diagnostic binary with OFF/ON controls. Sorted/Windowed
selection and all existing key-preparation policies are retained. The fresh
Aeglos executable is not claimed byte-identical to its retained reference.

The prospective gate requires the paired median-ratio interval to lie wholly
inside[0.95,1.05]. It uses sorted block ratios at indices1 and7 of nine:
nominal96.09375% median coverage under independent continuous block ratios,
without a simultaneous-coverage claim. All16 gates pass: Zopro OFF/REF and
ON/OFF at both counts, and Aeglos FRESH/REF, OFF/FRESH and ON/OFF in both modes
at both counts. This qualifies these local diagnostics within that tolerance;
it is not proof of zero perturbation or an optimization-adoption threshold.

This diagnostic used one executable backing per binary, unlike the playbook's
three-backing small-change comparison. That limits calibration to the files
actually captured; it does not qualify layout/backing independence. No small
source-level gain or adoption follows from this panel. Preserve this limitation
and include the prescribed backing replicas in the target experiment.

| Observer ON/OFF | Threads | Paired median | Interval |
|---|---:|---:|---:|
| Zopro | 16 | 1.002619 | [0.998239,1.009633] |
| Zopro | 32 | 1.000316 | [0.995021,1.003792] |
| Aeglos Sorted | 16 | 1.001137 | [0.989066,1.007977] |
| Aeglos Sorted | 32 | 1.003745 | [0.991616,1.011138] |
| Aeglos Windowed | 16 | 1.003006 | [0.986155,1.019038] |
| Aeglos Windowed | 32 | 1.000965 | [0.999411,1.005279] |

The new Rust observer owns eight fixed slots,576bytes, plus a4096-byte formatting
buffer. It starts after the original worker-entry hop and is explicitly borrowed
by the existing query closures. No TLS/global recorder, per-task hook, extra
preparation barrier, arithmetic change or new pool is introduced. Unsupported
routes are marked invalid; this is deliberately a primary1x2 diagnostic, not
an all-shape production facility. Existing callback overlap is preserved.

The [qualified Aeglos observer](../2026-11-15-msm-stage180/README.md) and native
archive are reused unchanged. Its fixed recorder is10808bytes with a32768-byte
formatting buffer. Actual full-request records use at most9rows for Sorted and
17for Windowed, below its64-row limit. Both observers emit after their recorded
phase sequence. Setup, clocks, formatting and output remain in full-request time;
phase intervals exclude some observer bookkeeping and output. Residuals are retained.

## Verification and limits

The integrated Zopro build passes all134 core tests, including five new observer
tests, and coherently links the SDK libraries. Aeglos's existing qualified native
archive is linked with the retained full-request harness; a separate fresh original
archive supplies its build control. An initial Rust syntax error was repaired
before SDK execution. A postprocessing helper's first compile selected the wrong
Rust version; compiling with the pinned1.98.1 toolchain resolved it. Neither
failure generated a proof or caused a successful capture to be replayed.

Initial4-worker integration:44SDKcalls,11fresh exports. Local16/32 preflight and
timed matrix:10780SDKcalls,220fresh exports. All231exports independently verify
with Gnark against the fixed image/public bindings and are distinct from the prior
canonical ledger. SDK verification runs on every request; only the final proof
from each process is exported for independent verification. There are2940 local
phase records including primes, all structurally complete and bounded. A separate
post-capture reader reproduces10120 raw timed samples, every gate, stream receipt,
proof binding and the220-export ledger delta, without replaying SDKs or verifiers.

Recorded wall-clock milestones: integration design to coherent Zopro build about
6min03s; local protocol to capture start2min09s; functional preflights plus timed
capture179.5s; remaining verification and frozen analysis3.8s; separate raw audit
completed another1min44s later. Design to that audit was about16min02s, excluding
subsequent report assembly. These are elapsed workflow intervals, not CPU timings.

Zopro records zero major faults in all main processes. The retained Aeglos harness
does not report major faults, so no all-engine zero-major-fault claim is made.
Minor faults and switches are retained in the raw summaries; neither count measures
disk paging cost or explains elapsed time by itself. Host state and topology are
captured, but no new PMU sampling, mobile, ARM, TEE, huge-circuit, swap, concurrency
throughput or memory-pressure result is claimed.

The next target is the same calibrated phase comparison on C7a at32/96 physical
cores, retaining all controls and fresh verification. Local calibration does not
qualify overhead at96. AWS authentication currently fails with an expired SSO
token. No AWS resource was created, and no new C7a result exists. Do not repeat
the completed local capture or select another scheduler rewrite from these means.

Canonical verified-export ledger:55440, private path
`/home/ubuntu/src/zopro-perf-work/msm-local180/proof-sha256-after-capture.json`,
SHA256`db66bc98a0abbc4c89000e84331cc3eadf7fb6d873c9b2df5ba472cc5cd3df3c`.
Accepted SDK remains`b646c7b8c1d8a26a0db45fb3f67c453dd616bd9f02e25c36192721ae02855abe`.
Retained Aeglos source remains0290613e8eeca17a4ce88f6f2d726e8e06a5385d.
This report and experiment are local; GitHub publication requires user approval.

Private roots: `/home/ubuntu/src/zopro-perf-work/msm-full180` and
`/home/ubuntu/src/zopro-perf-work/msm-local180`.
[Numerical results](artifacts/review.json), [raw cross-check](artifacts/audit.json),
[prospective protocol](artifacts/protocol.md), [observer source](artifacts/msm_phase_probe.rs),
[sources, plans and receipts](artifacts/experiment-records.tar.gz).
