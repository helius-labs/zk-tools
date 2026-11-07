# Real FqBatch inversion arrivals

**No inversion broker or production change is selected.** The observer records **1,665,404 scalar-root inversion intervals across 40 complete SDK requests**, with zero buffer overflow and ten fresh exported proofs independently verified by Gnark. Actual invocations are much sparser than the all-roots-ready population in the earlier sharing benchmark. This is a diagnostic feasibility result, **not a speedup or a critical-path bound**.

For the final 1×2 request, the time-average number of open observed inversion intervals is **1.64 at 32 workers** and **1.03 at 16**. A descriptive greedy packing of mutually overlapping intervals into disjoint groups of eight covers **14.67% / 0.97%** of roots, with median common intersections of **100 / 30 ns**. These groups already include invocations whose ordinary inversions have started; they are not ready queues that a broker can use without waiting. The observation does not justify spending the next implementation cycle on coordination, particularly given the [previous optimistic ready-root comparison's failed gate](../2026-11-06-root-sharing-waves176/README.md). It does not prove every asynchronous sharing strategy impossible.

## Scope and method

Local AMD Ryzen 9 9950X, 16 physical cores/32 logical CPUs; Rust 1.98.1. An isolated accepted-source snapshot inserts observation around the **one scalar Fq inverse already used by FqBatch::inverse_nonzero**. The existing eight-lane shuffle/product tree, scalar root product, inverse function, field arithmetic and all MSM, FFT, solver and routing algorithms remain unchanged. Scalar inversions outside this bridge are not observed; this is not a census of every Fq inverse. All 176 originals recover through two declared inverse edits, and 174 remain byte-exact. Production source and accepted SDK 6d83 remain unchanged.

A Linux diagnostic module records one monotonic begin/end interval per observed inversion and thread-local sequence numbers. No field values, points, witnesses or keys are recorded. Thread registration saves Linux TID, Rust thread/name, optional Rayon index and allowed CPU mask once; per-event CPU and migration are not measured. The existing global Rayon workers pre-register buffers before proving. There is no broker, new pool, wait policy or production candidate.

Timestamp calls, TLS/Arc handling, an uncontended per-thread buffer lock and potential preemption perturb the observed execution. Some registration can occur on first use of an existing additional worker. Export occurs after each joined request and changes the gap between requests. These timestamps are **instrumented wall intervals**, not uninstrumented inverse service time, simultaneous CPU execution or proof latency. Outliers and all four epochs are retained. No observer-overhead correction is estimated.

The fixed ten contexts are all nine shapes at 32 workers, plus 1×2 at 16; taskset masks 0–31/0–15 and the current SDK environment are retained. Each process runs three prime requests followed by one final request, all internally verified. Every epoch is observed. All ten observation processes finish before the ten independent Gnark checks. No owned build, hashing, helper analysis or report work overlaps the observation runs. No previous performance/profile/model/capture is rerun.

## Observed final-request geometry

The fourth request follows the declared SDK boundary. The preceding prime epochs remain in the full analysis and raw archive. Counts are identical across all four epochs of each context. Greedy groups use eight distinct TIDs and intersecting intervals, consuming each interval at most once; the procedure is descriptive and has no optimality claim.

|Shape / workers|Observed inversions per request|Mean open intervals|Peak open intervals|Time with >=8 open, %|Greedy eight-root coverage, %|Median arrival spread / common intersection, ns|
|---|---:|---:|---:|---:|---:|---:|
|1x2 / 32|15710|1.6440|14|1.04552|14.6658|751 / 100|
|1x2 / 16|15710|1.0304|10|0.01809|0.9675|551 / 30|
|2x2 / 32|21673|1.4665|14|1.20590|16.2783|712 / 100|
|4x2 / 32|21802|1.1565|15|0.70571|13.1364|722 / 90|
|8x2 / 32|29920|1.0489|13|0.35018|8.1818|721 / 100|
|12x2 / 32|38252|0.8726|13|0.20996|5.8350|731 / 100|
|16x2 / 32|45643|1.0280|14|0.21195|5.2056|762 / 100|
|24x2 / 32|59841|0.7754|15|0.38338|9.9865|671 / 151|
|merge-24x1 / 32|58587|0.7571|13|0.15379|5.4210|742 / 110|
|merge-54x1 / 32|109213|0.6052|18|0.17151|5.9773|712 / 120|

Primary 32 greedy coverage across epochs 1–4 is 14.00%,16.04%,18.08%,14.67%; primary 16 is 0.66%,1.58%,0.56%,0.97%. Final primary 32 median observed interval is 821 ns versus 592 ns at 16 workers; these are instrumented intervals, not a matched performance comparison. Final primary time with >=8 open intervals is 1.0455%/0.0181%. Larger contexts retain their own values without favorable subset selection. Peaks can include preemption; a peak 18 is not 18 roots available to one coordinator at the same instant.

The mean is summed observed interval duration divided by the whole observed request span. It includes idle/other proof phases in its denominator. The overlap sweep handles ends before starts at tied times. Its time histogram sums to the epoch span, and its overlap-weighted mass exactly equals the sum of individual interval durations. An independently implemented sorted-start/sorted-end two-pointer sweep verifies every histogram and maximum. Sample greedy cohorts have unique TIDs and a nonempty common intersection. Neither an open interval nor its common intersection measures an available coordination budget.

## Memory and qualification

Each diagnostic event has two 64-bit timestamps. A registered thread reserves capacity for 65,536 records; the largest registry has 33 buffers, corresponding to 33 MiB of reserved event payload, plus vector/header/registry/serialization overhead. Actual populated data are smaller. This diagnostic limit makes overflow visible; it is not an architectural RAM cap, minimum-RAM measurement or a new prover allocation policy. Overflow remains zero in all 40 epochs. Event export buffers and log I/O are outside requests. No phone, low-RAM, memory-pressure, huge-circuit, swap, AWS or Aeglos execution is measured.

Three new focused observer tests pass (125 inherited core tests filtered) and two new interval tests pass against an independent integer-time oracle and exact tie/cohort cases. The generic whole-prover no_asm/no_ifma build passes with the diagnostic flag, as does a coherent eight-library native SDK and optimized native/fat-LTO/one-codegen-unit link. Field tests and existing core/prover suites are inherited without replay. All 25 native/model/capture workflow commands complete once; no failed attempt or recording reroll occurs.

Independent review checks pins, command argv/exits/stream hashes, 176 source recoveries, all 40 epoch boundaries, unique thread/sequence records, per-thread nonoverlap/counts, every weighted histogram, cohort examples, all ten fresh proof/public/image/VK bindings and ledger set differences. The canonical proof ledger advances **33,939→33,949**. One final 164-byte proof and 44-byte public input are independently checked per process; other requests exercise the SDK verification path.

## Decision and unchanged proof timings

Do not implement a runtime sharing broker from this evidence. The earlier ready-root arithmetic qualifications and failed combined latency gate remain intact. This screen supplies the previously missing real-workload arrival observation and closes the immediate broker-engineering direction. It does not exclude a distinct later algorithm backed by a credible dependency/coordination model. No group-size/worker/threshold reroll or new native timing is selected. The next hypothesis must remove dominant MSM work with complete arithmetic, exception/tail, dependency and live-memory accounting.

The [latest complete SDK comparison](../2026-11-07-fft-column-prep-sdk176/README.md) remains authoritative. Its rejected FFT candidate is not adopted. Accepted timings below are inherited from that fresh matched run, with supplied M5 references; these cross-machine ratios are not equal-hardware efficiency. The original verified CPU 10× goal is active and unmet, **0/8 shapes**.

|Shape|Accepted ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|8.013551|37 / 4.617×|24 / 2.995×|3.7|
|2x2|11.254041|59 / 5.243×|30 / 2.666×|5.9|
|4x2|15.963789|109 / 6.828×|48 / 3.007×|10.9|
|8x2|24.269224|189 / 7.788×|74 / 3.049×|18.9|
|12x2|33.884727|284 / 8.381×|101 / 2.981×|28.4|
|16x2|40.199590|335 / 8.333×|124 / 3.085×|33.5|
|24x2|65.690345|528 / 8.038×|176 / 2.679×|52.8|
|merge-24x1|65.555031|507 / 7.734×|175 / 2.670×|50.7|

Merge54 accepted 161.737128ms has no supplied M5 reference. Earlier primary results below 8 ms used the same accepted binary; no slower production build is adopted here.

[Analysis, all 40 epochs](artifacts/analysis.json), [protocol](artifacts/protocol.json), [review](artifacts/review.json), [decision](artifacts/decision.json), [source audit](artifacts/source-audit.json), [source, receipts and all raw event/metadata streams](artifacts/evidence.tar.gz), [checksums](SHA256SUMS).
