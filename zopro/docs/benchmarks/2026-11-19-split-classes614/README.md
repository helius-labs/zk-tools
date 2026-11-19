# Increasing the surviving point count on one fixed witness

The split modes divide existing scalar classes into more point sums. The copy mode reconstructs all three layouts at their original sizes. Every timing comparison uses seven block pairs and full verified proofs. Positive differences mean that splitting took longer than the matching copy.

| Shape | Setting | Median ms | Paired change vs copy, ms | Paired % and faster blocks | Peak MiB |
|---|---|---:|---:|---:|---:|
| 6x2 | prior | 21.458 | -0.791 | -3.55% (4/7) | 375.5 |
| 6x2 | ordinary | 21.982 | +0.059 | +0.27% (3/7) | 378.2 |
| 6x2 | copy | 21.818 | +0.000 | +0.00% (0/7) | 417.2 |
| 6x2 | g1 | 28.444 | +6.625 | +29.92% (0/7) | 434.0 |
| 6x2 | g2 | 33.113 | +11.455 | +51.55% (0/7) | 458.2 |
| 6x2 | both | 39.007 | +16.904 | +77.58% (0/7) | 464.2 |
| 5x16 | prior | 24.323 | -0.483 | -1.95% (6/7) | 460.5 |
| 5x16 | ordinary | 24.364 | -0.518 | -2.09% (5/7) | 460.5 |
| 5x16 | copy | 24.819 | +0.000 | +0.00% (0/7) | 485.2 |

All 68 exported proofs verified. Individual contribution times can overlap inside one proof. [Protocol](protocol.md) · [Timed samples and exact query counts](artifacts/samples.json.gz) · [Source](artifacts/source.patch) · [Archived crossover](../2026-11-19-c7a-crossover601/README.md)
