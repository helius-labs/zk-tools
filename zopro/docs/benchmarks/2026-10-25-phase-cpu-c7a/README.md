# C7a CPU scaling and request-phase accounting

The current Zopro build remains less efficient at high worker counts for 1x2.
In a fresh interleaved comparison, 32 to 96 workers increases latency from
12.66 to 14.29 ms; pinned Aeglos Sorted improves from 11.62 to 7.90 ms.
Phase accounting locates increased CPU work in solving, FFTs and MSM queries.
This diagnostic adopts no new prover implementation.

## Verified request latency and memory

One temporary c7a.24xlarge: AMD EPYC 9R14, 96 physical cores, one thread per
core, 192 GiB RAM, Linux 7.0.0-1014-aws. The container permits CPUs 0–95 at
every worker count, has no CFS CPU quota, and uses 30 GiB RAM with swap disabled.
No cgroup memory-limit or OOM events, swapping or CPU throttling were recorded.

Three rotated/reversed blocks interleave current Zopro and the previously
pinned Aeglos Sorted executable at each worker count. Each fresh process
loads/prepares its key and makes three priming requests, then measures 31
verified full requests. Loading and preparation are excluded from latency;
whole-process peak RSS includes both. Aeglos reports cache hits for every
measured request. Both use the same established request/public-input fixture.

| Workers | Zopro ms | Aeglos Sorted ms | Zopro peak MiB | Aeglos peak MiB |
| --- | ---: | ---: | ---: | ---: |
| 32 | 12.659135 | 11.619870 | 107.47 | 415.30 |
| 64 | 12.515354 | 8.574364 | 102.43 | 427.64 |
| 96 | 14.285473 | 7.903981 | 101.49 | 460.34 |

Times are medians of three process medians; RSS is the maximum of those three
processes. Paired Zopro/Aeglos ratios are 1.089439, 1.471977 and 1.807377.
They need not equal ratios of the displayed medians. These RSS measurements
establish neither incremental workspace nor minimum RAM.

A separate fixed three-variant phase matrix compares accepted, diagnostic OFF
and ON. Its accepted primary results are 12.632945 / 12.554244 / 14.177925 ms
at 32 / 64 / 96 workers. Merge-54 results are 139.348213 / 129.293800 /
132.123606 ms. Every process and both comparison phases remain in the data;
the interleaved result is not substituted for a favorable matrix process.
There is no new Aeglos merge-54 measurement here.

## CPU work and instruction counters

Separate warm counter processes perform 201 requests after three primes.
Cycles, instructions, branches and branch misses each run at least 99% of
the measured interval. FIFO acknowledgements enable counters after preparation
and priming, then disable them before proof export. Counts follow the launched
process and its threads. The counter timings do not replace latency results.

| Engine | Workers | CPU ms/proof | Mean occupied cores | IPC | Voluntary switches/proof |
| --- | ---: | ---: | ---: | ---: | ---: |
| Zopro | 32 | 292.624 | 22.02 | 1.491 | 417.75 |
| Zopro | 64 | 497.612 | 37.40 | 0.994 | 861.24 |
| Zopro | 96 | 854.613 | 54.09 | 0.708 | 1498.61 |
| Aeglos Sorted | 32 | 331.798 | 28.06 | 1.530 | 48.36 |
| Aeglos Sorted | 64 | 430.287 | 50.04 | 1.433 | 71.65 |
| Aeglos Sorted | 96 | 529.257 | 65.81 | 1.255 | 101.96 |

Occupied cores include work search and spinning. At 96 workers, Zopro retires
1.871 billion instructions per proof and uses 2.628 billion user cycles;
Aeglos retires 2.107 billion instructions and uses 1.677 billion cycles.
The difference directs research toward coordination and cost per instruction.
Instruction counts, IPC and samples alone do not isolate cache misses, DRAM
bandwidth or a removable critical-path fraction.

## Where the additional CPU work occurs

The private recorder uses a fixed 32-stamp process buffer and allows one active
diagnostic request. It brackets process-wide user/system CPU accounting with
`CLOCK_MONOTONIC`; TIDs retain caller-to-worker handoff evidence. Removing its
insertions recovers three production call-site files byte for byte. Arithmetic
and scheduling decisions remain unchanged.

The following spans come from unprofiled diagnostic ON processes. Values are
medians of process means, with 31 requests per process. Separate medians need
not sum to request totals.

| Phase | Workers | Wall ms | Process CPU ms |
| --- | ---: | ---: | ---: |
| Solver | 32 | 2.236 | 17.972 |
| Solver | 64 | 2.686 | 43.048 |
| Solver | 96 | 2.919 | 80.764 |
| FFT product | 32 | 0.905 | 29.006 |
| FFT product | 64 | 1.145 | 73.301 |
| FFT product | 96 | 1.376 | 121.855 |
| MSM queries | 32 | 7.877 | 220.914 |
| MSM queries | 64 | 6.774 | 316.130 |
| MSM queries | 96 | 7.408 | 444.017 |

