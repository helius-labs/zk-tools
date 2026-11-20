# Scoped SDK placement for the three remaining C7a shapes

Prepared Aeglos retained the paired latency lead on all three shapes.

The [scoped Rust SDK candidate](../2026-11-20-scoped-g2-sdk-ryzen999/README.md) runs the existing exclusive G2 query on its own worker pool while the main pool computes the quotient and G1. The archived early profiles, pinned catalog controls, and prepared Aeglos use the same proving key and request on one c7a.24xlarge. The fused arms also enable Euler leaf fusion and packed final verification.

| Shape | Selected profile | Catalog Zopro / Aeglos ms | Held-out Zopro / Aeglos ms | Paired catalog / held-out | Peak Zopro / Aeglos MiB |
|---|---|---:|---:|---:|---:|
| 1×4 | SDK, fused 64+32 | 9.965 / 9.511 | 9.926 / 9.507 | +4.08% (0/8) / +3.91% (1/8) | 159.9 / 582.2 |
| 1×8 | SDK, fused 48+48 | 11.184 / 10.605 | 11.170 / 10.675 | +5.41% (0/8) / +4.46% (0/8) | 140.5 / 690.0 |
| 1×16 | SDK, fused 48+48 | 13.537 / 12.390 | 13.646 / 12.374 | +9.20% (0/8) / +10.52% (0/8) | 170.5 / 826.9 |

Selection uses the smallest worst-request median among profiles with at least six paired wins and lower paired medians on both requests. Where Aeglos leads, the table shows the tested Zopro profile with the smallest worst-request median.

