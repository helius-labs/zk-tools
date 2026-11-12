# Where Aeglos gains from additional C7a workers

The retained November 10 requests locate Aeglos Sorted's 32-to-96-worker gain
in MSM execution, not witness solving. For 1x2, MSM saves 4.437 ms while witness
solving costs an additional 0.336 ms. The full request saves 4.001 ms. MSM improves
in all 15 matching blocks for both 1x2 and 2x2; witness solving worsens in all 15.
This is a new decomposition of existing measurements, not a fresh AWS benchmark
or a Zopro optimization. Current accepted Zopro remains unchanged.

## Stage accounting

Use every retained unprofiled Aeglos request from the November 10 campaign at
32/96 workers: 15 blocks, 101 timed requests per primary process and 17 per 2x2
process, after three primes. The table averages process means over all blocks.
Equal request counts within each context make these also pooled request means.
The paired saving is the average of the 15 block differences, 32 minus 96.
Positive values indicate a saving. Mean accounting is deliberately separate from
the original report's median-of-process-medians latency estimand.

| Circuit | Stage | 32 workers, ms | 96 workers, ms | Saving, ms |
|---|---|---:|---:|---:|
| 1x2 | Witness | 1.288977 | 1.625243 | -0.336266 |
| 1x2 | Scalar preparation | 0.002525 | 0.002938 | -0.000413 |
| 1x2 | FFT | 0.893503 | 0.866543 | +0.026960 |
| 1x2 | MSM | 8.337837 | 3.900750 | +4.437088 |
| 1x2 | Assembly | 0.271099 | 0.301771 | -0.030672 |
| 1x2 | Unassigned engine time | 0.017799 | 0.020739 | -0.002940 |
| 1x2 | Outside engine | 1.122585 | 1.215193 | -0.092608 |
| 1x2 | **Full verified request** | **11.934325** | **7.933176** | **+4.001149** |
| 2x2 | Witness | 1.372456 | 1.782661 | -0.410204 |
| 2x2 | Scalar preparation | 0.003850 | 0.003878 | -0.000028 |
| 2x2 | FFT | 1.699293 | 1.288661 | +0.410632 |
| 2x2 | MSM | 14.087973 | 6.795358 | +7.292615 |
| 2x2 | Assembly | 0.285081 | 0.336429 | -0.051348 |
| 2x2 | Unassigned engine time | 0.033819 | 0.031749 | +0.002070 |
| 2x2 | Outside engine | 1.311247 | 1.529267 | -0.218020 |
| 2x2 | **Full verified request** | **18.793720** | **11.768003** | **+7.025717** |

Rows sum to full request time before rounding. Outside-engine time includes request
construction and verification/serialization; the separately reported `request_ms`
is already inside this residual and must not be added again. Engine residual is
computed per request, not by subtracting independent medians. The source's
`QuotientsAndMSMs` times the FFT call followed by MSM call construction and execution;
MSM is not arithmetic-only CPU service. All observed stage sums fit inside their
enclosing engine/request durations. These checks do not measure observer overhead.

The paired median 96/32 MSM ratios are 0.462870 (1x2) and 0.486119 (2x2), with
15/15 improving blocks each. Witness ratios are 1.252283 and 1.284809, with zero
improving blocks. Complete block differences, medians and means are retained in
[analysis.json](artifacts/analysis.json). No confidence interval or new causal
estimate is inferred from this retrospective stage decomposition.

The reader reproduces all original matched full-request medians for both engines
and counts. The original November 10 Zopro executable differs from today's
accepted build. Its stage timings must not be inferred from Aeglos's timers or
substituted with the later, perturbed Zopro task traces.

## A useful historical control

The October 25 campaign also measured Aeglos Sorted and Windowed using the same
pinned executable and worker pool, switching the MSM engine. It has three blocks,
not the 15 blocks above. Its full-request medians are:

