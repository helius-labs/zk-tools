# G2 partition timing on C7a

On 96 physical cores, all 22 G2 partition callbacks overlapped on 22 distinct workers in every one of 453 traced 1×2 proofs. The spread from the first to the last partition start averaged **0.033 ms**, down from **1.053 ms** at 32 cores. Yet the partition that finished later in the last window took **3.355 ms** at 96 cores versus **2.781 ms** at 32. Earlier dispatch helped G2, but slower partition execution consumed much of that saving.

## Timings inside the accepted G2 query

The table reports elapsed callback intervals. Worker counts are physical cores on the same c7a.24xlarge. Rows overlap. The last window is selected by its actual return time on each proof. Its later-returning partition is selected the same way.

| Mean per verified request | 32 cores | 96 cores |
|---|---:|---:|
| Complete G2 callback, ms | 6.949 | 6.634 |
| G2 entry until raw G2 entry, ms | 1.512 | 1.751 |
| G2 entry until first partition starts, ms | 2.647 | 2.550 |
| Spread of 22 partition starts, ms | 1.053 | 0.033 |
| Mean partition duration, ms | 2.531 | 2.887 |
| Later partition of the last window, ms | 2.781 | 3.355 |
| Weighting of that window, ms | 0.221 | 0.268 |
| Last global partition return until G2 return, ms | 0.504 | 0.629 |
| Maximum open partition callbacks | 21.78 | 22.00 |

The narrow top window finished last in 26 of 453 requests at 32 cores and zero at 96. Starting the existing partition jobs earlier offers little leverage at 96. Two opportunities remain measurable. The first 1.75 ms includes the grouping decision on this primary input. An exact distinct-scalar lower bound could run alongside the FFT and let the proof skip a grouping pass that would decline. The larger opportunity is the cost of filling the ordinary windows. Pertinent questions are how much each partition does and how its service time changes when the other proof queries run alongside it. Callback intervals alone cannot identify memory bandwidth, cache contention or exclusive field-arithmetic time.

## End-to-end comparison

| Engine | 32-core ms | 96-core ms | 96-core CPU ms/request | 96-core maximum RSS MiB |
|---|---:|---:|---:|---:|
| Accepted Zopro | 10.962 | 12.456 | 691.8 | 114.3 |
| Retained Aeglos | 11.849 | 7.804 | 511.2 | 479.1 |

These plain measurements average three 151-request process means. At 96 cores Zopro is 2.97× the supplied M5 CPU speed and 1.93× the M5 Metal speed on 1×2. The 10× CPU target is 3.7 ms. The seven other M5 shapes are outside this trace run. [Nine-shape reference panel](../2026-11-18-canonical-width255/README.md)

New sparse probes on the exact accepted SDK record function boundaries and window/partition identities. The reader pairs calls with per-thread stacks, reconstructs all 11 windows, verifies their 22 parts and 11 weighting calls within the request, and checks affinity and loss. All 94 events per traced request were present. Median traced/plain ratios were **1.0103** and **1.0168** at 32 and 96. Both met the predefined factor of 1.05. Overall observer overhead does not guarantee equal perturbation within each substage.

A local smoke verified one export. The cloud run made 2,796 SDK calls and verified all 24 exports. They passed host Gnark verification after retrieval. The first host verifier invocation encountered a concurrent local build lock and was retried once the build finished. The prover captures were not repeated. The unique ledger reached 64,829. The instance, tagged volume and temporary bucket were removed. [Per-block results](artifacts/host-summary.json) · [Aggregate timings](artifacts/phase-means.json) · [Trace reader](artifacts/trace.rs) · [Probe targets](artifacts/targets.tsv)
