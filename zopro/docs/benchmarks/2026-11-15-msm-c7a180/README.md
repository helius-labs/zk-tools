# C7a full-request MSM phase comparison

The fresh c7a.24xlarge capture confirms the primary 1x2 gap: accepted Zopro is
**11.258 ms at 32 cores**, while retained Aeglos Sorted is **8.098 ms at 96**.
Zopro's query stage gets slightly faster at 96, but solving, preparation and FFT
become slower enough to increase full-request latency. Sorted's fresh-build trace
locates its scaling gain mainly in bucket accumulation. There is **no new
optimization or production change** in this diagnostic.

All 220 exported proofs independently verify on AWS and again after transfer.
Calibration passes 62/64 checks; the two failures concern fresh-versus-retained
Aeglos Sorted at 96, not proof correctness or observer ON/OFF overhead. All ten
environmental checks pass. The instance, volume, temporary ECR tag and transfer
bucket are removed.

## Full verified requests

AMD EPYC 9R14, 96 physical cores, one thread per core, one NUMA node reported by
the guest. Compare CPU0–31 and CPU0–95. Nine fixed blocks cover each engine/control
and count, rotating/reversing order. Three independently written, byte-identical
backings per binary are balanced across blocks. Each main process performs three
primes then 51 sequential timed requests; 22 functional preflights precede them.
No owned build, analysis or helper overlaps capture.

The following rows use the exact retained reference binaries with tracing off.
Latency is the median of process medians. CPU is mean process CPU per request;
RSS is the largest whole-process peak across all nine processes.

| Reference, 1x2 | Cores | Latency, ms | CPU, ms/request | Peak RSS, MiB |
|---|---:|---:|---:|---:|
| Zopro accepted | 32 | 11.257745 | 243.43 | 111.20 |
| Zopro accepted | 96 | 12.986700 | 693.14 | 102.38 |
| Aeglos Sorted | 32 | 11.871786 | 328.77 | 416.10 |
| Aeglos Sorted | 96 | 8.097657 | 502.99 | 472.88 |
| Aeglos Windowed | 32 | 25.316315 | 574.70 | 412.91 |
| Aeglos Windowed | 96 | 54.980854 | 1784.68 | 758.38 |

Zopro worsens in all nine paired blocks; median 96/32 is 1.148357. Sorted improves
in all nine, ratio 0.684448. Windowed worsens in all nine, ratio 2.166129. Zopro
still wins the same-count 32-core comparison; Sorted wins at its better count.
These are observed counts, not a fresh search over every possible core count.
RSS includes loading and primes and is neither minimum RAM nor incremental scratch.

Against the supplied historical M5 CPU37ms and Metal24ms references, current
Zopro 1x2 is 3.29× and 2.13× respectively; the CPU10x target is3.7ms and remains
unmet. No other shape is rerun. The [accepted all-shape CPU/Metal comparison](../2026-11-12-bundle-adoption178/README.md)
remains the reference for the other seven shapes and merge54. Full goal: unmet0/8.

## Zopro's high-core penalty is before the queries

The following means include all459 timed ON requests per count. All applicable
Zopro calibration, backing and observer checks pass. The rows partition the
complete request; overlapping A/KH/G2 callbacks are deliberately excluded from
this additive table.

| Phase | 32 cores, ms | 96 cores, ms | 96 minus32, ms |
|---|---:|---:|---:|
| Solve | 1.900409 | 2.701639 | +0.801230 |
| Preparation | 0.372778 | 1.099037 | +0.726259 |
| FFT | 0.757154 | 1.173349 | +0.416194 |
| Complete query span | 7.069394 | 6.697745 | -0.371650 |
| Output | 0.005037 | 0.005394 | +0.000358 |
| Unassigned inside observer | 0.002432 | 0.002849 | +0.000417 |
| Request outside observer | 1.195627 | 1.295100 | +0.099473 |
| **Complete ON request** | **11.302832** | **12.975113** | **+1.672281** |

Solve, preparation and FFT each worsen in9/9 blocks; queries improve in9/9.
Together the three phases before queries add1.943684ms. The preparation boundary
covers allocator reclamation, dropping C, the folded public correction, in-place
canonical conversion and gathering A/B scalar vectors. It does not identify
which one of those operations causes the increase.

Holding the 96-core run's pre-query cost at the measured 32-core value would give
an arithmetic counterfactual of11.031430ms. This is not a measured optimization
or a claim that those costs can be changed independently. It also would not close
Sorted's approximately8ms gap. Fixing high-core setup alone is insufficient.

The G2 callback's finishing tail also differs substantially from the local SMT
capture. For each request, compute max(0,G2_end−max(A_end,KH_end)), then average
those actual tails; do not subtract separately averaged callback endpoints.

| Host/count | Mean G2 callback tail, ms | G2 finishes last |
|---|---:|---:|
| Local16 threads | 0.006292 | 136/459 |
| Local32 threads | 0.050638 | 216/459 |
| C7a32 cores | 0.442806 | 221/459 |
| C7a96 cores | 1.296600 | 394/459 |

