# Sampling scalar equality before validating a prepared query

For each of the three compact queries, the prototype checks up to 32 evenly spaced groups. If more than a quarter of sampled members differ from their representative, the ordinary query starts immediately. Otherwise the same full validation and residual handling as in the [three-query experiment](../2026-11-19-multi-groups484/README.md) is used. The sample affects dispatch only. Every computed proof still incorporates every scalar.

One 32-worker Ryzen ran three randomized matched blocks of three primes and eleven timed proofs with the production release, the same binary without prepared groups, the prepared groups with full validation, and the sampled variant. Requests 57–64 use adjacent leaves and 137–144 use scattered leaves. Both sets comprise eight newly benchmarked public fixtures independent of the training requests.

| Request | Release / off / full / sampled ms | Sampled vs release / off / full | Peak MiB release / sampled |
|---|---:|---:|---:|
| Original | 31.508 / 31.643 / 30.120 / 29.958 | -4.50% / -4.37% / +0.30% | 561.6 / 624.3 |
| 57 | 31.800 / 31.520 / 30.123 / 29.697 | -6.61% / -6.40% / -2.05% | 556.1 / 612.0 |
| 58 | 31.240 / 31.127 / 30.011 / 29.987 | -4.01% / -3.74% / -0.25% | 572.9 / 591.7 |
| 59 | 31.631 / 31.783 / 30.084 / 29.678 | -6.18% / -5.85% / -1.70% | 569.7 / 617.8 |
| 60 | 32.362 / 31.291 / 30.292 / 29.659 | -7.32% / -3.56% / -2.62% | 568.0 / 615.4 |
| 61 | 31.283 / 31.710 / 30.442 / 29.454 | -5.86% / -7.11% / -0.59% | 538.7 / 635.0 |
| 62 | 31.616 / 31.539 / 29.811 / 29.839 | -5.41% / -5.23% / +0.19% | 551.7 / 658.0 |
| 63 | 31.972 / 31.331 / 30.166 / 29.812 | -6.32% / -4.85% / +0.28% | 548.0 / 594.4 |
| 64 | 31.474 / 31.193 / 29.838 / 29.786 | -5.36% / -4.52% / -0.88% | 557.0 / 657.8 |
| 137 | 46.742 / 47.839 / 49.408 / 48.514 | +3.79% / +1.08% / -3.04% | 943.4 / 956.8 |
| 138 | 47.703 / 47.556 / 48.651 / 47.285 | -0.88% / -0.57% / -2.08% | 953.5 / 992.9 |
| 139 | 48.957 / 47.863 / 48.423 / 47.886 | -0.38% / +0.05% / -2.36% | 975.4 / 1016.1 |
| 140 | 46.960 / 47.638 / 48.588 / 47.599 | +0.75% / -0.00% / -2.00% | 942.7 / 1032.8 |
| 141 | 47.417 / 47.212 / 48.637 / 47.742 | +0.35% / +1.12% / -1.02% | 1007.9 / 1020.0 |
| 142 | 47.635 / 48.278 / 48.389 / 49.096 | +2.15% / +2.41% / +0.85% | 945.7 / 1012.5 |
| 143 | 47.124 / 47.888 / 49.205 / 47.465 | -0.39% / -1.37% / -3.36% | 928.4 / 988.3 |
| 144 | 47.152 / 47.445 / 48.725 / 47.654 | +1.16% / +0.70% / -2.20% | 981.8 / 950.9 |

Across adjacent leaves the sampled candidate changed latency by -5.04% against disabled (8/8 faster). Across scattered leaves it changed latency by +0.37% against disabled and -2.14% against full validation. All 209 exported proofs passed independent Gnark verification, including the zero-budget fallback. [Results and route probes](artifacts/results.json) · [Samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [Incremental experimental change](artifacts/incremental.patch)
