# Outline the bounded victim-snapshot scan

The fixed four-variant local speed screen fails. The candidate is not adopted and no unchanged timing rerun follows. All observations and failed checks are preserved.

Primary accepted/new OFF/outlined ON/old inlined ON is **8.622709/8.663186/8.638950/8.601599 ms**. Original-eight paired geometric outlined/accepted, outlined/OFF and outlined/inlined ratios are **1.007986/1.008686/0.992318**. Production remains unchanged; the all-eight tenfold CPU-reference goal remains unmet.

## Source and code-generation gate

The prepared source differs from the [previous inlined prototype](../2026-10-26-front-snapshot-fence/README.md)
by exactly one `#[inline(never)]` on Rayon's `steal_front_snapshot_tiles` method.
231 of 232 files are byte-identical; removing the attribute recovers the remaining
file byte-for-byte. The scan body, tile of eight, first scalar probe, saved-front
ordering, explicit common SC fence, normal epoch guard and retry semantics stay
identical. Kernels, dependency versions, queue layout, shrinking, reclamation
policy and proof representation are unchanged.

The source and linked-code gate ran before any SDK measurement. It requires a
separate scan symbol, a direct call from the scheduler, and a caller smaller
than the old inlined routine and no larger than twice the accepted caller.
Failure would close the experiment without SDK timing or another attribute.

The gate **passes**. `wait_until_cold` is 1661 bytes accepted, 5757 bytes inlined, and 1654 bytes outlined. The separate outlined scan is 4119 bytes. These sizes establish the intended boundary, not cache misses or a performance cause. [Exact linked disassembly](artifacts/codegen.txt) and [gate checks](artifacts/codegen-gate.json) are retained.

Scratch remains bounded by the same eight references/eight WorkerIndex values
plus ordinary call frames. No key table, retained buffer, copied point array,
queue field or per-proof scan heap allocation is added. Collection policy is
unchanged; changed scheduling can still change observed collection activity and
memory lifetime. Static footprint does not establish a whole-process RAM bound.

## Complete SDK comparison

Ryzen 9950X; seven rotated/reversed fresh-process blocks compare frozen accepted,
the new outlined executable OFF/ON, and the exact previous inlined executable ON.
The old prototype is a direct control for this source change; its original
negative result remains unchanged. All nine shapes use 32 workers on CPUs 0–31,
31 measured requests after three primes. Primary with 16 workers uses CPUs 0–15,
one logical CPU per physical core, and 101 measured requests.

Fresh solving, randomness, proving, internal verification and serialization are
timed. Loading and priming are excluded. All latency completes before sampling,
with no other owned build, benchmark, profiler or bulk audit overlapping.
Displayed times are medians of process medians; paired ratios are medians of
within-block ratios and can differ from ratios of displayed medians.

| Case | Accepted ms | New OFF ms | Outlined ON ms | Inlined ON ms | ON/accepted paired | ON/OFF paired | ON/inlined paired |
|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2-w32 | 8.622709 | 8.663186 | 8.638950 | 8.601599 | 1.001884 | 1.001586 | 1.004400 |
| 2x2-w32 | 11.769607 | 11.669077 | 11.786359 | 11.887239 | 1.001423 | 1.002174 | 0.997779 |
| 4x2-w32 | 16.411706 | 16.100078 | 16.379525 | 16.611514 | 1.000431 | 1.014573 | 0.985829 |
| 8x2-w32 | 24.476924 | 24.942502 | 25.157607 | 25.194868 | 1.012435 | 1.016424 | 0.986548 |
| 12x2-w32 | 34.556643 | 34.558356 | 34.297123 | 35.060773 | 0.992716 | 0.992441 | 0.984737 |
| 16x2-w32 | 40.949175 | 41.389176 | 40.780466 | 41.490004 | 0.997208 | 0.997995 | 0.985025 |
| 24x2-w32 | 68.358811 | 68.537127 | 69.289319 | 70.479302 | 1.013812 | 1.018988 | 0.985709 |
| merge-24x1-w32 | 67.243789 | 67.866942 | 69.319626 | 68.823048 | 1.030870 | 1.025313 | 0.998302 |
| merge-54x1-w32 | 176.364533 | 176.496112 | 179.274789 | 178.977861 | 1.016793 | 1.006303 | 0.985016 |
| 1x2-w16 | 11.063235 | 10.888986 | 10.492307 | 10.580904 | 0.956048 | 0.964663 | 0.991627 |

