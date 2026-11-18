# Starting all witness queries alongside the FFT on 24×2

On a fresh c7a.24xlarge, moving the witness curve queries ahead of FFT completion increased 24×2 verified latency at 96 cores from **57.23 to 68.86 ms** in the same executable. All seven paired blocks favored the existing schedule at both 32 and 96 cores. The exact accepted binary measured 56.99 ms at 96 cores. Its supplied M5 CPU target is 52.8 ms.

The tested change uses one Rayon join to start A, K, B2 and the public correction alongside the FFT. The H query follows its FFT dependency. The existing path starts curve queries after the FFT. All curve arithmetic, grouping admission and in-place scalar storage are retained. The change is limited to folded general proofs with an exclusive B2 query of at least 65,536 entries and no prepared equal-witness path. Although the dependency graph permits this concurrency, the existing schedule completed the proof faster on these hosts.

## AWS results

| Cores | Engine / schedule | Wall ms | CPU ms/proof | Peak MiB | Blocks ≤52.8 ms | M5 CPU | M5 Metal |
|---:|---|---:|---:|---:|---:|---:|---:|
| 32 | Accepted Zopro | 61.89 | 1767 | 1245 | 0/7 | 8.53× | 2.84× |
| 32 | Same-binary OFF | 61.60 | 1745 | 1256 | 0/7 | 8.57× | 2.86× |
| 32 | Candidate ON | 72.91 | 1779 | 1325 | 0/7 | 7.24× | 2.41× |
| 32 | Aeglos | 171.44 | 5193 | 6368 | 0/7 | 3.08× | 1.03× |
| 96 | Accepted Zopro | 56.99 | 3768 | 1264 | 0/7 | 9.26× | 3.09× |
| 96 | Same-binary OFF | 57.23 | 3756 | 1254 | 0/7 | 9.23× | 3.08× |
| 96 | Candidate ON | 68.86 | 3886 | 1336 | 0/7 | 7.67× | 2.56× |
| 96 | Aeglos | 86.81 | 7402 | 6800 | 0/7 | 6.08× | 2.03× |

Each row uses the median of seven uninstrumented process means. Peak RSS is the largest lifetime measurement in the arm, including the preflight. The two same-binary OFF controls agreed with accepted wall time within the prespecified factor of 1.03. At 96 cores peak RSS increased from 1,254 to 1,336 MiB. All 64 exported proofs passed Gnark on the cloud host and after retrieval. This includes 1,184 timed requests, one exported proof per process. The instance terminated, tagged EBS volumes were absent and the temporary bucket was removed. [Protocol](artifacts/protocol.md) · [Paired block data](artifacts/analysis.json) · [Verification](artifacts/verification.json)

## Local shape coverage

The Ryzen 9950X screen used 16 workers, three primes and seven timed proofs in one process per arm and shape. It establishes route coverage and correctness. Single-process latency and memory figures are exploratory.

| Shape | Routed / 7 | OFF ms | ON ms | Peak OFF / ON MiB | M5 CPU / ON | M5 Metal / ON |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 0/7 | 8.29 | 8.33 | 64 / 62 | 4.44× | 2.88× |
| 2x2 | 0/7 | 11.73 | 11.85 | 138 / 142 | 4.98× | 2.53× |
| 4x2 | 7/7 | 16.92 | 17.51 | 195 / 224 | 6.23× | 2.74× |
| 8x2 | 7/7 | 26.58 | 28.47 | 323 / 396 | 6.64× | 2.60× |
| 12x2 | 7/7 | 38.16 | 44.71 | 492 / 651 | 6.35× | 2.26× |
| 16x2 | 7/7 | 45.81 | 53.68 | 542 / 668 | 6.24× | 2.31× |
| 24x2 | 7/7 | 75.38 | 89.23 | 892 / 885 | 5.92× | 1.97× |
| merge-24x1 | 7/7 | 73.40 | 87.55 | 830 / 779 | 5.79× | 2.00× |
| merge-54x1 | 7/7 | 169.95 | 200.72 | 1601 / 1687 | — | — |

All 18 local exports verified. The change engaged on seven shapes and remained inactive for 1×2 and 2×2. Merge54 has no supplied M5 baseline. The native build and generic core check passed. The experiment was not adopted. [Local exports](artifacts/local-review.json) · [Source changes](artifacts/changes.patch.gz)
