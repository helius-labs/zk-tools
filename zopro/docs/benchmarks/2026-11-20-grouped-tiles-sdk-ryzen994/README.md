# SDK integration preflight for grouped point tiles

The integration candidate exposes grouped point tiling and the caller-owned solver pool through the Rust SDK. The prover reuses the existing point reduction and proof scratch account. Tilings are opt-in and selected per proof by the existing grouping and admission predicates. The new path uses the same fixed widths as the verified C7a experiment. The SDK build excludes the benchmark replay and timing hooks.

The Ryzen panel compares the [archived benchmark SDK](../2026-11-20-grouped-transfers-c7a988/README.md) with the integration candidate, with and without tiling in each. All arms use 24 main and eight frontend hardware threads. Witness work uses four of the frontend threads. Four paired blocks per input prime three proofs and time thirteen.

| Shape | Input | Archived off / on ms | Candidate off / on ms | Paired candidate vs archived off / on | Candidate on vs off | Peak off / on MiB |
|---|---|---:|---:|---:|---:|---:|
| 3×2 | catalog | 13.065 / 13.236 | 12.997 / 13.071 | -0.78% (3/4) / -1.04% (4/4) | +0.51% (0/4) | 182.0 / 190.8 |
| 3×2 | heldout | 12.937 / 13.378 | 12.973 / 13.213 | -0.55% (3/4) / -0.82% (3/4) | +2.94% (0/4) | 235.2 / 243.2 |
| 4×2 | catalog | 14.550 / 15.171 | 14.404 / 15.005 | -1.08% (3/4) / -1.31% (3/4) | +3.95% (0/4) | 195.5 / 221.5 |
| 4×2 | heldout | 14.468 / 15.137 | 14.541 / 14.997 | +0.56% (2/4) / -0.99% (3/4) | +2.85% (0/4) | 198.2 / 215.0 |
| 4×4 | catalog | 15.713 / 16.447 | 14.963 / 18.243 | -2.29% (3/4) / -0.12% (2/4) | +4.17% (1/4) | 222.0 / 297.7 |
| 4×4 | heldout | 15.169 / 15.658 | 14.985 / 15.558 | -1.21% (3/4) / -1.04% (3/4) | +3.84% (0/4) | 209.5 / 226.5 |
| 2×16 | catalog | 15.855 / 15.994 | 15.599 / 15.647 | -0.13% (2/4) / -2.66% (4/4) | -0.20% (2/4) | 247.1 / 229.8 |
| 2×16 | heldout | 15.774 / 15.826 | 15.625 / 15.710 | -0.25% (3/4) / -1.38% (3/4) | +0.12% (2/4) | 269.5 / 241.8 |

Relative to the archived SDK with tiling enabled, the candidate paired medians were within 2.67%. The catalog 4×4 case had variable process means, including a 22.86 ms tiled process. The separate C7a result will determine whether the integration preserves the server gains. All 165 native tests and 128 exported proofs passed, including an exact tiled weighted-sum test on G1 and G2. The generic x86-64 prover and Zolana crates passed their build check.

[Samples](artifacts/rows.json.gz) · [Summary](artifacts/summary.json) · [Candidate patch](artifacts/source.patch.gz) · [Validation](artifacts/validation.json)
