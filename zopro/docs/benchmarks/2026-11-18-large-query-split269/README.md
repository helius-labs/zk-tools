# Splitting large unsigned MSMs on 24×2

Forcing two point partitions increased verified proof latency on the accepted Ryzen build in all six paired blocks. At 16 workers, two partitions recovered 5.49 ms on the experimental full-overlap schedule, which still took 83.91 ms versus 73.24 ms with the original schedule and default partitioning. The accepted build retained its default.

| Workers | Schedule | Default / two partitions ms | Paired wall change | Paired CPU change | Two faster | Peak default / two MiB | M5 CPU / Metal with two |
|---:|---|---:|---:|---:|---:|---:|---:|
| 16 | Accepted | 73.24 / 75.96 | +3.2% | +2.2% | 0/3 | 883 / 912 | 6.95× / 2.32× |
| 16 | Full witness overlap | 89.39 / 83.91 | -6.4% | +1.4% | 3/3 | 910 / 900 | 6.29× / 2.10× |
| 32 | Accepted | 65.32 / 68.59 | +4.8% | +6.4% | 0/3 | 1016 / 1043 | 7.70× / 2.57× |
| 32 | Full witness overlap | 80.47 / 85.93 | +12.2% | +3.3% | 1/3 | 1073 / 1079 | 6.14× / 2.05× |

Each cell uses three processes with distinct executable backings across blocks, seven timed proofs and three primes. Wall values are median process means. Paired changes are medians of within-block ratios. Peak RSS is the maximum process lifetime RSS. M5 ratios use the supplied 528 ms CPU and 176 ms Metal timings. All 24 fresh exported proofs verified, one per process across 168 timed requests. The 32-worker Ryzen configuration includes SMT.

The unsigned point-partition override affects every eligible MSM. The 554,226-point H call ordinarily has one partition. Other calls can already have two. No per-call timings were taken here, so the full-proof change cannot be attributed exclusively to H. This local test leaves the C7a behavior of this override unmeasured. [Block ratios](artifacts/analysis.json) · [Process samples](artifacts/processes.json.gz) · [Protocol](artifacts/protocol.md)
