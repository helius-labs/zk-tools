# Query order in an uninstrumented C7a 1×2 proof

Neither ordering cleared the primary 96-core positive screen. At 96 cores, K/H first changed paired wall time by -1.30% and CPU by -1.45% (5/7 wins). At 96 cores, G2 first changed paired wall time by -0.79% and CPU by -1.55% (4/7 wins). The target was at least a 2% paired reduction with six or more wins in seven calibrated blocks.

| Cores | Engine / order | Wall ms | CPU ms | Peak MiB | Fast / slow out of 707 | M5 CPU | M5 Metal |
|---:|---|---:|---:|---:|---:|---:|---:|
| 96 | Accepted Zopro | 12.398 | 685 | 110 | 0 / 707 | 2.98× | 1.94× |
| 96 | Exact 257 | 9.878 | 451 | 78 | 528 / 171 | 3.75× | 2.43× |
| 96 | Same-binary original | 10.175 | 479 | 70 | 494 / 208 | 3.64× | 2.36× |
| 96 | K/H first | 10.071 | 475 | 102 | 507 / 199 | 3.67× | 2.38× |
| 96 | G2 first | 10.008 | 472 | 104 | 516 / 188 | 3.70× | 2.40× |
| 96 | Aeglos | 7.837 | 515 | 478 | 707 / 0 | 4.72× | 3.06× |
| 32 | Exact 257 | 10.690 | 231 | 70 | 8 / 173 | 3.46× | 2.25× |
| 32 | Same-binary original | 10.713 | 230 | 73 | 10 / 193 | 3.45× | 2.24× |
| 32 | K/H first | 10.727 | 230 | 111 | 10 / 195 | 3.45× | 2.24× |
| 32 | G2 first | 10.699 | 230 | 116 | 19 / 175 | 3.46× | 2.24× |

At 32 cores, K/H first changed paired wall time by +0.01% and CPU by -0.22% (3/7 wins). At 32 cores, G2 first changed paired wall time by +0.05% and CPU by -0.39% (3/7 wins). Median paired OFF/parent was 1.029 at 96 cores and 1.002 at 32 cores. All arms explicitly set both ordering flags. The 96-core experiments used the 32-worker frontend. Every experimental request certified. The circuit, scalars and curve algorithms matched across all three query orders. [Implementation and local experiment](../2026-11-18-join-order273/README.md)

Each wall-time figure is the median of seven process means of 101 warmed requests. Each fast request took under 10 ms and each slow request took at least 11 ms. CPU is the median process CPU time per proof, and peak RSS includes preflight. All 80 fresh exported proofs passed Gnark on AWS and on retrieval. The cloud instance terminated and both temporary EBS and S3 resources were checked. [Block ratios](artifacts/analysis.json) · [Process data](artifacts/processes.json.gz) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json)
