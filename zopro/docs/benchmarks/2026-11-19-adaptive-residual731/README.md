# Exact residual limits for the mapped 12×2 group route

The prepared group route approximates each scalar class with one representative and computes exact corrections for mismatches. On inputs with enough mismatches, the existing route falls back to the ordinary evaluator. The trial raises the exact correction fraction from 1/16 to 1/8 or 1/4 independently for the G1 and G2 group evaluators, and enters the exact count directly on a changed evaluator. The existing workspace reservation still applies. The trial adds no prepared points to the image.

Three screening blocks per input compared six cutoff combinations, a disabled control and the byte-pinned earlier mapped binary. The best mean relative median across inputs was 4-16. The first number is the denominator for G1 and the second for G2. Six fresh paired blocks per input followed. The 30 workers used all 16 Ryzen cores. All 92 exported proofs passed verification against the original key. Each latency process warmed three requests and timed thirteen. The catalog candidate measured 9.97× against the supplied M5 CPU reference and 3.54× against the supplied Metal reference.

| Input | Cutoff denominators | Median ms | Paired vs disabled | Peak MiB |
|---|---|---:|---:|---:|
| catalog | release | 28.751 | +0.86% (1/6) | 596.0 |
| catalog | off | 28.554 | +0.00% (0/6) | 572.5 |
| catalog | 4-16 | 28.492 | +0.25% (2/6) | 563.0 |
| heldout | release | 44.982 | -0.75% (4/6) | 1051.5 |
| heldout | off | 45.185 | +0.00% (0/6) | 1014.4 |
| heldout | 4-16 | 45.638 | +0.62% (1/6) | 1007.5 |

Separate route runs print the exact number of corrections relative to the number of grouped members when a call reaches the exact check. Identical messages across requests are deduplicated. The last column counts distinct messages reporting rejection by the sampled gate. These runs also inspect a 1/2 correction cap.

| Input | Cutoff denominators | Exact corrections / grouped members | Distinct sample rejections |
|---|---|---|---:|
| catalog | 8-8 | g1 4,064/169,821, g1 4,077/246,038, g2 1,899/115,738, g2 1,873/133,255 | 0 |
| catalog | 2-2 | g1 4,064/169,821, g1 4,077/246,038, g2 1,899/115,738, g2 1,873/133,255 | 0 |
| catalog | 4-4 | g1 4,064/169,821, g1 4,077/246,038, g2 1,899/115,738, g2 1,873/133,255 | 0 |
| catalog | off | g1 4,064/169,821, g1 4,077/246,038, g2 1,899/115,738, g2 1,873/133,255 | 0 |
| heldout | 4-4 | g1 68,700/169,821, g1 98,917/246,038, g2 94,713/115,738, g2 0/133,255 | 0 |
| heldout | 2-2 | g1 68,700/169,821, g1 98,917/246,038, g2 94,713/115,738, g2 0/133,255 | 0 |
| heldout | 8-8 | g1 68,700/169,821, g1 98,917/246,038, g2 94,713/115,738, g2 0/133,255 | 0 |
| heldout | off | g2 0/133,255 | 3 |

[Benchmark patch](artifacts/source.patch) · [Workflows](artifacts/workflow.json.gz) · [Timings and route logs](artifacts/rows.json.gz) · [Independent verification](artifacts/verification.json)
