# Affine dependencies among private K-query scalars

We tested whether a proving-key converter could reduce the private K query by moving point contributions onto surviving wires. The census reconstructed each candidate scalar as a constant plus a weighted sum of up to four single-symbol anchors. Each reconstruction matched the original symbolic field expression. Hints and nonlinear expressions used the existing conservative symbolic model. Folded K slots were excluded.

| Shape | Private wires | Removable in tiles of 256 | Removable across all wires | Share across all wires | Multi-symbol matches |
|---|---:|---:|---:|---:|---:|
| 1x2 | 30,519 | 127 | 185 | 0.61% | 0 |
| 2x2 | 53,062 | 224 | 303 | 0.57% | 0 |
| 4x2 | 97,044 | 431 | 541 | 0.56% | 0 |

All matches were single-symbol aliases. Even with a search across the whole query, this rule removed at most 0.61% of the original private rows. We stopped before changing the key format. This census measures available algebraic identities. It provides no proof-latency or runtime-memory measurement.

[Counts, input hashes and analyzer memory](artifacts/results.json) · [Model change](artifacts/model.patch) · [Driver](artifacts/census.rs) · [Base model](../2026-11-18-constraint-rank249/README.md)
