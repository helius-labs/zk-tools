# Sharing G2 inputs across eight complete windows on Ryzen

On the 1×2 query, grouping eight windows under one task slowed the window fill at both 16 and 32 workers. The typed broadcast kernel also lost to ordinary bucket updates within the same grouped traversal, even on a single worker. The existing window-parallel implementation remains the faster measured choice.

Each row gives the median of five fresh process means on a Ryzen 9950X. Each process timed seven fills using all 30,089 query entries. The two-part split used width 12 and the four-part split used width 11. Each fill ends with eight bucket vectors per partition. Cross-partition reduction, weighting and the rest of the proof fall outside the timer.

| Workers | Parts | Existing ms | Grouped ms | Broadcast ms | Broadcast vs existing, paired | Faster blocks / 5 | Broadcast vs grouped, paired |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 2 | 18.565 | 19.110 | 27.773 | +49.7% | 0 | +45.3% |
| 1 | 4 | 20.465 | 19.418 | 28.265 | +38.2% | 0 | +45.6% |
| 16 | 2 | 1.591 | 9.812 | 14.339 | +795.9% | 0 | +45.4% |
| 16 | 4 | 1.752 | 5.158 | 7.593 | +324.7% | 0 | +45.6% |
| 32 | 2 | 2.001 | 10.057 | 14.309 | +612.8% | 0 | +41.5% |
| 32 | 4 | 1.675 | 5.147 | 7.450 | +344.8% | 0 | +44.4% |

The four-part reference can schedule 32 independent fills. Grouping the windows leaves four independently scheduled partitions. At one worker and four parts, the grouped traversal with ordinary updates took 19.418 ms. Enabling packet admission and broadcast raised that to 28.265 ms. Lower concurrency and the cost inside each grouped task both deserve weight when interpreting the 32-worker result.

Each warmup and timed output matched the existing implementation bucket for bucket. A separate oracle exercised identities, signed digits, repeated points and cancellation against complete projective sums. The first harness invocation omitted the native runtime configuration and stopped before timing. The 90 successful processes all used the same native configuration.

[Window implementation and oracle](artifacts/shared_input.rs) · [Broadcast kernel](../2026-11-19-window-broadcast-kernel395/artifacts/kernel.rs) · [Protocol](artifacts/protocol.md) · [Samples](artifacts/processes.json.gz) · [Validation](artifacts/validation.json)
