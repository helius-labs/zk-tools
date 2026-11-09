# Complete-affine ring: complete-query comparison

**Rejected.** The fixed candidate makes primary G2 queries slower at every tested worker count and increases peak RSS. At 32 workers, ON measures **4.046769 ms**, versus **3.021513 ms** fresh OFF and **3.043465 ms** accepted-source REF. ON/OFF is **1.343000 [1.328302, 1.359073]**, with **0/15 wins**. These are complete-query timings, not proof times. Accepted full-proof 1×2 remains **7.857170 ms**.

Seventeen of 46 declared checks fail. All seven control calibrations, both unchanged-A stability checks and all diagnostic quality checks pass. This is a measured primary slowdown, not a failed-calibration or inconclusive-gain result. The exact schedule is closed without full-SDK follow-up or adoption. [Results](artifacts/analysis.json), [independent raw review](artifacts/review.json).

## Change and controls

The [native-qualified implementation](../2026-11-09-ring-affine-completion176/README.md) reuses one 2,731-bucket array across three ring unit phases and completes deferred additions through the existing affine batch at overflow and phase boundaries. It adds no Window field, heap buffer, input-point transform, retained key/point/scalar plan or pool. Batch200/queue199, radix128, nineteen positions, two partitions, leading indexed top, weighting and original exception handling stay fixed. It replaces the previous model's generic scalar spill with bounded affine completion; some modeled completions take 199 rounds.

REF is manifest-verified accepted source plus only the private query API. OFF is the qualified candidate source with the ring selector omitted. ON reuses the exact already-qualified core `f4085c8b…`; it is not rebuilt. All three share original field library `1200e635…`, dependencies, unchanged complete-query driver/perf support and native/fat-LTO/one-CGU consumer flags. The earlier API reference used a different field build and is excluded. Five missing core/consumer builds complete once; no native suite or field build is replayed. [Build review](artifacts/build-review.json), [identities](artifacts/identities.json).

Grouping is explicitly enabled. Before clocks, the scalar census predicts that retained primary inputs decline whole-scalar grouping and 2×2 inputs enter it. Actual route receipts are 316 for primary ON and zero for grouped 2×2/A. The grouping branch bypasses the new ring. A zero ring route is not a separate grouping counter; the source path and scalar census explain its expected behavior. No raw route is forced for coverage. [Routing census](artifacts/fixture-routing.json).

## Complete-query latency

Ryzen 9950X, 16 physical/32 logical CPUs. Fifteen fixed matched blocks rotate/reverse seven cases and three arms. Every process makes three checked primes, seventeen timed queries and one postcheck. Single-worker pools use CPU2; other workers i use CPUi, with 512 KiB stacks and coordinator affinity restricted to the case's CPUs. Timing includes scalar reset, grouping/admission, Phi2/recoding, allocation/drop, every window/top/merge/axis/weighting step and final affine normalization. Loading and pool construction are excluded.

Times are medians of process medians. Ratios pair blocks and need not equal quotients of displayed medians. Bounds are fixed ranks 4–12 of 15, nominal 96.484375% marginal median coverage under exchangeable independent blocks, without familywise coverage. All observations remain.

|Case|REF ms|OFF ms|ON ms|ON/OFF [bounds]|Wins/15|
|---|---:|---:|---:|---|---:|
|1x2-A-w32|1.187412|1.185538|1.179536|0.996387 [0.983599, 1.017940]|9|
|1x2-B-w1|37.338948|37.040203|42.743571|1.155033 [1.152013, 1.156618]|0|
|1x2-B-w16|3.419676|3.449662|4.470890|1.293194 [1.284195, 1.338136]|0|
|1x2-B-w32|3.043465|3.021513|4.046769|1.343000 [1.328302, 1.359073]|0|
|2x2-B-w1|39.686660|39.417432|39.882991|1.013157 [1.010553, 1.014370]|0|
|2x2-B-w16|3.920271|3.916423|4.203956|1.076846 [1.064181, 1.096712]|0|
|2x2-B-w32|3.474529|3.488857|3.817196|1.096719 [1.081262, 1.107680]|0|

Primary ON loses all fifteen blocks against both controls at all three worker counts. The equal-log two-population 32-worker aggregate ON/OFF is **1.212766 [1.203470, 1.221522]**, with 0/15 wins. Primary ON/REF is **1.332116 [1.307072, 1.347279]**, also 0/15.

Grouped 2×2 at 32 workers also has worse wall time, despite bypassing the ring: its ON/OFF ratio is **1.096719 [1.081262, 1.107680]**, with 0/15 wins. Its CPU service and PMU work are close between arms, as shown below. This trial does not isolate the source of that bypassed-context wall-time difference; it is not evidence that ring arithmetic ran there. Code layout or scheduling explanations remain unproven. The observations remain in the gate.

