# Merge proof latency with populated Merkle paths

The preceding [merge trial](../2026-11-19-merge-factors478/README.md) froze an optional 8,507,424-byte coefficient file on eight public fixtures. These eight additional public requests use the identical file. Each active state tree contains an occupied sibling subtree at every upper depth and nonzero leaves at the lower levels. Every active nullifier tree was populated by ordinary insertions at sequential indices using varied field values. All 32 state sibling values changed for every real input compared with its occupancy-matched fixture from the preceding trial. Four to seven of the 40 nullifier sibling values changed, depending on the input. Upper nullifier subtrees remain empty. The original benchmark request has 24 real inputs and all 24 use the same nullifier path at index zero. Each real nullifier's host-side non-inclusion witness was checked. These fixtures test one specific tree population pattern.

Both configurations use the same production prover, 32 Ryzen workers and local Euler weighting. Five shuffled pairs per request each issued three primes and eleven timed proofs. Each process exported a proof checked by Gnark against the independently generated public witness.

| Real inputs | Prior nullifier insertions in main tree | Baseline ms | File ms | Paired change | Faster blocks | Peak MiB, baseline / file |
|---:|---:|---:|---:|---:|---:|---:|
| 2 | 72 | 67.208 | 66.906 | -0.71% | 4/5 | 1080.0 / 1084.6 |
| 3 | 80 | 67.591 | 67.098 | -0.50% | 4/5 | 1115.2 / 1096.5 |
| 4 | 88 | 62.823 | 65.649 | +4.68% | 0/5 | 1066.2 / 1051.7 |
| 8 | 96 | 69.407 | 68.845 | -0.79% | 3/5 | 1147.0 / 1113.6 |
| 12 | 104 | 69.590 | 69.457 | -0.19% | 3/5 | 1174.0 / 1115.2 |
| 16 | 112 | 67.938 | 69.034 | +1.39% | 0/5 | 1148.8 / 1128.3 |
| 20 | 120 | 72.477 | 72.368 | -0.15% | 3/5 | 1211.5 / 1185.8 |
| 24 | 64 | 70.482 | 70.888 | +0.13% | 2/5 | 1230.0 / 1140.8 |

The median paired change was -0.17%, with 5/8 requests faster. The admission rule required a pass on the preceding trial, a favorable median here, gains on at least six of these eight inputs and a gain on the fully populated 24-input request. The frozen file missed that rule. Its one-time preparation on the preceding trial took 586.2 ms (median). The 80 additional exported proofs verified.

[Results and request hashes](artifacts/results.json) · [Per-process samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)

The qualified merge profile retains Euler weighting alone.

## Qualified 32-worker Ryzen results

The [24×2 report](../2026-11-19-transfer-factors474/README.md) documents the preceding eight-shape scorecard. Speedups use the supplied M5 CPU and Metal latency references.

| Shape | Warm proof ms | Peak RSS MiB | M5 CPU speedup | M5 Metal speedup |
|---|---:|---:|---:|---:|
| 1x2 | 7.033 | 65.0 | 5.26× | 3.41× |
| 2x2 | 9.619 | 117.5 | 6.13× | 3.12× |
| 4x2 | 15.421 | 263.5 | 7.07× | 3.11× |
| 8x2 | 23.898 | 436.2 | 7.91× | 3.10× |
| 12x2 | 31.064 | 580.4 | 9.14× | 3.25× |
| 16x2 | 39.553 | 670.1 | 8.47× | 3.14× |
| 24x2 | 64.422 | 1038.9 | 8.20× | 2.73× |
| merge-24x1 | 63.216 | 1030.2 | 8.02× | 2.77× |