The declared gate requires primary and original-eight aggregate improvement against all three controls, primary wins in at least four of seven blocks, every case paired ratio ≤1.02, every block ratio ≤1.05, and OFF/accepted calibration within a factor of 1.03. There is no minimum gain floor or post-measurement exclusion.

Speed gate: **fail**; 13 failed checks, 1 OFF-calibration failures. [All paired checks](artifacts/pilot-result.json) are retained.

## CPU work and memory

CPU milliseconds are summed user+system time per measured request, median across processes. RSS is maximum whole-process high-water observation, including loading/primes; it is not minimum RAM or query-only storage.

| Case | Variant | CPU ms/request | Maximum RSS MiB | Voluntary switches/request |
|---|---|---:|---:|---:|
| 1x2-w32 | accepted | 199.588 | 113.50 | 411.9 |
| 1x2-w32 | off | 199.642 | 114.25 | 408.0 |
| 1x2-w32 | on | 193.517 | 116.00 | 587.0 |
| 1x2-w32 | inlined | 192.753 | 113.25 | 578.7 |
| 12x2-w32 | accepted | 961.220 | 713.25 | 876.2 |
| 12x2-w32 | off | 960.647 | 703.00 | 863.5 |
| 12x2-w32 | on | 944.051 | 728.25 | 1713.1 |
| 12x2-w32 | inlined | 951.263 | 707.75 | 1758.3 |
| merge-54x1-w32 | accepted | 4863.519 | 1928.25 | 1768.0 |
| merge-54x1-w32 | off | 4839.089 | 1862.46 | 1781.7 |
| merge-54x1-w32 | on | 4890.990 | 1875.76 | 3911.3 |
| merge-54x1-w32 | inlined | 4913.203 | 1878.48 | 3876.4 |
| 1x2-w16 | accepted | 129.295 | 88.00 | 261.3 |
| 1x2-w16 | off | 129.543 | 91.50 | 264.9 |
| 1x2-w16 | on | 129.843 | 86.75 | 335.6 |
| 1x2-w16 | inlined | 127.692 | 90.75 | 338.2 |

## Separate profiles

Twelve profiles at 999 Hz follow all latency runs using the monotonic clock. Shares are exclusive sampled CPU-period categories with symbol/inlining limits, not removable wall-time fractions. Zero means no attributed samples. Outlining can move attribution between symbols; CPU and latency remain separate observations.

| Case | Variant | CPU ms/request | Epoch % | Scheduler % | FFT % | Voluntary switches/request |
|---|---|---:|---:|---:|---:|---:|
| 1x2-w32 | accepted | 199.168 | 9.274 | 3.737 | 5.114 | 423.6 |
| 1x2-w32 | off | 199.888 | 9.604 | 4.397 | 4.967 | 406.8 |
| 1x2-w32 | on | 193.304 | 3.385 | 6.039 | 5.127 | 580.6 |
| 1x2-w32 | inlined | 194.289 | 3.906 | 6.844 | 4.998 | 601.5 |
| 12x2-w32 | off | 958.810 | 4.885 | 3.880 | 28.017 | 917.7 |
| 12x2-w32 | on | 977.825 | 2.209 | 5.342 | 27.539 | 1818.1 |
| 12x2-w32 | inlined | 946.383 | 2.160 | 5.638 | 28.140 | 1821.5 |
| 12x2-w32 | accepted | 949.953 | 5.643 | 3.994 | 26.878 | 933.4 |
| merge-54x1-w32 | on | 4897.932 | 1.127 | 2.969 | 41.936 | 3998.4 |
| merge-54x1-w32 | inlined | 4925.202 | 0.911 | 2.816 | 41.823 | 4188.7 |
| merge-54x1-w32 | accepted | 4897.727 | 2.148 | 2.119 | 41.515 | 1828.6 |
| merge-54x1-w32 | off | 4743.520 | 2.307 | 2.016 | 43.086 | 1893.9 |

