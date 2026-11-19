# Running G2 on the caller branch of the 1×2 curve join

The previous timing test observed a faster 1×2 proof after delaying G1 by 900 microseconds. This experiment tries to advance the other branches by changing which branch of the outer Rayon join the caller executes. The G2-first arms compute all curve results anew for every witness. They change no proving-key data. The diagnostic delay recomputes the entire proof as well and isolates the effect of changing start time. Its four medians were 8.004 / 7.961 / 7.961 / 7.944 ms. Among the G2-first arms, g2-first-skip-both had the lowest mean ratio to the control across the four inputs. Its medians were 10.112 / 10.207 / 9.917 / 9.974 ms.

All measurements used one c7a.24xlarge. The main pool used 96 physical cores and the frontend used 16. The control, G2 bypass, G2-first and delay arms share a binary. The release control uses the pinned SDK from the catalog. Aeglos recorded a prepared-key cache hit for every timed proof. Each process primed three requests and timed fifteen. An independent verifier checked all 222 exports after retrieval. The host was terminated and temporary resources were removed. The catalog release was 4.57× faster than the supplied M5 CPU reference and 2.97× faster than the supplied Metal reference.

| Input | Configuration | Median ms | Paired vs same-binary control | Paired vs Aeglos | Peak MiB |
|---|---|---:|---:|---:|---:|
| catalog | release | 8.091 | -0.48% (5/6) | -3.35% (6/6) | 43.7 |
| catalog | off | 8.125 | +0.00% (0/6) | -2.83% (6/6) | 44.6 |
| catalog | skip-b2 | 7.970 | -1.97% (6/6) | -4.56% (6/6) | 43.7 |
| catalog | aeglos | 8.370 | +2.91% (0/6) | +0.00% (0/6) | 456.8 |
| catalog | g2-first | 8.025 | -1.23% (6/6) | -4.03% (6/6) | 46.8 |
| catalog | g2-first-skip-a | 10.485 | +28.82% (0/6) | +25.74% (0/6) | 44.6 |
| catalog | g2-first-skip-both | 10.112 | +24.31% (0/6) | +20.79% (0/6) | 43.0 |
| catalog | delay900 | 8.004 | -1.63% (6/6) | -4.08% (6/6) | 42.2 |
| heldout | release | 11.155 | +0.28% (2/6) | +34.55% (0/6) | 42.3 |
| heldout | off | 11.170 | +0.00% (0/6) | +35.03% (0/6) | 42.3 |
| heldout | skip-b2 | 10.329 | -7.25% (6/6) | +25.46% (0/6) | 40.1 |
| heldout | aeglos | 8.316 | -25.94% (6/6) | +0.00% (0/6) | 456.9 |
| heldout | g2-first | 11.149 | -0.05% (3/6) | +34.55% (0/6) | 42.9 |
| heldout | g2-first-skip-a | 10.319 | -7.01% (6/6) | +25.54% (0/6) | 44.5 |
| heldout | g2-first-skip-both | 10.207 | -7.65% (6/6) | +23.60% (0/6) | 40.8 |
| heldout | delay900 | 7.961 | -28.59% (6/6) | -4.35% (6/6) | 41.6 |
| v25 | release | 11.215 | -0.24% (4/6) | +33.13% (0/6) | 40.8 |
| v25 | off | 11.264 | +0.00% (0/6) | +33.46% (0/6) | 44.6 |
| v25 | skip-b2 | 10.571 | -6.10% (6/6) | +25.32% (0/6) | 41.5 |
| v25 | aeglos | 8.440 | -25.07% (6/6) | +0.00% (0/6) | 460.8 |
| v25 | g2-first | 11.232 | -0.30% (4/6) | +33.06% (0/6) | 41.6 |
| v25 | g2-first-skip-a | 10.225 | -8.70% (6/6) | +22.72% (0/6) | 40.7 |
| v25 | g2-first-skip-both | 9.917 | -11.10% (6/6) | +16.64% (0/6) | 41.5 |
| v25 | delay900 | 7.961 | -29.08% (6/6) | -5.91% (6/6) | 44.5 |
| v26 | release | 11.199 | -0.10% (4/6) | +35.54% (0/6) | 41.4 |
| v26 | off | 11.235 | +0.00% (0/6) | +35.30% (0/6) | 42.3 |
| v26 | skip-b2 | 10.488 | -6.81% (6/6) | +25.93% (0/6) | 41.6 |
| v26 | aeglos | 8.298 | -26.08% (6/6) | +0.00% (0/6) | 460.4 |
| v26 | g2-first | 11.011 | -1.91% (4/6) | +33.54% (0/6) | 43.7 |
| v26 | g2-first-skip-a | 10.348 | -7.88% (6/6) | +24.76% (0/6) | 42.3 |
| v26 | g2-first-skip-both | 9.974 | -10.88% (6/6) | +20.89% (0/6) | 40.0 |
| v26 | delay900 | 7.944 | -29.30% (6/6) | -3.54% (6/6) | 43.8 |

Each traced arm has three blocks on the original and second transactions. The time origin is SDK entry. K/H starts at the earliest feed entry and ends at the last feed return. Bucket weighting and combination may continue after the last feed. The profile effect compares each trace arm with the uninstrumented runs from matching blocks. The probes recorded 450 timed SDK requests and reported zero lost records.

| Input | Configuration | Profile effect | Raw G1 start / G1 end / G2 start / G2 end / first K/H feed / last K/H feed / canonical return ms |
|---|---|---:|---:|
| catalog | off | +3.1% | 4.202 / 6.509 / 3.037 / 7.473 / 3.678 / 6.772 / 7.506 |
| catalog | g2-first | +4.1% | 4.384 / 6.293 / 3.078 / 7.395 / 3.725 / 6.847 / 7.459 |
| catalog | g2-first-skip-a | +5.4% | 3.040 / 7.218 / 3.029 / 8.439 / 6.576 / 9.808 / 10.391 |
| catalog | g2-first-skip-both | +2.6% | 3.025 / 5.695 / 3.015 / 8.749 / 5.739 / 8.949 / 9.689 |
| catalog | delay900 | +3.0% | 4.060 / 6.045 / 3.088 / 7.209 / 3.763 / 6.909 / 7.461 |
| heldout | off | +4.6% | 3.271 / 7.505 / 3.010 / 7.778 / 7.271 / 10.337 / 10.912 |
| heldout | g2-first | +1.7% | 3.487 / 7.196 / 3.009 / 7.751 / 7.025 / 10.058 / 10.521 |
| heldout | g2-first-skip-a | +6.1% | 3.018 / 7.451 / 3.008 / 8.631 / 6.537 / 9.575 / 10.124 |
| heldout | g2-first-skip-both | +6.8% | 3.056 / 6.023 / 3.045 / 8.341 / 6.277 / 9.348 / 9.841 |
| heldout | delay900 | +2.5% | 4.039 / 5.878 / 3.079 / 7.264 / 3.750 / 6.853 / 7.309 |

[Proof samples and traces](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Scheduling patch](artifacts/schedule.patch) · [Independent verification](artifacts/verification.json) · [Previous start-time experiment](../2026-11-19-solver-order728/README.md)
