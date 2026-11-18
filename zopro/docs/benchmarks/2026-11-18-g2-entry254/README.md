# G2 query entry and completion on C7a

In the accepted 1×2 proof at 96 cores, G2 entered **0.009 ms after A** and returned **2.188 ms after A**, using signed differences averaged over 453 warm requests. The difference in elapsed time inside the callbacks accounts for 2.179 ms. Moving G2's call site ahead of A by the observed entry delay alone offers little scope. Internal task availability and how G2 executes once entered warrant investigation.

| Mean interval or count | 32 physical cores | 96 physical cores |
|---|---:|---:|
| A entry after FFT return, ms | 0.004 | 0.004 |
| G2 entry after FFT return, ms | 0.008 | 0.013 |
| A callback elapsed, ms | 5.712 | 4.431 |
| G2 callback elapsed, ms | 7.006 | 6.609 |
| G2 exit minus A exit, ms | 1.298 | 2.188 |
| G2 returned later, requests | 409/453 | 429/453 |
| Canonical return after later callback, ms | 0.024 | 0.025 |

The A callback shortened by 1.282 ms as the worker count increased. G2 shortened by 0.397 ms. Both callbacks include waits, nested Rayon work and possible execution of other ready tasks. A later return alone cannot distinguish scalar decomposition, bucket work, a delayed internal task or competition with K/H. A follow-up should distinguish time until the first useful G2 bucket operation, time until the last independent bucket completes, and the dependent work from that point to query return.

This analysis reuses the six verified decoded trace streams from the matched C7a run. It introduces no new proof calls or timing probes. The reader checks each log against its successful process receipt, pairs entry and return by thread and request ordinal, checks enclosing bounds, and verifies the signed identity on every request. Differences from the previous positive-only exit gap reflect retaining negative values when A returned later. Per-block results and source are included. [Matched run and probe overhead](../2026-11-18-c7a-phase251/README.md) · [Block results](artifacts/blocks.json) · [Reader](artifacts/timing.rs)
