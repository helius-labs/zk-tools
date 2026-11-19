# Grouping, scheduling and curve-query latency on 96 C7a cores

All arms ran on the same c7a.24xlarge. Zopro used 96 main workers and CPU IDs 0–15 for the frontend. Prepared Aeglos could use all 96 cores and reported cache hits on every timed request. Each cell is a median of process means, with three warmups and fifteen timed verified requests per process. All 244 exported proofs passed the independent verifier after retrieval. The instance terminated, with zero active tagged benchmark instances, zero tagged volumes and the temporary bucket removed.

## Confirming the exclusive G2 grouping bypass

Five fresh blocks cover the original and second transactions and two transactions outside the discovery set. The off and bypass arms use the same binary. The bypass omits the exclusive G2 grouping check and keeps the conservative enabled workspace forecast. For the original transaction, the bypass is 4.63× against the supplied 37 ms M5 CPU time and 3.01× against the supplied 24 ms M5 Metal time. The ordinary release supplies a separate control.

| Input | Release / off / bypass / Aeglos ms | Paired bypass vs off | Paired bypass vs Aeglos | Peak MiB bypass / Aeglos |
|---|---:|---:|---:|---:|
| catalog | 8.117 / 8.089 / 7.984 / 8.264 | -1.48% (5/5) | -4.35% (5/5) | 42.4 / 460.8 |
| heldout | 11.180 / 11.116 / 10.458 / 8.248 | -5.81% (5/5) | +27.71% (0/5) | 41.4 / 458.5 |
| v25 | 11.183 / 11.205 / 10.364 / 8.469 | -7.79% (5/5) | +22.96% (0/5) | 41.5 / 458.9 |
| v26 | 11.090 / 11.112 / 10.368 / 8.213 | -6.54% (5/5) | +25.00% (0/5) | 43.8 / 459.0 |

## Changing when G1 and K/H start

Four plain blocks compare the original scheduling with three ways of bypassing G1 grouping. Scan reads one low limb per pair of adjacent scalars, or approximately one load per 64-byte cache line, and delay sleeps for 900 microseconds before raw G1. The K/H-first arm gives K/H the caller branch of its Rayon join with G1. Its combined arm also bypasses exclusive G2 grouping. The scan and delay are diagnostic controls.

| Input | Configuration | ms | Paired vs off | Peak MiB |
|---|---|---:|---:|---:|
| catalog | release | 8.122 | +0.25% (2/4) | 41.5 |
| catalog | off | 8.073 | +0.00% (0/4) | 45.4 |
| catalog | skip-a | 9.761 | +21.15% (0/4) | 43.0 |
| catalog | scan | 10.962 | +35.41% (0/4) | 42.2 |
| catalog | delay900 | 7.999 | -1.08% (4/4) | 44.5 |
| catalog | kh-first | 8.099 | -0.12% (2/4) | 47.4 |
| catalog | kh-first-b2 | 8.240 | +1.58% (1/4) | 46.8 |
| heldout | release | 11.164 | +0.64% (1/4) | 39.3 |
| heldout | off | 11.093 | +0.00% (0/4) | 41.4 |
| heldout | skip-a | 9.834 | -11.46% (4/4) | 42.3 |
| heldout | scan | 10.515 | -5.33% (4/4) | 41.6 |
| heldout | delay900 | 7.915 | -28.60% (4/4) | 43.8 |
| heldout | kh-first | 10.839 | -2.19% (4/4) | 39.9 |
| heldout | kh-first-b2 | 10.315 | -6.79% (4/4) | 40.7 |

Three separately traced blocks per input and active configuration measure the start and end of the observed functions relative to SDK entry. All 540 timed traced requests had one canonical solver call, one canonical G1 call with a nested raw G1 call, one exclusive G2 call, and forty K/H feed calls. Each trace had zero lost records. K/H weighting and recombination can continue after its last measured feed. The signed last column measures the last K/H feed return minus the later of the observed G1 and G2 returns, computed separately for each request. The full-proof table uses only plain processes.