| Shape | Request | Profile | Proof ms | Paired vs archive | Paired vs Aeglos | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1×4 | catalog | Pinned | 11.775 | +16.88% (0/8) | +23.69% (0/8) | 124.0 |
| 1×4 | catalog | Archived 64+32 | 10.236 | +0.00% (0/8) | +6.26% (0/8) | 158.3 |
| 1×4 | catalog | SDK 64+32 | 10.061 | -1.31% (5/8) | +5.24% (0/8) | 156.1 |
| 1×4 | catalog | SDK 48+48 | 10.474 | +2.92% (2/8) | +9.35% (0/8) | 112.7 |
| 1×4 | catalog | SDK 56+40 | 10.112 | -1.22% (6/8) | +6.28% (0/8) | 151.7 |
| 1×4 | catalog | SDK 72+24 | 10.318 | +0.84% (2/8) | +7.70% (0/8) | 148.7 |
| 1×4 | catalog | SDK, fused 64+32 | 9.965 | -2.72% (8/8) | +4.08% (0/8) | 156.9 |
| 1×4 | catalog | Prepared Aeglos | 9.511 | -5.89% (8/8) | +0.00% (0/8) | 582.2 |
| 1×4 | heldout | Pinned | 11.935 | +16.85% (0/8) | +25.34% (0/8) | 121.1 |
| 1×4 | heldout | Archived 64+32 | 10.225 | +0.00% (0/8) | +6.58% (0/8) | 160.7 |
| 1×4 | heldout | SDK 64+32 | 10.238 | +0.75% (3/8) | +7.13% (0/8) | 156.9 |
| 1×4 | heldout | SDK 48+48 | 10.518 | +2.51% (0/8) | +9.68% (0/8) | 105.9 |
| 1×4 | heldout | SDK 56+40 | 10.057 | -1.42% (5/8) | +5.30% (0/8) | 163.0 |
| 1×4 | heldout | SDK 72+24 | 10.229 | +0.08% (4/8) | +7.09% (0/8) | 153.1 |
| 1×4 | heldout | SDK, fused 64+32 | 9.926 | -3.08% (7/8) | +3.91% (1/8) | 159.9 |
| 1×4 | heldout | Prepared Aeglos | 9.507 | -6.17% (8/8) | +0.00% (0/8) | 581.6 |
| 1×8 | catalog | Pinned | 13.178 | +17.69% (0/8) | +24.56% (0/8) | 129.2 |
| 1×8 | catalog | Archived 48+48 | 11.179 | +0.00% (0/8) | +5.60% (0/8) | 147.1 |
| 1×8 | catalog | SDK 48+48 | 11.185 | +0.36% (3/8) | +5.58% (0/8) | 145.6 |
| 1×8 | catalog | SDK 56+40 | 12.153 | +9.03% (0/8) | +15.14% (0/8) | 196.7 |
| 1×8 | catalog | SDK 64+32 | 12.134 | +8.70% (0/8) | +14.79% (0/8) | 190.6 |
| 1×8 | catalog | SDK 72+24 | 12.088 | +8.06% (0/8) | +14.10% (0/8) | 205.0 |
| 1×8 | catalog | SDK, fused 48+48 | 11.184 | +0.17% (3/8) | +5.41% (0/8) | 140.5 |
| 1×8 | catalog | Prepared Aeglos | 10.605 | -5.30% (8/8) | +0.00% (0/8) | 690.0 |
| 1×8 | heldout | Pinned | 13.108 | +15.05% (0/8) | +22.67% (0/8) | 136.6 |
| 1×8 | heldout | Archived 48+48 | 11.353 | +0.00% (0/8) | +6.80% (0/8) | 138.2 |
| 1×8 | heldout | SDK 48+48 | 11.275 | -0.06% (4/8) | +5.49% (0/8) | 137.6 |
| 1×8 | heldout | SDK 56+40 | 12.150 | +7.03% (0/8) | +14.06% (0/8) | 192.9 |
| 1×8 | heldout | SDK 64+32 | 12.066 | +6.17% (0/8) | +12.36% (0/8) | 209.4 |
| 1×8 | heldout | SDK 72+24 | 11.993 | +5.95% (0/8) | +11.45% (0/8) | 195.3 |
| 1×8 | heldout | SDK, fused 48+48 | 11.170 | -1.10% (7/8) | +4.46% (0/8) | 135.2 |
| 1×8 | heldout | Prepared Aeglos | 10.675 | -6.36% (8/8) | +0.00% (0/8) | 688.7 |
| 1×16 | catalog | Pinned | 15.551 | +13.12% (0/8) | +25.77% (0/8) | 161.5 |
| 1×16 | catalog | Archived 48+48 | 13.767 | +0.00% (0/8) | +11.60% (0/8) | 167.5 |
| 1×16 | catalog | SDK 48+48 | 13.765 | -0.48% (5/8) | +11.98% (0/8) | 164.5 |
| 1×16 | catalog | SDK 56+40 | 14.437 | +4.97% (0/8) | +17.12% (0/8) | 244.7 |
| 1×16 | catalog | SDK 64+32 | 14.215 | +3.35% (1/8) | +14.86% (0/8) | 249.2 |
| 1×16 | catalog | SDK 72+24 | 14.481 | +5.14% (0/8) | +17.32% (0/8) | 244.8 |
| 1×16 | catalog | SDK, fused 48+48 | 13.537 | -1.61% (7/8) | +9.20% (0/8) | 170.5 |
| 1×16 | catalog | Prepared Aeglos | 12.390 | -10.39% (8/8) | +0.00% (0/8) | 820.1 |
| 1×16 | heldout | Pinned | 15.558 | +14.18% (0/8) | +25.60% (0/8) | 166.7 |
| 1×16 | heldout | Archived 48+48 | 13.592 | +0.00% (0/8) | +10.12% (0/8) | 164.5 |
| 1×16 | heldout | SDK 48+48 | 13.651 | +0.50% (2/8) | +9.51% (0/8) | 171.9 |
| 1×16 | heldout | SDK 56+40 | 14.422 | +6.45% (0/8) | +16.61% (0/8) | 257.4 |
| 1×16 | heldout | SDK 64+32 | 14.254 | +4.87% (0/8) | +14.98% (0/8) | 244.0 |
| 1×16 | heldout | SDK 72+24 | 14.657 | +8.30% (0/8) | +18.45% (0/8) | 244.7 |
| 1×16 | heldout | SDK, fused 48+48 | 13.646 | +0.11% (4/8) | +10.52% (0/8) | 165.2 |
| 1×16 | heldout | Prepared Aeglos | 12.374 | -9.19% (8/8) | +0.00% (0/8) | 826.9 |

Eight paired timing blocks prime three proofs and time seventeen. The separate probe panel uses byte-identical executables to confirm G2 and quotient overlap and the zero-workspace fallback. All 414 exports passed independent verification. The temporary AWS resources were removed.

[Full results](artifacts/summary.json) · [Samples and probe phases](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Cleanup](artifacts/cleanup.json)
