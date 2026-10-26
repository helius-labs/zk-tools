# Bounded simultaneous victim scans: C7a target stage

The frozen target screen passes at 64 and 96 workers and fails at 32.
Primary improves by about 2% against accepted and 2.6–4.0% against the same
candidate binary switched OFF at the two higher counts. Aeglos remains
faster on primary. The candidate is preserved without production adoption;
all-nine, portable, concurrent and memory-pressure qualification is pending.
The [local-stage failure](../2026-10-26-bounded-steal-search-local/README.md)
remains a failure.

## Change and protocol

The exact private Rayon-core prototype from the local stage allows at most
`ceil(workers/2)` simultaneous finite victim sweeps per registry, using a
cache-padded counter and scoped permit. `ZOPRO_CPU_BOUNDED_STEAL_SEARCH=1`
selects ON; absent/zero selects OFF. Local/broadcast and injected work remain
available to deferred workers. The permit drops before job execution,
parking and retry. Queues, fences, epoch reclamation, cryptographic source
and worker counts retain their implementations. No new key tables, copies,
execution plans or per-proof heap buffers are introduced. Scheduling can
change allocation lifetimes and memory peaks.

This target stage was declared before compilation regardless of the local
speed sign, because prior C7a profiles showed much higher epoch overhead at
96 physical cores. One temporary `c7a.24xlarge` provides 96 physical cores,
SMT disabled. All cases use CPUs 0–95, no CPU quota, a 30 GiB container
limit, no swap and identical `znver4` crypto settings. The counter/profile
observer has a 256 MiB memlock limit and 512 perf buffer pages.

Three rotated/reversed fresh-process blocks measure accepted, OFF and ON
at 32/64/96 workers: 31 primary or nine merge-54 requests after three primes.
Fresh work includes solving, randomness, proving, internal verification and
serialization. Key loading and preparation are excluded. All unprofiled
latency, including the separate Aeglos comparison, finishes before any
sampling or counters. No other owned CPU workload runs on the instance.
Rust xtask orchestrates all measurements, receipts and verification.

## Main SDK matrix

| Case | Accepted ms | OFF ms | ON ms | Paired ON/accepted | Paired ON/OFF |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1x2 / 32 | 12.664744 | 12.769673 | 12.748023 | 1.004521 | 0.994941 |
| 1x2 / 64 | 12.751725 | 12.807365 | 12.518493 | 0.977372 | 0.974242 |
| 1x2 / 96 | 14.300623 | 14.497226 | 14.001573 | 0.979995 | 0.960296 |
| merge-54x1 / 32 | 140.952759 | 140.384480 | 139.366489 | 0.988746 | 0.992965 |
| merge-54x1 / 64 | 133.196539 | 127.968586 | 127.297767 | 0.949676 | 1.002300 |
| merge-54x1 / 96 | 131.813642 | 131.928022 | 129.318931 | 0.981074 | 0.980223 |

Times are medians of process medians. Ratios pair corresponding blocks, so
ratios of displayed medians differ. The predeclared screen requires a
positive primary paired result and at least two of three primary wins
against both controls, merge regression at most 3%, each block at most 5%,
and OFF calibration within 1.03. No minimum gain floor is used. Primary
wins against accepted are 0/3, 2/3 and 3/3 at 32/64/96. All observations,
including the variable 64-worker merge blocks, remain in
[the result](artifacts/local-review/result.json). No unchanged timing rerun
was performed.

## Fresh primary comparison

A separate rotated three-block series uses the same Zopro binaries and the
pinned Aeglos Sorted build at `0290613e8eeca17a4ce88f6f2d726e8e06a5385d`.
The branch head was checked before this experiment and matched that build.
Prepared shifted bases are enabled, and every Aeglos request reports a
setup-cache hit. Both engines perform fresh complete requests and export
proofs for independent Gnark verification.

| Workers | Accepted ms | OFF ms | ON ms | Aeglos ms | Accepted / ON / Aeglos peak RSS MiB |
| --- | ---: | ---: | ---: | ---: | ---: |
| 32 | 12.672823 | 12.696722 | 12.663603 | 11.598883 | 103.87 / 105.97 / 407.61 |
| 64 | 12.928104 | 12.784424 | 12.589153 | 8.648162 | 97.09 / 104.53 / 429.88 |
| 96 | 14.308684 | 14.415315 | 13.974226 | 8.010473 | 105.94 / 101.46 / 463.40 |

RSS is the maximum across the ordinary latency processes and includes
loading/preparation. This is primary-only Aeglos timing and does not supply
a fresh Aeglos merge-54 result. The main and comparison matrices are kept
separate; they are not pooled or selected by their best process.

Separate counters start after loading/preparation and three primes, and
stop before proof export. All four events run for at least 99% of their
enabled interval. CPU time includes spinning. Instructions per cycle (IPC)
and occupied CPUs describe the measured process, not useful work alone.