399578 samples; zero lost records and exactly one sampled PID per capture. All twelve declared case/variant profile pairs are checked independently on closeout.

## Supplied M5 references

These are supplied historical CPU/Metal timings, not fresh matched M5 measurements. Merge54 has no supplied reference.

| Shape | CPU reference ms | Metal reference ms | Outlined ON ms | CPU/ON | Metal/ON |
|---|---:|---:|---:|---:|---:|
| 1x2 | 37 | 24 | 8.638950 | 4.28× | 2.78× |
| 2x2 | 59 | 30 | 11.786359 | 5.01× | 2.55× |
| 4x2 | 109 | 48 | 16.379525 | 6.65× | 2.93× |
| 8x2 | 189 | 74 | 25.157607 | 7.51× | 2.94× |
| 12x2 | 284 | 101 | 34.297123 | 8.28× | 2.94× |
| 16x2 | 335 | 124 | 40.780466 | 8.21× | 3.04× |
| 24x2 | 528 | 176 | 69.289319 | 7.62× | 2.54× |
| merge-24x1 | 507 | 175 | 69.319626 | 7.31× | 2.52× |

## Validation and provenance

Prepared source identity is checked across all 232 files before building.
The inherited five instrumented Loom models (three-preemption bound) are rerun,
including an expected-message missing-fence negative control. Both runtime modes
pass 44 deque, 113 Rayon (two upstream tests ignored), and 109 core tests. ON
raw/cubic tests pass 2/6. Field kernels are unchanged; inherited 33-test field
qualification is not claimed as a new execution. No new test mirrors the attribute.
The bounded models do not verify the full generic slot protocol, epoch collector
or scheduler; the previous allocation/drop/guard-free-job tests are retained.

The 219 runtime packages retain the same versions and source choices as the
inlined prototype. Source, tool, compiler, actual linked libraries, all three
executables, original keys, images, requests and verifier are pinned. The prior
registry-source audit is inherited, with actual file/archive pins checked again.
All nine shapes pass new OFF/ON preflight with four workers. Every exported proof
is independently Gnark-verified, has the expected 164-byte compressed format,
matches fixture public inputs, and is fresh against all prior proof identities.

An initial protocol-builder compile error from an ambiguous string conversion
was repaired before SDK preflight. The original helper and error log remain;
runtime sources and measurement counts were unchanged. All command exit and
stdout/stderr hash receipts, samples and source/binary guards are preserved.

No cloud resource or new Aeglos/C7a comparison is created here. Portable,
whole-SDK concurrency, pressure and very-large-circuit qualification remain
outstanding. Production remains unchanged.

Total: **12232 complete SDK requests, 310 distinct independently Gnark-verified exports, 620 proof/public members**, plus two fixed source-fixture members. Performance uses 929 input pins and rejects 3774 prior identities before preflight.

See [declaration](artifacts/experiment.json), [source identity](artifacts/outline-audit.json), [code-generation gate](artifacts/codegen-gate.json), [speed checks](artifacts/pilot-result.json), [closeout](artifacts/closeout.json), and [artifact index](artifacts/artifact-index.json). [Every request sample](artifacts/perf/rows.json) is retained. Twelve raw captures and twelve expanded streams are kept locally with hashes and omitted from Git. Executables/compiler caches are represented by build/link hashes; the old inlined source is preserved in its linked prior report.