At 96 workers, solving and FFTs both take longer than at 32, while queries
gain little. More workers increase process CPU work throughout the request.

Separate 999 Hz cycle captures align with the same monotonic clock. Across
18 ON captures, 99.3387% or more of weighted periods lie inside known request
interiors. Boundary-ambiguous and outside-request samples are retained
separately. Each of all 54 captures has one sampled PID and no lost records.

| Phase | Epoch + scheduler share, 32 workers | 64 workers | 96 workers |
| --- | ---: | ---: | ---: |
| Solver | 46.77% | 75.92% | 87.72% |
| FFT product | 55.83% | 73.57% | 82.36% |
| MSM queries | 5.30% | 23.44% | 44.02% |

These are weighted sampled user-cycle shares, with inlining limitations.
The accepted executable's whole-request epoch/scheduler shares are about
15.00 / 41.29 / 61.34% at 32 / 64 / 96 workers. The observations support
investigating worker admission by phase and repeated work-search overhead.
Earlier empty-epoch-collection and fixed-time idle changes failed latency
checks; smaller epoch shares alone did not make those changes successful.
This report does not revive them.

Primary has 16 fine stamps. Merge-54 uses another inner solving path and has
nine coarse stamps; its internal solver/FFT/MSM split remains unavailable.
OFF/accepted median calibration factors stay within 1.017; ON/OFF primary
factors stay within 1.016. Instrumented profiles remain diagnostic.

## Validation, provenance and cleanup

The server build targets `znver4` and passes 33 field tests, 109 accepted core
tests, 110 diagnostic core tests, and two raw/six cubic tests for each SDK.
The source baseline is the qualified 170-file grouping-module configuration;
the generic CLI/helper are additional frozen link inputs. Native executables
and all eight linked libraries have exact build receipts. The Rust orchestrator
passes 23 tests, formatting and Clippy; the phase reader adds five tests and
the comparison reader adds two.

- Phase matrix: 4,374 SDK requests, 108 exports, 54 profiles.
- Interleaved primary latency/counter comparison: 4,284 requests, 36 exports.
- Remote native/Aeglos preflight: 28 requests, seven exports.
- AWS total: 8,686 requests and 151 distinct exports. All 302 proof/public
  members match fixtures and retain unique proof identities. Every export
  verifies independently with Gnark remotely and again after retrieval locally;
  the second check is not counted as another export.
- All 54 profiles contain 2,913,097 samples with no lost records. Local image
  preflight adds 28 requests and seven distinct exports; the earlier local
  phase/clock experiment is reported separately.

The first profiler failed before launching the SDK because the container's
locked-memory allowance could not map the counter buffers. A bounded 256 MiB
container memlock allowance permitted recording. Resume reused all 54 completed
latency processes and their proof checks; the failed record and command logs
remain. The source, image, clock, ring-buffer size and proof commands did not
change, and no global kernel setting was modified.

The complete 74,839,024-byte archive was retrieved through an AWS Session Manager
tunnel and a loopback-only single-file Rust server. Its SHA256 matches the
remote receipt. The owned instance is terminated, its EBS volume is absent and
its private benchmark image tag is deleted. No production infrastructure,
security group or IAM role was changed.

The earlier local default-clock attribution failure remains recorded in the
[local phase report](../2026-10-25-phase-cpu-local/README.md). This C7a campaign
uses the explicitly qualified monotonic clock. Source, build flags and kernel
differ from older C7a checkpoints, so differences from those checkpoints are
not attributed to one code change.

The supplied 37 ms CPU / 24 ms Metal primary references give 2.956x / 1.918x
against the current 64-worker Zopro result. They are historical references,
with no fresh M5 timing here. This two-shape diagnostic does not remeasure all
eight targets. The original all-eight tenfold CPU objective remains unmet.
There is no new phone, TEE, 250/4999, minimum-RAM or concurrent-throughput claim.

[Comparison and counters](artifacts/comparison-analysis/result.json),
[all comparison samples](artifacts/comparison-analysis/rows.json),
[phase attribution](artifacts/phase-analysis/result.json),
[unprofiled phases and calibration](artifacts/calibration-and-unprofiled-phases.json),
[fixed protocol](artifacts/experiment.json),
[cleanup receipt](artifacts/cleanup-complete.json),
[archive receipt](artifacts/retrieval-complete.json),
[artifact hashes](SHA256SUMS.json).

Numeric data, proof exports, source overlays and command receipts are published
here. Raw captures, expanded sample streams and native executables remain in
the local artifact archive. The hash index covers that complete archive;
the immutable image and source/link receipts record the measured inputs.
