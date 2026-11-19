# Sampled minor faults during complete proofs

Each cell includes three shuffled processes of eleven warm calls. The family column groups stacks by the nearest recognized prover operation. Stacks with no recognized operation are retained. The median process fault count and the number of sampled stacks have different denominators.

| Transfer | Placement | Median minor faults per proof | Sampled stacks | Most frequent family |
|---|---|---:|---:|---|
| 12x2 | main | 2,980 | 730 | other/unresolved (28.9%) |
| 12x2 | idle | 2,660 | 714 | other/unresolved (33.3%) |
| 12x2 | product | 918 | 209 | other/unresolved (58.9%) |
| 24x2 | main | 8,200 | 2,124 | other/unresolved (51.9%) |
| 24x2 | idle | 6,197 | 2,204 | other/unresolved (61.3%) |
| 24x2 | product | 19,614 | 5,022 | other/unresolved (95.2%) |

All 18 exports verified. [Protocol](protocol.md) · [Per-process leaf and family counts](artifacts/samples.json.gz) · [Uninstrumented latency and peak memory](../2026-11-19-product-pool607/README.md)
