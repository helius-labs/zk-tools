# C7a G1 and G2 placement follow-up

Zopro has repeatable paired leads on all 41 tested shapes across matched, shape-specific C7a runs. On 1×16, the last outstanding shape, the fused K/H profile won all eight paired blocks on each request.

The 1×16 profile keeps 40 main cores and 40 G2 cores with G1 width 13. Enabling the existing bounded K/H fusion option reduced paired proof time by 5.04% and 5.66% versus separate K/H at the same settings. Peak process RSS fell from 165–171 MiB to 90–91 MiB. The panel also repeated the 1×4 fused lead and tested changes to G2 range counts on the smaller shapes.

| Shape | Request | Selected Zopro profile | Zopro / Aeglos ms | Paired vs Aeglos | Peak Zopro / Aeglos MiB |
|---|---|---|---:|---:|---:|
| 1×2 | catalog | 48+48, two G2 ranges | 7.765 / 8.190 | -5.18% (8/8) | 45.2 / 458.9 |
| 1×2 | heldout | 48+48, two G2 ranges | 7.685 / 8.162 | -5.52% (8/8) | 48.2 / 458.8 |
| 1×4 | catalog | 48+32, fused K/H | 8.716 / 9.732 | -10.63% (8/8) | 54.2 / 584.3 |
| 1×4 | heldout | 48+32, fused K/H | 8.718 / 9.609 | -8.89% (8/8) | 57.3 / 584.0 |
| 1×16 | catalog | 40+40, G1 width 13 and fused K/H | 11.702 / 12.394 | -5.62% (8/8) | 91.0 / 825.7 |
| 1×16 | heldout | 40+40, G1 width 13 and fused K/H | 11.668 / 12.392 | -5.71% (8/8) | 89.5 / 831.0 |
| 2×2 | catalog | 32+32 | 10.291 / 11.775 | -12.61% (8/8) | 78.9 / 814.5 |
| 2×2 | heldout | 32+32 | 10.310 / 11.853 | -12.76% (8/8) | 74.5 / 812.5 |

On 1×2, two G2 ranges improved paired times by 0.42% and 1.50% versus the pinned SDK, with seven of eight wins on each request. On 2×2, the 32+24 profile won five of eight catalog blocks against 32+32 and the 32+32 profile remains the selected result.

A lead requires a lower median paired latency ratio and at least six of eight wins on both requests. Blocks use three warmups and seventeen timed proofs. Each cell reports the median of block means. The C7a instance had 96 physical cores, and Aeglos used its prepared CPU path with 96 threads. Zopro used pinned executables and the listed runtime options. The 41-shape score combines profiles from separate matched runs.

