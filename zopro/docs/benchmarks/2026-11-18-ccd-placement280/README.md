# Cache placement at 48 physical cores on C7a

A c7a.24xlarge ran the same accepted prover on 48 cores drawn from six shared L3 groups (compact) or twelve (spread). Two complementary masks per policy balanced CPU participation. Each row pairs the two policies over eight blocks on one instance.

| Shape | Compact ms | Spread ms | Paired wall change | Spread wins | Paired CPU change | 10× target ms | Blocks meeting target, compact / spread | Faster layout vs M5 CPU / Metal | Peak MiB compact / spread |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 10.636 | 11.100 | +4.12% | 1/8 | +4.83% | 3.7 | 0 / 0 | 3.48× / 2.26× | 92 / 91 |
| 2x2 | 14.173 | 14.939 | +5.89% | 0/8 | +4.46% | 5.9 | 0 / 0 | 4.16× / 2.12× | 223 / 223 |
| 4x2 | 17.925 | 18.438 | +3.05% | 0/8 | +2.82% | 10.9 | 0 / 0 | 6.08× / 2.68× | 284 / 283 |
| 8x2 | 24.648 | 25.686 | +4.53% | 0/8 | +5.30% | 18.9 | 0 / 0 | 7.67× / 3.00× | 462 / 469 |
| 12x2 | 32.572 | 33.635 | +2.72% | 0/8 | +3.41% | 28.4 | 0 / 0 | 8.72× / 3.10× | 674 / 658 |
| 16x2 | 36.795 | 38.030 | +3.19% | 0/8 | +3.61% | 33.5 | 0 / 0 | 9.10× / 3.37× | 778 / 786 |
| 24x2 | 53.557 | 54.204 | +1.15% | 2/8 | +1.48% | 52.8 | 1 / 0 | 9.86× / 3.29× | 1246 / 1217 |
| merge-24x1 | 55.998 | 57.136 | +1.67% | 1/8 | +2.04% | 50.7 | 0 / 0 | 9.05× / 3.13× | 1119 / 1112 |

The wall columns are medians of eight process means. Each mean covers 21 sequential timed proofs after three primes. Percentage changes are medians of paired spread/compact ratios. CPU includes user and system time. RSS is the largest lifetime peak across that shape and policy, including preflights. The target counts refer to eight blocks per layout. The M5 comparisons use supplied reference times and the quicker median for each shape.

Compact placement won 60 of 64 pairs. It met the prespecified placement screen on 1×2 through 16×2. Its smaller advantages on 24×2 and merge24 fell below the 2% cutoff. The 24×2 compact median of 53.557 ms was above the 52.8 ms target, previously met on two independent hosts. No additional shape reached its target.

The placement screen required at least a 2% paired median improvement and seven wins. A new absolute target requires independent confirmation. This experiment measured placement at a fixed worker count. It collected no bandwidth or hardware-cache counters.

The runner checked the CPU masks for the main thread and every worker before and after timing. All 160 exports passed Gnark verification on AWS and again after retrieval. The instance terminated, its volume query was empty and the temporary bucket was removed. See [per-process timings](artifacts/processes.json.gz), [observed topology](artifacts/topology.json), and the [protocol](artifacts/protocol.md).
