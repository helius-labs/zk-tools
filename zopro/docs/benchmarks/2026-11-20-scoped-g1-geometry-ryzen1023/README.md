# G1 signed window geometry on the scoped G2 path

On 1×4, fused K/H with width 12 for A reduced paired full-proof latency by 5.78% and 5.11% on the two inputs. On 1×16, width 13 with separate K and H reduced latency by 4.67% and 4.29%. No geometry qualified on 1×2 or 2×2.

All profiles use the same executable and 20+12 fixed Ryzen logical workers. The geometry settings apply to the small signed G1 path. The existing fused K/H kernel has its own geometry and retains the same budget check. Run-time diagnostics confirm the A/K/H or A/K+H route.

| Shape | Input | Profile | Full proof ms | Paired vs width 12 | Peak MiB |
|---|---|---|---:|---:|---:|
| 1×2 | catalog | Width 12, one range | 6.581 | +0.00% (0/6) | 59.8 |
| 1×2 | catalog | Width 11 | 6.578 | +0.70% (2/6) | 59.0 |
| 1×2 | catalog | Width 13 | 6.590 | +0.57% (2/6) | 63.0 |
| 1×2 | catalog | Width 14 | 6.692 | +1.53% (0/6) | 66.2 |
| 1×2 | catalog | Width 12, two ranges | 6.568 | -0.40% (3/6) | 63.8 |
| 1×2 | heldout | Width 12, one range | 6.406 | +0.00% (0/6) | 60.0 |
| 1×2 | heldout | Width 11 | 6.382 | +0.34% (1/6) | 62.5 |
| 1×2 | heldout | Width 13 | 6.418 | +1.18% (2/6) | 66.5 |
| 1×2 | heldout | Width 14 | 6.483 | +1.75% (1/6) | 73.2 |
| 1×2 | heldout | Width 12, two ranges | 6.395 | +0.55% (2/6) | 65.0 |
| 1×4 | catalog | Width 12, one range | 8.235 | +0.00% (0/6) | 103.5 |
| 1×4 | catalog | Width 11 | 8.273 | +0.78% (1/6) | 110.8 |
| 1×4 | catalog | Width 13 | 8.017 | -1.77% (5/6) | 108.5 |
| 1×4 | catalog | Width 14 | 8.567 | +3.72% (1/6) | 118.2 |
| 1×4 | catalog | Width 12, two ranges | 8.009 | -3.08% (6/6) | 115.2 |
| 1×4 | catalog | Fused K/H, A width 12 | 7.740 | -5.78% (6/6) | 74.0 |
| 1×4 | catalog | Fused K/H, A width 13 | 7.795 | -4.78% (6/6) | 72.2 |
| 1×4 | heldout | Width 12, one range | 8.203 | +0.00% (0/6) | 112.2 |
| 1×4 | heldout | Width 11 | 8.229 | +0.79% (2/6) | 113.2 |
| 1×4 | heldout | Width 13 | 8.052 | -2.14% (5/6) | 112.2 |
| 1×4 | heldout | Width 14 | 8.557 | +3.98% (1/6) | 117.5 |
| 1×4 | heldout | Width 12, two ranges | 8.025 | -2.16% (5/6) | 107.5 |
| 1×4 | heldout | Fused K/H, A width 12 | 7.820 | -5.11% (5/6) | 82.2 |
| 1×4 | heldout | Fused K/H, A width 13 | 7.784 | -4.88% (6/6) | 73.2 |
| 1×16 | catalog | Width 12, one range | 12.087 | +0.00% (0/6) | 161.9 |
| 1×16 | catalog | Width 11 | 12.427 | +1.89% (1/6) | 156.5 |
| 1×16 | catalog | Width 13 | 11.565 | -4.67% (5/6) | 157.9 |
| 1×16 | catalog | Width 14 | 11.910 | -1.63% (4/6) | 172.2 |
| 1×16 | catalog | Width 12, two ranges | 11.819 | -1.80% (5/6) | 167.9 |
| 1×16 | catalog | Fused K/H, A width 12 | 11.861 | -1.70% (4/6) | 105.2 |
| 1×16 | catalog | Fused K/H, A width 13 | 12.359 | -0.73% (4/6) | 125.0 |
| 1×16 | heldout | Width 12, one range | 12.132 | +0.00% (0/6) | 163.9 |
| 1×16 | heldout | Width 11 | 12.376 | +1.23% (1/6) | 146.6 |
| 1×16 | heldout | Width 13 | 11.582 | -4.29% (6/6) | 153.9 |
| 1×16 | heldout | Width 14 | 11.937 | -1.60% (4/6) | 166.6 |
| 1×16 | heldout | Width 12, two ranges | 11.895 | -2.07% (6/6) | 160.2 |
| 1×16 | heldout | Fused K/H, A width 12 | 11.796 | -2.77% (6/6) | 105.9 |
| 1×16 | heldout | Fused K/H, A width 13 | 11.820 | -2.20% (5/6) | 114.5 |
| 2×2 | catalog | Width 12, one range | 10.061 | +0.00% (0/6) | 99.8 |
| 2×2 | catalog | Width 11 | 10.007 | -0.97% (4/6) | 105.5 |
| 2×2 | catalog | Width 13 | 10.055 | -0.15% (3/6) | 105.0 |
| 2×2 | catalog | Width 14 | 10.263 | +1.28% (1/6) | 104.2 |
| 2×2 | catalog | Width 12, two ranges | 10.126 | +0.31% (2/6) | 94.5 |
| 2×2 | heldout | Width 12, one range | 10.161 | +0.00% (0/6) | 96.8 |
| 2×2 | heldout | Width 11 | 10.019 | -1.69% (4/6) | 100.5 |
| 2×2 | heldout | Width 13 | 10.020 | -1.22% (5/6) | 108.8 |
| 2×2 | heldout | Width 14 | 10.154 | +0.55% (2/6) | 107.8 |
| 2×2 | heldout | Width 12, two ranges | 10.106 | -0.43% (4/6) | 107.8 |

Six timing blocks prime three proofs and time thirteen per process. One diagnostic block per input and profile checks the selected G1 queries. All 336 exported proofs passed independent Gnark verification. The C7a transfer is a separate measurement.

[Summary](artifacts/summary.json) · [Samples and route diagnostics](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json)