| Shape | Request | Profile | Proof ms | Paired vs SDK control | Paired vs Aeglos | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1×2 | catalog | 48+48 | 8.084 | +0.00% (0/8) | -1.88% (7/8) | 52.7 |
| 1×2 | catalog | Pinned SDK | 7.792 | -3.51% (8/8) | -5.19% (8/8) | 43.8 |
| 1×2 | catalog | 48+48, two G2 ranges | 7.765 | -3.79% (8/8) | -5.18% (8/8) | 45.2 |
| 1×2 | catalog | 64+32, two G2 ranges | 8.152 | +0.71% (3/8) | -1.10% (5/8) | 47.4 |
| 1×2 | catalog | Prepared Aeglos | 8.190 | +1.92% (1/8) | +0.00% (0/8) | 458.9 |
| 1×2 | heldout | 48+48 | 7.765 | +0.00% (0/8) | -4.86% (8/8) | 52.8 |
| 1×2 | heldout | Pinned SDK | 7.769 | +0.15% (3/8) | -4.50% (8/8) | 42.9 |
| 1×2 | heldout | 48+48, two G2 ranges | 7.685 | -0.95% (8/8) | -5.52% (8/8) | 48.2 |
| 1×2 | heldout | 64+32, two G2 ranges | 7.909 | +1.61% (0/8) | -3.56% (8/8) | 46.1 |
| 1×2 | heldout | Prepared Aeglos | 8.162 | +5.11% (0/8) | +0.00% (0/8) | 458.8 |
| 1×4 | catalog | 48+32, fused K/H | 8.716 | +0.00% (0/8) | -10.63% (8/8) | 54.2 |
| 1×4 | catalog | 48+32, separate K/H | 9.591 | +9.88% (0/8) | -1.50% (6/8) | 101.4 |
| 1×4 | catalog | 56+32, fused K/H | 8.804 | +1.20% (1/8) | -8.98% (8/8) | 56.4 |
| 1×4 | catalog | 48+40, fused K/H | 8.783 | +1.01% (2/8) | -9.86% (8/8) | 53.4 |
| 1×4 | catalog | Prepared Aeglos | 9.732 | +11.89% (0/8) | +0.00% (0/8) | 584.3 |
| 1×4 | heldout | 48+32, fused K/H | 8.718 | +0.00% (0/8) | -8.89% (8/8) | 57.3 |
| 1×4 | heldout | 48+32, separate K/H | 9.577 | +9.24% (0/8) | -0.59% (6/8) | 99.9 |
| 1×4 | heldout | 56+32, fused K/H | 8.812 | +1.06% (0/8) | -7.76% (8/8) | 55.8 |
| 1×4 | heldout | 48+40, fused K/H | 8.771 | +0.40% (2/8) | -9.11% (8/8) | 53.5 |
| 1×4 | heldout | Prepared Aeglos | 9.609 | +9.76% (0/8) | +0.00% (0/8) | 584.0 |
| 1×16 | catalog | 40+40, G1 width 13 | 12.327 | +0.00% (0/8) | -0.08% (6/8) | 165.2 |
| 1×16 | catalog | 32+64, archived SDK | 12.814 | +3.02% (0/8) | +3.07% (0/8) | 170.5 |
| 1×16 | catalog | 48+40, G1 width 13 | 12.888 | +4.68% (0/8) | +3.56% (0/8) | 169.7 |
| 1×16 | catalog | 48+32, G1 width 13 | 12.633 | +2.37% (0/8) | +2.21% (1/8) | 160.7 |
| 1×16 | catalog | 56+32, G1 width 13 | 12.755 | +4.18% (0/8) | +3.60% (0/8) | 167.4 |
| 1×16 | catalog | 40+48, G1 width 13 | 12.458 | +0.42% (4/8) | +0.13% (4/8) | 158.4 |
| 1×16 | catalog | 40+40, G1 width 13 and fused K/H | 11.702 | -5.04% (8/8) | -5.62% (8/8) | 91.0 |
| 1×16 | catalog | 48+40, G1 width 13 and fused K/H | 12.024 | -2.16% (8/8) | -2.74% (8/8) | 93.9 |
| 1×16 | catalog | 40+40, G1 width 13 and two G1 ranges | 12.261 | -0.57% (5/8) | -1.08% (6/8) | 169.7 |
| 1×16 | catalog | 40+40, G1 width 13 and two G2 ranges | 12.307 | +0.01% (4/8) | -1.28% (5/8) | 157.7 |
| 1×16 | catalog | Prepared Aeglos | 12.394 | +0.08% (2/8) | +0.00% (0/8) | 825.7 |
| 1×16 | heldout | 40+40, G1 width 13 | 12.397 | +0.00% (0/8) | +0.17% (3/8) | 170.6 |
| 1×16 | heldout | 32+64, archived SDK | 12.723 | +2.63% (0/8) | +2.98% (0/8) | 171.2 |
| 1×16 | heldout | 48+40, G1 width 13 | 12.947 | +3.51% (0/8) | +4.78% (0/8) | 172.0 |
| 1×16 | heldout | 48+32, G1 width 13 | 12.550 | +1.29% (1/8) | +1.30% (2/8) | 172.8 |
| 1×16 | heldout | 56+32, G1 width 13 | 12.808 | +3.62% (0/8) | +3.62% (0/8) | 166.7 |
| 1×16 | heldout | 40+48, G1 width 13 | 12.299 | +0.05% (3/8) | -0.52% (5/8) | 165.3 |
| 1×16 | heldout | 40+40, G1 width 13 and fused K/H | 11.668 | -5.66% (8/8) | -5.71% (8/8) | 89.5 |
| 1×16 | heldout | 48+40, G1 width 13 and fused K/H | 12.044 | -2.46% (8/8) | -2.37% (8/8) | 84.3 |
| 1×16 | heldout | 40+40, G1 width 13 and two G1 ranges | 12.233 | -1.31% (5/8) | -1.15% (8/8) | 171.3 |
| 1×16 | heldout | 40+40, G1 width 13 and two G2 ranges | 12.317 | -0.79% (6/8) | -0.90% (6/8) | 162.9 |
| 1×16 | heldout | Prepared Aeglos | 12.392 | -0.17% (5/8) | +0.00% (0/8) | 831.0 |
| 2×2 | catalog | 32+32 | 10.291 | +0.00% (0/8) | -12.61% (8/8) | 78.9 |
| 2×2 | catalog | 32+64, archived SDK | 10.633 | +3.35% (0/8) | -10.13% (8/8) | 79.8 |
| 2×2 | catalog | 32+24 | 10.275 | -0.20% (5/8) | -12.92% (8/8) | 85.8 |
| 2×2 | catalog | 24+24 | 11.075 | +7.45% (0/8) | -6.11% (8/8) | 85.7 |
| 2×2 | catalog | 32+32, four G2 ranges | 10.747 | +4.33% (0/8) | -8.58% (8/8) | 80.4 |
| 2×2 | catalog | Prepared Aeglos | 11.775 | +14.44% (0/8) | +0.00% (0/8) | 814.5 |
| 2×2 | heldout | 32+32 | 10.310 | +0.00% (0/8) | -12.76% (8/8) | 74.5 |
| 2×2 | heldout | 32+64, archived SDK | 10.683 | +3.52% (0/8) | -9.86% (8/8) | 79.7 |
| 2×2 | heldout | 32+24 | 10.282 | -0.42% (7/8) | -13.31% (8/8) | 82.7 |
| 2×2 | heldout | 24+24 | 11.052 | +7.12% (0/8) | -6.65% (8/8) | 76.7 |
| 2×2 | heldout | 32+32, four G2 ranges | 10.756 | +4.17% (0/8) | -9.43% (8/8) | 79.8 |
| 2×2 | heldout | Prepared Aeglos | 11.853 | +14.63% (0/8) | +0.00% (0/8) | 812.5 |