| Input | Configuration | Profiling effect | Raw G1 start / G1 return / G2 return / first K/H feed / last K/H feed / canonical return ms | K/H last feed after observed curves ms |
|---|---|---:|---:|---:|
| catalog | off | +3.5% | 4.163 / 6.567 / 7.441 / 3.719 / 6.925 / 7.516 | -0.609 |
| catalog | skip-a | +16.4% | 3.046 / 6.916 / 8.748 / 6.826 / 9.926 / 10.578 | +1.085 |
| catalog | scan | +0.4% | 3.021 / 6.932 / 8.194 / 6.580 / 9.645 / 10.386 | +1.087 |
| catalog | delay900 | +4.6% | 4.044 / 6.049 / 7.352 / 3.765 / 6.956 / 7.516 | -0.396 |
| catalog | kh-first | +3.2% | 4.320 / 6.480 / 7.475 / 3.760 / 6.849 / 7.526 | -0.608 |
| catalog | kh-first-b2 | +1.6% | 4.273 / 6.126 / 7.058 / 3.797 / 6.992 / 7.417 | -0.124 |
| heldout | off | +4.6% | 3.244 / 7.518 / 8.144 / 7.213 / 10.379 / 10.815 | +2.088 |
| heldout | skip-a | +7.6% | 3.049 / 7.816 / 8.290 / 5.989 / 9.177 / 9.664 | +0.711 |
| heldout | scan | +5.2% | 3.034 / 7.701 / 8.039 / 6.747 / 9.837 / 10.334 | +1.683 |
| heldout | delay900 | +2.7% | 4.040 / 5.835 / 7.168 / 3.737 / 6.875 / 7.286 | -0.293 |
| heldout | kh-first | +4.3% | 3.463 / 7.593 / 7.784 / 7.055 / 10.110 / 10.626 | +2.063 |
| heldout | kh-first-b2 | +4.5% | 3.511 / 6.170 / 7.883 / 6.309 / 9.456 / 9.997 | +1.566 |

## Latency when a curve branch takes effectively zero time

Four blocks reuse selected curve results after first computing them in full, using the same isolated benchmark executable as the previous Ryzen experiment. Each process checks that its key and public-fixture witness are fixed. Proof assembly still uses fresh randomness and the verifier runs for every request. The diagnostic measures changes in full proof latency and contention for the other branches. These rows are counterfactual timings.

| Input | Fixed fixture results reused | ms | Paired ms saved vs control | CPU-ms per request |
|---|---|---:|---:|---:|
| catalog | Control | 8.130 | +0.000 | 375.5 |
| catalog | A | 7.902 | +0.213 | 354.9 |
| catalog | K/H | 7.903 | +0.218 | 276.7 |
| catalog | A + K/H | 7.517 | +0.599 | 236.1 |
| catalog | B2 | 7.652 | +0.462 | 285.6 |
| catalog | A + B2 | 7.575 | +0.561 | 206.0 |
| catalog | K/H + B2 | 5.959 | +2.175 | 125.6 |
| catalog | All three | 3.357 | +4.770 | 27.6 |
| catalog | Frozen release | 8.094 | +0.021 | 376.7 |
| heldout | Control | 11.195 | +0.000 | 456.0 |
| heldout | A | 7.890 | +3.315 | 349.9 |
| heldout | K/H | 7.769 | +3.447 | 285.4 |
| heldout | A + K/H | 7.508 | +3.682 | 235.1 |
| heldout | B2 | 8.853 | +2.364 | 283.2 |
| heldout | A + B2 | 7.522 | +3.679 | 202.9 |
| heldout | K/H + B2 | 5.366 | +5.829 | 122.9 |
| heldout | All three | 3.355 | +7.841 | 27.5 |
| heldout | Frozen release | 11.233 | -0.042 | 454.9 |

[Raw measurements](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Proof verification](artifacts/verification.json) · [Probe sites](artifacts/symbols.json) · [Scheduling patch](artifacts/schedule.patch) · [Fixed fixture patch](../2026-11-19-branch-capacity727/artifacts/diagnostic.patch) · [All M5 references](../2026-11-19-equal-core-scorecard707/README.md)