| Workers | Engine | CPU ms/request | IPC | Voluntary switches/request |
| --- | --- | ---: | ---: | ---: |
| 32 | accepted | 293.729 | 1.487 | 427.45 |
| 32 | ON | 298.850 | 1.436 | 352.31 |
| 32 | Aeglos | 326.452 | 1.557 | 53.56 |
| 64 | accepted | 496.672 | 0.998 | 847.66 |
| 64 | ON | 512.498 | 0.934 | 697.68 |
| 64 | Aeglos | 431.070 | 1.442 | 70.59 |
| 96 | accepted | 859.217 | 0.717 | 1518.76 |
| 96 | ON | 862.847 | 0.642 | 1183.15 |
| 96 | Aeglos | 537.495 | 1.248 | 101.70 |

Eighteen separate 999 Hz captures use the monotonic clock and contain
965,839 samples, one process per capture, with no lost samples. At 96
workers, primary epoch share falls from 48.67% OFF to 28.44% ON, while the
scheduler category rises from 12.37% to 23.74% and other work rises from
4.63% to 14.42%. Profiled CPU time rises from 813.77 to 853.55 ms/request.
The latency gain does not establish a comparable total-CPU saving: much
of the overhead moved. Category shares are sampled user-cycle weights,
include attribution limits from inlining and are not removable wall time.
[Profile observations](artifacts/remote-results/output/perf/report.md)
retain all categories and raw-capture receipts.

## Memory and qualification

| Main case | Accepted peak MiB | OFF peak MiB | ON peak MiB |
| --- | ---: | ---: | ---: |
| 1x2 / 32 | 102.16 | 104.53 | 103.13 |
| 1x2 / 64 | 104.61 | 103.66 | 100.73 |
| 1x2 / 96 | 103.11 | 104.49 | 104.69 |
| merge-54x1 / 32 | 1519.53 | 1488.98 | 1502.18 |
| merge-54x1 / 64 | 1710.29 | 1687.20 | 1783.29 |
| merge-54x1 / 96 | 1824.31 | 1918.33 | 2030.51 |

These are ordinary latency-process peaks, distinct from profiler RSS.
Merge-54's higher ON peaks at 64/96 workers make pressure testing material
even though the policy adds no per-proof buffer. There is no demonstrated
minimum RAM, mobile, TEE, large-circuit or concurrent-request qualification.
Observed container checks find no CPU quota, throttling, memory-event delta,
OOM or swap under the 30 GiB configuration. That is not a pressure test.
No automatic worker policy or default dependency replacement is adopted.

## Validation and archival record

The target build passes 33 field, 104 Rayon library plus 11 integration,
109 core, two raw-image and six cubic tests. The earlier OFF/ON integration,
overload and Loom evidence and its limits are recorded in the local report.
Source, transitive library and executable hashes are frozen. Accepted
production's 170-file manifest remains unchanged.

The AWS preflight completes 28 SDK requests and seven exports; main latency
and profiles add 2,286 requests and 72 exports; comparison latency/counters
add 8,568 requests and 72 exports. Total: **10,882 requests, 151 distinct
exports and 302 proof/public files**, all independently Gnark-verified,
fixture-bound and checked for fresh proof identities. After download, the
frozen reviewer reproduces the remote result byte-for-byte and all 151
exports pass local Gnark verification again. Re-verification is not counted
as new exports. The separate exact-image local preflight adds 28 requests
and seven distinct exports, excluded from the AWS total.

The first local image attempt failed before any proof because an empty
bind directory had the wrong owner after dropping container capabilities.
A scoped owner correction preceded the successful exact-image preflight.
The first loopback archive server expired before retrieval; only the file
server was restarted. The unchanged archive then passed its recorded
25,038,158-byte size and SHA-256 checks. No benchmark or proof process was
restarted to repair transport.

The owned instance is terminated, its EBS volume is gone and its ECR tag
is deleted. No bucket, security-group, IAM or production change was made.
[Cleanup receipt](artifacts/cleanup-complete.json),
[resource checks](artifacts/resource-review-result.json),
[local verification](artifacts/local-verify/result.json),
[closeout](artifacts/closeout.json),
[protocol](artifacts/context/declaration.json),
[image/build identity](artifacts/image-manifest.json),
[proof inventory](artifacts/local-review/proof-inventory.json).

The best ON primary in the main matrix, 12.518493 ms, is 2.96 times the
supplied 37 ms M5 CPU reference and 1.92 times its 24 ms Metal reference.
Those are supplied historical references, not matched-machine runs. No
merge-54 reference was supplied. The all-eight 10x CPU goal remains unmet.

Full local experiment: `/tmp/zopro-bounded-steal-search/aws/`.
Raw perf captures and expanded symbol streams remain locally available and
are omitted from Git; the artifact index includes their hashes. Binaries,
compiler caches and private infrastructure launch/identity payloads are
excluded. Their relevant source/build/link and image identities are retained.
