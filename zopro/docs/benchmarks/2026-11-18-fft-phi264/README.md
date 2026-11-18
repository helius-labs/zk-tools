# Moving G2 scalar decomposition alongside the FFT

Moving the in-place split earlier made the full 1×2 proof **6.3% slower** in the paired 96-core comparison. All seven blocks favored the original schedule. Median process means were 10.70 ms with the early split and 10.05 ms with the original split in the same executable. The preceding prototype reproduced at 10.07 ms and Aeglos measured 7.87 ms on the same machine. The earlier split remains experimental.

| 1×2 on c7a.24xlarge | Cores | Verified wall ms | CPU ms/proof | Peak RSS MiB |
|---|---:|---:|---:|---:|
| Accepted Zopro | 96 | 12.58 | 697 | 119.6 |
| Exact experiment 257, limited frontend and diversity check | 96 | 10.07 | 455 | 74.5 |
| Same-binary late split, limited frontend | 96 | 10.05 | 453 | 67.8 |
| Early split, limited frontend | 96 | 10.70 | 468 | 71.7 |
| Same-binary late split, unrestricted frontend | 96 | 10.69 | 596 | 65.7 |
| Early split, unrestricted frontend | 96 | 11.21 | 608 | 64.8 |
| Early split, 32 cores available | 32 | 11.14 | 230 | 74.6 |
| Aeglos | 96 | 7.87 | 516 | 480.1 |

The scalar-diversity check runs alongside the FFT. Once it certifies, the prototype splits the exclusively held canonical G2 scalars into signed components in the same vector. The later G2 query consumes those components directly. Inconclusive checks take the original route. The same arithmetic moved between phases without a second scalar vector, a point table or another thread pool. The directly measured 257 reference and new original-schedule control agreed to 0.6% in median paired wall time. Enabling the early split raised median paired CPU time 3.9% with the limited frontend. With the unrestricted frontend it raised wall time 5.6% and CPU time 2.5%. [Protocol](artifacts/protocol.md) · [Implementation relative to 257](artifacts/changes.patch.gz) · [Per-block effects](artifacts/analysis.json)

## Faster individual proofs, more slow proofs

| Limited frontend, 96 cores | Pooled p50 | Pooled p95 | Under 10 ms | 10 to under 11 ms | At least 11 ms | Within 9 ms |
|---|---:|---:|---:|---:|---:|---:|
| Late split | 9.34 ms | 12.63 ms | 780 | 3 | 274 | 63 |
| Early split | 11.15 ms | 12.68 ms | 498 | 23 | 536 | 116 |

Each arm had 1,057 timed requests across seven independent process backings. Moving the split doubled the frequency of the slow group from 25.9% to 50.7%. Some requests improved enough to reach 9 ms. Those wins did not compensate for the larger number of slow proofs. The earlier trace showed that the excess time on individual proofs can appear before, during or after the bucket fills. This experiment measured how moving work affects full latency. It cannot identify the cause of the excess time. [Latency distributions](artifacts/distribution.json) · [Per-request trace and excess analysis](../2026-11-18-path-split262/README.md)

The exact 257 reference reached 3.68× the supplied M5 CPU speed and 2.38× Metal. The early split reached 3.46× CPU and 2.24× Metal. This 1×2 confirmation used 64 remote exports, 8,464 timed requests and 8,656 calls including primes. Six local exports exercised successful certification, the 2×2 fallback and the merge54 general route with both settings. The native certificate test and generic core check passed. [Local route checks](artifacts/local-review.json)

All 64 exported proofs passed independent Gnark verification on AWS and after retrieval. The cumulative ledger reached 65,234 distinct exported hashes. The instance terminated, the tagged EBS query returned zero volumes and the temporary bucket was removed. [Verification](artifacts/verification.json) · [Process means](artifacts/rows.json.gz) · [Executable digests](artifacts/binaries.sha256)
