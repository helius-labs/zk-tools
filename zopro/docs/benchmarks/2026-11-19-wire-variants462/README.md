# Public wire offsets across 1×2 transaction types

The same previously prepared coefficients and executable from experiment 461 were used for every input. The eight new public requests vary the asset, amount, output recipient nullifier key, blinding and Merkle trees. Two requests include an SPL deposit, two include an SPL withdrawal, two split a private transfer across recipients, and two carry data with zero value on the second output. The original reference request was also measured. Each cell uses seven randomized matched blocks of 31 warm SDK requests on the 32-worker Ryzen profile.

| Input | Transaction | Release / disabled / external offsets ms | Paired vs release | Paired vs disabled | Peak RSS with file MiB |
|---:|---|---:|---:|---:|---:|
| 0 | original | 7.177 / 7.240 / 7.057 | -1.62% (7/7) | -2.43% (7/7) | 64.8 |
| 25 | private split | 7.068 / 7.097 / 6.989 | -1.17% (7/7) | -1.72% (7/7) | 61.8 |
| 26 | deposit | 7.042 / 7.064 / 6.979 | -0.85% (7/7) | -1.03% (7/7) | 64.2 |
| 27 | withdrawal | 7.073 / 7.121 / 6.971 | -1.76% (7/7) | -1.97% (7/7) | 66.5 |
| 28 | data-bearing, zero second amount | 7.092 / 7.086 / 6.973 | -1.92% (7/7) | -1.58% (7/7) | 66.0 |
| 29 | private split | 7.045 / 7.081 / 6.997 | -1.04% (7/7) | -1.33% (7/7) | 68.8 |
| 30 | deposit | 7.068 / 7.129 / 6.971 | -1.73% (7/7) | -2.07% (7/7) | 66.8 |
| 31 | withdrawal | 7.090 / 7.082 / 7.015 | -1.46% (5/7) | -1.22% (6/7) | 62.8 |
| 32 | data-bearing, zero second amount | 7.074 / 7.063 / 6.975 | -1.37% (7/7) | -1.23% (7/7) | 63.2 |

Across the eight new transactions, the paired median change was -1.42% against release (8/8) and -1.46% against disabled (8/8). The original reached 7.057 ms, 5.24× the supplied M5 CPU time and 3.40× Metal. Median eager setup was 5.27 ms. The broader qualification passed. All 198 exported proofs passed independent Gnark verification.

[Protocol](artifacts/protocol.md) · [Samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [New input identities](artifacts/inputs.json) · [Generator changes](artifacts/generator.patch) · [External format and file](../2026-11-19-external-wire461/README.md)
