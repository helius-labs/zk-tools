# Sampled minor faults during complete proofs

Moving 24×2 product evaluation to 28 workers concentrated warm page faults in the first inverse FFT layer. The last column counts samples whose faulting instruction was inside the input-packing function itself. It writes the first transformed columns into a newly allocated output vector. The samples identify the instruction that touched the page. They do not establish why the allocator supplied that page.

Each cell includes three shuffled processes of eleven warm calls. The family column groups stacks by the nearest recognized prover operation after Rust symbol demangling. The median process fault count and the number of sampled stacks have different denominators.

| Transfer | Placement | Median minor faults per proof | Sampled stacks | Most frequent family | Direct pack_inverse_columns leaf |
|---|---|---:|---:|---|---:|
| 12x2 | main | 2,980 | 730 | curve queries (33.7%) | 5 (0.7%) |
| 12x2 | idle | 2,660 | 714 | curve queries (29.4%) | 4 (0.6%) |
| 12x2 | product | 918 | 209 | other/unresolved (43.1%) | 1 (0.5%) |
| 24x2 | main | 8,200 | 2,124 | other/unresolved (30.6%) | 29 (1.4%) |
| 24x2 | idle | 6,197 | 2,204 | other/unresolved (36.6%) | 7 (0.3%) |
| 24x2 | product | 19,614 | 5,022 | FFT/product (84.7%) | 4,253 (84.7%) |

All 18 exports verified. [Protocol](protocol.md) · [Per-process leaf and family counts](artifacts/samples.json.gz) · [Uninstrumented latency and peak memory](../2026-11-19-product-pool607/README.md)
