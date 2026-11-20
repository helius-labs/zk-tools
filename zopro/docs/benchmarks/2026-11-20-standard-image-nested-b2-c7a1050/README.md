# Nested G2 grouping with the standard 12×2 image on C7a

The nested G2 trial uses the current SDK and standard Euler image. The four configurations hold the requests, key, and worker placement fixed. Point sums and indices for both levels reserve about 25.9 MiB at startup. When preparation declines, the existing path produces the proof. The v139 input used the first level and declined the second under the mismatch rule.

All four inputs fell short of the paired latency criterion. The nested catalog result was 29.454 ms against the 28.4 ms CPU target and the 10.1 ms Metal target. It was 9.64× faster than the supplied M5 CPU timing and 3.43× faster than the Metal timing. The held-out request took 29.626 ms. On the catalog request, process CPU time per warm proof fell from 1376 to 1347 ms, and the cold response took 21.7 ms longer.

| Request | Frozen / control / first level / nested warm ms | Nested vs control | Nested vs frozen | Control / nested cold ms | Control / nested peak warm MiB |
|---|---:|---:|---:|---:|---:|
| catalog | 29.623 / 29.475 / 29.547 / 29.454 | -0.51% (5/8) | -1.25% (5/8) | 314.1 / 335.8 | 546.2 / 530.5 |
| heldout | 29.766 / 29.689 / 29.665 / 29.626 | -0.36% (5/8) | +0.10% (4/8) | 312.8 / 336.2 | 547.8 / 542.5 |
| v58 | 29.822 / 29.791 / 29.557 / 29.598 | -1.05% (7/8) | -0.60% (6/8) | 313.9 / 335.7 | 541.6 / 534.2 |
| v139 | 40.166 / 40.066 / 40.487 / 40.437 | -0.22% (4/8) | +0.71% (4/8) | 328.2 / 349.4 | 850.0 / 752.5 |

One c7a.24xlarge ran 64 main and 32 frontend workers. Eight order-balanced blocks measure thirteen warm proofs per process after three warmups. Warm time is the median of process means. Cold time includes key loading, preparation, and the first verified proof from a fresh process with a shared file cache. Peak RSS is the maximum whole-process peak across blocks. Qualification requires a lower paired median and at least seven paired wins against both controls, with same-build control calibration within 2% of the frozen binary. First-response peak memory and first-level results are in the summary.

Under a zero optional preparation budget, all four inputs selected the fallback and produced verified proofs. Gnark independently reverified all 280 first and warm proof exports after retrieval. The temporary instance, volume, and bucket were removed.

[Summary](artifacts/summary.json) · [Samples and route checks](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Local trial and source](../2026-11-20-standard-image-nested-b2-ryzen1049/README.md) · [All eight M5 reference shapes](../2026-11-20-c7a-three-provers/README.md#supplied-m5-references)
