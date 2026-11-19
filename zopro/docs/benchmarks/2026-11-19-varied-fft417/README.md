# Fused FFT input columns on eight 12×2 transfer witnesses

The 16-worker profile uses 18 signed H windows with one range. The 32-worker profile uses ordinary H. Both use compact Poseidon outputs. The enabled and disabled measurements use the same clean executable. The predecessor is the exact preceding release. Three shuffled fresh-process blocks per witness, width and setting timed 21 verified requests each. Ratios compare process means within each matched block.

| Threads | Witness | Predecessor ms | Disabled ms | Enabled ms | Paired vs predecessor | Paired vs disabled |
|---:|---:|---:|---:|---:|---:|---:|
| 16 | 1 | 37.050 | 36.942 | 37.170 | +0.76% | -0.93% |
| 16 | 2 | 36.583 | 37.033 | 36.512 | -0.34% | -0.99% |
| 16 | 3 | 37.574 | 37.232 | 36.914 | -0.37% | +0.06% |
| 16 | 4 | 36.890 | 37.308 | 36.793 | -0.42% | -1.38% |
| 16 | 5 | 37.583 | 37.168 | 36.496 | -2.29% | -0.52% |
| 16 | 6 | 37.262 | 37.037 | 36.229 | -2.77% | -2.15% |
| 16 | 7 | 36.866 | 36.880 | 36.481 | +0.31% | -1.08% |
| 16 | 8 | 36.968 | 37.877 | 36.807 | -0.43% | -2.16% |
| 32 | 1 | 33.144 | 32.914 | 32.569 | -1.74% | -0.73% |
| 32 | 2 | 32.996 | 32.830 | 32.723 | -1.35% | -0.22% |
| 32 | 3 | 33.727 | 33.216 | 34.362 | +2.30% | +4.21% |
| 32 | 4 | 33.398 | 33.039 | 33.034 | -0.50% | -0.90% |
| 32 | 5 | 32.952 | 32.695 | 32.692 | -1.00% | -0.10% |
| 32 | 6 | 33.176 | 33.083 | 32.692 | -1.87% | -1.60% |
| 32 | 7 | 33.508 | 32.852 | 32.698 | -2.42% | -0.47% |
| 32 | 8 | 33.156 | 33.557 | 32.813 | -0.54% | -0.44% |

| Threads | Mode | Median ms across witnesses | Median CPU-ms | Maximum peak MiB | M5 CPU / Metal speedup |
|---:|---|---:|---:|---:|---:|
| 16 | release | 37.009 | 543.3 | 564.0 | 7.67× / 2.73× |
| 16 | off | 37.102 | 542.3 | 568.5 | 7.65× / 2.72× |
| 16 | on | 36.652 | 537.5 | 561.0 | 7.75× / 2.76× |
| 32 | release | 33.166 | 922.7 | 697.5 | 8.56× / 3.05× |
| 32 | off | 32.977 | 916.8 | 703.2 | 8.61× / 3.06× |
| 32 | on | 32.710 | 908.0 | 699.0 | 8.68× / 3.09× |

At 16 workers the enabled path changed paired latency by -0.40% against the predecessor (6/8 witnesses faster) and -1.04% against the identical disabled build (7/8). This profile qualified.

At 32 workers the enabled path changed paired latency by -1.18% against the predecessor (7/8 witnesses faster) and -0.46% against the identical disabled build (7/8). This profile qualified.

[Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json) · [All process means and timings](artifacts/processes.json.gz) · [Decision](artifacts/decision.json) · [Integration checks](artifacts/integration.json) · [Clean source and oracle](../2026-11-19-fft-input-release415/README.md)