| Circuit | Engine | 32 workers, ms | 96 workers, ms |
|---|---|---:|---:|
| 1x2 | Sorted | 11.606352 | 7.966389 |
| 1x2 | Windowed | 26.808818 | 55.113735 |
| 2x2 | Sorted | 18.053038 | 12.083592 |
| 2x2 | Windowed | 40.981986 | 61.261247 |

The shared pool is insufficient to guarantee good scaling. Both modes use the
shared phase runner in the pinned MSM source; their data organization, work
partitioning, arithmetic and preparation differ. In particular, Windowed chooses
its window width with the worker count and creates multiple chunks per window.
This is not a clean ablation of shifted tables alone and does not prove that
large retained tables are necessary for fast scaling. It does argue against
attributing Sorted's advantage solely to its pool or implementation language.
[Selected historical rows](artifacts/historical-sorted-windowed.json) preserve
all three process medians; the [original report](../2026-10-25-latest-c7a-aeglos/README.md)
contains the full matrix and limits.

## Consequence for the next experiment

Replace the broad question "why is Aeglos's scheduler better?" with:
**which MSM substage shortens at 96 workers in Sorted, and which work or dependency
prevents the current unshifted Zopro MSMs from doing the same?**

The existing pool instruction and whole-recording worker-distribution audits
already answer their narrower questions. Do not repeat them. The solver-only
search restriction is also closed. Nor does this result reopen the failed joint
query-window queue: that experiment changed preparation barriers, scratch lifetimes
and scheduling together and lost locally. A shared task list is not a demonstrated
solution.

The missing measurement is a matched coarse MSM timeline and work accounting,
with current accepted Zopro and pinned Aeglos Sorted/Windowed at 32 and 96 workers.
First retain full-request controls; separately measure the time from query inputs
being ready to all query results being ready. For Zopro distinguish scalar/group
preparation from query execution. For Aeglos distinguish recode/count/scatter,
accumulate, merge and weighted sums, preserving their existing order. Compare
completed work counts and available task counts per phase before selecting an
implementation. Sum sequential stages only; overlapping query spans require a
critical-path view and cannot be added as exclusive time.

Use coarse boundary records and bounded scratch, with no per-field/per-point or
per-steal tracing. Retain separate disabled/enabled controls and explicitly test
observer perturbation. The previous 96-worker task observer failed its overhead
check; its traces cannot supply current unperturbed phase evidence. Declare the
new capture matrix and validation before running it. No fresh AWS experiment is
launched by this report, and no new algorithm or production pool is selected.

An intervention must address the observed substage while keeping original key
storage and bounded workspace. Faster elapsed time accompanied by extra work,
CPU or memory must remain visible. The complete all-eight CPU10x goal is still
unmet (0/8). The [accepted local CPU/Metal results](../2026-11-12-bundle-adoption178/README.md)
and [latest C7a accepted measurements](../2026-11-12-phase-search-c7a178/README.md)
remain the current references.

## Validation and provenance

The offline Rust reader checks 3,540 Aeglos requests and 7,080 original full-request
samples across 120 processes. Every expected block, round index, cache hit, sample
count and original median matches; durations are finite/nonnegative and the
per-request and paired-mean decompositions conserve totals. One xtask command
passes with 70 pinned inputs in 0.1 seconds or less; no successful capture or
existing analysis is rerun. This is a same-agent analysis with checks against the
original parser output, not an independent audit of timing instrumentation.

The five stage-definition source files are retained from the previously pinned
Aeglos source; the exact binary remains the original reference. Pinned Aeglos
commit: `0290613e8eeca17a4ce88f6f2d726e8e06a5385d`. The original campaign's proof
verification is inherited, not repeated. There are zero new SDK requests, proof
exports or AWS resources. Canonical verified exports remain 55,209. AWS login
was valid when checked; that check was not a launch or new benchmark.

Private root: `/home/ubuntu/src/zopro-perf-work/scaling-divergence180`.
Reader source, prospective read protocol, input pins, numerical output and xtask
receipts are included under [artifacts](artifacts/). Complete source inputs,
original streams and executables stay at their existing private locations.
This report is local; publication requires user approval.
