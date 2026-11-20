# Draining ready G2 bucket collisions on Ryzen

The queue scan fell short of the paired latency threshold on all tested shapes.

The scan reduced projective spill operands by 37.7–68.0% in the route probes. It moves ready entries into the next affine batch even when a deferred entry for an occupied bucket comes first. A last scan can use one more affine batch before spilling the remaining collisions. Both modes use the same queue capacity and preserve the projective path for queued entries that remain blocked.

| Shape | Request | Frozen / control / scan ms | Scan vs control | Scan vs frozen | Spilled control / scan | Peak control / scan MiB |
|---|---|---:|---:|---:|---:|---:|
| 1×2 | catalog | 7.494 / 7.487 / 7.412 | -1.00% (6/6) | -1.09% (6/6) | 1120 / 358 | 61.5 / 58.5 |
| 1×2 | heldout | 7.404 / 7.446 / 7.468 | +0.20% (2/6) | +0.40% (1/6) | 846 / 482 | 57.0 / 57.2 |
| 2×2 | catalog | 11.333 / 11.310 / 11.253 | -1.11% (5/6) | -0.97% (4/6) | 1349 / 725 | 89.8 / 85.5 |
| 2×2 | heldout | 11.274 / 11.269 / 11.240 | -0.22% (3/6) | -0.23% (5/6) | 1483 / 881 | 86.5 / 88.8 |
| 1×4 | catalog | 8.355 / 8.416 / 8.326 | -1.31% (6/6) | -0.56% (4/6) | 1062 / 432 | 70.2 / 68.2 |
| 1×4 | heldout | 8.417 / 8.417 / 8.280 | -1.74% (6/6) | -1.29% (6/6) | 901 / 497 | 67.8 / 65.5 |
| 1×16 | catalog | 12.738 / 12.664 / 12.665 | +0.25% (3/6) | -0.48% (4/6) | 2371 / 1478 | 92.6 / 93.4 |
| 1×16 | heldout | 12.641 / 12.663 / 12.576 | -1.17% (5/6) | -0.40% (5/6) | 1990 / 1217 | 93.7 / 93.0 |

Six matched blocks use 24 main and eight frontend Ryzen hardware threads. A profile needs at least five paired wins and a lower paired median against both controls on both requests, with the ordinary build calibrating within 2% of the frozen SDK. The separate route probes count operands passed to the projective spill handler. They leave same-point doubling outside the spill count.

One native test compares signed bucket outputs with collisions, repeated bases and identity points. Gnark independently reverified all 160 exported proofs.

[Summary](artifacts/summary.json) · [Samples and route probes](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Trial patch](artifacts/source.patch.gz)