The SDK control is the first Zopro profile for each request. Gnark independently reverified all 432 exported proofs. The temporary C7a instance, volumes and bucket were removed.

## All eight supplied M5 references

| Shape | C7a catalog ms | M5 CPU / Metal ms | CPU / Metal speedup | C7a panel |
|---|---:|---:|---:|---|
| 1×2 | 7.765 | 37 / 24 | 4.76× / 3.09× | This panel |
| 2×2 | 10.287 | 59 / 30 | 5.74× / 2.92× | [Narrower G2 pools](../2026-11-20-idle-core-width-c7a1024/README.md) |
| 4×2 | 13.909 | 109 / 48 | 7.84× / 3.45× | [Grouped SDK widths](../2026-11-20-grouped-sdk-widths-c7a1000/README.md) |
| 8×2 | 18.646 | 189 / 74 | 10.14× / 3.97× | [Balanced LLC placement](../2026-11-20-grouped-solver-epochs-c7a1011/README.md#82-against-the-189-ms-m5-cpu-target) |
| 12×2 | 27.636 | 284 / 101 | 10.28× / 3.65× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 16×2 | 31.036 | 335 / 124 | 10.79× / 4.00× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 24×2 | 46.118 | 528 / 176 | 11.45× / 3.82× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| merge-24×1 | 46.141 | 507 / 175 | 10.99× / 3.79× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |

5 of eight catalog shapes meet 10× against M5 CPU. 0 of eight meet 10× against Metal.

[Summary](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Cleanup](artifacts/cleanup.json)
