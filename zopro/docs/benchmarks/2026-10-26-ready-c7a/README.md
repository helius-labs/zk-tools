# Current Zopro and Aeglos: C7a scaling and arithmetic readiness

On a fresh c7a.24xlarge, current accepted Zopro takes **12.383 ms** for 1x2 at its best tested 32 workers; pinned Aeglos Sorted takes **7.885 ms** at 96. For 8x2, Zopro takes **27.374 ms** at 64 workers versus Aeglos's **31.089 ms** at 96. This confirms the remaining small-proof difference with the current 170-file Zopro build. It does not establish a new optimization or production adoption.

The new diagnostic finds about 25 active recorded arithmetic intervals at 32 workers and 30 at 96. More workers largely remove the delay between declaring G2 fill units ready and starting them, but G2's declaration remains late. The arithmetic interval barely shortens, while SDK time outside it grows. Coordination samples also increase substantially. These observations favor investigating preparation and dependency boundaries before another field-kernel or queue change.

## Request latency

Dedicated c7a.24xlarge, 96 physical cores without SMT, EPYC 9R14, 30 GiB container. Three rotated/reversed fresh-process blocks at 16/32/64/96 workers. All engines use taskset on CPUs 0 through N−1. Primary measures 31 requests and 8x2 measures 17 after 3 primes. Loading and preparation are excluded; fresh witness solving, randomness, proving, SDK verification and serialization are included. Independent Gnark verification runs between completed process captures. No build or other benchmark overlaps a request or profile.

Both Zopro builds use target-cpu=znver4 and the accepted flags and environment. Accepted is the clean 170-file source. OFF and ON use the same private diagnostic executable; ON records arithmetic intervals. Aeglos uses the same pinned 0290613 Sorted executable as the earlier comparison. Its native byte budget and preparation policy are retained. This run pins compact CPU masks, unlike the earlier unpinned worker-count comparison; cross-run changes cannot be attributed solely to source changes.

Each cell is the median of three process medians in milliseconds. Every sample is retained.

| Circuit | Workers | Accepted Zopro | Diagnostic OFF | Diagnostic ON | Aeglos Sorted |
|---|---:|---:|---:|---:|---:|
| 1x2 | 16 | 18.376383 | 18.161999 | 18.133082 | 19.265104 |
| 1x2 | 32 | 12.383334 | 12.333426 | 12.293644 | 11.832472 |
| 1x2 | 64 | 12.796556 | 12.871939 | 12.841626 | 8.859061 |
| 1x2 | 96 | 14.166512 | 14.228332 | 14.232280 | 7.885125 |
| 8x2 | 16 | 41.940992 | 42.286054 | 42.140192 | 104.148266 |
| 8x2 | 32 | 28.005279 | 27.959784 | 27.974529 | 54.395062 |
| 8x2 | 64 | 27.374031 | 27.096330 | 27.186686 | 35.340691 |
| 8x2 | 96 | 29.212100 | 28.286679 | 29.635926 | 31.089053 |

The [machine-readable analysis](artifacts/analysis.json) is authoritative for all precision, paired ratios, CPU/resource observations and failures. Best-worker selection is descriptive and is not a qualified production worker-selection policy.

## CPU time and whole-process RSS

These are uninstrumented accepted/Aeglos observations. CPU is median warm-loop user+system time per request, summed over threads. It includes coordination and is not a count of useful arithmetic. RSS is the maximum complete SDK process across the three repetitions, including loading, preparation and primes; it is not incremental scratch, cgroup peak or minimum RAM.

| Circuit/configuration | Request ms | CPU ms/request | Maximum RSS MiB |
|---|---:|---:|---:|
| 1x2 Zopro32 | 12.383 | 273.893 | 100.184 |
| 1x2 Zopro96 | 14.167 | 735.815 | 102.871 |
| 1x2 Aeglos32 | 11.832 | 327.628 | 410.695 |
| 1x2 Aeglos96 | 7.885 | 519.865 | 459.184 |
| 8x2 Zopro64 | 27.374 | 1298.343 | 451.383 |
| 8x2 Aeglos96 | 31.089 | 2588.370 | 2464.203 |

