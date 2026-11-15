# Preparation and G2 completion on C7a

The detailed full-proof trace localizes the preparation increase to canonical
conversion and scalar gathering. G2 becomes the late dependency because it
scales less than G1: its own callback becomes faster, not slower. Little of its
late tail occurs after all G2 windows have finished. This is a diagnostic;
production and the accepted SDK remain unchanged.

The C7a observer passes 15/16 calibration/backing checks and all ten environmental
checks. Its disabled build fails the 96-core OFF/REF interval guard. Consequently
96-core substages describe the diagnostic build and are **not qualified as exact
attribution to accepted production**. ON/OFF passes at both counts. The local
observer passed all 16 checks. No threshold, block or capture was changed.

## Full verified requests

One c7a.24xlarge, 96 physical cores without SMT, primary 1x2. Nine rotated/reversed
blocks compare accepted REF and a shared diagnostic OFF/ON binary at 32/96 cores.
Three independently written byte-identical backings per binary are balanced
across blocks. Each main process makes three primes and 31 timed requests.
Two functional OFF/ON preflights at four workers precede the main panel.

| C7a build | Cores | Median proof, ms | Mean CPU, ms/request | Maximum RSS, MiB |
|---|---:|---:|---:|---:|
| Accepted REF | 32 | 11.226110 | 243.55 | 107.50 |
| Diagnostic OFF | 32 | 11.242709 | 243.36 | 104.61 |
| Diagnostic ON | 32 | 11.252924 | 243.94 | 103.69 |
| Accepted REF | 96 | 12.815133 | 688.16 | 94.05 |
| Diagnostic OFF | 96 | 13.326855 | 775.89 | 93.90 |
| Diagnostic ON | 96 | 13.407435 | 773.37 | 94.78 |

Latency is the median of nine process medians. RSS is the largest whole-process
peak including key loading and primes, not minimum RAM or incremental scratch.
This run adds no new Aeglos timing. The preceding [C7a comparison](../2026-11-15-msm-c7a180/README.md)
measured retained Aeglos Sorted at 8.097657 ms/96 cores on a different instance.
Current accepted 1x2 is 3.30x the supplied M5 CPU 37 ms reference and 2.14x Metal 24 ms;
the CPU 10x target remains 3.7 ms. Other shapes are not rerun; retain the
[accepted all-shape comparison](../2026-11-12-bundle-adoption178/README.md).

## Preparation: two parallel passes explain the observed increase

Means below cover all 279 timed ON requests per count, excluding primes.

| Preparation boundary | 32 cores, ms | 96 cores, ms | Increase, ms |
|---|---:|---:|---:|
| Complete preparation | 0.387020 | 1.209255 | 0.822236 |
| W canonicalization | 0.179057 | 0.590807 | 0.411750 |
| Combined A/B gather span | 0.204480 | 0.612418 | 0.407939 |
| Public correction | 0.002000 | 0.001727 | -0.000272 |
| Reclamation | 0.000064 | 0.000051 | -0.000014 |
| Drop C | 0.000033 | 0.000035 | 0.000001 |

Canonicalization and gathering increase in all nine blocks and together account
for 0.819689 ms of the 0.822236 ms increase in this build. A/B gather callbacks
overlap; the table uses their combined span rather than adding their durations.
Reclamation is not the large cost here. This does not distinguish useful field
work, allocation, cache effects and synchronization inside the two passes.

## G2: the tail is mostly before the last window completes

| Callback or substage | 32 cores, ms | 96 cores, ms |
|---|---:|---:|
| A callback | 6.350673 | 4.900367 |
| KH callback | 6.303471 | 4.124472 |
| G2 callback | 7.044007 | 6.630071 |
| G2 scalar-group admission | 1.712702 | 1.934685 |
| G2 scalar splitting | 0.838994 | 0.605066 |
| G2 window collection | 4.430201 | 4.024872 |
| G2 final combination | 0.060799 | 0.063859 |
| G2 tail after both G1 callbacks | 0.438095 | 1.280190 |
| Collection return after last G2 window | 0.030743 | 0.042770 |

G2 window collection improves in all nine blocks; its finishing tail nevertheless
increases in all nine. The tail is computed per request before averaging. G1
callbacks improve more, exposing the remaining G2 dependency. These overlapping
callbacks must not be added into a complete-proof decomposition.

At 96 cores, the last-finishing window's span through part completion occupies about
0.880 ms of the tail; its subsequent merge and weighting intervals contribute
0.141 and 0.132 ms. The part-join gap contributes 0.043 ms, the collection-return
gap occurring after G1 finishes 0.031 ms, and final combination 0.052 ms. These
are descriptive wall intervals; they can include helping/waiting and do not
establish exclusive CPU, hardware utilization or automatically removable time.
The 0.043 ms collection-return interval is too small to account for the full
1.280 ms callback tail in this diagnostic.

