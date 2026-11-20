# Transferring the newer proving profile to three grouped keys

The newer profile reduced paired full-proof latency by 20.54–22.65% across the six Ryzen cases. It uses the newer witness and quotient schedule from the [4×2 C7a comparison](../2026-11-20-a-class-tiles-c7a975/README.md). Adding the point tiles separately increased full-proof latency by 0.62–4.29% versus the newer profile alone on this host.

The three shapes use their original proving keys and matched requests. Every arm runs with a 24-worker main pool and eight-worker frontend on Ryzen. The newer modes also use a four-worker solver pool inside the frontend. Older profiles had different worker placements on C7a, so these timings characterize the Ryzen transfer test.

| Shape | Input | Configuration | Proof ms | Paired vs older | Paired vs newer | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 2×16 | catalog | Older pinned profile | 20.382 | — | +29.28% (0/8) | 257.6 |
| 2×16 | catalog | Newer profile | 15.711 | -22.65% (8/8) | — | 242.3 |
| 2×16 | catalog | Newer profile with tiles | 15.823 | -21.84% (8/8) | +0.62% (2/8) | 253.3 |
| 2×16 | heldout | Older pinned profile | 20.346 | — | +28.39% (0/8) | 265.2 |
| 2×16 | heldout | Newer profile | 15.699 | -22.11% (8/8) | — | 236.6 |
| 2×16 | heldout | Newer profile with tiles | 15.849 | -21.70% (8/8) | +0.90% (0/8) | 253.4 |
| 3×8 | catalog | Older pinned profile | 18.707 | — | +26.73% (0/8) | 245.2 |
| 3×8 | catalog | Newer profile | 14.678 | -21.09% (8/8) | — | 237.4 |
| 3×8 | catalog | Newer profile with tiles | 15.147 | -18.97% (8/8) | +2.87% (1/8) | 234.0 |
| 3×8 | heldout | Older pinned profile | 18.497 | — | +25.85% (0/8) | 246.5 |
| 3×8 | heldout | Newer profile | 14.740 | -20.54% (8/8) | — | 228.0 |
| 3×8 | heldout | Newer profile with tiles | 15.056 | -18.70% (8/8) | +2.18% (1/8) | 238.2 |
| 4×16 | catalog | Older pinned profile | 24.048 | — | +28.01% (1/8) | 318.1 |
| 4×16 | catalog | Newer profile | 18.734 | -21.88% (7/8) | — | 331.8 |
| 4×16 | catalog | Newer profile with tiles | 19.580 | -18.47% (7/8) | +4.19% (0/8) | 339.7 |
| 4×16 | heldout | Older pinned profile | 24.085 | — | +27.82% (0/8) | 320.9 |
| 4×16 | heldout | Newer profile | 18.736 | -21.77% (8/8) | — | 295.2 |
| 4×16 | heldout | Newer profile with tiles | 19.550 | -18.89% (8/8) | +4.29% (0/8) | 294.7 |

Route probes confirmed K and B2 tiling on all three shapes and A tiling on 4×16. They checked exact class membership, the full curve result for active lanes and the ordinary fallback at zero optional scratch. All 180 proof exports independently verified. The timing panel uses eight blocks per input, with two separate clocked blocks for the newer modes.

[Results](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Exact SDK source](../2026-11-20-a-class-tiles-ryzen974/README.md)
