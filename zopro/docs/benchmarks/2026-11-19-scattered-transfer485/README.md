# Transfer point sums on scattered state leaves

The prepared groups came from eight earlier public 12×2 requests with adjacent state leaves. Each of these eight new transactions has twelve real coins at independently chosen 32-bit state positions, occupied sibling subtrees along each input path, and 128–184 prior random nullifier insertions. Roots and witnesses were regenerated through the protocol fixture code. A published public witness pins each independent proof verification.

The original and each held-out request used three matched randomized blocks, three primes and eleven timed calls per process on a 32-worker Ryzen. The release and same-build disabled paths provide controls for the B2-only and three-query options. The columns under routes count how many of four untimed instrumented proofs used the compact query. An ordinary query is selected whenever the number of exceptions exceeds its configured threshold.

| Input | Distinct predecessor indices | Compact routes A / K / B2 | Release / off / B2 / all / 482 ms | Paired B2 / all vs off | Peak MiB release / all |
|---|---:|---:|---:|---:|---:|
| Original | — | 4 / 4 / 4 | 31.164 / 31.739 / 31.117 / 30.333 / 30.637 | -1.10% / -3.31% | 550.6 / 654.5 |
| 137 | 11 | 0 / 0 / 0 | 48.039 / 47.501 / 48.277 / 48.210 / 52.998 | +1.50% / +1.49% | 968.4 / 1007.2 |
| 138 | 11 | 0 / 0 / 0 | 48.009 / 48.155 / 47.940 / 48.709 / 53.365 | +0.72% / +0.95% | 1024.2 / 1002.1 |
| 139 | 11 | 0 / 0 / 0 | 47.732 / 47.365 / 48.520 / 48.419 / 53.463 | +2.61% / +2.22% | 1004.3 / 983.3 |
| 140 | 11 | 0 / 0 / 0 | 47.709 / 47.016 / 48.299 / 48.680 / 52.537 | +3.36% / +1.86% | 959.0 / 977.8 |
| 141 | 11 | 0 / 0 / 0 | 48.228 / 48.101 / 48.506 / 48.614 / 53.018 | +1.71% / +1.07% | 974.1 / 991.5 |
| 142 | 11 | 0 / 0 / 0 | 48.463 / 49.044 / 47.601 / 47.931 / 53.213 | -0.73% / -1.97% | 1008.6 / 951.5 |
| 143 | 11 | 0 / 0 / 0 | 48.213 / 47.253 / 48.029 / 48.958 / 53.101 | +1.64% / +3.25% | 1036.9 / 999.0 |
| 144 | 12 | 0 / 0 / 0 | 48.174 / 47.555 / 48.377 / 49.440 / 52.924 | +2.65% / +5.73% | 1027.6 / 991.8 |

Against the same binary disabled, B2 alone changed held-out latency by +1.68% (1/8 faster) and all three queries changed it by +1.68% (1/8 faster). The earlier point-copy prototype (482) changed latency by +10.84% vs release. Its executable differs from the disabled control used for the residual prototypes. The nine route probes and 135 timed exports all verified with Gnark. [Results and attribution](artifacts/results.json) · [Fixture changes](artifacts/fixture.patch) · [Samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)
