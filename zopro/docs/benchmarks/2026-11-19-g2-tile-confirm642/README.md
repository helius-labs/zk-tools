# G2 tile fusion on four 2×2 transfers

The original request had 12 balanced blocks. The private split (33), deposit (34) and data-bearing transfer (40) had eight each. Each block timed 21 warm proofs per arm. The 144 timing exports, 16 separate CPU profile exports and four trace exports verified against the pinned key and expected public outputs. Percentages are medians of within-block differences. Parentheses give the number of blocks where fusion was faster.

On the original, fusion beat staged weighting in 8/12 pairs, repeating the initial direction. Against the ordinary path in the same executable, it split 6/12 pairs on the original and 19/36 across all four inputs. The paired median against that control stayed within 0.10% on each witness. Peak RSS on the original reached 115.0 MiB for fusion and 109.8 MiB for disabled. The follow-up supports combining the steps if using the tiled algorithm, but gives insufficient evidence to switch the production algorithm.

| Input and workers | Release / disabled / staged / fused ms | Fused vs staged / disabled / release | Peaks in same order, MiB | Fused vs M5 CPU / Metal |
|---|---:|---:|---:|---:|
| 2x2-0-w32 | 9.628 / 9.589 / 9.604 / 9.531 | -0.72% (8/12) / -0.10% (6/12) / -0.97% (8/12) | 111.0 / 109.8 / 114.0 / 115.0 | 6.19× / 3.15× |
| 2x2-33-w32 | 9.622 / 9.599 / 9.606 / 9.588 | -0.25% (5/8) / -0.05% (5/8) / -0.34% (4/8) | 111.0 / 109.8 / 116.8 / 110.5 | — |
| 2x2-34-w32 | 9.636 / 9.539 / 9.590 / 9.518 | -0.27% (5/8) / +0.05% (4/8) / -0.74% (7/8) | 110.5 / 113.5 / 114.0 / 113.5 | — |
| 2x2-40-w32 | 9.655 / 9.599 / 9.555 / 9.573 | +0.28% (4/8) / +0.01% (4/8) / -0.60% (4/8) | 110.8 / 111.5 / 112.0 / 111.2 | — |

[Protocol](protocol.md) · [Initial result](../2026-11-19-g2-merge-weight640/README.md) · [All paired comparisons](artifacts/summary.json) · [Dispatch traces](artifacts/traces.json.gz) · [Samples and CPU profiles](artifacts/rows.json.gz) · [Configurations](artifacts/configs.json.gz) · [Verification](artifacts/verification.json)
