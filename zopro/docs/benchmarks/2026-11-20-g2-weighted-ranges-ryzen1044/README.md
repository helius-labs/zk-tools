# G2 weighting before range merges on Ryzen

The trial applied each window's bucket weights in each point range, then added the projective sums. With this order, one range can finish while another is still building its buckets. The control merges the affine buckets first and performs a single weighting pass. The trial compares changing only the narrow top window with changing every window. It holds a checked reservation for additional weighting partials and uses the control order on denial.

Changing the top window gave paired full-proof changes from +0.13% to +3.06% against the same build. Changing every window gave +2.04% to +4.70%. The trial stays isolated.

| Shape | Input | Route | Proof ms | Versus same build | Versus frozen | Window phase ms | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|
| 1×2 | catalog | Frozen SDK | 7.538 | +0.60% (1/6) | +0.00% (0/6) | — | 58.5 |
| 1×2 | catalog | Merge buckets | 7.496 | +0.00% (0/6) | -0.59% (5/6) | 5.851 | 59.8 |
| 1×2 | catalog | Weight top ranges | 7.697 | +2.89% (0/6) | +2.23% (0/6) | 5.794 | 60.0 |
| 1×2 | catalog | Weight every range | 7.836 | +4.12% (0/6) | +3.52% (0/6) | 6.059 | 57.8 |
| 1×2 | heldout | Frozen SDK | 7.509 | +0.61% (2/6) | +0.00% (0/6) | — | 57.2 |
| 1×2 | heldout | Merge buckets | 7.480 | +0.00% (0/6) | -0.60% (4/6) | 5.821 | 57.8 |
| 1×2 | heldout | Weight top ranges | 7.662 | +3.06% (0/6) | +1.77% (0/6) | 5.975 | 59.2 |
| 1×2 | heldout | Weight every range | 7.793 | +3.92% (0/6) | +3.68% (0/6) | 6.043 | 57.5 |
| 2×2 | catalog | Frozen SDK | 11.337 | +0.17% (2/6) | +0.00% (0/6) | — | 85.2 |
| 2×2 | catalog | Merge buckets | 11.272 | +0.00% (0/6) | -0.17% (4/6) | 8.948 | 87.2 |
| 2×2 | catalog | Weight top ranges | 11.443 | +1.32% (0/6) | +0.82% (2/6) | 9.143 | 87.0 |
| 2×2 | catalog | Weight every range | 11.562 | +2.04% (0/6) | +1.79% (0/6) | 9.286 | 88.0 |
| 2×2 | heldout | Frozen SDK | 11.349 | +0.38% (1/6) | +0.00% (0/6) | — | 85.0 |
| 2×2 | heldout | Merge buckets | 11.309 | +0.00% (0/6) | -0.38% (5/6) | 8.988 | 86.0 |
| 2×2 | heldout | Weight top ranges | 11.448 | +0.90% (0/6) | +0.85% (1/6) | 9.214 | 87.8 |
| 2×2 | heldout | Weight every range | 11.716 | +3.57% (0/6) | +3.41% (0/6) | 9.113 | 84.8 |
| 1×4 | catalog | Frozen SDK | 8.432 | +0.85% (1/6) | +0.00% (0/6) | — | 69.0 |
| 1×4 | catalog | Merge buckets | 8.365 | +0.00% (0/6) | -0.84% (5/6) | 6.202 | 70.0 |
| 1×4 | catalog | Weight top ranges | 8.615 | +3.02% (1/6) | +1.97% (0/6) | 6.450 | 70.0 |
| 1×4 | catalog | Weight every range | 8.755 | +4.70% (1/6) | +3.81% (0/6) | 6.245 | 65.8 |
| 1×4 | heldout | Frozen SDK | 8.482 | +0.61% (2/6) | +0.00% (0/6) | — | 69.2 |
| 1×4 | heldout | Merge buckets | 8.423 | +0.00% (0/6) | -0.61% (4/6) | 6.188 | 66.8 |
| 1×4 | heldout | Weight top ranges | 8.570 | +1.63% (0/6) | +0.60% (0/6) | 6.537 | 66.2 |
| 1×4 | heldout | Weight every range | 8.762 | +4.07% (0/6) | +3.70% (0/6) | 6.428 | 64.5 |
| 1×16 | catalog | Frozen SDK | 12.690 | -1.20% (6/6) | +0.00% (0/6) | — | 93.7 |
| 1×16 | catalog | Merge buckets | 12.875 | +0.00% (0/6) | +1.22% (0/6) | 9.255 | 95.7 |
| 1×16 | catalog | Weight top ranges | 12.970 | +0.13% (2/6) | +1.59% (0/6) | 9.183 | 98.9 |
| 1×16 | catalog | Weight every range | 13.115 | +2.29% (0/6) | +3.38% (0/6) | 9.165 | 92.9 |
| 1×16 | heldout | Frozen SDK | 12.760 | +0.44% (1/6) | +0.00% (0/6) | — | 93.0 |
| 1×16 | heldout | Merge buckets | 12.665 | +0.00% (0/6) | -0.44% (5/6) | 9.003 | 95.1 |
| 1×16 | heldout | Weight top ranges | 13.025 | +2.29% (0/6) | +2.08% (0/6) | 9.317 | 93.7 |
| 1×16 | heldout | Weight every range | 13.048 | +3.08% (0/6) | +2.08% (0/6) | 9.438 | 93.4 |

Six order-balanced timing blocks use three warmups and thirteen timed proofs on a Ryzen 9950X with 24 main and eight frontend logical workers. A qualified profile requires five of six paired wins against the same build and frozen SDK on both inputs, a lower paired median, and same-build calibration within 2% of frozen. Window-phase timings come from separate route probes around all windows. Peak memory is the whole-process RSS.

The arithmetic test compared both orders against generator multiplication with repeated, opposite and identity points, signed scalars and uneven ranges. It exercised two and four ranges on one and eight Rayon workers. Separate proof probes verified both range layouts and zero-scratch fallback. Gnark independently reverified all 236 proof exports.

[Summary](artifacts/summary.json) · [Block samples and route probes](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Trial patch](artifacts/source.patch.gz)
