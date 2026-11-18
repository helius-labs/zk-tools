# Partitioning the last H window on 16 cores

Splitting the final H window into four or eight point ranges missed the prespecified 2% paired full-proof latency threshold on merge24. The C7a follow-up therefore omits both variants.

Five randomized blocks per shape on 16 pinned local cores. Each fresh process made three warm-up proofs and 11 timed proofs. Latency shows the median of the five process means. Paired change uses matching blocks against the rebuilt binary with partitioning disabled. RSS is the largest process peak.

| Shape | Partitions | Latency (ms) | Paired change | Wins / 5 | RSS (MiB) |
|---|---|---:|---:|---:|---:|
| merge-24x1 | reference | 70.564 | +0.77% | 1 | 887 |
| merge-24x1 | off | 70.505 | +0.00% | — | 902 |
| merge-24x1 | 4 | 70.238 | -0.58% | 3 | 884 |
| merge-24x1 | 8 | 70.436 | -0.13% | 4 | 884 |
| 24x2 | reference | 72.829 | -1.57% | 3 | 949 |
| 24x2 | off | 73.096 | +0.00% | — | 983 |
| 24x2 | 4 | 72.619 | -0.48% | 4 | 959 |
| 24x2 | 8 | 71.704 | -0.69% | 4 | 943 |
| 16x2 | reference | 45.628 | -0.60% | 4 | 597 |
| 16x2 | off | 45.580 | +0.00% | — | 592 |
| 16x2 | 4 | 45.844 | -0.51% | 3 | 587 |
| 16x2 | 8 | 45.585 | -0.04% | 3 | 586 |

The reference uses the retained experiment 306 binary. Uprobes observed one invocation of the partitioned reduction per proof in both enabled arms and zero with partitioning disabled or with zero optional memory. All 60 timed processes and four route checks exported independently verified proofs. The patch is relative to experiment 306.
