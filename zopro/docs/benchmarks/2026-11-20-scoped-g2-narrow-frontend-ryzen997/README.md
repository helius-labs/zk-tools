# Scoped G2 with a smaller disjoint frontend on Ryzen

On the 12+4 layout the scoped path lowered paired latency by 7.81–19.99% versus the same-build late schedule. The 8+8 late schedule remained as fast or faster across the ten requests. The result isolates the gain for a fixed narrow layout and supports testing the 80+16 and 64+32 C7a profiles.

The [prior early schedule](../2026-11-20-scoped-g2-overlap-c7a993/README.md) required an equally wide or wider frontend. The candidate keeps witness preparation on the frontend when it is smaller, then moves the quotient to the main pool while G2 runs. Both worker sets use distinct physical cores. The SDK rejects overlapping fixed placements in early mode. Four recurring timing blocks prime three proofs and time thirteen per process.

| Shape | Request | 8+8 late / early ms | Paired change | 12+4 late / early ms | Paired change | 12+4 overlap ms |
|---|---|---:|---:|---:|---:|---:|
| 1×2 | catalog | 8.566 / 8.604 | +0.35% (2/4) | 11.466 / 10.487 | -8.51% (4/4) | 0.494 |
| 1×2 | heldout | 8.293 / 8.268 | +0.56% (2/4) | 11.394 / 10.504 | -7.81% (4/4) | 0.584 |
| 2×2 | catalog | 12.414 / 12.449 | -0.27% (2/4) | 14.398 / 12.635 | -12.25% (4/4) | 1.039 |
| 2×2 | heldout | 12.364 / 12.512 | +1.12% (2/4) | 14.460 / 12.639 | -12.12% (4/4) | 1.090 |
| 1×4 | catalog | 10.300 / 10.271 | -0.15% (3/4) | 13.899 / 12.079 | -12.92% (4/4) | 1.096 |
| 1×4 | heldout | 10.303 / 10.260 | -0.53% (3/4) | 14.008 / 12.235 | -12.72% (4/4) | 1.066 |
| 2×4 | catalog | 13.016 / 13.010 | +0.15% (1/4) | 16.267 / 14.409 | -11.28% (4/4) | 1.237 |
| 2×4 | heldout | 13.023 / 13.023 | +0.28% (1/4) | 16.378 / 14.428 | -11.91% (4/4) | 1.202 |
| 2×8 | catalog | 16.742 / 17.030 | +1.35% (0/4) | 21.044 / 16.986 | -19.30% (4/4) | 2.276 |
| 2×8 | heldout | 16.809 / 17.127 | +1.56% (0/4) | 20.991 / 16.846 | -19.99% (4/4) | 2.312 |

Every forced preparation-budget denial retained the late schedule. All 220 proof exports passed independent verification.

[Summary](artifacts/summary.json) · [Proofs and route observations](artifacts/rows.json.gz) · [Source patch](artifacts/source.patch.gz) · [Placement rejection](artifacts/overlap-rejection.json)