No grouped-point reduction occurs: optional scalar-group admission returns no
groups. This trace does not record whether that decision comes from cardinality,
resource denial or another admission path. Do not label its entire duration a
rejected hash-map build. G2 starts within about 0.003 ms of the query span at 96.

Source review identifies ten ordinary windows, each with two point ranges whose
borrowed input feeds process the high component then the low component. This
exposes twenty ordinary feed streams; top-window reduction, normalization,
merging and weighting have their own parallel work, so twenty is not a global
worker-utilization limit. The narrow top window is never last in the 279 timed
96-core traces; ordinary windows 0–9 all appear as the last finisher.

## Controls and prior experiments

All 12 backing medians and three of four interval guards pass. The sole failed
guard is 96-core OFF/REF: paired median 1.033358, interval [1.023449,1.053941],
versus the frozen [0.95,1.05] range. This is a calibration limitation, not a proof
correctness failure. ON/OFF intervals are [0.994427,1.016282] at 32 and
[0.984441,1.021610] at 96. CPU throttling, memory high/max/OOM and swap-event deltas
are zero. This is not a memory-pressure, phone, huge-circuit or concurrency test.

The local Ryzen 9950X screen passed 16/16 guards: accepted 1x2 was 8.811371 ms at 16
workers and 7.657485 ms at 32. Local collection returns only 0.008579/0.007303 ms
after the final window. Local preparation is likewise mainly conversion/gathers.

Earlier experiments already tested [bounded gathers](../2026-11-11-bounded-gather-sdk177/README.md),
[global grouping bypass](../2026-10-28-dedup-off174/README.md),
[grouping lower bounds](../2026-10-26-grouping-lower-bound/README.md),
[independent G2 component planes](../2026-10-31-g2-independent-planes176/README.md)
and [routed G2 bucket rounds](../2026-10-24-g2-routed-rounds/README.md).
Their failures remain. This result does not reopen those exact recipes.

The next intervention should remove a preparation pass/synchronization boundary
or shorten ordinary G2 fill dependencies while accounting for extra bucket
initialization, merging and memory. Tweaking the final collection return cannot
explain most of this tail. No new algorithm is selected or adopted here; the
broader goal remains unmet 0/8.

## Verification, artifacts and cleanup

The observer borrows one request-owned 74-slot record across original joins,
with bounded stack formatting and no per-leaf traces, TLS attribution, new pool
or retained tables. All 135 core tests pass, including concurrent traced G2
results checked against an independent group oracle and nested interval checks.
Complete SDK latency includes clocks, record formatting and internal verification.

Local screening completes 1,844 SDK calls and 56 exports. Container compatibility adds
8 calls and 2 exports. AWS adds 1,844 calls and 56 exports, with 1,676 timed samples and 616 phase
records including primes. Every export independently verifies with Gnark; the
container and AWS exports also verify after transfer. Total: 3,696 calls and 114 exports.
The independent host audit recomputes all 16 target calibration/backing guards.

The source/build preparation, local calibration, container checks and cloud
bootstrap take about 13 minutes from experiment-directory creation to target
capture start; native build/test/link command time totals 149.36 seconds. Target
capture takes 39 seconds. No owned build, analysis or other measurement overlaps
capture. The subsequent analysis/transport/cleanup finish before the local report
commit. No successful SDK capture or proof verification is repeated.

Owned instance i-0e80a047a6f534b58 is terminated, its volume is absent, and the
temporary ECR tag and transfer object/bucket are deleted. No owned AWS resource
or live measurement remains from this run. Accepted source/SDK are unchanged.
Report commits are local; no GitHub push is authorized.

Canonical 55778:
`/home/ubuntu/src/zopro-perf-work/prep-g2-c7a181/transport/proof-sha256.json`,
SHA256 `34ac77076db3000bf79866ee031fec523725966de90aea16f1c01d31015ee9ca`.
Private roots: `/home/ubuntu/src/zopro-perf-work/prep-g2-detail181` and
`/home/ubuntu/src/zopro-perf-work/prep-g2-c7a181`.

[Target results](artifacts/target-review.json), [target substage analysis](artifacts/target-detail-analysis.json),
[local results](artifacts/local-review.json), [local substage analysis](artifacts/local-detail-analysis.json),
[proof transport verification](artifacts/transport-review.json), [completion audit](artifacts/completion-review.json),
[local protocol](artifacts/protocol.md), [target protocol](artifacts/target-protocol.md),
[raw target results](artifacts/target-results.tar.gz), [local sources and receipts](artifacts/local-records.tar.gz),
[target workflow and cleanup receipts](artifacts/target-records.tar.gz).