The frozen gate requires all OFF/REF calibration intervals inside [1/1.03,1.03], all fourteen ON/control nonregression upper bounds ≤1.02, primary32 and two-population32 aggregate gains against both controls with upper<1 and ≥10/15 wins, both unchanged-A intervals inside [1/1.03,1.03], all seven RSS guards and all twelve diagnostic quality checks. RSS requires ON≤max(REF,OFF)×1.05+1 MiB. No minimum gain percentage, exclusions, sample extension, reroll, threshold change or favorable subset promotion is used. Passing would select later full-SDK qualification, not adoption. [Prospective protocol](artifacts/protocol.md), [pre-clock review](artifacts/prospective-review.json).

All failed checks:

- `1x2-B-w1/ON/OFF/nonregression`
- `1x2-B-w1/ON/REF/nonregression`
- `1x2-B-w1/RSS`
- `1x2-B-w16/ON/OFF/nonregression`
- `1x2-B-w16/ON/REF/nonregression`
- `1x2-B-w16/RSS`
- `1x2-B-w32/ON/OFF/nonregression`
- `1x2-B-w32/ON/OFF/gain`
- `1x2-B-w32/ON/REF/nonregression`
- `1x2-B-w32/ON/REF/gain`
- `1x2-B-w32/RSS`
- `2x2-B-w16/ON/OFF/nonregression`
- `2x2-B-w16/ON/REF/nonregression`
- `2x2-B-w32/ON/OFF/nonregression`
- `2x2-B-w32/ON/REF/nonregression`
- `B32-two-shape/ON/OFF/gain`
- `B32-two-shape/ON/REF/gain`

## CPU service and memory observations

CPU service is summed user+system time per query, summarized by the median process mean. RSS is maximum lifetime rusage among latency processes, including input/setup/primes. Minor faults and switches are per-query differences inside the timed interval, then medians across processes. These values are not minimum RAM, swap/pressure qualification or a rigorous live working-set bound. The unchanged query driver exposes minor faults, not major faults.

|Case / arm|CPU ms/query|Peak RSS MiB|Minor faults/query|Voluntary switches/query|
|---|---:|---:|---:|---:|
|1x2-B-w1 / REF|37.370|15.50|0.118|2.000|
|1x2-B-w1 / OFF|37.048|15.50|0.118|2.000|
|1x2-B-w1 / ON|42.750|19.50|0.118|2.000|
|1x2-B-w16 / REF|45.155|27.75|41.647|117.647|
|1x2-B-w16 / OFF|44.830|27.50|42.765|121.588|
|1x2-B-w16 / ON|52.348|81.50|451.353|158.412|
|1x2-B-w32 / REF|70.831|32.00|51.765|203.882|
|1x2-B-w32 / OFF|69.695|32.00|51.765|196.941|
|1x2-B-w32 / ON|86.835|113.00|707.294|252.882|
|2x2-B-w1 / REF|39.688|27.25|0.118|2.000|
|2x2-B-w1 / OFF|39.423|27.25|0.118|2.000|
|2x2-B-w1 / ON|39.891|27.50|0.118|2.000|
|2x2-B-w16 / REF|48.219|43.75|27.294|108.706|
|2x2-B-w16 / OFF|48.089|43.50|33.235|107.706|
|2x2-B-w16 / ON|48.030|43.00|29.824|112.059|
|2x2-B-w32 / REF|75.985|53.50|36.176|183.118|
|2x2-B-w32 / OFF|75.241|53.25|31.588|184.176|
|2x2-B-w32 / ON|75.110|53.50|32.882|183.118|
|1x2-A-w32 / REF|24.674|23.75|70.824|76.941|
|1x2-A-w32 / OFF|24.800|24.00|68.529|77.000|
|1x2-A-w32 / ON|24.635|24.50|70.706|76.294|

Primary32 CPU service rises **69.695→86.835 ms/query**, RSS **32→113 MiB**, and minor faults **51.765→707.294/query** from OFF to ON. The candidate's ordinary bucket coordinates are three times smaller than the earlier rejected three-plane ring, but that comparison does not establish low memory use relative to accepted. A lower coordinate payload or absence of generic scalar spill does not imply lower whole-process RSS. These measurements do not isolate allocator retention or page-fault costs as the cause of the slowdown.

## Fixed counters and profiles

All 315 latency processes finish before six FIFO-gated grouped PMU panels and six separate cycles:u profiles. Each diagnostic makes 200 checked queries after three primes, then a postcheck. PMU and profiles exclude load/primes/postchecks. Every recording finishes before all decoding, hashing, analysis and publication work. No independent owned helper or second workload overlaps recording.

