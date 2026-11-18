# Parallel K and H feeds within balanced G1 windows

Splitting the feeds creates two temporary bucket sets per window. The prover merges them before a single weighted reduction. The two-range rows use ordinary G2 decomposition. The four-range rows use independent range decomposition from the previous experiment. All paths verify full blinded proofs.

| Shape | Workers | Mode | Wall ms | Paired vs corresponding unsplit | Wins | CPU-ms | Peak MiB |
|---|---:|---|---:|---:|---:|---:|---:|
| 1x2 | 32 | base2 | 7.788 | +2.12% | 2/5 | 151.5 | 66 |
| 1x2 | 32 | off2 | 7.682 | +0.00% | 0/5 | 151.2 | 78 |
| 1x2 | 32 | split2 | 7.994 | +3.78% | 1/5 | 160.2 | 72 |
| 1x2 | 32 | base4 | 8.003 | -0.35% | 3/5 | 160.0 | 66 |
| 1x2 | 32 | off4 | 7.990 | +0.00% | 0/5 | 159.7 | 80 |
| 1x2 | 32 | split4 | 8.212 | +3.23% | 1/5 | 167.7 | 68 |
| 2x2 | 32 | base2 | 10.768 | +0.15% | 2/5 | 202.7 | 114 |
| 2x2 | 32 | off2 | 10.735 | +0.00% | 0/5 | 201.9 | 114 |
| 2x2 | 32 | split2 | 10.732 | -0.78% | 3/5 | 200.7 | 112 |
| 2x2 | 32 | base4 | 11.106 | +0.74% | 2/5 | 213.0 | 120 |
| 2x2 | 32 | off4 | 11.020 | +0.00% | 0/5 | 214.1 | 110 |
| 2x2 | 32 | split4 | 11.231 | +2.34% | 1/5 | 212.7 | 115 |

All 64 exported proofs verified.

[Protocol](artifacts/protocol.md) · [Samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)
