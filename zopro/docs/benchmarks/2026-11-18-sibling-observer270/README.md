# Distinguishing sibling work from off-CPU time on 1×2

A G2 worker can execute part of the fused K/H query before the first G2 bucket begins. We observed this directly in 2 of 17 Ryzen proofs. K/H work also ran on the G2 worker after the last bucket returned in 3 proofs. These observations establish that the proposed mechanism occurs on Ryzen. The frequency and cost in the 96-core C7a slow group still require direct measurement there.

| Phase and coordinating thread | Mean interval ms | Mean off-CPU ms | Mean running inside K/H feed ms | Requests containing a feed on that thread |
|---|---:|---:|---:|---:|
| G2 entry to first bucket, G2 thread | 0.360 | 0.001 | 0.257 | 2/17 |
| Last bucket return to G2 return, G2 thread | 0.834 | 0.234 | 0.331 | 3/17 |
| G2 return to canonical return, canonical thread | 0.143 | 0.010 | 0.087 | 1/17 |

The exact experimental SDK from experiment 257 ran on 32 pinned Ryzen hardware threads, with eight frontend workers. Each timed request produced 40 feed calls and 22 G2 bucket calls. The compiled window routine has two calls to the same feed function, one for K and one for H. Linux emitted 14,598 per-task switch records within the 17 proof intervals. Intersecting a feed with its worker’s scheduled intervals excludes time spent off-CPU from the feed column. Other on-CPU work in these intervals was outside the observed feed function.

With three primes and 17 timed requests per process, the uninstrumented process averaged 8.426 ms and the instrumented one averaged 8.192 ms. That single pair tests feasibility. The experiment does not estimate observer bias across multiple blocks or distinguish the AWS fast and slow classes. Both fresh exports passed Gnark. The plain process reached 4.39× the supplied M5 CPU speed and 2.85× Metal. [Process and request data](artifacts/analysis.json) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json)