All PMU events have 100% scheduled coverage. Counts are single diagnostic panels, not repeated-trial confidence intervals. Generic cache misses do not establish cache level, DRAM traffic or a bandwidth bottleneck.

|Panel|Instructions M/query|Cycles M/query|Generic misses M/query|Branch misses/query|
|---|---:|---:|---:|---:|
|stat-1x2-B-w16-OFF|549.827622|222.216762|1.000928|271928.940|
|stat-1x2-B-w16-ON|562.525206|259.985365|2.407340|910265.205|
|stat-1x2-B-w32-OFF|567.666824|315.520221|1.217952|310257.040|
|stat-1x2-B-w32-ON|586.957825|394.046752|3.093168|970152.235|
|stat-2x2-B-w32-OFF|595.951982|345.227430|1.317720|318374.480|
|stat-2x2-B-w32-ON|596.635653|344.852609|1.189383|317424.105|

Primary32 uses more instructions (**567.667→586.958 million**) and cycles (**315.520→394.047 million**) despite the preceding model's lower generic addition count and vector-product proxy. Solo latency also regresses; the result cannot be described solely as poor scaling at high worker counts.

All six profiles qualify: **23,490 samples**, zero LOST/LOST_SAMPLES, exact 16,000,000-cycle period, allowed PID/TID/CPU ownership and matching executable MMAP/FORK metadata. Unknown share stays below 1.24%. Absolute coverage of the whole timed cycle budget is not validated.

|Profile|Samples|Unknown share|Qualified|
|---|---:|---:|---|
|cycles-1x2-B-w16-OFF|2759|0.9061%|true|
|cycles-1x2-B-w16-ON|3250|0.9538%|true|
|cycles-1x2-B-w32-OFF|3954|1.2393%|true|
|cycles-1x2-B-w32-ON|4932|0.9327%|true|
|cycles-2x2-B-w32-OFF|4297|0.9076%|true|
|cycles-2x2-B-w32-ON|4298|1.0935%|true|

Borrowed affine apply remains a large primary sample population. Candidate samples also cover the ring's admitted closure, recoding and axis grouping. Primary32 inverse samples decline 292→194 while total CPU work rises. Flat samples include inlined work and do not establish exclusive critical-path costs or an isolated cause. Grouped 2×2 PMU work remains close, consistent with its bypassed ring path. [Full symbols](artifacts/symbols.json).

## Verification and disposition

All **378 capture-workflow commands**, **348 caller processes** and **9,168 checked complete-query invocations**, including primes/postchecks, complete successfully once. Every result matches the retained independent Arkworks complete-MSM hash, and point inputs remain unchanged. Independent fixture qualification is inherited and hash-bound rather than replayed. These are MSM queries, not new Groth16 proofs.

The separate same-agent reader checks 413 frozen pins, all command/stream and managed-environment receipts, actual routes/worker placement, every raw latency ratio and all 46 decisions, twenty-one resource cells and profile metadata. It independently sorts/processes raw latency observations and reconstructs selection; the PMU/sample text-schema parsers are shared with the producer. The frozen analyzer and reader both pass on their first executions. No repair, successful stage repetition or recapture occurs.

The partial model correctly identified fewer generic additions and removed generic projective spills, but did not predict a complete CPU improvement. This implementation adds recoding/routing, unit-phase scans, inversion depth, axis work and memory ownership that the partial count proxy cannot price adequately. Actual instructions, CPU service, wall time and memory all worsen on primary. Close this fixed schedule without SDK follow-up; do not re-label its valid native correctness as failed or transfer the result to every possible ring/queue algorithm.

## Accepted CPU and Metal comparison, inherited

These local Ryzen 9950X full-proof times with 32 workers are unchanged accepted REF results from the completed singleton-elision SDK campaign. They are neither candidate query timings nor new M5 measurements.

|Shape|Accepted CPU ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|7.857170|37|24|4.709×|3.055×|3.7|
|2x2|11.052363|59|30|5.338×|2.714×|5.9|
|4x2|15.952018|109|48|6.833×|3.009×|10.9|
|8x2|24.080322|189|74|7.849×|3.073×|18.9|
|12x2|34.012417|284|101|8.350×|2.970×|28.4|
|16x2|40.128912|335|124|8.348×|3.090×|33.5|
|24x2|66.348981|528|176|7.958×|2.653×|52.8|
|merge-24x1|65.829291|507|175|7.702×|2.658×|50.7|

The all-eight 10× goal remains active and unmet, 0/8. Production and standard SDK remain unchanged. There are no new SDK requests, field/native candidate builds, proofs or independent Gnark exports; the unique verified proof ledger remains 40,154. This stage adds no AWS/Aeglos, phone, ARM, huge-circuit, pressure, swap, minimum-RAM or throughput result and owns no cloud resource.

[Continuation](artifacts/next.md), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS).
