# Shifted Euler inputs on the current 12×2 SDK

The benchmark loads the archived shifted image with the current SDK. The image supplies a compact read-only table for subtracting a fixed reference from the A and B FFT inputs. Its K and public queries contain the matching linear adjustments. Both image variants use the same pinned original proving parameters and public verification. The optional two-level G2 plan is held fixed within each image comparison.

The shifted image met the local paired criterion on catalog, heldout, v58. The C7a follow-up cleared the 12×2 M5 CPU threshold on the catalog and held-out inputs. [C7a follow-up](../2026-11-20-shifted-euler-sdk-c7a1054/README.md)

| Request | Frozen / standard / shifted / standard grouped / shifted grouped ms | Shifted vs standard | Shifted grouped vs standard grouped | Shifted grouped vs frozen | Standard / shifted grouped peak MiB |
|---|---:|---:|---:|---:|---:|
| catalog | 32.351 / 31.935 / 31.214 / 31.002 / 30.152 | -1.93% (6/6) | -2.48% (6/6) | -7.11% (6/6) | 503.5 / 480.3 |
| heldout | 32.252 / 32.006 / 31.244 / 31.412 / 29.993 | -2.78% (6/6) | -5.15% (6/6) | -7.63% (6/6) | 507.8 / 498.5 |
| v58 | 32.100 / 32.258 / 31.269 / 31.244 / 30.063 | -3.19% (5/6) | -4.13% (6/6) | -6.59% (6/6) | 516.8 / 482.5 |
| v139 | 46.428 / 46.440 / 45.981 / 45.942 / 45.689 | -0.84% (6/6) | -0.86% (4/6) | -1.47% (6/6) | 753.0 / 669.3 |

Six order-balanced blocks on a Ryzen 9950X used 24 main and eight frontend logical workers. Each process warmed three proofs and timed thirteen. Warm latency is the median of process means. Peak RSS is the maximum whole-process peak across blocks. The shifted image is compared against the same binary with the standard image at both grouping settings. The frozen SDK uses the ordinary image without the optional groups. The paired criterion requires at least five wins out of six and a lower paired median in all three comparisons.

The native test verified every shifted row in a fixture, partial blocks, and malformed table rejection. Route probes confirmed the Euler and nested G2 paths separately from timing. Gnark independently reverified all 136 exports against the standard image and the expected public values.

[Summary](artifacts/summary.json) · [Samples and route probes](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Trial patch](artifacts/source.patch.gz) · [Previous 12×2 decomposition](../2026-11-20-indexed-12x2-provenance-ryzen1048/README.md)