At their selected counts, Aeglos/Zopro RSS ratios are 4.58 for 1x2 and5.46 for 8x2. Concurrent throughput and memory-pressure behavior are unmeasured. Lower summed CPU time alone does not establish better throughput.

## What the arithmetic recorder adds

The [local diagnostic](../2026-10-26-ready-arithmetic/README.md) describes the unchanged bounded recorder and its source/test qualification. Every primary proof here records 1,275 nonforking units, including21 A fills,19 fused K/H fills and20 G2 fills. The remaining units are much shorter tree/reduction groups. A declared-ready unit is a data-ready independent unit of the existing iterator, not necessarily an already enqueued Rayon job. An active interval can include OS descheduling or memory stalls; it is not proof of on-CPU execution.

| Primary workers | Recorded arithmetic interval ms | SDK time outside that interval ms | Mean active recorded intervals | Median G2 ready-to-start delay us |
|---|---:|---:|---:|---:|
| 16 | 13.707 | 4.393 | 13.62 | 4949.1 |
| 32 | 7.536 | 4.752 | 25.17 | 1158.9 |
| 64 | 6.737 | 6.119 | 30.60 | 6.7 |
| 96 | 7.238 | 7.062 | 30.00 | 8.2 |

Interval and outside-time calculations are paired within each traced SDK request before aggregation; independent medians need not add. Outside time includes both prefix and suffix around the recorded interval. It is not an exclusive solver, FFT or verification cost, and uninstrumented work may also occur inside the interval.

At 96 workers, G2 fill units are declared a median3.014 ms after the first recorded arithmetic batch, and then start quickly. Median G2 fill body duration is3.361 ms; fused K/H fill bodies take3.434 ms. The last G2 fill ends around6.631 ms into the recorded interval, followed by about 0.610 ms of recorded work. These separate medians describe the captured distribution, not one synthetic trace or a complete dependency graph. At 32 workers, G2 declaration is2.603 ms after the first batch and its ready-to-start delay is much longer. Thus additional cores improve dispatch of already declared work, but do not shorten the overall arithmetic interval substantially.

This distinguishes the problem from simply having too few total tasks or uniformly slow dispatch. Late preparation, long indivisible fills, work outside the observed interval and coordination are plausible contributors. The current records do not isolate the time spent in grouping, scalar decomposition, entry, FFT or final verification. Those boundaries should be measured before deciding which mechanism to change. Prior negative whole-query overlap, extra-partition, indexed-tree and executor experiments remain relevant; none is revived unchanged by this report.

The 8x2 large-G2 streaming path is not instrumented by this recorder. Its observed interval concurrency cannot be interpreted as complete arithmetic utilization. The full tracing qualification fails at 8x2/96: paired ON/OFF1.041830, block0 ON/OFF1.058557, and OFF/accepted0.964541 and0.960199 in blocks1/2. All primary configurations pass their individual declared calibration checks, but the full-matrix failure remains. No observation is excluded or repeated and no instrumented timing replaces accepted latency.

## Warm CPU profiles

Eight separate199Hz cycles:u profiles follow all latency measurements: primary32/96 for all four modes,51 requests after 3 primes. They contain61,934 samples, zero lost records and one sampled SDK process each. Profile timings are not latency-acceptance measurements.

Accepted Zopro's epoch+scheduler category share changes from 13.74% at 32 workers to60.65% at 96. G1+G2 categories change from 69.35% to28.34%. These are aggregate sampled user cycles, not removable fractions of request wall time. Aeglos's Pool::work symbol accounts for about 27.2%/27.5% at 32/96; it also has coordination costs. Its MSM stage timer falls from 8.160 ms to4.047 ms, while its request-minus-MSM remainder changes from 3.664 ms to3.930 ms. Aeglos's stage timer and Zopro's selected arithmetic interval have different boundaries; only their respective scaling is compared here.

