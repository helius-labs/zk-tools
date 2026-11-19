# Completing initial K/H windows before publishing G1 on 96 C7a cores

The previous C7a experiment measured a large gain from delaying the G1 query. This trial spends the start interval doing K/H curve work used in the current proof. The seeded arms compute one, two or four original K/H windows before publishing G1 A and the remaining K/H windows. They bypass the canonical A grouping check. The combined arm also bypasses the exclusive G2 grouping check. The delayed arm sleeps for 900 microseconds before raw G1, and the ordinary same-binary control uses its usual launch order.

The strongest seeded arm by mean of the four relative medians was seed1-skip-a. Its medians were 10.813 / 10.746 / 10.746 / 10.690 ms for catalog, heldout, v25 and v26. It had a lower median than Aeglos on 0/4 inputs and won 0/20 paired blocks. On catalog it measured 3.42× the supplied M5 CPU speed and 2.22× the supplied M5 Metal speed.

All arms ran on one c7a.24xlarge with 96 physical cores. Zopro had a 16-core frontend subset. Prepared Aeglos reported cache hits across the timed proofs. Five blocks per input used three warmups and fifteen timed requests in each process. The independent verifier checked all 190 exports. The instance was terminated, and the temporary bucket and volumes were removed.

| Input | Configuration | Median ms | Paired vs same-binary control | Paired vs Aeglos | Peak MiB |
|---|---|---:|---:|---:|---:|
| catalog | release | 8.052 | -0.28% (3/5) | -3.14% (5/5) | 45.2 |
| catalog | off | 8.048 | +0.00% (0/5) | -2.32% (5/5) | 46.1 |
| catalog | aeglos | 8.273 | +2.38% (0/5) | +0.00% (0/5) | 458.6 |
| catalog | seed1-skip-a | 10.813 | +34.36% (0/5) | +30.86% (0/5) | 43.1 |
| catalog | seed2-skip-a | 10.901 | +35.54% (0/5) | +31.76% (0/5) | 42.3 |
| catalog | seed4-skip-a | 10.994 | +36.61% (0/5) | +32.98% (0/5) | 45.2 |
| catalog | seed2-skip-both | 10.985 | +37.01% (0/5) | +32.83% (0/5) | 38.5 |
| catalog | delay900 | 7.989 | -1.00% (4/5) | -3.35% (4/5) | 42.2 |
| heldout | release | 11.147 | +0.16% (1/5) | +36.39% (0/5) | 43.0 |
| heldout | off | 11.154 | +0.00% (0/5) | +35.94% (0/5) | 40.7 |
| heldout | aeglos | 8.208 | -26.44% (5/5) | +0.00% (0/5) | 461.1 |
| heldout | seed1-skip-a | 10.746 | -3.89% (5/5) | +30.34% (0/5) | 44.5 |
| heldout | seed2-skip-a | 10.768 | -3.69% (5/5) | +30.89% (0/5) | 46.8 |
| heldout | seed4-skip-a | 10.793 | -3.28% (5/5) | +31.49% (0/5) | 41.5 |
| heldout | seed2-skip-both | 10.898 | -2.08% (5/5) | +30.65% (0/5) | 45.2 |
| heldout | delay900 | 7.883 | -29.05% (5/5) | -3.55% (5/5) | 43.8 |
| v25 | release | 11.126 | +0.24% (2/5) | +36.59% (0/5) | 41.5 |
| v25 | off | 11.124 | +0.00% (0/5) | +36.36% (0/5) | 44.5 |
| v25 | aeglos | 8.167 | -26.67% (5/5) | +0.00% (0/5) | 459.5 |
| v25 | seed1-skip-a | 10.746 | -3.17% (5/5) | +30.99% (0/5) | 42.9 |
| v25 | seed2-skip-a | 10.692 | -3.62% (5/5) | +31.67% (0/5) | 42.3 |
| v25 | seed4-skip-a | 10.746 | -3.18% (5/5) | +32.11% (0/5) | 42.4 |
| v25 | seed2-skip-both | 10.803 | -3.04% (5/5) | +32.65% (0/5) | 43.0 |
| v25 | delay900 | 7.862 | -29.20% (5/5) | -3.56% (5/5) | 43.8 |
| v26 | release | 11.115 | +0.29% (1/5) | +35.17% (0/5) | 43.7 |
| v26 | off | 11.082 | +0.00% (0/5) | +35.39% (0/5) | 41.5 |
| v26 | aeglos | 8.208 | -26.14% (5/5) | +0.00% (0/5) | 459.8 |
| v26 | seed1-skip-a | 10.690 | -3.55% (5/5) | +30.15% (0/5) | 43.8 |
| v26 | seed2-skip-a | 10.734 | -2.53% (4/5) | +30.78% (0/5) | 43.7 |
| v26 | seed4-skip-a | 10.752 | -2.61% (5/5) | +31.85% (0/5) | 42.3 |
| v26 | seed2-skip-both | 10.794 | -2.52% (5/5) | +31.56% (0/5) | 38.5 |
| v26 | delay900 | 7.862 | -28.75% (5/5) | -3.31% (5/5) | 40.8 |

Three traced blocks per input and configuration on the original and second transactions covered 450 timed requests. All recordings reported zero lost events. The K/H columns bound the feed calls. Window weighting and recombination can continue afterward. The time origin is SDK entry, and the latency comparison above uses uninstrumented processes.

| Input | Configuration | Trace effect | Raw G1 start / G1 return / G2 return / first K/H feed / last K/H feed / canonical return ms |
|---|---|---:|---:|
| catalog | off | +2.9% | 4.180 / 6.494 / 7.383 / 3.671 / 6.844 / 7.478 |
| catalog | seed1-skip-a | +3.5% | 6.589 / 8.381 / 9.733 / 3.620 / 9.879 / 10.403 |
| catalog | seed2-skip-a | +4.3% | 6.775 / 8.642 / 10.427 / 3.654 / 10.042 / 10.608 |
| catalog | seed2-skip-both | +4.1% | 6.758 / 8.694 / 6.845 / 3.666 / 10.096 / 10.592 |
| catalog | delay900 | +2.4% | 3.989 / 5.861 / 7.124 / 3.684 / 6.885 / 7.337 |
| heldout | off | +4.2% | 3.247 / 7.391 / 7.971 / 7.287 / 10.318 / 10.824 |
| heldout | seed1-skip-a | +4.0% | 6.714 / 8.364 / 9.482 / 3.651 / 9.967 / 10.379 |
| heldout | seed2-skip-a | +3.0% | 6.717 / 8.353 / 9.700 / 3.683 / 9.880 / 10.313 |
| heldout | seed2-skip-both | +3.7% | 6.909 / 8.613 / 7.032 / 3.843 / 10.267 / 10.671 |
| heldout | delay900 | +2.2% | 4.014 / 5.777 / 7.079 / 3.742 / 6.857 / 7.229 |

[Samples and intervals](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Implementation](artifacts/schedule.patch) · [Verification](artifacts/verification.json) · [Ryzen timings](../2026-11-19-kh-seed733/README.md)
