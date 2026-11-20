# Joint signed G2 top buckets on Ryzen

No shape met the paired full-proof latency threshold, so the trial stays isolated. Paired full-proof latency changed by +1.43% to +2.94% against the same-build control. In route probes, median top-window time per point range changed by +3.8% to +37.3% across the eight inputs.

The G2 endomorphism path ends with a narrow window. The control builds positive and negative digit buckets separately for each of its two scalar components and then merges signs. The trial records the sign in each point index and routes both components in one pass. Indexed tile workers apply signs as they load points. This removes two sign merges while preserving the final endomorphism and component merge. The new index vector lives for one window and is admitted under a checked scratch reservation. The original path runs when the reservation is denied.

| Shape | Request | Frozen / control / signed proof ms | Signed vs control | Signed vs frozen | Top per range control / signed µs | Peak control / signed MiB |
|---|---|---:|---:|---:|---:|---:|
| 1×2 | catalog | 7.571 / 7.462 / 7.660 | +2.11% (0/6) | +1.13% (0/6) | 1530.9 / 1782.9 | 59.0 / 58.5 |
| 1×2 | heldout | 7.486 / 7.476 / 7.681 | +2.82% (0/6) | +2.88% (0/6) | 1392.3 / 1911.6 | 57.5 / 59.2 |
| 2×2 | catalog | 11.197 / 11.329 / 11.474 | +1.43% (0/6) | +2.75% (0/6) | 2283.9 / 2511.9 | 87.0 / 88.0 |
| 2×2 | heldout | 11.337 / 11.336 / 11.555 | +2.03% (2/6) | +2.11% (1/6) | 2335.8 / 2423.9 | 89.0 / 86.2 |
| 1×4 | catalog | 8.414 / 8.367 / 8.588 | +2.26% (0/6) | +2.02% (0/6) | 1713.8 / 1796.4 | 69.0 / 65.5 |
| 1×4 | heldout | 8.378 / 8.415 / 8.630 | +2.51% (0/6) | +2.66% (0/6) | 1412.0 / 1521.5 | 68.8 / 65.8 |
| 1×16 | catalog | 12.731 / 12.676 / 13.081 | +2.87% (1/6) | +2.58% (0/6) | 2256.9 / 2485.1 | 99.2 / 93.7 |
| 1×16 | heldout | 12.640 / 12.626 / 12.935 | +2.94% (0/6) | +2.63% (0/6) | 2392.9 / 2681.0 | 96.1 / 95.9 |

Six order-balanced blocks use three warmups and thirteen timed proofs on the same Ryzen 9950X with 24 main and eight frontend logical workers. The frozen binaries and requests come from the C7a scorecard. A qualified result needs five of six wins against both controls on both inputs, a lower paired median, and same-build control calibration within 2% of the frozen binary. The route probes use elapsed time around each point range and run separately from the uninstrumented full-proof timing. Peak memory is whole-process RSS.

A direct group test compared the indexed output with separate signed reducers on ragged inputs with identity, repeated and opposite points, and one and eight Rayon workers. Diagnostic proofs exercised two ranges and 12-bit windows, four ranges and 11-bit windows, and zero-scratch fallback. Gnark independently reverified all 176 run exports and 16 preflight exports from the first attempt. That attempt stopped before timing because the four-range probe still requested an exclusive G2 pool after removing its frontend pool. The final probe uses one fixed 32-worker pool.

[Summary](artifacts/summary.json) · [Block samples and route probes](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Trial patch](artifacts/source.patch.gz)
