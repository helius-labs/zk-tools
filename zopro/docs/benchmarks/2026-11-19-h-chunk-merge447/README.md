# Merging and weighting H buckets in a single parallel pass

After filling two ranges of scalar-digit buckets, the prover combines buckets at the same index and computes their weighted sum. The candidate performs both operations in each 512-bucket chunk. The ordinary path completes the merge of the whole window before beginning the weighted sum. Both use the same group operations. Additional temporary workspace is charged before starting the combined pass. Seven matched blocks timed eleven complete proofs per process on 32 Ryzen hardware threads.

| Shape | Release / disabled / enabled ms | Paired vs release | Paired vs disabled | CPU-ms disabled / enabled | Peak MiB disabled / enabled | M5 CPU / Metal speedup |
|---|---:|---:|---:|---:|---:|---:|
| 16x2 | 40.199 / 40.013 / 39.928 | -0.87% (5/7) | -0.21% (4/7) | 1155.2 / 1151.2 | 733.8 / 726.0 | 8.39× / 3.11× |
| 24x2 | 67.405 / 67.284 / 67.097 | +0.03% (3/7) | -0.00% (4/7) | 1957.9 / 1948.4 | 1119.5 / 1116.2 | 7.87× / 2.62× |
| merge-24x1 | 65.782 / 65.147 / 64.353 | -2.17% (6/7) | -0.70% (5/7) | 1854.7 / 1840.5 | 1046.2 / 1039.0 | 7.88× / 2.72× |

Only merge-24×1 met the confirmation criterion against both controls. This qualifies it for a varied-witness test. The combined pass uses the same paired 512-bucket reduction as the release. [Earlier 256-bucket comparison](../2026-11-18-merge-weight362/README.md)

All 69 exports passed independent Gnark verification. Separate route probes recorded the combined pass on each shape.

[Source patch](artifacts/source.patch) · [Process samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [Protocol](artifacts/protocol.md)
