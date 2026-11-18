# Integrated two-range H on C7a

This panel used the exact integrated executable on a fresh C7a independently of the preceding H query study. Five randomized fresh processes per setting timed eleven verified SDK requests each after three primes. The retained and qualified binaries respectively provide the preceding integrated release and the earlier experimental two-range code. The one- and two-range settings share the new release binary. RSS is peak process high-water memory. The candidate uses no retained H tables.

| Shape | Cores | Earlier release ms | Integrated one range ms | Integrated two ranges ms | Two vs one paired | Faster blocks | Two vs earlier release paired | Faster blocks | Earlier candidate ms | Peak MiB one / two | CPU / Metal speedup, two |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 16x2 | 64 | 37.505 | 37.440 | 35.169 | -6.28% | 5/5 | -5.54% | 5/5 | 36.324 | 721 / 721 | 9.53× / 3.53× |
| 16x2 | 96 | 40.423 | 40.159 | 38.517 | -4.59% | 5/5 | -6.25% | 5/5 | 38.387 | 714 / 716 | 8.70× / 3.22× |
| 24x2 | 64 | 51.640 | 51.128 | 47.786 | -8.75% | 5/5 | -9.34% | 5/5 | 49.446 | 1104 / 1127 | 11.05× / 3.68× |
| 24x2 | 96 | 54.225 | 54.064 | 52.516 | -2.59% | 5/5 | -3.00% | 5/5 | 50.357 | 1150 / 1157 | 10.05× / 3.35× |
| merge-24x1 | 56 | 53.303 | 54.312 | 50.258 | -8.02% | 5/5 | -5.71% | 5/5 | 51.520 | 1032 / 1070 | 10.09× / 3.48× |
| merge-24x1 | 64 | 54.981 | 54.810 | 51.855 | -7.62% | 5/5 | -5.30% | 5/5 | 51.247 | 1025 / 1057 | 9.78× / 3.37× |
| merge-24x1 | 96 | 58.880 | 59.310 | 56.302 | -4.77% | 5/5 | -3.96% | 5/5 | 56.562 | 1056 / 1073 | 9.01× / 3.11× |

At 56 cores, merge24 took 50.258 ms on the integrated two-range executable. Its 10× M5 CPU threshold is 50.7 ms. 4/5 process means met that threshold on the fresh host.

## Full process comparison

| Shape | Cores | Mode | Wall ms | CPU-ms/proof | Request p95 ms | Peak MiB |
|---|---:|---|---:|---:|---:|---:|
| 16x2 | 64 | one | 37.440 | 1761.9 | 40.184 | 721 |
| 16x2 | 64 | qualified | 36.324 | 1813.2 | 37.839 | 749 |
| 16x2 | 64 | retained | 37.505 | 1756.1 | 39.249 | 744 |
| 16x2 | 64 | two | 35.169 | 1761.6 | 37.632 | 721 |
| 16x2 | 96 | one | 40.159 | 2646.1 | 42.534 | 714 |
| 16x2 | 96 | qualified | 38.387 | 2786.2 | 43.295 | 743 |
| 16x2 | 96 | retained | 40.423 | 2661.7 | 45.307 | 716 |
| 16x2 | 96 | two | 38.517 | 2681.4 | 40.918 | 716 |
| 24x2 | 64 | one | 51.128 | 2406.1 | 54.547 | 1104 |
| 24x2 | 64 | qualified | 49.446 | 2522.3 | 54.700 | 1159 |
| 24x2 | 64 | retained | 51.640 | 2436.6 | 55.274 | 1131 |
| 24x2 | 64 | two | 47.786 | 2440.9 | 51.328 | 1127 |
| 24x2 | 96 | one | 54.064 | 3517.9 | 56.026 | 1150 |
| 24x2 | 96 | qualified | 50.357 | 3679.2 | 57.141 | 1149 |
| 24x2 | 96 | retained | 54.225 | 3509.7 | 57.439 | 1155 |
| 24x2 | 96 | two | 52.516 | 3557.2 | 56.545 | 1157 |
| merge-24x1 | 56 | one | 54.312 | 2185.3 | 58.071 | 1032 |
| merge-24x1 | 56 | qualified | 51.520 | 2242.4 | 53.626 | 1053 |
| merge-24x1 | 56 | retained | 53.303 | 2167.9 | 56.311 | 1039 |
| merge-24x1 | 56 | two | 50.258 | 2198.6 | 52.617 | 1070 |
| merge-24x1 | 64 | one | 54.810 | 2433.7 | 57.259 | 1025 |
| merge-24x1 | 64 | qualified | 51.247 | 2458.7 | 54.726 | 1025 |
| merge-24x1 | 64 | retained | 54.981 | 2396.0 | 57.748 | 992 |
| merge-24x1 | 64 | two | 51.855 | 2425.8 | 53.637 | 1057 |
| merge-24x1 | 96 | one | 59.310 | 3623.6 | 61.407 | 1056 |
| merge-24x1 | 96 | qualified | 56.562 | 3856.2 | 60.074 | 1073 |
| merge-24x1 | 96 | retained | 58.880 | 3621.4 | 61.247 | 1077 |
| merge-24x1 | 96 | two | 56.302 | 3691.5 | 59.639 | 1073 |

All 140 primary and 80 supplementary exported proofs passed independent verification on AWS and after retrieval. All process outputs were checked against the archive manifest. The instance terminated, no tagged EBS volumes remained, and the temporary bucket was removed.

[Protocol](artifacts/aws-protocol.md) · [Timing data](artifacts/aws-processes.json.gz) · [Verification](artifacts/aws-verification.json)

[Local implementation and memory fallback](../2026-11-18-h-range-release354/README.md) · [Earlier C7a geometry comparison](../2026-11-18-h-scale353/README.md)

[Eight-shape M5 scorecard](scorecard.md) · [Solver epoch experiment](epochs.md)