This is callback completion order, not exclusive G2 arithmetic, worker utilization
or removable latency. Callbacks may help other tasks or wait. The local panel
also had one executable backing per binary. Those limits remain. The target
nevertheless exposes a much larger late dependency than the local capture did.

## Aeglos: accumulation scales; Windowed merge planning does not

These are fresh diagnostic ON means. Sorted's OFF/FRESH and ON/OFF controls pass
at both counts, but fresh-versus-retained calibration fails at96. Accordingly,
the Sorted substages describe the fresh implementation and are **not qualified
as exact attribution for the retained96 reference** in the first table.

| Instrumented stage | 32 cores, ms | 96 cores, ms |
|---|---:|---:|
| Sorted accumulation | 6.899617 | 2.982794 |
| Sorted complete MSM observer | 8.329389 | 4.085218 |
| Windowed accumulation | 10.362926 | 18.854340 |
| Windowed merge planning | 8.118641 | 26.942056 |
| Windowed merge execution | 1.209664 | 2.234258 |
| Windowed complete MSM observer | 21.328294 | 49.469434 |

Sorted accumulation improves in9/9 blocks and saves3.916824ms. It schedules1280
accumulation items at32 and2560 at96 and executes no merge pass in either context.
Windowed schedules652 and1944 accumulation items, with four and five merge rounds.
Its merge planning worsens in9/9 blocks, adding18.823415ms. The source runs these
merge-plan calls serially before dispatching each parallel merge pass.

Both modes use the same native pool, yet have opposite scaling. Their work
organization, arithmetic and key preparation differ; this is not a clean ablation
of tables alone. The data does not establish that large retained tables are
necessary, that copying the pool would fix Zopro, or that RAM capacity is the
sole cause. Scheduled items are not equal-cost operations or a measured ready queue.
All serial substages, per-job counts and residuals are retained in the artifacts.

## Calibration and interpretation

Fifteen of16 paired interval checks and47of48 per-backing median checks pass.
Both failures are Sorted FRESH/REF at96:

- Paired median0.983535, interval[0.923658,1.015759], outside the frozen[0.95,1.05]
  equivalence range on the lower side.
- Backing0 median0.949580, also below0.95. Its three block ratios are0.923658,
  0.949580 and1.012144.

The fresh build is faster in some blocks; this is an equivalence failure, not a
correctness failure or proof that it is slower. No block is discarded, gate
relaxed or capture repeated. All six ON/OFF interval comparisons pass, as do their
backing guards. Zopro OFF/REF, both Windowed build controls, and Sorted OFF/FRESH
also pass. Preserve each conclusion separately from the overall failed gate flag.

All ten environmental deltas are zero: cgroup CPU throttling, memory high/max/OOM
and swap events. The guest reports96 physical cores,384MiB L3 across12 instances,
and AVX-512 IFMA support. Cache size alone is not causal evidence. Aeglos major
faults remain unreported by its retained harness; no all-engine zero-major-fault
claim is made. This run does not measure memory-pressure, mobile/ARM/TEE,
huge-circuit or concurrent-proof behavior.

## Consequence and validation

The next work should address concrete passes or dependencies in G2 and the
pre-query path. Broad claims that the complete MSM gets slower at96, or that only
more workers are needed, are contradicted by these measurements. The prior
solver-search restriction, weighted query groups and fused-product overlap have
completed negative results; this capture does not reopen those exact recipes.
No next production implementation is selected or adopted by this report.

The original [protocol](artifacts/protocol.md), container compatibility results and
frozen package checks remain retained. Target capture completes once in344seconds:
10780SDK calls,220 fresh exports,10120 raw timed samples and2940 phase records.
Independent host verification checks every transferred export, stream/source
receipts, sample medians and ledger ancestry. A separate offline decomposition
checks additive phase deltas, actual callback tails and all48 backing guards.
No successful SDK capture or verifier step is replayed for analysis.

One owned instance is terminated; its encrypted volume is absent; the temporary
ECR tag and private transfer bucket are deleted. The downloaded archive matches
remote SHA256`11167fac3552cdae8f3f5586f4103e35642d3b7a18021f15979b22441575d174`.
No live benchmark or owned resource remains from this run. No GitHub push is authorized.

Current canonical ledger: **55664**, private path
`/home/ubuntu/src/zopro-perf-work/msm-c7a180/transport/proof-sha256.json`,
SHA256`5aa3b76fe67d3cc541636a7c94fa8eef7459b91a1cc7840d5330c901ddef0a4a`.
Accepted SDK remains`b646c7b8c1d8a26a0db45fb3f67c453dd616bd9f02e25c36192721ae02855abe`;
Aeglos source remains0290613e8eeca17a4ce88f6f2d726e8e06a5385d. Full goal unchanged.

[Complete target results](artifacts/target-review.json),
[phase analysis](artifacts/stage-analysis.json),
[host proof verification](artifacts/transport-review.json),
[raw target archive](artifacts/target-results.tar.gz),
[capture and cleanup summary](artifacts/completion-review.json),
[verification and cleanup receipts](artifacts/completion-records.tar.gz),
[prepared sources and receipts](artifacts/preparation-records.tar.gz).
Private root: `/home/ubuntu/src/zopro-perf-work/msm-c7a180`.
