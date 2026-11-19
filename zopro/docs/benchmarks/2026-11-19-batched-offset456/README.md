# Batched B2 setup on eight new 1×2 witnesses

The Ryzen test used 32 logical workers on 16 physical cores. Each process loaded the same key. Its first SDK proof includes lazy preparation and excludes key loading. Seven randomized matched blocks per input timed 31 warm proofs after three primes. All inputs were generated after the fixed coefficients were selected. The release column uses the exact integrated binary from experiment 451.

| New input | Warm release / disabled / batched ms | Paired vs release | Paired vs disabled | First proof disabled / batched ms | Process peak disabled / batched MiB |
|---:|---:|---:|---:|---:|---:|
| 9 | 7.061 / 7.098 / 7.032 | -0.31% (4/7) | -0.59% (5/7) | 59.66 / 60.45 | 63.0 / 65.8 |
| 10 | 7.047 / 7.095 / 7.008 | -0.61% (5/7) | -1.58% (6/7) | 59.62 / 60.64 | 63.5 / 61.0 |
| 11 | 7.076 / 7.084 / 7.002 | -0.98% (6/7) | -1.32% (7/7) | 59.15 / 60.88 | 62.8 / 63.0 |
| 12 | 7.091 / 7.057 / 7.036 | -0.43% (5/7) | -0.28% (6/7) | 60.00 / 61.01 | 61.8 / 63.5 |
| 13 | 7.067 / 7.106 / 6.973 | -0.79% (5/7) | -1.43% (6/7) | 59.58 / 60.83 | 61.5 / 63.0 |
| 14 | 7.049 / 7.086 / 7.034 | -0.26% (4/7) | -0.73% (5/7) | 59.43 / 60.52 | 60.5 / 63.2 |
| 15 | 7.043 / 7.033 / 6.989 | -0.82% (6/7) | -0.22% (5/7) | 58.88 / 60.92 | 62.5 / 62.5 |
| 16 | 7.078 / 7.055 / 7.022 | -0.84% (6/7) | -0.69% (5/7) | 59.00 / 60.49 | 65.2 / 62.2 |

On input 9, the median first proof took 196.80 ms with serial setup and 60.45 ms with batched setup. Batching won 7/7 blocks. The diagnostic probes measured 136.86 ms and 3.88 ms inside setup, including validation of the key digest. The batched route reserved 49,685,752 bytes of temporary workspace under the existing estimator. The zero and one-MiB optional budgets selected the ordinary prover, and both proofs verified. Across eight inputs the median paired extra first-proof time against disabled was +1.26 ms.

Across these inputs the median paired warm change was -0.70% against release (8/8 wins) and -0.71% against disabled (8/8 wins). 8/8 inputs favored batching against both controls. The candidate met the predeclared six-input replication criterion. Every one of the 180 exported proofs passed the independent Gnark verifier.

[Protocol](artifacts/protocol.md) · [Samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [Incremental patch from experiment 455](artifacts/changes.patch) · [Original offset experiment](../2026-11-19-public-offset-sdk455/README.md)
