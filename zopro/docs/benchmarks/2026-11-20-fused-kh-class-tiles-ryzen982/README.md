# Direct class tiles in the fused 2×2 K/H query on Ryzen

The eight-shard mode shortened the measured K/H sum by about 1.2–1.3 ms before compaction. Compaction took 0.83–0.86 ms, and full proofs were 1.39–1.99% slower in paired blocks. The sixteen-shard profile had essentially equal full-proof latency to ordinary on both inputs.

The trial reduces the 53,079 K rows to 27,287 and 27,340 exact classes for catalog and held-out. The class tiles write directly into the final K point and scalar arrays. The same fused kernel consumes those compact rows and the original H rows on 24 main workers with an eight-worker frontend. This addresses the preparation cost seen in the [earlier grouping trial](../2026-11-19-fused-kh-grouped-k-ryzen797/README.md).

| Input | Mode | Proof ms | Paired vs ordinary | Compact ms | K/H window ms | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| catalog | Exact 943 | 9.403 | -0.49% (6/8) | — | — | 96.8 |
| catalog | Ordinary | 9.421 | — | — | 5.968 | 94.8 |
| catalog | 8 shards | 9.621 | +1.99% (1/8) | 0.835 | 4.676 | 96.5 |
| catalog | 16 shards | 9.419 | +0.01% (4/8) | 0.872 | 5.168 | 99.8 |
| catalog | 32 shards | 9.476 | +0.27% (2/8) | 0.647 | 5.336 | 99.8 |
| heldout | Exact 943 | 9.432 | -0.05% (5/8) | — | — | 94.5 |
| heldout | Ordinary | 9.420 | — | — | 5.976 | 94.5 |
| heldout | 8 shards | 9.531 | +1.39% (0/8) | 0.862 | 4.762 | 97.8 |
| heldout | 16 shards | 9.430 | +0.10% (4/8) | 0.655 | 5.351 | 101.0 |
| heldout | 32 shards | 9.461 | +0.38% (3/8) | 0.639 | 5.341 | 95.0 |

Ordinary calibrated within 0.50% of the exact Euler FFT SDK. The separate clock panel shifted paired latency by up to 2.74%. Each timed comparison uses eight blocks of seventeen proofs. The additional peak reservation across shard counts was 19.25–19.29 MiB. Route probes compared the complete K/H curve point with the ordinary result and exercised the ordinary path at zero optional scratch. Two native tests and all 122 independent proof verifications passed.

[Results](artifacts/summary.json) · [Samples](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Source patch](artifacts/source.patch.gz)
