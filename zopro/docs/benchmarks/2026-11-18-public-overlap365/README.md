# Scheduling public-input verification during proof generation

Each request computes a fresh public-input pairing on an existing worker. Early overlaps it with proof generation. Late starts it at verification. Both run the Krs pairing alongside variable G2 preparation and the Ar/Bs pairing. Every SDK call checks the final pairing equation. The additional live public result is a single field-extension element, with no result cached between proofs.

| Stage | Shape | Release ms | Disabled ms | Late ms | Early ms | Paired early vs release | Faster blocks | Early peak MiB |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| Screen | 1x2 | 7.641 | 7.733 | 7.575 | 7.581 | -1.48% | 5/7 | 64 |
| Screen | 2x2 | 10.886 | 10.743 | 10.784 | 10.836 | +0.63% | 2/7 | 139 |
| Screen | 4x2 | 17.197 | 16.989 | 16.902 | 17.041 | -1.86% | 4/7 | 204 |
| Screen | 16x2 | 45.407 | 45.295 | 45.196 | 47.260 | +4.16% | 2/7 | 585 |
| Confirmation | 1x2 | 7.661 | — | — | 7.639 | -0.27% | 5/9 | 78 |

[Source](artifacts/source.patch.gz) · [Process timings](artifacts/local.json.gz) · [Protocol](artifacts/protocol.md)

[Matched C7a results](aws.md)
