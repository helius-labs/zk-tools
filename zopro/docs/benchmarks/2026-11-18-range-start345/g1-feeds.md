# Parallel K and H feeds within balanced G1 windows

Splitting the feeds creates two temporary bucket sets per window. The prover merges them before a single weighted reduction. The two-range rows use ordinary G2 decomposition. The four-range rows use independent range decomposition from the previous experiment. All paths verify full blinded proofs.

| Shape | Workers | Mode | Wall ms | Paired vs corresponding unsplit | Wins | CPU-ms | Peak MiB |
|---|---:|---|---:|---:|---:|---:|---:|
| 1x2 | 64 | base2 | 8.479 | -0.45% | 5/7 | 278.6 | 51 |
| 1x2 | 64 | off2 | 8.531 | +0.00% | 0/7 | 279.4 | 51 |
| 1x2 | 64 | split2 | 10.116 | +18.04% | 0/7 | 325.9 | 59 |
| 1x2 | 64 | base4 | 9.497 | +0.42% | 2/7 | 308.3 | 50 |
| 1x2 | 64 | off4 | 9.453 | +0.00% | 0/7 | 306.7 | 58 |
| 1x2 | 64 | split4 | 9.627 | +2.01% | 1/7 | 335.2 | 59 |
| 1x2 | 96 | base2 | 10.296 | +1.61% | 2/7 | 414.0 | 51 |
| 1x2 | 96 | off2 | 10.224 | +0.00% | 0/7 | 412.0 | 52 |
| 1x2 | 96 | split2 | 9.479 | -6.30% | 7/7 | 424.8 | 53 |
| 1x2 | 96 | base4 | 8.963 | -0.05% | 4/7 | 419.6 | 48 |
| 1x2 | 96 | off4 | 8.931 | +0.00% | 0/7 | 418.6 | 49 |
| 1x2 | 96 | split4 | 9.828 | +9.73% | 0/7 | 466.1 | 53 |

All 84 exported proofs verified.

At 96 cores, splitting G1 feeds improved the ordinary two-range case in seven pairs, but the 9.479 ms result was slower than the serial G2 result on the same host. With range-local G2 decomposition, the same G1 change increased latency. At 64 cores the ordinary two-range case also regressed.

[Protocol](artifacts/sibling-protocol.md) · [Samples](artifacts/sibling-processes.json.gz) · [Verification](artifacts/sibling-verification.json)