RAM capacity or bandwidth saturation is not established as the sole cause. No server memory-access sampling or instruction/cache-miss counter experiment occurs here. Affinity is applied by taskset; the captured perf events omit the CPU-ID attribute, so postcapture decoding cannot reconstruct sampled CPU placement.

## Verification, profiling repair and cleanup

Clean/diagnostic Zen4 targets pass33 field tests each,109/113 core tests and two raw plus six cubic tests each. The exact-container preflight completes 32 SDK calls and8 Gnark exports. AWS completes 2,592 latency SDK calls plus432 profiling SDK calls, with 104 distinct exports. Combined: **3,056 SDK calls,112 distinct Gnark exports and224 proof/public members**. All 112 exports verify again locally after transport; repeat verification is counted once. Fixture public hashes,164-byte proofs, source/image/executable hashes and disjoint proof identities are checked against5,104 preceding exports. The next complete prior set contains5,216 identities.

The container's Ubuntu perf launcher initially rejected host kernel7.0.0-1014-aws after all 96 latency processes completed. It exited before starting the first profile SDK: no proof directory or perf.data exists for that failure. A separate /bin/true availability check validates the already installed perf6.8.12 executable. Prepending its directory to PATH in the same pinned image allows identical-config xtask --resume to reuse every successful latency/verifier checkpoint and collect the eight profiles. The failed command, stderr and initial container inspection are retained. No successful SDK run, timing or profile is repeated. No prover source, binary, input, iteration count or gate changes.

The initial latency container lacks a final cgroup-event snapshot because its outer runner stopped on profile startup failure. Initial/final container states are retained and the resumed profile container has separate before/after counters. This report does not claim zero latency throttling from missing counters. Other retained setup corrections concern adapter validation, local mount ownership, transport CLI responses and analyzer compilation; they do not change a prover or repeat captures.

The owned instance is **terminated**. Its EBS volume, temporary ECR tag, transfer object and private transfer bucket are confirmed absent. No temporary benchmark instance remains running in the region. Production services, networking and IAM policies are unchanged. [Cleanup receipt](artifacts/cleanup-complete.json).

## Supplied M5 references and scope

These are supplied historical references, not fresh Apple measurements. Ratios use accepted Zopro's best tested worker count.

| Circuit | C7a Zopro ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |
|---|---:|---:|---:|---:|---:|
| 1x2 | 12.383 | 37 | 2.99× | 24 | 1.94× |
| 8x2 | 27.374 | 189 | 6.90× | 74 | 2.70× |

The [latest full local shape comparison](../2026-10-26-g2-owned-finish/README.md) and [earlier nine-shape C7a comparison](../2026-10-26-c7a-architecture-review/inputs/2026-10-25-latest-c7a-aeglos.txt) remain separate evidence. This run measures two shapes and does not establish the tenfold objective for all eight. It adds no TEE-overhead, phone, concurrent-throughput, pressure, swap or huge-circuit qualification. Accepted production remains unchanged.

[Full analysis](artifacts/analysis.json), [paired geometry](artifacts/geometry.json), [declared design](artifacts/design.md), [raw rows](artifacts/remote-results/output/measure/performance/rows.json), [source/build pins](artifacts/build-pins.json), [posttransport Gnark log](artifacts/post-transport-gnark.log) and [artifact index](artifacts/artifact-index.json) provide the complete retained result. Task JSONs are compressed losslessly. Raw perf.data and executables remain local with hashes; decoded profiles, proof exports and build/lifecycle records are archived. Reproduction uses the recorded source, flags, existing key inputs and private base image/cache; this archive is not a standalone public build environment.
