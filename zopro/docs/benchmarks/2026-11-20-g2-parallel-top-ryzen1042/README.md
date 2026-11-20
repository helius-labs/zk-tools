# Parallel G2 top components on Ryzen

No shape met the paired full-proof latency threshold, so the trial stays isolated. Paired full-proof latency changed by +1.13% to +2.86% against the same-build control. In route probes, median top-window time per point range changed by -8.1% to +54.0% across the eight inputs.

The G2 endomorphism path ends with a narrow indexed window. The trial lets each point range compute its high and low scalar-component reductions concurrently in the existing Rayon pool. It keeps each component's bucket algorithm and the final endomorphism and component merge. The reservation covers the second live component's parent buffers. The same worker pool bounds concurrent reducer scratch. The earlier joint signed-index trial increased proof latency. [Signed-index comparison](../2026-11-20-g2-signed-top-ryzen1041/README.md)

| Shape | Request | Frozen / control / parallel proof ms | Parallel vs control | Parallel vs frozen | Top per range control / parallel µs | Peak control / parallel MiB |
|---|---|---:|---:|---:|---:|---:|
| 1×2 | catalog | 7.518 / 7.514 / 7.733 | +2.86% (0/6) | +2.35% (0/6) | 1447.7 / 1408.4 | 60.2 / 61.0 |
| 1×2 | heldout | 7.460 / 7.489 / 7.651 | +2.53% (0/6) | +3.17% (0/6) | 1405.9 / 2165.0 | 57.5 / 58.8 |
| 2×2 | catalog | 11.253 / 11.233 / 11.396 | +1.46% (0/6) | +1.47% (1/6) | 2385.4 / 2723.8 | 90.0 / 88.0 |
| 2×2 | heldout | 11.309 / 11.316 / 11.496 | +1.13% (0/6) | +1.91% (0/6) | 2324.4 / 2162.7 | 85.5 / 87.0 |
| 1×4 | catalog | 8.416 / 8.376 / 8.612 | +2.74% (0/6) | +2.39% (1/6) | 1583.4 / 1553.9 | 69.8 / 67.8 |
| 1×4 | heldout | 8.417 / 8.401 / 8.608 | +1.71% (0/6) | +2.39% (0/6) | 1576.8 / 1644.8 | 68.0 / 67.5 |
| 1×16 | catalog | 12.648 / 12.704 / 13.017 | +2.54% (0/6) | +3.00% (0/6) | 3390.5 / 3114.3 | 96.6 / 95.0 |
| 1×16 | heldout | 12.608 / 12.691 / 12.995 | +2.25% (1/6) | +3.12% (0/6) | 2250.1 / 2154.5 | 92.9 / 93.6 |

Six order-balanced blocks use three warmups and thirteen timed proofs on the same Ryzen 9950X with 24 main and eight frontend logical workers. The frozen binaries and requests come from the C7a scorecard. A qualified result needs five of six wins against both controls on both inputs, a lower paired median, and same-build control calibration within 2% of the frozen binary. The route probes use elapsed time around each point range and run separately from the uninstrumented full-proof timing. Peak memory is whole-process RSS.

A direct group test compared the concurrent and serial outputs on ragged inputs with identity, repeated and opposite points, and one and eight Rayon workers. Diagnostic proofs exercised two ranges and 12-bit windows, four ranges and 11-bit windows, and zero-scratch fallback. Gnark independently reverified all 176 exported proofs.

[Summary](artifacts/summary.json) · [Block samples and route probes](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Trial patch](artifacts/source.patch.gz)
