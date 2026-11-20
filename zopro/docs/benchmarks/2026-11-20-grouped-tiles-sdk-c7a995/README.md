# Consolidated grouped point tiles on 96-core C7a

Across the four existing shapes, paired median changes for the integrated tiled SDK were -1.24% to +0.71% relative to the archived tiled SDK and -22.95% to +0.22% relative to the pinned catalog profiles.

The integrated tiled 8×2 catalog proof took 19.250 ms against the 18.9 ms 10× M5 CPU threshold, yielding 9.82× CPU and 3.84× Metal. The exact catalog control on this instance took 19.104 ms.

The [integrated Rust SDK](../2026-11-20-grouped-tiles-sdk-ryzen994/README.md) and archived tiled build use 64 main cores, 32 frontend cores and a 16-core solver subset. The pinned catalog controls retain their existing placements. Prepared Aeglos uses all 96 physical cores. Each block compares the same request and proving key on one c7a.24xlarge.

| Shape | Request | Pinned / archived / off / tiled / Aeglos ms | Tiled paired vs archived | Tiled paired vs off | Tiled paired vs Aeglos | Peak tiled / Aeglos MiB |
|---|---|---:|---:|---:|---:|---:|
| 3×2 | catalog | 16.183 / 13.475 / 13.994 / 13.524 / 15.775 | +0.49% (2/8) | -2.99% (8/8) | -13.72% (8/8) | 189.1 / 1163.0 |
| 3×2 | heldout | 16.285 / 13.395 / 13.850 / 13.531 / 15.937 | +0.38% (3/8) | -2.31% (7/8) | -15.20% (8/8) | 197.3 / 1169.6 |
| 4×2 | catalog | 14.258 / 14.171 / 14.782 / 14.161 / 18.004 | -0.14% (4/8) | -4.38% (8/8) | -21.62% (8/8) | 205.8 / 1349.9 |
| 4×2 | heldout | 14.204 / 14.119 / 14.725 / 14.189 / 18.113 | +0.45% (3/8) | -3.74% (8/8) | -21.86% (8/8) | 211.0 / 1353.9 |
| 4×8 | catalog | 19.389 / 15.695 / 16.391 / 15.640 / 19.278 | -0.11% (5/8) | -4.48% (8/8) | -18.88% (8/8) | 237.9 / 1462.6 |
| 4×8 | heldout | 19.437 / 15.567 / 16.223 / 15.666 / 19.466 | +0.71% (2/8) | -3.24% (8/8) | -19.57% (8/8) | 236.4 / 1463.6 |
| 2×16 | catalog | 19.537 / 15.105 / 16.825 / 14.995 / 16.507 | -1.24% (6/8) | -11.17% (8/8) | -8.94% (8/8) | 233.4 / 1224.7 |
| 2×16 | heldout | 19.330 / 15.118 / 17.022 / 15.101 / 16.576 | +0.28% (4/8) | -11.12% (8/8) | -8.86% (8/8) | 235.7 / 1237.2 |
| 8×2 | catalog | 19.104 / 19.121 / 19.531 / 19.250 / 30.220 | +0.08% (3/8) | -1.75% (7/8) | -36.43% (8/8) | 366.9 / 2468.9 |
| 8×2 | heldout | 19.267 / 19.490 / 19.641 / 19.454 / 30.494 | -0.34% (6/8) | -1.03% (6/8) | -36.34% (8/8) | 366.7 / 2491.1 |

| Reference shape | Request | Pinned CPU / Metal | Tiled CPU / Metal | CPU 10× target ms |
|---|---|---:|---:|---:|
| 4×2 | catalog | 7.64× / 3.37× | 7.70× / 3.39× | 10.9 |
| 4×2 | heldout | 7.67× / 3.38× | 7.68× / 3.38× | 10.9 |
| 8×2 | catalog | 9.89× / 3.87× | 9.82× / 3.84× | 18.9 |
| 8×2 | heldout | 9.81× / 3.84× | 9.72× / 3.80× | 18.9 |

Eight blocks prime three proofs and time seventeen per process. Each timed Aeglos proof hits its prepared cache. Three traced legs per case used byte-identical SDK executables and checked G1/G2 compaction and fallback under a zero preparation budget. All 430 exports passed independent verification, and the temporary AWS resources were removed.

[Summary](artifacts/summary.json) · [Samples and routes](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Cleanup](artifacts/cleanup.json)
