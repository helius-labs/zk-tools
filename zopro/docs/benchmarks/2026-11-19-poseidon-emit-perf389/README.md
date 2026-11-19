# Hardware counters for compact Poseidon writes on Ryzen

The exact same clean SDK binary ran with address encoding enabled or disabled. Each cell uses three independent shuffled process blocks. Linux perf started disabled and used the SDK’s acknowledged control FIFO to count only the verified measurement loop after three warmups. Counters and cycle samples came from separate processes. Workers were pinned to their usual CPUs. Event counts are medians of per-process counts per proof.

| Shape | Mode | SDK ms | Millions of cycles | Millions of instructions | Cache misses | Branch misses | Context switches | Minor faults | Major faults |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | off | 7.344 | 676.2 | 1242.3 | 2719280 | 737301 | 531 | 41 | 0 |
| 1x2 | on | 7.300 | 671.4 | 1242.1 | 2638754 | 741034 | 534 | 38 | 0 |
| 16x2 | off | 45.486 | 3291.8 | 5449.8 | 15623244 | 2121744 | 665 | 1138 | 0 |
| 16x2 | on | 44.409 | 3239.5 | 5446.7 | 15514014 | 2119339 | 661 | 1119 | 0 |

## User-space cycle samples

Samples use the instruction-pointer symbol. Inlined code can contribute to its caller. Other includes all symbols outside the named categories.

| Shape | Mode | Samples | Poseidon functions | General solver functions | Crossbeam | Other |
|---|---|---:|---:|---:|---:|---:|
| 1x2 | off | 14554 | 0.5% | 0.7% | 7.0% | 91.8% |
| 1x2 | on | 14393 | 0.4% | 0.9% | 6.6% | 92.2% |
| 16x2 | off | 30601 | 4.0% | 4.5% | 2.1% | 89.3% |
| 16x2 | on | 30147 | 2.1% | 4.7% | 2.0% | 91.2% |

For 16×2, enabling address encoding reduced the median cycles per proof by 1.59% across the three counter blocks. Retired instructions changed by 0.06%. The share of samples in Poseidon functions fell from 4.0% to 2.1%. The direct nine-block latency comparison and its 16×2 memory measurements are in the [qualification report](../2026-11-19-poseidon-emit-release388/README.md).

For 1×2 with encoding disabled, the signed G1 bucket arithmetic function `stream_g1_impl` and the borrowed G2 bucket arithmetic function `g2_borrowed::apply` accounted for 23.2% and 18.3% of cycle samples respectively. Poseidon functions accounted for 0.5%. The two bucket functions merit a closer look at their sampled instructions on this machine. The samples describe time spent running on CPUs. They do not give the fraction of elapsed proof latency available as a speedup.

[All process counters and symbol counts](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)
